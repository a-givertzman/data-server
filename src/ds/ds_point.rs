#![allow(non_snake_case)]

use chrono::{DateTime, Utc};

use crate::ds::{
    ds_status::DsStatus,
};


#[derive(Debug, PartialEq)]
pub enum DsPointValue {
    DsPointBool(bool),
    DsPointInt(i16),
    DsPointReal(f32),
}

#[derive(Debug)]
pub struct DsPoint {
    pub name: String,
    pub dataType: String,
    pub value: DsPointValue,
    pub status: DsStatus,
    pub timestamp: DateTime<Utc>,
    pub history: Option<u8>,
    pub alarm: Option<u8>,
}


impl DsPoint {
    ///
    /// creates new instance containing value of type bool
    pub fn newBool(
        name: &str,
        value: bool,
        status: DsStatus,
        timestamp: DateTime<Utc>,
        history: Option<u8>,
        alarm: Option<u8>,
    ) -> DsPoint {
        DsPoint {
            name: name.to_string(),
            dataType: "Bool".to_string(),
            value: DsPointValue::DsPointBool(value),
            status: status,
            timestamp: timestamp,
            history: history,
            alarm: alarm,
        }
    }
    ///
    /// creates new instance containing value of type i16
    pub fn newInt(
        name: &str,
        value: i16,
        status: DsStatus,
        timestamp: DateTime<Utc>,
        history: Option<u8>,
        alarm: Option<u8>,
    ) -> DsPoint {
        DsPoint {
            name: name.to_string(),
            dataType: "Int".to_string(),
            value: DsPointValue::DsPointInt(value),
            status: status,
            timestamp: timestamp,
            history: history,
            alarm: alarm,
        }
    }
    ///
    /// creates new instance containing value of type f32
    pub fn newReal(
        name: &str,
        value: f32,
        status: DsStatus,
        timestamp: DateTime<Utc>,
        history: Option<u8>,
        alarm: Option<u8>,
    ) -> DsPoint {
        DsPoint {
            name: name.to_string(),
            dataType: "Real".to_string(),
            value: DsPointValue::DsPointReal(value),
            status: status,
            timestamp: timestamp,
            history: history,
            alarm: alarm,
        }
    }
}
