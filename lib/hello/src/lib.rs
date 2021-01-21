extern crate iota;
extern crate libc;
extern crate tokio;

use std::ffi::{CStr, CString};

use iota::{Client as IotaClient, MessageId, Result};
use libc::c_char;
use tokio::runtime::Runtime;

pub struct NodeClient {
    _runtime: Runtime,
    client: IotaClient,
}

impl NodeClient {
    fn new(node_uri: &str) -> Self {
        let node_client = IotaClient::new().node(node_uri).unwrap().build().unwrap();
        let runtime = tokio::runtime::Runtime::new().unwrap();
        NodeClient {
            _runtime: runtime,
            client: node_client,
        }
    }

    fn get_tips(&mut self) -> Result<(MessageId, MessageId)> {
        let res = self._runtime.block_on(self.client.get_tips());
        res
    }
}

#[no_mangle]
pub extern "C" fn client_new(s: *const c_char) -> *mut NodeClient {
    let c_str = unsafe {
        assert!(!s.is_null());
        CStr::from_ptr(s)
    };
    let node_uri = c_str.to_str().unwrap();
    Box::into_raw(Box::new(NodeClient::new(node_uri)))
}

#[no_mangle]
pub extern "C" fn client_free(ptr: *mut NodeClient) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

// A struct that can be passed between C and Rust
#[repr(C)]
pub struct GetTipsResponse {
    tip_1: *mut c_char,
    tip_2: *mut c_char,
    error: u32,
}

#[no_mangle]
pub extern "C" fn get_tips_response_free(ptr: *mut GetTipsResponse) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn client_get_tips(ptr: *mut NodeClient) -> *mut GetTipsResponse {
    let node_client = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    match node_client.get_tips() {
        Ok(tips) => {
            Box::into_raw(Box::new(GetTipsResponse {
                tip_1: CString::new(tips.0.to_string()).unwrap().into_raw(),
                tip_2: CString::new(tips.1.to_string()).unwrap().into_raw(),
                error: 0,
            }))
        }
        Err(e) => {
            panic!("unable to fetch tips: {}", e)
        }
    }
}