
use super::*;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn channel(seed: f32, len: usize) -> SemioAudioChannel {
    SemioAudioChannel { samples: (0..len).map(|i| seed + i as f32 * 0.1).collect() }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn base_snapshot() -> SemioAudioSnapshot {
    SemioAudioSnapshot {
        sample_rate: 44_100,
        format: SemioAudioFormat::Pcm16,
        channels: vec![channel(0.0, 4), channel(1.0, 4), channel(2.0, 4)],
        tags: vec![SemioAudioTag { key: "title".into(), value: "one".into() }],
        ..SemioAudioSnapshot::default()
    }
}

/// 🧪️ Canonical absorb case 1: `InsertChannel(2,c)` then `RemoveChannel(0)` →
/// `{removed:[0], added:[(1,c)]}`.
#[semio_framework_async_macros::async_test]
async fn absorb_insert_then_remove_before_shifts_index() {
    let c = channel(9.0, 2);
    let mut d1: SemioAudioChannelsDiff = IndexedTripleDiff { added: vec![IndexAdded { index: 2, item: c.clone() }], ..Default::default() };
    let d2: SemioAudioChannelsDiff = IndexedTripleDiff { removed: vec![0], ..Default::default() };
    channels_absorb(&mut d1, d2);
    assert_eq!(d1.removed, vec![0]);
    assert_eq!(d1.added, vec![IndexAdded { index: 1, item: c }]);
    assert!(d1.modified.is_empty());
}

/// 🧪️ Canonical absorb case 2: `InsertChannel(2,c)` then `InsertChannel(2,d)` → BOTH survive.
#[semio_framework_async_macros::async_test]
async fn absorb_insert_insert_same_index_both_survive() {
    let c = channel(1.0, 2);
    let d = channel(2.0, 2);
    let mut d1: SemioAudioChannelsDiff = IndexedTripleDiff { added: vec![IndexAdded { index: 2, item: c.clone() }], ..Default::default() };
    let d2: SemioAudioChannelsDiff = IndexedTripleDiff { added: vec![IndexAdded { index: 2, item: d.clone() }], ..Default::default() };
    channels_absorb(&mut d1, d2);
    assert_eq!(d1.added, vec![IndexAdded { index: 2, item: d }, IndexAdded { index: 3, item: c }]);
}

/// 🧪️ Canonical absorb case 3: `InsertChannel(1,c)` then `SetChannelSamples(1,..)` patches
/// INTO the added payload — merged has only `added`, no separate `modified` entry.
#[semio_framework_async_macros::async_test]
async fn absorb_insert_then_set_field_patches_into_added() {
    let c = channel(1.0, 2);
    let mut d1: SemioAudioChannelsDiff = IndexedTripleDiff { added: vec![IndexAdded { index: 1, item: c.clone() }], ..Default::default() };
    let d2: SemioAudioChannelsDiff = IndexedTripleDiff { modified: vec![IndexModified { index: 1, diff: SemioAudioChannelDiff { samples: Some(vec![9.0, 9.0]) } }], ..Default::default() };
    channels_absorb(&mut d1, d2);
    assert!(d1.modified.is_empty());
    assert_eq!(d1.added.len(), 1);
    assert_eq!(d1.added[0].item.samples, vec![9.0, 9.0]);
    assert_eq!(d1.added[0].index, 1);
}

#[semio_framework_async_macros::async_test]
async fn absorb_law_holds_over_curated_ops() {
    let base = base_snapshot();
    let mid = {
        let mut s = base.clone();
        s.channels.insert(1, channel(9.0, 4));
        s.channels.remove(0);
        s.tags.push(SemioAudioTag { key: "artist".into(), value: "a".into() });
        s
    };
    let after = {
        let mut s = mid.clone();
        s.channels[0].samples = vec![5.0, 5.0, 5.0, 5.0];
        s.channels.push(channel(5.0, 4));
        s.tags[0].value = "changed".into();
        s
    };
    let mut d1 = <SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::between(&base, &mid);
    let d2 = <SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::between(&mid, &after);
    d1.absorb(d2);
    assert_eq!(d1.apply(&base).expect("apply must succeed for a well-formed fixture"), after);
}

#[semio_framework_async_macros::async_test]
async fn between_roundtrip_law() {
    let a = base_snapshot();
    let mut b = base_snapshot();
    b.sample_rate = 48_000;
    b.channels.push(channel(3.0, 4));
    let ab = <SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::between(&a, &b);
    assert_eq!(ab.apply(&a).expect("apply must succeed for a well-formed fixture"), b);
    let ba = <SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::between(&b, &a);
    assert_eq!(ba.apply(&b).expect("apply must succeed for a well-formed fixture"), a);
    assert!(<SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::between(&a, &a).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn inverse_law() {
    let base = base_snapshot();
    let next = {
        let mut s = base.clone();
        s.channels[0].samples = vec![7.0, 7.0, 7.0, 7.0];
        s.channels.remove(1);
        s.channels.push(channel(6.0, 4));
        s.sample_rate = 22_050;
        s.tags.clear();
        s
    };
    let d = <SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::between(&base, &next);
    let mutated = d.apply(&base).expect("apply must succeed for a well-formed fixture");
    let inv = d.inverse(&base);
    assert_eq!(inv.apply(&mutated).expect("apply must succeed for a well-formed fixture"), base);
}

/// 🧪️ Field sweep — the acceptance criterion: `sweep_a`/`sweep_b` differ in EVERY mutable
/// field, with asymmetric collection lengths so both `removed` and `added` get exercised
/// (split across both directions, matching the recipe's own guidance).
#[semio_framework_async_macros::async_test]
async fn field_sweep_covers_every_mutable_field() {
    let sweep_a =
        SemioAudioSnapshot { sample_rate: 44_100, format: SemioAudioFormat::Pcm16, channels: vec![channel(0.0, 4), channel(1.0, 4)], tags: vec![SemioAudioTag { key: "title".into(), value: "first".into() }], ..SemioAudioSnapshot::default() };
    let sweep_b = SemioAudioSnapshot { sample_rate: 96_000, format: SemioAudioFormat::Float64, channels: vec![channel(9.0, 4), channel(1.0, 4), channel(2.0, 4)], tags: vec![], ..SemioAudioSnapshot::default() };

    let ab = <SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::between(&sweep_a, &sweep_b);
    assert_eq!(ab.apply(&sweep_a).expect("apply must succeed for a well-formed fixture"), sweep_b);
    assert!(ab.sample_rate.is_some());
    assert!(ab.format.is_some());
    let channels_ab = ab.channels.as_ref().expect("channels must differ");
    assert!(!channels_ab.modified.is_empty(), "sweep must exercise a modified channel");
    assert!(!channels_ab.added.is_empty(), "sweep must exercise an added channel (b is longer)");
    let tags_ab = ab.tags.as_ref().expect("tags must differ");
    assert!(!tags_ab.removed.is_empty(), "sweep must exercise a removed tag (b has none)");

    let ba = <SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::between(&sweep_b, &sweep_a);
    assert_eq!(ba.apply(&sweep_b).expect("apply must succeed for a well-formed fixture"), sweep_a);
    let channels_ba = ba.channels.as_ref().expect("channels must differ");
    assert!(!channels_ba.removed.is_empty(), "reverse direction must exercise a removed channel (a is shorter)");
    let tags_ba = ba.tags.as_ref().expect("tags must differ");
    assert!(!tags_ba.added.is_empty(), "reverse direction must exercise an added tag");

    assert!(<SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::between(&sweep_a, &sweep_a).is_empty());
}

/// 🧪️ `DiffCodec` text/binary round-trip law — exercises scalars and both collection triples
/// (`removed`/`modified`/`added`) simultaneously via a real `between()` result.
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    let a = base_snapshot();
    let mut b = base_snapshot();
    b.sample_rate = 48_000;
    b.format = SemioAudioFormat::Float32;
    b.channels[0].samples = vec![9.9, 8.8];
    b.channels.remove(1);
    b.channels.push(channel(4.0, 3));
    b.tags.push(SemioAudioTag { key: "artist".into(), value: "someone".into() });

    let cases = vec![SemioAudioDiff::default(), <SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::between(&a, &b), <SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::between(&b, &a)];
    for d in cases {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = SemioAudioDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = SemioAudioDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}

#[semio_framework_async_macros::async_test]
async fn snapshot_bracket_codec_round_trips() {
    let s = base_snapshot();
    let encoded = enc_snapshot(&s);
    let decoded = dec_snapshot(&encoded).expect("decode");
    assert_eq!(decoded, s);
}
