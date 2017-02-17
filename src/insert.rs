

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
    user_ID: String,
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
    conn.execute("INSERT INTO users (user_name, email, password) VALUES ($1, $2, $3)", &[&new_u.username, &new_u.email, &new_u.password]).unwrap();
    println!("Just inserted.");
}


// Pass through User ID, Name, Description
pub fn insertBarCode(name: String, descrip: String) {
    let conn = Connection::connect("postgresql://root:toor@localhost/spacebardb", TlsMode::None).unwrap();
    let mut check_bar = false;
    let mut test_str = String::new();
    let userID = String::new();

    let mut barcode_prev = String::new();
    for row in &conn.query("SELECT user_barcode FROM barcode", &[]).unwrap() {
        println!("HERE IN FOR");
        check_bar=true;
        barcode_prev = row.get(0);
        // println!("{:?}", &conn.query("SELECT user_barcode FROM barcode", &[]).unwrap());

    }

    match check_bar {
        false => test_str = spacebar_generator::generate_barcode(),
        true => test_str = spacebar_generator::generate_barcode_from_previous(barcode_prev)
    }

    println!("{:?}", test_str);
    let bar =  barUser {
        user_ID: String::from("1"),
        user_barcode: String::from(test_str),
        name_barcode: String::from(name),
        description: String::from(descrip),
    }; 
    println!("HERE BEFORE INSERT.");
    conn.execute("INSERT INTO barcode (id_user, user_barcode, name_barcode, description) VALUES ($1, $2, $3, $4)", &[&bar.user_ID, &bar.user_barcode, &bar.name_barcode, &bar.description]);
    println!("Just inserted with.");
}
