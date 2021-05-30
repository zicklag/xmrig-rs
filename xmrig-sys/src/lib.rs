use std::os::raw::{c_char, c_int};

#[link(name = "xmrig", kind = "static")]
#[link(name = "xmrig-asm", kind = "static")]
#[link(name = "ethash", kind = "static")]
#[link(name = "ssl", kind = "static")]
#[link(name = "crypto", kind = "static")]
#[link(name = "pthread", kind = "static")]
#[link(name = "rt", kind = "static")]
#[link(name = "dl", kind = "static")]
#[link(name = "stdc++", kind = "dylib")]
extern "C" {
    pub fn xmrig_start(x: c_int, y: *mut *mut c_char);
}
