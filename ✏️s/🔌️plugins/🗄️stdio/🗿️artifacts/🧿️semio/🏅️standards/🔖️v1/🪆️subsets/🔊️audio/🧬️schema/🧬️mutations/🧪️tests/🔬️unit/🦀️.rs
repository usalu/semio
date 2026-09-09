use super::*;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_channel(seed: f32) -> SemioAudioChannel {
    SemioAudioChannel { samples: vec![seed, seed + 1.0, seed + 2.0] }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn base_snapshot() -> SemioAudioSnapshot {
    SemioAudioSnapshot {
        sample_rate: 44_100,
        format: SemioAudioFormat::Pcm16,
        channels: vec![sample_channel(1.0), sample_channel(2.0), sample_channel(3.0)],
        tags: vec![SemioAudioTag { key: "title".into(), value: "t0".into() }],
        ..SemioAudioSnapshot::default()
    }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn round_trips(base: &SemioAudioSnapshot, mutation: SemioAudioMutation) {
    let diff = mutation.diff(base);
    let mutated = <SemioAudioDiff as protocol::MutationDiff<SemioAudioSnapshot>>::apply(diff.diff(), base).expect("apply must succeed for a well-formed fixture");
    let inverses = mutation.inverse(base);
    let mut restored = mutated.clone();
    for inv in &inverses {
        let inv_diff = inv.diff(&restored);
        restored = <SemioAudioDiff as protocol::MutationDiff<SemioAudioSnapshot>>::apply(inv_diff.diff(), &restored).expect("apply must succeed for a well-formed fixture");
    }
    assert_eq!(&restored, base, "apply(inverse(m), apply(m, base)) must recover base for {mutation:?}");
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn all_variants(base: &SemioAudioSnapshot) -> Vec<SemioAudioMutation> {
    vec![
        SemioAudioMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: SemioAudioSnapshot { sample_rate: 9_000, ..base.clone() } }),
        SemioAudioMutation::SetSampleRate(set_sample_rate::SetSampleRate { sample_rate: 48_000 }),
        SemioAudioMutation::SetFormat(set_format::SetFormat { format: SemioAudioFormat::Float32 }),
        SemioAudioMutation::InsertChannel(insert_channel::InsertChannel { index: 1, channel: sample_channel(9.0) }),
        SemioAudioMutation::RemoveChannel(remove_channel::RemoveChannel { index: 1 }),
        SemioAudioMutation::SetChannelSamples(set_channel_samples::SetChannelSamples { index: 0, samples: vec![0.25, 0.5, 0.75] }),
        SemioAudioMutation::InsertTag(insert_tag::InsertTag { index: 0, tag: SemioAudioTag { key: "artist".into(), value: "a".into() } }),
        SemioAudioMutation::RemoveTag(remove_tag::RemoveTag { index: 0 }),
        SemioAudioMutation::SetTagValue(set_tag_value::SetTagValue { index: 0, value: "changed".into() }),
    ]
}

/// 🧪️ `mutation_diff_law`: every variant's `diff()` matches what `apply_semio_audio_mutation`
/// returns.
#[semio_framework_async_macros::async_test]
async fn mutation_diff_law() {
    let base = base_snapshot();
    for mutation in all_variants(&base) {
        let mut snap = base.clone();
        let returned_diff = apply_semio_audio_mutation(&mut snap, &mutation);
        let expected_diff = mutation.diff(&base);
        assert_eq!(returned_diff, expected_diff, "returned diff must equal mutation.diff(base) for {mutation:?}");
        assert_eq!(
            snap,
            <SemioAudioDiff as protocol::MutationDiff<SemioAudioSnapshot>>::apply(expected_diff.diff(), &base).expect("apply must succeed for a well-formed fixture"),
            "apply_semio_audio_mutation must match diff.diff().apply(base) for {mutation:?}"
        );
    }
}

/// 🧪️ `inverse_law` (mutation-level): every variant round-trips.
#[semio_framework_async_macros::async_test]
async fn mutation_apply_inverse_round_trips_every_variant() {
    let base = base_snapshot();
    for mutation in all_variants(&base) {
        round_trips(&base, mutation);
    }
}

#[semio_framework_async_macros::async_test]
async fn remove_channel_out_of_range_is_noop_not_panic() {
    let base = base_snapshot();
    let mut snap = base.clone();
    apply_semio_audio_mutation(&mut snap, &SemioAudioMutation::RemoveChannel(remove_channel::RemoveChannel { index: 99 }));
    assert_eq!(snap, base);
}

/// 🧪️ `kinds_match_the_enum_and_the_catalog`: `KINDS` names every declared variant, in the
/// declaration order `variant_ordinal` assigns and the spelling `print_audio_mutation` emits,
/// and every one of those names also appears in the committed oracle manifest's catalog. The
/// bijection against `all_variants` is what makes a newly added variant fail here instead of
/// silently shrinking the vocabulary the `🔊️mutate-semio-audio` case claims to cover.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    assert_eq!(KINDS, &OP_KEYWORDS[..], "KINDS must be exactly the op keyword table — one kebab-case name per declared variant, in declaration order");
    let base = base_snapshot();
    let mut seen = vec![false; KINDS.len()];
    for mutation in all_variants(&base) {
        let ordinal = variant_ordinal(&mutation) as usize;
        assert!(!seen[ordinal], "ordinal {ordinal} is represented twice — all_variants must carry exactly one case per declared variant");
        seen[ordinal] = true;
        assert_eq!(KINDS[ordinal], print_audio_mutation(&mutation).split(' ').next().unwrap_or_default(), "KINDS[{ordinal}] must be the keyword {mutation:?} prints");
    }
    assert!(seen.iter().all(|hit| *hit), "every declared variant must be represented in all_variants");
    let manifest = include_str!("../../../../🔮️oracle/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
}

/// 🧪️ `op_text_binary_roundtrip_law`: hand-rolled `OpText`/`OpBinary` round-trip over the
/// full variant vocabulary.
#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    let base = base_snapshot();
    for mutation in all_variants(&base) {
        let printed = mutation.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = SemioAudioMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

        let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
        let decoded = SemioAudioMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
        assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
    }
}
