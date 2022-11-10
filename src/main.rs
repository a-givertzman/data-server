#![allow(non_snake_case)]
use std;
use std::env;
use std::collections::HashMap;
mod s7_client;
mod ds_config;
mod ds_line;
mod ds_ied;
mod ds_db;
mod ds_point;
mod s7_parse_point;
use ds_config::ds_config::DsConfig;
use crate::ds_line::ds_line::DsLine;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let dir = std::env::current_dir().unwrap();
    let path: &str = &format!("{}/conf.json", dir.to_str().unwrap());
    let config = DsConfig::new(path.to_string());
    let mut lines = HashMap::new();
    for (lineKey, lineConf) in config.lines {
        println!("line {:?}: ", lineKey);
        let mut line = DsLine::new(lineConf);
        line.start();
        lines.insert(
            lineKey, 
            line,
        );
    }
}
