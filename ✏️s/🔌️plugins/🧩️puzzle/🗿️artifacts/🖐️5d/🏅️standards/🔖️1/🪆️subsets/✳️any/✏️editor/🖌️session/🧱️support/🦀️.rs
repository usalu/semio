//! 🧱️ Support of the puzzle 5d editor session: the reserved wire preflight, the job faults and checkpoints and the fixed-page step envelope the clipboard, paste and import jobs share.

use semio_framework_plugin::Fault;
use std::collections::HashSet;
use crate::editor::puzzle5d::entities::{PUZZLE5D_GRANULARITY_FASTENER, PUZZLE5D_GRANULARITY_PART, PUZZLE5D_INTERACTION_DOMAIN, PUZZLE5D_RESERVED_PAGE_BYTES};
use semio_framework_job::StepContext;

//#region 🧵️ReservedJobs
pub fn puzzle5d_preflight_reserved_wire(raw: Vec<u8>, maximum_bytes: usize) -> Result<Vec<u8>, (Fault, Vec<u8>)> {
    if raw.len() > maximum_bytes {
        return Err((Fault::from("puzzle5d reserved wire exceeds its exact route cap before fixed-page copy"), raw));
    }
    Ok(raw)
}

pub fn puzzle5d_job_fault(detail: impl AsRef<str>) -> crate::puzzle_job::JobTurn {
    let bytes = detail.as_ref().as_bytes();
    crate::puzzle_job::JobTurn::Fault(bytes[..bytes.len().min(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)].to_vec())
}

pub fn puzzle5d_job_checkpoint(stage: u8, cursor: usize, progress: u64) -> crate::puzzle_job::JobTurn {
    let mut state = [0; 17];
    state[0] = stage;
    state[1..9].copy_from_slice(&(cursor as u64).to_le_bytes());
    state[9..17].copy_from_slice(&progress.to_le_bytes());
    crate::puzzle_job::JobTurn::Checkpoint { applied_progress: progress, state: state.to_vec() }
}

fn puzzle5d_import_checkpoint_bytes(stage: u8, cursor: usize, nested_cursor: usize, decoded_items: usize, progress: u64) -> [u8; 33] {
    let mut state = [0; 33];
    state[0] = stage;
    state[1..9].copy_from_slice(&(cursor as u64).to_le_bytes());
    state[9..17].copy_from_slice(&(nested_cursor as u64).to_le_bytes());
    state[17..25].copy_from_slice(&(decoded_items as u64).to_le_bytes());
    state[25..33].copy_from_slice(&progress.to_le_bytes());
    state
}

pub fn puzzle5d_import_checkpoint(stage: u8, cursor: usize, nested_cursor: usize, decoded_items: usize, progress: u64) -> crate::puzzle_job::JobTurn {
    crate::puzzle_job::JobTurn::Checkpoint { applied_progress: progress, state: puzzle5d_import_checkpoint_bytes(stage, cursor, nested_cursor, decoded_items, progress).to_vec() }
}

pub fn puzzle5d_step_envelope(raw: &[u8], cursor: &mut usize, page: &mut [u8; PUZZLE5D_RESERVED_PAGE_BYTES], page_len: &mut usize, progress: &mut u64, cx: &mut StepContext<'_>) -> Option<crate::puzzle_job::JobTurn> {
    if *cursor >= raw.len() {
        *page_len = 0;
        return None;
    }
    let units = raw.len().saturating_sub(*cursor).min(page.len()).min(cx.fuel_remaining() as usize);
    if units == 0 {
        return Some(crate::puzzle_job::JobTurn::Yield);
    }
    let end = cursor.checked_add(units).filter(|end| *end <= raw.len()).expect("Puzzle5d fixed-page ingress preflights the source range before copy");
    page[..units].copy_from_slice(&raw[*cursor..end]);
    *page_len = units;
    *cursor = end;
    *progress = progress.saturating_add(units as u64);
    cx.consume_fuel(units as u64);
    Some(puzzle5d_job_checkpoint(0, *cursor, *progress))
}

pub fn puzzle5d_selection_ids(interaction: &semio_framework::InteractionState) -> (HashSet<String>, HashSet<String>) {
    match interaction.selection.get(PUZZLE5D_INTERACTION_DOMAIN) {
        Some(selection) if selection.granularity == PUZZLE5D_GRANULARITY_PART => (selection.ids.iter().cloned().collect(), HashSet::new()),
        Some(selection) if selection.granularity == PUZZLE5D_GRANULARITY_FASTENER => (HashSet::new(), selection.ids.iter().cloned().collect()),
        _ => (HashSet::new(), HashSet::new()),
    }
}
