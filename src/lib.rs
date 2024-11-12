pub use maflow::*;
pub use type_cell::*;
pub use deki_derive::*;
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
use std::ops::{Mul, Range, RangeInclusive};

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

// Quick Interpolation \\
use std::ops::{Add,Deref,DerefMut,Sub};

    /// Any type that implements necessary math traits for linear interpolation (lerp)
    pub trait Lerpable {
        /// Perform linear interpolation
        fn lerp(&self,to:Self,lerp:f32) -> Self;
    }

    impl <A> Lerpable for A
    where A: Add<Output=A>+Sub<Output=A>+Mul<f32,Output=A>+Clone{
        fn lerp(&self,to:Self,lerp:f32) -> Self {
            self.clone().add(to.sub(self.clone()).mul(lerp))
        }
    }
    
    /// Any type that deferes to a type that implements necessary math traits for linear interpolation (lerp)
    pub trait LerpableDeref {
        /// Perform linear interpolation
        fn lerp(&self,to:Self,lerp:f32) -> Self;
    }

    impl <A:Deref+DerefMut+Clone> LerpableDeref for A
    where A::Target: Add<Output=A::Target>+Sub<Output=A::Target>+Mul<f32,Output=A::Target>+Clone{
        fn lerp(&self,to:Self,lerp:f32) -> Self {
            let mut out = self.clone();
            let a = self.deref().clone();
            let b = to.deref().to_owned();
            let c = a.clone().add(b.sub(a).mul(lerp));
            *out.deref_mut() = c;
            out
        }
    }

    #[ext(pub trait DekiExtF32)]
    impl f32 {
        /// quick easing (smooth-step): has to be within 0..=1, consider using .clamp_unit() if not sure
        #[inline]
        fn smooth(self) -> f32 {
            self * self * (3. - 2. * self)
        }
        /// same as `clamp(0.,1.)`
        #[inline]
        fn clamp_unit(self) -> f32 {self.clamp(0.,1.)}
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

// EOF \\
