mod bitreflection;
mod utils;
mod crc;
mod lut_generator;

pub use crc::Crc; 
pub use bitreflection::{NoReflect,Reflect};



pub type Crc8 = Crc<u8, 0x07, 0x00, NoReflect>;
pub type Crc8Cdma2000 = Crc<u8, 0x9B, 0xFF, NoReflect>;
pub type Crc8Darc = Crc<u8, 0x39, 0x00, Reflect>;
pub type Crc8DvbS2 = Crc<u8, 0xD5, 0x00, NoReflect>;

