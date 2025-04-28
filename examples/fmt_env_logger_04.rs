// FROM HERE
// https://github.com/MathiasStadler/rust_env_logger/blob/master/examples/fmt_env_logger.rs

use log::{Level, debug, error, info, log_enabled};
#[allow(unused_imports)]
use env_logger::{Builder, Env};
use std::io::Write;

fn init_logger() { 
    
    env_logger::builder()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}:{} {} [{}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();
}

fn main() {
    init_logger();

    debug!("this is a debug {}", "message");
    error!("this is printed by default");
    info!("this is printed by default");

    if log_enabled!(Level::Info) {
        let x = 3 * 4; // expensive computation
        info!("the answer was: {}", x);
    }
}
