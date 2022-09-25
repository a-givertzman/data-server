#![allow(non_snake_case)]
use std;
use std::fs;
use std::collections::HashMap;
// use serde_json::{self, Value};
use serde::{Serialize, Deserialize};


#[derive(Debug, Deserialize)]
pub struct Config {
    _path: String,
    // #[serde(flatten)]
    _lines: HashMap<String, Line>,
}

impl Config {
    fn new(path: String) -> Self {
        Self{_path: path, _lines: HashMap::from([])}
    }
    ///
    fn _readFromFile(&self, path: &String) -> HashMap<String, HashMap<String, serde_json::Value>> {
        println!("reading from file: \"{}\"", &path);
        let configJson = fs::read_to_string(&path)
            .expect(&format!("Error read file {}", path));
        let config: HashMap<String, HashMap<String, serde_json::Value>> = serde_json::from_str(&configJson).unwrap();
        println!("config: {:?}", config);
        config
    }
    ///
    fn build(&mut self) {
        let config = self._readFromFile(&self._path);
        self._lines = 
        // for (lineKey, line) in config {
        //     print!("\n\t{}:\t{:?}", &lineKey, line);
        //     self._lines.entry(lineKey).or_insert(Line::new(line));
            // for (iedKey, ied) in line.ieds {
            //     print!("\n\t\t{}:\t{:?}", iedKey, ied);
            // }
        // }    
    }
}

#[derive(Debug, Deserialize)]
pub struct Line {
    // #[serde(flatten)]
    _iedsConfig: HashMap<String, HashMap<String, serde_json::Value>>,
    _ieds: HashMap<String, Ied>,
}
impl Line {
    fn new(iedsConfigs: HashMap<String, HashMap<String, serde_json::Value>>) -> Self {
        Self{_iedsConfig: iedsConfigs, _ieds: HashMap::from([])}
    }
    ///
    fn build(&mut self) {
        for (iedKey, ied) in &self._iedsConfig {
            print!("\n\t{}:\t{:?}", &iedKey, ied);
            self._ieds.entry(iedKey.to_string()).or_insert(ied);
            // for (iedKey, ied) in line.ieds {
            //     print!("\n\t\t{}:\t{:?}", iedKey, ied);
            // }
        }    
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Ied {
    #[serde(flatten)]
    _dbsConfig: HashMap<String, HashMap<String, serde_json::Value>>,
}


#[derive(Serialize, Deserialize)]
pub struct LineConf {
    // ied11: Object(Map<String, Value>),
}

fn main() {
    let dir = std::env::current_dir().unwrap();
    let path: &str = &format!("{}/conf.json", dir.to_str().unwrap());
    let mut config = Config::new(path.to_string());// = serde_json::from_str(&configJson).unwrap();
    println!("config: {:?}", config);
    config.build();
}
