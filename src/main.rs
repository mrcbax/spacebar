
extern crate postgres;
extern crate rand;
mod spacebar_generator;

use postgres::{Connection, TlsMode};
use spacebar_generator::generate;

struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>
}

fn main() {
    println!("{}", spacebar_generator::generate());
    let conn = Connection::connect("postgresql://root:toor@localhost", TlsMode::None)
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
