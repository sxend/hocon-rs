use std::collections::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

#[derive(Debug, Clone)]
pub struct Config {
    underlying: HashMap<String, ConfigValue>,
}

#[derive(Debug, Clone)]
pub enum ConfigValue {
    None,
    String(String),
    Bool(bool),
    Config(Config)
}

impl ConfigValue {
}

impl Config {
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        match self.underlying.get(key) {
            Some(&ConfigValue::Bool(value)) => Some(value),
            _ => None
        }
    }
    pub fn get_string(&self, key: &str) -> Option<String> {
        match self.underlying.get(key) {
            Some(&ConfigValue::String(ref value)) => Some(value.to_owned()),
            _ => None
        }
    }

    pub fn get_config(&self, key: &str) -> Option<Config> {
        match self.underlying.get(key) {
            Some(&ConfigValue::Config(ref value)) => Some(value.to_owned()),
            _ => None
        }
    }
}

pub struct ConfigFactory {
}

impl ConfigFactory {
    pub fn parse_from_path(path: &String) -> Result<Config, Error> {
        match File::open(path) {
            Ok(file) => ConfigFactory::parse_from_file(&file),
            Err(t) => Err(t)
        }
    }
    pub fn parse_from_file(mut config_file: &File) -> Result<Config, Error> {
        let mut config_string = String::new();
        match config_file.read_to_string(&mut config_string) {
            Ok(_) => ConfigFactory::parse_from_string(&config_string),
            Err(t) => Err(t)
        }
    }
    pub fn parse_from_string(config_string: &String) -> Result<Config, Error> {
        let mut underlying: HashMap<String, ConfigValue> = HashMap::new();
        underlying.insert("foo.bar.fizz".to_string(), ConfigValue::String(config_string.to_owned())); // FIXME
        Ok(Config {
            underlying: underlying
        })
    }
    pub fn load() -> Result<Config, Error> {
        let path = "$HOME/application.conf"; // TODO: resolve config path
        ConfigFactory::parse_from_path(&path.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
