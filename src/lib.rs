use std::fs::File;
use std::io::{self, BufReader, Read, BufWriter, Write, Error, ErrorKind};
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

pub mod secs_micros;

/// Configuration for benching.
#[derive(Debug, Clone)]
pub struct Bench2 {
    run_iters:   u32,
    input_iters: u32,
    arguments:   Vec<String>,
    input:       Option<Vec<u8>>,
}

impl Bench2 {
    /// Creates a new benching configuration with 10 run iterations, 1 input iteration, no
    /// command-line arguments, and input from stdin.
    pub fn new() -> Self {
        Bench2 {
            run_iters:   10,
            input_iters: 1,
            arguments:   Vec::new(),
            input:       None,
        }
    }

    /// Sets the number of times to repeat running the program.
    pub fn run_iters(&mut self, n: u32) -> &mut Self {
        self.run_iters = n;
        self
    }

    /// Sets the number of times to send the specified input to the program.
    pub fn input_iters(&mut self, m: u32) -> &mut Self {
        self.input_iters = m;
        self
    }

    /// Adds an argument to pass to the program under test.
    pub fn arg<S>(&mut self, argument: S) -> &mut Self
        where S: ToOwned<Owned = String>,
              String: std::borrow::Borrow<S>
    {
        self.arguments.push(argument.to_owned());
        self
    }

    /// Adds a sequence of arguments to pass to the program under test.
    pub fn args<S, I>(&mut self, arguments: I) -> &mut Self
        where S: ToOwned<Owned = String>,
              String: std::borrow::Borrow<S>,
              I: IntoIterator<Item = S>
    {
        for argument in arguments {
            self.arg(argument);
        }

        self
    }

    fn input_mut(&mut self) -> &mut Vec<u8> {
        self.input.get_or_insert_with(Vec::new)
    }

    /// Adds the given string to the input to send to the program under test.
    pub fn add_input_str(&mut self, input: &str) -> &mut Self {
        self.input_mut().extend(input.bytes());
        self
    }

    /// Adds the contents of the given file to the input to send to the program under test.
    pub fn add_input_file<P: AsRef<Path>>(&mut self, path: P) -> io::Result<&mut Self> {
        let mut file = BufReader::new(File::open(path)?);
        file.read_to_end(self.input_mut())?;
        Ok(self)
    }

    /// Attempts to run `cargo build --release`.
    pub fn build_release(&self) -> io::Result<()> {
        let mut command = Command::new("cargo");
        command.arg("build").arg("--release");

        if command.status()?.success() {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "Could not build"))
        }
    }

    /// Times some number of runs of `cargo run --release`.
    pub fn time_subject(&self) -> io::Result<Duration> {
        let mut command = Command::new("cargo");
        command.arg("run").arg("--release").arg("--").args(&self.arguments);

        // We want to prepare to pipe input, but only if we're doing that.
        if self.input.is_some() {
            command.stdin(Stdio::piped());
        }

        self.time_command(&mut command)
    }

    fn time_command(&self, cmd: &mut Command) -> io::Result<Duration> {
        // One warmup run.
        self.run_command_once(cmd)?;

        let start = Instant::now();

        for _ in 0 .. self.run_iters {
            self.run_command_once(cmd)?;
        }

        Ok(start.elapsed() / self.run_iters)
    }

    fn run_command_once(&self, cmd: &mut Command) -> io::Result<()> {
        let mut child = cmd.spawn()?;

        if let Some(ref input) = self.input {
            let mut child_stdin = BufWriter::new(child.stdin.as_mut()
                .expect("pipe to child process"));

            for _ in 0 .. self.input_iters {
                child_stdin.write_all(&input)?;
            }
        }

        child.wait()?;

        Ok(())
    }
}

