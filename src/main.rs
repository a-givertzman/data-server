#![allow(non_snake_case)]
mod s7;
mod ds;
use std;
use std::env;
use std::collections::HashMap;
use ds::{
    ds_config::DsConfig,
    ds_line::DsLine,
};
use log::{
    info,
    debug,
};

fn main() {
    const logPref: &str = "[main]";
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let dir = std::env::current_dir().unwrap();
    let path: &str = &format!("{}/conf.json", dir.to_str().unwrap());
    let config = DsConfig::new(path.to_string());
    let mut lines = HashMap::new();
    for (lineKey, lineConf) in config.lines {
        debug!("{} line {:?}: ", logPref, lineKey);
        let mut line = DsLine::new(lineConf);
        line.run();
        lines.insert(
            lineKey, 
            line,
        );
    }
    info!("{} all lines started", logPref);
    loop {}
}
