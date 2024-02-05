#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Frequency(pub f32);

impl From<Frequency> for f32 {
    #[inline]
    fn from(value: Frequency) -> Self {
        value.0
    }
}

impl From<f32> for Frequency {
    #[inline]
    fn from(value: f32) -> Self {
        Frequency(value)
    }
}
