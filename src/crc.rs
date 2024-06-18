use std::marker::PhantomData;
use crate::bitreflection::*;
use crate::lut_generator::LutGenerator;

pub struct Crc<const POLY: usize, const INIT: usize, REFL: BitReflecttion> {
    phantom: PhantomData<REFL>,
}

impl<const POLY: usize, const INIT: usize, REFL: BitReflecttion> Crc<POLY, INIT, REFL> {
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


}
