use super::*;
use crate::default_remodeling_scene;
use crate::{FrameRef, MediaKind};
use protocol::os_spr::protocol_laws::{assert_diff_algebra_between_law, assert_diff_algebra_inverse_law, assert_mutation_diff_absorb_law};

fn stream(id: &str) -> MediaStream {
    MediaStream { id: id.into(), name: id.into(), kind: MediaKind::ImageSequence, camera_id: None, sync_offset_ms: 0.0, fps_hint: 30.0, frames: vec![FrameRef { index: 1, timestamp_ms: 33.0, asset_id: "a1".into() }], source: None }
}

fn scene_with_streams(ids: &[&str]) -> RemodelingSnapshot {
    RemodelingSnapshot { streams: ids.iter().map(|id| stream(id)).collect(), ..default_remodeling_scene() }
}

fn sync(id: &str, offset: f64) -> RemodelingDiff {
    RemodelingDiff::stream_rows(vec![RemodelingRow::Patch { key: id.into(), patch: MediaStreamPatch { sync_offset_ms: Some(offset), ..Default::default() } }])
}

fn frames(id: &str, removed: Vec<FrameRef>, added: Vec<FrameRef>) -> RemodelingDiff {
    RemodelingDiff::stream_rows(vec![RemodelingRow::Patch { key: id.into(), patch: MediaStreamPatch { frames: Some(RemodelingMembers { removed, added }), ..Default::default() } }])
}

fn frame(index: u32) -> FrameRef {
    FrameRef { index, timestamp_ms: f64::from(index) * 33.0, asset_id: "a1".into() }
}

#[semio_framework_async_macros::async_test]
async fn empty_diff_is_identity_and_absorb_is_fieldwise_last_writer() {
    let scene = default_remodeling_scene();
    assert_eq!(protocol::apply_diff(&RemodelingDiff::default(), &scene).expect("valid identity diff"), scene);

    let mut diff = RemodelingDiff::default();
    diff.absorb(RemodelingDiff::gcp_rows(Vec::new()));
    assert!(diff.gcps.is_some());
    diff.absorb(RemodelingDiff::default());
    assert!(diff.gcps.is_some(), "absorbing empty never clobbers a real entry");
}

/// 🧩 Keyed rows apply in key order: an insert lands at the key's ordered position, a replace keeps it, a remove drops it, a patch writes the addressed member only.
#[semio_framework_async_macros::async_test]
async fn rows_apply_at_their_ordered_position() {
    let base = scene_with_streams(&["b", "d"]);
    let mut diff = RemodelingDiff::stream_rows(vec![RemodelingRow::Insert { entity: stream("c") }]);
    diff.absorb(RemodelingDiff::stream_rows(vec![RemodelingRow::Insert { entity: stream("a") }]));
    diff.absorb(sync("d", 5.0));
    let next = protocol::apply_diff(&diff, &base).expect("diff applies");
    assert_eq!(next.streams.iter().map(|stream| stream.id.as_str()).collect::<Vec<_>>(), vec!["a", "b", "c", "d"]);
    assert_eq!(next.streams[3].sync_offset_ms, 5.0);
}

/// 🚫️ Malformed rows are rejected with a typed error.
#[semio_framework_async_macros::async_test]
async fn malformed_rows_are_rejected() {
    let base = scene_with_streams(&["b"]);
    for diff in [
        RemodelingDiff::stream_rows(vec![RemodelingRow::Insert { entity: stream("b") }]),
        RemodelingDiff::stream_rows(vec![RemodelingRow::Replace { entity: stream("x") }]),
        RemodelingDiff::stream_rows(vec![RemodelingRow::Remove { key: "x".into() }]),
        sync("x", 1.0),
        frames("b", vec![frame(9)], Vec::new()),
        RemodelingDiff::content_rows(vec![RemodelingContentRow::Truncate { id: "ghost".into(), from: 0 }]),
    ] {
        assert!(protocol::apply_diff(&diff, &base).is_err(), "{diff:?} must be rejected");
    }
}

