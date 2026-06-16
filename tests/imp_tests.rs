use deki_macros::imp;
use deki::core::ext;

// 1. Basic: add method to owned type
struct MyStruct {
    value: i32,
}
#[imp(MyStruct)]
fn new(value: i32) -> Self {
    Self { value }
}

#[test]
fn imp_basic() {
    let s = MyStruct::new(42);
    assert_eq!(s.value, 42);
}

// 2. Auto-create unit struct with * prefix
#[imp(*Counter)]
fn new_unit() -> Self {
    Self
}

#[test]
fn imp_unit_struct() {
    let c = Counter::new_unit();
    let _ = c;
}

// 3. Impl a single-method trait
trait Greet {
    fn greet(&self) -> String;
}

#[imp(MyStruct|Greet)]
fn greet(&self) -> String {
    format!("Hello, value is {}", self.value)
}

#[test]
fn imp_trait_impl() {
    let s = MyStruct::new(7);
    assert_eq!(s.greet(), "Hello, value is 7");
}

// 4. Foreign type: * generates a new trait for a local type
#[imp(LocalWrapper|*)]
fn wrap_inner(&self) -> Self {
    LocalWrapper(self.0 + 1)
}
struct LocalWrapper(i32);

#[test]
fn imp_foreign_type() {
    let w = LocalWrapper(5);
    let wrapped = w.wrap_inner();
    assert_eq!(wrapped.0, 6);
}

// 5. Generics
struct GenericStruct<T> {
    value: T,
}
#[imp(GenericStruct<T>)]
fn new_generic(value: T) -> Self {
    Self { value }
}

#[test]
fn imp_generics() {
    let s = GenericStruct::new_generic(42i32);
    assert_eq!(s.value, 42);
    let s2 = GenericStruct::new_generic("hello");
    assert_eq!(s2.value, "hello");
}

// 6. Method with multiple params
struct MultiParamStruct {
    a: i32,
    b: String,
    c: bool,
}
#[imp(MultiParamStruct)]
fn create(a: i32, b: String, c: bool) -> Self {
    Self { a, b, c }
}

#[test]
fn imp_multi_params() {
    let s = MultiParamStruct::create(1, "test".into(), true);
    assert_eq!(s.a, 1);
    assert_eq!(s.b, "test");
    assert!(s.c);
}


// Block \\

#[imp(Greet)]
impl String {
    fn greet(&self) -> Self {self.clone()}
}

#[test]
fn imp_block_impl_trait() {
    let s = "hello".to_string();
    assert_eq!(Greet::greet(&s), "hello");
}

#[imp(*GreetToo)]
impl String {
    fn greet_too(&self) -> Self {self.clone()}
}

#[test]
fn imp_block_gen_trait() {
    let s = "world".to_string();
    assert_eq!(GreetToo::greet_too(&s), "world");
}

#[imp(*)]
impl String {
    fn greet(&self) -> Self {self.clone()}
}

#[test]
fn imp_block_auto_trait() {
    let s = "auto".to_string();
    assert_eq!(StringGreetExt::greet(&s), "auto");
}
