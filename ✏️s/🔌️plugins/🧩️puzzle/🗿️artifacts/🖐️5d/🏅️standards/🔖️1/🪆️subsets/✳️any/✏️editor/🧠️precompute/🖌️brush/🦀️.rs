//! 🖌️ Puzzle 5d brush suggestions run job: the puzzle 3d brush suggestions search over the 5d document, behind the
//! planner bridge's translation (`🧠️precompute`). It follows the instance's brush suggestions link, which 5d's
//! instance operation owner carries exactly like puzzle 3d's.

use crate::editor::puzzle5d::modes::edit::windows::board2d::utilities::brush::UTILITY_ID;
use crate::editor::puzzle5d::precompute::{puzzle3d_config, puzzle3d_kind_catalogs, puzzle3d_snapshot, puzzle5d_authored_kind_catalogs, Puzzle5dPlannerBoard, Puzzle5dPlannerToolRunJob};
use crate::editor::puzzle5d::{Puzzle5dDocument, Puzzle5dFastener, Puzzle5dPart, Puzzle5dPlayApp};
use crate::standards::v1::subsets::any::schema::mutations::text::Puzzle5dPlaySnapshot;
use semio_framework_plugin::{EditorApp, Fault, ToolRunJob, ToolRunJobPurpose, ToolRunJobRequest};
use semio_s_artifact_puzzle_3d::editor::puzzle3d::modes::edit::windows::main::utilities::brush as brush3d;
use semio_s_artifact_puzzle_3d::editor::puzzle3d::precompute::brush::BrushSuggestionsFound;
use std::sync::Arc;

/// 🧵️ Builds the brush suggestions run job for the framework ledger; `None` for any other tool.
pub fn build_run_job(request: ToolRunJobRequest<'_, EditorApp<Puzzle5dPlayApp>>) -> Result<Option<ToolRunJob>, Fault> {
    if request.tool_id != UTILITY_ID || request.purpose != ToolRunJobPurpose::Run {
        return Ok(None);
    }
    let document: Puzzle5dDocument = serde_json::from_value(request.snapshot.value().clone()).map_err(|error| Fault::from(format!("puzzle5d-brush-run-document: {error}")))?;
    let board = Puzzle5dPlannerBoard::new(&document, &[])?;
    let snapshot = Arc::new(puzzle3d_snapshot(&document, puzzle5d_authored_kind_catalogs(&request.snapshot)?)?);
    let inner = brush3d::build_run_job(ToolRunJobRequest {
        tool_id: brush3d::UTILITY_ID,
        definition: request.definition,
        purpose: request.purpose,
        identity: request.identity,
        snapshot,
        config: Arc::new(puzzle3d_config(&request.config)),
        window_id: request.window_id,
        window_config: request.window_config,
        checkpoint: request.checkpoint,
        provisional: &[],
        instance_owner: request.instance_owner,
        port: request.port,
        trace_keys: request.trace_keys,
        entity_marks: request.entity_marks,
    })?
    .ok_or_else(|| Fault::from("puzzle5d-brush-run-search-missing"))?;
    Ok(Some(Box::new(Puzzle5dPlannerToolRunJob::new(inner, board))))
}

/// 🟢️ The part and fastener that place the `index`-th free candidate the brush run found (cycling), preferring
/// `part_kind` when given, posed exactly as the search posed it; `None` while the search found nothing free.
pub fn puzzle5d_brush_placement(snapshot: &Puzzle5dPlaySnapshot, document: &Puzzle5dDocument, found: &BrushSuggestionsFound, part_kind: Option<&str>, index: usize, part_id: String, fastener_id: String) -> Result<Option<(Puzzle5dPart, Puzzle5dFastener)>, Fault> {
    let free: Vec<_> = found.free().collect();
    let preferred: Vec<_> = free.iter().filter(|preview| part_kind.is_none_or(|kind| preview.object_kind_id == kind)).collect();
    let Some(preview) = (if preferred.is_empty() { None } else { Some(preferred[index % preferred.len()]) }) else { return Ok(None) };
    let catalogs = puzzle3d_kind_catalogs(document, puzzle5d_authored_kind_catalogs(snapshot)?)?;
    Ok(Some(Puzzle5dPlannerBoard::new(document, &[])?.adopt_suggestion(&catalogs, preview, part_id, fastener_id)?))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
