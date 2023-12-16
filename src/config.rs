use std::path::PathBuf;

use directories::ProjectDirs;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub lexer: LexerConfig,
}

#[derive(Deserialize)]
pub struct LexerConfig {
    pub extended: bool,
}

const DEFAULT: &'static str = r#"
[lexer]
extended = false
"#;

fn compute_config_dir(path: Option<PathBuf>) -> Option<PathBuf> {
    match path {
        Some(path) => {
            return Some(path);
        }
        None => {
            if let Some(proj_dir) = ProjectDirs::from("dev", "Arad-Fadaei", "abnf-toolkit") {
                return Some(proj_dir.config_dir().to_owned());
            } else {
                None
            }
        }
    }
}

pub fn load_config(path: Option<PathBuf>) -> Config {
    if let Some(dir) = compute_config_dir(path) {
        let joiner;
        if dir.is_file() {
            joiner = dir;
        } else {
            joiner = dir.join("config.toml")
        }

        let loaded = match std::fs::read_to_string(joiner) {
            Ok(s) => s,
            Err(e) => {
                println!("{e}");

                DEFAULT.to_string()
            }
        };

        let config: Config = toml::from_str(&loaded).unwrap();

        config
    } else {
        toml::from_str(DEFAULT).unwrap()
    }
}
