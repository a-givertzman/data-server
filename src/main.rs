#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
mod tests;
mod s7;
mod ds;
use std;
use std::env;
use log::{
    info,
    debug,
};

use crate::ds::ds_server::DsServer;

fn main() {
    const logPref: &str = "[main]";
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    // let dir = std::env::current_dir().unwrap();
    // let path: &str = &format!("{}/conf.json", dir.to_str().unwrap());
    // let config = DsConfig::new(path.to_string());
    // let mut lines = HashMap::new();
    // for (lineKey, lineConf) in config.lines {
    //     debug!("{} line {:?}: ", logPref, lineKey);
    //     let mut line = DsLine::new(lineConf);
    //     line.run();
    //     lines.insert(
    //         lineKey, 
    //         line,
    //     );
    // }
    // info!("{} all lines started", logPref);
    info!("{} starting application", logPref);
    let mut dsServer = DsServer::new();
    dsServer.run();
    loop {
        for queue in &dsServer.queues {
            while !queue.is_empty() {
                let point = queue.pop().unwrap();
                debug!("{} point ({:?}): {:?} {:?}", logPref, point.dataType, point.name, point.value);
            }
        }
    }
}
