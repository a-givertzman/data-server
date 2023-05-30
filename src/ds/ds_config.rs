#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std;
use std::fs;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_with;


// #[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct DsConfig {
    // #[serde(flatten)]
    pub lines: HashMap<String, DsLineConf>,
}
impl DsConfig {
    pub fn new(path: String) -> DsConfig {
        let configJson = fs::read_to_string(&path)
            .expect(&format!("Error read file {}", path));
        let lines: HashMap<String, DsLineConf> = serde_json::from_str(&configJson).unwrap();
        DsConfig{lines}
    }
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DsLineConf {
    // #[serde(flatten)]
    pub name: Option<String>,
    pub description: Option<String>,
    pub ieds: Option<HashMap<String, DsIedConf>>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DsIedConf {
    // #[serde(flatten)]
    pub name: String,
    pub description: Option<String>,
    pub ip: Option<String>,
    pub rack: Option<u32>,
    pub slot: Option<u32>,
    pub dbs: Option<HashMap<String, DsDbConf>>,
}

#[serde_with::skip_serializing_none]
// #[derive(Clone)]: #[derive(Clone)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DsDbConf {
    // #[serde(flatten)]
    pub name: String,
    pub description: Option<String>,
    pub number: Option<u32>,
    pub offset: Option<u32>,
    pub size: Option<u32>,
    pub delay: Option<u32>,
    pub points: Option<HashMap<String, DsPointConf>>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DsPointConf {
    // #[serde(flatten)]
    pub vrt: Option<u8>,
    pub dataType: Option<String>,
    pub offset: Option<u32>,
    pub bit: Option<u8>,
    pub h: Option<u8>,
    pub a: Option<u8>,
    pub comment: Option<String>,
}
