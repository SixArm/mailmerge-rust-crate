#[macro_use]
extern crate log;
extern crate env_logger;
extern crate assertables;

pub(crate) mod app; // Application

fn main() {
    env_logger::init();
    match crate::app::run::run() {
        Ok(()) => {
            std::process::exit(0);
        }
        Err(err) => {
            error!("{:?}", err);
            std::process::exit(1);
        }
    }
}
