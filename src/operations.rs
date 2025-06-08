use eyre::Result;
use std::time;

use crate::subtitles;

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

// Returns items in range [start_frame_number..end_frame_number]
fn items_range(
    items: &[subtitles::SubItem],
    start_frame_number: u32,
    finish_frame_number: u32,
) -> Result<&[subtitles::SubItem]> {
    let start_idx: usize;
    if let Some(i) = items.iter().position(|f| f.number == start_frame_number) {
        start_idx = i;
    } else {
        return Err(eyre::eyre!("Frame {} not found", start_frame_number));
    }
    let end_idx: usize;
    if let Some(i) = items[start_idx..]
        .iter()
        .position(|f| f.number == finish_frame_number)
    {
        end_idx = start_idx + i;
    } else {
        return Err(eyre::eyre!("Frame {} not found", start_frame_number));
    }
    Ok(&items[start_idx..=end_idx])
}

fn rescale_range(
    src_items: &[subtitles::SubItem],
    ref_items: &[subtitles::SubItem],
    result: &mut Vec<subtitles::SubItem>,
) {
    assert!(!src_items.is_empty());
    assert!(!ref_items.is_empty());
    let src_start = src_items.first().unwrap().start_tp;
    let ref_start = ref_items.first().unwrap().start_tp;
    let src_duration = src_items.last().unwrap().start_tp - src_start;
    let ref_duration = ref_items.last().unwrap().start_tp - ref_start;
    for item in src_items {
        let duration_from_start = item.start_tp - src_start;
        let scaled_duration_from_start = (duration_from_start * (ref_duration.as_millis() as u32))
            / (src_duration.as_millis() as u32);
        let new_start = ref_start + scaled_duration_from_start;
        let new_duration = (item.duration() * (ref_duration.as_millis() as u32))
            / (src_duration.as_millis() as u32);
        let new_end = new_start + new_duration;
        result.push(subtitles::SubItem {
            number: item.number,
            start_tp: new_start,
            end_tp: new_end,
            text: item.text.clone(),
        })
    }
}

pub fn perform_sync(
    items: Vec<subtitles::SubItem>,
    mut alignmen_points: Vec<crate::FramePair>,
    reference_items: &[subtitles::SubItem],
) -> Result<Vec<subtitles::SubItem>> {
    if items.is_empty() || reference_items.is_empty() {
        return Ok(items);
    }
    // Ensure dummy sync points at the beginning and the end of intervals, to
    // ensure all source range is covered.
    if alignmen_points.is_empty() || alignmen_points[0].src_frame != items[0].number {
        alignmen_points.insert(
            0,
            crate::FramePair {
                src_frame: items[0].number,
                reference_frame: reference_items[0].number,
            },
        );
    }
    if alignmen_points.last().unwrap().src_frame != items.last().unwrap().number {
        alignmen_points.push(crate::FramePair {
            src_frame: items.last().unwrap().number,
            reference_frame: reference_items.last().unwrap().number,
        });
    }
    let mut it = alignmen_points.iter().peekable();
    let mut result = Vec::<subtitles::SubItem>::new();
    loop {
        // We will exit by encountering end of array in peek() call.
        // Unwrap here is OK since we guaranted to have at leas one point.
        let mut cur_point = *it.next().unwrap();
        let next_point = if let Some(p) = it.peek() {
            p
        } else {
            break;
        };
        if !result.is_empty() {
            // Avoid handling same point twice.
            cur_point.src_frame += 1;
            cur_point.reference_frame += 1;
        }

        let src_items = items_range(&items, cur_point.src_frame, next_point.src_frame)?;
        let ref_items = items_range(
            reference_items,
            cur_point.reference_frame,
            next_point.reference_frame,
        )?;
        rescale_range(src_items, ref_items, &mut result);
    }

    Ok(result)
}
