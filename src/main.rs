#![allow(non_snake_case)]
use std;
use std::fs;
use std::collections::HashMap;
// use serde_json::{self, Value};
use serde::{Serialize, Deserialize};
use serde_with;


// #[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct DsConfig {
    // #[serde(flatten)]
    lines: HashMap<String, DsLineConf>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct DsLineConf {
    // #[serde(flatten)]
    name: Option<String>,
    ieds: Option<HashMap<String, DsIedConf>>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct DsIedConf {
    // #[serde(flatten)]
    name: Option<String>,
    ip: Option<String>,
    rack: Option<u32>,
    slot: Option<u32>,
    dbs: Option<HashMap<String, DsDbConf>>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct DsDbConf {
    // #[serde(flatten)]
    name: Option<String>,
    description: Option<String>,
    number: Option<u32>,
    offset: Option<u32>,
    size: Option<u32>,
    delay: Option<u32>,
    points: Option<HashMap<String, DsPointConf>>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct DsPointConf {
    // #[serde(flatten)]
    dataType: Option<String>,
    offset: Option<u32>,
    comment: Option<String>,
    vrt: Option<u8>,
}

fn main() {
    let dir = std::env::current_dir().unwrap();
    let path: &str = &format!("{}/conf.json", dir.to_str().unwrap());
    let configJson = fs::read_to_string(&path)
        .expect(&format!("Error read file {}", path));
    let config: DsConfig = serde_json::from_str(&configJson).unwrap();
    // for (key, line) in config.lines {
    //     println!("line {:?}: {:?}", key, line);
    // }
    println!("config {:?}", config);
    // config.build();
}
//
// fn build(&mut self) {
//     let configJson = self._readFromFile(&self._path);
//     let config: HashMap<String, HashMap<String, serde_json::Value>> = serde_json::from_str(&configJson).unwrap();
//     for (lineKey, lineConf) in config {
//         // print!("\n\t{}:\t{:?}", &lineKey, lineConf);
//         self._lines.insert(
//             lineKey, 
//             Line::new(lineConf),
//         );
//         // for (iedKey, ied) in line.entry("ieds") {
//         //     print!("\n\t\t{}:\t{:?}", iedKey, ied);
//         // }
//     }    
// }
// ///
// fn _readFromFile(&self, path: &String) -> String {
//     println!("reading from file: \"{}\"", &path);
//     let configJson = fs::read_to_string(&path)
//         .expect(&format!("Error read file {}", path));
//     println!("configJson: {:?}", configJson);
//     configJson
//     // : HashMap<String, HashMap<String, serde_json::Value>>
// }
