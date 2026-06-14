use std::ops::Mul;

use crate::*;

// Linear Interpolation \\

    /// Any type that implements necessary math traits for linear interpolation (lerp)
    pub trait Lerpable {
        /// Perform linear interpolation
        fn lerp(&self, to: Self, lerp: f32) -> Self;
    }

    impl<A> Lerpable for A
    where
        A: Clone
            + Add<Output = A>
            + Sub<Output = A>
            + Mul<f32, Output = A>,
    {
        fn lerp(&self, to: Self, lerp: f32) -> Self {
            self.clone().add(to.sub(self.clone()).mul(lerp))
        }
    }

    /// Any type that implements necessary math traits for linear interpolation (lerp)
    pub trait LerpableF32 {
        /// Perform linear interpolation
        fn lerp(&self, to: Self, lerp: f32) -> Self;
    }

    impl<A> LerpableF32 for A
    where
        A: Clone
            + Add<Output = A>
            + Sub<Output = A>
            + MulF32,
    {
        /// Wonky lerp, mainly meant for integers
        fn lerp(&self, to: Self, lerp: f32) -> Self {
            self.clone().add(to.sub(self.clone()).mul_f32(lerp))
        }
    }

// Gated Linear Interpolation \\

    /// Any type that implements necessary math traits for linear interpolation (lerp)
    pub trait Glerpable {
        /// Perform linear interpolation or snap to a threshold, true if 'arrived'
        fn glerp(&mut self, to: Self, lerp: f32, thresh: Self) -> bool;
    }

    compose! {
        impl Glerpable for ^0 {
            #[inline]
            fn glerp(&mut self, to: Self, lerp: f32, thresh: Self) -> bool {
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
        /// Get closest delta, whichever direction is closer
        /// - assumes self and the target are inside range
        fn delta_qucy(&self, to: Self, min: Self, max: Self) -> Self;
        /// Lerp to a value, auto-choosing which direction is fastest
        /// - assumes self and the target are inside range
        fn lerp_qucy(&self, to: Self, lerp: f32, min: Self, max: Self) -> Self;
        /// Lerp to a value, auto-choosing which direction is fastest, snaps to goal with
        /// using a threshold
        /// - assumes self and the target are inside range
        fn glerp_qucy(&mut self, to: Self, lerp: f32, thresh: Self, min: Self, max: Self) -> bool;
    }

    compose! {
        impl Clerpable for ^0 {
            #[inline]
            fn delta_qucy(&self, to: Self, min: Self, max: Self) -> Self {
                let delta = to - *self;
                let deltabs = delta.abs();
                let dolta = max - min - deltabs;
                if deltabs < dolta {
                    delta
                } else {
                    -dolta * delta.signum()
                }
            }
            #[inline]
            fn lerp_qucy(&self, to: Self, lerp: f32, min: Self, max: Self) -> Self {
                let delta = self.delta_qucy(to, min, max);
                self.add_qucy((delta as f32 * lerp) as ^0, min, max)
            }
            #[inline]
            fn glerp_qucy(&mut self, to: Self, lerp: f32, thresh: Self, min: Self, max: Self) -> bool {
                let delta = self.delta_qucy(to, min, max);
                if delta.abs() <= thresh || lerp >= 1. {
                    *self = to;
                    true
                } else {
                    *self = self.add_qucy(delta.mul_f32(lerp), min, max);
                    false
                }
            }
        }
        #f32 #f64 #i8 #i16 #i32 #i64 #i128 #isize
    }

// Lerp by Steps \\

    pub trait Stepable {
        fn sterp(&mut self, to: Self, step: Self) -> bool;
    }

    compose! {
        impl Stepable for ^0 {
            fn sterp(&mut self, to: Self, step: Self) -> bool {
                let delta = to - *self;
                if delta.abs() <= step {
                    *self = to;
                    true
                } else {
                    *self += delta.signum() * step;
                    false
                }
            }
        }
        #f32 #f64 #i8 #i16 #i32 #i64 #i128 #isize
    }

// Cyclic Lerp by Steps \\

    pub trait CycleStapable {
        fn sterp_qucy(&mut self, to: Self, step: Self, min: Self, max: Self) -> bool;
    }

    compose! {
        impl CycleStapable for ^0 {
            fn sterp_qucy(&mut self, to: Self, step: Self, min: Self, max: Self) -> bool {
                let delta = self.delta_qucy(to, min, max);
                if delta.abs() <= step {
                    *self = to;
                    true
                } else {
                    *self = self.add_qucy(delta.signum() * step, min, max);
                    false
                }
            }
        }
        #f32 #f64 #i8 #i16 #i32 #i64 #i128 #isize
    }

// Goodies \\

    #[ext(pub trait DekiExtF32)]
    impl f32 {
        /// Quick easing (smooth-step): has to be within 0..=1, consider using .clamp_unit() if not sure
        #[inline]
        fn smooth(self) -> f32 {
            self * self * (3. - 2. * self)
        }
        /// Same as `clamp(0., 1.)`
        #[inline]
        fn clamp_unit(self) -> f32 {
            self.clamp(0., 1.)
        }
    }

// Tests \\

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stapable_i32() {
        let mut num = 1i32;
        assert!(!num.sterp(4, 2));
        assert!(num.sterp(4, 5));
        assert_eq!(num, 4);
        assert!(!num.sterp(-4, 6));
        assert!(num.sterp(-4, 1337));
        assert_eq!(num, -4);
    }

    #[test]
    fn stapable_f32() {
        let mut num = 1.0f32;
        assert!(!num.sterp(2., 0.6));
        assert!(num.sterp(2., 0.6));
        assert_eq!(num, 2.);
        assert!(!num.sterp(-2., 3.2));
        assert!(num.sterp(-2., 100.));
        assert_eq!(num, -2.);
    }

    #[test]
    fn stapable_auto_left_overflow() {
        let mut num = 2.9f32;
        let (min, max) = (1.5, 8.);
        assert!(!num.sterp_qucy(7., 1., min, max));
        assert!(num < 2.);
        assert!(!num.sterp_qucy(7., 1., min, max));
        assert!(num > 7.);
        assert!(num.sterp_qucy(7., 1., min, max));
        assert_eq!(num, 7.);
    }

    #[test]
    fn stapable_auto_right_overflow() {
        let mut num = 6.9f32;
        let (min, max) = (1.5, 8.);
        assert!(!num.sterp_qucy(3., 1., min, max));
        assert!(num > 7.);
        assert!(!num.sterp_qucy(3., 1., min, max));
        assert!(num < 2.5);
        assert!(num.sterp_qucy(3., 1., min, max));
        assert_eq!(num, 3.);
    }
}
