#![feature(plugin)]
#![plugin(rocket_codegen)]
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate postgres;
extern crate rand;


extern crate rocket;

mod spacebar_generator;
mod token_generator;
mod user;
mod insert;

use postgres::{Connection, TlsMode};
use spacebar_generator::*;
use token_generator::gen_token;
use user::readPostgreSQL;
use insert::*;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    userID: i32,
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct barUser {
    user_ID: String,
    user_barcode: String,
    name_barcode: String,
    description: String,
}

#[get("/query")]
fn hello() -> String {
    let usertes = user::readUserBarcode();
    // println!("{:?}", usertes);
    format!("{:?}", usertes)
}


fn main() {
    // Rocket Tester
    //rocket::ignite().mount("/", routes![hello]).launch();

    //Token Generator Test
    // for i in 1 .. 1000 {
    //     println!("{}", token_generator::gen_token());
    // }

    //Spacebar Generator Test
    //First, generate the user partion of the spacebar.
    //let test_str = spacebar_generator::generate_barcode();
    //Generate and print the full spacebar, passing the previously generated user portion.
    //println!("{}", spacebar_generator::generate_barcode_from_previous(String::from(test_str.as_str())));
    //println!("{}", spacebar_generator::generate_barcode());

    // let usertes = user::readPostgreSQL();
    // println!("{:?}", usertes);

    // let insertu = insert::insertUser();

    let mut u = "Diego Bustamante";
    let mut d = "This is the very second barcode!";
    insert::insertBarCode(format!("{:?}", u), format!("{:?}", d));
    // u = "Chad Baxter";
    // d = "And... if you see this, good job.";
    // insert::insertBarCode(format!("{:?}", u), format!("{:?}", d));

    let conn = Connection::connect("postgresql://root:toor@localhost/spacebardb", TlsMode::None)
            .unwrap();

    /*conn.execute("CREATE TABLE person (
                    id              SERIAL PRIMARY KEY,
                    name            VARCHAR NOT NULL,
                    data            BYTEA
                  )", &[]).unwrap();
    let me = Person {
        id: 0,
        name: "Steven".to_owned(),
        data: None
    };
    conn.execute("INSERT INTO person (name, data) VALUES ($1, $2)",&[&me.name, &me.data]).unwrap();

    for row in &conn.query("SELECT id, name, data FROM person", &[]).unwrap() {
        let person = Person {
            id: row.get(0),
            name: row.get(1),
            data: row.get(2)
        };
        println!("Found person {}", person.name);
    }*/
}
