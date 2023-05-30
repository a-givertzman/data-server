#![allow(non_upper_case_globals)]
#[cfg(test)]

use std::env;
use log::{
    info, 
    debug, 
    warn,
    error, 
};
use std::time::Instant;
use chrono::{Utc};
use concurrent_queue::ConcurrentQueue;
use rand::Rng;

use crate::ds::{
    ds_point::{DsPoint, DsPointValue}, 
    ds_status::DsStatus,
};

#[test]
fn construct_bool() {
    let value = true;
    let point = DsPoint::newBool(
        "test.point",
        value,
        DsStatus::Ok,
        Utc::now(),
        None,
        None,
    );
    let pValue = match point.value {
        DsPointValue::DsPointBool(value) => value,
        DsPointValue::DsPointInt(_) => todo!(),
        DsPointValue::DsPointReal(_) => todo!(),
    };
    assert_eq!(pValue, value);
}
#[test]
fn construct_int() {
    let value = 134_i16;
    let point = DsPoint::newInt(
        "test.point",
        value,
        DsStatus::Ok,
        Utc::now(),
        None,
        None,
    );
    let pValue = match point.value {
        DsPointValue::DsPointBool(_) => todo!(),
        DsPointValue::DsPointInt(value) => value,
        DsPointValue::DsPointReal(_) => todo!(),
    };    
    assert_eq!(pValue, value);
}
#[test]
fn construct_real() {
    let value = 123.567_f32;
    let point = DsPoint::newReal(
        "test.point",
        value,
        DsStatus::Ok,
        Utc::now(),
        None,
        None,
    );
    let pValue = match point.value {
        DsPointValue::DsPointBool(_) => todo!(),
        DsPointValue::DsPointInt(_) => todo!(),
        DsPointValue::DsPointReal(value) => value,
    };    
    assert_eq!(pValue, value);
}
#[test]
fn add_to_queue() {
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();    
    let count = 100_000usize;
    let queue = ConcurrentQueue::unbounded();
    let mut buf = vec![];
    let mut rng = rand::thread_rng();
    info!("[ds_point_test.add_to_queue] count: {:?}", count);
    for _ in 0..count {
        let value = match rng.gen_range(0..=2) {
            0 => {
                let mut rng = rand::thread_rng();
                let value = rng.gen_bool(0.5);
                // debug!("[ds_point_test.add_to_queue] bool value: {:?}", &value);
                DsPointValue::DsPointBool(value)
            }
            1 => {
                let mut rng = rand::thread_rng();
                let value = rng.gen_range(-32_768i16..32_767i16);
                // debug!("[ds_point_test.add_to_queue] int  value: {:?}", &value);
                DsPointValue::DsPointInt(value)
            }
            _ => {
                let mut rng = rand::thread_rng();
                let value = rng.gen_range((-1.0e5)..(1.0e5));
                // debug!("[ds_point_test.add_to_queue] real  value: {:?}", &value);
                DsPointValue::DsPointReal(value)
            }
        };
        // debug!("[ds_point_test.add_to_queue] point value: {:?}", &value);
        buf.push(value);
    }
    // debug!("[ds_point_test.add_to_queue] buffer ready: {:?}", &buf);
    let t = Instant::now();
    for value in &buf {
        let point = match value {
            DsPointValue::DsPointBool(value) => {
                DsPoint::newBool(
                    "test.point.bool",
                    *value,
                    DsStatus::Ok,
                    Utc::now(),
                    None,
                    None,
                )
            }
            DsPointValue::DsPointInt(value) => {
                DsPoint::newInt(
                    "test.point.int",
                    *value,
                    DsStatus::Ok,
                    Utc::now(),
                    None,
                    None,
                )
            }
            DsPointValue::DsPointReal(value) => {
                DsPoint::newReal(
                    "test.point.real",
                    *value,
                    DsStatus::Ok,
                    Utc::now(),
                    None,
                    None,
                )
            }
        };
        // debug!("[ds_point_test.add_to_queue] point: {:?}", &point);
        queue.push(point).unwrap();
    }
    assert_eq!(buf.len(), queue.len(), "length of source values array must be equal to the length of target queue");
    for value in buf {
        let point = queue.pop().unwrap();
        assert_eq!(point.value, value);
    }
    let elapsed = Instant::now() - t;
    info!("[ds_point_test.add_to_queue] elapsed: {:?}", elapsed);
}
