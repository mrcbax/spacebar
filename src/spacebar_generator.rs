extern crate rand;

use rand::Rng;

static ZERO: &'static str = "\u{FEFF}";
static ONE: &'static str = "\u{200B}";

struct Identifier {
    user: u64,
    puser: u32
}

pub fn generate() -> String {
    let mut ident: Identifier = gen_ident();
    let mut bin_nums: String = String::from(format!("{:b}", ident.user));
    bin_nums += format!("{:b}", ident.puser).as_str();
    let bar_nums = bin_to_string(&bin_nums);
    String::from(format!("The binary representation is: {0}\nBarcode representation is: {1}", bin_nums, bar_nums))
}

fn gen_ident() -> Identifier {
    let mut rng = rand::thread_rng();
    let mut ident = Identifier {user: (rng.gen::<u64>()), puser: (rng.gen::<u32>())};
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
