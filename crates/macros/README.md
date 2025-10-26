<h1 align="center">My Macros</h1>
<p align="center">
    <a href="https://github.com/dekirisu/deki-rs" style="position:relative"><img src="https://img.shields.io/badge/github-dekirisu/deki-ee6677"></a>
    <a href="https://crates.io/crates/deki_macros" style="position:relative"><img src="https://img.shields.io/crates/v/deki_macros"></a>
</p>

Various macros I made and liked to use occasionally!

## Attach functionality to type variants
Basically write this:
```rust
 enum Object {RedSphere, GreenCube}
 match_fns!{

     // 1. Define Methods - '&self' is assumed as parameter
     [Object]
     shape() -> &'static str;
     color(brightness:f32) -> &'static str;

     // 2. Add Code
     [::RedSphere]
     shape: "sphere";
     color: if brightness > 0.5 {"bright-red"} else {"red"};

     [::GreenCube]
     shape: "cube";
     color: "just-green";

 }
 ```
Instead of:
```rust
impl Object {
    pub fn shape(&self) -> &'static str {
        match self {
            Color::RedSphere => "sphere",
            Color::GreenCube => "cube",
        }
    }
    pub fn color(&self, brightness: f32) -> &'static str {
        match self {
            Color::RedSphere => {
                if brightness > 0.5 {"bright-red"} else {"red"}
            }
            Color::GreenCube => "just-green",
        }
    }
}
```

## Quick Implementations
Alternative syntax for implementing trait with ONE required method.
 ```rust
 quimp!{StructName
    fn clone(&self) -> Self {Self::new(self.0)};
    fn default() -> Self {Self::new(100)};
 }
 ```

Or using this:
 - `#[imp(Struct)]`: for a owned Type
 - `#[imp(Struct|Trait)]`: to impl a singe-method trait
 - `#[imp(Struct|*)]`: for a foreign Type (generates a new trait)
```rust
#[imp(Struct)]
fn function(){}
```

## Bool Aliases
Simple macro that replaces `X -> true` and `O -> false`:
 ```rust
xoxo!{match [true,false,true] {
     [O,O,O] => "nope",
     [O,O,X] => "nope",
     [X,O,X] => "YEP!",
     [_,_,_] => "nope"
 }}
 ```

