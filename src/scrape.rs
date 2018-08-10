extern crate curl;

use self::curl::easy::Easy;

pub fn scrape_url(url: &str) -> String {
    let mut dst = Vec::new();
    let mut easy = Easy::new();
    easy.url(url).unwrap();

    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            dst.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
     match String::from_utf8(dst) {
         Ok(o) => o,
        Err(_) => {
            println!("Failed to parse web page.");
            String::from("")
        },
    }
}
