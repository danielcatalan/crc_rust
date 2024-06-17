use crc::{Crc8Cdma2000,Crc8DARC};

#[test]
pub fn test_crc8_cdma2000() {
    let input = "123456789";

    let crc = Crc8Cdma2000::calc_crc(input.as_bytes());
    assert_eq!(0xDA, crc);
}

#[test]
pub fn test_crc8_darc() {
    let input = "123456789";

    let crc = Crc8DARC::calc_crc(input.as_bytes());
    assert_eq!(0x15, crc);
}
