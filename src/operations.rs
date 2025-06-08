use crate::subtitles;
use std::time;

pub fn perform_offset(
    mut items: Vec<subtitles::SubItem>,
    offset_ms: i32,
) -> Vec<subtitles::SubItem> {
    if offset_ms >= 0 {
        let offset = time::Duration::from_millis(offset_ms.try_into().unwrap());
        for item in &mut items {
            item.start_tp += offset;
            item.end_tp += offset;
        }
        items
    } else {
        let offset = time::Duration::from_millis((-offset_ms).try_into().unwrap());
        // Some items may be absent if they will get negative duration after offset
        let mut new_items = Vec::with_capacity(items.len());
        for mut item in items {
            if let Some(new_start) = item.start_tp.checked_sub(offset) {
                item.start_tp = new_start;
                item.end_tp -= offset;
                new_items.push(item);
            }
        }
        new_items
    }
}
