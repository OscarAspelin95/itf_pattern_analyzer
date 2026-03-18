use crate::{
    direction::{Angle, Direction, Foot, Rotation},
    distance::{Distance, Margin, Measurement},
};

#[derive(Debug, Clone, Copy)]
pub struct FootPosition {
    pub x: f64,
    pub y: f64,
}

impl FootPosition {
    pub fn move_x(&mut self, dist: f64) {
        self.x += dist
    }

    pub fn move_y(&mut self, dist: f64) {
        self.y += dist
    }
}

#[derive(Debug)]
pub enum StartPosition {
    /// Outside edges of both feet equal shoulder width apart.
    ShoulderWidth,
    /// Insides of both feet touching.
    NoSpace,
}

#[derive(Debug)]
pub struct Position {
    pub angle: f64,
    // Offset from (x, y) start.
    pub left_foot: FootPosition,
    pub right_foot: FootPosition,
    // body measurements in cm.
    pub measurement: Measurement,
    // (x, y) start. Depends on start stance.
    pub start_left: FootPosition,
    pub start_right: FootPosition,
}

/// We are tracking the (x, y) position of the left and right foot respectively.
/// Specifically, we are tracking the middle (center) of the foot and their offset from the center (x, y) = (0, 0). If we assume
/// no offset in y at start (start line is exactly through the middle of the foot vertically),
/// we only have an offset in the x-direction, dictated by the start stance.

/// This is a bit more complex than tracking the (x, y) position of the center of the body,
/// but is more precise and provides more resolution.
///
/// Since we are using radians here, the following applies:
/// * pi / 2 	-> 90 degrees	-> direction forward.
/// * pi	 	-> 180 degrees 	-> direction left.
/// * pi * 3/2 	-> 270 degrees 	-> direction backward.
/// * 0 		-> 0 degrees 	-> direction right.
///
/// This is why we start with pi/2 (direction forward).
impl Position {
    // We always start with an offset in the x-direction for both feet.
    // * For narani junbi sogi, we
    pub fn new(measurement: Measurement, start: StartPosition) -> Self {
        let (left_x, right_x) = match start {
            StartPosition::ShoulderWidth => {
                // Outside edges = shoulder_width, so center of each foot is
                // inset by half a foot_width from shoulder_width/2.
                let offset = (measurement.shoulder_width - measurement.foot_width) / 2.0;
                (-offset, offset)
            }
            StartPosition::NoSpace => {
                // Insides touching at x=0, so centers are half a foot_width apart.
                let offset = measurement.foot_width / 2.0;
                (-offset, offset)
            }
        };

        let left_foot = FootPosition { x: left_x, y: 0.0 };
        let right_foot = FootPosition { x: right_x, y: 0.0 };

        Self {
            angle: Angle::Degree90.radians(),
            left_foot: left_foot,
            right_foot: right_foot,
            measurement: measurement,
            start_left: left_foot,
            start_right: right_foot,
        }
    }
}

impl Position {
    pub fn custom_dist(&self, distance: Distance) -> f64 {
        match distance {
            Distance::ShoulderWidth(dist) => dist * self.measurement.shoulder_width,
            Distance::FootLength(dist) => dist * self.measurement.foot_length,
            Distance::FootWidth(dist) => dist * self.measurement.foot_width,
        }
    }

    /// Rotate the "forward direction" with a particular angle.
    ///
    /// For example, we always start a pattern forward, which is an angle of 90 degrees
    /// because we define the angle 0 as along the x-axis.
    ///
    /// If we turn to the left, we increase the angle by 90 degrees.
    /// If we turn to the right, we decrease the angle by 90 degrees
    ///
    ///
    ///     y (90 degrees)
    ///
    /// 	^
    ///		|
    /// 	|
    /// 	|
    /// 	|- - - - - - - - - ->  x (0 degrees)
    ///
    /// We define:
    /// * Left turn as increasing the angle (anti-clockwise).
    /// * Right turn as decreasing the angle (clockwise).
    pub fn rotate(&mut self, rotation: Rotation, angle: Angle) {
        match rotation {
            Rotation::Left => self.angle += angle.radians(),
            Rotation::Right => self.angle -= angle.radians(),
        }
    }

