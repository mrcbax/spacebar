use super::clipboard::*;

use log::*;
use rand::Rng;

pub static ZERO: &'static str = "\u{FEFF}";
pub static ONE: &'static str = "\u{200B}";

#[derive(Clone, Debug)]
pub struct Spacebar {
    pub spacebar: i64,
    pub name: String,
    pub description: String,
}


pub fn generate_spacebar(name: String, desc: String) -> Spacebar {
    let mut rng = rand::thread_rng();
    let gen_bar: i64 = rng.gen();

    let spacebar: Spacebar = Spacebar {
        spacebar: gen_bar,
        name: name,
        description: desc,
    };

    export_clipboard(bin_to_string(gen_bar));
    debug!("Created spacebar {:#?}", spacebar);
    return spacebar;
}

fn bin_to_string (num_rep: i64) -> String {
    let bin_rep = String::from(format!("{:b}", num_rep));
    let mut bar_rep = String::new();
    for c in bin_rep.chars() {
        if c == '0' {
            bar_rep += ZERO;
        }
        else {
            bar_rep += ONE;
        }
    }
    return bar_rep
}
