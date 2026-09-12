//! 📤 Downloads the live fixture as round-trippable JSON.

use crate::editor::puzzle3d::{puzzle3d_projection_value, Puzzle3dActionCtx, Puzzle3dFixture};
use crate::retained_command::PUZZLE_COMMAND_OUTPUT_BYTES;
use dsl::os_pack::json::to_json_string;
use semio_framework_plugin::app::{ArtifactDownloadOutput, ArtifactOutputChunks};
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::Fault;
use semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES;

/// 📤 The download filename for one export: the active example's own id — `concrete-forest.json`,
/// `nakagin-capsule-tower.json` — so exporting two different examples never lands as two files with
/// one name. A document that came from no example (blank, or already imported from elsewhere) keeps
/// the app-generic `puzzle-3d.json`.
pub fn puzzle3d_export_filename(active_example_id: &str) -> String {
    if active_example_id.is_empty() {
        return "puzzle-3d.json".into();
    }
    format!("{active_example_id}.json")
}

/// 📏️ Largest export payload that may ride INLINE inside one `Effect::DownloadMediaExport`.
///
/// 🧊️ Derived, never a literal: it is the guest's own per-request contiguous ceiling
/// ([`GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES`], one wasm page — `🧮️memory/🦀️.rs`'s own docstring:
/// "Largest CONTIGUOUS block a routine, per-command or per-turn guest path may request"). An inline
/// effect payload is exactly such a per-command block, and the measured pair sits astride this line:
/// Concrete Forest's 7 542 B export downloads, Nakagin Capsule Tower's 145 714 B export leaves the
/// guest (`performInvocation settled … "effects":1`) and reaches no file
/// (`📓️2026-09-12-wave-B36-full-run-bisect-2.md` §5). Above it the payload goes through the
/// framework's segmented-download lane instead, which is what that lane exists for.
pub const fn puzzle3d_export_inline_budget_bytes() -> usize {
    GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES
}

/// 📤 The exported bytes of one fixture — the same projection `importFixture` round-trips.
pub fn puzzle3d_export_json(fixture: &Puzzle3dFixture) -> String {
    to_json_string(&puzzle3d_projection_value(dsl::ToValue::to_value(fixture)))
}

/// 📤 One inline host download. Only legal at or under [`puzzle3d_export_inline_budget_bytes`].
pub fn puzzle3d_export_inline_effect(filename: String, data: String) -> Effect {
    Effect::DownloadMediaExport { filename, mime_type: "application/json".into(), data, encoding: Some("utf-8".into()) }
}

/// 📤 The same download as a SEGMENTED output: the payload sliced into the wire's own
/// [`ArtifactOutputChunks::CHUNK_BYTES`] pages and sealed, so the host drains it one bounded chunk per
/// awaited turn (`📤️SegmentedDownload/🟦️.ts` `drainSegmentedMediaExport`) instead of receiving the whole
/// JSON as one inline effect field.
///
/// 🧾️ `identity` encoding, not `base64`: the payload is UTF-8 JSON, and the drain writes identity chunks
/// straight through. The cap is this command's OWN declared contract output budget
/// ([`PUZZLE_COMMAND_OUTPUT_BYTES`]) — a fixture larger than what the tool declared it may produce must
/// fault here, in the producer, rather than be silently truncated on the wire.
pub fn puzzle3d_export_segmented(filename: String, data: &str) -> Result<ArtifactDownloadOutput, Fault> {
    if data.len() > PUZZLE_COMMAND_OUTPUT_BYTES {
        return Err(Fault::from("puzzle3d-export-exceeds-declared-output-budget"));
    }
    let chunks = ArtifactOutputChunks::new(PUZZLE_COMMAND_OUTPUT_BYTES);
    for page in data.as_bytes().chunks(ArtifactOutputChunks::CHUNK_BYTES) {
        chunks.push(page.to_vec())?;
    }
    chunks.seal()?;
    ArtifactDownloadOutput::new(filename, "application/json", Some("identity".into()), chunks)
}

/// 📤 What one `exportFixture` publishes: the inline effect for a payload that fits one wire page, the
/// segmented output for anything larger.
pub enum Puzzle3dExportPublication {
    Inline(Effect),
    Segmented(ArtifactDownloadOutput),
}

/// 📤 Resolves the publication for one fixture without touching any emission lane — the one place the
/// budget decision lives, so the interactive job and the leftover reducer arm cannot disagree about it.
pub fn puzzle3d_export_publication(fixture: &Puzzle3dFixture, active_example_id: &str) -> Result<Puzzle3dExportPublication, Fault> {
    let filename = puzzle3d_export_filename(active_example_id);
    let data = puzzle3d_export_json(fixture);
    if data.len() <= puzzle3d_export_inline_budget_bytes() {
        return Ok(Puzzle3dExportPublication::Inline(puzzle3d_export_inline_effect(filename, data)));
    }
    puzzle3d_export_segmented(filename, &data).map(Puzzle3dExportPublication::Segmented)
}

/// 📤 Emits a host download of the current fixture JSON.
///
/// 🧾️ This is the LEFTOVER (non-interactive) arm, which owns no typed-operation completion authority and
/// therefore cannot publish a segmented output: it pushes the inline effect and, above the budget, a
/// notice naming the lane the interactive route takes instead — silence is the one answer a download must
/// never give.
pub fn export_fixture(ctx: &mut Puzzle3dActionCtx<'_>) {
    let filename = puzzle3d_export_filename(&ctx.scene.runtime.active_example_id);
    let data = puzzle3d_export_json(&ctx.scene.fixture);
    if data.len() > puzzle3d_export_inline_budget_bytes() {
        ctx.effects.push(Effect::Notify { message: format!("{filename} is {} B, over the inline download budget of {} B — export it through the window action, which streams it", data.len(), puzzle3d_export_inline_budget_bytes()) });
        return;
    }
    ctx.effects.push(puzzle3d_export_inline_effect(filename, data));
}
