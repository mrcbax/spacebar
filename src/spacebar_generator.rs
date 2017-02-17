extern crate rand;

use rand::Rng;

static ZERO: &'static str = "\u{FEFF}";
static ONE: &'static str = "\u{200B}";

struct Identifier {
    user: u64,
    puser: u32
}

pub fn generate_new_user_id() -> String {
    let mut rng = rand::thread_rng();
    let mut ident: Identifier = Identifier{user: rng.gen::<u64>(), puser: 0};
    let mut bin_nums: String = String::from(format!("{:b}", ident.user));
    String::from(format!("{}", bin_to_string(&bin_nums)))
}

pub fn generate_spacebar_with_user_id(UID: String) -> String {
    let mut rng = rand::thread_rng();
    let mut ident: Identifier = Identifier{user: 0, puser: rng.gen::<u32>()};
    let mut bin_nums: String = String::from(format!("{:b}", ident.puser));
    let mut full_bar: String = String::from(format!("{0}{1}", UID, bin_to_string(&bin_nums)));
    String::from(format!("{0}{1}", &bin_nums, full_bar))
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
