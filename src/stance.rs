use crate::distance::Distance;

pub struct StanceSpec {
    pub length: Distance,
    pub width: Distance,
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
    pub fn spec(&self) -> StanceSpec {
        match self {
            Self::MoaSogi => StanceSpec {
                length: Distance::ShoulderWidth(0.0),
                width: Distance::ShoulderWidth(0.0),
            },
            Self::NaraniSogi => StanceSpec {
                length: Distance::ShoulderWidth(0.0),
                width: Distance::ShoulderWidth(1.0),
            },
            Self::AnnunSogi => StanceSpec {
                length: Distance::ShoulderWidth(0.0),
                width: Distance::ShoulderWidth(1.5),
            },
            Self::GunnunSogi => StanceSpec {
                length: Distance::ShoulderWidth(1.5),
                width: Distance::ShoulderWidth(1.0),
            },
            Self::NiunjaSogi => StanceSpec {
                length: Distance::ShoulderWidth(1.5),
                width: Distance::ShoulderWidth(0.0),
            },
            Self::GojungSogi => StanceSpec {
                length: Distance::ShoulderWidth(1.5),
                width: Distance::ShoulderWidth(0.0),
            },
            Self::SoojikSogi => StanceSpec {
                length: Distance::ShoulderWidth(1.0),
                width: Distance::ShoulderWidth(0.0),
            },
            Self::DwitbalSogi => StanceSpec {
                length: Distance::ShoulderWidth(1.0),
                width: Distance::ShoulderWidth(0.0),
            },
            // Not sure about this one.
            Self::KyochaSogiFront => StanceSpec {
                length: Distance::FootLength(0.5),
                width: Distance::ShoulderWidth(1.0),
            },
            // Not sure about this one.
            Self::KyochaSogi45 => StanceSpec {
                length: Distance::FootLength(0.5),
                width: Distance::ShoulderWidth(1.0),
            },
        }
    }
}
