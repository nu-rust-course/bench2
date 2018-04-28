extern crate bench2;

use bench2::secs_micros::*;
use bench2::*;

use std::env::args;

fn main() {
    let relayed_argument = args().nth(1)
        .expect(&format!("Usage: {} ARGUMENT", prog_name()));

    build_release();

    let timing = run_with_arg_and_input(&relayed_argument, "");

    println!("{}", SecsMicros(timing));
}

fn prog_name() -> String {
    args().next().unwrap_or_else(|| "bench2".to_owned())
}
