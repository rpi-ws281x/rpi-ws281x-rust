use bindings;
use util::{ws2811_init, ws2811_fini};
use util::errors::WS281xError;

pub struct Controller {
    c_struct: ws2811_t
}

impl Controller {
    pub unsafe fn new(value: ws2811_t) -> Result<Controller,()> {
        return Ok(Controller{ c_struct: value });
    }

    pub fn init(&self) -> Result<(), WS281xError>  {
        return ws2811_init(&mut self.c_struct);
    }

}

impl Drop for Controller {
    fn drop(&mut self) {
        ws2811_fini(&mut self.c_struct);
    }
}