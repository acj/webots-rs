pub mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!("bindings.rs");
}

use bindings::WbDeviceTag;
use std::ffi::CString;

pub fn wb_distance_sensor_enable(tag: WbDeviceTag, sampling_period: i32) {
    unsafe {
        crate::bindings::wb_distance_sensor_enable(tag, sampling_period);
    }
}

pub fn wb_distance_sensor_get_value(tag: WbDeviceTag) -> f64 {
    unsafe { crate::bindings::wb_distance_sensor_get_value(tag) }
}

pub fn wb_motor_set_position(device: WbDeviceTag, position: f64) {
    unsafe { crate::bindings::wb_motor_set_position(device, position) }
}

pub fn wb_motor_set_velocity(device: WbDeviceTag, velocity: f64) {
    unsafe { crate::bindings::wb_motor_set_velocity(device, velocity) }
}

pub fn wb_robot_get_device(id: &str) -> WbDeviceTag {
    let name = CString::new(id).expect("CString::new failed");
    unsafe { crate::bindings::wb_robot_get_device(name.as_ptr()) }
}

pub fn wb_robot_cleanup() {
    unsafe { crate::bindings::wb_robot_cleanup() }
}

pub fn wb_robot_init() {
    unsafe {
        crate::bindings::wb_robot_init();
    }
}

pub fn wb_robot_step(step: i32) -> i32 {
    unsafe { crate::bindings::wb_robot_step(step) }
}
