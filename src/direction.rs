use std::f64::consts::PI;

pub enum Rotation {
    // Anti-clockwise (increasing the angle)
    Left,
    // Clockwise (decreasing the angle)
    Right,
}

#[derive(Debug)]
pub enum Foot {
    Left,
    Right,
}

#[derive(Debug)]
pub enum Direction {
    X(f64),
    Y(f64),
}

pub enum Angle {
    Degree0,
    Degree45,
    Degree90,
    Degree180,
    Degree270,
    Degree360,
    DegreeCustom(f64),
}

impl Angle {
    pub fn radians(&self) -> f64 {
        match self {
            Angle::Degree0 => 0.0,
            Angle::Degree45 => PI / 4.0,
            Angle::Degree90 => PI / 2.0,
            Angle::Degree180 => PI,
            Angle::Degree270 => PI * 3.0 / 2.0,
            Angle::Degree360 => PI * 2.0,
            Angle::DegreeCustom(custom_angle_degree) => custom_angle_degree * (PI / 180.0),
        }
    }

    pub fn resolve_x(&self) -> f64 {
        self.radians().cos().round()
    }

    pub fn resolve_y(&self) -> f64 {
        self.radians().sin().round()
    }
}
