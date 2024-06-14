use crc::Crc;

#[test]
pub fn test_crc8_cdma2000() {
    let input = "123456789";

    let crc = Crc::<0x9B,0xFF>::calc_crc(input);
    assert_eq!(0xDA, crc);
}

#[test]
pub fn test_crc8_darc() {
    let input = "123456789";

    let crc = Crc::<0x39,0x00>::calc_crc(input);
    assert_eq!(0x15, crc);
}
