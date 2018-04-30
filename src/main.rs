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
        DisplayUsage,
        DisplayVersion,
        Positional(String),
    }

    let progname = args.next().unwrap_or_else(|| "bench2".to_owned());

    let config = Config::new(progname)
        .version(env!("CARGO_PKG_VERSION"))
        .author("Jesse A. Tov <jesse.tov@gmail.com>")
        .about("a simple whole-program Rust bencher")
        .arg(Arg::flag(|| Opt::IncreaseVerbosity)
            .short('v').long("verbose")
            .description("increases verbosity"))
        .arg(Arg::parsed_param("INPUT", Opt::InputString)
            .short('i').long("input")
            .description("input string to send to program under test"))
        .arg(Arg::parsed_param("FILE", Opt::InputFile)
            .short('f').long("input-file")
            .description("input file to send to program under test"))
        .arg(Arg::parsed_param("ITERS", Opt::RunIters)
            .short('n').long("run-iters")
            .description("number of times to run the program"))
        .arg(Arg::parsed_param("ITERS", Opt::InputIters)
            .short('m').long("input-iters")
            .description("number of times to repeat the input"))
        .arg(Arg::flag(|| Opt::DisplayUsage)
            .short('h').long("help")
            .description("displays this help message"))
        .arg(Arg::flag(|| Opt::DisplayVersion)
            .short('V').long("version")
            .description("displays version information"))
        .arg(Arg::parsed_param("ARG", Opt::Positional)
            .description("passed on to the program under test"));

    let mut verbosity = 0;
    let mut result = Bench2::new();

    for opt in config.iter(args) {
        match opt.unwrap_or_else(|e| config.exit_error(&e)) {
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
            Opt::DisplayUsage      => { config.exit_usage(); }
            Opt::DisplayVersion    => { config.exit_version(); }
            Opt::Positional(s)     => { result.arg(s); }
        }
    }

    result.verbosity(verbosity);

    result
}
