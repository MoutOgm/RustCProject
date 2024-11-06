pub mod values;

#[no_mangle]
pub extern "C" fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(1, 3);
        assert_eq!(result, 4);
        assert_eq!(42, values::value());
    }
}
