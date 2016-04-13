// src/main.rs

#[macro_use] 
extern crate nickel;
extern crate config;

use std::process;
use std::path::Path;
use nickel::{Nickel, JsonBody, HttpRouter, Request, Response, MiddlewareResult, MediaType};
use config::reader;

mod domains;
mod synonyms;

fn main() {

    // Configuration setup
    let app_config = reader::from_file(Path::new("app.conf"));
    assert!(app_config.is_ok());
    let configuration = app_config.unwrap();
    let dictionary_api_key = configuration.lookup_str("application.keys.dictionary");
    assert!(dictionary_api_key.is_some());

    let mut server = Nickel::new();

    server.utilize(router! {
        get "/" => |_req, _res| {
            format!("Hello!")
        }
    });

    server.utilize(router! {
        get "/synonyms/:word" => |_req, _res| {
            synonyms::lookup(_req.param("word").unwrap().to_owned())
        }
    });

    server.utilize(router! {
        get "/whois/:domain.:tld" => |_req, _res| {
            let domain = _req.param("domain").unwrap();
            let tld = _req.param("tld").unwrap();
            let full_domain = format!("{}.{}", domain, tld);
            domains::whois(full_domain)
        }
    });

    server.listen("127.0.0.1:9000");

}