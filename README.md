# deki-rs

[![GitHub](https://img.shields.io/badge/github-dekirisu/deki-ee6677)](https://github.com/dekirisu/deki-rs/)
[![crates.io](https://img.shields.io/badge/crates.io-deki-orange)](https://crates.io/crates/deki)
[![CI](https://github.com/dekirisu/deki-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/dekirisu/deki-rs/actions/workflows/ci.yml)

A personal Rust utility crate — helper types, traits, macros, and re-exports that reduce boilerplate.

---

## Quick Start

```toml
[dependencies]
deki = { version = "0.4", features = ["random", "lerp"] }
```

```rust
use deki::lerp::Lerpable;

let val: f32 = 0.0.lerp(10.0, 0.5);  // 5.0
```

---

## Features

| Feature | Default | What it adds |
|---|---|---|
| `random` | ✅ | `fastrand` re-exports, `f32r()`, `Vec::random()` |
| `approx` | ✅ | Fast bit-hack math via `DekiExtApprox` — `sqrt_fast`, `exp_fast`, `pow_fast`, `log2_fast` |
| `lerp` | ✅ | Linear / gated / cyclic / step interpolation traits and methods |
| `derive_more` | ✅ | Full `derive_more` re-export (Debug, Clone, PartialEq, etc.) |
| `proc` | — | Proc-macro support (string→ident, token stream helpers) |

---

## deki_core — Runtime Utilities

### Core Types

#### `StackMap<K, V>` — Insertion-Ordered Key-Value Map

Preserves insertion order and allows duplicate keys. Keys stored in a `Vec` — linear search.

```rust
let mut map: StackMap<&str, i32> = Default::default();
*map.entry("count") = 42;
assert_eq!(*map.entry("count"), 42);

for (key, value) in map.iter() { ... } // insertion order
```

#### `Syncable` — `'static + Send + Sync` in One Word

```rust
fn spawn<T: Syncable>(val: T) { ... }
```

#### `DefaultClear` — Reset Any Default Type

Adds `.clear()` to any `Default` type (resets to `Self::default()`).

```rust
let mut state = MyState::default();
state.clear();  // resets to MyState::default()
```

#### Aliases

| Name | Is |
|---|---|
| `Ghost` | `PhantomData` |
| `Str` | `&'static str` |
| `New` | `derive_new::new` |
| `ext` | `extension_traits::extension` |

Plus re-exports of `buns`, `type_cell`, and `maflow`.

---

### Math

#### Approximate Math (`approx` feature)

Fast bit-hack math (~1% error, bit-hack based):

```rust
use deki_core::math::DekiExtApprox;

let x = 1.5f32;
x.exp_fast();      // ~1% error, 2.2× faster than std
x.pow_fast(3.0);   // ~1% error, 1.5× faster than std
x.log2_fast();     // ~1% error
x.sqrt_fast();     // ~0.1% error, 1.4× faster than std
```

#### Interpolation (`lerp` feature)

**Linear Interpolation**

```rust
use deki::lerp::Lerpable;

let val: f32 = 0.0.lerp(10.0, 0.5);       // 5.0

// For integers (uses mul_f32 rounding)
use deki::lerp::LerpableF32;
let val: i32 = 0i32.lerp(100, 0.3);       // 30
```

**Gated Lerp — Snap When Close**

```rust
use deki::lerp::Glerpable;
let mut val = 0.0f32;
let arrived = val.glerp(10.0, 0.1, 0.5);  // snaps when within 0.5 of target
```

**Cyclic Lerp — Auto-Choose Shortest Direction**

```rust
use deki::lerp::Clerpable;

let val: f32 = 0.0.lerp_qucy(0.9, 0.1, 0.0, 1.0);
// Result: 0.99 (not 0.09) — picks the shorter path

// Gated cyclic variant
let mut val = 0.99f32;
let arrived = val.glerp_qucy(0.0, 0.5, 0.05, 0.0, 1.0);  // snapped
```

**Step Interpolation**

```rust
use deki::lerp::Stepable;
let mut val = 0.0f32;
let arrived = val.sterp(10.0, 2.0);  // moves 2.0 per call, true when arrived
```

**Cyclic Step Interpolation**

```rust
use deki::lerp::CycleStapable;
let mut val = 0.0f32;
let arrived = val.sterp_qucy(0.9, 0.1, 0.0, 1.0);  // cyclic, moves 0.1 per call
```

**Easing**

```rust
let t = 0.5f32;
let eased = t.smooth();  // smoothstep: t*t*(3-2*t)
```

#### Cycling Math

**`add_qucy` / `sub_qucy` — Modular Arithmetic**

Fast modular arithmetic (assumes current value is in range):

```rust
// Circular buffer: advance index, wrap to 0 at capacity
let idx: usize = 99usize.add_qucy(1, 0, 100);  // 0

// Move backwards through a 10-slot inventory, wrapping to the end
let slot: i32 = 2i32.sub_qucy(3, 0, 10);  // 9
```

**`mul_f32` — Multiply Integer by f32 (rounded)**

```rust
let count: i32 = 5.mul_f32(2.5);  // 13 (rounded)
```

#### `#[derive(Cycle)]` — Auto-Cycling Enums

Generates `cycle_next()` and `cycle_prev()` for unit-variant enums:

```rust
use deki_macros::Cycle;

#[derive(Cycle)]
enum Direction { North, East, South, West }

let mut dir = Direction::North;
dir = dir.cycle_next();  // East
dir = dir.cycle_prev();  // North
```

---

### Randomness (`random` feature)

```rust
use deki::random::*;

let val: f32 = f32r(0.0..1.0);
let my_vec = vec![1, 2, 3];
let random_item = my_vec.random();  // picks a random element
```

---

### Helper Macros

#### `qonst!` — Named Constants

Creates a `pub const` named after the type, without requiring `Default`:

```rust
qonst!(Vec3: x: 1.0, y: 2.0, z: 3.0);
// => pub const VEC3: Vec3 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };

qonst!(Direction::North);
// => pub const DIRECTION: Direction = Direction::North;
```

#### `trait_alias!` — Trait Aliases

```rust
deki_core::trait_alias!(MyTrait: Clone + Send + Sync);
// => pub trait MyTrait: Clone + Send + Sync {}
// => impl<C: Clone + Send + Sync> MyTrait for C {}
```

#### `default!` — Shorthand Default

```rust
deki_core::default!(MyStruct = Self { a: 0, b: Default::default() });
// => impl Default for MyStruct { fn default() -> Self { Self { a: 0, b: Default::default() } } }
```

---

## deki_macros — Proc Macros

### Derive Macros

#### `#[derive(Cycle)]`

Generates `cycle_next()` and `cycle_prev()` for unit-variant enums.

```rust
#[derive(Cycle)]
enum Color { Red, Green, Blue }
```

#### `#[derive(ForceDefault)]`

Generates `Default` by calling `.default()` on each field:

```rust
#[derive(ForceDefault)]
struct Config { timeout: u32, name: String }
// => Default produces Config { timeout: 0, name: String::default() }
```

#### `#[derive(EnumFieldCount)]`

Generates `fn field_count(&self) -> usize` for enums:

```rust
use deki_macros::EnumFieldCount;

#[derive(EnumFieldCount)]
enum Color { Red, Green(Rgb), Blue(u8, u8, u8) }

assert_eq!(Color::Red.field_count(), 0);
assert_eq!(Color::Green(Rgb { r: 0, g: 0, b: 0 }).field_count(), 1);
assert_eq!(Color::Blue(0, 0, 0).field_count(), 3);
```

---

### Procedural Macros

#### `xoxo!` — Bool Pattern Matching

```rust
deki_macros::xoxo!{match [true, false, true] {
    [O, O, O] => "all false",
    [X, O, X] => "first and last true",
    [_, _, _] => "default",
}}
```

#### `quimp!` — Quick Trait Implementation

For traits with a single required method:

```rust
dekimacros::quimp!{MyWrapper
    fn clone(&self) -> Self { Self::new(self.0.clone()) };
    fn default() -> Self { Self::new(42) };
}
```

#### `match_fns!` — Declarative Enum Methods

```rust
enum Object { RedSphere, GreenCube }

deki_macros::match_fns!{
    [Object]
    shape() -> &'static str;
    color(brightness: f32) -> &'static str;

    [::RedSphere]
    shape: "sphere";
    color: if brightness > 0.5 { "bright-red" } else { "red" };

    [::GreenCube]
    shape: "cube";
    color: "just-green";
}
```

#### `derive_from!` — Generate `From` Impl

Generate `impl From<A> for B` from function signatures:

```rust
struct Wrapper(i32);
impl Wrapper {
    fn new(v: i32) -> Self { Self(v) }
    fn from_str(s: &str) -> Self { Self(s.len() as i32) }
}

deki_macros::derive_from!{
    Wrapper
    i32 -> new;
    String -> from_str;
}

let w: Wrapper = (42).into();
assert_eq!(w.0, 42);
```

#### `derive_math!` — Generate Arithmetic Impl

Generate `impl Add/Sub/Mul/Div<A> for T` from function signatures:

```rust
#[derive(Debug, PartialEq)]
struct Vec2 { x: f32, y: f32 }

deki_macros::derive_math!{
    Vec2
    Add: Vec2 -> Vec2 -> self.x + rhs.x, self.y + rhs.y -> Vec2;
    Add: f32 -> f32 -> self.x + rhs, self.y + rhs -> Vec2;
    Mul: f32 -> f32 -> self.x * rhs, self.y * rhs -> Vec2;
}

let a = Vec2 { x: 1.0, y: 2.0 };
let b = Vec2 { x: 3.0, y: 4.0 };
assert!((a + b).x - 4.0 < 1e-5);
```

#### `foname!` — Name Mangling

Converts identifiers to different cases:

```rust
deki_macros::foname!{ myFunctionName@snake }   // my_function_name
deki_macros::foname!{ my_function_name@camel }  // myFunctionName
deki_macros::foname!{ my_function_name@scream } // MY_FUNCTION_NAME
deki_macros::foname!{ my_function_name@flat }   // myfunctionname
deki_macros::foname!{ my_function_name@upper }  // MYFUNCTIONNAME
```

---

### Attribute Macros

#### `#[imp(...)]` — Attach Methods to Types or Impl Blocks

**Function-level syntax:**

```rust
#[imp(MyStruct)]
fn new(value: i32) -> Self { Self { value } }

#[imp(MyStruct|MyTrait)]
fn clone(&self) -> Self { Self::new(self.value) }
```

**Foreign types (auto-generates a trait):**

```rust
#[imp(String|*)]
fn trim_all(&self) -> Self { self.trim().to_string() }
// Generates: trait StringTrimAllExt { fn trim_all(&self) -> Self; }
// and impls it for String
```

**Impl block syntax:**

```rust
#[imp(TraitName)]
impl String {
    fn greet(&self) -> Self { self.clone() }
}

#[imp(*NewTraitName)]
impl String {
    fn new_method(&self) -> Self { self.clone() }
}

#[imp(*)]
impl String {
    fn auto_method(&self) -> Self { self.clone() }
}
// Auto-generates: StringAutoMethodExt
```

#### `#[derived(...)]` — Batch Apply Derives

Batch-apply derive macros by name:

```rust
use deki_macros::derived;

#[derived(_Hashable)]
struct Point { x: i32, y: i32 }
// => #[derive(PartialEq, Eq, Hash, Clone, Copy)]
```

---

## License

Dual licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE).
