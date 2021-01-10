extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod bindings;
mod channel;
mod controller;
mod util;

pub use channel::ChannelBuilder;
pub use controller::{Controller, ControllerBuilder};
pub use util::{RawColor, Result, StripType, WS2811Error};
