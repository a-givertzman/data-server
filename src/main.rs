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
use crate::s7_parse_point::s7_parse_point::{ParsePointType, S7ParsePointBool, S7ParsePointInt, S7ParsePointReal, ParsePoint};



pub trait Point<T> {
    fn convert(&self) -> T;
}

#[derive(Debug)]
pub struct PointInt {
    v: i16,
}
impl Point<i16> for PointInt {
    fn convert(&self) -> i16 {
        i16::from(16i16)
    }
}

#[derive(Debug)]
pub struct PointBool {
    v: bool,
}
impl Point<bool> for PointBool {
    fn convert(&self) -> bool {
        bool::from(self.v)
    }
}

#[derive(Debug)]
enum PointConfVar {
    PBool(PointBool),
    PInt(PointInt),
    // VecOfString(Vec<String>),
    // AnotherHashMap(HashMap<&'a str, u32>),
}


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
                                // match db.points {
                                //     None => (),
                                //     Some(points) => {
                                //         for (pointKey, point) in points {
                                //             println!("\t\t\tdb {:?}: {:?}", &pointKey, &point);
                                //             let dataType = &point.dataType.clone().unwrap();
                                //             if *dataType == "Bool".to_string() {
                                //                 parsePoints.insert(
                                //                     pointKey.clone(),
                                //                     ParsePointType::Bool(
                                //                         S7ParsePointBool::new(
                                //                             db.name.clone(),
                                //                             pointKey,
                                //                           point,
                                //                         ),
                                //                     ),
                                //                 );
                                //             } else if *dataType == "Int".to_string() {
                                //                 parsePoints.insert(
                                //                     pointKey.clone(),
                                //                     ParsePointType::Int(
                                //                         S7ParsePointInt::new(
                                //                             db.name.clone(),
                                //                             pointKey, 
                                //                             point,
                                //                         ),
                                //                     ),
                                //                 );
                                //             } else if *dataType == "Real".to_string() {
                                //                 parsePoints.insert(
                                //                 pointKey.clone(),
                                //                     ParsePointType::Real(
                                //                         S7ParsePointReal::new(
                                //                             db.name.clone(),
                                //                             pointKey, 
                                //                             point,
                                //                         ),
                                //                     ),
                                //                 );
                                //             }
                                //         }
                                //     }
                                // }
                            }
                        },
                    }
        
                }
            },
        }
    }
    // println!("config {:?}", config);
    // config.build();

    // type fnConv = fn(i32, &str);
    // let toI: fnConv = |a: i32, b: &str| {println!("a: {:#?}", a);};
    // let toS: fnConv = |a: i32, b: &str| {println!("b: {:#?}", b);};
    // let convert = HashMap::from([
    //     ("toInt", &toI), 
    //     ("toStr", &toS), 
    // ]);
    // let vInt = 12;
    // let vStr = "test";
    // let mut conv: fnConv = |_, _| {};
    // if (typeOf(&vInt) == "i32") {
    //     conv = *convert["toInt"];
    // }
    // conv(12, "");
    // if (typeOf(&vStr) == "&str") {
    //     conv = *convert["toStr"];
    // }
    // conv(0, "test");
    let pBool = PointBool{v: true};
    println!("pBool: {:#?}", &pBool);
    let convertedBool = pBool.convert();
    println!("convertedBool: {:#?}", convertedBool);
    let pInt = PointInt{v: 166};
    let convertedInt = pInt.convert();
    println!("convertedInt: {:#?}", convertedInt);
    println!("pInt: {:#?}", pInt);


        // let driveSpeed = toReal(&bytes, 0);
        // println!("driveSpeed: {:#?}", driveSpeed);

        // let driveOutputVoltage = toReal(&bytes, 4);
        // println!("driveOutputVoltage: {:#?}", driveOutputVoltage);

        // let driveDCVoltage = toReal(&bytes, 8);
        // println!("driveDCVoltage: {:#?}", driveDCVoltage);

        // let driveCurrent = toReal(&bytes, 12);
        // println!("driveCurrent: {:#?}", driveCurrent);

        // let driveTorque = toReal(&bytes, 16);
        // println!("driveTorque: {:#?}", driveTorque);
        // let drivepositionFromMru = toReal(&bytes, 20);
        // println!("drivepositionFromMru: {:#?}", drivepositionFromMru);
        // let drivepositionFromHoist = toReal(&bytes, 24);
        // println!("drivepositionFromHoist: {:#?}", drivepositionFromHoist);
        // let capacitorCapacity = toInt(&bytes, 28);
        // println!("capacitorCapacity: {:#?}", capacitorCapacity);
        // let chargeInOn = toBool(&bytes, 30, 0);
        // println!("chargeInOn: {:#?}", chargeInOn);
        // let chargeOutOn = toBool(&bytes, 32, 0);
        // println!("chargeOutOn: {:#?}", chargeOutOn);

}

// fn typeOf<T>(_: &T) -> &str {
//     std::any::type_name::<T>()
// }

// fn toBool(bytes: &Vec<u8>, start: usize, bit: usize) -> bool {
//     let i = toInt(&bytes, start);
//     let b = i >> bit & 1;
//     b > 0
//     // f32::from_be_bytes(bytes[start..end].try_into().unwrap())
// }

// fn toInt(bytes: &Vec<u8>, start: usize) -> i16 {
//     let end = start + 2;
//     i16::from_be_bytes(bytes[start..end].try_into().expect("Conversion error"))
// }

// fn toReal(bytes: &Vec<u8>, start: usize) -> f32 {
//     let end = start + 4;
//     f32::from_be_bytes(bytes[start..end].try_into().unwrap())
// }

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
