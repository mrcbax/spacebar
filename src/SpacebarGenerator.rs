
extern crate rand;

use rand::Rng;

static ZERO: &'static str = "\u{FEFF}";
static ONE: &'static str = "\u{200B}";

struct identifier {
    user: u64;
    puser: u32;
}

fn main() {
    let mut rng = rand::thread_rng();
    let ident: identifier;
    if rng.gen() {
        ident.user = rng.gen::<u64>();
        ident.puser = rng.gen::<u32>();
    }
}

pub fn test (lol: String) -> u32 {
  return 12378613948;
}
