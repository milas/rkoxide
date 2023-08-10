#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::raw::c_void;

    #[test]
    fn do_thing() {
        let mut ctx: rknn_context = 0;
        let mut model_data: Vec<u8> = vec![0, 4];
        unsafe {
            rknn_init(
                &mut ctx,
                model_data.as_mut_ptr() as *mut c_void,
                model_data.len() as u32,
                0,
                std::ptr::null_mut(),
            );
        }
    }
}
