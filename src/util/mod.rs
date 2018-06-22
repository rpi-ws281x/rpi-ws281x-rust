pub type RawColor = [u8; 4];

mod strip_type;
pub use strip_type::StripType;

mod error;
pub use error::{Result, WS2811Error};