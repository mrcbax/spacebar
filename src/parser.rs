use super::generator::*;

use std::fs;

use log::*;
use prettytable::Table;

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
            debug!("return none in ptr_start");
            return None;
        }
        if start == 0 {
            return Some(string_to_bin(String::from(body.split_at(187).0)));
        }
        return Some(string_to_bin(String::from(body.split_at(start).1.to_string().split_at(187).0)));
    } else {
        debug!("return none in contains");
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

pub fn string_to_bin(bar_rep: String) -> i64 {
    let mut bin_rep: i64 = 0;
    let mut iters = 0;
    for c in bar_rep.chars() {
        if iters >= 61 {
            break;
        }
        iters += 1;
        debug!("s{:#b}\t{}", bin_rep, bin_rep);
        if c.to_string().as_str() == ONE {
            bin_rep += 1;
            bin_rep = bin_rep.rotate_left(1);
        } else {
            bin_rep = bin_rep.rotate_left(1);
        }

    }
    //bin_rep = bin_rep.rotate_right(1);
    debug!("f{:#b}\t{}", bin_rep, bin_rep);
    return bin_rep;
}


pub fn print_spacebar(spacebar: Spacebar) {
    let mut table = Table::new();
    if spacebar.description.is_some() {
        table.add_row(row![BwbFb => "NAME", "DESCRIPTION", "SPACEBAR"]);
        table.add_row(row![spacebar.name.as_str(), spacebar.description.unwrap().as_str(), c -> format!("⭲{}⭰", bin_to_string(spacebar.spacebar))]);
        table.printstd();
    } else {
        table.add_row(row![BwbFb => "NAME", "SPACEBAR"]);
        table.add_row(row![spacebar.name.as_str(), c -> format!("⭲{}⭰", bin_to_string(spacebar.spacebar))]);
        table.printstd();
    }
}

pub fn parse_file(path: &str) -> Option<i64> {
    match fs::read_to_string(path) {
        Ok(o) => return locate_spacebar(o),
        Err(e) => {
            info!("failed to read file: {}", e);
            return None;
        },
    }
}

pub fn parse_clipboard() -> Option<i64> {
    match super::clipboard::parse_clipboard() {
        Some(s) => return locate_spacebar(s),
        None => return None,
    }
}
