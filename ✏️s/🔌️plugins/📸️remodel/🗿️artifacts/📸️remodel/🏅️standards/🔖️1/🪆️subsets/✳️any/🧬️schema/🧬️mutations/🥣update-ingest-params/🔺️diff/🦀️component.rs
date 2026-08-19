//! 🔺️ Sparse diff builder for `UpdateIngestParams` — the field is always present, so there is no
//! missing-target case. A non-finite/negative sharpness gate or a zero frame count/stride ⇒ Fatal
//! `mutation.invariant`; identical params ⇒ Warning `mutation.no-op`.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::UpdateIngestParams, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
    if !payload.params.min_sharpness.is_finite() || payload.params.min_sharpness < 0.0 || payload.params.max_frames == 0 || payload.params.frame_sample_stride == 0 {
        return protocol::MutationOutcome::fatal(
            "mutation.invariant",
            format!(
                "Ingest params need a finite non-negative min sharpness and positive max frames/frame sample stride (got min_sharpness={}, max_frames={}, frame_sample_stride={}).",
                payload.params.min_sharpness, payload.params.max_frames, payload.params.frame_sample_stride
            ),
            Vec::<String>::new(),
        );
    }
    if payload.params == base.params.ingest {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Ingest params are unchanged.");
    }
    let mut params = base.params.clone();
    params.ingest = payload.params.clone();
    protocol::MutationOutcome::new(RemodelDiff { params: Some(params), ..Default::default() })
}
//#endregion 🔖️Diff
