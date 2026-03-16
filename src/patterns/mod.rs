mod chon_ji;
pub mod utils;

use utils::Patterns;

use crate::{
    distance::{Margin, Measurement},
    position::{Position, StartPosition},
};

pub fn analyze_pattern(pattern: Patterns, measurement: Measurement) {
    let coordinate = Position::new(measurement, StartPosition::ShoulderWidth);

    // margin is always one shoulder width (in cm) in x and y axis respectively.
    let margin = Margin { x: 0.0, y: 0.0 };

    match pattern {
        Patterns::ChonJi => chon_ji::analyze_chon_ji(coordinate, margin),
        _ => panic!("Pattern {:?} not supported", pattern),
    }
}
