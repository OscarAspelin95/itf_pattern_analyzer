use std::fmt::Display;

use crate::{
    direction::{Angle, Rotation},
    distance::{Margin, Measurement},
};

#[derive(Debug, Clone, Copy)]
pub struct Foot {
    pub x: f64,
    pub y: f64,
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
    angle: f64,
    // Offset from (x, y) start.
    left_foot: Foot,
    right_foot: Foot,
    // body measurements in cm.
    measurement: Measurement,
    // (x, y) start. Depends on start stance.
    start_left: Foot,
    start_right: Foot,
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

        let left_foot = Foot { x: left_x, y: 0.0 };
        let right_foot = Foot { x: right_x, y: 0.0 };

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
    pub fn rotate(&mut self, rotation: Rotation, angle: Angle) {
        match rotation {
            Rotation::Left => self.angle += angle.radians(),
            Rotation::Right => self.angle -= angle.radians(),
        }
    }

    pub fn is_within_margin(&self, margin: Margin) -> bool {
        (self.left_foot.x - self.start_left.x).abs() <= margin.x
            && (self.left_foot.y - self.start_left.y).abs() <= margin.y
            && (self.right_foot.x - self.start_right.x).abs() <= margin.x
            && (self.right_foot.y - self.start_right.y).abs() <= margin.y
    }
}

#[cfg(test)]
pub mod test {
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

    // TODO - fix
    fn test_turn() {
        let measurement = Measurement {
            shoulder_width: 100.0,
            foot_length: 30.0,
            foot_width: 10.0,
        };

        let mut position = Position::new(measurement, StartPosition::ShoulderWidth);
    }
}
