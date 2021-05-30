pub fn start() {
    unsafe {
        xmrig_sys::xmrig_start(0, std::ptr::null_mut());
    }
}
