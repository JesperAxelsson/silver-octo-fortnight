extern crate chrono;

use std::sync::mpsc::channel;
use std::time::Duration;
// use std::thread;
use std::process;

use timer::{Timer, Guard};

use notify::{Watcher, RecursiveMode, watcher};
use notify::DebouncedEvent::{Write, Create};

use globset::GlobSet;


pub struct SuperRunner {}

impl SuperRunner{
    pub fn new() -> Self {
        SuperRunner{}
    }

    pub fn start(&self) {
        // thread::spawn(runner);
        self.runner();
    }

     fn runner(&self) {

        let glob_set = compile_blacklist();

        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_secs(0)).unwrap();
        watcher.watch(".", RecursiveMode::Recursive).unwrap();

        let timer = Timer::new();
        let mut guard: Option<Guard> = None;

        loop {
            let evt = rx.recv().expect("Failed to recieve event");
            match evt {
                Write(ref path) | Create(ref path) => {

                    if !glob_set.is_match(path) {
                        // println!("Happen: {:?}", evt);

                        if let Some(g) = guard {
                            drop(g);
                        }

                        guard = Some(timer.schedule_with_delay(chrono::Duration::seconds(5), build_thread ));

                    }
                }
                _ => {}
            }
        }
    }
}


fn build_thread() {
    println!("Starting build");
    let output  = process::Command::new("cmd").args(&["/C", "cargo check"]).output().expect("Failed to launch build");

// println!("status: {}", output.status);
// println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
// println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    let build_msg;
    if output.status.success() {
        build_msg = String::from("A OK");
    } else {
        build_msg = String::from_utf8_lossy(&output.stderr).into_owned();
    }

    println!("Building! (Not) {}", build_msg);
}


fn compile_blacklist() -> GlobSet {
    use globset::{Glob, GlobSetBuilder};

    let black_list = ["*/.git*", "*/target/*", "*/.vscode/*"];

    let mut glob = GlobSetBuilder::new();

    for b in black_list.iter() {
        glob.add(Glob::new(b).expect("Failed to create glob!"));
    }

    let glob_set = glob.build().expect("Failed to build globset");

    glob_set
}
