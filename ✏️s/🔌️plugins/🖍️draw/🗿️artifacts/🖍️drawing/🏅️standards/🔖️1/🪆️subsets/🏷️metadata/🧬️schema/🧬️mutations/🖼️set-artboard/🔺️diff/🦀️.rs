//! 🔺️ Sparse diff builder for `SetArtboard`.
use semio_s_artifact_draw_drawing_diff::DrawingDiff;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;

//#region 🔖️Diff
/// 🔺️ One `artboard` field patch — a real sparse `DrawingDiff`, never apply-then-capture.
pub fn diff(payload: &super::mutation::SetArtboard, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
    if !super::mutation::valid(&payload.artboard) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Artboard dimensions must be finite and positive", ["artboard"]);
    }
    if payload.artboard == base.artboard {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The Drawing artboard already has these dimensions");
    }
    protocol::MutationOutcome::new(DrawingDiff { artboard: Some(super::mutation::copy(&payload.artboard)), ..Default::default() })
}
//#endregion 🔖️Diff
