//! 🔺️ Sparse diff builder for `AddStreamFrame`. Guard order is the vocabulary's: a missing owner
//! stream ⇒ Error `mutation.target-missing`, then the invariant ⇒ Fatal `mutation.invariant`, then an
//! already-present exact frame ⇒ Warning `mutation.no-op`. `kind` ASSERTS the owner stream's media
//! kind rather than rewriting it — a stream's provenance is fixed when the stream is created, and a
//! verb that silently rewrote it had no inverse in this vocabulary. The frame lands at its canonical
//! `(index, asset_id)` position so `remove-stream-frame` puts it back exactly where it was.
use crate::artifacts::remodeling::diff::{RemodelingDiff, RemodelingMediaStreamList};
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::AddStreamFrame, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    let Some(stream) = base.streams.iter().find(|stream| stream.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Stream \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.kind != stream.kind {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Stream \"{}\" is not of the media kind this frame declares.", payload.id), [payload.id.clone()]);
    }
    if stream.frames.contains(&payload.frame) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Stream \"{}\" already has frame {}.", payload.id, payload.frame.index));
    }
    let mut streams = base.streams.clone();
    if let Some(stream) = streams.iter_mut().find(|stream| stream.id == payload.id) {
        let at = crate::artifacts::remodeling::mutations::ordered_index(&stream.frames, &(payload.frame.index, payload.frame.asset_id.clone()), |frame| (frame.index, frame.asset_id.clone()));
        stream.frames.insert(at, payload.frame.clone());
    }
    protocol::MutationOutcome::new(RemodelingDiff { streams: Some(RemodelingMediaStreamList { values: streams }), ..Default::default() })
}
//#endregion 🔖️Diff
