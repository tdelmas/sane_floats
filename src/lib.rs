pub trait Sane {
    type SqrtOutput;

    fn sqrt(&self) -> Self::SqrtOutput;
}

impl Sane for f64 {
    type SqrtOutput = f64;

    
    fn sqrt(&self) -> Self::SqrtOutput {
        self.sqrt()
    }
}
