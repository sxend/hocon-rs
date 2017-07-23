
//pub mod parser;

#[derive(Debug)]
pub struct Config;
impl Config {
    pub fn get_bool(&self) -> bool {
        true
    }
}

pub struct ConfigFactory;
impl ConfigFactory {
    pub fn load() -> Config {
        Config {}
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
