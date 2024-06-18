use crc::Crc8;

#[inline(never)]
pub fn foo(input:&str)-> u8 {
    Crc8::calc_crc(input.as_bytes())
}

fn main() {
    let input = "123456789";
    println!("Input data: {input}");

    let crc = foo(input);
    println!("crc is 0x{:2x}",crc);
}