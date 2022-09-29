#![allow(non_snake_case)]

pub mod ds_point {
    use crate::ds_config::ds_config::DsPointConf;

    #[derive(Debug)]
    pub struct DsPoint {
        pub name: String,
        pub path: String,
        pub dataType: Option<String>,
        pub offset: Option<u32>,
        pub comment: Option<String>,
        pub vrt: Option<u8>,
    }
    impl DsPoint {
        pub fn new(
            name: String,
            path: String,
            config: DsPointConf,
        ) -> DsPoint {
            DsPoint {
                name: name,
                path: path,
                dataType: config.dataType,
                offset: config.offset,
                vrt: config.vrt,
                comment: config.comment,
            }
    
        }
    }
}
