//clipboard code
use clipboard::*;

pub fn parse_clipboard(String: spacebar) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(spacebar.as_str().to_owned()).unwrap();
}

pub fn export_clipboard() {
    unimplemented!();
}
