

pub mod shortener {
use std::io::{stdout, Write}; 
extern crate curl;
use self::curl::easy::Easy;


fn get_content(url: &str) {
    let mut easy = Easy::new();
    easy.url(&format!("{}", url )).unwrap();
    easy.write_function(|data| {
    Ok(stdout().write(data).unwrap())
    }).unwrap();
    easy.perform().unwrap();
}


pub fn shorten(key: &str, s_url: &str) {
    get_content(&format!("https://api.awau.moe/shorten/polr?action=shorten&key={}&url={}", key, s_url));
}
}
