#![allow(non_snake_case)]
use std;
mod s7_client;
mod ds_config;
mod ds_line;
mod ds_ied;
mod ds_db;
mod ds_point;
mod s7_parse_point;
use std::env;
use std::collections::HashMap;
use ds_config::ds_config::DsConfig;
use crate::ds_line::ds_line::DsLine;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let dir = std::env::current_dir().unwrap();
    let path: &str = &format!("{}/conf.json", dir.to_str().unwrap());
    let config = DsConfig::new(path.to_string());
    let mut lines = HashMap::new();
    for (lineKey, lineConf) in config.lines {
        println!("line {:?}: ", lineKey);
        let mut line = DsLine::new(lineConf);
        line.start();
        lines.insert(
            lineKey, 
            line,
        );
        // match lineConf.ieds {
        //     None => (),
        //     Some(ieds) => {
        //         for (iedKey, iedConf) in ieds {
        //             let ied = DsIed::new(iedConf);
        //             println!("\tied {:?}: ", iedKey);
                    // match ied.dbs {
                    //     None => (),
                    //     Some(dbs) => {
                    //         for (dbKey, dbConf) in dbs {
                    //             let db = DsDb::new(dbConf);
                    //             println!("\t\tdb {:?}: {:?}", dbKey, db);
                                // match db.points {
                                //     None => (),
                                //     Some(points) => {
                                //         for (pointKey, point) in points {
                                //             println!("\t\t\tdb {:?}: {:?}", pointKey, &point);
                                //             let dataType = &point.dataType.clone().unwrap();
                                //             if *dataType == "Bool".to_string() {
                                //                 parsePoints.insert(
                                //                     pointKey.clone(),
                                //                     ParsePointType::Bool(
                                //                         S7ParsePointBool::new(
                                //                             pointKey.clone(),
                                //                             pointKey.clone(),
                                //                           point,
                                //                         ),
                                //                     ),
                                //                 );
                                //             } else if *dataType == "Int".to_string() {
                                //                 parsePoints.insert(
                                //                     pointKey.clone(),
                                //                     ParsePointType::Int(
                                //                         S7ParsePointInt::new(
                                //                             pointKey.clone(), 
                                //                             pointKey.clone(), 
                                //                             point,
                                //                         ),
                                //                     ),
                                //                 );
                                //             } else if *dataType == "Real".to_string() {
                                //                 parsePoints.insert(
                                //                     pointKey.clone(),
                                //                     ParsePointType::Real(
                                //                         S7ParsePointReal::new(
                                //                             pointKey.clone(), 
                                //                             pointKey.clone(), 
                                //                             point,
                                //                         ),
                                //                     ),
                                //                 );
                                //             }
                                //         }
                                //     }
                                // }
                    //         }
                    //     },
                    // }
        
        //         }
        //     },
        // }
    }
    // println!("config {:?}", config);
    // config.build();
    // let mut client = S7Client::new(String::from("192.168.120.243"));

    // client.connect();

    // println!("parsePoints: {:#?}", parsePoints);
    // println!("client: {:#?}", client);

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
    // let pBool = PointBool{v: true};
    // println!("pBool: {:#?}", &pBool);
    // let convertedBool = pBool.convert();
    // println!("convertedBool: {:#?}", convertedBool);
    // let pInt = PointInt{v: 166};
    // let convertedInt = pInt.convert();
    // println!("convertedInt: {:#?}", convertedInt);
    // println!("pInt: {:#?}", pInt);
    // loop {
    //     let t = Utc::now();
    //     let result = client.read(899, 0, 34);
    //     match result {
    //         Err(err) => println!("client.read error: {}", err),
    //         Ok(bytes) => {
    //             // let bytes = client.read(899, 0, 34).unwrap();
    //             // print!("\x1B[2J\x1B[1;1H");
    //             println!("{:#?}", bytes);
    //             for (key, pointType) in &parsePoints {
    //                 match pointType.clone() {
    //                     ParsePointType::Bool(mut point) => {
    //                         point.addRaw(&bytes);
    //                         println!("point Bool: {:#?}", point);
    //                     },
    //                     ParsePointType::Int(mut point) => {
    //                         point.addRaw(&bytes);
    //                         println!("point Int: {:#?}", point);
    //                     },
    //                     ParsePointType::Real(mut point) => {
    //                         point.addRaw(&bytes);
    //                         println!("point Real: {:#?}", point);
    //                     },
    //                 }
    //             }
        
        
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
        //     },
        // }
        // let dt = Utc::now() - t;
        // println!("elapsed: {:?}sec {:?}ms", dt.num_seconds(), dt.num_milliseconds());
        // std::thread::sleep(std::time::Duration::from_millis(1));
    // }
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
