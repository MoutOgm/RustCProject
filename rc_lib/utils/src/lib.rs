#[cxx::bridge(namespace = "rc::utils")]
mod ffi {

    struct T {
        a: i32,
        b: i32,
    }

    extern "Rust" {
        fn add(self: &T) -> i32;
        fn n() -> T;
    }
}

impl ffi::T {
    fn add(&self) -> i32 {
        self.a + self.b
    }
}
fn n() -> ffi::T {
    ffi::T { a: 1, b: 2 }
}