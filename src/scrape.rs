extern crate curl;

use self::curl::easy::Easy;

pub fn scrape_url(url: &str) -> String {
    let mut dst = Vec::new();
    let mut easy = Easy::new();
    easy.url(url).unwrap();

    {
        easy.transfer().write_function(|data| {
            dst.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();
        match easy.follow_location(true) {
            Ok(_) => (),
            Err(_) => println!("Failed to follow a redirect. A spacebar may not be found."),
        };
        easy.transfer().perform().unwrap();
    }
    if easy.response_code().unwrap() != 200 {
        println!("Server response was not as expected. A spacebar may not be found even if there is one. Response code: {}", easy.response_code().unwrap());
    }
    match String::from_utf8(dst) {
        Ok(o) => o,
        Err(_) => {
            println!("Failed to parse web page.");
            String::from("")
        },
    }
}
