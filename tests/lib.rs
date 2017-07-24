extern crate hocon;

use hocon::*;

#[test]
fn parse_from_path() -> () {
    match ConfigFactory::parse_from_path("./tests/application.conf".to_owned()) {
        Ok(config) => assert_eq!(config.to_map().is_empty(), true),
        Err(t) => panic!(t)
    }
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
    match ConfigFactory::parse_from_string(config_string) {
        Ok(config) => {
            assert_eq!(config.get_string(&"foo.bar.fizz".to_owned()), "fizz");
            assert_eq!(config.get_string(&"foo.bar.buzz".to_owned()), "buzz");
        },
        Err(t) => panic!(t)
    }

}