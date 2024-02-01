use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Quality {
    P,
    A,
    D,
    Min,
    Maj,
}

impl Display for Quality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Quality::P => "P",
            Quality::A => "A",
            Quality::D => "d",
            Quality::Min => "m",
            Quality::Maj => "M",
        };
        write!(f, "{s}")
    }
}
