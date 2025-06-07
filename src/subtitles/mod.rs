mod loader;
mod writer;
use std::time;

const SECONDS_IN_MINUTE: u64 = 60;
const SECONDS_IN_HOUR: u64 = 60 * SECONDS_IN_MINUTE;

#[derive(Default, Clone)]
pub struct SubItem {
    number: u32,
    start_tp: time::Duration,
    end_tp: time::Duration,
    text: String,
}

pub use loader::Loader;
pub use writer::Writer;
