use crate::{
    direction::{Angle, Rotation},
    distance::{Margin, Measurement},
    position::{Position, StartPosition},
    stance::Stance,
};

pub fn analyze_chon_ji(measurement: Measurement) {
    let mut position = Position::new(measurement, StartPosition::ShoulderWidth);

    step_1(&mut position);
    step_2(&mut position);
}

/// Step 1 includes:
/// * 90 degree turn to the left with left foot.
/// * step out in walking stance with left foot.
fn step_1(position: &mut Position) {
    position.rotate(Rotation::Left, Angle::Degree90);
}

fn step_2(position: &mut Position) {
    position.rotate(Rotation::Left, Angle::Degree90);
}
