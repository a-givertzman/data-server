#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use chrono::{DateTime, Utc};

use crate::ds::{
    ds_status::DsStatus,
};


#[derive(Debug, Clone, PartialEq)]
pub enum DsPointType {
    Bool(bool),
    Int(i16),
    Real(f32),
}
impl DsPointType {
    ///
    /// 
    pub fn valueBool(&self) -> bool{
        match *self {
            DsPointType::Bool(value) => value,
            DsPointType::Int(_) => todo!(),
            DsPointType::Real(_) => todo!(),
        }      
    }
    ///
    /// 
    pub fn valueInt(&self) -> i16{
        match *self {
            DsPointType::Int(value) => value,
            DsPointType::Bool(_) => todo!(),
            DsPointType::Real(_) => todo!(),
        }      
    }
    ///
    /// 
    pub fn valueReal(&self) -> f32{
        match *self {
            DsPointType::Real(value) => value,
            DsPointType::Int(_) => todo!(),
            DsPointType::Bool(_) => todo!(),
        }      
    }
}

#[derive(Debug)]
pub struct DsPoint {
    pub name: String,
    pub dataType: String,
    pub value: DsPointType,
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
            value: DsPointType::Bool(value),
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
            value: DsPointType::Int(value),
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
            value: DsPointType::Real(value),
            status: status,
            timestamp: timestamp,
            history: history,
            alarm: alarm,
        }
    }
    ///
    /// 
    pub fn valueBool(&self) -> bool{
        match self.value {
            DsPointType::Bool(value) => value,
            DsPointType::Int(_) => todo!(),
            DsPointType::Real(_) => todo!(),
        }      
    }
    ///
    /// 
    pub fn valueInt(&self) -> i16{
        match self.value {
            DsPointType::Int(value) => value,
            DsPointType::Bool(_) => todo!(),
            DsPointType::Real(_) => todo!(),
        }      
    }
    ///
    /// 
    pub fn valueReal(&self) -> f32{
        match self.value {
            DsPointType::Real(value) => value,
            DsPointType::Int(_) => todo!(),
            DsPointType::Bool(_) => todo!(),
        }      
    }
}
