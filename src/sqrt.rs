/// This trait provide a sane version of the `sqrt` function:
/// It returns `NaN` for negative floats,, INCLUDING `-0.0`.
pub trait Sqrt {
    /// The output type of the `sqrt` function.
    type SqrtOutput;

    /// Returns the square root of a float.
    /// If the number is negative, it returns `NaN`.
    fn sqrt(&self) -> Self::SqrtOutput;
}

impl Sqrt for f64 {
    type SqrtOutput = Self;
    
    fn sqrt(&self) -> Self::SqrtOutput {
        if self.is_sign_negative() {
            Self::NAN
        } else {
            (*self).sqrt()
        }
    }
}

impl Sqrt for f32 {
    type SqrtOutput = Self;
    
    fn sqrt(&self) -> Self::SqrtOutput {
        if self.is_sign_negative() {
            Self::NAN
        } else {
            (*self).sqrt()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sqrt() {
        #[allow(clippy::float_cmp)]
        let should_be_true = 4.0_f64.sqrt() == 2.0;
        assert!(should_be_true);

        let neg_zero = -0.0_f64;
        
        let sqrt_std = neg_zero.sqrt();
        assert!(sqrt_std.is_sign_negative());
        assert!(sqrt_std == 0.0);

        let sqrt_sane = crate::Sqrt::sqrt(&neg_zero);

        assert!(sqrt_sane.is_nan());
    }
}