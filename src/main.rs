#[macro_use]
extern crate clap;

pub mod database;
pub mod generator;
pub mod clipboard;
pub mod parser;

use std::env;

use clap::{App, Arg, SubCommand};
use log::*;
use rusqlite::Connection;

fn main() {
    env::set_var("RUST_LOG", "spacebar=info");
    env_logger::init();

    let matches = App::new("Spacebar Tagging System")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(SubCommand::with_name("init")
                    .help("Initialize a new spacebar database in the current directory"))
        .subcommand(SubCommand::with_name("new")
                    .help("Create a new spacebar")
                    .arg(Arg::with_name("name")
                         .short("n")
                         .long("name")
                         .value_name("STRING")
                         .help("A name for the new spacebar")
                         .required(true)
                         .takes_value(true))
                    .arg(Arg::with_name("description")
                         .short("d")
                         .long("description")
                         .value_name("STRING")
                         .help("A description of the new spacebar")
                         .required(true)
                         .takes_value(true))
        )
        .subcommand(SubCommand::with_name("search")
                    .help("Search for a spacebar at a given location")
                    .arg(Arg::with_name("clipboard")
                         .short("c")
                         .long("clipboard")
                         .help("Search the system clipboard for a spacebar")
                         .required(false)
                         .takes_value(false))
                    .arg(Arg::with_name("clipboard_fuzzy")
                         .short("z")
                         .long("clipboard_fuzzy")
                         .help("Search for a spacebar using a spacebar segment from the clipboard.")
                         .required(false)
                         .takes_value(false))
                    .arg(Arg::with_name("web")
                         .short("w")
                         .long("website")
                         .value_name("URL")
                         .help("Scrape the page of the given URL for a spacebar")
                         .required(false)
                         .takes_value(true))
                    .arg(Arg::with_name("file")
                         .short("f")
                         .long("file")
                         .value_name("FILE_PATH")
                         .help("Search the given file for a spacebar")
                         .required(false)
                         .takes_value(true))
        )
        .subcommand(SubCommand::with_name("show")
        .help("Show spacebars in database."))
        .get_matches();

    let conn = match database::connect("./.sbdb/spacebars.sqlite") {
        Some(s) => s,
        None => std::process::exit(1),
    };
    println!("Hello, world!");
}
