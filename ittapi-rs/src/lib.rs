
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![allow(unused)]
#[cfg(not(target_os = "windows"))]
include!(concat!(env!("OUT_DIR"), "/ittnotify_bindings.rs"));
#[cfg(not(target_os = "windows"))]
include!(concat!(env!("OUT_DIR"), "/jitprofiling_bindings.rs"));
