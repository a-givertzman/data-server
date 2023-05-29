#![allow(non_snake_case)]

pub mod ds_line {
    use std::{collections::HashMap};
    use log::{
        debug,
    };
    use crate::{
        ds_config::ds_config::{DsLineConf}, 
        ds_ied::ds_ied::DsIed,
    };

    #[derive(Debug)]
    pub struct DsLine {
        pub name: String,
        pub description: Option<String>,
        pub ieds: HashMap<String, DsIed>,
    }
    impl DsLine {
        ///
        pub fn new(
            config: DsLineConf,
        ) -> DsLine {
            let _path = config.name.clone().unwrap();
            let mut ieds: HashMap<String, DsIed> = HashMap::new();
            match config.ieds.clone() {
                None => (),
                Some(confIeds) => {
                    for (iedKey, iedConf) in confIeds {
                        let ied = DsIed::new(iedConf);
                        // debug!("\t\tdb {:?}: {:?}", iedKey, ied);
                        ieds.insert(
                            iedKey,
                            ied,
                        );
                    }
                }
            }
            DsLine {
                name: match config.name { None => String::new(), Some(v) => v },
                description: config.description,
                ieds: ieds,
            }
    
        }
        ///
        pub fn run(&mut self) {
            for (_key, ied) in &mut self.ieds {
                ied.run();
            }
        }
    }
}
