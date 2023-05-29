#![allow(non_snake_case)]

use std::{
    sync::{Arc, Mutex},
    collections::HashMap,
};
use log::{
    // info,
    debug,
    // error,
};
use crate::ds::{
    ds_db::DsDb,
    ds_config::DsIedConf, 
};
use crate::s7::{
    s7_client::S7Client,
};

#[derive(Debug)]
pub struct DsIed {
    pub name: String,
    pub description: Option<String>,
    pub ip: String,
    pub rack: u32,
    pub slot: u32,
    pub dbs: HashMap<String, Arc<Mutex<DsDb>>>,
}
impl DsIed {
    ///
    pub fn new(
        config: DsIedConf,
    ) -> DsIed {
        let _path = config.name.clone();
        let mut dbs: HashMap<String, Arc<Mutex<DsDb>>> = HashMap::new();
        match config.dbs.clone() {
            None => (),
            Some(confDbs) => {
                for (dbKey, dbConf) in confDbs {
                    let db = Arc::new(Mutex::new(DsDb::new(dbConf)));
                    // debug!("\t\tdb {:?}: {:#?}", dbKey, db);
                    dbs.insert(
                        dbKey, 
                        db,
                    );
                }
            }
        }
        DsIed {
            name: config.name,
            description: config.description,
            ip: match config.ip { None => String::new(), Some(v) => v },
            rack: match config.rack { None => 0, Some(v) => v },
            slot: match config.slot { None => 0, Some(v) => v },
            dbs: dbs,
        }

    }
    ///
    pub fn run(&mut self) {
        const logPref: &str = "[DsIed.run]";
        for (_key, db) in &self.dbs {
            let mut client = S7Client::new(
                self.ip.clone(),
                None,
            );
            // debug!("{} client: {:#?}", logPref, client);
            client.connect();
            // debug!("{} client: {:#?}", logPref, client);
            DsDb::run(db.clone(), client);
        }
    }
}

