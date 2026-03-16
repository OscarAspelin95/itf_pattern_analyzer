use crate::distance::{Distance, Measurement};

/// Theoretical, relative length and width.
pub struct StanceSpecTheoretical {
    pub length: Distance,
    pub width: Distance,
}

impl StanceSpecTheoretical {
    fn resolve_length(&self, measurement: &Measurement) -> f64 {
        match self.length {
            Distance::ShoulderWidth(dist) => dist * measurement.shoulder_width,
            Distance::FootLength(dist) => dist * measurement.foot_length,
            Distance::FootWidth(dist) => dist * measurement.foot_width,
        }
    }
    fn resolve_width(&self, measurement: &Measurement) -> f64 {
        match self.width {
            Distance::ShoulderWidth(dist) => dist * measurement.shoulder_width,
            Distance::FootLength(dist) => dist * measurement.foot_length,
            Distance::FootWidth(dist) => dist * measurement.foot_width,
        }
    }

    pub fn resolve(&self, measurement: &Measurement) -> StanceSpec {
        StanceSpec {
            length: self.resolve_length(measurement),
            width: self.resolve_width(measurement),
        }
    }
}

// Actual length and width in cm for a specific person.
pub struct StanceSpec {
    pub length: f64,
    pub width: f64,
}

pub enum Stance {
    MoaSogi,
    NaraniSogi,
    AnnunSogi,
    GunnunSogi,
    NiunjaSogi,
    GojungSogi,
    SoojikSogi,
    DwitbalSogi,
    KyochaSogiFront,
    KyochaSogi45,
}

impl Stance {
    pub fn resolve(&self, measurement: &Measurement) -> StanceSpec {
        match self {
            Self::MoaSogi => StanceSpecTheoretical {
                length: Distance::ShoulderWidth(0.0),
                width: Distance::ShoulderWidth(0.0),
            }
            .resolve(measurement),
            Self::NaraniSogi => StanceSpecTheoretical {
                length: Distance::ShoulderWidth(0.0),
                width: Distance::ShoulderWidth(1.0),
            }
            .resolve(measurement),
            Self::AnnunSogi => StanceSpecTheoretical {
                length: Distance::ShoulderWidth(0.0),
                width: Distance::ShoulderWidth(1.5),
            }
            .resolve(measurement),
            Self::GunnunSogi => StanceSpecTheoretical {
                length: Distance::ShoulderWidth(1.5),
                width: Distance::ShoulderWidth(1.0),
            }
            .resolve(measurement),
            Self::NiunjaSogi => StanceSpecTheoretical {
                length: Distance::ShoulderWidth(1.5),
                width: Distance::ShoulderWidth(0.0),
            }
            .resolve(measurement),
            Self::GojungSogi => StanceSpecTheoretical {
                length: Distance::ShoulderWidth(1.5),
                width: Distance::ShoulderWidth(0.0),
            }
            .resolve(measurement),
            Self::SoojikSogi => StanceSpecTheoretical {
                length: Distance::ShoulderWidth(1.0),
                width: Distance::ShoulderWidth(0.0),
            }
            .resolve(measurement),
            Self::DwitbalSogi => StanceSpecTheoretical {
                length: Distance::ShoulderWidth(1.0),
                width: Distance::ShoulderWidth(0.0),
            }
            .resolve(measurement),
            // Not sure about this one.
            Self::KyochaSogiFront => StanceSpecTheoretical {
                length: Distance::FootLength(0.5),
                width: Distance::ShoulderWidth(1.0),
            }
            .resolve(measurement),
            // Not sure about this one.
            Self::KyochaSogi45 => StanceSpecTheoretical {
                length: Distance::FootLength(0.5),
                width: Distance::ShoulderWidth(1.0),
            }
            .resolve(measurement),
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::distance::Measurement;
    use rstest::*;

    #[rstest::fixture]
    fn measurement() -> Measurement {
        Measurement {
            shoulder_width: 100.0,
            foot_length: 30.0,
            foot_width: 10.0,
        }
    }

    #[rstest]
    #[case(Stance::GunnunSogi)]
    pub fn test_stances(measurement: Measurement, #[case] stance: Stance) {
        let stance_spec = stance.resolve(&measurement);

        assert_eq!(stance_spec.length, 150.0);
        assert_eq!(stance_spec.width, 100.0);
    }
}
