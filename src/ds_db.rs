#![allow(non_snake_case)]

pub mod ds_db {
    use std::collections::{HashMap, hash_map::RandomState};

    use crate::{
        s7_client::s7_client::S7Client,
        ds_config::ds_config::{
            DsDbConf, 
            DsPointConf
        }, 
        s7_parse_point::s7_parse_point::{
            ParsePoint,
            ParsePointType, 
            S7ParsePointBool, 
            S7ParsePointInt, 
            S7ParsePointReal
        }
    };

    #[derive(Debug)]
    pub struct DsDb {
        parsePoints: HashMap<String, ParsePointType, RandomState>,
        pub name: String,
        pub description: Option<String>,
        pub number: Option<u32>,
        pub offset: Option<u32>,
        pub size: Option<u32>,
        pub delay: Option<u32>,
        pub points: Option<HashMap<String, DsPointConf>>,
    }
    impl DsDb {
        pub fn new(
            config: DsDbConf,
        ) -> DsDb {
            DsDb {
                parsePoints: HashMap::new(),
                name: config.name,
                description: config.description,
                number: config.number,
                offset: config.offset,
                size: config.size,
                delay: config.delay,
                points: config.points,  // Some(localPoints),
            }
        }
        pub fn run(&mut self) {
            // let mut parsePoints = HashMap::new();
            let path = &self.name;
            match &self.points {
                None => (),
                Some(points) => {
                    for (pointKey, point) in points.clone() {
                        println!("\t\t\tdb {:?}: {:?}", &pointKey, &point);
                        let dataType = &point.dataType.clone().unwrap();
                        if *dataType == "Bool".to_string() {
                            self.parsePoints.insert(
                                pointKey.clone(),
                                ParsePointType::Bool(
                                    S7ParsePointBool::new(
                                        path.to_string(),
                                        pointKey.to_string(),
                                      point,
                                    ),
                                ),
                            );
                        } else if *dataType == "Int".to_string() {
                            self.parsePoints.insert(
                                pointKey.clone(),
                                ParsePointType::Int(
                                    S7ParsePointInt::new(
                                        path.to_string(),
                                        pointKey.to_string(), 
                                        point,
                                    ),
                                ),
                            );
                        } else if *dataType == "Real".to_string() {
                            self.parsePoints.insert(
                            pointKey.clone(),
                                ParsePointType::Real(
                                    S7ParsePointReal::new(
                                        path.to_string(),
                                        pointKey.to_string(), 
                                        point,
                                    ),
                                ),
                            );
                        }
                    }
                }
            }
        
            let mut client = S7Client::new(String::from("192.168.120.241"));

            client.connect();
        
            // println!("parsePoints: {:#?}", parsePoints);
            println!("client: {:#?}", client);
        
            loop {
                let bytes = client.read(899, 0, 34).unwrap();
                print!("\x1B[2J\x1B[1;1H");
                println!("{:#?}", bytes);
                for (key, pointType) in &self.parsePoints {
                    match pointType.clone() {
                        ParsePointType::Bool(mut point) => {
                            point.addRaw(&bytes);
                            println!("point {:#?} Bool: {:#?}", key, point);
                        },
                        ParsePointType::Int(mut point) => {
                            point.addRaw(&bytes);
                            println!("point {:#?} Int: {:#?}", key, point);
                            println!("point Int: {:#?}", point);
                        },
                        ParsePointType::Real(mut point) => {
                            point.addRaw(&bytes);
                            println!("point {:#?} Real: {:#?}", key, point);
                        },
                    }
                }
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
    }
}
