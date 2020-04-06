#[macro_use]
extern crate clap;

pub mod database;
pub mod generator;
pub mod clipboard;
pub mod parser;

use std::env;
use std::fs;

use clap::{App, Arg, SubCommand};
use log::*;
use rusqlite::Connection;

fn main() {
    let mut builder = env_logger::Builder::from_default_env();
    builder.format_level(false).format_module_path(false).format_timestamp(None);
    match env::var("RUST_LOG") {
        Ok(_) => builder.init(),
        Err(_) => {
            env::set_var("RUST_LOG", "spacebar=info");
            builder.init();
            debug!("Defaulting to info level logging."); //Schrodinger's dead code.
        },
    }

    let matches = App::new("Spacebar Tagging System")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(SubCommand::with_name("init")
                    .about("Initialize a new spacebar database in the current directory"))
        .subcommand(SubCommand::with_name("new")
                    .about("Create a new spacebar")
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
                         .required(false)
                         .takes_value(true))
        )
        .subcommand(SubCommand::with_name("search")
                    .about("Search for a spacebar")
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
                    .about("Show spacebars in database")
                    .help("Show spacebars in database"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("init") {
        match fs::create_dir("./.sbdb/") {
            Ok(_) => {
                match database::connect("./.sbdb/spacebars.db") {
                    Some(_) => {
                        info!("Initialized successfully.");
                        std::process::exit(0);
                    },
                    None => {
                        error!("Failed to initialize.");
                        std::process::exit(1);
                    }
                }
            },
            Err(e) => {
                error!("Failed to initialize: {}", e);
                std::process::exit(1);
            }
        }
    }

    let conn = match database::connect("./.sbdb/spacebars.db") {
        Some(s) => s,
        None => std::process::exit(1),
    };

    if let Some(matches_new) = matches.subcommand_matches("new") {
        if matches_new.is_present("name") {
            let spacebar: generator::Spacebar;
            debug!("{}", format!("{:b}", spacebar.spacebar));
            if matches_new.is_present("description") {
                spacebar = generator::generate_spacebar(matches_new.value_of("name").unwrap().to_string(), Some(matches_new.value_of("description").unwrap().to_string()));
            } else {
                spacebar = generator::generate_spacebar(matches_new.value_of("name").unwrap().to_string(), None);
            }
            database::insert_spacebar(&conn, spacebar);
        } else {
            error!("You must specify a name for the new spacebar.");
        }
    }

    println!("Hello, world!");
}
