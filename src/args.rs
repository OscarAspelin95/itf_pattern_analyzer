use clap::Parser;

use crate::{distance::Measurement, patterns::utils::Patterns};

#[derive(Debug, Parser)]
#[command(version, about, long_about=None)]
pub struct Args {
    #[arg(long)]
    pub shoulder_width: f64,
    #[arg(long)]
    pub foot_width: f64,
    #[arg(long)]
    pub foot_length: f64,
    #[arg(long, value_enum)]
    pub pattern: Patterns,
}

impl Args {
    pub fn destructure(self) -> (Patterns, Measurement) {
        let measurement = Measurement {
            shoulder_width: self.shoulder_width,
            foot_length: self.foot_length,
            foot_width: self.foot_width,
        };

        (self.pattern, measurement)
    }
}
