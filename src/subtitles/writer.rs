use super::SubItem;
use super::SECONDS_IN_HOUR;
use super::SECONDS_IN_MINUTE;
use eyre::Result;
use std::io;
use std::time;

pub struct Writer<T>
where
    T: io::Write,
{
    output: T,
}

impl<T> Writer<T>
where
    T: io::Write,
{
    pub fn new(output: T) -> Self {
        Self { output }
    }

    pub fn write_item(&mut self, item: SubItem) -> Result<()> {
        self.output.write_fmt(format_args!(
            "{}\n{} --> {}\n{}\n\n",
            item.number,
            Self::format_duration(item.start_tp),
            Self::format_duration(item.end_tp),
            item.text
        ))?;
        Ok(())
    }

    fn format_duration(duration: time::Duration) -> String {
        let mut remaining_secs = duration.as_secs();
        let hours = remaining_secs / SECONDS_IN_HOUR;
        remaining_secs -= hours * SECONDS_IN_HOUR;

        let minutes = remaining_secs / SECONDS_IN_MINUTE;
        remaining_secs -= minutes * SECONDS_IN_MINUTE;

        format!(
            "{:02}:{:02}:{:02},{:03}",
            hours,
            minutes,
            remaining_secs,
            duration.as_millis() % 1000
        )
    }
}
