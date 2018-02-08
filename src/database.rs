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

pub fn read_database(path: String) -> Database {
    if Path::new(&path).exists() {
        let file = File::open(path).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();
        let deserialized: Database = serde_json::from_str(&contents).unwrap();
        return deserialized;
    } else {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                save_database(Database{name: input, idents: vec!()}, path)
            }
            Err(error) => {
                println!("error: {}", error);
                Database{name: input, idents: vec!()}
            },
        }
    }
}

pub fn save_database(db: Database, path: String) -> Database {
    let serialized = serde_json::to_string(&db).unwrap();
    let mut file = File::create(path).unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
    db
}

pub fn lookup_spacebar(spacebar: String, db: Database) -> Option<(Identifiers, Spacebar)> {
    let mut clean_bar = String::new();
    match if spacebar.find(ZERO).unwrap_or_default() < spacebar.find(ONE).unwrap_or_default() {
        clean_bar = spacebar.chars().skip(spacebar.find(ZERO).unwrap()).take(96).collect();
    } else {
        clean_bar = spacebar.chars().skip(spacebar.find(ONE).unwrap()).take(96).collect();
    } {
        Ok(_) => println!("Spacebar found."),
        Err(e) => return None,
    };
    for ident in db.idents {
        for spc in ident.spacebars {
            if spc.spacebar == clean_bar {
                return Some((ident, spc));
            }
        }
    }
    return None;
}
