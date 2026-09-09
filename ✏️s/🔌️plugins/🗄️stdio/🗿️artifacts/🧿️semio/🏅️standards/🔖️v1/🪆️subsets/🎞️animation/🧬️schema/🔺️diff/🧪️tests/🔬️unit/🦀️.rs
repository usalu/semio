use super::*;

//#region AbsorbCanonical
/// 🧪️ Canonical absorb case 1 (at the innermost `keyframes` level): `Insert(2,f)` then
/// `Remove(0)` -> `{removed:[0], added:[(1,f)]}`.
#[semio_framework_async_macros::async_test]
async fn absorb_insert_then_remove_before_shifts_index() {
    let f = kf(9.0, AnimValue::Scalar { value: 9.0 });
    let mut d1: IndexedTripleDiff<AnimKeyframeDiff, AnimKeyframe> = IndexedTripleDiff { added: vec![IndexAdded { index: 2, item: f.clone() }], ..Default::default() };
    let d2: IndexedTripleDiff<AnimKeyframeDiff, AnimKeyframe> = IndexedTripleDiff { removed: vec![0], ..Default::default() };
    absorb_indexed(&mut d1, d2, |d, o| d.absorb(o), |d, item| d.apply(item));
    assert_eq!(d1.removed, vec![0]);
    assert_eq!(d1.added, vec![IndexAdded { index: 1, item: f }]);
    assert!(d1.modified.is_empty());
}

/// 🧪️ Canonical absorb case 2: `Insert(2,f)` then `Insert(2,g)` -> BOTH survive.
#[semio_framework_async_macros::async_test]
async fn absorb_insert_insert_same_index_both_survive() {
    let f = kf(1.0, AnimValue::Scalar { value: 1.0 });
    let g = kf(2.0, AnimValue::Scalar { value: 2.0 });
    let mut d1: IndexedTripleDiff<AnimKeyframeDiff, AnimKeyframe> = IndexedTripleDiff { added: vec![IndexAdded { index: 2, item: f.clone() }], ..Default::default() };
    let d2: IndexedTripleDiff<AnimKeyframeDiff, AnimKeyframe> = IndexedTripleDiff { added: vec![IndexAdded { index: 2, item: g.clone() }], ..Default::default() };
    absorb_indexed(&mut d1, d2, |d, o| d.absorb(o), |d, item| d.apply(item));
    assert_eq!(d1.added, vec![IndexAdded { index: 2, item: g }, IndexAdded { index: 3, item: f }]);
}

/// 🧪️ Canonical absorb case 3: `Insert(1,f)` then `SetField(1,v)` patches INTO the added
/// payload — no separate `modified` entry survives.
#[semio_framework_async_macros::async_test]
async fn absorb_insert_then_set_field_patches_into_added() {
    let f = kf(1.0, AnimValue::Scalar { value: 1.0 });
    let mut d1: IndexedTripleDiff<AnimKeyframeDiff, AnimKeyframe> = IndexedTripleDiff { added: vec![IndexAdded { index: 1, item: f.clone() }], ..Default::default() };
    let d2: IndexedTripleDiff<AnimKeyframeDiff, AnimKeyframe> = IndexedTripleDiff { modified: vec![IndexModified { index: 1, diff: AnimKeyframeDiff { t: Some(42.0), value: None } }], ..Default::default() };
    absorb_indexed(&mut d1, d2, |d, o| d.absorb(o), |d, item| d.apply(item));
    assert!(d1.modified.is_empty());
    assert_eq!(d1.added.len(), 1);
    assert_eq!(d1.added[0].item.t, 42.0);
    assert_eq!(d1.added[0].index, 1);
}
//#endregion AbsorbCanonical

