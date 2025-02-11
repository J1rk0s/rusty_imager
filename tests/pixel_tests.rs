use rusty_imager::models::{Colors, Pixel};

#[test]
fn test_from_valid_hex() {
    let white = Pixel::from_hex("#ffffff");

    assert!(white.is_some());
    assert_eq!(white.unwrap(), Pixel { r: 255, g: 255, b: 255});
}

#[test]
fn test_from_invalid_hex() {
    let fail1 = Pixel::from_hex("ABCDEF");
    assert!(fail1.is_none());

    let fail2 = Pixel::from_hex("#g1f2a3");
    assert!(fail2.is_none());

    let fail3 = Pixel::from_hex("");
    assert!(fail3.is_none());
}

#[test]
fn test_to_hex() {
    let white = Colors::WHITE;

    assert_eq!(white.to_hex(), "#ffffff");
}