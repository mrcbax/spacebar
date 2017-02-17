extern crate rand;

use rand::Rng;

static ZERO: &'static str = "\u{FEFF}";
static ONE: &'static str = "\u{200B}";

struct Identifier {
    user: String,
    puser: String
}

pub fn generate_barcode () -> String {
    let mut rng = rand::thread_rng();
    let mut u64_string = String::new();
    let mut u32_string = String::new();
    for i in 0 .. 64 {
        u64_string += &format!("{}", rng.gen_range(0, 2));
    }

    for i in 0 .. 32 {
        u32_string += &format!("{}", rng.gen_range(0, 2));
    }
    let mut ident: Identifier = Identifier{user: String::from(u64_string.as_str()), puser: String::from(u32_string.as_str())};
    let bin_nums:String = u64_string + &u32_string;
    println!("{:?}", "Added as new");
    String::from(format!("{}", bin_to_string(&bin_nums)))
}

pub fn generate_barcode_from_previous (last_gen: String) -> String {
    let mut rng = rand::thread_rng();
    let string_bin = string_to_bin(&last_gen);
    let mut u64_string = String::from(string_bin.split_at(64).0);
    let mut u32_string = String::new();
    for i in 0 .. 32 {
        u32_string += &format!("{}", rng.gen_range(0, 2));
    }
    println!("{:?}", "Added with previous");
    String::from(format!("{}", bin_to_string(&String::from(u64_string + &u32_string))))
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
fn string_to_bin (barcode_string: &String) -> String {
    let mut bin_string = String::new();
    for c in barcode_string.chars() {
        if c.to_string() == ZERO {
            bin_string += "0";
        }
        else {
            bin_string += "1";
        }
    }
    bin_string
}
