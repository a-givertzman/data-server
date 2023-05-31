#![allow(non_upper_case_globals)]
#![cfg(test)]

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
        ds_point::{DsPoint, DsPointType}, 
        ds_status::DsStatus, 
        ds_config::DsPointConf,
    }, 
    s7::s7_parse_point::{ParsePointType, S7ParsePointBool, S7ParsePointInt, S7ParsePointReal, ParsePoint}, 
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
    let mut confPoints: HashMap<String, DsPointConf> = HashMap::new();
    confPoints.insert(
        "point1Bool".to_string(),
        DsPointConf { vrt: None, dataType: Some("Bool".to_string()), offset: Some(0), bit: Some(0), h: None, a: None, comment: None },
    );
    confPoints.insert(
        "point2Bool".to_string(),
        DsPointConf { vrt: None, dataType: Some("Bool".to_string()), offset: Some(0), bit: Some(1), h: None, a: None, comment: None },
    );
    confPoints.insert(
        "pointInt".to_string(),
        DsPointConf { vrt: None, dataType: Some("Int".to_string()), offset: Some(0), bit: None, h: None, a: None, comment: None },
    );
    confPoints.insert(
        "pointReal".to_string(),
        DsPointConf { vrt: None, dataType: Some("Real".to_string()), offset: Some(0), bit: None, h: None, a: None, comment: None },
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
    // debug!("{} dbPoints: {:?}", logPref, dbPoints);
    let testData = vec![
        ("point1Bool", (vec![0b0u8, 0b001u8], DsPointType::Bool(true))), 
        ("point1Bool", (vec![0b0u8, 0b000u8], DsPointType::Bool(false))),
        ("point1Bool", (vec![0b1u8, 0b111u8], DsPointType::Bool(true))), 
        ("point1Bool", (vec![0b1u8, 0b110u8], DsPointType::Bool(false))),
        ("point2Bool", (vec![0b0u8, 0b010u8], DsPointType::Bool(true))), 
        ("point2Bool", (vec![0b0u8, 0b000u8], DsPointType::Bool(false))),
        ("point2Bool", (vec![0b1u8, 0b111u8], DsPointType::Bool(true))), 
        ("point2Bool", (vec![0b1u8, 0b101u8], DsPointType::Bool(false))),
        ("pointInt", (vec![0u8, 0u8], DsPointType::Int(0))),
        ("pointInt", (vec![0u8, 1u8], DsPointType::Int(1))),
        ("pointInt", (vec![0u8, 2u8], DsPointType::Int(2))),
        ("pointInt", (vec![0u8, 3u8], DsPointType::Int(3))),
        ("pointInt", (vec![0x04u8, 0xD2u8], DsPointType::Int(1234))),
        ("pointInt", (vec![0x80u8, 0x00u8], DsPointType::Int(-32768))),
        ("pointInt", (vec![0x7Fu8, 0xFFu8], DsPointType::Int(32767))),
        ("pointReal", (vec![68u8, 113u8, 192u8, 179u8], DsPointType::Real(967.0109))),
        ("pointReal", (vec![0x44u8, 0x71u8, 0xC0u8, 0xB3u8], DsPointType::Real(967.0109))),
        ("pointReal", (vec![0x46u8, 0x40u8, 0xE4u8, 0x7Eu8], DsPointType::Real(12345.12345))),
        ("pointReal", (vec![0x3Du8, 0xFCu8, 0xD3u8, 0x5Bu8], DsPointType::Real(0.12345))),
        ("pointReal", (vec![0x46u8, 0x40u8, 0xE4u8, 0x00u8], DsPointType::Real(12345.0))),
    ];

    for (key, (bytes, result)) in testData {
        debug!("{} test on key: {:?},  bytes: {:?},  result: {:?}", logPref, key, bytes, result);
        // let key = *pointKey.clone();
        // let key = key.as_str();
        let parsePoint = dbPoints[key].clone();
        match parsePoint {
            ParsePointType::Bool(mut point) => {
                let resultValue = result.valueBool();
                point.addRaw(&bytes);
                debug!("{} raw: {:?},  parsed value: {:?}", logPref, result, point.value);
                assert_eq!(point.value, resultValue);
            },
            ParsePointType::Int(mut point) => {
                let resultValue = result.valueInt();
                point.addRaw(&bytes);
                debug!("{} raw: {:?},  parsed value: {:?}", logPref, result, point.value);
                assert_eq!(point.value, resultValue);
            },
            ParsePointType::Real(mut point) => {
                let resultValue = result.valueReal();
                point.addRaw(&bytes);
                debug!("{} raw: {:?},  parsed value: {:?}", logPref, result, point.value);
                assert_eq!(point.value, resultValue);
            },
        }
    } 

    let elapsed = Instant::now() - t;
    info!("{} elapsed: {:?}", logPref, elapsed);
}
