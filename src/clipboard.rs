use copypasta::{ClipboardContext, ClipboardProvider};

use std::io;
use std::io::prelude::*;

use log::*;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "Press any key when done to clear the clipboard...").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
    clear_clipboard();
}

pub fn parse_clipboard() -> Option<String> {
    let mut ctx = ClipboardContext::new().unwrap();
    match ctx.get_contents() {
        Ok(o) => return Some(o),
        Err(_) => {
            error!("Failed to read clipboard.");
            return None;
        }
    }
}

pub fn export_clipboard(spacebar: String) {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(spacebar).unwrap();
    info!("Spacebar copied to clipboard.");
    pause();
}

pub fn clear_clipboard() {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(String::new()).unwrap();
}
