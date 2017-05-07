extern crate notify;
extern crate globset;
extern crate timer;
extern crate chrono;

mod super_runner;



fn main() {
    let runner = super_runner::SuperRunner::new();
    runner.start();
}
