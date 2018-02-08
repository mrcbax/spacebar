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

use database::*;
use generator::*;

use clap::{App, Arg};
use spinners::{Spinner, Spinners};

pub fn main() {
    let matches = App::new("My Super Program")
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
    let mut sentinel: bool = true;
    while sentinel {
        match display_menu().trim() {
            "1" => {
                println!("Input the new username: ");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                &db.idents.push(generate_barcode(input, new_user_id(), String::from("Default"), String::from("The default spacebar.")));
                println!("New user created successfully.");
            },
            "2" => {
                println!("Name: ");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                println!("Short description: ");
                let mut desc = String::new();
                io::stdin().read_line(&mut desc).unwrap();
                for ident in &db.idents.clone() {
                    if ident.user_name == user {
                        &db.idents.push(generate_barcode_from_previous(ident.clone(), name, desc));
                        &db.idents.dedup_by(|a, b| a.user_id.as_str().eq_ignore_ascii_case(b.user_id.as_str()) && a.spacebars.len() > b.spacebars.len());
                        break;
                    }
                }
            },
            "3" => {
                for ident in &db.idents {
                    if ident.user_name == user {
                        for spc in &ident.spacebars {
                            println!("Name:\t{}\nDescription:\t{}\nSpacebar:\'{}\'", &spc.name, &spc.desc, &spc.spacebar);
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
                    },
                    None => {
                        println!("No spacebar found in line.");
                    },
                };
            },
            "5" => {
                println!("Enter the path to the file: ");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let f = File::open(input).expect("File not found.");
                let mut file = BufReader::new(&f);
                let sp = Spinner::new(Spinners::Dots9, "Searching...".into());
                for line in file.lines() {
                    let l = line.unwrap();
                    match lookup_spacebar(l, &db) {
                        Some(e) => {
                            println!("Spacebar found!");
                            println!("Username: {}", e.0.user_name);
                            println!("Spacebar name: {}", e.1.name);
                            println!("Spacebar description: {}", e.1.desc);
                        },
                        None => {
                            print!("");
                        },
                    };
                }
                sp.stop();
            }
            "6" => {
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
    println!("5.\tSearch file for spacebar.");
    println!("6.\tQuit");
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

