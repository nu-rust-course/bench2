use std::env::args;
use std::fmt;
use std::io::Write;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

const N: usize = 10;
const M: usize = 1;
const INPUT: &'static str = "h\nhe\nhel\nhell\nhello\nhelloo\nhellooo\n";

#[derive(Copy, Clone)]
struct SecsMicros(Duration);

fn main() {
    let corpus = args().nth(1).expect("Usage: cargo run corpus.txt");

    run_command(Command::new("cargo")
                    .arg("build")
                    .arg("--release"));

    let build = run_corrector_with_input(&corpus, "");
//    let correct = run_corrector_with_input(&corpus, INPUT);
//    let diff = if build < correct {correct - build} else {build - build};

    println!("{}", SecsMicros(build));
}

impl fmt::Display for SecsMicros {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{:06}", self.0.as_secs(), self.0.subsec_nanos() / 1000)
    }
}

fn run_corrector_with_input(corpus: &str, input: &str)
    -> Duration
{
    time_command_with_input(
        Command::new("cargo")
            .arg("run")
            .arg("--release")
            .arg(&corpus),
        input)
}

fn time_command_with_input(cmd: &mut Command, input: &str)
    -> Duration
{
    cmd.stdin(Stdio::piped());

    run_command_with_input(cmd, input);

    let start = Instant::now();

    for _ in 0 .. N {
        run_command_with_input(cmd, input);
    }

    start.elapsed() / N as u32
}

fn run_command_with_input(cmd: &mut Command, input: &str) {
    let mut child = cmd.spawn().unwrap();
    for _ in 0 .. M {
        write!(child.stdin.as_mut().unwrap(), "{}", input).unwrap();
    }
    child.wait().unwrap();
}

fn run_command(cmd: &mut Command) {
    assert!(cmd.status().unwrap().success(),
            format!("{:?}", cmd));
}

