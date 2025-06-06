use super::SubItem;
use super::SECONDS_IN_HOUR;
use super::SECONDS_IN_MINUTE;
use eyre::eyre;
use eyre::Result;
use std::io;
use std::time;

#[derive(Debug)]
enum LoaderState {
    Number,
    Timestamps,
    Text,
}

pub struct Loader<T>
where
    T: io::BufRead,
{
    working_item: SubItem,
    ready_item: Option<SubItem>,
    expect_next_line_state: LoaderState,
    input: T,
}

impl<T> Loader<T>
where
    T: io::BufRead,
{
    pub fn new(input: T) -> Self
    where
        T: io::BufRead,
    {
        Self {
            working_item: SubItem::default(),
            ready_item: None,
            expect_next_line_state: LoaderState::Number,
            input,
        }
    }

    pub fn read_next(&mut self) -> Result<Option<SubItem>> {
        loop {
            let mut line = String::new();
            let read_count = self.input.read_line(&mut line)?;
            if read_count == 0 {
                // EOF reached.
                return Ok(None);
            }
            // Some files have BOM marker at start.
            const BOM: char = '\u{feff}';
            let line_trimmed = line.trim_matches(|c: char| c.is_whitespace() || c == BOM);
            self.feed_line(line_trimmed)?;
            if self.ready_item.is_some() {
                return Ok(self.ready_item.take());
            }
        }
    }

    fn feed_line(&mut self, line: &str) -> Result<()> {
        match self.expect_next_line_state {
            LoaderState::Number => self.parse_number_line(line),
            LoaderState::Timestamps => self.parse_timestamps_line(line),
            LoaderState::Text => self.parse_text_line(line),
        }
    }

    fn parse_number_line(&mut self, line: &str) -> Result<()> {
        let num = line.parse::<u32>()?;
        self.working_item.number = num;
        self.expect_next_line_state = LoaderState::Timestamps;
        Ok(())
    }

    // Parses string like "00:00:08,520 --> 00:00:10,240"
    fn parse_timestamps_line(&mut self, line: &str) -> Result<()> {
        if let Some(components) = line.split_once(" --> ") {
            self.working_item.start_tp = self.parse_duration(components.0)?;
            self.working_item.end_tp = self.parse_duration(components.1)?;
            self.expect_next_line_state = LoaderState::Text;
            Ok(())
        } else {
            Err(eyre!("Wrong timestamps line: {}", line))
        }
    }

    fn parse_text_line(&mut self, line: &str) -> Result<()> {
        if line.is_empty() {
            self.ready_item = Some(std::mem::take(&mut self.working_item));
            self.expect_next_line_state = LoaderState::Number;
            return Ok(());
        }
        if self.working_item.text.is_empty() {
            self.working_item.text = line.to_string();
        } else {
            self.working_item.text.push('\n');
            self.working_item.text.push_str(line);
        }
        Ok(())
    }

    // Parses stirng linke "00:00:08,520"
    fn parse_duration(&self, dur_string: &str) -> Result<time::Duration> {
        let sec_string: &str;
        let mut result_duration = time::Duration::ZERO;
        if let Some(ms_sep) = dur_string.find(',') {
            if ms_sep > 0 && ms_sep < (dur_string.len() - 1) {
                let ms_count = dur_string[(ms_sep + 1)..].parse::<u64>()?;
                result_duration += time::Duration::from_millis(ms_count);
                sec_string = &dur_string[..ms_sep];
            } else {
                return Err(eyre!("Invalid duration string {}", dur_string));
            }
        } else {
            return Err(eyre!("Invalid duration string {}", dur_string));
        }
        for (idx, part) in sec_string.split(':').enumerate() {
            let component = part.parse::<u64>()?;
            match idx {
                0 => {
                    result_duration += time::Duration::from_secs(component * SECONDS_IN_HOUR);
                }
                1 => {
                    result_duration += time::Duration::from_secs(component * SECONDS_IN_MINUTE);
                }
                2 => {
                    result_duration += time::Duration::from_secs(component);
                }
                _ => {
                    return Err(eyre!("Invalid duration string {}", dur_string));
                }
            }
        }
        Ok(result_duration)
    }
}
