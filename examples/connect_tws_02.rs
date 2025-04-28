// This example demonstrates how to connect to TWS using the `tws` crate.
// It is a simple example that prints "Hello, world!" to the console.
// #![allow(unused_imports)]

#![allow(dead_code)]
#[allow(unused_imports)]
use env_logger::{Builder, Env};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

fn initialize_logger() {
    // The `Env` lets us tweak what the environment
    // variables to read are and what the default
    // value is if they're missing
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        //.filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always")
        .fmt(|buf, record| {
            writeln!(
                buf,
                "{}:{} {} [{}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        });

    env_logger::init_from_env(env);
   
}

fn main() {
    initialize_logger();

    info!("Hello, world!");
    // trace!("some trace log");
    // debug!("some debug log");
    // info!("some information log");
    // warn!("some warning log");
    // error!("some error log");
}
