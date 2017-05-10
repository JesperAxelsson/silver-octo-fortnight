#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
extern crate chrono;

use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;

use std::time::Duration;
// use std::thread;
use std::process;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use timer::{Timer, Guard};

use notify::{Watcher, RecursiveMode, watcher};
use notify::DebouncedEvent::{Write, Create};

use globset::GlobSet;


pub struct SuperRunner {
    path: PathBuf,
    func: Arc<Fn() -> ()>
}

impl SuperRunner {
    pub fn new(path: &str) -> Self {
        SuperRunner {
            path: PathBuf::from(path),
            func: Arc::new(|| { println!("Hello");})
        }
    }

    fn test_func<F>(&self, func: &F)
        where F: Fn() -> () {
        func();
    }


    pub fn start(&self) {
        // thread::spawn(runner);
        // let ff = || {build_thread(func)};

        self.test_func(&|| {});

        let f= || {
                build_thread();
            };
        self.runner(f);
        // self.runner(self.test_func);
    }

     fn runner<'a, F>(&self, func: F)
            where F: 'static + Send + Sync + Fn() -> () {

        let glob_set = self.compile_blacklist();

        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_secs(0)).unwrap();
        watcher.watch(self.path.clone(), RecursiveMode::Recursive).unwrap();

        // let gg = std::thread::scoped(|| {

        // });

        // let f = self.func.clone();
        // let ff = ||{ (f)(); };

        let func_a = Arc::new(Mutex::new(func));
        // let f_clone = func.clone();
        // let f_ref: & F = func.as_ref();

    {

        let timer = Timer::new();
        let mut guard: Option<Guard> = None;


        // timer.schedule_with_delay(chrono::Duration::seconds(3), f_ref);

        // let (trex, rrex) = channel();


        loop {

            let evt = rx.recv().expect("Failed to recieve event");
            match evt {
                Write(ref path) | Create(ref path) => {

                    if !glob_set.is_match(path) {
                        // println!("Happen: {:?}", evt);

                        if let Some(g) = guard {
                            drop(g);
                        }

                        let f_clone = func_a.clone();
                        guard = Some(timer.schedule_with_delay(chrono::Duration::seconds(2), move || {
                            let f = (*f_clone).lock().unwrap();
                            f();
                        } ));
                        // guard = Some(timer.schedule_with_delay(chrono::Duration::seconds(2), func ));
                        // guard = Some(timer.schedule_with_delay(chrono::Duration::seconds(2), build_thread ));
                        // guard = Some( timer.schedule_with_delay(chrono::Duration::seconds(3), self.func));

                    }
                }
                _ => {}
            }
        }
    }
    }

    fn compile_blacklist(&self) -> GlobSet {
        use globset::{Glob, GlobSetBuilder};

        let black_list = ["*/.git*", "*/target/*", "*/.vscode/*"];

        let mut glob = GlobSetBuilder::new();

        for b in black_list.iter() {
            glob.add(Glob::new(b).expect("Failed to create glob!"));
        }

        let glob_set = glob.build().expect("Failed to build globset");

        glob_set
    }
}


fn build_thread() {
    println!("Starting build");
    let output  = process::Command::new("cmd").args(&["/C", "cargo check"]).output().expect("Failed to launch build");

    let mut build_msg: &str = &String::from_utf8_lossy(&output.stderr).to_owned();
    if output.status.success() && !build_msg.contains("warning") {
        build_msg = "A OK";
    }

    println!("Building! \n{}\n", build_msg);
}



// fn test_func(tx: Sender<u32>) {

// }
