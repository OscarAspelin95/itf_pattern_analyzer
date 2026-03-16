mod args;
mod direction;
mod distance;
mod patterns;
mod position;
mod stance;

use args::Args;
use clap::Parser;
use patterns::analyze_pattern;

fn main() {
    let args = Args::parse();

    let (pattern, measurement) = args.destructure();
    analyze_pattern(pattern, measurement);
}
