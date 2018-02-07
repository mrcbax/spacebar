#[macro_use]
extern crate serde_derive;

extern crate clap;
extern crate clipboard;
extern crate rand;
extern crate serde;
extern crate serde_json;

pub mod board;
pub mod database;
pub mod generator;

use board::*;
use database::*;
use generator::*;

pub fn main() {
    //let mut db: Database = read_database(String::from("./database.json"));
    let mut ident: Identifiers = generate_barcode(new_user_id(), String::from("Test"), String::from("A description"));
    let spacebar: Spacebar = ident.spacebars.pop().unwrap();
    let spacebarn: String = spacebar.name;
    let spacebard: String = spacebar.desc;
    let spacebars: String =  spacebar.spacebar;
    println!("Data: user_id:'{}',spacebarn:{},spacebard:{},spacebar:\'{}\'", &ident.user_id, spacebarn, spacebard, &spacebars);
    export_clipboard(spacebars);
}

