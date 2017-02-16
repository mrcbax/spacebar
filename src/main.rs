
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate postgres;
extern crate rand;

mod spacebar_generator;
mod token_generator;
mod user;

use postgres::{Connection, TlsMode};
use spacebar_generator::generate;
use token_generator::gen_token;
use user::readPostgreSQL;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: i32,
    username: String,
    email: String,
    password: String,
    userID: i32,
}

struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}


fn main() {

    //Token Generator Test
    println!("{}", token_generator::gen_token());

    let usertes = user::readPostgreSQL();
    println!("{:?}", usertes);

    let point = Point { x: 1, y: 2 };

    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);

    println!("{}", spacebar_generator::generate());
    let conn = Connection::connect("postgresql://root:toor@localhost/spacebardb", TlsMode::None)
            .unwrap();

    conn.execute("CREATE TABLE person (
                    id              SERIAL PRIMARY KEY,
                    name            VARCHAR NOT NULL,
                    data            BYTEA
                  )", &[]).unwrap();
    let me = Person {
        id: 0,
        name: "Steven".to_owned(),
        data: None
    };
    conn.execute("INSERT INTO person (name, data) VALUES ($1, $2)",
                 &[&me.name, &me.data]).unwrap();

    for row in &conn.query("SELECT id, name, data FROM person", &[]).unwrap() {
        let person = Person {
            id: row.get(0),
            name: row.get(1),
            data: row.get(2)
        };
        println!("Found person {}", person.name);
    }
}
