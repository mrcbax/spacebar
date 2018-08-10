extern crate serde_json;

use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

use generator::ZERO;
use generator::ONE;
use generator::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub name: String,
    pub idents: Vec<Identifiers>
}

pub fn read_database(path: &String) -> Database {
    if Path::new(&path).exists() {
        let file = File::open(path).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();
        let deserialized: Database = serde_json::from_str(&contents).unwrap();
        return deserialized;
    } else {
        println!("Please input a new database name: ");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input = String::from(input.trim());
                save_database(Database{name: input, idents: vec!()}, path)
            }
            Err(e) => {
                input = String::from(input.trim());
                println!("error: {}", e);
                Database{name: input, idents: vec!()}
            },
        }
    }
}

pub fn save_database(db: Database, path: &String) -> Database {
    let serialized = serde_json::to_string_pretty(&db).unwrap();
    let mut file = File::create(path).unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
    db
}

pub fn lookup_spacebar(spacebar: String, db: &Database) -> Option<(Identifiers, Spacebar)> {
    let mut clean_bar: String = spacebar.clone().replace("\n", "");
    if clean_bar.contains(ZERO) || clean_bar.contains(ONE) {
        clean_bar.retain(|c| c.to_string() == ZERO || c.to_string() == ONE);
        if clean_bar.len() > 96 {
            clean_bar = clean_bar.chars().take(96).collect();
        }
        if clean_bar.len() < 96 {
            println!("This is a malformed spacebar: -->{}<--", clean_bar);
            return None;
        }
        for ident in &db.idents {
            for spc in &ident.spacebars {
                if spc.spacebar.contains(clean_bar.as_str()) {
                    println!("third if");
                    return Some((ident.clone(), spc.clone()));
                }
            }
        }
    }
    return None;
}

pub fn find_user_name(user_name: &String, db: &Database) -> bool {
    for ident in &db.idents {
        if ident.user_name.eq(user_name.as_str()) {
            return true;
        }
    }
    return false;
}
