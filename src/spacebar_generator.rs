extern crate rand;

use rand::Rng;

static ZERO: &'static str = "\u{FEFF}";
static ONE: &'static str = "\u{200B}";

struct Identifier {
    user: u64,
    puser: u32
}

pub fn generate_barcode () -> String {
    let mut rng = rand::thread_rng();
    let mut ident: Identifier = Identifier{user: rng.gen::<u64>(), puser: rng.gen::<u32>()};
    let mut bin_nums: String = String::from(format!("{:b}", ident.user));
    bin_nums += format!("{:b}", ident.puser).as_str();
    String::from(format!("{}", bin_to_string(&bin_nums)))
}

pub fn generate_barcode_from_previous (last_gen: String) -> String {
    let string_bin = string_to_bin(&last_gen);
    let mut u64_string = String::from(string_bin.split_at(63).0);
    let mut u32_string = String::from(string_bin.split_at(63).1);
    String::from("u64 String: {0}, u32 String: {1}", u64_string, u32_string)


    //let mut ident: Identifier = Identifier{user: 0, puser: 0};



    //let mut bin_nums: String = String::from(format!("{:b}", ident.puser));
    //let mut full_bar: String = String::from(format!("{0}{1}", UID, bin_to_string(&bin_nums)));
    //String::from(format!("{}", full_bar))
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
