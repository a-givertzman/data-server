#![allow(non_snake_case)]

pub mod ds_config {
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
        
            DsConfig { lines:  lines}
        }
    }
    
    #[serde_with::skip_serializing_none]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DsLineConf {
        // #[serde(flatten)]
        pub name: Option<String>,
        pub ieds: Option<HashMap<String, DsIedConf>>,
    }
    
    #[serde_with::skip_serializing_none]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DsIedConf {
        // #[serde(flatten)]
        pub name: Option<String>,
        pub description: Option<String>,
        pub ip: Option<String>,
        pub rack: Option<u32>,
        pub slot: Option<u32>,
        pub dbs: Option<HashMap<String, DsDbConf>>,
    }
    
    #[serde_with::skip_serializing_none]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DsDbConf {
        // #[serde(flatten)]
        pub name: Option<String>,
        pub description: Option<String>,
        pub number: Option<u32>,
        pub offset: Option<u32>,
        pub size: Option<u32>,
        pub delay: Option<u32>,
        pub points: Option<HashMap<String, DsPointConf>>,
    }
    
    #[serde_with::skip_serializing_none]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DsPointConf {
        // #[serde(flatten)]
        pub dataType: Option<String>,
        pub offset: Option<u32>,
        pub comment: Option<String>,
        pub vrt: Option<u8>,
    }
    
}