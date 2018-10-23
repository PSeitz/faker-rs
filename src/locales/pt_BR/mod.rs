pub const title: &str = "Portuguese (Brazil)";
mod address;
mod company;
mod internet;
mod lorem;
mod name;
mod phone_number;
pub use self::address::*;
pub use self::company::*;
pub use self::internet::*;
pub use self::lorem::*;
pub use self::name::*;
pub use self::phone_number::*;
