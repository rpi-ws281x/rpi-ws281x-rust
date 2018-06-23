use super::super::bindings::{ws2811_fini, ws2811_render, ws2811_t};
use super::super::channel::Channel;
use super::super::util::Result;

#[derive(Debug)]
pub struct Controller {
    c_struct: ws2811_t,
}

impl Controller {
    /// Creates a new Controller
    /// Note: This is only to be called from the Builder struct, because some unsafe
    /// things have to be done, and thus it is appropriately marked unsafe.
    pub unsafe fn new(c_struct: ws2811_t) -> Self {
        Controller { c_struct }
    }

    pub fn channel(&mut self, index: usize) -> Channel {
        return Channel::new(&mut self.c_struct.channel[index]);
    }

    /// Render the colors to the string.
    ///
    /// It doesn't automatically do this because it
    /// is a somewhat costly operation that should
    /// be batched.
    pub fn render(&mut self) -> Result<()> {
        unsafe {
            return ws2811_render(&mut self.c_struct).into();
        }
    }
}
impl Drop for Controller {
    fn drop(&mut self) {
        unsafe {
            ws2811_fini(&mut self.c_struct);
        }
    }
}
