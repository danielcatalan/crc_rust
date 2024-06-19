use crc::Crc8Basic;

#[inline(never)]
pub fn calc_crc8(input: &str) -> u8 {
    Crc8Basic::calc_crc(input.as_bytes())
}

fn main() {
    let input = "123456789";
    println!("Input data: {input}");

    let crc = calc_crc8(input);
    println!("crc is 0x{:2x}", crc);
}
