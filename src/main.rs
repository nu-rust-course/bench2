extern crate bench2;

use bench2::{Bench2, /*secs_micros::SecsMicros*/};

use std::env::args;

fn main() {
    let _relayed_argument = args().nth(1)
        .expect(&format!("Usage: {} ARGUMENT", prog_name()));

    let mut bench = Bench2::new();
    bench.run_iters(8);

    bench.build_release().unwrap();

//    let timing = run_with_arg_and_input(&relayed_argument, "");

//    println!("{}", SecsMicros(timing));
}

fn prog_name() -> String {
    args().next().unwrap_or_else(|| "bench2".to_owned())
}
