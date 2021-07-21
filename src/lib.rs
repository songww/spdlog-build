#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub const spdlog_level_level_enum_trace: spdlog_level_level_enum = 0;
pub const spdlog_level_level_enum_debug: spdlog_level_level_enum = 1;
pub const spdlog_level_level_enum_info: spdlog_level_level_enum = 2;
pub const spdlog_level_level_enum_warn: spdlog_level_level_enum = 3;
pub const spdlog_level_level_enum_err: spdlog_level_level_enum = 4;
pub const spdlog_level_level_enum_critical: spdlog_level_level_enum = 5;
pub const spdlog_level_level_enum_off: spdlog_level_level_enum = 6;
pub const spdlog_level_level_enum_n_levels: spdlog_level_level_enum = 7;
pub type spdlog_level_level_enum = ::std::os::raw::c_uint;
extern "C" {
    #[link_name = "\u{1}__ZN6spdlog5level14to_short_c_strENS0_10level_enumE"]
    pub fn spdlog_level_to_short_c_str(l: spdlog_level_level_enum)
        -> *const ::std::os::raw::c_char;
}
