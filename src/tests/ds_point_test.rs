#[cfg(test)]

use chrono::{Utc};
use concurrent_queue::ConcurrentQueue;

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
        0,
        0,
        Utc::now(),
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
        0,
        0,
        Utc::now(),
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
        0,
        0,
        Utc::now(),
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
    let queue = ConcurrentQueue::unbounded();
    let buf = vec![
        DsPointValue::DsPointReal(123.567_f32),
        DsPointValue::DsPointInt(134_i16),
        DsPointValue::DsPointBool(true),
    ];
    for value in &buf {
        let point = match value {
            DsPointValue::DsPointBool(value) => {
                DsPoint::newBool(
                    "test.point.bool",
                    *value,
                    DsStatus::Ok,
                    0,
                    0,
                    Utc::now(),
                )
            }
            DsPointValue::DsPointInt(value) => {
                DsPoint::newInt(
                    "test.point.int",
                    *value,
                    DsStatus::Ok,
                    0,
                    0,
                    Utc::now(),
                )
            }
            DsPointValue::DsPointReal(value) => {
                DsPoint::newReal(
                    "test.point.real",
                    *value,
                    DsStatus::Ok,
                    0,
                    0,
                    Utc::now(),
                )
            }
        };
        queue.push(point).unwrap();
    }
    for value in buf {
        let point = queue.pop().unwrap();
        assert_eq!(point.value, value);
    }
    // let value = 123.567_f32;
    // let point = DsPoint::newReal(
    //     "test.point.real",
    //     value,
    //     DsStatus::Ok,
    //     0,
    //     0,
    //     Utc::now(),
    // );
    // queue.push(point).unwrap();
    // let value = 134_i16;
    // let point = DsPoint::newInt(
    //     "test.point.int",
    //     value,
    //     DsStatus::Ok,
    //     0,
    //     0,
    //     Utc::now(),
    // );
    // queue.push(point).unwrap();
    // let value = true;
    // let point = DsPoint::newBool(
    //     "test.point.bool",
    //     value,
    //     DsStatus::Ok,
    //     0,
    //     0,
    //     Utc::now(),
    // );
    // queue.push(point).unwrap();

    // assert_eq!(point.value, value);
}