    /// Move the left or right foot in either x or y direction.
    /// Arguments:
    /// 	* `foot` - What foot to move.
    /// 	* `direction` - Along what axis to move.
    /// 	* `angle_offset` - See explanation below
    ///
    /// `angle_offset` - The angle at which the foot moves with respect to the
    /// currently facing angle. For example, if facing "forward" we have an angle of 90 degrees
    /// (see `fn rotate`).
    ///
    /// If we take a step `forward` with the left foot in a walking stance,
    /// we'll move the left foot in the positive y-direction. This would be an offset angle of
    /// 0 degrees with respect to where we are facing. For the x-direction, we'll move to the `left`
    /// direction, which is an offset angle of +90 degrees with respect to where we are facing.
    pub fn move_foot(&mut self, foot: Foot, direction: Direction, angle_offset: f64) {
        match (foot, direction) {
            (Foot::Left, Direction::X(dist)) if dist > 0.0 => {
                self.left_foot
                    .move_x(dist * (self.angle + angle_offset).cos().round());
            }
            (Foot::Left, Direction::Y(dist)) if dist > 0.0 => {
                self.left_foot
                    .move_y(dist * (self.angle + angle_offset).sin().round());
            }
            (Foot::Right, Direction::X(dist)) if dist > 0.0 => {
                self.right_foot
                    .move_x(dist * (self.angle + angle_offset).cos().round());
            }
            (Foot::Right, Direction::Y(dist)) if dist > 0.0 => {
                self.right_foot
                    .move_y(dist * (self.angle + angle_offset).sin().round());
            }
            (foot, direction) => {
                panic!(
                    "Invalid input: {:?}, {:?}. Check that value is > 0.0.",
                    foot, direction
                );
            }
        }
    }

    pub fn is_within_margin(&self, margin: Margin) -> bool {
        (self.left_foot.x - self.start_left.x).abs() <= margin.x
            && (self.left_foot.y - self.start_left.y).abs() <= margin.y
            && (self.right_foot.x - self.start_right.x).abs() <= margin.x
            && (self.right_foot.y - self.start_right.y).abs() <= margin.y
    }

    pub fn print_feet_position(&self) {
        println!("Left: {:?} | Right: {:?}", self.left_foot, self.right_foot);
    }
}

#[cfg(test)]
pub mod test {
    use crate::stance::Stance;

    use super::*;
    use rstest::*;

    #[rstest]
    #[case(Angle::Degree90, 0.0, 1.0)] // forward/up with respect to coordinate system.
    #[case(Angle::Degree180, -1.0, 0.0)] // left with respect to coordinate system.
    #[case(Angle::Degree270, 0.0, -1.0)] // backward/down with respect to coordinate system.
    #[case(Angle::Degree360, 1.0, 0.0)] // no change
    fn test_angles(#[case] angle: Angle, #[case] expected_x: f64, #[case] expected_y: f64) {
        assert_eq!(angle.resolve_x(), expected_x);
        assert_eq!(angle.resolve_y(), expected_y);
    }

    fn test_move() {
        let measurement = Measurement {
            shoulder_width: 100.0,
            foot_length: 30.0,
            foot_width: 10.0,
        };

        let mut position = Position::new(measurement, StartPosition::NoSpace);

        // 1. step FORWARD in walking stance with left foot.
        let stance_spec = Stance::GunnunSogi.resolve(&position.measurement);

        let dy = stance_spec.length;

        // measures toe to toe, which is equal to (vertical) middle of foot to middle of foot.
        position.move_foot(Foot::Left, Direction::Y(dy), Angle::Degree0.radians());

        assert_eq!(
            position.left_foot.y,
            1.5 * position.measurement.shoulder_width
        )
    }
}
