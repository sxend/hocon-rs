extern crate hocon;

use hocon::*;

#[test]
fn load_from_not_exists_path() -> () {
    let config: Box<Config> = ConfigFactory::parse_from_path("".to_string());
    assert_eq!(config.to_map().is_empty(), true);
}

#[test]
fn parse_from_string() -> () {
    let config_string = "\
    foo {
      bar {
        fizz = fizz
      }
    }
    foo.bar.buzz = buzz;
    ".to_string();
    let config: Box<Config> = ConfigFactory::parse_from_string(config_string);
    assert_eq!(config.get_string("foo.bar.fizz".to_string()), "fizz");
    assert_eq!(config.get_string("foo.bar.buzz".to_string()), "buzz");
}