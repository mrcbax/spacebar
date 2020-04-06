use super::clipboard::*;
use super::parser::*;

use log::*;
use rand::Rng;

pub static ZERO: &'static str = "\u{FEFF}";
pub static ONE: &'static str = "\u{200B}";

#[derive(Clone, Debug)]
pub struct Spacebar {
    pub spacebar: i64,
    pub name: String,
    pub description: Option<String>,
}


pub fn generate_spacebar(name: String, desc: Option<String>) -> Spacebar {
    let mut rng = rand::thread_rng();
    let gen_bar: i64 = rng.gen();

    let spacebar: Spacebar = Spacebar {
        spacebar: gen_bar,
        name: name,
        description: desc,
    };

    clipboard::export_clipboard(parser::bin_to_string(gen_bar));
    debug!("Created spacebar {:#?}", spacebar);
    return spacebar;
}
