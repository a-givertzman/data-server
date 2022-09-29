#![allow(non_snake_case)]
use std;
use std::env;
mod ds_config;
mod ds_db;
mod ds_point;
use ds_config::ds_config::DsConfig;
use ds_db::ds_db::DsDb;


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

    loop {
        println!("{:#?}", client.read(899, 0, 34));
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

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