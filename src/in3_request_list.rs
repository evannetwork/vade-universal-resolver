use std::os::raw::{c_char, c_void};

extern "C" {
    pub fn resolve_http_request(
    vade_req_ctx: *const c_void, 
    url: *const c_char, 
    method: *const c_char, 
    path: *const c_char, 
    payload: *const c_char, 
    res: *const c_char) -> i32;
}