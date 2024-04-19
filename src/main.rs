use hex;
use std::env;

use wl_clipboard_rs::copy::{ClipboardType, MimeType, Options, Source};

use std::fs::File;
use std::io::Read;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    salt: String,
    domain: String,
}

fn main() {
    // Read the configuration from the config.toml file
    let mut config_file = File::open("config.toml").expect("Failed to open config file");
    let mut config_content = String::new();
    config_file.read_to_string(&mut config_content).expect("Failed to read config file");

    let config: Config = toml::from_str(&config_content).expect("Failed to parse config file");

    // Use the values from the config
    let salt: String = config.salt.to_string();
    let email_domain: String = config.domain.to_string();
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Error: Please provide an argument!");
    } else if args[0] == "verify" {
        if args.len() <= 1 {
            eprint!("Error: Please provide an email adresse!")
        } else {
        }
    } else {
        let website_domain: &str = args[1].as_str();
        let hash: md5::Digest = md5::compute(format!("{}+{}", website_domain, salt));
        let finalized_email: String = format!(
            "{}-{}@{}",
            website_domain,
            hex::encode(*hash).get(..8).unwrap(),
            email_domain
        );
        println!("{}", finalized_email);
        let mut opts = Options::new();
        opts.clipboard(ClipboardType::Regular);
        opts.foreground(true);
        opts.copy(
            Source::Bytes(finalized_email.into_bytes().into()),
            MimeType::Autodetect,
        )
        .unwrap();
    }
}
