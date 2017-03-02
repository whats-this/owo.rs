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

pub mod uploader {
    use std::io::{stdout, Write};
    extern crate curl;
    use self::curl::easy::Easy;
    use self::curl::easy::Form;
    use std::path::{Path, PathBuf};
    fn post_content(url: String, files: Vec<PathBuf>) {
        curl::init();
    let mut handle = Easy::new();
    for file in &files {
        let mut form = Form::new();
        form.part("file").file(&file).add().unwrap();

        handle.url(&url).unwrap();
        handle.httppost(form).unwrap();

        handle.write_function(|a| {
            Ok(stdout().write(a).unwrap())
        }).unwrap();

        handle.perform().unwrap();
    }
    }
    pub fn upload(key: &str, files: Vec<PathBuf>) {
        post_content(format!("https://api.awau.moe/upload/pomf?key={}", key), files)
    }
}
