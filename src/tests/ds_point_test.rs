#[cfg(test)]
use chrono::{Utc};

use crate::ds::{ds_point::DsPoint, ds_status::DsStatus};

#[test]
fn construct_bool() {
    let value = true;
    let point = DsPoint::<bool>::new(
        "test.point",
        value,
        DsStatus::Ok,
        0,
        0,
        Utc::now(),
    );
    assert_eq!(*point.value, value);
}
#[test]
fn construct_int() {
    let value = 134_i16;
    let point = DsPoint::<i16>::new(
        "test.point",
        value,
        DsStatus::Ok,
        0,
        0,
        Utc::now(),
    );
    assert_eq!(*point.value, value);
}
#[test]
fn construct_real() {
    let value = 123.567_f32;
    let point = DsPoint::<f32>::new(
        "test.point",
        value,
        DsStatus::Ok,
        0,
        0,
        Utc::now(),
    );
    assert_eq!(*point.value, value);
}
#[test]
fn add_to_vec() {
    let mut buf = vec![];
    let value = 123.567_f32;
    let point = DsPoint::<f32>::new(
        "test.point",
        value,
        DsStatus::Ok,
        0,
        0,
        Utc::now(),
    );
    buf.push(point);
    let value = 134_i16;
    let point = DsPoint::<i16>::new(
        "test.point",
        value,
        DsStatus::Ok,
        0,
        0,
        Utc::now(),
    );
    buf.push(point);

    // assert_eq!(point.value, value);
}
