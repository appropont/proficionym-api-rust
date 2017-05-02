use std::process::Command;
use regex::Regex;

pub fn whois(domain: String) -> String {
    whois_cli(domain)
}

fn whois_cli(domain: String) -> String {

    let command_string = format!("whois {}", domain);
    let whois = Command::new("sh")
                    .arg("-c")
                    .arg(command_string)
                    .output()
                    .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    let whois_response = String::from_utf8(whois.stdout).unwrap();

    let whois_available_regex = Regex::new("No match for domain").unwrap();
    let whois_unavailable_regex = Regex::new("Domain Name:").unwrap();

    let mut whois_status = "error";
    if whois_available_regex.is_match(&whois_response) {
        whois_status = "available";
    } else if whois_unavailable_regex.is_match(&whois_response) {
        whois_status = "registered";
    }

    return whois_status.to_string();

}

fn whois_custom(domain: String) -> String {
    domain
}

// TODO: Consider adapting https://gist.github.com/gkbrk/0c2317e9f72dbe55695b
//       using hyper instead of raw tcp.
