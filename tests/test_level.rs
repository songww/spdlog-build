use spdlog_src::*;
use std::ffi::{CStr, CString};

#[test]
fn test_level_to_c_str() {
    assert_eq!(
        unsafe { CStr::from_ptr(spdlog_level_to_short_c_str(spdlog_level_level_enum_debug)) },
        CString::new("D").unwrap().as_c_str()
    );
}
