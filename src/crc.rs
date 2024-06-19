use std::marker::PhantomData;
use crate::bitreflection::*;
use crate::lut_generator::LutGenerator;

pub struct Crc<T,const POLY: usize, const INIT: usize, REFL: BitReflecttion> {
    phantom1: PhantomData<T>,
    phantom2: PhantomData<REFL>,
}

impl<const POLY: usize, const INIT: usize, REFL: BitReflecttion> Crc<u8, POLY, INIT, REFL> {
    const LUT: [u8; 256] = LutGenerator::<u8>::generate_lut(POLY as u8);

    pub fn calc_crc(input: &[u8]) -> u8 {

        let crc = input.iter()
        .map(|b| REFL::process(*b))
        .fold(INIT as u8, |crc, b| {
            let data = (b ^ crc) as usize;
            Self::LUT[data]
        });
        REFL::process(crc)
    }
}

impl<const POLY: usize, const INIT: usize, REFL: BitReflecttion> Crc<u16, POLY, INIT, REFL> {
    const LUT: [u16; 256] = LutGenerator::<u16>::generate_lut(POLY as u16);

    pub fn calc_crc(input: &[u8]) -> u16 {

        let crc = input.iter()
        .map(|b| REFL::process(*b))
        .fold(INIT as u16, |crc, b| {
            let data = (((crc>>8) as u8) ^ b) as usize;
            (crc << 8) ^ (Self::LUT[data])
        });
        REFL::process(crc)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    type Crc8 = Crc<u8, 0x07, 0x00, NoReflect>;
    type Crc16XModem = Crc::<u16,0x1021,0x0000,NoReflect>;

    #[test]
    fn test_simple_crc8() {
        let input = "123456789";

        let crc = Crc8::calc_crc(input.as_bytes());
        assert_eq!(0xF4, crc);
    }

    #[test]
    fn test_simple_crc16() {
        let input = "123456789";
        // using CRC16/XMODEM
        let crc = Crc16XModem::calc_crc(input.as_bytes());
        assert_eq!(0x31C3, crc);
    }


}
