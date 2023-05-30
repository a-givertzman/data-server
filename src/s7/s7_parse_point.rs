#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::array::TryFromSliceError;

use chrono::{DateTime, Utc};
use log::{
    // info, 
    debug, 
    warn,
    // error, 
};

use crate::ds::{ds_config::{DsPointConf}, ds_status::DsStatus};


// #[derive(Debug, Clone)]
// pub struct ParsePoint<T> {
//     v: T,
//     pub vrt: Option<u8>,
//     pub dataType: Option<String>,
//     pub offset: Option<u32>,
//     pub bit: Option<u8>,
//     pub h: Option<u8>,
//     pub a: Option<u8>,
//     pub comment: Option<String>,
// }

pub trait ParsePoint<T> {
    fn addRaw(&mut self, bytes: &Vec<u8>);
    fn convert(&self, bytes: &Vec<u8>, start: usize, bit: usize) -> Result<T, TryFromSliceError>;
    fn isChanged(&self) -> bool;
}

///
/// 
#[derive(Debug, Clone)]
pub struct S7ParsePointBool {
    pub value: bool,
    pub status: DsStatus,
    isChanged: bool,
    pub path: String,
    pub name: String,
    pub dataType: Option<String>,
    pub vrt: Option<u8>,
    pub offset: Option<u32>,
    pub bit: Option<u8>,
    pub h: Option<u8>,
    pub a: Option<u8>,
    pub comment: Option<String>,
    pub timestamp: DateTime<Utc>,
}
impl S7ParsePointBool {
    pub fn new(
        // dataType: DSDataType,
        path: String,
        name: String,
        config: DsPointConf,
        // filter: Filter<T>,
        // convert: Function,
    ) -> S7ParsePointBool {
        S7ParsePointBool {
            value: false,
            status: DsStatus::Invalid,
            isChanged: false,
            path: path,
            name: name,
            dataType: config.dataType,
            vrt: config.vrt,
            offset: config.offset,
            bit: config.bit,
            h: config.h,
            a: config.a,
            comment: config.comment,
            timestamp: Utc::now(),
        }
    }
}
///
impl ParsePoint<bool> for S7ParsePointBool {
    ///
    fn addRaw(&mut self, bytes: &Vec<u8>) {
        let timestamp = Utc::now();
        let result = self.convert(bytes, self.offset.unwrap() as usize, self.bit.unwrap() as usize);
        match result {
            Ok(newVal) => {
                if newVal != self.value {
                    self.value = newVal;
                    self.status = DsStatus::Ok;
                    self.timestamp = timestamp;
                    self.isChanged = true;
                }        
            },
            Err(e) => {
                self.status = DsStatus::Invalid;
                warn!("[S7ParsePoint<bool>.addRaw] convertion error: {:?}", e);
            }
        }
    }
    ///
    fn convert(&self, bytes: &Vec<u8>, start: usize, bit: usize) -> Result<bool, TryFromSliceError> {
        match bytes[start..(start + 2)].try_into() {
            Ok(v) => {
                let i = i16::from_be_bytes(v);
                let b: i16 = i >> bit & 1;
                Ok(b > 0)
            },
            Err(e) => {
                debug!("[S7ParsePoint<bool>.convert] error: {}", e);
                Err(e)
            }
        }
    }
    ///
    /// returns true if value of point wath updated
    fn isChanged(&self) -> bool {
        self.isChanged
    }
}
///
/// 
#[derive(Debug, Clone)]
pub struct S7ParsePointInt {
    pub value: i16,
    pub status: DsStatus,
    isChanged: bool,
    pub path: String,
    pub name: String,
    pub dataType: Option<String>,
    pub vrt: Option<u8>,
    pub offset: Option<u32>,
    pub bit: Option<u8>,
    pub h: Option<u8>,
    pub a: Option<u8>,
    pub comment: Option<String>,
    pub timestamp: DateTime<Utc>,
}
///
impl S7ParsePointInt {
    pub fn new(
        // dataType: DSDataType,
        path: String,
        name: String,
        config: DsPointConf,
        // filter: Filter<T>,
        // convert: Function,
    ) -> S7ParsePointInt {
        S7ParsePointInt {
            value: 0,
            status: DsStatus::Invalid,
            isChanged: false,
            path: path,
            name: name,
            dataType: config.dataType,
            vrt: config.vrt,
            offset: config.offset,
            bit: config.bit,
            h: config.h,
            a: config.a,
            comment: config.comment,
            timestamp: Utc::now(),
        }
    }
}
impl ParsePoint<i16> for S7ParsePointInt {
    ///
    fn addRaw(&mut self, bytes: &Vec<u8>) {
        let timestamp = Utc::now();
        let result = self.convert(bytes, self.offset.unwrap() as usize, 0);
        match result {
            Ok(newVal) => {
                if newVal != self.value {
                    self.value = newVal;
                    self.status = DsStatus::Ok;
                    self.timestamp = timestamp;
                    self.isChanged = true;
                }        
            },
            Err(e) => {
                self.status = DsStatus::Invalid;
                warn!("[S7ParsePoint<i16>.addRaw] convertion error: {:?}", e);
            }
        }
    }
    ///
    fn convert(&self, bytes: &Vec<u8>, start: usize, _bit: usize) -> Result<i16, TryFromSliceError> {
        match bytes[start..(start + 2)].try_into() {
            Ok(v) => Ok(i16::from_be_bytes(v)),
            Err(e) => {
                debug!("[S7ParsePoint<i16>.convert] error: {}", e);
                Err(e)
            }
        }
    }
    ///
    /// returns true if value of point wath updated
    fn isChanged(&self) -> bool {
        self.isChanged
    }
}
///
/// 
#[derive(Debug, Clone)]
pub struct S7ParsePointReal {
    pub value: f32,
    pub status: DsStatus,
    isChanged: bool,
    pub path: String,
    pub name: String,
    pub dataType: Option<String>,
    pub vrt: Option<u8>,
    pub offset: Option<u32>,
    pub bit: Option<u8>,
    pub h: Option<u8>,
    pub a: Option<u8>,
    pub comment: Option<String>,
    pub timestamp: DateTime<Utc>,
}
///
impl S7ParsePointReal {
    ///
    pub fn new(
        // dataType: DSDataType,
        path: String,
        name: String,
        config: DsPointConf,
        // filter: Filter<T>,
        // convert: Function,
    ) -> S7ParsePointReal {
        S7ParsePointReal {
            value: 0.0,
            status: DsStatus::Invalid,
            isChanged: false,
            path: path,
            name: name,
            dataType: config.dataType,
            vrt: config.vrt,
            offset: config.offset,
            bit: config.bit,
            h: config.h,
            a: config.a,
            comment: config.comment,
            timestamp: Utc::now(),
        }
    }
}
impl ParsePoint<f32> for S7ParsePointReal {
    ///
    fn addRaw(&mut self, bytes: &Vec<u8>) {
        let timestamp = Utc::now();
        let result = self.convert(bytes, self.offset.unwrap() as usize, 0);
        match result {
            Ok(newVal) => {
                if newVal != self.value {
                    self.value = newVal;
                    self.status = DsStatus::Ok;
                    self.timestamp = timestamp;
                    self.isChanged = true;
                }        
            },
            Err(e) => {
                self.status = DsStatus::Invalid;
                warn!("[S7ParsePoint<f32>.addRaw] convertion error: {:?}", e);
            }
        }
    }
    ///
    fn convert(&self, bytes: &Vec<u8>, start: usize, _bit: usize) -> Result<f32, TryFromSliceError> {
        debug!("[S7ParsePoint<f32>.convert] start: {},  end: {:?}", start, start + 4);
        let raw: [u8; 4] = (bytes[start..(start + 4)]).try_into().unwrap();
        debug!("[S7ParsePoint<f32>.convert] raw: {:?}", raw);
        match bytes[start..(start + 4)].try_into() {
            // Ok(v) => Ok(f32::from_le_bytes(v)),
            Ok(v) => Ok(f32::from_be_bytes(v)),
            Err(e) => {
                debug!("[S7ParsePoint<f32>.convert] error: {}", e);
                Err(e)
            }
        }
    }
    ///
    /// returns true if value of point wath updated
    fn isChanged(&self) -> bool {
        self.isChanged
    }        
}
///
#[derive(Debug, Clone)]
pub enum ParsePointType {
    Bool(S7ParsePointBool),
    Int(S7ParsePointInt),
    Real(S7ParsePointReal),
}
