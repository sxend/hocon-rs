extern crate hocon;

#[test]
fn config_factory_load() -> () {
    let config = hocon::ConfigFactory::load();
    println!("{:?}", config);
}