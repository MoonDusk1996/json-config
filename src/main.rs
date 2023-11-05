mod config;
mod crypto;

use config::ArgsType;
use crypto::encrypt;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!(
                "Nostr-bot v1.0\n\n--relays <relay1, relay2, ...>\nAdd relays.\n\n--clear-relays\nClear relays.\n\n --privkey <nsec...>\nAdd a private key from the existing one. 
"
        );
        return Ok(());
    }

    let command = &args[1];
    match command.as_str() {
        "--relays" => {
            if args.len() > 2 {
                let relays: Vec<String> = args.iter().skip(2).map(|s| s.to_string()).collect();
                config::edit_config_file(ArgsType::Relays(relays), "relays")?;
            }
        }
        "--clear-relays" => {
            if args.len() > 1 {
                println!("{:?}", args.len());
                config::edit_config_file(ArgsType::None, "clear-relays")?;
            }
        }
        "--privkey" => {
            if args.len() > 2 {
                let privkey = &args[2];
                config::edit_config_file(ArgsType::Privkey(encrypt(privkey)), "privkey")?;
            }
        }

        _ => {
            println!("Unknown command: {}", command);
        }
    }

    Ok(())
}
