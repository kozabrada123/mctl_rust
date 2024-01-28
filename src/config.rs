use custom_error::custom_error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Defines the app config
pub struct Config {
    /// Path to the server's root folder
    pub server_folder: String,
    /// Argon2id hash of the auth key
    pub auth_key_hash: String,
}

impl Config {
    /// Tries to load the config from ./Config.toml
    pub fn load() -> Result<Config, ConfigError> {
        let file_as_string_result = std::fs::read_to_string("./Config.toml");

        if let Err(e) = file_as_string_result {
            return Err(ConfigError::FsError { e });
        }

        let file_as_string = file_as_string_result.unwrap();

        let parse_result = toml::from_str::<Config>(&file_as_string);

        if let Err(e) = parse_result {
            return Err(ConfigError::ParseError { e });
        }

        let parsed = parse_result.unwrap();

        return Ok(parsed);
    }
}

custom_error! {
    pub ConfigError
    FsError{e: std::io::Error} = "Encountered io error: {e}",
    ParseError{e: toml::de::Error} = "Encountered parse error: {e}"
}
