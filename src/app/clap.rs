//! Command line argument parsing (CLAP) for the application.
//!
//! clap is a crate for command line argument parsing.
//! 
//! See https://docs.rs/clap/
//!
//! Clap has a variety of setup approaches:
//!
//!   * via typical functions, which favors advanced uses yet is verbose.
//!   * via usage strings, which looks more like writing documentation.
//!   * via macros, which is fast and less verbose, yet atypical to read.
//!   * via YAML file, which favors localization and text file readability.
//!
//! We prefer the typical functions, because they provide maximum capability,
//! and in our experience are the easiest for Rust IDEs to read and debug.
//!
//! We favor our convention of doing clap setup in a file named `clap.rs`,
//! rather than in `main.rs`, because we like the separation of concerns.


use clap::{Arg, Command, value_parser};
use crate::app::args::Args;

/// Create a clap app.
pub fn app() -> Command {
    trace!("app");
    Command::new(env!("CARGO_PKG_NAME"))
    .name(env!("CARGO_PKG_NAME"))
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .arg(Arg::new("test")
        .help("Print test output for debugging, verifying, tracing, and the like.\nExample: --test")
        .long("test")
        .action(clap::ArgAction::SetTrue)
    )
    .arg(Arg::new("verbose")
        .help("Set the verbosity level: 0=none, 1=error, 2=warn, 3=info, 4=debug, 5=trace.\nExample: --verbose …")
        .short('v')
        .long("verbose")
        .action(clap::ArgAction::Count)
    )
    .arg(Arg::new("username")
        .help("Set username.\nExample: alice@example.com")
        .short('u')
        .long("username")
        .value_parser(value_parser!(String))
    )
    .arg(Arg::new("password")
        .help("Set password.\nExample: secret")
        .short('p')
        .long("password")
        .value_parser(value_parser!(String))
    )
    .arg(Arg::new("host")
        .help("Set host.\nExample: smtp.example.com")
        .long("host")
        .value_parser(value_parser!(String)),
    )
    .arg(Arg::new("port")
        .help("Set port number. A valid port number for this program is from 1 to 65535.\nExample: 465")
        .long("port")
        .value_parser(value_parser!(u16))
    )
    .arg(Arg::new("security")
        .help("Set security.\nExample: SSL/TLS")
        .long("security")
        .value_parser(value_parser!(String))
    )
    .arg(Arg::new("ratelimit")
        .help("Set rate limit.\nExample: 0")
        .long("ratelimit")
        .value_parser(value_parser!(u16))
    )

}

/// Create an Args struct initiated with the clap App settings.
pub fn args() -> Args {
    trace!("args");
    let matches = app().get_matches();
    trace!("args ➡ matches: {:?}", matches);

    let test = matches.get_flag("test");

    let log_level = match matches.get_count("verbose") {
        0 => None,
        1 => Some(::log::Level::Error),
        2 => Some(::log::Level::Warn),
        3 => Some(::log::Level::Info),
        4 => Some(::log::Level::Debug),
        5 => Some(::log::Level::Trace),
        _ => Some(::log::Level::Trace),
    };

    let username = matches.get_one::<String>("username").map(|s| s.to_string());

    let password = matches.get_one::<String>("password").map(|s| s.to_string());

    let host = matches.get_one::<String>("host").map(|s| s.to_string());

    let port = matches.get_one::<u16>("port").map(|x| *x);

    let security= matches.get_one::<String>("security").map(|s| s.to_string());

    let ratelimit = matches.get_one::<u16>("ratelimit").map(|x| *x);

    let args = Args {
        test: test,
        log_level: log_level,
        username: username,
        password: password,
        host: host,
        port: port,
        security: security,
        ratelimit: ratelimit,
    };

    trace!("args ➡ args: {:?}", args);
    args
}

#[cfg(test)]
mod tests {
    //use super::*;
    use crate::app::paths::*;
    use assertables::*;

