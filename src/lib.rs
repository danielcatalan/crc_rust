pub struct Crc;

impl Crc {
    const LUT: [u8;256] = Self::lut_generator();

    pub fn calc_crc(input: &str)-> u8 {


        todo!()
    }

    const fn lut_generator() -> [u8;256]{
        let table = [0; 256];

        let mut dividend = 0;
        while dividend < 256 {
            let currbyte = dividend as u8;

            let mut bit: u8 = 0;
            while bit < 8{
                bit
            }
            
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

        let crc = Crc::calc_crc(input);
        assert_eq!(0xF4, crc);
    }
}
