pub type RawColor = [u8; 4];

mod strip_type;
pub use self::strip_type::StripType;

mod error;
pub use self::error::{Result, WS2811Error};