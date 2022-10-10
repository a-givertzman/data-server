#![allow(non_snake_case)]
use std;
use std::collections::HashMap;
use std::env;
mod ds_config;
mod ds_db;
mod ds_point;
mod s7_parse_point;
pub mod ds_s7_client;
use ds_config::ds_config::DsConfig;
use ds_db::ds_db::DsDb;


fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let dir = std::env::current_dir().unwrap();
    let path: &str = &format!("{}/conf.json", dir.to_str().unwrap());
    let config = DsConfig::new(path.to_string());
    let mut localDbs: HashMap<String, DsDb> = HashMap::new();
    for (lineKey, line) in config.lines {
        println!("line {:?}: ", lineKey);
        match line.ieds {
            None => (),
            Some(ieds) => {
                for (iedKey, ied) in ieds {
                    println!("\tied {:?}: ", iedKey);
                    match ied.dbs {
                        None => (),
                        Some(dbs) => {
                            for (dbKey, dbConf) in dbs {
                                let mut db = DsDb::new(dbConf);
                                println!("\t\tdb {:?}: {:?}", dbKey, &db);
                                db.run();
                                localDbs.insert(dbKey, db);
                            }
                        },
                    }
        
                }
            },
        }
    }
}
