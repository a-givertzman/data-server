#![allow(non_snake_case)]

pub mod ds_point {
    use chrono::{DateTime, Utc};

    use crate::ds_config::ds_config::DsPointConf;

    #[derive(Debug)]
    pub struct DsPoint<T> {
        pub name: String,
        pub dataType: Option<String>,
        pub value: T,
        history: u8,
        alarm: u8,
        pub timestamp: DateTime<Utc>,
    }



    impl DsPoint<bool> {
        pub fn new(
            name: String,
            config: DsPointConf,
        ) -> DsPoint<bool> {
            DsPoint {
                name: name,
                dataType: config.dataType,
                value: todo!(),
                history: todo!(),
                alarm: todo!(),
                timestamp: todo!(),
            }
    
        }
    }
}
