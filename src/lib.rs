use std::io::{stdout, Write};
extern crate curl;
use curl::easy::Easy;


// Write the contents of rust-lang.org to stdo
fn get_content(url: &str) {
    let mut easy = Easy::new();
    easy.url(&format!("{}", url )).unwrap();
    easy.write_function(|data| {
    Ok(stdout().write(data).unwrap())
    }).unwrap();
    easy.perform().unwrap();
}


fn shorten(key: &str, s_url: &str) {
    get_content(&format!("https://api.awau.moe/shorten/polr?action=shorten&key={}&url={}", key, s_url));
    //println!("You typed: {}",s);
}
