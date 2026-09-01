//! 🔺️ Sparse diff builder for `DeleteStream` — a real cascade-aware removal (the stream's own
//! `frames` go with it automatically; any GCP observation addressing this stream is severed too).
//! Missing target ⇒ Error.
use crate::artifacts::remodel::diff::{RemodelDiff, RemodelGcpList, RemodelMediaStreamList};
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteStream, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
    if !base.streams.iter().any(|stream| stream.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Stream \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let streams: Vec<_> = base.streams.iter().filter(|stream| stream.id != payload.id).cloned().collect();
    let cascaded_observation_count: usize = base.gcps.iter().map(|gcp| gcp.observations.iter().filter(|observation| observation.stream_id == payload.id).count()).sum();
    let mut diff = RemodelDiff { streams: Some(RemodelMediaStreamList { values: streams }), ..Default::default() };
    if cascaded_observation_count > 0 {
        let gcps: Vec<_> = base
            .gcps
            .iter()
            .cloned()
            .map(|mut gcp| {
                gcp.observations.retain(|observation| observation.stream_id != payload.id);
                gcp
            })
            .collect();
        diff.gcps = Some(RemodelGcpList { values: gcps });
    }
    let outcome = protocol::MutationOutcome::new(diff);
    if cascaded_observation_count == 0 {
        outcome
    } else {
        outcome.info("mutation.cascade", format!("Deleting stream \"{}\" also removed {} GCP observation(s).", payload.id, cascaded_observation_count))
    }
}
//#endregion 🔖️Diff
