mod utils;


pub struct Crc<const poly: usize>{}

pub type Crc8 = Crc<7>;

impl<const poly: usize> Crc<poly> {
    const LUT: [u8;256] = Self::lut_generator();

    pub fn calc_crc(input: &str)-> u8 {
        let mut crc = 0;
        for b in input.as_bytes(){
            let data = (b ^ crc) as usize;
            crc = Self::LUT[data];
        }

        crc
    }

    const fn lut_generator() -> [u8;256]{
        let generator = poly as u8;
        let mut table = [0; 256];

        let mut dividend = 0;
        while dividend < 256 {
            let mut currbyte = dividend as u8;
            ////////////////////////
            c_for!(let mut bit = 0; bit < 8; bit +=1; {
                if currbyte & 0x80 != 0 {
                    currbyte <<=1;
                    currbyte ^= generator;
                }
                else {
                    currbyte <<=1;
                }
            });
            /////////////////////////////////
            table[dividend] = currbyte;
            dividend +=1;
        }

        table
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "123456789";

        let crc = Crc8::calc_crc(input);
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
