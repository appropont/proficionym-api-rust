use std::process::Command;

pub fn whois(domain: String) -> String {
    //format!("whois lookup for {}", domain)
    let command_string = format!("whois {}", domain);
    let whois = Command::new("sh")
                            .arg("-c")
                            .arg(command_string)
                            .output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });
    String::from_utf8(whois.stdout).unwrap()
}

// TODO: Consider adapting https://gist.github.com/gkbrk/0c2317e9f72dbe55695b
//       using hyper instead of raw tcp.
