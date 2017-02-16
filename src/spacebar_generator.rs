extern crate rand;

use rand::Rng;

static ZERO: &'static str = "\u{FEFF}";
static ONE: &'static str = "\u{200B}";

struct Identifier {
    user: u32,
    puser: u16
}

pub fn generate() -> String {
    let mut ident: Identifier = gen_ident();
    let bin_nums: String = String::from(format!("{:b}", ident.user));
    String::from(format!("The binary representation is: {}", bin_nums))
}

fn gen_ident() -> Identifier {
    let mut rng = rand::thread_rng();
    let mut ident = Identifier {user: (rng.gen::<u32>()), puser: (rng.gen::<u16>())};
    ident
}
