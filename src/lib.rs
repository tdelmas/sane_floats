pub trait Sane {
    type SqrtOutput;

    fn sqrt(&self) -> Self::SqrtOutput;
}

impl Sane for f64 {
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
        assert!(4.0_f64.sqrt() == 2.0);

        let neg_zero = -0.0_f64;
        
        let sqrt_std = neg_zero.sqrt();
        assert!(sqrt_std.is_sign_negative());
        assert!(sqrt_std == 0.0);

        let sqrt_sane = crate::Sane::sqrt(&neg_zero);

        assert!(sqrt_sane.is_nan());
    }
}