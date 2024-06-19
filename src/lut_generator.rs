use crate::c_for;
use std::marker::PhantomData;

pub struct LutGenerator<T> {
    phantom: PhantomData<T>,
}

impl LutGenerator<u8> {
    #[inline]
    pub const fn generate_lut(poly: u8) -> [u8; 256] {
        let generator = poly;
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

impl LutGenerator<u16> {
    #[inline]
    pub const fn generate_lut(poly: u16) -> [u16; 256] {
        let generator = poly;
        let mut table = [0; 256];

        c_for!(let mut dividend = 0; dividend < 256; dividend += 1; {
            let mut currbyte = (dividend << 8) as u16;

            c_for!(let mut bit = 0; bit < 8; bit +=1; {
                if currbyte & 0x8000 != 0 {
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

    #[test]
    fn test_crc8_table() {
        const TABLE: [u8; 256] = LutGenerator::<u8>::generate_lut(0x07);
        let x = TABLE[0];
        assert_eq!(0x00, x);

        let x = TABLE[1];
        assert_eq!(0x07, x);

        let x = TABLE[255];
        assert_eq!(0xF3, x);
    }

    #[test]
    fn test_crc16_table() {
        const TABLE: [u16; 256] = LutGenerator::<u16>::generate_lut(0x1021);
        let x = TABLE[0];
        assert_eq!(0x0000, x);

        let x = TABLE[1];
        assert_eq!(0x1021, x);

        let x = TABLE[255];
        assert_eq!(0x1EF0, x);
    }
}
