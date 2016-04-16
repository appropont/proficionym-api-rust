use hyper::Client;
use hyper::header::Connection;

use std::io::Read;
use std::env::var;
use std::vec::Vec;
use regex::Regex;

use quick_xml::{XmlReader, Event, AsStr};

pub fn lookup(word: String) -> Vec<String> {
    //format!("looking up synonyms for {}", word)

    let mut client = Client::new();
    let api_key = var("dictionary_api_key").unwrap();

    let url = format!("http://www.dictionaryapi.com/api/v1/references/thesaurus/xml/{}?key={}",word ,api_key);

    let mut response = client.get(&url)
        .header(Connection::close())
        .send().unwrap();

    // Read the Response.
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();

    let reader = XmlReader::from_str(&body).trim_text(true);
    let mut raw_words = String::new();

    let mut should_capture = false;

    for r in reader {
        match r {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"syn" | b"rel" => {
                        should_capture = true;
                    },
                    _ => (),
                }
            },
            Ok(Event::Text(e)) => {
                if should_capture == true {
                    raw_words.push_str(e.into_string().unwrap().as_str());
                    raw_words.push_str(",");
                    should_capture = false;
                }
            },
            Err((e, pos)) => panic!("{:?} at position {}", e, pos),
            _ => (),
        }
    }

    println!("raw_words: {}", raw_words.clone());
    //regexes (several of these could be combined)
    // TODO: Convert logic to token-based streaming string analysis for performance
    let regex_removals = Regex::new(r"(\s|\[\]|-)").unwrap();
    let regex_semicolons = Regex::new(r"([;])").unwrap();
    let regex_parens = Regex::new(r"(\(.*\)|\()").unwrap();

    let raw_words_after_removals = regex_removals.replace_all(&raw_words, "");
    let raw_words_after_semicolons = regex_semicolons.replace_all(&raw_words_after_removals, ",");

    let mut words = Vec::new();

    for word in raw_words_after_semicolons.split(",") {
        if word != "" {
            // Scrub parens here since doing so at the same time as other removals was causing issues.
            words.push(regex_parens.replace_all(&word.to_string(), ""));
        }
    }

    return words;

}
