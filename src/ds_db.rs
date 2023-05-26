#![allow(non_snake_case)]

pub mod ds_db {
    use std::{collections::HashMap, thread::{self, JoinHandle}, sync::{Arc, Mutex}, any::Any};

    use chrono::Utc;
    use log::{
        info,
        debug,
        error,
    };

    use crate::{
        ds_config::ds_config::{DsDbConf, DsPointConf}, 
        s7_parse_point::s7_parse_point::{ParsePoint, ParsePointType, S7ParsePointBool, S7ParsePointInt, S7ParsePointReal}, 
        s7_client::s7_client::S7Client
    };

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
            }
    
        }
        ///
        // fn read() {

        // }
        ///
        pub fn start(this: Arc<Mutex<Self>>, client: S7Client) {
            const logPref: &str = "[DsDb.start]";
            info!("{} starting in thread: {:?}...", logPref, thread::current().name().unwrap());
            // let h = &mut self.handle;
            let me = this.clone();
            let me1 = this.clone();
            let handle = thread::Builder::new().name("DsDb.thread".to_string()).spawn(move || {
                let me = me.lock().unwrap();
                let cancel = me.cancel;
                while !cancel {
                    let t = Utc::now();
                    match client.read(me.number, me.offset, me.size) {
                        Ok(bytes) => {
                            // let bytes = client.read(899, 0, 34).unwrap();
                            // print!("\x1B[2J\x1B[1;1H");
                            // debug!("{:#?}", bytes);
                            for (_key, pointType) in &me.localPoints {
                                match pointType.clone() {
                                    ParsePointType::Bool(mut point) => {
                                        point.addRaw(&bytes);
                                        debug!("{} point Bool: {:#?}", logPref, point);
                                    },
                                    ParsePointType::Int(mut point) => {
                                        point.addRaw(&bytes);
                                        debug!("{} point Int: {:#?}", logPref, point);
                                    },
                                    ParsePointType::Real(mut point) => {
                                        point.addRaw(&bytes);
                                        debug!("{} point Real: {:#?}", logPref, point);
                                    },
                                }
                            }
                        }        
                        Err(err) => {
                            error!("{} client.read error: {}", logPref, err);
                            std::thread::sleep(std::time::Duration::from_millis(me.delay as u64));
                        },
                    }
                    let dt = Utc::now() - t;
                    debug!("{} {:#?} elapsed: {:?}sec {:?}ms", logPref, me.name , dt.num_seconds(), dt.num_milliseconds());
                    std::thread::sleep(std::time::Duration::from_millis(me.delay as u64));
                }
                info!("{} exit", logPref);
            }).unwrap();
            me1.lock().unwrap().handle = Some(handle);
            info!("{} started", logPref);
        }
    }
}
