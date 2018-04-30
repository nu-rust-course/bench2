extern crate bench2;

use bench2::{Bench2, secs_micros::SecsMicros};

use std::env::args;

fn main() {
    let bench = process_args(args());

    bench.build_release().unwrap();

    let timing = bench.time_subject().unwrap();

    println!("{}", SecsMicros(timing));
}

fn process_args<I: Iterator<Item = String>>(mut args: I) -> Bench2 {
    args.next(); // Discard program name

    let mut result = Bench2::new();

    while let Some(arg) = args.next() {
        if let Some(param) = accept_arg(&arg, &mut args, "-i", "--input") {
            result.add_input_str(&param);
        } else if let Some(param) = accept_arg(&arg, &mut args, "-f", "--input-file") {
            result.add_input_file(&param)
                .expect("Could not read input file");
        } else if let Some(param) = accept_arg(&arg, &mut args, "-n", "--run-iters") {
            result.run_iters(param.parse().expect("Could not parse --run-iters parameter"));
        } else if let Some(param) = accept_arg(&arg, &mut args, "-m", "--input-iters") {
            result.input_iters(param.parse().expect("Could not parse --input-iters parameter"));
        } else if arg == "-v" {
            result.inc_verbosity();
        } else if arg == "--" {
            break;
        } else {
            result.arg(arg);
        }
    }

    result.args(args);

    result
}

fn accept_arg<I>(arg: &str, rest: &mut I, short: &str, long: &str) -> Option<String>
    where I: Iterator<Item=String>
{
    if arg == short {
        Some(rest.next().expect(&format!("{} option requires parameter", short)).to_owned())
    } else if arg == long {
        Some(rest.next().expect(&format!("{} option requires parameter", long)).to_owned())
    } else if let Some(param) = strip_prefix(arg, short) {
        Some(param.to_owned())
    } else if let Some(param) = strip_prefix(arg, &format!("{}=", long)) {
        Some(param.to_owned())
    } else {
        None
    }
}

fn strip_prefix<'a>(haystack: &'a str, needle: &str) -> Option<&'a str> {
    if haystack.len() < needle.len() {return None}

    let (before, after) = haystack.split_at(needle.len());
    if before == needle {
        Some(after)
    } else {None}
}
