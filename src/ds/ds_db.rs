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
    ds_config::{DsDbConf, DsPointConf}, 
    ds_point::DsPoint, ds_status::DsStatus,
};
use crate::s7::{
    s7_client::S7Client,
    s7_parse_point::{ParsePoint, ParsePointType, S7ParsePointBool, S7ParsePointInt, S7ParsePointReal},
};
pub(crate) const MAX_QUEUE_SIZE: usize = 1024 * 16;

#[derive(Debug)]
pub struct DsDb {
    pub name: String,
    pub description: Option<String>,
    pub number: u32,
    pub offset: u32,
    pub size: u32,
    pub delay: u32,
    pub points: Option<HashMap<String, DsPointConf>>,
    localPoints: HashMap<String, ParsePointType>,
    handle: Option<JoinHandle<()>>,
    cancel: bool,
    // sender: Arc<ConcurrentQueue<DsPoint>>,
    // pub receiver: Arc<ConcurrentQueue<DsPoint>>,        
}
impl DsDb {
    ///
    pub fn new(
        config: DsDbConf,
    ) -> DsDb {
        let _path = config.name.clone();
        let mut dbPoints: HashMap<String, ParsePointType> = HashMap::new();
        match config.points.clone() {
            None => (),
            Some(confPoints) => {
                for (pointKey, point) in confPoints {
                    // debug!("\t\t\tdb {:?}: {:?}", pointKey, &point);
                    let dataType = &point.dataType.clone().unwrap();
                    if *dataType == "Bool".to_string() {
                        dbPoints.insert(
                            pointKey.clone(),
                            ParsePointType::Bool(
                                S7ParsePointBool::new(
                                    pointKey.clone(),
                                    pointKey.clone(),
                                    point,
                                ),
                            ),
                        );
                    } else if *dataType == "Int".to_string() {
                        dbPoints.insert(
                            pointKey.clone(),
                            ParsePointType::Int(
                                S7ParsePointInt::new(
                                    pointKey.clone(), 
                                    pointKey.clone(), 
                                    point,
                                ),
                            ),
                        );
                    } else if *dataType == "Real".to_string() {
                        dbPoints.insert(
                            pointKey.clone(),
                            ParsePointType::Real(
                                S7ParsePointReal::new(
                                    pointKey.clone(), 
                                    pointKey.clone(), 
                                    point,
                                ),
                            ),
                        );
                    } else {

                    }
                }
            }
        }
        // let sender = Arc::new(ConcurrentQueue::bounded(MAX_QUEUE_SIZE)); 
        DsDb {
            name: config.name,
            description: config.description,
            number: match config.number { None => 0, Some(v) => v },
            offset: match config.offset { None => 0, Some(v) => v },
            size: match config.size { None => 0, Some(v) => v },
            delay: match config.delay { None => 0, Some(v) => v },
            points: config.points,  // Some(localPoints),
            localPoints: dbPoints,
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
    pub fn run(this: Arc<Mutex<Self>>, client: S7Client) {
        const logPref: &str = "[DsDb.run]";
        info!("{} starting in thread: {:?}...", logPref, thread::current().name().unwrap());
        // let h = &mut self.handle;
        let me = this.clone();
        let me1 = this.clone();
        let delay = this.clone().lock().unwrap().delay as u64;
        let handle = thread::Builder::new().name("DsDb.thread".to_string()).spawn(move || {
            // let sender = me.clone().lock().unwrap().sender.clone();
            while !me.clone().lock().unwrap().cancel {
                let me = me.lock().unwrap();
                let t = Instant::now();
                // let t = Utc::now();
                if client.isConnected {
                    debug!("{} reading DB: {:?}, offset: {:?}, size: {:?}", logPref, me.number, me.offset, me.size);
                    match client.read(me.number, me.offset, me.size) {
                        Ok(bytes) => {
                            // let bytes = client.read(899, 0, 34).unwrap();
                            // print!("\x1B[2J\x1B[1;1H");
                            // debug!("{:#?}", bytes);
                            for (_key, pointType) in &me.localPoints {
                                match pointType.clone() {
                                    ParsePointType::Bool(mut point) => {
                                        point.addRaw(&bytes);
                                        debug!("{} parsed point Bool: {:#?}", logPref, point);
                                        if point.isChanged() {
                                            let dsPoint = DsPoint::newBool(
                                                point.name.as_str(),
                                                false,
                                                DsStatus::Ok,
                                                0,
                                                0,
                                                point.timestamp,
                                            );
                                            // sender.push(value)
                                            debug!("{} point Bool: {:#?}", logPref, point);
                                        }
                                    },
                                    ParsePointType::Int(mut point) => {
                                        point.addRaw(&bytes);
                                        debug!("{} parsed point Int: {:#?}", logPref, point);
                                        if point.isChanged() {
                                            let dsPoint = DsPoint::newInt(
                                                point.name.as_str(),
                                                0,
                                                DsStatus::Ok,
                                                0,
                                                0,
                                                point.timestamp,
                                            );
                                            // sender.push(value)
                                            debug!("{} point Int: {:#?}", logPref, point);
                                        }
                                    },
                                    ParsePointType::Real(mut point) => {
                                        point.addRaw(&bytes);
                                        // debug!("{} parsed point Real: {:#?}", logPref, point);
                                        if point.isChanged() {
                                            let dsPoint = DsPoint::newReal(
                                                point.name.as_str(),
                                                0.0,
                                                DsStatus::Ok,
                                                0,
                                                0,
                                                point.timestamp,
                                            );
                                            debug!("{} point Real: {:#?}", logPref, point);
                                        // sender.push(value)
                                        }
                                    },
                                }
                            }
                        }        
                        Err(err) => {
                            error!("{} client.read error: {}", logPref, err);
                            std::thread::sleep(std::time::Duration::from_millis((delay * 100) as u64));
                        },
                    }
                } else {

                }
                let dt = Instant::now() - t;
                // debug!("{} {:#?} elapsed: {:?} ({:?})", logPref, me.name , dt, dt.as_millis());
                let wait: i128 = (delay as i128) - (dt.as_millis() as i128);
                if wait > 0 {
                    std::thread::sleep(std::time::Duration::from_millis(wait as u64));
                }
                let dt = Instant::now() - t;
                debug!("{} {:#?} elapsed: {:?} ({:?})", logPref, me.name , dt, dt.as_millis());
            }
            info!("{} exit", logPref);
        }).unwrap();
        me1.lock().unwrap().handle = Some(handle);
        info!("{} started", logPref);
    }
}
