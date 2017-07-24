extern crate hocon;

use hocon::*;

#[test]
fn config_factory_load() -> () {
    let config = ConfigFactory::load();
    println!("{:?}", config);
}