#![allow(non_snake_case)]

pub mod s7_client {
    // #[macro_use]
    // extern crate snap7_sys;

    use snap7_sys::*;
    use std::ffi::CString;
    use std::os::raw::{
        c_int,
        c_char,
        c_void,
    };
    
    
    #[derive(Debug)]
    pub struct S7Client {
        ip: CString,
        handle: S7Object,
        req_len: usize,
        neg_len: usize,
    }
    
    impl S7Client {
        pub fn new(ip: String) -> Self {
            Self {
                ip: CString::new(ip).unwrap(),
                handle: unsafe { Cli_Create() },
                req_len: 0,
                neg_len: 0,
            }
        }
        pub fn connect(&mut self) {
            let mut req: c_int = 0;
            let mut neg: c_int = 0;
            unsafe {
                #[warn(temporary_cstring_as_ptr)]
                Cli_ConnectTo(self.handle, self.ip.as_ptr(), 0, 1);
                Cli_GetPduLength(self.handle, &mut req, &mut neg);
                self.req_len = req as usize;
                self.neg_len = neg as usize;
            }
        }
        pub fn read(&self, dbNum: u32, start: u32, size: u32) -> Result<Vec<u8>, String> {
            let mut buf = Vec::<u8>::new();
            buf.resize(size as usize, 0);
            let res;
            unsafe {
                res = Cli_DBRead(
                    self.handle,
                    dbNum as c_int,
                    start as c_int,
                    size as c_int,
                    buf.as_mut_ptr() as *mut c_void
                ) as i32;
    
            }
            if res == 0 {
                Ok(buf)
            } else {
                Err(String::from(error_text(res)))
            }
        }
        pub fn close(&mut self) {
            unsafe {
                Cli_Disconnect(self.handle);
            }
        }
    }
    
    impl Drop for S7Client {
        fn drop(&mut self) {
            self.close();
            unsafe {
                Cli_Destroy(&mut self.handle);
            }
        }
    }
    
    pub fn error_text(code: i32) -> String {
        let mut err = Vec::<u8>::new();
        err.resize(1024, 0);
        unsafe {
            Cli_ErrorText(
                code as c_int, 
                err.as_mut_ptr() as *mut c_char, 
                err.len() as c_int
            );
        }
        if let Some(i) = err.iter().position(|&r| r == 0) {
            err.truncate(i);
        }
        let err = unsafe {
            std::str::from_utf8_unchecked(&err)
        };
        err.to_owned()
    }
    
    
    // struct CtlRecord {
    //     plc_counter: u64,
    //     ctl_counter: u64,
    // }
}