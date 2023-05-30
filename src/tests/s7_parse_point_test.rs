#![allow(non_upper_case_globals)]
#![cfg(test)]

use std::env;
use log::{
    info, 
    debug, 
    warn,
    error, 
};
use std::{time::Instant, collections::{HashMap, BTreeMap}};
use chrono::{Utc};

use crate::{
    ds::{
        ds_point::{DsPoint, DsPointValue}, 
        ds_status::DsStatus, 
        ds_config::DsPointConf,
    }, 
    s7::s7_parse_point::{ParsePointType, S7ParsePointBool, S7ParsePointInt, S7ParsePointReal}, 
    tests::setup::setup,
};

const logPref: &str = "[s7_parse_point_test]";

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
        DsPointValue::DsPointBool(value) => value,
        DsPointValue::DsPointInt(_) => todo!(),
        DsPointValue::DsPointReal(_) => todo!(),
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
        DsPointValue::DsPointBool(_) => todo!(),
        DsPointValue::DsPointInt(value) => value,
        DsPointValue::DsPointReal(_) => todo!(),
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
        DsPointValue::DsPointBool(_) => todo!(),
        DsPointValue::DsPointInt(_) => todo!(),
        DsPointValue::DsPointReal(value) => value,
    };    
    assert_eq!(pValue, value);
}
#[test]
fn add_to_queue() {
    setup();
    let count = 100_000usize;
    info!("{} count: {:?}", logPref, count);
    let mut confPoints: HashMap<String, DsPointConf> = HashMap::new();
    confPoints.insert(
        "tset1".to_string(),
        DsPointConf { vrt: None, dataType: Some("Real".to_string()), offset: Some(0), bit: None, h: None, a: None, comment: None },
    );
    confPoints.insert(
        "tset1".to_string(),
        DsPointConf { vrt: None, dataType: Some("Real".to_string()), offset: Some(4), bit: None, h: None, a: None, comment: None },
    );
    let mut dbPoints: BTreeMap<String, ParsePointType> = BTreeMap::new();
    let t = Instant::now();
    for (pointKey, point) in confPoints {
        // debug!("\t\t\tdb {:?}: {:?}", pointKey, &point);
        let dataType = &point.dataType.clone().unwrap();
        if *dataType == "Bool".to_string() {
            dbPoints.insert(
                pointKey.clone(),
                ParsePointType::Bool(
                    S7ParsePointBool::new(
                        pointKey.clone(),
                        pointKey.clone(),
                        point,
                    ),
                ),
            );
        } else if *dataType == "Int".to_string() {
            dbPoints.insert(
                pointKey.clone(),
                ParsePointType::Int(
                    S7ParsePointInt::new(
                        pointKey.clone(), 
                        pointKey.clone(), 
                        point,
                    ),
                ),
            );
        } else if *dataType == "Real".to_string() {
            dbPoints.insert(
                pointKey.clone(),
                ParsePointType::Real(
                    S7ParsePointReal::new(
                        pointKey.clone(), 
                        pointKey.clone(), 
                        point,
                    ),
                ),
            );
        } else {
            error!("{} point {:?}: uncnoun data type {:?}", logPref, pointKey, dataType);
        }
    }
    info!("{} elapsed: {:?}", logPref, dbPoints);
    // assert_eq!(buf.len(), queue.len(), "length of source values array must be equal to the length of target queue");
    // for value in buf {
    //     let point = queue.pop().unwrap();
    //     assert_eq!(point.value, value);
    // }
    let elapsed = Instant::now() - t;
    info!("{} elapsed: {:?}", logPref, elapsed);
}
