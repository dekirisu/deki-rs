use deki_core::*;
use std::ops::{Add, Mul, Sub};
mod relative; pub use relative::*;
mod absolute; pub use absolute::*;

// Goodies \\

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

// EOF \\
