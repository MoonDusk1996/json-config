use dirs;
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_writer};
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Nostr {
    pub privkey: Option<String>,
    pub relays: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub nostr: Nostr,
}

pub enum ArgsType {
    Privkey(String),
    Relays(Vec<String>),
    None,
}

pub fn edit_config_file(args: ArgsType, option: &str) -> Result<(), Box<dyn Error>> {
    let config_file_path = get_config_file_path()?;
    let mut config: Config = get_config(&config_file_path)?;

    match option {
        "privkey" => match args {
            ArgsType::Privkey(arg) => {
                config.nostr.privkey = Some(arg);
                save_config(&config, &config_file_path)?;
                println!("Private key added");
            }
            _ => {
                println!("--privkey expects a valid private key");
            }
        },
        "relays" => match args {
            ArgsType::Relays(arg) => {
                for element in arg {
                    if !config.nostr.relays.contains(&element) {
                        config.nostr.relays.push(element.clone());
                    }
                }

                save_config(&config, &config_file_path)?;
                println!("Relays added!");
            }
            _ => {
                println!("--relays expects a relay or a valid list of relays");
            }
        },
        "clear-relays" => match args {
            ArgsType::None => {
                config.nostr.relays = vec![];
                save_config(&config, &config_file_path)?;
                println!("List of relays has been cleaned");
            }
            _ => {
                println!("--clear-relays waits for no arguments");
            }
        },
        _ => {
            println!("Unknown command: {}", option);
        }
    }
    Ok(())
}

fn get_config_file_path() -> Result<std::path::PathBuf, Box<dyn Error>> {
    let config_dir = dirs::config_dir().ok_or("Não foi possível obter o local de configuração")?;
    let config_file_path = config_dir.join("nostr-bot/config.json");
    if let Some(parent_dir) = config_file_path.parent() {
        fs::create_dir_all(parent_dir)?;
    }
    Ok(config_file_path)
}

fn get_config(config_file_path: &Path) -> Result<Config, Box<dyn Error>> {
    if let Ok(file) = std::fs::File::open(config_file_path) {
        Ok(from_reader(file)?)
    } else {
        Ok(Config {
            nostr: Nostr {
                privkey: None,
                relays: vec![],
            },
        })
    }
}

fn save_config(config: &Config, config_file_path: &Path) -> Result<(), Box<dyn Error>> {
    let file = std::fs::File::create(config_file_path)?;
    to_writer(file, &config)?;
    Ok(())
}
