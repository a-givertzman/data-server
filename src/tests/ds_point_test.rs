#![allow(non_upper_case_globals)]
#![cfg(test)]

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

use crate::{
    ds::{
        ds_point::{DsPoint, DsPointType}, 
        ds_status::DsStatus,
    }, 
    tests::setup::setup
};

#[test]
fn construct_bool() {
    setup();
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
        DsPointType::Bool(value) => value,
        DsPointType::Int(_) => todo!(),
        DsPointType::Real(_) => todo!(),
    };
    assert_eq!(pValue, value);
}
#[test]
fn construct_int() {
    setup();
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
        DsPointType::Bool(_) => todo!(),
        DsPointType::Int(value) => value,
        DsPointType::Real(_) => todo!(),
    };    
    assert_eq!(pValue, value);
}
#[test]
fn construct_real() {
    setup();
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
        DsPointType::Bool(_) => todo!(),
        DsPointType::Int(_) => todo!(),
        DsPointType::Real(value) => value,
    };    
    assert_eq!(pValue, value);
}
#[test]
fn add_to_queue() {
    setup();
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
                DsPointType::Bool(value)
            }
            1 => {
                let mut rng = rand::thread_rng();
                let value = rng.gen_range(-32_768i16..32_767i16);
                // debug!("[ds_point_test.add_to_queue] int  value: {:?}", &value);
                DsPointType::Int(value)
            }
            _ => {
                let mut rng = rand::thread_rng();
                let value = rng.gen_range((-1.0e5)..(1.0e5));
                // debug!("[ds_point_test.add_to_queue] real  value: {:?}", &value);
                DsPointType::Real(value)
            }
        };
        // debug!("[ds_point_test.add_to_queue] point value: {:?}", &value);
        buf.push(value);
    }
    // debug!("[ds_point_test.add_to_queue] buffer ready: {:?}", &buf);
    let t = Instant::now();
    for value in &buf {
        let point = match value {
            DsPointType::Bool(value) => {
                DsPoint::newBool(
                    "test.point.bool",
                    *value,
                    DsStatus::Ok,
                    Utc::now(),
                    None,
                    None,
                )
            }
            DsPointType::Int(value) => {
                DsPoint::newInt(
                    "test.point.int",
                    *value,
                    DsStatus::Ok,
                    Utc::now(),
                    None,
                    None,
                )
            }
            DsPointType::Real(value) => {
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
