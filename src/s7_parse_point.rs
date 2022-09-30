#![allow(non_snake_case)]

pub mod s7_parse_point {
    #[derive(Debug)]
    type ConvertFunction = fn(i32, &str);

    pub struct S7ParsePoint {
        convert: ConvertFunction,
        pub name: Option<String>,
        pub description: Option<String>,
        pub number: Option<u32>,
        pub offset: Option<u32>,
        pub size: Option<u32>,
        pub delay: Option<u32>,
        pub points: Option<HashMap<String, DsPoint>>,
    }
    impl S7ParsePoint {
        pub fn new(
            dataType: DSDataType,
            path: str,
            name: str,
            config: DsPointConf,
            filter: Filter,
            // convert: Function,
        ) -> S7ParsePoint {
            let convert = HashMap::from([
                ("toBool", &toBool), 
                ("toInt", &toInt), 
                ("toReal", &toReal), 
            ]);
            let conv: ConvertFunction = |_, _, _| {};;
            if (config.dataType == "Bool") {
                conv = toBool;
            }
            if (config.dataType == "Int") {
                conv = toInt;
            }
            if (config.dataType == "Real") {
                conv = toReal;
            }
        }
        ///
        fn toBool(bytes: &Vec<u8>, start: usize, bit: usize) -> bool {
            let i = toInt(&bytes, start);
            let b = i >> bit & 1;
            b > 0
            // f32::from_be_bytes(bytes[start..end].try_into().unwrap())
        }
        ///
        fn toInt(bytes: &Vec<u8>, start: usize) -> i16 {
            let end = start + 2;
            i16::from_be_bytes(bytes[start..end].try_into().expect("Conversion error"))
        }
        ///
        fn toReal(bytes: &Vec<u8>, start: usize) -> f32 {
            let end = start + 4;
            f32::from_be_bytes(bytes[start..end].try_into().unwrap())
        }        
    }
}