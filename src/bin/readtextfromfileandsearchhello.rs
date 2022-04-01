extern crate rust_practice;
use rust_practice::closure::run;
use rust_practice::closure::Config;
use std::env;
use std::process;

fn main() {
    let args:Vec<String> = env::args().collect();

    let cfg = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("err {}", err);
        process::exit(1);}
    );

    if let Err(e) = run(cfg) {
            eprintln!("err {}", e);
            process::exit(1);
    }
}
