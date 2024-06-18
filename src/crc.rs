use std::marker::PhantomData;
use crate::bitreflection::*;
use crate::c_for;

pub struct Crc<const POLY: usize, const INIT: usize, REFL: BitReflecttion> {
    phantom: PhantomData<REFL>,
}

impl<const POLY: usize, const INIT: usize, REFL: BitReflecttion> Crc<POLY, INIT, REFL> {
    const LUT: [u8; 256] = Self::lut_generator();

    pub fn calc_crc(input: &[u8]) -> u8 {

        let crc = input.iter()
        .map(|b| REFL::process(*b))
        .fold(INIT as u8, |crc, b| {
            let data = (b ^ crc) as usize;
            Self::LUT[data]
        });
        REFL::process(crc)
    }

    const fn lut_generator() -> [u8; 256] {
        let generator = POLY as u8;
        let mut table = [0; 256];

        c_for!(let mut dividend = 0; dividend < 256; dividend += 1; {
            let mut currbyte = dividend as u8;

            c_for!(let mut bit = 0; bit < 8; bit +=1; {
                if currbyte & 0x80 != 0 {
                    currbyte <<=1;
                    currbyte ^= generator;
                }
                else {
                    currbyte <<=1;
                }
            });

            table[dividend] = currbyte;
        });

        table
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type Crc8 = Crc<0x07, 0x00, NoReflect>;

    #[test]
    fn it_works() {
        let input = "123456789";

        let crc = Crc8::calc_crc(input.as_bytes());
        assert_eq!(0xF4, crc);
    }

    #[test]
    fn test_table() {
        let x = Crc8::LUT[0];
        assert_eq!(0x00, x);

        let x = Crc8::LUT[1];
        assert_eq!(0x07, x);

        let x = Crc8::LUT[255];
        assert_eq!(0xF3, x);
    }
}
