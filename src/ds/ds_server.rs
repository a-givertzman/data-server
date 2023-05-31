#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::{
    collections::HashMap, 
    thread::{self, JoinHandle}, sync::Arc,
};

use concurrent_queue::ConcurrentQueue;
use log::{
    info,
    debug,
    // error,
};

use crate::ds::{
    ds_config::DsConfig, 
    ds_line::DsLine, ds_point::DsPoint,
};

///
/// 
#[derive(Debug)]
pub struct DsServer {
    pub name: String,
    pub description: Option<String>,
    pub config: DsConfig,
    lines: HashMap<String, DsLine>,
    pub queues: Vec<Arc<ConcurrentQueue<DsPoint>>>,
    // handle: Option<JoinHandle<()>>,
    // cancel: bool,
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
        DsServer {
            name: "DsServer".to_string(),   // config.name
            description: Some("DsServer".to_string()), // config.description,
            config: config,
            lines: HashMap::new(),
            queues: vec![],
            // handle: None,
            // cancel: false,
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
        // let mut receivers: Vec<Arc<ConcurrentQueue<DsPoint>>>  = vec![];
        for (lineKey, lineConf) in &(self.config.lines) {
            debug!("{} line {:?}: ", logPref, lineKey);
            let mut line = DsLine::new((*lineConf).clone());
            for (_iedKey, ied) in &line.ieds {
                for (_dbKey, db) in &ied.dbs {
                    let rcv = &db.lock().unwrap().receiver;
                    self.queues.push(rcv.clone());
                }
            }
            line.run();
            self.lines.insert(
                lineKey.clone(), 
                line,
            );
        }
        info!("{} all lines started", logPref);
    }
}
