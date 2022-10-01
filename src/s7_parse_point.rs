#![allow(non_snake_case)]

pub mod s7_parse_point {
    use std::{array::TryFromSliceError};
    use log::{debug, error, info, warn};

    use crate::{ds_config::ds_config::{DsPointConf}};

    #[derive(Debug)]
    pub struct S7ParsePoint<T> {
        v: T,
        pub vrt: Option<u8>,
        pub dataType: Option<String>,
        pub offset: Option<u32>,
        pub bit: Option<u8>,
        pub h: Option<u8>,
        pub a: Option<u8>,
        pub comment: Option<String>,
    }
    impl<T> S7ParsePoint<T> {
        pub fn new(
            // dataType: DSDataType,
            path: String,
            name: String,
            config: DsPointConf,
            // filter: Filter<T>,
            // convert: Function,
        ) -> S7ParsePoint<T> 
            where
                T: From<i16> + From<f32> + From<bool>,
        {
            S7ParsePoint {
                v: T::from(0i16),
                vrt: config.vrt,
                dataType: config.dataType,
                offset: config.offset,
                bit: config.bit,
                h: config.h,
                a: config.a,
                comment: config.comment,
            }
        }
    }
    ///
    impl S7ParsePoint<bool> {
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
    impl S7ParsePoint<i16> {
        fn convert(&self, bytes: &Vec<u8>, start: usize) -> Result<i16, TryFromSliceError> {
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
    impl S7ParsePoint<f32> {
        fn convert(&self, bytes: &Vec<u8>, start: usize) -> Result<f32, TryFromSliceError> {
            match bytes[start..(start + 4)].try_into() {
                Ok(v) => Ok(f32::from_be_bytes(v)),
                Err(e) => {
                    error!("ERROR in S7ParsePoint<f32>: {}", e);
                    Err(e)
                }
            }
        }        
    }
}
