pub const title: &str = "Portuguese (Portugal)";
mod address;
mod internet;
mod name;
mod phone_number;
mod cell_phone;
mod commerce;
mod date;
pub use self::address::*;
pub use self::internet::*;
pub use self::name::*;
pub use self::phone_number::*;
pub use self::cell_phone::*;
pub use self::commerce::*;
pub use self::date::*;
