use super::generator::*;

use log::*;

pub fn locate_spacebar(body: String) -> Option<i64> {
    if body.contains(ZERO) || body.contains(ONE) {
        let ptr_start_zero = body.find(ZERO);
        let ptr_start_one = body.find(ONE);
        let start;
        if ptr_start_zero.is_some() && ptr_start_one.is_some() {
            if ptr_start_zero.unwrap() < ptr_start_one.unwrap() {
                start = ptr_start_zero.unwrap();
            } else {
                start = ptr_start_one.unwrap();
            }
        } else if ptr_start_zero.is_some() {
            start = ptr_start_zero.unwrap();
        } else if ptr_start_one.is_some() {
            start = ptr_start_zero.unwrap();
        } else {
            return None;
        }

        let ptr_end_zero = body.rfind(ZERO);
        let ptr_end_one = body.rfind(ONE);
        let end;
        if ptr_end_zero.is_some() && ptr_end_one.is_some() {
            if ptr_end_zero.unwrap() > ptr_end_one.unwrap() {
                end = ptr_end_zero.unwrap();
            } else {
                end = ptr_end_one.unwrap();
            }
        } else if ptr_end_zero.is_some() {
            end = ptr_end_zero.unwrap();
        } else if ptr_end_one.is_some() {
            end = ptr_end_zero.unwrap();
        } else {
            return None;
        }

        let len = end - start;
        if len != 64 {
            return None;
        }

        return Some(string_to_bin(String::from(body.split_at(start).1.split_at(start + 64).0)));
    } else {
        return None;
    }
}

pub fn bin_to_string(num_rep: i64) -> String {
    let bin_rep = String::from(format!("{:b}", num_rep));
    debug!("{}", bin_rep);
    let mut bar_rep = String::new();
    for c in bin_rep.chars() {
        if c == '0' {
            bar_rep += ZERO;
        } else {
            bar_rep += ONE;
        }
    }
    return bar_rep
}

fn string_to_bin(bar_rep: String) -> i64 {
    let bin_rep = String::from("0b");
    for c in bar_rep.chars() {
        if c.to_string().as_str() == ZERO {
            bin_rep += "0";
        } else {
            bin_rep += "1";
        }
    }
    return bin_rep.parse::<i64>().unwrap();
}
