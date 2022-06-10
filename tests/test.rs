use color_hex::color_from_hex;

#[test]
fn test_const() {
    const DATA: [u8; 3] = color_from_hex!("010203");
    assert_eq!(DATA, [1, 2, 3]);
}

#[test]
fn test_character_cases() {
    assert_eq!(color_from_hex!("a1 b2 c3 d4"), [0xA1, 0xB2, 0xC3, 0xD4]);
    assert_eq!(color_from_hex!("E5 E6 90 92"), [0xE5, 0xE6, 0x90, 0x92]);
    assert_eq!(color_from_hex!("0a0B 0C"), [10, 11, 12]);
    assert_eq!(color_from_hex!("a2  	b9 c4 D 3"), [0xA2, 0xB9, 0xC4, 0xD3]);
}

#[test]
fn test_hash() {
    assert_eq!(color_from_hex!("#2d2d2d"), [0x2d, 0x2d, 0x2d]);
}