/// 🧪️ absorb_law: full 3-level snapshot chain, base -> mid -> after.
#[semio_framework_async_macros::async_test]
async fn absorb_law_holds_over_curated_ops() {
    let base = SemioAnimationSnapshot {
        timelines: vec![timeline(Some("walk"), vec![channel("hip", AnimTargetProperty::Translation, AnimInterpolation::Linear, vec![kf(0.0, AnimValue::Vec3 { value: SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 } })])]), timeline(Some("blink"), vec![])],
        ..SemioAnimationSnapshot::default()
    };
    let mid = {
        let mut s = base.clone();
        s.timelines[0].channels[0].keyframes.push(kf(1.0, AnimValue::Vec3 { value: SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 } }));
        s.timelines.remove(1);
        s.timelines.push(timeline(Some("wave"), vec![]));
        s
    };
    let after = {
        let mut s = mid.clone();
        s.timelines[0].channels[0].interpolation = AnimInterpolation::CubicSpline;
        s.timelines[1].channels.push(channel("hand", AnimTargetProperty::Rotation, AnimInterpolation::Step, vec![kf(0.0, AnimValue::Quat { value: SemioQuaternion::default() })]));
        s
    };
    let mut d1 = <SemioAnimationDiff as DiffAlgebra<SemioAnimationSnapshot>>::between(&base, &mid);
    let d2 = <SemioAnimationDiff as DiffAlgebra<SemioAnimationSnapshot>>::between(&mid, &after);
    d1.absorb(d2);
    assert_eq!(d1.apply(&base).expect("apply must succeed for a well-formed fixture"), after);
}

