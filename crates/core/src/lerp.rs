use std::ops::Mul;

use crate::*;

// Linear Interpolation \\

    /// Linearly interpolate between two values using `Add`, `Sub`, `Mul<f32>`.
    pub trait Lerpable {
        /// Interpolate from self toward to by fraction t in [0, 1].
        ///
        /// # Example
        /// ```
        /// use deki_core::lerp::Lerpable;
        /// assert_eq!(0.0_f32.lerp(10.0, 0.5), 5.0);
        /// ```
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

    /// Like `Lerpable` but rounds with `mul_f32` for integer output.
    pub trait LerpableF32 {
        /// Interpolate using mul_f32 rounding, mainly for integers.
        ///
        /// # Example
        /// ```
        /// use deki_core::lerp::LerpableF32;
        /// assert_eq!(0i32.lerp(100, 0.0), 0);
        /// assert_eq!(0i32.lerp(100, 0.5), 50);
        /// assert_eq!(0i32.lerp(100, 1.0), 100);
        /// ```
        fn lerp(&self, to: Self, lerp: f32) -> Self;
    }

    impl<A> LerpableF32 for A
    where
        A: Clone
            + Add<Output = A>
            + Sub<Output = A>
            + MulF32,
    {
        fn lerp(&self, to: Self, lerp: f32) -> Self {
            self.clone().add(to.sub(self.clone()).mul_f32(lerp))
        }
    }

// Gated Linear Interpolation \\

    /// Interpolate toward a target and snap when within a threshold.
    pub trait Glerpable {
        /// Interpolate toward to, snapping when within thresh; returns true if arrived.
        ///
        /// # Example
        /// ```
        /// use deki_core::lerp::Glerpable;
        /// // snap when within threshold
        /// let mut val = 9.7f32;
        /// assert!(val.glerp(10.0, 0.1, 0.5));
        /// assert_eq!(val, 10.0);
        /// ```
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

    /// Interpolate cyclically, always taking the shortest wrapping path.
    pub trait Clerpable {
        /// Compute the shortest wrapping delta between self and to within [min, max).
        ///
        /// # Example
        /// ```
        /// use deki_core::lerp::Clerpable;
        /// // from 0.0→0.9 in [0,1): shortest is backward (-0.1)
        /// assert!((0.0_f32.delta_qucy(0.9, 0.0, 1.0) - (-0.1)).abs() < 1e-5);
        /// // from 0.9→0.0: shortest is forward (+0.1)
        /// assert!((0.9_f32.delta_qucy(0.0, 0.0, 1.0) - 0.1).abs() < 1e-5);
        /// ```
        fn delta_qucy(&self, to: Self, min: Self, max: Self) -> Self;
        /// Lerp toward to along the shortest wrapping path within [min, max).
        ///
        /// # Example
        /// ```
        /// use deki_core::lerp::Clerpable;
        /// // from 0.8→0.0 in [0,1): shortest is +0.2 forward
        /// let val: f32 = 0.8.lerp_qucy(0.0, 0.5, 0.0, 1.0);
        /// assert_eq!(val, 0.9);  // 0.8 + 0.1 = 0.9
        /// ```
        fn lerp_qucy(&self, to: Self, lerp: f32, min: Self, max: Self) -> Self;
        /// Gated cyclic lerp: interpolates along the shortest path, snapping within thresh.
        ///
        /// # Example
        /// ```
        /// use deki_core::lerp::Clerpable;
        /// // snap when within threshold
        /// let mut val = 0.99f32;
        /// assert!(val.glerp_qucy(0.0, 0.5, 0.05, 0.0, 1.0));  // snapped
        /// assert_eq!(val, 0.0);
        /// ```
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
                self.add_qucy(delta.mul_f32(lerp), min, max)
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

    /// Enable fixed-step interpolation toward a target.
    pub trait Stepable {
        /// Move toward a target in fixed steps, returning true when arrived.
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

    /// Enable cyclic fixed-step interpolation.
    pub trait CycleStapable {
        /// Move toward a cyclic target in fixed steps, wrapping around a range.
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
        /// Apply smoothstep easing: an S-curve from 0 to 1 for `t` in `[0, 1]`.
        ///
        /// # Example
        /// ```
        /// use deki_core::lerp::DekiExtF32;
        /// assert_eq!(0.0_f32.smooth(), 0.0);
        /// assert_eq!(0.25_f32.smooth(), 0.15625);
        /// assert_eq!(0.5_f32.smooth(), 0.5);
        /// assert_eq!(0.75_f32.smooth(), 0.84375);
        /// assert_eq!(1.0_f32.smooth(), 1.0);
        /// ```
        #[inline]
        fn smooth(self) -> f32 {
            self * self * (3. - 2. * self)
        }
        /// Clamp the value to the unit interval [0, 1].
        ///
        /// # Example
        /// ```
        /// use deki_core::lerp::DekiExtF32;
        ///
        /// // in-range
        /// assert_eq!(0.5_f32.clamp_unit(), 0.5);
        /// assert_eq!(0.0_f32.clamp_unit(), 0.0);
        /// assert_eq!(1.0_f32.clamp_unit(), 1.0);
        ///
        /// // below range
        /// assert_eq!((-1.0).clamp_unit(), 0.0);
        /// assert_eq!((-100.0).clamp_unit(), 0.0);
        ///
        /// // above range
        /// assert_eq!(2.0.clamp_unit(), 1.0);
        /// assert_eq!(100.0.clamp_unit(), 1.0);
        /// ```
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

    #[test]
    fn glerpable_coverage() {
        use super::Glerpable;
        let mut val = 0.0f32;
        assert!(!val.glerp(10.0, 0.1, 0.5));  // hasn't snapped
        assert!((val - 1.0).abs() < 1e-5);
        assert!(!val.glerp(10.0, 0.1, 0.5));
        assert!((val - 1.9).abs() < 1e-5);
        // snap when within threshold
        let mut val = 9.7f32;
        assert!(val.glerp(10.0, 0.1, 0.5));  // snapped
        assert_eq!(val, 10.0);
        // lerp >= 1 snaps immediately
        let mut val = 0.0f32;
        assert!(val.glerp(10.0, 1.0, 0.5));
        assert_eq!(val, 10.0);
    }

    #[test]
    fn clerpable_coverage() {
        use super::Clerpable;
        // shortest path: 0.9 → 0.1 goes forward (0.2), not backward (0.8)
        // lerp=1.0 means move full distance
        assert!((0.9_f32.lerp_qucy(0.1, 1.0, 0.0, 1.0) - 0.1).abs() < 1e-5);
        // delta_qucy: from 0.0 to 0.9 in [0,1) shortest is -0.1 (backward wraps to 0.9)
        assert!((0.0_f32.delta_qucy(0.9, 0.0, 1.0) - (-0.1)).abs() < 1e-5);
        // from 0.9 to 0.0 in [0,1) shortest is 0.1 (forward)
        assert!((0.9_f32.delta_qucy(0.0, 0.0, 1.0) - 0.1).abs() < 1e-5);
        // integer version: delta from 9 to 1 in [0,10) is 2, 2*0.5=1, 9+1=10 wraps to 0
        assert_eq!(9i32.lerp_qucy(1, 0.5, 0, 10), 0);
        // glerp_qucy: shortest path from 0.9→0.1 is +0.2 forward, lerp=0.1 → +0.02
        let mut val = 0.9f32;
        assert!(!val.glerp_qucy(0.1, 0.1, 0.05, 0.0, 1.0));
        assert!((val - 0.92).abs() < 1e-5);
    }
}
