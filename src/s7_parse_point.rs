#![allow(non_snake_case)]

    #[derive(Debug, Clone)]
    pub struct ParsePoint<T> {
    v: T,
    pub vrt: Option<u8>,
    pub dataType: Option<String>,
    pub offset: Option<u32>,
    pub bit: Option<u8>,
    pub h: Option<u8>,
    pub a: Option<u8>,
    pub comment: Option<String>,
}
pub mod s7_parse_point {
    use std::array::TryFromSliceError;

    use log::{debug, error, info, warn};

    use crate::{ds_config::ds_config::{DsPointConf}};

    pub trait ParsePoint<T> {
        fn addRaw(&mut self, bytes: &Vec<u8>);
        fn convert(&self, bytes: &Vec<u8>, start: usize, bit: usize) -> Result<T, TryFromSliceError>;
    }
    
    
    #[derive(Debug, Clone)]
    pub struct S7ParsePointBool {
        value: bool,
        pub path: String,
        pub name: String,
        pub dataType: Option<String>,
        pub vrt: Option<u8>,
        pub offset: Option<u32>,
        pub bit: Option<u8>,
        pub h: Option<u8>,
        pub a: Option<u8>,
        pub comment: Option<String>,
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
                path: path,
                name: name,
                dataType: config.dataType,
                vrt: config.vrt,
                offset: config.offset,
                bit: config.bit,
                h: config.h,
                a: config.a,
                comment: config.comment,
            }
        }
    }
    ///
    impl ParsePoint<bool> for S7ParsePointBool {
        ///
        fn addRaw(&mut self, bytes: &Vec<u8>) {
            let result = self.convert(bytes, self.offset.unwrap() as usize, self.bit.unwrap() as usize);
            match result {
                Ok(newVal) => {
                    if newVal != self.value {
                        self.value = newVal;
                    }        
                },
                Err(e) => {
                    warn!("[S7ParsePoint<i16>.addRaw] convertion error: {:#?}", e);
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
                    error!("ERROR in S7ParsePoint<i16>: {}", e);
                    Err(e)
                }
            }
        }
    }

    ///
    /// add new value to bee parsed
    /// if new value is not equal to current, then current will be updated with new one
    // impl<T> S7ParsePoint<T> {

    // }
    ///
    #[derive(Debug, Clone)]
    pub struct S7ParsePointInt {
        value: i16,
        pub path: String,
        pub name: String,
        pub dataType: Option<String>,
        pub vrt: Option<u8>,
        pub offset: Option<u32>,
        pub bit: Option<u8>,
        pub h: Option<u8>,
        pub a: Option<u8>,
        pub comment: Option<String>,
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
                path: path,
                name: name,
                dataType: config.dataType,
                vrt: config.vrt,
                offset: config.offset,
                bit: config.bit,
                h: config.h,
                a: config.a,
                comment: config.comment,
            }
        }
    }
    impl ParsePoint<i16> for S7ParsePointInt {
        fn addRaw(&mut self, bytes: &Vec<u8>) {
            let result = self.convert(bytes, self.offset.unwrap() as usize, 0);
            match result {
                Ok(newVal) => {
                    if newVal != self.value {
                        self.value = newVal;
                    }        
                },
                Err(e) => {
                    warn!("[S7ParsePoint<i16>.addRaw] convertion error: {:#?}", e);
                }
            }
        }
        fn convert(&self, bytes: &Vec<u8>, start: usize, bit: usize) -> Result<i16, TryFromSliceError> {
            match bytes[start..(start + 2)].try_into() {
                Ok(v) => Ok(i16::from_be_bytes(v)),
                Err(e) => {
                    error!("ERROR in S7ParsePoint<i16>: {}", e);
                    Err(e)
                }
            }
        }
    }
    ///
    #[derive(Debug, Clone)]
    pub struct S7ParsePointReal {
        value: f32,
        pub path: String,
        pub name: String,
        pub dataType: Option<String>,
        pub vrt: Option<u8>,
        pub offset: Option<u32>,
        pub bit: Option<u8>,
        pub h: Option<u8>,
        pub a: Option<u8>,
        pub comment: Option<String>,
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
                path: path,
                name: name,
                dataType: config.dataType,
                vrt: config.vrt,
                offset: config.offset,
                bit: config.bit,
                h: config.h,
                a: config.a,
                comment: config.comment,
            }
        }
    }
    impl ParsePoint<f32> for S7ParsePointReal {
        ///
        fn addRaw(&mut self, bytes: &Vec<u8>) {
            let result = self.convert(bytes, self.offset.unwrap() as usize, 0);
            match result {
                Ok(newVal) => {
                    if newVal != self.value {
                        self.value = newVal;
                    }        
                },
                Err(e) => {
                    warn!("[S7ParsePoint<i16>.addRaw] convertion error: {:#?}", e);
                }
            }
        }
        ///
        fn convert(&self, bytes: &Vec<u8>, start: usize, bit: usize) -> Result<f32, TryFromSliceError> {
            match bytes[start..(start + 4)].try_into() {
                Ok(v) => Ok(f32::from_be_bytes(v)),
                Err(e) => {
                    error!("ERROR in S7ParsePoint<f32>: {}", e);
                    Err(e)
                }
            }
        }        
    }
    ///
    #[derive(Debug, Clone)]
    pub enum ParsePointType {
        Bool(S7ParsePointBool),
        Int(S7ParsePointInt),
        Real(S7ParsePointReal),
    }    
}
