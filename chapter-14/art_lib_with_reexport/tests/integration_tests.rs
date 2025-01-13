use art_lib_with_reexport::{mix, PrimaryColor};

/// Tests the [`mix`] function with the [`PrimaryColor::Red`] and [`PrimaryColor::Yellow`] colors.
#[test]
fn red_and_yellow_make_orange() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    let result = mix(red, yellow);
    assert_eq!(result, Some(art_lib_with_reexport::SecondaryColor::Orange));
}

/// Tests the [`mix`] function with the [`PrimaryColor::Red`] and [`PrimaryColor::Blue`] colors.
#[test]
fn red_and_blue_make_purple() {
    let red = PrimaryColor::Red;
    let blue = PrimaryColor::Blue;
    let result = mix(red, blue);
    assert_eq!(result, Some(art_lib_with_reexport::SecondaryColor::Purple));
}

/// Tests the [`mix`] function with the [`PrimaryColor::Yellow`] and [`PrimaryColor::Blue`] colors.
#[test]
fn yellow_and_blue_make_green() {
    let yellow = PrimaryColor::Yellow;
    let blue = PrimaryColor::Blue;
    let result = mix(yellow, blue);
    assert_eq!(result, Some(art_lib_with_reexport::SecondaryColor::Green));
}

/// Tests the [`mix`] function with the [`PrimaryColor::Red`] and [`PrimaryColor::Red`] colors.
#[test]
fn red_and_red_make_none() {
    let red = PrimaryColor::Red;
    let result = mix(red, red);
    assert_eq!(result, None);
}

/// Tests the [`mix`] function with the [`PrimaryColor::Yellow`] and [`PrimaryColor::Yellow`] colors.
#[test]
fn yellow_and_yellow_make_none() {
    let yellow = PrimaryColor::Yellow;
    let result = mix(yellow, yellow);
    assert_eq!(result, None);
}

/// Tests the [`mix`] function with the [`PrimaryColor::Blue`] and [`PrimaryColor::Blue`] colors.
#[test]
fn blue_and_blue_make_none() {
    let blue = PrimaryColor::Blue;
    let result = mix(blue, blue);
    assert_eq!(result, None);
}