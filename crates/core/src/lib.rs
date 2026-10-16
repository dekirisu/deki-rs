use std::ops::{Add, Range, RangeInclusive, Rem, Sub};
use crossbeam::channel as beam;
pub use buns;
use deki_macros::imp;
pub use maflow::*;
pub use paste;
pub use type_cell::*;

#[cfg(feature = "derive_more")]
pub use derive_more;

#[cfg(feature = "lerp")]
pub mod lerp;

#[cfg(feature="approx")]
pub mod math;

pub mod collections;


// Renames \\

    pub use std::marker::PhantomData as Ghost;
    pub use derive_new::new as New;
    pub use extension_traits::extension as ext;
    pub use buns::compose;
    pub type Str = &'static str;


// Cycle Trait \\

    /// Enable cycling through unit-variant enum variants with wrapping.
    pub trait Cycle {
        /// Advance to next variant, wrapping to first.
        ///
        /// # Example
        /// ```
        /// use deki_core::Cycle;
        /// #[derive(::deki_macros::Cycle, PartialEq, Debug)]
        /// enum D {A, B, C}
        /// assert_eq!(D::A.cycle_next(), D::B);
        /// assert_eq!(D::C.cycle_next(), D::A); // wraps
        /// ```
        fn cycle_next(&self) -> Self;

        /// Move to prev variant, wrapping to last.
        ///
        /// # Example
        /// ```
        /// use deki_core::Cycle;
        /// #[derive(::deki_macros::Cycle, PartialEq, Debug)]
        /// enum D {A, B, C}
        /// assert_eq!(D::B.cycle_prev(), D::A);
        /// assert_eq!(D::A.cycle_prev(), D::C); // wraps
        /// ```
        fn cycle_prev(&self) -> Self;
    }


// Traits \\

    /// Enable marking a type as `'static + Send + Sync` with a single bound.
    pub trait Syncable:'static+Send+Sync {}
    impl <T:'static+Send+Sync> Syncable for T {}

    /// Add `.clear()` to any `Default` type.
    pub trait DefaultClear: Default {
        /// Reset to the default value.
        ///
        /// # Example
        /// ```
        /// use deki_core::DefaultClear;
        /// #[derive(Default)]
        /// struct State { x: i32, y: String }
        ///
        /// let mut s = State { x: 42, y: "hello".into() };
        /// s.clear();
        /// assert_eq!(s.x, 0);
        /// assert!(s.y.is_empty());
        /// ```
        fn clear(&mut self){*self = Self::default();}
    }
    impl <A:Default> DefaultClear for A {}

    /// Enable collecting any `Iterator` into a `Vec`.
    pub trait CollectShort: Iterator {
        fn into_vec(self) -> Vec<Self::Item>;
    }
    impl <A:Iterator> CollectShort for A {
        fn into_vec(self) -> Vec<Self::Item> { self.collect() }
    }


// Extensions \\

    /// Add a value to both bounds of the range.
    ///
    /// # Example
    /// ```
    /// use deki_core::RangeOffset;
    /// let r = (0..=10).offset(5);
    /// assert_eq!(r, 5..=15);
    /// ```
    #[imp(RangeInclusive<Idx:Clone+Add<Output=Idx>>|*RangeOffset)]
    fn offset(&self,rhs:Idx) -> Self {
        self.start().clone()+rhs.clone()
        ..=self.end().clone()+rhs.clone()
    }

    /// Add a value to both bounds of the range.
    ///
    /// # Example
    /// ```
    /// use deki_core::RangeOffset;
    /// let r = (0..10).offset(5);
    /// assert_eq!(r, 5..15);
    /// ```
    #[imp(Range<Idx:Clone+Add<Output=Idx>>|RangeOffset<Idx>)]
    fn offset(&self,rhs:Idx) -> Self {
        self.start.clone()+rhs.clone()..self.end.clone()+rhs.clone()
    }


