use crate::bitreflection::*;
use crate::lut_generator::LutGenerator;
use std::marker::PhantomData;

pub struct Crc8<const POLY: u8, const INIT: u8, REFL: BitReflecttion> {
    phantom: PhantomData<REFL>,
}

pub struct Crc16<const POLY: u16, const INIT: u16, REFL: BitReflecttion> {
    phantom: PhantomData<REFL>,
}

pub struct Crc32<const POLY: u32, const INIT: u32, REFL: BitReflecttion> {
    phantom: PhantomData<REFL>,
}

impl<const POLY: u8, const INIT: u8, REFL: BitReflecttion> Crc8<POLY, INIT, REFL> {
    const LUT: [u8; 256] = LutGenerator::<u8>::generate_lut(POLY);

    pub fn calc_crc(input: &[u8]) -> u8 {
        let crc = input
            .iter()
            .map(|b| REFL::process(*b))
            .fold(INIT, |crc, b| {
                let pos = (b ^ crc) as usize;
                Self::LUT[pos]
            });
        REFL::process(crc)
    }
}

impl<const POLY: u16, const INIT: u16, REFL: BitReflecttion> Crc16<POLY, INIT, REFL> {
    const LUT: [u16; 256] = LutGenerator::<u16>::generate_lut(POLY);

    pub fn calc_crc(input: &[u8]) -> u16 {
        let crc = input
            .iter()
            .map(|b| REFL::process(*b))
            .fold(INIT, |crc, b| {
                let pos = (((crc >> 8) as u8) ^ b) as usize;
                (crc << 8) ^ (Self::LUT[pos])
            });
        REFL::process(crc)
    }
}

impl<const POLY: u32, const INIT: u32, REFL: BitReflecttion> Crc32<POLY, INIT, REFL> {
    const LUT: [u32; 256] = LutGenerator::<u32>::generate_lut(POLY);

    pub fn calc_crc(input: &[u8]) -> u32 {
        let crc = input
            .iter()
            .map(|b| REFL::process(*b))
            .fold(INIT, |crc, b| {
                let pos = ((crc ^ ((b as u32) <<24)) >> 24) as usize;
                (crc << 8) ^ (Self::LUT[pos])
            });
        REFL::process(crc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type Crc8Basic = Crc8<0x07, 0x00, NoReflect>;
    type Crc16XModem = Crc16<0x1021, 0x0000, NoReflect>;
    type Crc32Jamcrc  = Crc32<0x04C11DB7, 0xFFFFFFFF, Reflect>;

    #[test]
    fn test_simple_crc8() {
        let input = "123456789";

        let crc = Crc8Basic::calc_crc(input.as_bytes());
        assert_eq!(0xF4, crc);
    }

    #[test]
    fn test_simple_crc16() {
        let input = "123456789";
        // using CRC16/XMODEM
        let crc = Crc16XModem::calc_crc(input.as_bytes());
        assert_eq!(0x31C3, crc);
    }

    #[test]
    fn test_simple_crc32() {
        let input = "123456789";
        // using CRC-32/JAMCRC
        let crc = Crc32Jamcrc::calc_crc(input.as_bytes());
        assert_eq!(0x340BC6D9, crc);
    }
}
