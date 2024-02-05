mod down;
mod up;
mod up_down;

#[allow(unused_imports)]
pub use down::DownArpeggiator;
#[allow(unused_imports)]
pub use up::UpArpeggiator;
pub use up_down::UpDownArpeggiator;

use crate::theory::Note;

#[allow(dead_code)]
pub type Arpeggiator = dyn Iterator<Item = Note>;
