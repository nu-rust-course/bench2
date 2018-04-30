# bench2: a simple whole-program Rust bencher

`bench2` is for timing runs of whole Rust programs via `cargo run --release`.
It builds the program via `cargo`, times some number of runs, and returns the
average time. It can also be configured to send text to the program's 
standard input, and by default hids the program's standard output.

More documentation is needed.