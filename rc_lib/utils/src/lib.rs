use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::CStr;
use std::slice;


/// Function adding a int with another int and return an int
#[no_mangle]
pub extern fn add(a: i32, b: i32) -> i32 {
    a + b
}

macro_rules! create_print {
    ($func_name:ident, $ty:ty) => {
        #[no_mangle]
        pub extern "C" fn $func_name(text: *const c_char, values: $ty) {
            unsafe {
                if let Ok(str) = CStr::from_ptr(text).to_str() {
                    let vec = str.split("{}").collect::<Vec<&str>>();
                    let vec_values = slice::from_raw_parts(values, vec.len()-1);
                    for i in 0..vec.len()-1 {
                        print!("{}{}", vec[i], vec_values[i]);
                    }
                    println!("{}", vec.last().unwrap());
                } else {
                    panic!("Error print text")
                }
            }
        }
    };
}
create_print!(print_str, *const c_char);
create_print!(print_int, *const c_int);
create_print!(print_bool, *const bool);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