// Cycling Addition \\

    compose!{paste!{
        #[inline]
        pub fn [<quick_cycle_ ^0>]<A>(cur:A,rhs:A,min:A,max:A) -> A 
        where A: Clone
            + Add<Output=A>
            + Sub<Output=A>
            + Rem<Output=A>
        {
            let len = max.sub(min.clone());
            let bas = cur.sub(min.clone()).add(len.clone());
            let unc = bas.^0(rhs.rem(len.clone()));
            min.add(unc.rem(len))
        }
    } #sub #add }

    /// Perform cycling addition/subtraction for numeric types within a range.
    pub trait CycleMath {
        /// Add within [min, max), wrapping around.
        ///
        /// # Example
        /// ```
        /// use deki_core::CycleMath;
        /// assert_eq!(8i32.add_qucy(5, 0, 10), 3);  // wraps
        /// assert_eq!(5i32.add_qucy(3, 0, 10), 8);  // normal
        /// ```
        fn add_qucy(self,rhs:Self,min:Self,max:Self) -> Self;
        /// Subtract within [min, max), wrapping around.
        ///
        /// # Example
        /// ```
        /// use deki_core::CycleMath;
        /// assert_eq!(2i32.sub_qucy(5, 0, 10), 7);  // wraps
        /// assert_eq!(5i32.sub_qucy(3, 0, 10), 2);  // normal
        /// ```
        fn sub_qucy(self,rhs:Self,min:Self,max:Self) -> Self;
    }
    
    compose!{
        impl CycleMath for ^0 {
            #[inline]
            fn add_qucy(self,rhs:Self,min:Self,max:Self) -> Self {
                quick_cycle_add(self,rhs,min,max)
            }
            #[inline]
            fn sub_qucy(self,rhs:Self,min:Self,max:Self) -> Self {
                quick_cycle_sub(self,rhs,min,max)
            }
        }
        #f32 #f64
        #i8 #i16 #i32 #i64 #i128 #isize
        #u8 #u16 #u32 #u64 #u128 #usize
    }


// Combined f32 Multiplication \\

    /// Multiply by an f32 and round to the nearest integer.
    #[inline]
    #[imp(f32 | *MulF32)]
    fn mul_f32(self,rhs:f32) -> Self {self * rhs}

    #[inline]
    #[imp(f64 | MulF32)]
    fn mul_f32(self,rhs:f32) -> Self {self * rhs as f64}
 
    compose!{
        impl MulF32 for ^0 {
            #[inline]
            fn mul_f32(self,rhs:f32) -> Self {
                (self as f32 * rhs).round() as ^0 
            }
        }
        #i8 #i16 #i32 #i64 #i128 #isize
    }


// Randomness \\

    /// Random number generation via fastrand.
    #[cfg(feature="random")]
    pub mod random {
        use std::ops::Range;
        pub use fastrand::*;

        /// Generate a random f32 within the given range, inclusive of both endpoints.
        #[inline]
        pub fn f32r(range:Range<f32>) -> f32 {
            range.start + f32() * (range.end - range.start)
        }    

    }

    /// Return a random element from the vector.
    #[cfg(feature="random")]
    #[inline]
    #[imp(Vec<T> | *)]
    fn random(&self) -> &T {
        exit![>if (self.len()==1) &self[0]];
        &self[random::usize(0..self.len())]
    }


// Vec Extensions \\

    /// Return a reference to the element at `idx`, wrapping around.
    #[inline]
    #[imp(Vec<T> | *)]
    fn get_cycling(&self, idx: usize) -> &T {
        &self[idx % self.len()]
    }


// Macros \\

    /// Declare a pub const named after its type; no Default required.
    ///
    /// # Example
    /// ```
    /// use deki_core::{qonst,paste};
    ///
    /// struct Point { x: f32, y: f32 }
    /// qonst!(Point: x: 1.0, y: 2.0);
    /// assert_eq!(POINT.x, 1.0);
    ///
    /// #[derive(PartialEq, Debug)]
    /// enum Dir { North, South }
    /// qonst!(Dir::North);
    /// assert_eq!(DIR, Dir::North);
    /// ```
    #[macro_export]
    macro_rules! qonst {
        ($ty:ty: $($tt:tt)*) => {paste!{
            pub const [<$ty:snake:upper>]: $ty = $ty {$($tt)*};
        }};
        ($ty:ident::$($tt:tt)*) => {paste!{
            pub const [<$ty:snake:upper>]: $ty = $ty::$($tt)*;
        }};
    }

    /// Define a trait alias with a blanket impl for any type satisfying the bounds.
    ///
    /// # Example
    /// ```
    /// deki_core::trait_alias!(CloneSend: Clone + Send);
    /// // CloneSend is now a trait alias for Clone + Send
    /// // Any type that implements Clone + Send automatically implements CloneSend
    /// fn takes_clone_send<T: CloneSend>(_t: T) {}
    /// takes_clone_send(42i32);
    /// ```
    #[macro_export]
    macro_rules! trait_alias {($trait:ident:$($tt:tt)*)=>{
        pub trait $trait: $($tt)* {}
        impl <C:$($tt)*> $trait for C {}
    }}

    /// Implement `Default` inline without a separate `impl` block.
    ///
    /// # Example
    /// ```
    /// struct Config { timeout: u32, name: String }
    /// deki_core::default!(Config = Self { timeout: 30, name: String::new() });
    /// let cfg = Config::default();
    /// assert_eq!(cfg.timeout, 30);
    /// ```
    #[macro_export]
    macro_rules! default {($name:ty = $($tt:tt)*) => {
        impl Default for $name {
            fn default() -> Self {$($tt)*}
        }
    }}

    /// Ternary operator — any type.
    ///
    /// # Example
    /// ```
    /// use deki_core::choose;
    /// let x: i32 = choose!(true, 1, 2);
    /// assert_eq!(x, 1);
    /// let y: &str = choose!(false, "yes", "no");
    /// assert_eq!(y, "no");
    /// ```
    #[macro_export]
    macro_rules! choose {($cond:expr, $then:expr, $else:expr) => {{
        if $cond { $then } else { $else }
    }}}



// DekiNote — Static Notification Channel \\

    /// A log/event note: `level`, `source` (caller crate), and formatted `message`.
    #[derive(New)]
    pub struct DekiNote {
        pub level: &'static str,
        pub source: &'static str,
        pub message: String 
    }

    /// Global unbounded channel shared across all crates.
    static DEKI_NOTIFY: Lazy<(beam::Sender<DekiNote>,beam::Receiver<DekiNote>)> = Lazy::new(||beam::unbounded());

    /// Drain the channel, returning all pending notes.
    #[imp(DekiNote)]
    pub fn read() -> Vec<Self> {
        let mut out = vec![];
        while let Ok(msg) = DEKI_NOTIFY.1.try_recv() {
            out.push(msg);
        }
        out
    }

    /// Drain and filter pending notes.
    #[imp(DekiNote)]
    #[inline]
    pub fn read_filter<F:FnMut(&Self)->bool>(filter:F) -> Vec<Self> {
        Self::read().into_iter().filter(filter).into_vec()
    }

    /// Format a note as `[level][source]message`.
    #[imp(DekiNote)]
    #[inline]
    pub fn print_row(&self) -> String {
        let Self { level, source, message } = self;
        format!("[{level}][{source}]{message}")
    }

    /// Send this note into the global channel.
    #[imp(DekiNote)]
    #[inline] 
    pub fn write(self){
        DEKI_NOTIFY.0.send(self).unwrap();
    }

    /// Log at `info` level (gated by `note-info` feature).
    #[imp(DekiNote)]
    #[inline]
    #[allow(unused_variables)]
    pub fn info(source:&'static str,message:String){
        #[cfg(feature = "note-info")]
        DekiNote::new("info", source, message).write();
    }

    /// Log at info level using `NOTE_SOURCE` from the caller.
    #[macro_export]
    macro_rules! note_info { ($($any:tt)*) => {
        DekiNote::info(NOTE_SOURCE, format!($($any)*))
    }}

    /// Log at `debug` level (gated by `note-debug` feature).
    #[imp(DekiNote)]
    #[inline]
    #[allow(unused_variables)]
    pub fn debug(source:&'static str,message:String){
        #[cfg(feature = "note-debug")]
        DekiNote::new("debg", source, message).write();
    }

    /// Log at debug level using `NOTE_SOURCE` from the caller.
    #[macro_export]
    macro_rules! note_debug { ($($any:tt)*) => {
        DekiNote::debug(NOTE_SOURCE, format!($($any)*))
    }}

    /// Log at `warn` level (gated by `note-warning` feature).
    #[imp(DekiNote)]
    #[inline]
    #[allow(unused_variables)]
    pub fn warn(source:&'static str,message:String){
        #[cfg(feature = "note-warning")]
        DekiNote::new("warn", source, message).write();
    }

    /// Log at warn level using `NOTE_SOURCE` from the caller.
    #[macro_export]
    macro_rules! note_warn { ($($any:tt)*) => {
        DekiNote::warn(NOTE_SOURCE, format!($($any)*))
    }}

    /// Log at `error` level (gated by `note-error` feature).
    #[imp(DekiNote)]
    #[inline]
    #[allow(unused_variables)]
    pub fn error(source:&'static str,message:String){
        #[cfg(feature = "note-error")]
        DekiNote::new("erro", source, message).write();
    }

    /// Log at error level using `NOTE_SOURCE` from the caller.
    #[macro_export]
    macro_rules! note_error { ($($any:tt)*) => {
        DekiNote::error(NOTE_SOURCE, format!($($any)*))
    }}

    /// Log at `critical` level (gated by `note-critical` feature).
    #[imp(DekiNote)]
    #[inline]
    #[allow(unused_variables)]
    pub fn crit(source:&'static str,message:String){
        #[cfg(feature = "note-critical")]
        DekiNote::new("crit", source, message).write();
    }

    /// Log at critical level using `NOTE_SOURCE` from the caller.
    #[macro_export]
    macro_rules! note_crit { ($($any:tt)*) => {
        DekiNote::crit(NOTE_SOURCE, format!($($any)*))
    }}



// EOF \\
