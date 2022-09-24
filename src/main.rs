#![allow(non_snake_case)]
use std;
use std::fs;
use std::collections::HashMap;
// use serde_json::{self, Value};
use serde::{Serialize, Deserialize};


#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    lines: HashMap<String, Line>,
}

#[derive(Debug, Deserialize)]
pub struct Line {
    #[serde(flatten)]
    ieds: HashMap<String, HashMap<String, serde_json::Value>>,
}

#[derive(Serialize, Deserialize)]
pub struct LineConf {
    // ied11: Object(Map<String, Value>),
}

fn main() {
    let path = std::env::current_dir().unwrap();
    let name: &str = &format!("{}/conf.json", path.to_str().unwrap());
    println!("opening the file: \"{}\"", &name);
    let configJson = fs::read_to_string(&name)
        .expect(&format!("Error read file {}", name));
    let config: Config = serde_json::from_str(&configJson).unwrap();
    println!("config: {:?}", config);
    for (lineKey, line) in config.lines {
        print!("\n\t{}:\t{:?}", lineKey, line);
        for (iedKey, ied) in line.ieds {
            print!("\n\t\t{}:\t{:?}", iedKey, ied);
        }
    }
}
