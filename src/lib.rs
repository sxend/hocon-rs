use std::collections::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

type BoxConfig = Box<Config>;

pub trait Config : std::fmt::Debug {
    fn get_bool(&self, key: &String) -> Option<bool>;
    fn get_string(&self, key: &String) -> Option<String>;
    fn get_config(&self, key: &String) -> Option<BoxConfig>;
}

#[derive(Debug)]
struct ConfigImpl {
    underlying: HashMap<String, String>
}

impl Config for ConfigImpl {
    fn get_bool(&self, key: &String) -> Option<bool> {
        match self.underlying.get(key) {
            Some(x) if x == "true" => Some(true),
            _ => Some(false)
        }
    }
    fn get_string(&self, key: &String) -> Option<String> {
        self.underlying.get(key).map(|x| { x.to_string()})
    }

    fn get_config(&self, key: &String) -> Option<BoxConfig>{
        unimplemented!()
    }
}

pub struct ConfigFactory;

impl ConfigFactory {
    pub fn parse_from_path(path: &String) -> Result<BoxConfig, Error> {
        match File::open(path) {
            Ok(file) => ConfigFactory::parse_from_file(&file),
            Err(t) => Err(t)
        }

    }
    pub fn parse_from_file(mut config_file: &File) -> Result<BoxConfig, Error> {
        let mut config_string = String::new();
        match config_file.read_to_string(&mut config_string) {
            Ok(_) => ConfigFactory::parse_from_string(&config_string),
            Err(t) => Err(t)
        }
    }
    pub fn parse_from_string(config_string: &String) -> Result<BoxConfig, Error> {
        let mut underlying: HashMap<String, String> = HashMap::new();
        Ok(Box::new(ConfigImpl{
            underlying: underlying
        }))
    }
    pub fn load() -> Result<BoxConfig, Error> {
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
