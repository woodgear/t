use log::*;
use context_attribute::context;
use failure::ResultExt;

#[context(fn)]
fn init_log(config:&str) ->Result<(),failure::Error> {
    use log4rs::{
        config::Config,
        file::{Deserializers, RawConfig},
        Logger,
    };
    
    use serde_yaml;
    let log4rs_config: RawConfig = serde_yaml::from_str(config)?;

    let (appenders, _) = log4rs_config.appenders_lossy(&Deserializers::default());

    let (config, _) = Config::builder()
        .appenders(appenders)
        .loggers(log4rs_config.loggers())
        .build_lossy(log4rs_config.root());

    let log4rs_logger = Logger::new(config);

    let logger = Box::new(log4rs_logger);
    log::set_max_level(log::LevelFilter::Info);
    log::set_boxed_logger(logger)?;
    Ok(())
}

fn app() ->Result<(),failure::Error>{
    init_log(std::include_str!("./log.yaml"))?;
    info!("start");
    Ok(())
}

fn main() {
    if let Err(e)  = app() {
        eprintln!("{:?}",e);
        error!("{:?}",e);
        std::process::exit(-1);
    }
}
