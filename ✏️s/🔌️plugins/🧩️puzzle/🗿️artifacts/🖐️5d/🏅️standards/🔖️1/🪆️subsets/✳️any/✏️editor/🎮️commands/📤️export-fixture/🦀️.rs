//! 📤 Downloads the live document as round-trippable JSON — the 5d twin of
//! `🧊️3d/…/📤️export-fixture` and `◻️2d/…/📤️export-fixture`: inline under one guest wire page,
//! the framework's segmented-download lane above it, a notice above what one segmented download
//! may carry.

use crate::editor::puzzle5d::Puzzle5dDocument;
use crate::retained_command::PUZZLE_COMMAND_OUTPUT_BYTES;
use semio_framework_plugin::app::{ArtifactDownloadOutput, ArtifactOutputChunks};
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::Fault;
use semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES;

/// 📤 The download filename for one export: the document's own `label` as a slug
/// (`concrete-forest.json`, `nakagin-capsule-tower.json`, `capsule-dream.json`), so exporting two
/// different examples never lands as two files with one name. A document carrying no label keeps the
/// app-generic `puzzle-5d.json`.
pub fn puzzle5d_export_filename(document: &Puzzle5dDocument) -> String {
    let mut slug = String::new();
    for character in document.label.as_deref().unwrap_or_default().chars() {
        if character.is_ascii_alphanumeric() {
            slug.extend(character.to_lowercase());
        } else if !slug.ends_with('-') {
            slug.push('-');
        }
    }
    let slug = slug.trim_matches('-');
    if slug.is_empty() {
        return "puzzle-5d.json".into();
    }
    format!("{slug}.json")
}

/// 📏️ Largest export payload that may ride INLINE inside one `Effect::DownloadMediaExport`.
///
/// 🧊️ Derived, never a literal: it is the guest's own per-request contiguous ceiling
/// ([`GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES`], one wasm page), and an inline effect payload is
/// exactly such a per-command block. The shipped examples sit astride this line — Concrete Forest
/// fits it, Nakagin Capsule Tower does not — so above it the payload goes through the framework's
/// segmented-download lane instead, which is what that lane exists for.
pub const fn puzzle5d_export_inline_budget_bytes() -> usize {
    GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES
}

/// 📤 The exported bytes of one document — the same projection `importFixture` round-trips.
pub fn puzzle5d_export_json(document: &Puzzle5dDocument) -> String {
    serde_json::to_string(document).unwrap_or_default()
}

/// 📤 One inline host download. Only legal at or under [`puzzle5d_export_inline_budget_bytes`].
pub fn puzzle5d_export_inline_effect(filename: String, data: String) -> Effect {
    Effect::DownloadMediaExport { filename, mime_type: "application/json".into(), data, encoding: Some("utf-8".into()) }
}

/// 📏️ Largest export ONE segmented download may carry: this command's declared contract output
/// budget ([`PUZZLE_COMMAND_OUTPUT_BYTES`]), itself admitted against the framework's end-to-end
/// segmented cap — so the budget a payload is measured against is the SAME number the shard worker
/// and the host drain enforce, and an app can never declare a download larger than the wire admits.
pub fn puzzle5d_export_segmented_budget_bytes() -> Result<usize, Fault> {
    ArtifactOutputChunks::admit_maximum(PUZZLE_COMMAND_OUTPUT_BYTES)
}

/// 📤 The same download as a SEGMENTED output: the payload sliced into the wire's own
/// [`ArtifactOutputChunks::CHUNK_BYTES`] pages and sealed, so the host drains it one bounded chunk
/// per awaited turn instead of receiving the whole JSON as one inline effect field. `identity`
/// encoding, not `base64`: the payload is UTF-8 JSON and the drain writes identity chunks straight
/// through.
pub fn puzzle5d_export_segmented(filename: String, data: &str) -> Result<ArtifactDownloadOutput, Fault> {
    let budget = puzzle5d_export_segmented_budget_bytes()?;
    if data.len() > budget {
        return Err(Fault::from("puzzle5d-export-exceeds-declared-output-budget"));
    }
    let chunks = ArtifactOutputChunks::new(budget);
    for page in data.as_bytes().chunks(ArtifactOutputChunks::CHUNK_BYTES) {
        chunks.push(page.to_vec())?;
    }
    chunks.seal()?;
    ArtifactDownloadOutput::new(filename, "application/json", Some("identity".into()), chunks)
}

/// 📤 The one refusal an export above [`puzzle5d_export_segmented_budget_bytes`] gives.
///
/// 🧾️ A NOTICE, never a fault: a download that cannot be carried is an answer the user must read,
/// and a producer fault on this path surfaces as a dead job. Capsule Dream is this arm's live case.
pub fn puzzle5d_export_refusal(filename: &str, bytes: usize, budget: usize) -> String {
    format!("{filename} is {bytes} B, over the {budget} B one export may stream — export fewer parts")
}

/// 📤 What one `exportFixture` publishes: the inline effect for a payload that fits one wire page,
/// the segmented output for anything larger, and a NOTICE for a payload above what one segmented
/// download may carry.
pub enum Puzzle5dExportPublication {
    Inline(Effect),
    Segmented(ArtifactDownloadOutput),
    Refused(String),
}

/// 📤 Resolves the publication for one document without touching any emission lane — the one place
/// the budget decision lives, so no two routes can disagree about it.
pub fn puzzle5d_export_publication(document: &Puzzle5dDocument) -> Result<Puzzle5dExportPublication, Fault> {
    let filename = puzzle5d_export_filename(document);
    let data = puzzle5d_export_json(document);
    if data.len() <= puzzle5d_export_inline_budget_bytes() {
        return Ok(Puzzle5dExportPublication::Inline(puzzle5d_export_inline_effect(filename, data)));
    }
    let budget = puzzle5d_export_segmented_budget_bytes()?;
    if data.len() > budget {
        return Ok(Puzzle5dExportPublication::Refused(puzzle5d_export_refusal(&filename, data.len(), budget)));
    }
    puzzle5d_export_segmented(filename, &data).map(Puzzle5dExportPublication::Segmented)
}
