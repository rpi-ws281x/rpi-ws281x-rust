use errors::*;
use bindings;
use bindings::{ws2811_t, ws2811_return_t};

fn ws2811_init(c_struct: *mut ws2811_t) -> Result<(), WS281xError> {
    unsafe {
        let ret = bindings::ws2811_init(c_struct);
        if ret == 0 {
            Ok();
        } else {
            Err(ret as WS281xError);
        }
    }
}
fn ws2811_fini(c_struct: *mut ws2811_t) -> Result<(), WS281xError> {
    unsafe {
        let ret = bindings::ws2811_fini(c_struct);
        if ret == 0 {
            Ok();
        } else {
            Err(ret as WS281xError);
        }
    }
}
fn ws2811_render(c_struct: *mut ws2811_t) -> Result<(), WS281xError> {
    unsafe {
        let ret = bindings::ws2811_render(c_struct);
        if ret == 0 {
            Ok();
        } else {
            Err(ret as WS281xError);
        }
    }
}
fn ws2811_wait(c_struct: *mut ws2811_t) -> Result<(), WS281xError> {
    unsafe {
        let ret = bindings::ws2811_wait(c_struct);
        if ret == 0 {
            Ok();
        } else {
            Err(ret as WS281xError);
        }
    }
}
