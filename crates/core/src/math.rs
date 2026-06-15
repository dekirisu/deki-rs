use extension_traits::extension as ext;

// Approximate math (~1% error, bit-hack based) \\

/// Provide fast approximate math methods on `f32`.
///
/// All methods use bit-hack techniques derived from fastapprox.
/// Accuracy is ~1% — fast but approximate. Use `std::math` when precision matters.
///
/// # Examples
/// ```
/// use deki_core::math::DekiExtApprox;
///
/// let x = 0.5_f32;
/// assert!((x.exp_fast() - 1.6487).abs() < 0.1);
/// assert!((x.pow_fast(2.0) - 0.25).abs() < 0.01);
/// assert!((x.sqrt_fast() - 0.7071).abs() < 0.01);
/// ```
#[allow(clippy::excessive_precision, clippy::approx_constant)]
#[ext(pub trait DekiExtApprox)]
impl f32 {
    /// Compute the approximate square root using bit-hack + 1 Newton-Raphson step (~0.1% error, 1.4× faster than std).
    #[inline]
    fn sqrt_fast(self) -> f32 {
        if self <= 0.0 { return 0.0; }
        let bits = self.to_bits();
        let guess = f32::from_bits(0x1fbd1daf + (bits >> 1));
        guess + (self / guess - guess) * 0.5
    }

    /// Compute the approximate exponential using bit-hack (2^(x * LOG2_E)) (~1% error, 2.2× faster than std).
    #[inline]
    fn exp_fast(self) -> f32 {
        pow2_fast(1.442695040_f32 * self) // LOG2_E
    }

    /// Compute the approximate power using bit-hack (2^(exp * log2(base))) (~1% error, 1.5× faster than std).
    #[inline]
    fn pow_fast(self, b: f32) -> f32 {
        pow2_fast(b * self.log2_fast())
    }

    /// Compute the base-2 logarithm using bit-hack (~1% error).
    #[inline]
    fn log2_fast(self) -> f32 {
        let vx = self.to_bits();
        let mx = f32::from_bits((vx & 0x007FFFFF_u32) | 0x3f000000);
        let mut y = vx as f32;
        y *= 1.1920928955078125e-7_f32;
        y - 124.22551499_f32 - 1.498030302_f32 * mx - 1.72587999_f32 / (0.3520887068_f32 + mx)
    }
}

// Private helpers \\

/// Raise 2 to a floating point power using bit-hack
#[allow(clippy::excessive_precision, clippy::approx_constant)]
#[inline]
fn pow2_fast(p: f32) -> f32 {
    let clipp = if p < -126.0 { -126.0_f32 } else { p };
    let v = ((1 << 23) as f32 * (clipp + 126.94269504_f32)) as u32;
    f32::from_bits(v)
}

// Tests \\

#[cfg(test)]
#[allow(clippy::excessive_precision, clippy::approx_constant)]
mod tests {
    use super::DekiExtApprox;

    fn approx_eq(a: f32, b: f32, max_abs: f32, max_rel: f32) -> bool {
        let diff = (a - b).abs();
        diff <= max_abs || diff / b.abs() <= max_rel
    }

    // sqrt_fast \|

    #[test]
    fn sqrt_fast_close_to_std() {
        for i in 1..=100 {
            let x = (i as f32) * 0.1;
            let expected = x.sqrt();
            let got = x.sqrt_fast();
            assert!(
                approx_eq(got, expected, 1e-2, 7e-2),
                "sqrt_fast({}) = {} (expected {})",
                x, got, expected
            );
        }
    }

    // exp_fast \|

    #[test]
    fn exp_fast_close_to_std() {
        for i in -10..=10 {
            let x = (i as f32) * 0.1;
            let expected = x.exp();
            let got = x.exp_fast();
            assert!(
                approx_eq(got, expected, 5e-2, 5e-2),
                "exp_fast({}) = {} (expected {})",
                x, got, expected
            );
        }
    }

    // pow_fast \|

    #[test]
    fn pow_fast_close_to_std() {
        for i in 1..=50 {
            let base = (i as f32) * 0.2;
            let exp = 2.0;
            let expected = base.powf(exp);
            let got = base.pow_fast(exp);
            assert!(
                approx_eq(got, expected, 1.0, 7e-2),
                "pow_fast({}, {}) = {} (expected {})",
                base, exp, got, expected
            );
        }
    }

    // log2_fast \|

    #[test]
    fn log2_fast_close_to_std() {
        for i in 1..=100 {
            let x = (i as f32) * 0.1;
            let expected = x.log2();
            let got = x.log2_fast();
            assert!(
                approx_eq(got, expected, 1e-1, 6e-2),
                "log2_fast({}) = {} (expected {})",
                x, got, expected
            );
        }
    }

    #[test]
    fn pow_fast_identity() {
        // Bit-hack is ~1% error
        assert!((10.0_f32.pow_fast(0.0) - 1.0).abs() < 0.1);
    }
}
