use crc::{Crc16AugCcitt, Crc16Ccitt};

#[test]
pub fn test_crc16_ccitt() {
    let input = "123456789";

    let crc = Crc16Ccitt::calc_crc(input.as_bytes());
    assert_eq!(0x31C3, crc);
}

#[test]
pub fn test_crc16_aug_ccitt() {
    let input = "123456789";

    let crc = Crc16AugCcitt::calc_crc(input.as_bytes());
    assert_eq!(0xE5CC, crc);
}
