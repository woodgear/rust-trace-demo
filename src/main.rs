mod util;
use failure::Fail;
use sugar::SResultExt;
use tracing::{error, info};

pub fn my_function(my_arg: usize) -> Result<(), failure::Error> {
    b()?;
    c()?;
    e_open_file()?;
    if my_arg == 0 {
        e_c()?;
    }
    e_c()?;
    Ok(())
}

fn b() -> Result<(), failure::Error> {
    Ok(())
}

fn c() -> Result<(), failure::Error> {
    Ok(())
}

fn e_c() -> Result<(), failure::Error> {
    Err(failure::format_err!("xxx"))
}

fn e_open_file() -> Result<(), failure::Error> {
    other_crate::other()?;
    std::fs::read_to_string("./xxx")?;
    Ok(())
}

use saber_tools::FailureErrorExt;

mod errors;
use errors::{ErrKind, LogErrorExt};
use std::fmt::Display;

fn test_sentry() {
    // sentry::capture_message("Hello World! again", sentry::Level::Info);
}

fn app() -> Result<(), failure::Error> {
    info!("start");
    // test_sentry();
    my_function(1)?;
    Ok(())
}

fn main() {
    let _guard = sentry::init("http://f0aef0d814e54bafad2d14d3e0cb80ca@localhost:9000/1");
    let _trace = util::init_trace();
    app().log_err(ErrKind::AppFail, "oh my god app fail");
    // app().log_err(ErrKind::OtherFail, "oh my god other fail");
}
