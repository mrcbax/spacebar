extern crate serde_json;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use generator::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub name: String,
    pub idents: Vec<Identifiers>
}

pub fn read_database(path: String) -> Database {
    let file = File::open(path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let deserialized: Database = serde_json::from_str(&contents).unwrap();
    deserialized
}

pub fn save_database(db: Database, path: String) {
    let serialized = serde_json::to_string(&db).unwrap();
    let mut file = File::create(path).unwrap();
    file.write_all(serialized).unwrap();
}

pub fn lookup_spacebar(spacebar: String) -> (Identifiers, Spacebar) {
    unimplemented!();
}
