use super::super::bindings::{ws2811_t, ws2811_device, rpi_hw_t};
use super::super::util::Result;

#[derive(Debug)]
pub struct Controller {
    c_struct: ws2811_t,
    channels: Vec<Channel>,
}

impl Controller {
    /// Creates a new Controller
    /// Note: This is only to be called from the Builder struct, because some unsafe
    /// things have to be done, and thus it is appropriately marked unsafe.
    pub unsafe fn new(c_struct: ws2811_t) -> Self {
        let channels: Vec<Channel> = c_struct.channel.iter()
            .map(|val: _| Channel::new(val.clone()))
            .collect();
        Controller {
            c_struct,
            channels,
        }
    }
}
impl Drop for Controller {
    fn drop(&mut self) {
        unsafe {
            ws2811_fini(self.c_struct).into();
        }
    }
}