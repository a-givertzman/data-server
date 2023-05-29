#![allow(non_snake_case)]

use chrono::{DateTime, Utc};

use crate::ds::{
    ds_status::DsStatus,
};

#[derive(Debug)]
pub struct DsPoint<T> {
    pub name: String,
    pub dataType: String,
    pub value: Box<T>,
    pub status: DsStatus,
    history: u8,
    alarm: u8,
    pub timestamp: DateTime<Utc>,
}



impl DsPoint<bool> {
    pub fn new(
        name: &str,
        value: bool,
        status: DsStatus,
        history: u8,
        alarm: u8,
        timestamp: DateTime<Utc>,
    ) -> DsPoint<bool> {
        DsPoint {
            name: name.to_string(),
            dataType: "Bool".to_string(),
            value: Box::new(value),
            status: status,
            history: history,
            alarm: alarm,
            timestamp: timestamp,
        }
    }
}


impl DsPoint<i16> {
    pub fn new(
        name: &str,
        value: i16,
        status: DsStatus,
        history: u8,
        alarm: u8,
        timestamp: DateTime<Utc>,
    ) -> DsPoint<i16> {
        DsPoint {
            name: name.to_string(),
            dataType: "Bool".to_string(),
            value: Box::new(value),
            status: status,
            history: history,
            alarm: alarm,
            timestamp: timestamp,
        }
    }
}


impl DsPoint<f32> {
    pub fn new(
        name: &str,
        value: f32,
        status: DsStatus,
        history: u8,
        alarm: u8,
        timestamp: DateTime<Utc>,
    ) -> DsPoint<f32> {
        DsPoint {
            name: name.to_string(),
            dataType: "Real".to_string(),
            value: Box::new(value),
            status: status,
            history: history,
            alarm: alarm,
            timestamp: timestamp,
        }
    }
}