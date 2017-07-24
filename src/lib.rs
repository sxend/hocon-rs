use std::collections::*;
use std::fs::File;
use std::io::prelude::*;

//pub mod parser;
pub trait Config : std::fmt::Debug {
    fn get_bool(&self, key: String) -> bool;
    fn get_string(&self, key: String) -> String;
    fn with_fallback(&self, config: &Config) -> Box<Config>;
    fn to_map(&self) -> HashMap<String, String>;
}


#[derive(Debug)]
struct ConfigImpl {
    underlying: HashMap<String, String>
}

impl Config for ConfigImpl {
    fn get_bool(&self, key: String) -> bool {
        match self.underlying.get(&key).unwrap().as_ref() {
            "true" => true,
            _ => false
        }
    }
    fn get_string(&self, key: String) -> String {
        self.underlying.get(&key).map(|x| { x.to_string()}).unwrap()
    }
    fn to_map(&self) -> HashMap<String, String> {
        self.underlying.clone()
    }
    fn with_fallback(&self, config: &Config) -> Box<Config> {
        let mut new_underlying: HashMap<String, String> = HashMap::new();
        for (key, value) in self.to_map().iter() {
            new_underlying.insert(key.to_string(), value.to_string());
        }
        for (key, value) in config.to_map().iter() {
            new_underlying.insert(key.to_string(), value.to_string());
        }
        Box::new(ConfigImpl{
            underlying: new_underlying
        })
    }
}

pub struct ConfigFactory;

impl ConfigFactory {
    pub fn parse_from_path(path: String) -> Box<Config> {
        let mut config_string = String::new();
        if let Ok(mut config_file) = File::open(path) {
            config_file.read_to_string(&mut config_string).unwrap();
        }
        ConfigFactory::parse_from_string(config_string)
    }
    pub fn parse_from_string(config_string: String) -> Box<Config> {
        let mut underlying: HashMap<String, String> = HashMap::new();
        Box::new(ConfigImpl{
            underlying: underlying
        })
    }
    pub fn load() -> Box<Config> {
        let path = "$HOME/application.conf"; // TODO: detect config directory
        ConfigFactory::parse_from_path(path.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
