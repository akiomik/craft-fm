#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Duration {
    ThirtySecond,
    Sixteenth,
    Eighth,
    Quarter,
    Half,
    Whole,
}

impl Duration {
    pub fn beats_per_measure(&self) -> usize {
        match self {
            Duration::ThirtySecond => 32,
            Duration::Sixteenth => 16,
            Duration::Eighth => 8,
            Duration::Quarter => 4,
            Duration::Half => 2,
            Duration::Whole => 1,
        }
    }

    pub fn relative(&self) -> f32 {
        match self {
            Duration::ThirtySecond => 0.03125,
            Duration::Sixteenth => 0.0625,
            Duration::Eighth => 0.125,
            Duration::Quarter => 0.25,
            Duration::Half => 0.5,
            Duration::Whole => 1.0,
        }
    }
}
