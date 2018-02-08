extern crate rand;

use rand::Rng;

use board::*;

pub static ZERO: &'static str = "\u{FEFF}";
pub static ONE: &'static str = "\u{200B}";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Identifiers {
    pub user_name: String,
    pub user_id: String,
    pub spacebars: Vec<Spacebar>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Spacebar {
    pub name: String,
    pub desc: String,
    pub spacebar: String
}

pub fn new_user_id() -> String {
    let mut rng = rand::thread_rng();
    let mut u64_string = String::new();
    for _ in 0 .. 64 {
        u64_string += &format!("{}", rng.gen_range(0, 2));
    }
    String::from(format!("{}", bin_to_string(&u64_string)))
}

pub fn generate_barcode(user_name: String, user_id: String, name: String, desc: String) -> Identifiers {
    let mut rng = rand::thread_rng();
    let mut u32_string = String::new();

    for _ in 0 .. 32 {
        u32_string += &format!("{}", rng.gen_range(0, 2));
    }

    let bin_nums: String = String::from(format!("{}{}", &user_id, u32_string));

    let spacebar: Spacebar = Spacebar{
        name: name,
        desc: desc,
        spacebar: String::from(format!("{}", bin_to_string(&bin_nums)))
    };

    let spacebars: Vec<Spacebar> = vec!(spacebar);
    export_clipboard(bin_nums);
    Identifiers{user_name: user_name, user_id: user_id, spacebars: spacebars}
}

pub fn generate_barcode_from_previous (mut ident: Identifiers, name: String, desc: String) -> Identifiers {
    let mut rng = rand::thread_rng();
    let mut u32_string = String::new();

    for _ in 0 .. 32 {
        u32_string += &format!("{}", rng.gen_range(0, 2));
    }

    let spacebar = Spacebar{
        name: name,
        desc: desc,
        spacebar: String::from(format!("{}{}", ident.user_id, bin_to_string(&String::from(u32_string))))
    };
    export_clipboard(spacebar.clone().spacebar);
    ident.spacebars.push(spacebar);
    ident
}

fn bin_to_string (bin_rep: &String) -> String {
    let mut bar_rep = String::new();
    for c in bin_rep.chars() {
        if c == '0' {
            bar_rep += ZERO;
        }
        else {
            bar_rep += ONE;
        }
    }
    bar_rep
}
