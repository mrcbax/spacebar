use generator::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub name: String,
    pub idents: Vec<Identifiers>
}

pub fn read_database(path: String) -> Database {
    unimplemented!();
}

pub fn lookup_spacebar(spacebar: String) -> (Identifiers, Spacebar) {
    unimplemented!();
}
