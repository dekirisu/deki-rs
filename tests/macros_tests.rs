use deki_macros::*;

// 1. ForceDefault
#[derive(ForceDefault)]
struct Config { timeout: u32, name: String }

#[test]
fn force_default_test() {
    let cfg = Config::default();
    assert_eq!(cfg.timeout, 0);
    assert!(cfg.name.is_empty());
}

// 2. match_fns!
enum Color { Red, Green, Blue }

match_fns!{
    [Color]
    name() -> &'static str;
    rgb() -> (u8, u8, u8);

    [::Red]
    name: "red";
    rgb: (255, 0, 0);

    [::Green]
    name: "green";
    rgb: (0, 255, 0);

    [::Blue]
    name: "blue";
    rgb: (0, 0, 255);
}

#[test]
fn match_fns_test() {
    assert_eq!(Color::Red.name(), "red");
    assert_eq!(Color::Green.rgb(), (0, 255, 0));
    assert_eq!(Color::Blue.name(), "blue");
}

// 4. EnumFieldCount
#[allow(dead_code)]
struct Rgb { r: u8, g: u8, b: u8 }

#[derive(EnumFieldCount)]
#[allow(dead_code)]
enum ColorEnum { Red, Green(Rgb), Blue(u8, u8, u8) }

#[test]
fn enum_field_count_test() {
    assert_eq!(ColorEnum::Red.field_count(), 0);
    assert_eq!(ColorEnum::Green(Rgb { r: 0, g: 0, b: 0 }).field_count(), 1);
    assert_eq!(ColorEnum::Blue(0, 0, 0).field_count(), 3);
}

// 5. xoxo!
#[test]
fn xoxo_test() {
    let result = xoxo!{match [true, false, true] {
        [O, O, O] => "all false",
        [O, O, X] => "nope",
        [X, O, X] => "YEP!",
        [_, _, _] => "default",
    }};
    assert_eq!(result, "YEP!");
}

// 7. quimp!
struct QuimpWrapper(i32);
impl QuimpWrapper {
    fn new(v: i32) -> Self { Self(v) }
}
quimp!{QuimpWrapper
    fn clone(&self) -> Self { Self::new(self.0) };
    fn default() -> Self { Self::new(100) };
}

#[test]
fn quimp_test() {
    assert_eq!(QuimpWrapper::default().0, 100);
    let w = QuimpWrapper::new(42);
    assert_eq!(w.clone().0, 42);
}

// 8. derived!
#[derived(_Hashable)]
struct DerivedPoint { x: i32, y: i32 }

#[test]
fn derived_test() {
    let p1 = DerivedPoint { x: 1, y: 2 };
    let p2 = DerivedPoint { x: 1, y: 2 };
    assert!(p1 == p2);
}
