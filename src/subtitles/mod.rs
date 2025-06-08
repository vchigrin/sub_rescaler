mod loader;
mod writer;
use std::time;

const SECONDS_IN_MINUTE: u64 = 60;
const SECONDS_IN_HOUR: u64 = 60 * SECONDS_IN_MINUTE;

#[derive(Default, Clone, Debug)]
pub struct SubItem {
    pub number: u32,
    pub start_tp: time::Duration,
    pub end_tp: time::Duration,
    pub text: String,
}

impl SubItem {
    pub fn duration(&self) -> time::Duration {
        self.end_tp - self.start_tp
    }
}

pub use loader::Loader;
pub use writer::Writer;
