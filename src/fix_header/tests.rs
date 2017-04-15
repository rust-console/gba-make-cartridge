use std::io::Cursor;
use super::fix_header;

const GAME_TITLE: [u8; 12] = [60, 0, 0, 0,
                               0, 0, 0, 0,
                               0, 0, 0, 0];
const GAME_CODE: [u8; 4] = [60, 61, 62, 63];
const MANU_CODE: [u8; 2] = [64, 65];
const VERSION: u8 = 1;

#[test]
fn first_four_bytes_unchanged() {
    let first_four = [0xAA, 0xBB, 0xCC, 0xDD];
    let mut header = vec!(0u8; 4);
    header[0..4].copy_from_slice(&first_four);
    let mut header_cursor = Cursor::new(header);
    assert!(
        fix_header(&mut header_cursor,
               &GAME_TITLE,
               &GAME_CODE,
               &MANU_CODE,
                   VERSION).is_ok());
    assert_eq!(&header_cursor.get_ref()[0..4], &first_four[..])
}

#[test]
fn valid_metadata() {
    let mut header_cursor = Cursor::new(vec!(0xCA; 0xD0));
    assert!(
        fix_header(&mut header_cursor,
               &GAME_TITLE,
               &GAME_CODE,
               &MANU_CODE,
               VERSION).is_ok());
    assert_eq!(&header_cursor.get_ref()[0xA0..0xAC], &GAME_TITLE);
    assert_eq!(&header_cursor.get_ref()[0xAC..0xB0], &GAME_CODE);
    assert_eq!(&header_cursor.get_ref()[0xB0..0xB2], &MANU_CODE);
    assert_eq!(header_cursor.get_ref()[0xB2], 0x96); // fixed value
    assert_eq!(header_cursor.get_ref()[0xB3], 0x00); // main unit code
    assert_eq!(header_cursor.get_ref()[0xB4] & 0x7F, 0x00); // sub-device
    assert_eq!(&header_cursor.get_ref()[0xB5..0xBC], &[0; 7]); // reserved
    assert_eq!(header_cursor.get_ref()[0xBC], VERSION); // version
    assert_eq!(&header_cursor.get_ref()[0xBE..0xC0], &[0, 0]); // reserved
}

#[test]
fn no_overwrite() {
    let rom_data = [0xBA; 0x10];
    let mut header_cursor = Cursor::new(vec!(0xCA; 0xD0));
    header_cursor.get_mut()[0xC0..0xD0].copy_from_slice(&rom_data);
    assert!(
        fix_header(&mut header_cursor,
               &GAME_TITLE,
               &GAME_CODE,
               &MANU_CODE,
               VERSION).is_ok());
    assert_eq!(&header_cursor.get_ref()[0xC0..0xD0], &rom_data);
}