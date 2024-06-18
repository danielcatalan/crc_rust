use std::marker::PhantomData;
use crate::c_for;


pub struct LutGenerator <T>{
    phantom: PhantomData<T>
}



impl LutGenerator<u8>{
    pub const fn generate_lut(poly: usize) -> [u8; 256] {
        let generator = poly as u8;
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

    #[test]
    fn test_table() {
        const TABLE: [u8; 256] = LutGenerator::<u8>::generate_lut(0x07);
        let x = TABLE[0];
        assert_eq!(0x00, x);

        let x = TABLE[1];
        assert_eq!(0x07, x);

        let x = TABLE[255];
        assert_eq!(0xF3, x);
    }
}


