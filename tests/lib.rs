extern crate hocon;

use hocon::*;

#[test]
fn parse_from_path() -> () {
    match ConfigFactory::parse_from_path(&"./tests/application.conf".to_string()) {
        Ok(config) => assert_eq!(config.get_string("foo.bar.fizz"), Some("fizz".to_string())),
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
    match ConfigFactory::parse_from_string(&config_string) {
        Ok(config) => {
            assert_eq!(config.get_string("foo.bar.fizz"), Some("fizz".to_string()));
            assert_eq!(config.get_string("foo.bar.buzz"), Some("buzz".to_string()));
        },
        Err(t) => panic!(t)
    }

}