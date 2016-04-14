// extern crate hyper;

// use hyper::Client;
// use hyper::header::Connection;

pub fn lookup(word: String) -> String {
    format!("looking up synonyms for {}", word)
}
