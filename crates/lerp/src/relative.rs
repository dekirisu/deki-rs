use crate::*;

// Linear Interpolation \\

    /// Any type that implemients necessary math traits for linear interpolation (lerp)
    pub trait Lerpable {
        /// Perform linear interpolation
        fn lerp(&self,to:Self,lerp:f32) -> Self;
    }

    impl <A> Lerpable for A 
    where A: Clone
        + Add<Output=A>
        + Sub<Output=A>
        + Mul<f32,Output=A>
    {
        fn lerp(&self,to:Self,lerp:f32) -> Self {
            self.clone().add(to.sub(self.clone()).mul(lerp))
        }
    }

    /// Any type that implemients necessary math traits for linear interpolation (lerp)
    pub trait LerpableF32 {
        /// Perform linear interpolation
        fn lerp(&self,to:Self,lerp:f32) -> Self;
    }

    impl <A> LerpableF32 for A 
    where A: Clone
        + Add<Output=A>
        + Sub<Output=A>
        + MulF32
    {
        /// wonky lerp, mainly meant for integers
        fn lerp(&self,to:Self,lerp:f32) -> Self {
            self.clone().add(to.sub(self.clone()).mul_f32(lerp))
        }
    }

// Gated Linear Interpolation \\

    /// Any type that implements necessary math traits for linear interpolation (lerp)
    pub trait Glerpable {
        /// Perform linear interpolation or snap to a threshold, true if 'arrived'
        fn glerp(&mut self,to:Self,lerp:f32,thresh:Self) -> bool;
    }

    buns::sandwich!{
        impl Glerpable for ^0 {
            #[inline]
            fn glerp(&mut self,to:Self,lerp:f32,thresh:Self) -> bool {
                let delta = to - *self;
                if delta.abs() <= thresh || lerp >= 1. {
                    *self = to; 
                    true
                } else {
                    *self += delta.mul_f32(lerp);
                    false
                }
            }
        }
        #f32 #f64 #i8 #i16 #i32 #i64 #i128 #isize
    }
    
// Cycling Linear Interpolation \\

    pub trait Clerpable {
        /// get closest delta, whichever direction is closer
        /// - assumes self and the target are inside range
        fn delta_qucy(&self,to:Self,min:Self,max:Self) -> Self;
        /// lerp to a value, auto-choosing which direction is fastest
        /// - assumes self and the target are inside range
        fn lerp_qucy(&self,to:Self,lerp:f32,min:Self,max:Self) -> Self;
        /// lerp to a value, auto-choosing which direction is fastest, snaps to goal with 
        /// using a threshold
        /// - assumes self and the target are inside range
        fn glerp_qucy(&mut self,to:Self,lerp:f32,thresh:Self,min:Self,max:Self) -> bool;
    }

    buns::sandwich!{
        impl Clerpable for ^0 {
            #[inline]
            fn delta_qucy(&self,to:Self,min:Self,max:Self) -> Self {
                let delta = to - *self;
                let deltabs = delta.abs();
                let dolta = max - min - deltabs;
                if deltabs < dolta {delta} else {-dolta * delta.signum()}
            }
            #[inline]
            fn lerp_qucy(&self,to:Self,lerp:f32,min:Self,max:Self) -> Self {
                let delta = self.delta_qucy(to,min,max);
                self.add_qucy((delta as f32 * lerp) as ^0,min,max)
            }
            #[inline]
            fn glerp_qucy(&mut self,to:Self,lerp:f32,thresh:Self,min:Self,max:Self) -> bool {
                let delta = self.delta_qucy(to,min,max);
                if delta.abs() <= thresh || lerp >= 1. {
                    *self = to; 
                    true
                } else {
                    *self += delta.mul_f32(lerp);
                    false
                }          
            }
        }
        #f32 #f64 #i8 #i16 #i32 #i64 #i128 #isize
    }

// EOF \\
