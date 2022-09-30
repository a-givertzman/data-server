#![allow(non_snake_case)]
use std;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::env;
mod ds_config;
mod ds_db;
mod ds_point;
mod s7_parse_point;
use ds_config::ds_config::DsConfig;
use ds_db::ds_db::DsDb;



#[derive(Debug)]
pub struct PointInt<T> {
    v: T,
}

impl PointInt<i16> {
    pub fn convert(&self) -> i16 {
        i16::from(16i16)
    }
}

impl PointInt<bool> {
    pub fn convert(&self) -> bool {
        bool::from(true)
    }
}

// trait Point<T> where T: From<i16> + From<f32>,  {
//     type Output;
//     fn convert() -> T;
// }

// impl<T> Point<T> for PointInt {
//     type Output = i16;
//     fn convert() -> T where T: From<i16>, {
//         i16::from(1)
//     }
// }


fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let dir = std::env::current_dir().unwrap();
    let path: &str = &format!("{}/conf.json", dir.to_str().unwrap());
    let config = DsConfig::new(path.to_string());
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
                                let db = DsDb::new(dbConf);
                                println!("\t\tdb {:?}: {:?}", dbKey, db);
                                // match db.points {
                                //     None => (),
                                //     Some(points) => {
                                //         for (pointKey, point) in points {
                                //             println!("\t\t\tdb {:?}: {:?}", pointKey, point);
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
    let mut client = Client::new(String::from("192.168.120.241"));

    client.connect();

    println!("{:#?}", client);

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

    let pBool = PointInt::<bool>{v: true };
    println!("pBool: {:#?}", &pBool);
    let convertedBool = pBool.convert();
    println!("convertedBool: {:#?}", convertedBool);
    let pInt = PointInt::<i16>{v: 0i16 };
    let convertedInt = pInt.convert();
    println!("convertedInt: {:#?}", convertedInt);
    println!("pInt: {:#?}", pInt);
    loop {
        let bytes = client.read(899, 0, 34).unwrap();
        println!("{:#?}", bytes);

        let driveSpeed = toReal(&bytes, 0);
        println!("driveSpeed: {:#?}", driveSpeed);

        let driveOutputVoltage = toReal(&bytes, 4);
        println!("driveOutputVoltage: {:#?}", driveOutputVoltage);

        let driveDCVoltage = toReal(&bytes, 8);
        println!("driveDCVoltage: {:#?}", driveDCVoltage);

        let driveCurrent = toReal(&bytes, 12);
        println!("driveCurrent: {:#?}", driveCurrent);

        let driveTorque = toReal(&bytes, 16);
        println!("driveTorque: {:#?}", driveTorque);
        let drivepositionFromMru = toReal(&bytes, 20);
        println!("drivepositionFromMru: {:#?}", drivepositionFromMru);
        let drivepositionFromHoist = toReal(&bytes, 24);
        println!("drivepositionFromHoist: {:#?}", drivepositionFromHoist);
        let capacitorCapacity = toInt(&bytes, 28);
        println!("capacitorCapacity: {:#?}", capacitorCapacity);
        let chargeInOn = toBool(&bytes, 30, 0);
        println!("chargeInOn: {:#?}", chargeInOn);
        let chargeOutOn = toBool(&bytes, 32, 0);
        println!("chargeOutOn: {:#?}", chargeOutOn);

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn typeOf<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}


fn toBool(bytes: &Vec<u8>, start: usize, bit: usize) -> bool {
    let i = toInt(&bytes, start);
    let b = i >> bit & 1;
    b > 0
    // f32::from_be_bytes(bytes[start..end].try_into().unwrap())
}

fn toInt(bytes: &Vec<u8>, start: usize) -> i16 {
    let end = start + 2;
    i16::from_be_bytes(bytes[start..end].try_into().expect("Conversion error"))
}

fn toReal(bytes: &Vec<u8>, start: usize) -> f32 {
    let end = start + 4;
    f32::from_be_bytes(bytes[start..end].try_into().unwrap())
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



// #[macro_use]
// extern crate snap7_sys;

use snap7_sys::*;
use std::ffi::CString;
use std::os::raw::{
    c_int,
    c_char,
    c_void,
};

use crate::s7_parse_point::s7_parse_point::S7ParsePoint;

#[derive(Debug)]
struct Client {
    ip: CString,
    handle: S7Object,
    req_len: usize,
    neg_len: usize,
}

impl Client {
    pub fn new(ip: String) -> Self {
        Self {
            ip: CString::new(ip).unwrap(),
            handle: unsafe { Cli_Create() },
            req_len: 0,
            neg_len: 0,
        }
    }

    pub fn connect(&mut self) {

        let mut req: c_int = 0;
        let mut neg: c_int = 0;

        // let ip = CString::new(self.ip).unwrap().as_ptr();
        unsafe {
            #[warn(temporary_cstring_as_ptr)]
            Cli_ConnectTo(self.handle, self.ip.as_ptr(), 0, 1);

            Cli_GetPduLength(self.handle, &mut req, &mut neg);

            self.req_len = req as usize;
            self.neg_len = neg as usize;
        }
    }


    pub fn read(&self, dbNum: u32, start: u32, size: u32) -> Result<Vec<u8>, String> {

        let mut buf = Vec::<u8>::new();

        buf.resize(size as usize, 0);

        let res;
        unsafe {
            res = Cli_DBRead(
                self.handle,
                dbNum as c_int,
                start as c_int,
                size as c_int,
                buf.as_mut_ptr() as *mut c_void
            ) as i32;

        }


        if res == 0 {
            Ok(buf)
        } else {
            Err(String::from(error_text(res)))
        }
    }

    pub fn close(&mut self) {

        unsafe {
            Cli_Disconnect(self.handle);
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {

        self.close();

        unsafe {
            Cli_Destroy(&mut self.handle);
        }
    }
}


pub fn error_text(code: i32) -> String {
    let mut err = Vec::<u8>::new();

        err.resize(1024, 0);

        unsafe {
            Cli_ErrorText(code as c_int, err.as_mut_ptr() as *mut c_char, err.len() as c_int);
        }

        if let Some(i) = err.iter().position(|&r| r == 0) {
            err.truncate(i);
        }

        let err = unsafe {
            std::str::from_utf8_unchecked(&err)
        };

        err.to_owned()
}


// struct CtlRecord {
//     plc_counter: u64,
//     ctl_counter: u64,
// }