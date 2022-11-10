#![allow(non_snake_case)]

pub mod ds_ied {
    use std::{collections::HashMap};

    use crate::{
        ds_config::ds_config::{DsIedConf}, 
        ds_db::ds_db::DsDb,
        s7_client::s7_client::S7Client
    };

    #[derive(Debug)]
    pub struct DsIed {
        pub name: String,
        pub description: Option<String>,
        pub ip: String,
        pub rack: u32,
        pub slot: u32,
        pub dbs: HashMap<String, DsDb>,
    }
    impl DsIed {
        ///
        pub fn new(
            config: DsIedConf,
        ) -> DsIed {
            let path = config.name.clone().unwrap();
            let mut dbs: HashMap<String, DsDb> = HashMap::new();
            match config.dbs.clone() {
                None => (),
                Some(confDbs) => {
                    for (dbKey, dbConf) in confDbs {
                        let db = DsDb::new(dbConf);
                        println!("\t\tdb {:?}: {:#?}", dbKey, db);
                        dbs.insert(
                            dbKey, 
                            db,
                        );
                    }
                }
            }
            DsIed {
                name: match config.name { None => String::new(), Some(v) => v },
                description: config.description,
                ip: match config.ip { None => String::new(), Some(v) => v },
                rack: match config.rack { None => 0, Some(v) => v },
                slot: match config.slot { None => 0, Some(v) => v },
                dbs: dbs,
            }
    
        }
        ///
        pub fn start(&mut self) {
            for (key, db) in &mut self.dbs {
                let mut client = S7Client::new(self.ip.clone());
                println!("client: {:#?}", client);
                client.connect();
                println!("client: {:#?}", client);
                db.start(client);
            }
        }
    }
}
