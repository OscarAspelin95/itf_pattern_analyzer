use std::f64::consts::PI;

pub enum Rotation {
    Left,
    Right,
}

pub enum Angle {
    Degree45,
    Degree90,
    Degree180,
    Degree270,
    Degree360,
}

impl Angle {
    pub fn radians(&self) -> f64 {
        match self {
            Angle::Degree45 => PI / 4.0,
            Angle::Degree90 => PI / 2.0,
            Angle::Degree180 => PI,
            Angle::Degree270 => PI * 3.0 / 2.0,
            Angle::Degree360 => PI * 2.0,
        }
    }

    pub fn resolve_x(&self) -> f64 {
        self.radians().cos().round()
    }

    pub fn resolve_y(&self) -> f64 {
        self.radians().sin().round()
    }
}
