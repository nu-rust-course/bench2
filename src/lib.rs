use std::io::Write;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

pub mod secs_micros;

const N: usize = 10;
const M: usize = 1;

pub fn build_release() {
    run_command(Command::new("cargo")
        .arg("build")
        .arg("--release"));
}

pub fn run_with_arg_and_input(corpus: &str, input: &str)
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

pub fn run_command(cmd: &mut Command) {
    assert!(cmd.status().unwrap().success(),
            format!("{:?}", cmd));
}

