use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

use std::io;
use std::io::prelude::*;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "Press any key when done to clear the clipboard...").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
    clear_clipboard();
}

pub fn parse_clipboard() -> String {
    unimplemented!();
}

pub fn export_clipboard(spacebar: String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(spacebar).unwrap();
    println!("Spacebar copied to clipboard.");
    pause();
}

pub fn clear_clipboard() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(String::new()).unwrap();
}
