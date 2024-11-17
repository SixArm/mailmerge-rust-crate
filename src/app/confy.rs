//! Configuration test of the `confy` crate for the application.
//!
//! Our source code convention is using this file `confy.rs`
//! in order to test configuration file loading and parsing.
//!
//! See also the project file `config.rs` for our `Config` struct.

pub fn config() -> Result<crate::app::config::Config, ::confy::ConfyError> {
    ::confy::load(env!("CARGO_PKG_NAME"), None)
}

#[cfg(test)]
mod test {
    use assertables::*;
    use crate::app::config::Config;

    #[test]
    fn test_config() {
        let config: Config = assert_ok!(super::config());
        dbg!(config);
    }

}
