// src/main.rs

#[macro_use]
extern crate nickel;
extern crate config;
extern crate regex;
extern crate hyper;
extern crate quick_xml;
extern crate serde;
extern crate serde_json;
extern crate redis;

use std::path::Path;
use std::env::set_var;
use nickel::{Nickel, MediaType};
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
    set_var("dictionary_api_key", dictionary_api_key.unwrap());


    let mut server = Nickel::new();

    server.utilize(router! {
        get "/" => |_req, _res| {
            format!("Hello!")
        }
    });

    server.utilize(router! {
        get "/synonyms/:word" => |_req,mut _res| {
            _res.set(MediaType::Json);
            let synonyms = synonyms::lookup(_req.param("word").unwrap().to_owned());
            format!("{} \"synonyms\": {} {}",
                "{",
                    serde_json::to_string(&synonyms).unwrap(),
                "}"
            )
        }
    });

    server.utilize(router! {
        get "/whois/:domain.:tld" => |_req, mut _res| {
            _res.set(MediaType::Json);

            let domain = _req.param("domain").unwrap();
            let tld = _req.param("tld").unwrap();

            let full_domain = format!("{}.{}", domain, tld);
            let status = domains::whois(full_domain.clone());

            format!("{} \"{}\": \"{}\", \"{}\": \"{}\" {}",
                "{",
                    "domain", full_domain,
                    "status", status,
                "}"
            )

        }
    });

    server.listen("127.0.0.1:9000");

}
