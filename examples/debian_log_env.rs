// FROM HERE
// https://github.com/rust-cli/env_logger/blob/main/examples/default.rs

/*!
Using `env_logger`.

Before running this example, try setting the `MY_LOG_LEVEL` environment variable to `info`:

```no_run,shell
$ export MY_LOG_LEVEL='info'
```

Also try setting the `MY_LOG_STYLE` environment variable to `never` to disable colors
or `auto` to enable them:

```no_run,shell
$ export MY_LOG_STYLE=never
```
*/

use log::{debug, error, info, trace, warn};

#[allow(unused_imports)]
use env_logger::{Builder, Env};

fn main() {
    init_logger();

    trace!("some trace log");
    debug!("some debug log");
    info!("some information log");
    warn!("some warning log");
    error!("some error log");
}

fn init_logger() {
    // The `Env` lets us tweak what the environment
    // variables to read are and what the default
    // value is if they're missing
    // let env = Env::default()
    //     //.filter_or("MY_LOG_LEVEL", "trace")
    //     .filter_or("MY_LOG_LEVEL", "info")
    //     .write_style_or("MY_LOG_STYLE", "always");

    // env_logger::init_from_env(env);

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
