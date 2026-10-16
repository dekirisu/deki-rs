use deki_macros::{phantom, *};
use deki_core::choose;

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

// 9. phantom — PhantomData + auto new() constructor \

// The real use-case: generics declared but never used in fields.
// Without PhantomData, Rust complains about unused type/lifetime params.

// 9a. Named struct with unused generic
#[phantom]
struct Wrapper<T> {
    data: String,
}

#[test]
fn phantom_named_unused_generic() {
    let w = Wrapper::<i32>::new("hello".into());
    assert_eq!(w.data, "hello");
}

// 9b. Named struct with unused generic (multiple)
#[phantom]
struct Pair<A, B> {
    data: i32,
}

#[test]
fn phantom_named_multi_generics() {
    let p = Pair::<String, f64>::new(42);
    assert_eq!(p.data, 42);
}

// 9c. Tuple struct with unused generic
#[phantom]
#[allow(dead_code)]
struct Boxed<T>(i32, f64);

#[test]
fn phantom_tuple_unused_generic() {
    let b = Boxed::<String>::new(1, 2.0);
    assert_eq!(b.0, 1);
}

// 9d. Tuple struct with no generics (PhantomData<()>)
#[phantom]
#[allow(dead_code)]
struct UnitLike(i32, f64);

#[test]
fn phantom_tuple_no_generics() {
    let u = UnitLike::new(10, 3.);
    assert_eq!(u.0, 10);
}

// 9e. Unit struct (converted to single-field tuple)
#[phantom]
struct Singleton;

#[test]
fn phantom_unit_struct() {
    let _s = Singleton::new();
}

// 10. Enum — adds Phantom(G1, G2, ...) variant \

// Same principle: generics declared but never used in any variant.

// 10a. Enum with unused generic
#[phantom]
enum Maybe<T> { None, Some(i32) }

#[test]
fn phantom_enum_unused_generic() {
    let m = Maybe::<String>::None;
    assert!(matches!(m, Maybe::None));
}

// 10b. Enum with unused generic (no variants use it)
#[phantom]
enum Marker<T> { Pending, Done }

#[test]
fn phantom_enum_marker() {
    let m = Marker::<String>::Pending;
    assert!(matches!(m, Marker::Pending));
}

// 10c. Enum with no generics (PhantomData<()>)
#[phantom]
enum State { Pending, Done }

#[test]
fn phantom_enum_no_generics() {
    let s = State::Pending;
    assert!(matches!(s, State::Pending));
}

// 10d. Enum with multiple unused generics
#[phantom]
enum Tag<A, B> { Ready, Waiting }

#[test]
fn phantom_enum_multi_generics() {
    let t = Tag::<i32, String>::Ready;
    assert!(matches!(t, Tag::Ready));
}

// 11. if_! — ternary for any type \

#[test]
fn choose_true_i32() {
    let x: i32 = choose!(true, 42, 0);
    assert_eq!(x, 42);
}

#[test]
fn choose_false_i32() {
    let x: i32 = choose!(false, 42, 0);
    assert_eq!(x, 0);
}

#[test]
fn choose_str() {
    let s: &str = choose!(true, "yes", "no");
    assert_eq!(s, "yes");
}

#[test]
fn choose_vec() {
    let v: Vec<i32> = choose!(false, vec![1], vec![2]);
    assert_eq!(v, vec![2]);
}
