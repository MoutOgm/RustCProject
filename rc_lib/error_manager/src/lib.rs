#[cxx::bridge(namespace = "rc::error_manager")]
mod ffi {
    extern "Rust" {
        fn new_manager();

    }
}

#[no_mangle]
pub extern fn new_manager() {

}