#[semio_framework_async_macros::async_test]
async fn between_roundtrip_law() {
    let a = SemioAnimationSnapshot { timelines: vec![timeline(Some("walk"), vec![channel("hip", AnimTargetProperty::Translation, AnimInterpolation::Linear, vec![kf(0.0, AnimValue::Scalar { value: 1.0 })])])], ..SemioAnimationSnapshot::default() };
    let b = SemioAnimationSnapshot {
        timelines: vec![timeline(Some("walk"), vec![channel("hip", AnimTargetProperty::Translation, AnimInterpolation::Step, vec![kf(0.0, AnimValue::Scalar { value: 1.0 }), kf(1.0, AnimValue::Scalar { value: 2.0 })])]), timeline(None, vec![])],
        ..SemioAnimationSnapshot::default()
    };
    let ab = <SemioAnimationDiff as DiffAlgebra<SemioAnimationSnapshot>>::between(&a, &b);
    assert_eq!(ab.apply(&a).expect("apply must succeed for a well-formed fixture"), b);
    let ba = <SemioAnimationDiff as DiffAlgebra<SemioAnimationSnapshot>>::between(&b, &a);
    assert_eq!(ba.apply(&b).expect("apply must succeed for a well-formed fixture"), a);
    assert!(<SemioAnimationDiff as DiffAlgebra<SemioAnimationSnapshot>>::between(&a, &a).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn inverse_law() {
    let base = SemioAnimationSnapshot { timelines: vec![timeline(Some("walk"), vec![channel("hip", AnimTargetProperty::Translation, AnimInterpolation::Linear, vec![kf(0.0, AnimValue::Scalar { value: 1.0 })])])], ..SemioAnimationSnapshot::default() };
    let next = {
        let mut s = base.clone();
        s.timelines[0].name = None;
        s.timelines[0].channels[0].interpolation = AnimInterpolation::CubicSpline;
        s.timelines[0].channels[0].keyframes[0].value = AnimValue::Vec3 { value: SemioPoint3 { x: 1.0, y: 2.0, z: 3.0 } };
        s.timelines.push(timeline(Some("wave"), vec![]));
        s
    };
    let d = <SemioAnimationDiff as DiffAlgebra<SemioAnimationSnapshot>>::between(&base, &next);
    let mutated = d.apply(&base).expect("apply must succeed for a well-formed fixture");
    let inv = d.inverse(&base);
    assert_eq!(inv.apply(&mutated).expect("apply must succeed for a well-formed fixture"), base);
}

/// 🧪️ field_sweep — the acceptance criterion: `sweep_a`/`sweep_b` differ in every mutable
/// field at every nesting depth (timeline/channel/keyframe).await, including the `name` tri-state
/// and an `AnimValue` variant change, with a removed+modified collection member exercised at
/// SOME direction/level and an added+modified member exercised at the opposite
/// direction/level — `timelines`/`channels`/`keyframes` are ALL `IndexedTripleDiff`
/// (position-keyed, per `between_indexed`'s own truncating-tail semantics: an index only ever
/// falls in `removed` when `base` is strictly longer than `other` at that tail, or in `added`
/// when `other` is strictly longer — never both from the SAME `between()` call, at ANY
/// nesting level, since one collection can't simultaneously be longer AND shorter than the
/// other; W2b closer fix: the fixture previously kept every level's `sweep_a`/`sweep_b` pair
/// at EQUAL length, so neither `removed` nor `added` could ever be non-empty anywhere — only
/// `modified` — even though the doc comment already correctly named this exact structural
/// trap. Every level below now carries a genuine length asymmetry, deliberately alternating
/// which direction shows `removed` vs `added` per level (timelines: removed-forward/
/// added-reverse; channels: removed-forward/added-reverse; keyframes: added-forward, the
/// mirror case) — same split `presentation`'s own field_sweep test already uses for its
/// index-keyed `slides`, applied consistently at all 3 nesting depths here.
#[semio_framework_async_macros::async_test]
async fn field_sweep_covers_every_mutable_field() {
    let sweep_a = SemioAnimationSnapshot {
        timelines: vec![
            timeline(
                Some("kept"),
                vec![channel("kept-node", AnimTargetProperty::Translation, AnimInterpolation::Linear, vec![kf(9.0, AnimValue::Scalar { value: 9.0 })]), channel("gone-node", AnimTargetProperty::Weights, AnimInterpolation::Linear, vec![])],
            ),
            timeline(Some("filler"), vec![]),
            timeline(Some("gone"), vec![]),
        ],
        ..SemioAnimationSnapshot::default()
    };
    let sweep_b = SemioAnimationSnapshot {
        timelines: vec![
            timeline(None, vec![channel("kept-node", AnimTargetProperty::Rotation, AnimInterpolation::CubicSpline, vec![kf(1.0, AnimValue::Quat { value: SemioQuaternion::default() }), kf(2.0, AnimValue::Weights { values: vec![0.5, 0.5] })])]),
            timeline(Some("filler2"), vec![]),
        ],
        ..SemioAnimationSnapshot::default()
    };

    let ab = <SemioAnimationDiff as DiffAlgebra<SemioAnimationSnapshot>>::between(&sweep_a, &sweep_b);
    assert_eq!(ab.apply(&sweep_a).expect("apply must succeed for a well-formed fixture"), sweep_b);
    let timelines_ab = ab.timelines.as_ref().expect("timelines must differ");
    assert!(!timelines_ab.removed.is_empty(), "sweep must exercise a removed timeline");
    assert!(!timelines_ab.modified.is_empty(), "sweep must exercise a modified timeline");
    let timeline_diff = &timelines_ab.modified[0].diff;
    assert_eq!(timeline_diff.name, Some(None), "name Some->None must be tri-state Some(None)");
    let channels_ab = timeline_diff.channels.as_ref().expect("channels must differ");
    assert!(!channels_ab.removed.is_empty(), "sweep must exercise a removed channel");
    assert!(!channels_ab.modified.is_empty(), "sweep must exercise a modified channel");
    let channel_diff = &channels_ab.modified[0].diff;
    assert!(channel_diff.target.is_some());
    assert!(channel_diff.interpolation.is_some());
    let keyframes_ab = channel_diff.keyframes.as_ref().expect("keyframes must differ");
    assert!(!keyframes_ab.modified.is_empty(), "sweep must exercise a modified keyframe");
    assert!(!keyframes_ab.added.is_empty(), "sweep must exercise an added keyframe");
    let keyframe_diff = &keyframes_ab.modified[0].diff;
    assert!(keyframe_diff.t.is_some());
    assert!(keyframe_diff.value.is_some(), "AnimValue variant change must be captured");

    let ba = <SemioAnimationDiff as DiffAlgebra<SemioAnimationSnapshot>>::between(&sweep_b, &sweep_a);
    assert_eq!(ba.apply(&sweep_b).expect("apply must succeed for a well-formed fixture"), sweep_a);
    let timelines_ba = ba.timelines.as_ref().expect("timelines must differ");
    assert!(!timelines_ba.added.is_empty(), "reverse direction must exercise an added timeline");
    assert!(!timelines_ba.modified.is_empty(), "reverse direction must exercise a modified timeline");
    let timeline_diff_ba = &timelines_ba.modified[0].diff;
    let channels_ba = timeline_diff_ba.channels.as_ref().expect("channels must differ in reverse too");
    assert!(!channels_ba.added.is_empty(), "reverse direction must exercise an added channel");
    assert!(!channels_ba.modified.is_empty(), "reverse direction must exercise a modified channel");

    assert!(<SemioAnimationDiff as DiffAlgebra<SemioAnimationSnapshot>>::between(&sweep_a, &sweep_a).is_empty());
}

/// 🧪️ diff_codec_text_binary_roundtrip_law: hand-rolled `DiffCodec` text/binary grammar —
/// exercises the empty diff, the tri-state `name`, an `AnimValue` variant change, and all
/// three collection triples (removed/modified/added) at every nesting depth.
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    for d in demo_diff_cases() {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = SemioAnimationDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = SemioAnimationDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}
