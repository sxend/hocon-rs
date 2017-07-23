
//pub mod parser;
pub trait Config : std::fmt::Debug {
    fn get_bool(&self) -> bool;
}

#[derive(Debug)]
struct ConfigImpl;

impl Config for ConfigImpl {
    fn get_bool(&self) -> bool {
        true
    }
}

pub struct ConfigFactory;

impl ConfigFactory {
    pub fn load() -> Box<Config> {
        Box::new(ConfigImpl{}) as Box<Config>
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
