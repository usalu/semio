//! 📤 Downloads the live fixture as round-trippable JSON.

use crate::retained_command::PUZZLE_COMMAND_OUTPUT_BYTES;
use semio_framework_plugin::app::{ArtifactDownloadOutput, ArtifactOutputChunks};
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::Fault;
use semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES;
use serde_json::Value;

/// 📤 The download filename for one export: the document's own manifest id (`concrete-forest.json`,
/// `nakagin.json`), or the app-generic `puzzle-2d.json` for a document that came from no example.
pub fn puzzle2d_export_filename(fixture: &Value) -> String {
    match fixture.get("meta").and_then(|meta| meta.get("manifestId")).and_then(Value::as_str).filter(|id| !id.is_empty()) {
        Some(id) => format!("{id}.json"),
        None => "puzzle-2d.json".into(),
    }
}

/// 📏️ Largest export payload that may ride INLINE inside one `Effect::DownloadMediaExport` — the guest's
/// own per-request contiguous ceiling (one wasm page); above it the payload goes through the framework's
/// segmented-download lane (Nakagin's fixture is ~94 KiB).
pub const fn puzzle2d_export_inline_budget_bytes() -> usize {
    GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES
}

/// 📤 The exported bytes of one fixture — the same projection `importFixture` round-trips.
pub fn puzzle2d_export_json(fixture: &Value) -> String {
    fixture.to_string()
}

pub fn puzzle2d_export_inline_effect(filename: String, data: String) -> Effect {
    Effect::DownloadMediaExport { filename, mime_type: "application/json".into(), data, encoding: Some("utf-8".into()) }
}

/// 📏️ Largest export ONE segmented download may carry: this command's declared contract output budget.
pub fn puzzle2d_export_segmented_budget_bytes() -> Result<usize, Fault> {
    ArtifactOutputChunks::admit_maximum(PUZZLE_COMMAND_OUTPUT_BYTES)
}

/// 📤 The same download as a SEGMENTED output, sliced into the wire's own chunk pages and sealed.
pub fn puzzle2d_export_segmented(filename: String, data: &str) -> Result<ArtifactDownloadOutput, Fault> {
    let budget = puzzle2d_export_segmented_budget_bytes()?;
    if data.len() > budget {
        return Err(Fault::from("puzzle2d-export-exceeds-declared-output-budget"));
    }
    let chunks = ArtifactOutputChunks::new(budget);
    for page in data.as_bytes().chunks(ArtifactOutputChunks::CHUNK_BYTES) {
        chunks.push(page.to_vec())?;
    }
    chunks.seal()?;
    ArtifactDownloadOutput::new(filename, "application/json", Some("identity".into()), chunks)
}

pub fn puzzle2d_export_refusal(filename: &str, bytes: usize, budget: usize) -> String {
    format!("{filename} is {bytes} B, over the {budget} B one export may stream — export fewer nodes")
}

/// 📤 What one `exportFixture` publishes: inline under one wire page, segmented above it, a NOTICE above
/// what one segmented download may carry.
pub enum Puzzle2dExportPublication {
    Inline(Effect),
    Segmented(ArtifactDownloadOutput),
    Refused(String),
}

pub fn puzzle2d_export_publication(fixture: &Value) -> Result<Puzzle2dExportPublication, Fault> {
    let filename = puzzle2d_export_filename(fixture);
    let data = puzzle2d_export_json(fixture);
    if data.len() <= puzzle2d_export_inline_budget_bytes() {
        return Ok(Puzzle2dExportPublication::Inline(puzzle2d_export_inline_effect(filename, data)));
    }
    let budget = puzzle2d_export_segmented_budget_bytes()?;
    if data.len() > budget {
        return Ok(Puzzle2dExportPublication::Refused(puzzle2d_export_refusal(&filename, data.len(), budget)));
    }
    puzzle2d_export_segmented(filename, &data).map(Puzzle2dExportPublication::Segmented)
}
