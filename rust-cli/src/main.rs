#![deny(warnings)]
#![allow(clippy::needless_return)]
#![allow(clippy::cast_ptr_alignment)]

mod util;

use tracing::{error, info};

fn app() -> Result<(), failure::Error> {
    Ok(())
}

fn main() {
    util::init_trace();
    info!("start");

    if let Err(e) = app() {
        eprintln!("{:?}", e);
        error!("{:?}", e);
        std::process::exit(-1);
    }
}
