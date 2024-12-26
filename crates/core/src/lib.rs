pub use maflow::*;
pub use type_cell::*;
pub use derive_more;
pub use buns;

// Renames \\

    pub use std::marker::PhantomData as Ghost;
    pub use derive_new::new as Constructor;
    pub use extension_traits::extension as ext;
    pub use derive_more as drv;
    pub use buns::sandwich;

// Traits \\

    /// 'Short form' for: 'static+Send+Sync
    pub trait Syncable:'static+Send+Sync {}
    impl <T:'static+Send+Sync> Syncable for T {}

    pub trait DefaultClear: Default {
        fn clear(&mut self){*self = Self::default();}
    }
    impl <A:Default> DefaultClear for A {}

// Extensionss  \\
use std::ops::{Add, Range, RangeInclusive, Rem, Sub};

    #[ext(pub trait RangeOffset)]
    impl <Idx:Clone+Add<Output=Idx>> RangeInclusive<Idx> {
        fn offset(&self,rhs:Idx) -> Self {
            self.start().clone()+rhs.clone()
            ..=self.end().clone()+rhs.clone()
        }
    }

    impl <Idx:Clone+Add<Output=Idx>> RangeOffset<Idx> for Range<Idx> {
        fn offset(&self,rhs:Idx) -> Self {
            self.start.clone()+rhs.clone()..self.end.clone()+rhs.clone()
        }
    }

// Cycling Addition \\

    buns::sandwich!{paste!{
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

    pub trait CycleMath {
        /// quick cycling addition within a exclusive range, which assumes:
        /// - current value is within the range
        /// - range length < _::MAX/3
        fn add_qucy(self,rhs:Self,min:Self,max:Self) -> Self;
        /// quick cycling subtraction within a exclusive range, which assumes:
        /// - current value is within the range
        /// - range length < _::MAX/3
        fn sub_qucy(self,rhs:Self,min:Self,max:Self) -> Self;
    }
    
    buns::sandwich!{
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
        /// wonky float multiplication, mainly meant for integers
        fn mul_f32(self,rhs:f32) -> Self {self * rhs}
    }

    impl MulF32 for f64 {
        #[inline]
        fn mul_f32(self,rhs:f32) -> Self {self * rhs as f64}
    }

    buns::sandwich!{
        impl MulF32 for ^0 {
            #[inline]
            fn mul_f32(self,rhs:f32) -> Self {
                (self as f32 * rhs).round() as ^0 
            }
        }
        #i8 #i16 #i32 #i64 #i128 #isize
    }

// Randomness \\

    /// quick and easy randomness
    #[cfg(feature="random")]
    pub mod random {
        use std::ops::Range;
        pub use fastrand::*;

        /// random f32 within a range (including)
        #[inline]
        pub fn f32r(range:Range<f32>) -> f32 {
            range.start + f32() * (range.end - range.start)
        }    

    }

    #[cfg(feature="random")]
    #[ext(pub trait DekiExtVecRng)]
    impl <T> Vec<T> {
        /// get a random entry of this vec
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
        /// approx sine, faster but inaccurate
        #[inline]
        fn sin_ca(self) -> f32 {approx::sin(self)}

        /// approx cosine, faster but inaccurate
        #[inline]
        fn cos_ca(self) -> f32 {approx::cos(self)}

        /// approx tangent, faster but inaccurate
        #[inline]
        fn tan_ca(self) -> f32 {approx::tan(self)}
        
        /// approx exponential, faster but inaccurate
        #[inline]
        fn exp_ca(self) -> f32 {approx::exp(self)}

        /// approx logarithm, faster but inaccurate
        #[inline]
        fn log_ca(self,b:f32) -> f32 {approx::log(self,b)}

        /// approx square, faster but inaccurate
        #[inline]
        fn sqrt_ca(self) -> f32 {approx::sqrt(self)}

        /// approx power by, faster but inaccurate
        #[inline]
        fn pow_ca(self,b:f32) -> f32 {approx::pow(self,b)}

    }

    /// cheaper but inaccurate math implementations
    #[cfg(feature="approx")]
    pub mod approx {
        use std::f32::consts::PI;
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
            if b==2. {approx::log2(a)} 
            else if b==2.72 {approx::ln(a)} 
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
                {a = -limit2 + a;} 
            a
        }

    }

// Quick Constant \\

    /// create a constant struct and name it after itself
    #[macro_export]
    macro_rules! qonst {
        ($ty:ty: $($tt:tt)*) => {paste!{
            pub const [<$ty:snake:upper>]: $ty = $ty {
                $($tt)*
            };
        }};
        ($ty:ident::$($tt:tt)*) => {paste!{
            pub const [<$ty:snake:upper>]: $ty = $ty::$($tt)*;
        }};
    }

// EOF \\
