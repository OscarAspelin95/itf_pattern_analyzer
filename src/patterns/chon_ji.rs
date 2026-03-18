use crate::{
    direction::{Angle, Direction, Foot, Rotation},
    distance::Measurement,
    position::{Position, StartPosition},
    stance::Stance,
};

pub fn analyze_chon_ji(measurement: Measurement) {
    let mut position = Position::new(measurement, StartPosition::ShoulderWidth);

    position.print_feet_position();

    step_1(&mut position);
    position.print_feet_position();

    step_2(&mut position);
    position.print_feet_position();

    step_3(&mut position);
    position.print_feet_position();

    step_4(&mut position);
    position.print_feet_position();

    step_5(&mut position);
    position.print_feet_position();
}

/// Step 1 includes:
/// 1.1 - 90 degree turn to the left with left foot.
/// 1.2 - Step out in walking stance with left foot (negative x- and y-direction).
/// 	1.2.1 - A move in negative x-direction.
/// 	1.2.2 - A move in negative y-direction.
fn step_1(position: &mut Position) {
    let stance = Stance::GunnunSogi.resolve(&position.measurement);

    // 1.1 - Turn left 90 degrees.
    position.rotate(Rotation::Left, Angle::Degree90);

    // 1.2 - Left foot to walking stance.

    // 1.2.1 - Move left foot in negative x-direction.
    //
    // COMMENT - Since we already have an x-offset due to narani junbi sogi, we need to account for this.
    // The stance length is 1.5 shoulders, but we already stand with a shoulder width, measured from the
    // outside of the feet. This means the absolute distance we move the left foot in the x-direction is NOT
    // 1.5 shoulders, but rather that subtracted by (|start_left_x| + |start_right_x|)
    //
    // ------------------------------------ Ready stance ------------------------------------
    //
    // 						       				^   ^	 |    ^   ^
    // 						       				  +		 |  	+
    // 						       				|   | 	 |    |   |
    // 						       				  <---x--><--x-->
    // 						       				<----1 shoulder--->
    //
    // The offsets in x direction for left/right foot we have from the Position struct.
    //
    //
    //
    // ------------------------------------ Walking stance ------------------------------------
    //
    //													|	<    --
    // 						   							|	   +
    // 													|	<    --
    //
    // 						   <-----target---->
    // 											<--x---><--x--->
    // 						<    --
    // 						   +
    // 						<    --
    //
    // 						   <------------1.5 shoulders------>
    //
    // Which means the actual distance we move the left foot in x-direction is 1.5 shoulders - (|start_left_x| + |start_x_right|)
    position.move_foot(
        Foot::Left,
        Direction::X(stance.length - (position.start_left.x.abs() + position.start_right.x.abs())),
        Angle::Degree0.radians(),
    );

    // 1.2.2 - Move left foot in negative y-direction.
    //
    // COMMENT - Both feet start at the same level. Since we track the middle
    // of both feet, the distance we move the left foot is in fact exactly 1.0 shoulder width.
    position.move_foot(
        Foot::Left,
        Direction::Y(stance.width),
        Angle::Degree90.radians(),
    );
}

/// Step 2 includes:
/// 1 - Step with right to a right walking stance.
/// 1.1 - A move in negative x-direction.
fn step_2(position: &mut Position) {
    let stance = Stance::GunnunSogi.resolve(&position.measurement);

    // 1 - Step to right walking stance.
    // 1.1 - We are moving the right foot a total of 3 shoulder widths (2 * 1.5).
    position.move_foot(
        Foot::Right,
        Direction::X(2.0 * stance.length),
        Angle::Degree0.radians(),
    );

    // COMMENT - No offset in y-direction since we keep the shoulder width distance.
}

/// Step 3 includes:
/// 1.1 - A 180 degree spot turn.
/// 1.2 - Move right foot to a right walking stance.
/// 	1.2.1 - A move in positive x-direction.
/// 	1.2.2 - A move in negative y-direction.
fn step_3(position: &mut Position) {
    let stance = Stance::GunnunSogi.resolve(&position.measurement);

    // 1.1 - Rotation
    position.rotate(Rotation::Right, Angle::Degree180);

    // 1.2

    // 1.2.1 - Move right foot in positive x-direction.
    //
    // COMMENT: We move a total of 3 shoulder widths (2 * 1.5).
    position.move_foot(
        Foot::Right,
        Direction::X(2.0 * stance.length),
        Angle::Degree0.radians(),
    );

    // 1.2.2 - Move right foot in negative y-direction.
    //
    // COMMENT: We move a total of 2 shoulder widths (2 * 1.0).
    position.move_foot(
        Foot::Right,
        Direction::Y(2.0 * stance.width),
        -Angle::Degree90.radians(),
    );
}

/// Step 4 includes:
/// 1 - Step with the left foot to a left walking stance.
/// 1.1 - A move in positive x-direction.
fn step_4(position: &mut Position) {
    let stance = Stance::GunnunSogi.resolve(&position.measurement);

    // 1 - Step to right walking stance.
    // 1.1 - We are moving the right foot a total of 3 shoulder widths (2 * 1.5).
    position.move_foot(
        Foot::Left,
        Direction::X(2.0 * stance.length),
        Angle::Degree0.radians(),
    );

    // COMMENT - No offset in y-direction since we keep the shoulder width distance.
}

/// Step 5 includes:
/// 1.1 - 90 degree left turn with the left foot
/// 1.2 - Step with left foot to a left walking stance
/// 	1.2.1 - A move in negative x-direction
///		1.2.2 - A move in positive y-direction of 1.5 shoulders MINUS 1.0 widths
fn step_5(position: &mut Position) {
    let stance = Stance::GunnunSogi.resolve(&position.measurement);

    // 1.1 - Turn 90 degrees to the left.
    position.rotate(Rotation::Left, Angle::Degree90);

    // 1.2 - Left walking stance

    // 1.2.1 - Move left foot in x-direction
    //
    // COMMENT - We move 1.5 shoulders "in" and then another 1.0 shoulders.
    position.move_foot(
        Foot::Left,
        Direction::X(stance.length + stance.width),
        Angle::Degree90.radians(),
    );

    // 1.2.2 - Move left foot in y-direction.
    //
    // COMMENT - The actual distance we move is 1.5 shoulders - 1.0 shoulders.
    position.move_foot(
        Foot::Left,
        Direction::Y(stance.length - stance.width),
        Angle::Degree0.radians(),
    );
}
