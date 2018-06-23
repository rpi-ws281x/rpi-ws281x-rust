use super::super::bindings::{ws2811_t, ws2811_fini, ws2811_render};
use super::super::util::Result;
use super::super::channel::{Channel};

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
        let channels: Vec<Channel> = c_struct.channel.iter_mut()
            .map(|val: &mut _| Channel::new(val))
            .collect();
        Controller {
            c_struct,
            channels,
        }
    }

    pub fn channel(&self, index: usize) -> &Channel {
        return &self.channels[index];
    }

    pub fn channel_mut(&mut self, index: usize) -> &mut Channel {
        return &mut self.channels[index];
    }

    /// Render the colors to the string.
    ///
    /// It doesn't automatically do this because it
    /// is a somewhat costly operation that should 
    /// be batched.
    pub fn render(&mut self) -> Result<()> {
        unsafe {
            return ws2811_render(self.c_struct).into();
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