    /// Test that the special argument `--test` is working.
    ///
    /// This test must succeed in order for any of the rest of the tests here to
    /// succeed, because the `--test` argument turns on the runtime output to stdout,
    /// which includes a typical print debug of the entire `args` structure.
    ///
    /// Example:
    /// 
    ///     sita --test
    /// 
    #[test]
    fn test_test() {
        assert_program_args_stdout_string_contains!(&*COMMAND_OS, &["--test"], r#"Args { "#);
    }

    /// Test that the special argument `--verbose` is working.
    ///
    /// This test must succeed in order for any of the rest of the tests here to
    /// show diagnostics, because the `--verbose` argument turns on logging output,
    /// which can include debugging messages, warnings, errors, and so on.
    ///
    /// Example:
    /// 
    ///     sita --verbose
    /// 
    #[test]
    fn test_verbose() {
        assert_program_args_stdout_string_contains!(&*COMMAND_OS, &["--test"], r#" log_level: None"#);
        assert_program_args_stdout_string_contains!(&*COMMAND_OS, &["--test", "-v"], r#" log_level: Some(Error)"#);
        assert_program_args_stdout_string_contains!(&*COMMAND_OS, &["--test", "-vv"], r#" log_level: Some(Warn)"#);
        assert_program_args_stdout_string_contains!(&*COMMAND_OS, &["--test", "-vvv"], r#" log_level: Some(Info)"#);
        assert_program_args_stdout_string_contains!(&*COMMAND_OS, &["--test", "-vvvv"], r#" log_level: Some(Debug)"#);
        assert_program_args_stdout_string_contains!(&*COMMAND_OS, &["--test", "-vvvvv"], r#" log_level: Some(Trace)"#);
        assert_program_args_stdout_string_contains!(&*COMMAND_OS, &["--test", "--verbose"], r#" log_level: Some(Error)"#);
        assert_program_args_stdout_string_contains!(&*COMMAND_OS, &["--test", "--verbose", "--verbose"], r#" log_level: Some(Warn)"#);
        assert_program_args_stdout_string_contains!(&*COMMAND_OS, &["--test", "--verbose", "--verbose", "--verbose"], r#" log_level: Some(Info)"#);
        assert_program_args_stdout_string_contains!(&*COMMAND_OS, &["--test", "--verbose", "--verbose", "--verbose", "--verbose"], r#" log_level: Some(Debug)"#);
        assert_program_args_stdout_string_contains!(&*COMMAND_OS, &["--test", "--verbose", "--verbose", "--verbose", "--verbose", "--verbose"], r#" log_level: Some(Trace)"#);
    }

    /// Test --username.
    ///
    /// Example:
    /// 
    ///     sita --username "alice@example.com"
    /// 
    #[test]
    fn test_username() {
        assert_program_args_stdout_string_contains!(&*COMMAND_OS, &["--test", "-u", "alice@example.com"], r#" username: Some("alice@example.com")"#);
        assert_program_args_stdout_string_contains!(&*COMMAND_OS, &["--test", "--username", "alice@example.com"], r#" username: Some("alice@example.com")"#);
    }

    /// Test --password.
    ///
    /// Example:
    /// 
    ///     sita --password "secret"
    /// 
    #[test]
    fn test_password() {
        assert_program_args_stdout_string_contains!(&*COMMAND_OS, &["--test", "-p", "secret"], r#" password: Some("secret")"#);
        assert_program_args_stdout_string_contains!(&*COMMAND_OS, &["--test", "--password", "secret"], r#" password: Some("secret")"#);
    }

    /// Test --host.
    ///
    /// Example:
    /// 
    ///     sita --host "smtp.example.com"
    /// 
    #[test]
    fn test_host() {
        assert_program_args_stdout_string_contains!(&*COMMAND_OS, &["--test", "--host", "smtp.example.com"], r#" host: Some("smtp.example.com")"#);
    }

    /// Test --port.
    ///
    /// Example:
    /// 
    ///     sita --port 465
    /// 
    #[test]
    fn test_port() {
        assert_program_args_stdout_string_contains!(&*COMMAND_OS, &["--test", "--port", "465"], r#" port: Some(465)"#);
    }

    /// Test --security.
    ///
    /// Example:
    /// 
    ///     sita --security "tls"
    /// 
    #[test]
    fn test_security() {
        assert_program_args_stdout_string_contains!(&*COMMAND_OS, &["--test", "--security", "tls"], r#" security: Some("tls")"#);
    }

    /// Test --ratelimit.
    ///
    /// Example:
    /// 
    ///     sita --ratelimit 0
    /// 
    #[test]
    fn test_ratelimit() {
        assert_program_args_stdout_string_contains!(&*COMMAND_OS, &["--test", "--ratelimit", "0"], r#" ratelimit: Some(0)"#);
    }

}
