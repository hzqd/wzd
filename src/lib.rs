use aoko::no_std::algebraic::sum::TimeUnit;
use clap::Parser;

/// A CLI tool for write zeros to disk

#[derive(Parser)]
#[clap(version = "0.0.2", author = "hzqd <hzqelf@yeah.net>")]
pub struct Args {
    /// Specify the ?GB data to disk
    #[clap()]
    pub size: String,

    /// Specify the output file name
    #[clap(short, long, default_value = "occupied")]
    pub output: String,

    /// Specify the time unit, support nanos, micros, millis, secs
    #[clap(short, long, default_value = "millis")]
    pub time: TimeUnit,
}

pub fn get_args() -> Args {
    Args::parse()
}