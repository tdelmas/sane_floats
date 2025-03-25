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
    use super::*;

    #[test]
    fn test_sqrt() {
        #[allow(clippy::float_cmp)]
        assert_eq!(4.0.sqrt(), 2.0);

        assert!((-0.0).sqrt().is_nan());
    }
}