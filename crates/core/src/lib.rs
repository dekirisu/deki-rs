use std::ops::{Add, Range, RangeInclusive, Rem, Sub};

pub use buns;
pub use maflow::*;
pub use paste;
pub use type_cell::*;

#[cfg(feature = "derive_more")]
pub use derive_more;

#[cfg(feature = "lerp")]
pub mod lerp;

pub mod collections;


// Renames \\

    pub use std::marker::PhantomData as Ghost;
    pub use derive_new::new as Constructor;
    pub use extension_traits::extension as ext;
    pub use buns::compose;
    pub type Str = &'static str;


// Cycle Trait \\

    /// Cycle through unit-variant enum variants with wrapping.
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

    /// Mark a type as `'static + Send + Sync` with a single bound.
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


// Extensions \\

    /// Add a value to both bounds of a range.
    #[ext(pub trait RangeOffset)]
    impl <Idx:Clone+Add<Output=Idx>> RangeInclusive<Idx> {
        /// Add a value to both bounds of the range.
        ///
        /// # Example
        /// ```
        /// use deki_core::RangeOffset;
        /// let r = (0..=10).offset(5);
        /// assert_eq!(r, 5..=15);
        /// ```
        fn offset(&self,rhs:Idx) -> Self {
            self.start().clone()+rhs.clone()
            ..=self.end().clone()+rhs.clone()
        }
    }

    impl <Idx:Clone+Add<Output=Idx>> RangeOffset<Idx> for Range<Idx> {
        /// Add a value to both bounds of the range.
        ///
        /// # Example
        /// ```
        /// use deki_core::RangeOffset;
        /// let r = (0..10).offset(5);
        /// assert_eq!(r, 5..15);
        /// ```
        fn offset(&self,rhs:Idx) -> Self {
            self.start.clone()+rhs.clone()..self.end.clone()+rhs.clone()
        }
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

    /// Cycling addition/subtraction for numeric types within a range.
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

    #[ext(pub trait MulF32)]
    impl f32 {
        #[inline]
        /// Multiply by an f32 and round to the nearest integer.
        fn mul_f32(self,rhs:f32) -> Self {self * rhs}
    }

    impl MulF32 for f64 {
        #[inline]
        fn mul_f32(self,rhs:f32) -> Self {self * rhs as f64}
    }

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

    /// Fast randomness utilities powered by fastrand.
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

    #[cfg(feature="random")]
    #[ext(pub trait DekiExtVecRng)]
    impl <T> Vec<T> {
        /// Return a random element from the vector.
        #[inline]
        fn random(&self) -> &T {
            exit!{>if (self.len()==1) &self[0]}
            &self[random::usize(0..self.len())]
        }
    }


// Approx Math \\

    #[cfg(feature="approx")]
    #[ext(pub trait DekiExtApprox)]
    impl f32 {
        /// Approximate sine using fastapprox, trading precision for speed.
        #[inline]
        fn sin_ca(self) -> f32 {approx::sin(self)}

        /// Approximate cosine using fastapprox, trading precision for speed.
        #[inline]
        fn cos_ca(self) -> f32 {approx::cos(self)}

        /// Approximate tangent using fastapprox, trading precision for speed.
        #[inline]
        fn tan_ca(self) -> f32 {approx::tan(self)}
        
        /// Approximate exponential using fastapprox, trading precision for speed.
        #[inline]
        fn exp_ca(self) -> f32 {approx::exp(self)}

        /// Approximate logarithm using fastapprox, trading precision for speed.
        #[inline]
        fn log_ca(self,b:f32) -> f32 {approx::log(self,b)}

        /// Approximate square root using fastapprox, trading precision for speed.
        #[inline]
        fn sqrt_ca(self) -> f32 {approx::sqrt(self)}

        /// Approximate power using fastapprox, trading precision for speed.
        #[inline]
        fn pow_ca(self,b:f32) -> f32 {approx::pow(self,b)}

    }

    /// Fast, approximate math functions via fastapprox, suitable for games and simulations.
    #[cfg(feature="approx")]
    pub mod approx {
        use std::f32::consts::{E, PI};
        use fastapprox::faster as approx;

        #[inline]
        pub fn sin(a:f32) -> f32 {approx::sin(pi_clamp(a))}
        #[inline]
        pub fn cos(a:f32) -> f32 {approx::cos(pi_clamp(a))}
        #[inline]
        pub fn tan(a:f32) -> f32 {approx::tan(pih_clamp(a))}
        #[inline]
        pub fn exp(a:f32) -> f32 {approx::exp(a)}
        #[inline]
        pub fn log(a:f32,b:f32) -> f32 { 
            if (b - 2.).abs() < 1e-3 {approx::log2(a)} 
            else if (b - E).abs() < 1e-3 {approx::ln(a)} 
            else {a.log(b)}
        }
        #[inline]
        pub fn sqrt(a:f32) -> f32 {approx::pow(a,0.5)}
        #[inline]
        pub fn pow(a:f32,b:f32) -> f32 {approx::pow(a,b)}
        
        const PI2: f32 = PI*2.;
        fn pi_clamp(i:f32) -> f32 {sym_clamp(i,PI,PI2)}

        const PI2H: f32 = PI/2.;
        fn pih_clamp(i:f32) -> f32 {sym_clamp(i,PI2H,PI)}

        fn sym_clamp(i:f32,limit:f32,limit2:f32) -> f32 {
            let mut a = i % limit2;
            if a > limit && a < limit2 
                {a += -limit2;} 
            a
        }

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


// EOF \\
