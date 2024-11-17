//! Args for the application.
//!
//! These args correspond to the matches in the file `clap.rs`.
//! We have these args in their own file in order to be flexible,
//! such as being able to start our app with other arg parsers.

#[derive(Debug)]
pub struct Args {

    /// Test flag that sets whether the app prints diagnostics.
    /// Example: true means print diagnostics.
    pub(crate) test: bool,

    /// Log level: 0=none, 1=error, 2=warn, 3=info, 4=debug, 5=trace.
    /// Example: 5 means print debug diagnostics.
    pub(crate) log_level: Option<::log::Level>,

    /// The username to use.
    pub(crate) username: Option<String>,

    /// The password to use.
    pub(crate) password: Option<String>,

    /// The host to connect to.
    pub(crate) host: Option<String>,

    /// The port to connect to.
    pub(crate) port: Option<u16>,
    
    /// The security to use.
    pub(crate) security: Option<String>,

    /// The ratelimit to use.
    pub(crate) ratelimit: Option<u16>,

}

impl std::default::Default for Args {
    fn default() -> Self { Self {
        test: false,
        log_level: None,
        username: None,
        password: None,
        host: None,
        port: None,
        security: None,
        ratelimit: None,
    }}
}
