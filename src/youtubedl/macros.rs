#[macro_export]
macro_rules! vec_extern {
    ($clazz:ident, $vec:ident, $r:ty, $func_get_name:ident, $func_len_name:ident) => {
        #[no_mangle]
        pub extern "C" fn $func_get_name(self_i: &$clazz, index: usize) -> *const $r {
            unsafe {
                return match (*self_i.$vec).get(index) {
                    Some(e) => e,
                    None => ptr::null(),
                };
            }
        }

        #[no_mangle]
        pub extern "C" fn $func_len_name(self_i: &$clazz) -> usize {
            unsafe {
                return (*self_i.$vec).len();
            }
        }
    };
}

#[macro_export]
macro_rules! drop_extern {
    ($clazz:ident, $methodName:ident) => {
        #[no_mangle]
        pub extern "C" fn $methodName(self_i: *mut $clazz) {
            unsafe {
                self_i.drop_in_place();
            }
        }
    };
}

#[macro_export]
macro_rules! map_extern {
    ($clazz:ident, $hashmap:ident, $k:ty, $r:ty, $func_get_name:ident, $func_len_name:ident, $func_get_key_name:ident) => {
        #[no_mangle]
        pub extern "C" fn $func_get_name(self_i: &$clazz, index: &$k) -> *const $r {
            unsafe {
                return match (*self_i.$hashmap).get(index) {
                    Some(e) => e,
                    None => ptr::null(),
                };
            }
        }

        #[no_mangle]
        pub extern "C" fn $func_get_key_name(self_i: &$clazz, index: usize) -> *const $k {
            unsafe {
                let keys: Vec<&$k> = (*self_i.$hashmap).keys().collect();
                return match keys.get(index) {
                    Some(e) => *e,
                    None => ptr::null(),
                };
            }
        }

        #[no_mangle]
        pub extern "C" fn $func_len_name(self_i: &$clazz) -> usize {
            unsafe {
                return (*self_i.$hashmap).len();
            }
        }
    };
}
