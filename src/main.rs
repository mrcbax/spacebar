#[macro_use]
extern crate clap;

pub mod database;
pub mod generator;
pub mod clipboard;
pub mod parser;

use std::env;

use log::*;
use rusqlite::Connection;

fn main() {
    env::set_var("RUST_LOG", "spacebar=info");
    env_logger::init();
    let conn = match database::connect("./.sbdb/spacebars.sqlite") {
        Some(s) => s,
        None => std::process::exit(1),
    };
    println!("Hello, world!");
}
