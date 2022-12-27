use std::{fs, time::Duration};
use aoko::{standard::functions::fun::{time_conversion_with_unit, measure_time_with_value}, no_std::{algebraic::sum::TimeUnit, pipelines::pipe::Pipe}};
use wzd::get_args;

fn wzd() -> (impl FnOnce(Duration) -> u128, TimeUnit) {
    let args = get_args();
    let n = args.size.parse::<usize>().unwrap() * 1024 * 1024 * 1024;
    let mut v: Vec<u8> = Vec::with_capacity(n);
    for _ in 0..n {
        v.push(0)
    }
    fs::write(args.output, v).unwrap();
    time_conversion_with_unit(args.time)
}

fn main() {
    measure_time_with_value(wzd)
        .pipe(|(e, (f, u))| println!("Execution time: {} {u:?}.", f(e)));
}