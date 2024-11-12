<h1 align="center">🦀 My Rusty Base 🦀</h1>
<p align="center">
    <a href="https://github.com/dekirisu/deki-rs" style="position:relative"><img src="https://img.shields.io/badge/github-dekirisu/deki-ee6677"></a>
    <a href="https://crates.io/crates/deki" style="position:relative"><img src="https://img.shields.io/crates/v/deki"></a>
</p>

## What is this?
- A collection of crates, functions and renames I tend to use and rate as beeing 'general purpose'.
- A base where I don't have to look up how things works.

> [!NOTE]
> This mainly exists so I can depend on it quick, easy and anywhere.

> [!IMPORTANT]
> Since this crate doesn't have a specific purpose, it may change a lot between 'minor' versions.
> That said, I'll follow semantic versioning of course! ✨

## What does it do?
- add `PhantomData` alias `Ghost`
- re-export [maflow](https://github.com/dekirisu/maflow): all `*`
- re-export [type_cell](https://github.com/dekirisu/type_cell): all `*`
- re-export [buns](https://github.com/dekirisu/buns): `self` and `sandwich`
- re-export [derive-new](https://github.com/nrc/derive-new): `new` as `Constructor`
    - e.g. `#[derive(Constructor)] struct AStruct{field:u32}` -> `AStruct::new(2);`
- re-export [extension-traits](https://github.com/danielhenrymantilla/ext-trait.rs): `extension` as `ext`
    - e.g. `#[ext(trait F32Ext)] impl f32 {fn plus(self,rhs:f32)->f32{self+rhs}}` -> `4.0.plus(4.0);`
- re-export [derive_more](https://github.com/JelteF/derive_more): `self` as `derive_more` and `drv`
    - e.g. `#[derive(drv::Deref)] struct AStruct(#[deref]u32);`
- derive presets, by using [derive_preset](https://github.com/dekirisu/derive_preset):
    - `#[hashable(..)]` = `#[derive(PartialEq,Eq,Hash,Clone,Copy,..)]`
    - `#[serde(..)]` = `#[derive(Serialize,Deserialize,Clone,..)]`
    - `#[serde_hash(..)]` = `#[derive(Serialize,Deserialize,PartialEq,Eq,Hash,Clone,Copy,..)]`
    - `#[deref(..)]` = `#[derive(drv::Deref,drv::DerefMut,..)]`
    - Note: Assuming any `Hash` derivator is small and therefore fine to be copied!
- auto-impl trait marker `Syncable` for anything implementing `'static+Send+Sync`
    - mainly used as 'rename' to use in trait bounds
- auto-impl trait `DefaultClear` for anything implementing `Default`, id adds `.clear()` to set it back to default
- auto-impl trait `Lerpable` for any type with necessary maths to perform linear interpolation
    - e.g. `3.0.lerp(4.0,0.1)` or any future type you impl maths for
- auto-impl trait `LerpableDeref` for any type that derefs to a type with necessary maths to perform linear interpolation
    - e.g. `#[deref] struct AStruct(#[deref]f32);` -> `AStruct(3.0).lerp(AStruct(4.0),0.1)`
- extend `f32` by `.smooth()` to apply cheap ease-in and -out (smooth-step) if within 0..=1
- extend `f32` by `.clamp_unit()` = `.clamp(0.0,1.0)`
- extend `Ramge<T>` & `RangeInclusive<T>` by `.add(T)` to offset it
- macro `qonst!` (quick const): set a type and a value, name is automatically set to the type name in uppercase
- (optional) re-export [fastrand](https://github.com/smol-rs/fastrand): `self` as `random`
    - extend `Vec` by `.random()`
- (optional) re-export [fastapprox](https://github.com/loony-bean/fastapprox-rs): all `*` (modified) as `approx`
    - extend `f32` by `.{operation}_ca` (ca = circa (latin))

## Synergies
A struct with `PhantomData`:
```rust
#[derive(Constructor)]
pub struct AStruct<T>(u32,#[new(default)]Ghost<T>)
// Construct somewhere:
AStruct::<String>::new(3);
```
Quick smooth interpolation of a struct with a f32:
```rust
#[deref(Constructor,Clone)]
struct AStruct {a:u32,#[deref]b:f32}
// run:
let from = AStruct::new(0,0.);
let to = AStruct::new(0,0.);
from.lerp(to,progress.smooth())
```
