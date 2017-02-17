extern crate rand;

use rand::Rng;

pub fn gen_token() -> String {
    let mut rng = rand::thread_rng();
    let mut s = String::new();
    let mut randnum_1:u64 = 0;
    let mut randnum_2:u64 = 0;
    for i in 1 .. 19 {
        s += &format!("{}", rng.gen_range(1, 10));
    }
    randnum_1 = s.parse::<u64>().unwrap();
    s=String::new();
    for i in 1 .. 19 {
        s += &format!("{}", rng.gen_range(1, 10));
    }
    randnum_2 = s.parse::<u64>().unwrap();
    let hex_string = String::from(format!("{:x}{:x}", randnum_1, randnum_2));
    hex_string
}
