extern crate bench2;

use bench2::{Bench2, secs_micros::SecsMicros};

use std::env::args;

fn main() {
    let relayed_argument = args().nth(1)
        .expect(&format!("Usage: {} ARGUMENT", prog_name()));

    let mut bench = Bench2::new();
    bench.arg(relayed_argument);
    bench.run_iters(8);
    bench.add_input_str("h\nhe\nhel\nhell\nhello\nhelloo\nhellooo\n");

    bench.build_release().unwrap();

    let timing = bench.time_subject().unwrap();

    println!("{}", SecsMicros(timing));
}

fn prog_name() -> String {
    args().next().unwrap_or_else(|| "bench2".to_owned())
}
