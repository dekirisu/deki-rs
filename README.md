# deki-rs

[![GitHub](https://img.shields.io/badge/github-dekirisu/deki-ee6677)](https://github.com/dekirisu/deki-rs/)
[![crates.io](https://img.shields.io/crates/v/deki_macros)](https://crates.io/crates/deki_macros)

A personal Rust utility crate — a curated bundle of helper types, traits, macros, and re-exports that change how you write Rust. Less boilerplate, more flow.

---

## What changes when you use `deki`?

| Without `deki` | With `deki` |
|---|---|
| `std::marker::PhantomData` | `Ghost` |
| `&'static str` | `Str` |
| Manual enum cycling | `#[derive(Cycle)]` + `.cycle_next()` / `.cycle_prev()` |
| Manual lerp boilerplate | `.lerp()`, `.glerp()`, `.lerp_qucy()`, `.sterp()` |
| Approximate math calls | `x.exp_fast()`, `x.pow_fast()`, etc. |
| `impl Default for T { fn default() -> Self { Self { a: Default::default(), b: Default::default() } } }` | `ForceDefault` derive |
| `#[derive(Debug, Clone, PartialEq, Eq, Hash)]` | `hashable` preset |

---

## Features

| Feature | Default | What it adds |
|---|---|---|
| `random` | ✅ | `fastrand` re-exports, `f32r()`, `Vec::random()` |
| `approx` | ✅ | Fast bit-hack math via `DekiExtApprox` — `sqrt_fast`, `exp_fast`, `pow_fast`, `log2_fast` |
| `lerp` | ✅ | Linear / gated / cyclic / step interpolation traits and methods |
| `proc` | — | Proc-macro support (string→ident, token stream helpers) |

```toml
[dependencies]
deki = { version = "0.3", default-features = false, features = ["random", "lerp"] }
```

---

## deki_core — Runtime Utilities

### Aliases

Shorter names for common types and crates:

| Name | Is |
|---|---|
| `Ghost` | `PhantomData` |
| `Str` | `&'static str` |
| `Constructor` | `derive_new::new` |
| `ext` | `extension_traits::extension` |

| `compose` | `buns::compose` |

Plus re-exports of `buns`, `type_cell`, and `maflow`.

### `Syncable` — `'static + Send + Sync` in One Word

Use as a bound instead of writing `'static + Send + Sync` everywhere:

```rust
fn spawn<T: Syncable>(val: T) { ... }
```

### `DefaultClear` — Clear Any Default Type

Adds `.clear()` to any `Default` type:

```rust
let mut buf = Vec::new();
buf.clear();  // standard
let mut map = HashMap::new();
map.clear();  // standard

// But also works for types where you just want to reset to default
let mut state = MyState::default();
state.clear();  // resets to MyState::default()
```

### `StackMap` — Insertion-Ordered Key-Value Map

A map that preserves insertion order and allows duplicate keys:

```rust
let mut map = StackMap::<&str, i32>::new();
*map.entry("count") = 42;
assert_eq!(*map.entry("count"), 42);

for (key, value) in map.iter() { ... } // insertion order
```

Keys are stored in a `Vec` — linear search, but preserves order and allows duplicates.

### Cycling Math

#### `#[derive(Cycle)]` — Auto-Cycling Enums

Generates `cycle_next()` and `cycle_prev()` for enums:

```rust
#[derive(Cycle)]
enum Direction { North, East, South, West }

let mut dir = Direction::North;
dir = dir.cycle_next();  // East
dir = dir.cycle_prev();  // North
```

#### `add_qucy` / `sub_qucy` — Modular Arithmetic

Fast modular arithmetic on any number type (assumes current value is in range):

```rust
// Circular buffer: advance index, wrap to 0 at capacity
let idx: usize = 99usize.add_qucy(1, 0, 100);  // 0

// Move backwards through a 10-slot inventory, wrapping to the end
let slot: i32 = 2i32.sub_qucy(3, 0, 10);  // 9
```

#### `mul_f32` — Multiply Integer by f32

```rust
let count: i32 = 5.mul_f32(2.5);  // 13 (rounded)
```

### Interpolation (`lerp` feature)

#### Linear Interpolation

```rust
use deki::lerp::*;

let val: f32 = 0.0.lerp(10.0, 0.5);       // 5.0
let val: i32 = 0i32.lerp(100, 0.3);       // generic
```

#### Gated Lerp — Snap When Close

```rust
let mut val = 0.0f32;
let arrived = val.glerp(10.0, 0.1, 0.5);  // snaps when within 0.5 of target
```

#### Cyclic Lerp — Auto-Choose Shortest Direction

```rust
use deki::lerp::Clerpable;

let val: f32 = 0.0.lerp_qucy(0.9, 0.1, 0.0, 1.0);
// Result: 0.9 (not -0.1) — picks the shorter path
```

#### Step Interpolation

```rust
let mut val = 0.0f32;
let arrived = val.sterp(10.0, 2.0);  // moves 2.0 per call, true when arrived
```

### Approximate Math (`approx` feature)

Faster but less precise than standard math (~1% error, bit-hack based):

```rust
let x = 1.5f32;
use deki_core::math::DekiExtApprox;

x.exp_fast();      // exp(x), ~1% error, 2.2× faster than std
x.pow_fast(3.0);   // pow(x, 3.0), ~1% error, 1.5× faster than std
x.log2_fast();     // log2(x), ~1% error
x.sqrt_fast();     // sqrt(x), ~0.1% error, 1.4× faster than std
```

### Randomness (`random` feature)

```rust
use deki::random::*;

let val: f32 = f32r(0.0..1.0);
let random_item = my_vec.random();
```

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
trait_alias!(MyTrait: Clone + Send + Sync);
// => pub trait MyTrait: Clone + Send + Sync {}
// => impl<C: Clone + Send + Sync> MyTrait for C {}
```

#### `default!` — Shorthand Default

```rust
default!(MyStruct = Self { a: 0, b: Default::default() });
// => impl Default for MyStruct { fn default() -> Self { Self { a: 0, b: Default::default() } } }
```

### Easing

```rust
let t = 0.5f32;
let eased = t.smooth();  // smoothstep: t*t*(3-2*t)
```

---

## deki_macros — Proc Macros

### Derive Macros

#### `#[derive(Cycle)]`

Generates `cycle_next()` and `cycle_prev()` for enums. Works on unit variants only.

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

### Preset Derives (via `derive_preset`)

```rust
derive_preset::create!{
    hashable    "PartialEq,Eq,Hash,Clone,Copy"
    serde       "Serialize,Deserialize,Clone"
    serde_hash  "Serialize,Deserialize,PartialEq,Eq,Hash,Clone,Copy"
    deref       "Deref,DerefMut"
}
```

### `xoxo!` — Bool Pattern Matching

```rust
xoxo!{match [true, false, true] {
    [O, O, O] => "all false",
    [X, O, X] => "first and last true",
    [_, _, _] => "default",
}}
```

### `quimp!` — Quick Trait Implementation

For traits with a single required method:

```rust
quimp!{MyWrapper
    fn clone(&self) -> Self { Self::new(self.0.clone()) };
    fn default() -> Self { Self::new(42) };
}
```

### `#[imp(...)]` — Attach Methods to Types

```rust
#[imp(MyStruct)]
fn new(value: i32) -> Self { Self { value } }

#[imp(MyStruct|Clone)]
fn clone(&self) -> Self { Self::new(self.value) }
```

For foreign types, use `*` to auto-generate a trait:

```rust
#[imp(String|*)]
fn trim_all(&self) -> Self { self.trim().to_string() }
// Generates: trait StringTrimAllExt { fn trim_all(&self) -> Self; }
// and impls it for String
```

### `match_fns!` — Declarative Enum Methods

```rust
enum Object { RedSphere, GreenCube }

match_fns!{
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

### `foname!` — Name Mangling

Converts identifiers to different cases:

```rust
foname!{ my_function_name@snake }   // my_function_name
foname!{ my_function_name@camel }   // myFunctionName
foname!{ my_function_name@scream }  // MY_FUNCTION_NAME
foname!{ my_function_name@flat }    // myfunctionname
foname!{ my_function_name@upper }   // MYFUNCTIONNAME
```

---

## Commit Conventions

Commit messages use animal emojis to denote change type — yes, really. It's just for fun.

| Emoji | Animal | Meaning |
|---|---|---|
| 🐤 | chick | **Add** — new code, features, macros, traits |
| 🐋 | whale | **Dependency / version** — version bumps, optional deps, semver |
| 🐍 | snake | **Functional refactor** — changes behavior or signatures |
| 🦎 | lizard | **Structural refactor** — code moves, reorg, no behavior change |
| 🦉 | owl | **Docs** — README, documentation |
| 🐞 | bug | **Fix** — bug fixes |
| 🐇 | rabbit | **Tests** — adding tests |
| 🐣 | chickling | **Birth** — new crate |
| 🐝 | bee | **Merge** — merge code between crates |
| 🐜 | ant | **Cleanup** — tidying, removing dead code |

---

## License

Dual licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE).
