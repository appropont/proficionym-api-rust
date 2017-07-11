use hyper::Client;
use hyper::header::Connection;

use std::io::Read;
use std::env::var;
use std::vec::Vec;
use regex::Regex;

use quick_xml::reader::Reader;
use quick_xml::events::Event;

use redis;
use redis::parse_redis_url;
use redis::Commands;

pub fn lookup(word: String) -> Vec<String> {

    let cached_synonyms = get_cached_synonyms(word.clone());

    if cached_synonyms.is_empty() {
        println!("No cached synonyms found, fetching...");
        let fetched_synonyms = fetch_synonyms(word.clone());
        set_cached_synonyms(word,
                            join_synonyms_to_string(fetched_synonyms.to_owned()).to_owned());
        return fetched_synonyms;
    } else {
        println!("Cached synonyms found. Bypassing fetch...");
        return split_synonyms_string(cached_synonyms);
    }

}

fn fetch_synonyms(word: String) -> Vec<String> {

    let client = Client::new();
    let api_key = var("dictionary_api_key").unwrap();

    let url = format!("http://www.dictionaryapi.com/api/v1/references/thesaurus/xml/{}?key={}",
                      word,
                      api_key);

    let mut response = client.get(&url)
        .header(Connection::close())
        .send()
        .unwrap();

    // Read the Response.
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();

    let mut reader = Reader::from_str(body.as_ref());
    let mut raw_words = String::new();

    let mut should_capture = false;
    let mut buf = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"syn" | b"rel" => {
                        should_capture = true;
                    }
                    _ => (),
                }
            }
            Ok(Event::Text(e)) => {
                if should_capture == true {
                    raw_words.push_str(&e.unescape_and_decode(&reader).unwrap());
                    raw_words.push_str(",");
                    should_capture = false;
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }

    // regexes (several of these could be combined)
    // TODO: Convert logic to token-based streaming string analysis for performance
    let regex_removals = Regex::new(r"(\s|\[\]|-)").unwrap();
    let regex_semicolons = Regex::new(r"([;])").unwrap();

    let raw_words_after_removals = regex_removals.replace_all(&raw_words, "");
    let raw_words_after_semicolons = regex_semicolons.replace_all(&raw_words_after_removals, ",");

    return split_synonyms_string(raw_words_after_semicolons.into_owned());

}

// TODO: Refactor this to return a proper Result or Option (unsure) instead of an empty string
fn get_cached_synonyms(word: String) -> String {

    let client = redis::Client::open(parse_redis_url(&var("REDIS_URL").unwrap()).unwrap()).unwrap();
    let connection = client.get_connection().unwrap();

    let cached_synonyms = connection.get(format!("synonyms:{}", word));

    if cached_synonyms.is_ok() {
        return cached_synonyms.unwrap();
    } else {
        return "".to_owned();
    }

}

// TODO: Make this function return a Result or Option
fn set_cached_synonyms(word: String, synonyms: String) {

    let client = redis::Client::open(parse_redis_url(&var("REDIS_URL").unwrap()).unwrap()).unwrap();
    let connection = client.get_connection().unwrap();
    let key = format!("synonyms:{}", word);
    let expiration = 60 * 60 * 24 * 180; //seconds * minutes * hours * days

    // This function doesnt return anything and this let seems superfluous,
    //   but the value needed the type annotation for the compiler
    let result: String = connection.set_ex(key, synonyms, expiration).unwrap();

}

fn split_synonyms_string(synonyms: String) -> Vec<String> {
    let mut words = Vec::new();
    let regex_parens = Regex::new(r"(\(.*\)|\()").unwrap();

    for word in synonyms.split(",") {
        if word != "" {
            // Scrub parens here since doing so at the same time as other removals
            //   was causing issues.
            // TODO: Fine tune the parens regex
            //   so that it can be done at the pre-split string level for performance.
            words.push(regex_parens.replace_all(&word.to_string(), "").into_owned());
        }
    }

    return words;
}

fn join_synonyms_to_string(synonyms: Vec<String>) -> String {
    return synonyms.join(",");
}
