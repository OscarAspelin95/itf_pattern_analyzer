mod chon_ji;
pub mod utils;

use utils::Patterns;

use crate::distance::Measurement;

pub fn analyze_pattern(pattern: Patterns, measurement: Measurement) {
    match pattern {
        Patterns::ChonJi => chon_ji::analyze_chon_ji(measurement),
        _ => panic!("Pattern {:?} not supported", pattern),
    }
}
