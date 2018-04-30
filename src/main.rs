extern crate bench2;
extern crate foropts;

use bench2::{Bench2, secs_micros::SecsMicros};

use std::{
    env::args,
    process::exit,
};

fn main() {
    let bench = process_args(args());

    bench.build_release().unwrap();

    let timing = bench.time_subject().unwrap();

    println!("{}", SecsMicros(timing));
}

fn process_args<I: Iterator<Item = String>>(mut args: I) -> Bench2 {
    use foropts::{Arg, Config};

    enum Opt {
        InputString(String),
        InputFile(String),
        RunIters(u32),
        InputIters(u32),
        IncreaseVerbosity,
        Positional(String),
    }

    let progname = args.next().unwrap_or_else(|| "bench2".to_owned());

    let config = Config::new(progname)
        .version(env!("CARGO_PKG_VERSION"))
        .author("Jesse A. Tov <jesse.tov@gmail.com>")
        .about("a simple whole-program Rust bencher")
        .arg(Arg::flag(|| Opt::IncreaseVerbosity)
            .short('v').long("verbose"))
        .arg(Arg::parsed_param("INPUT", Opt::InputString)
            .short('i').long("input"))
        .arg(Arg::parsed_param("FILE", Opt::InputFile)
            .short('f').long("input-file"))
        .arg(Arg::parsed_param("ITERS", Opt::RunIters)
            .short('n').long("run-iters"))
        .arg(Arg::parsed_param("ITERS", Opt::InputIters)
            .short('m').long("input-iters"))
        .arg(Arg::parsed_param("ARG", Opt::Positional));

    let mut verbosity = 0;
    let mut result = Bench2::new();

    for opt in config.iter(args) {
        let opt = opt.unwrap_or_else(|e| {
            eprintln!("{}", e);
            exit(1);
        });

        match opt {
            Opt::InputString(i)    => { result.add_input_str(&i); }
            Opt::InputFile(f)      => {
                result.add_input_file(&f).unwrap_or_else(|e| {
                    eprintln!("Could not read input file: {}: {}", f, e);
                    exit(2);
                });
            },
            Opt::RunIters(n)       => { result.run_iters(n); }
            Opt::InputIters(m)     => { result.input_iters(m); },
            Opt::IncreaseVerbosity => { verbosity += 1; },
            Opt::Positional(s)     => { result.arg(s); }
        }
    }

    result.verbosity(verbosity);

    result
}
