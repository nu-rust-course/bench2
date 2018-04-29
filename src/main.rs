extern crate bench2;

use bench2::{Bench2, secs_micros::SecsMicros};

use std::env::args;

fn main() {
    let mut args = args();
    args.next();
    let bench = process_args(args);

    bench.build_release().unwrap();

    let timing = bench.time_subject().unwrap();

    println!("{}", SecsMicros(timing));
}

fn process_args<I: Iterator<Item = String>>(args: I) -> Bench2 {
    let mut bench = Bench2::new();

    bench.args(args);
    bench.run_iters(8);
    bench.add_input_str("h\nhe\nhel\nhell\nhello\nhelloo\nhellooo\n");

    bench
}