/// ➕️ Absorb coalesces same-key rows: insert∘patch folds into the insert, insert∘remove cancels, patch∘patch merges per slot, remove∘insert is a replace, patch∘remove is a remove, and member edits cancel.
#[semio_framework_async_macros::async_test]
async fn absorb_coalesces_rows_of_one_key() {
    let mut created = RemodelingDiff::stream_rows(vec![RemodelingRow::Insert { entity: stream("x") }]);
    created.absorb(sync("x", 7.0));
    assert_eq!(created.streams.as_ref().expect("streams delta").rows, vec![RemodelingRow::Insert { entity: MediaStream { sync_offset_ms: 7.0, ..stream("x") } }], "insert∘patch folds into the insert");
    created.absorb(RemodelingDiff::stream_rows(vec![RemodelingRow::Remove { key: "x".into() }]));
    assert_eq!(created.streams, Some(RemodelingRows::default()), "insert∘remove cancels");

    let mut patched = sync("a", 1.0);
    patched.absorb(sync("a", 2.0));
    patched.absorb(frames("a", Vec::new(), vec![frame(2)]));
    patched.absorb(frames("a", vec![frame(2)], vec![frame(3)]));
    let rows = patched.streams.expect("streams delta").rows;
    assert_eq!(rows.len(), 1, "patch∘patch of one member is one row");
    assert_eq!(rows[0], RemodelingRow::Patch { key: "a".into(), patch: MediaStreamPatch { sync_offset_ms: Some(2.0), frames: Some(RemodelingMembers { removed: Vec::new(), added: vec![frame(3)] }), ..Default::default() } }, "the member edits cancel and merge");

    let mut replaced = RemodelingDiff::stream_rows(vec![RemodelingRow::Remove { key: "a".into() }]);
    replaced.absorb(RemodelingDiff::stream_rows(vec![RemodelingRow::Insert { entity: stream("a") }]));
    assert!(matches!(replaced.streams.expect("streams delta").rows.as_slice(), [RemodelingRow::Replace { .. }]), "remove∘insert is a replace");

    let mut removed = sync("a", 1.0);
    removed.absorb(RemodelingDiff::stream_rows(vec![RemodelingRow::Remove { key: "a".into() }]));
    assert!(matches!(removed.streams.expect("streams delta").rows.as_slice(), [RemodelingRow::Remove { .. }]), "patch∘remove is a remove");

    let mut content = RemodelingDiff::content_rows(vec![RemodelingContentRow::Append { id: "c".into(), header: None, chunks: Vec::new() }]);
    content.absorb(RemodelingDiff::content_rows(vec![RemodelingContentRow::Append { id: "c".into(), header: None, chunks: vec![ByteBuffer::from_u8_slice(b"x")] }]));
    assert_eq!(content.durable_artifacts.expect("content delta").rows.len(), 1, "contiguous appends to one artifact merge");
}

/// ➕️ LAW: `absorb(d1, d2)` applies like `d1` then `d2` over the row kinds this vocabulary builds.
#[semio_framework_async_macros::async_test]
async fn absorb_equals_sequential_application() {
    let base = scene_with_streams(&["a", "b"]);
    let firsts = [sync("a", 1.0), RemodelingDiff::stream_rows(vec![RemodelingRow::Insert { entity: stream("c") }]), RemodelingDiff::stream_rows(vec![RemodelingRow::Remove { key: "a".into() }]), frames("a", vec![frame(1)], Vec::new())];
    let seconds = [
        sync("a", 2.0),
        sync("c", 3.0),
        frames("a", Vec::new(), vec![frame(4)]),
        frames("a", Vec::new(), vec![frame(1)]),
        RemodelingDiff::stream_rows(vec![RemodelingRow::Remove { key: "c".into() }]),
        RemodelingDiff::stream_rows(vec![RemodelingRow::Insert { entity: stream("a") }]),
    ];
    for d1 in &firsts {
        let mid = protocol::apply_diff(d1, &base).expect("first diff applies");
        for d2 in &seconds {
            if protocol::apply_diff(d2, &mid).is_ok() {
                assert_mutation_diff_absorb_law(&base, d1.clone(), d2.clone()).await;
            }
        }
    }
}

/// 🔁️ LAW: the negative delta applied after the diff restores the base — streams, member edits, params facets, result slots and content rows included.
#[semio_framework_async_macros::async_test]
async fn the_negative_delta_restores_the_base() {
    let mut base = scene_with_streams(&["a", "b"]);
    base.durable_artifacts.insert("held".into(), crate::RemodelingDurableArtifact { kind: "sparse".into(), mime: None, width: 0, height: 0, chunks: vec![ByteBuffer::from_u8_slice(b"1"), ByteBuffer::from_u8_slice(b"2")] });
    let mut diff = RemodelingDiff::stream_rows(vec![RemodelingRow::Remove { key: "a".into() }]);
    diff.absorb(RemodelingDiff::stream_rows(vec![RemodelingRow::Insert { entity: stream("z") }]));
    diff.absorb(frames("b", vec![frame(1)], vec![frame(5)]));
    diff.absorb(sync("b", 9.0));
    diff.absorb(RemodelingDiff::content_rows(vec![RemodelingContentRow::Truncate { id: "held".into(), from: 1 }, RemodelingContentRow::Append { id: "fresh".into(), header: Some(RemodelingContentHeader { kind: "sparse".into(), mime: None, width: 0, height: 0 }), chunks: vec![ByteBuffer::from_u8_slice(b"3")] }]));
    diff.absorb(RemodelingDiff { params: Some(RemodelingParamsDiff { sfm: Some(crate::SfmParams::default()), ..Default::default() }), results: Some(RemodelingResultsDiff { tracks: Some(Vec::new()), qc: Some(RemodelingAssigned::new(None)), ..Default::default() }), ..Default::default() });
    assert_diff_algebra_inverse_law::<RemodelingSnapshot, RemodelingDiff>(&base, &diff).await;
}

/// 🧭️ LAW: `between(a, b)` carries `a` to `b` and `between(a, a)` is empty.
#[semio_framework_async_macros::async_test]
async fn between_reaches_the_other_snapshot() {
    let a = scene_with_streams(&["a", "b", "c"]);
    let mut b = scene_with_streams(&["b", "d"]);
    b.streams[0].sync_offset_ms = 4.0;
    b.params.ingest.max_frames = 77;
    b.durable_artifacts.insert("fresh".into(), crate::RemodelingDurableArtifact { kind: "sparse".into(), mime: None, width: 0, height: 0, chunks: vec![ByteBuffer::from_u8_slice(b"x")] });
    assert_diff_algebra_between_law::<RemodelingSnapshot, RemodelingDiff>(&a, &b).await;
}
