use validator::Validate;

/// All kinds of distances we allow in the pattern.
#[derive(Debug)]
pub enum Distance {
    ShoulderWidth(f64),
    FootLength(f64),
    FootWidth(f64),
}

impl Distance {
    /// Convert relative to actual distance for a person.
    pub fn resolve(&self, m: &Measurement) -> f64 {
        match self {
            Distance::ShoulderWidth(n) => n * m.shoulder_width,
            Distance::FootLength(n) => n * m.foot_length,
            Distance::FootWidth(n) => n * m.foot_width,
        }
    }
}

/// Measurements for a person.
#[derive(Debug, Validate, Clone)]
pub struct Measurement {
    #[validate(range(min = 0.0))]
    pub shoulder_width: f64,
    #[validate(range(min = 0.0))]
    pub foot_length: f64,
    #[validate(range(min = 0.0))]
    pub foot_width: f64,
}

/// How far from the start spot one is allowed to end at from.
/// Measurement must be in centimeters.
#[derive(Debug)]
pub struct Margin {
    pub x: f64,
    pub y: f64,
}

#[cfg(test)]
pub mod test {
    use super::*;
    use rstest::*;

    #[rstest::fixture]
    fn measurement() -> Measurement {
        Measurement {
            shoulder_width: 50.0,
            foot_length: 30.0,
            foot_width: 10.0,
        }
    }

    #[rstest]
    #[case(Distance::ShoulderWidth(1.0), 50.0)]
    #[case(Distance::ShoulderWidth(1.5), 75.0)]
    #[case(Distance::FootLength(0.5), 15.0)]
    fn test_resolve(#[case] distance: Distance, measurement: Measurement, #[case] expected: f64) {
        assert_eq!(distance.resolve(&measurement), expected);
    }
}
