#![allow(non_snake_case)]

pub mod ds_db {
    use std::{collections::HashMap, thread::{self, JoinHandle}};

    use chrono::Utc;

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
    }
    impl DsDb {
        ///
        pub fn new(
            config: DsDbConf,
        ) -> DsDb {
            let path = config.name.clone().unwrap();
            let mut dbPoints: HashMap<String, ParsePointType> = HashMap::new();
            match config.points.clone() {
                None => (),
                Some(confPoints) => {
                    for (pointKey, point) in confPoints {
                        // println!("\t\t\tdb {:?}: {:?}", pointKey, &point);
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
                name: match config.name { None => String::from("Unnamed DsDb"), Some(v) => v },
                description: config.description,
                number: match config.number { None => 0, Some(v) => v },
                offset: match config.offset { None => 0, Some(v) => v },
                size: match config.size { None => 0, Some(v) => v },
                delay: match config.delay { None => 0, Some(v) => v },
                points: config.points,  // Some(localPoints),
                localPoints: dbPoints,
            }
    
        }
        ///
        fn read() {

        }
        ///
        pub fn start(&mut self, client: S7Client) {
            // let h = &mut self.handle;
            thread::scope(|s| {
                s.spawn(move || {
                    loop {
                        let t = Utc::now();
                        let result = client.read(self.number, self.offset, self.size);
                        match result {
                            Err(err) => println!("client.read error: {}", err),
                            Ok(bytes) => {
                                // let bytes = client.read(899, 0, 34).unwrap();
                                // print!("\x1B[2J\x1B[1;1H");
                                // println!("{:#?}", bytes);
                                for (key, pointType) in &self.localPoints {
                                    match pointType.clone() {
                                        ParsePointType::Bool(mut point) => {
                                            point.addRaw(&bytes);
                                            println!("point Bool: {:#?}", point);
                                        },
                                        ParsePointType::Int(mut point) => {
                                            point.addRaw(&bytes);
                                            println!("point Int: {:#?}", point);
                                        },
                                        ParsePointType::Real(mut point) => {
                                            point.addRaw(&bytes);
                                            println!("point Real: {:#?}", point);
                                        },
                                    }
                                }
                            }        
                        }
                        let dt = Utc::now() - t;
                        println!("{:#?} elapsed: {:?}sec {:?}ms",self.name , dt.num_seconds(), dt.num_milliseconds());
                        std::thread::sleep(std::time::Duration::from_millis(self.delay as u64));
                    }
                });
            }); 
        }
        ///
        pub fn join(&mut self) {
            // match &mut self.handle {
            //     None => (),
            //     Some(handle) => *handle.join().unwrap(),
            // }
        }
    }
}
