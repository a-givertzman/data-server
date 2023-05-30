#![allow(non_snake_case)]

use std::{collections::HashMap, thread::{self, JoinHandle}, sync::{Arc, Mutex}, any::Any, time::Instant};

use chrono::Utc;
use concurrent_queue::ConcurrentQueue;
use log::{
    info,
    debug,
    error,
};

use crate::ds::{
    ds_config::DsConfig, 
    ds_point::DsPoint, ds_status::DsStatus,
    ds_line::DsLine,
};
use crate::s7::{
    s7_client::S7Client,
    s7_parse_point::{ParsePoint, ParsePointType},
};
pub(crate) const MAX_QUEUE_SIZE: usize = 1024 * 16;

#[derive(Debug)]
pub struct DsServer {
    pub name: String,
    pub description: Option<String>,
    pub config: DsConfig,
    lines: HashMap<String, DsLine>,
    handle: Option<JoinHandle<()>>,
    cancel: bool,
    // sender: Arc<ConcurrentQueue<DsPoint>>,
    // pub receiver: Arc<ConcurrentQueue<DsPoint>>,        
}
impl DsServer {
    ///
    pub fn new(
    ) -> DsServer {
        let dir = std::env::current_dir().unwrap();
        let path: &str = &format!("{}/conf.json", dir.to_str().unwrap());
        let config = DsConfig::new(path.to_string());
        let mut lines = HashMap::new();
        DsServer {
            name: "DsServer".to_string(),   // config.name
            description: Some("DsServer".to_string()), // config.description,
            config: config,
            lines: lines,
            handle: None,
            cancel: false,
            // sender: sender.clone(),
            // receiver: sender,                
        }

    }
    ///
    // fn read() {

    // }
    ///
    pub fn run(&mut self) {
        const logPref: &str = "[DsServer.run]";
        info!("{} starting in thread: {:?}...", logPref, thread::current().name().unwrap());
        for (lineKey, lineConf) in &(self.config.lines) {
            debug!("{} line {:?}: ", logPref, lineKey);
            let mut line = DsLine::new((*lineConf).clone());
            line.run();
            self.lines.insert(
                lineKey.clone(), 
                line,
            );
        }
        info!("{} all lines started", logPref);
    }
}
