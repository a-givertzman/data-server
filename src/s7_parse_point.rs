#![allow(non_snake_case)]

pub mod s7_parse_point {
    use std::{io::Error, collections::HashMap};

    use crate::{ds_config::ds_config::{DsPointConf}};

    #[feature(specialization)]
       
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
            S7ParsePoint{
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
        ///
        fn toBool<K>(&self, bytes: &Vec<u8>, start: usize, bit: usize) -> Result<K, Error> // bool
        where
            K: From<bool>,
        {
            let i: i16 = self.toInt(&bytes, start, 0).unwrap();
            let b: i16 = i >> bit & 1;
            Ok(K::from(b > 0))
            // f32::from_be_bytes(bytes[start..end].try_into().unwrap())
        }
        ///
        fn toInt<K>(&self, bytes: &Vec<u8>, start: usize, bit: usize) -> Result<K, Error> // i16
        where
            K: From<i16>,
        {
            let end = start + 2;
            Ok(K::from(i16::from_be_bytes(bytes[start..end].try_into().expect("Conversion error"))))
        }
        ///
        fn toReal<K>(&self, bytes: &Vec<u8>, start: usize, bit: usize) -> Result<K, Error> // f32 
        where
            K: From<f32>,
        {
            let end = start + 4;
            Ok(K::from(f32::from_be_bytes(bytes[start..end].try_into().unwrap())))
        }        
    }
}
