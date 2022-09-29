#![allow(non_snake_case)]

pub mod ds_db {
    use std::collections::HashMap;

    use crate::{ds_config::ds_config::{DsDbConf}, ds_point::ds_point::DsPoint};

    #[derive(Debug)]
    pub struct DsDb {
        pub name: Option<String>,
        pub description: Option<String>,
        pub number: Option<u32>,
        pub offset: Option<u32>,
        pub size: Option<u32>,
        pub delay: Option<u32>,
        pub points: Option<HashMap<String, DsPoint>>,
    }
    impl DsDb {
        pub fn new(
            config: DsDbConf,
        ) -> DsDb {
            let mut localPoints: HashMap<String, DsPoint> = HashMap::new();
            let path = config.name.clone().unwrap();
            match config.points {
                None => (),
                Some(points) => {
                    for (pointKey, pointConf) in points {
                        // println!("\t\t\tdb {:?}: {:?}", &pointKey, pointConf);
                        localPoints.insert(
                            pointKey.clone(),
                            DsPoint::new(
                                pointKey, 
                                path.clone(), 
                                pointConf
                            ),
                        );
                    }
                }
            }
            DsDb {
                name: config.name,
                description: config.description,
                number: config.number,
                offset: config.offset,
                size: config.size,
                delay: config.delay,
                points: Some(localPoints),
            }
    
        }
    }
}
