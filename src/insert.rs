

use postgres::{Connection, TlsMode};
use serde_json::Value;
use token_generator::gen_token;
use spacebar_generator;

struct newUser {
    username: String,
    email: String,
    password: String,
}

struct barUser {
    user_ID: u32,
    user_barcode: String,
    name_barcode: String,
    description: String
}


pub fn insertUser(){
    let conn = Connection::connect("postgresql://root:toor@localhost/spacebardb", TlsMode::None).unwrap();
    let new_u = newUser {
        username: String::from("diego"),
        email: String::from("diegotest@gmail.com"),
        password: String::from("password123")
    };
    //conn.execute("INSERT INTO users (user_name, email, password) VALUES ($1, $2, $3)", &[&new_u.username, &new_u.email, &new_u.password]).unwrap();
    println!("Just inserted.");
}


// Pass through User ID, Name, Description
pub fn insertBarCode(name: String, descrip: String) {
    let conn = Connection::connect("postgresql://root:toor@localhost/spacebardb", TlsMode::None).unwrap();
    let check_bar = false;

    let barcode_prev = String::new();
    for row in &conn.query("SELECT user_barcode FROM barcode", &[]).unwrap() {
        //check_bar=true;
        println!("{:?}", &conn.query("SELECT user_barcode FROM barcode", &[]).unwrap());

    }

    // match checkBar {
    //     false => let test_str = spacebar_generator::generate_barcode();,
    //     true => let test_str = spacebar_generator::generate_barcode(barcode_prev: String);
    // }

    // let bar =  barUser {
    //     user_ID: 1,
    //     user_barcode: String::from(test_str),
    //     name_barcode: String::from(name),
    //     description: String::from(descrip),
    // }; 

    // conn.execute("INSERT INTO barcode (user_barcode, name_barcode, description) VALUES ($1, $2, $3)", &[&bar.user_ID, &bar.user_barcode, &bar.name_barcode, &bar.description]).unwrap();
    println!("Just inserted with.");
}
