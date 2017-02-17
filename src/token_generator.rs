extern crate rand;

use rand::Rng;

pub fn gen_token() -> String {
    let mut rng = rand::thread_rng();
    let rand_num = rng.gen::<u64>();
    let rand_num_2 = rng.gen::<u64>();
    let hex_string = String::from(format!("{:x}{:x}", rand_num, rand_num_2));
    hex_string
}
