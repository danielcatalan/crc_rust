use crc::{Crc32Jamcrc};

#[test]
pub fn test_crc32_jamcrc() {
    let input = "123456789";

    let crc = Crc32Jamcrc::calc_crc(input.as_bytes());
    assert_eq!(0x340BC6D9, crc);
}
