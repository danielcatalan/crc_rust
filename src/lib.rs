mod bitreflection;
mod crc;
mod lut_generator;
mod utils;

pub use bitreflection::{NoReflect, Reflect};
pub use crc::{Crc16, Crc8};

pub type Crc8Basic = Crc8<0x07, 0x00, NoReflect>;
pub type Crc8Cdma2000 = Crc8<0x9B, 0xFF, NoReflect>;
pub type Crc8Darc = Crc8<0x39, 0x00, Reflect>;
pub type Crc8DvbS2 = Crc8<0xD5, 0x00, NoReflect>;

pub type Crc16Ccitt = Crc16<0x1021, 0x0000, NoReflect>;
pub type Crc16AugCcitt = Crc16<0x1021, 0x1D0F, NoReflect>;
