pub mod errors;
pub mod strip;

use errors::WS281xResult;
use bindings;
use bindings::{ws2811_t, ws2811_return_t};

fn return_result(ret: ws2811_return_t) -> Result<(), WS281xResult> {
    if ret == ws2811_return_t::WS2811_SUCCESS {
        Ok(())
    } else {
        Err(ret.into())
    }
}

/// A low-level function that initializes the ws281x strand.
///
/// This requires that a ws2811_t struct be instantiated with
/// some default data before calling this function. See both the
/// ControllerBuilder and ChannelBuilder structs for the required
/// information needed.
pub fn ws2811_init(c_struct: *mut ws2811_t) -> Result<(), WS281xResult> {
    unsafe {
        return_result(bindings::ws2811_init(c_struct))
    }
}

/// A low-level function that frees memory associated with the ws281x strand.
pub fn ws2811_fini(c_struct: *mut ws2811_t) {
    unsafe {
        bindings::ws2811_fini(c_struct);
    }
}

/// A low-level function that sends the local pixel data to the strand
///
/// The strand *must* be initialized first before calling!
pub fn ws2811_render(c_struct: *mut ws2811_t) -> Result<(), WS281xResult> {
    unsafe {
        return_result(bindings::ws2811_render(c_struct))
    }
}

/// A low-level function that waits for any executing DMA operation to
/// complete before returning.
///
/// It shouldn't be necessary to call this function normally as both ws2811_fini
/// and ws2811_render call this function already.
pub fn ws2811_wait(c_struct: *mut ws2811_t) -> Result<(), WS281xResult> {
    unsafe {
        return_result(bindings::ws2811_wait(c_struct))
    }
}
