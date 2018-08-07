#![feature(string_retain)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate clap;

extern crate clipboard;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate spinners;

pub mod board;
pub mod database;
pub mod generator;

use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

use board::*;
use database::*;
use generator::*;

use clap::{App, Arg};
use spinners::{Spinner, Spinners};

pub fn main() {
    let matches = App::new("Spacebar Tagging System")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("database")
             .short("d")
             .long("database")
             .value_name("FILE")
             .help("Path of database to load.")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("username")
             .short("u")
             .long("username")
             .value_name("STRING")
             .help("The user you'd like to log in as.")
             .required(true)
             .takes_value(true))
        .get_matches();
    let db_name: String = String::from(matches.value_of("database").unwrap());
    let mut db: Database = read_database(&db_name);
    let user: String = String::from(matches.value_of("username").unwrap());
    if !find_user_name(&user, &db) {
        println!("User provided in arguments does not exist. Create? [y,N]");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = String::from(input.trim());
        match input.to_lowercase().as_str() {
            "y" => {
                &db.idents.push(generate_barcode(user.clone(), new_user_id(), String::from("Default"), String::from("The default spacebar.")));
                println!("New user {} created successfully.", input);
                println!("------------");
            },
            "n" => {
                panic!("Exiting...")
            },
            _ => panic!("Invalid input. Please re-launch and try again."),
        }
    }
    let mut sentinel: bool = true;
    while sentinel {
        match display_menu().trim() {
            "1" => {
                println!("Input the new username: ");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                input = String::from(input.trim());
                &db.idents.push(generate_barcode(input.clone(), new_user_id(), String::from("Default"), String::from("The default spacebar.")));
                println!("New user {} created successfully.", input);
                println!("------------");
            },
            "2" => {
                println!("Name: ");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                name = String::from(name.trim());
                println!("Short description: ");
                let mut desc = String::new();
                io::stdin().read_line(&mut desc).unwrap();
                desc = String::from(desc.trim());
                for i in 0 .. db.idents.len() {
                    if db.idents[i].user_name == user {
                        let mut old_ident: Identifiers = db.idents.remove(i);
                        &db.idents.push(generate_barcode_from_previous(old_ident, name, desc));
                        break;
                    }
                }
                println!("------------");
            },
            "3" => {
                for ident in &db.idents {
                    if ident.user_name == user {
                        for spc in &ident.spacebars {
                            println!("Name:\t{}\nDescription:\t{}\nSpacebar:\t-->{}<--", &spc.name, &spc.desc, &spc.spacebar);
                            println!("------------");
                        }
                    }
                }
            },
            "4" => {
                println!("Paste a line of text you think has a spacebar: ");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                match lookup_spacebar(input, &db) {
                    Some(e) => {
                        println!("Spacebar found!");
                        println!("Username: {}", e.0.user_name);
                        println!("Spacebar name: {}", e.1.name);
                        println!("Spacebar description: {}", e.1.desc);
                        println!("Spacebar: -->{}<--", e.1.spacebar);
                    },
                    None => {
                        println!("No spacebar found in line.");
                    },
                };
                println!("------------");
            },
            "5" => {
                println!("Enter the path to the file: ");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                input = String::from(input.trim());
                let f = match File::open(input) {
                    Ok(o) => o,
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    },
                };
                let mut file = BufReader::new(&f);
                let sp = Spinner::new(Spinners::Dots9, "Searching...".into());
                let mut results: Vec<(Identifiers, Spacebar, String)> = vec!();
                for line in file.lines() {
                    match line {
                        Ok(e) =>  match lookup_spacebar(e.clone(), &db) {
                            Some(o) => results.push((o.0,o.1,e.clone())),
                            None => {},
                        },
                        Err(_) => {},
                    };

                }
                sp.stop();
                results.dedup_by(|a, b| a.1.spacebar.eq(b.1.spacebar.as_str()));
                for result in results {
                    println!("Spacebar found!");
                    println!("Username: {}", result.0.user_name);
                    println!("Spacebar name: {}", result.1.name);
                    println!("Spacebar description: {}", result.1.desc);
                    println!("Spacebar: -->{}<--", result.1.spacebar);
                    println!("Found in line: {}", result.2);
                }
                println!("------------");
            }
            "6" => {
                match lookup_spacebar(parse_clipboard(), &db) {
                    Some(e) => {
                        println!("Spacebar found!");
                        println!("Username: {}", e.0.user_name);
                        println!("Spacebar name: {}", e.1.name);
                        println!("Spacebar description: {}", e.1.desc);
                        println!("Spacebar: -->{}<--", e.1.spacebar);
                    },
                    None => {
                        println!("No spacebar found in clipboard.");
                    },
                };
                println!("------------");
            }
            "7" => {
                sentinel = false;
            },
            _ => {
                println!("Invalid input.");
            }
        };
    }
    save_database(db, &db_name);
}

fn display_menu() -> String {
    println!("1.\tNew user.");
    println!("2.\tNew spacebar.");
    println!("3.\tDisplay your spacebars.");
    println!("4.\tLookup unknown spacebar.");
    println!("5.\tSearch file for spacebars.");
    println!("6.\tSearch clipboard for spacebars.");
    println!("7.\tQuit");
    println!("\nInput the number of your choice: ");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            return input;
        },
        Err(error) => {
            println!("error: {}", error);
            return display_menu();
        },
    }
}
