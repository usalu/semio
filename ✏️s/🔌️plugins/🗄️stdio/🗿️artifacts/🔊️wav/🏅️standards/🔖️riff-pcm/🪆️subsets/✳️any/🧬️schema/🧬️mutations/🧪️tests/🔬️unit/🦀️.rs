use super::*;
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn base_snapshot() -> WavSnapshot {
    WavSnapshot { data: WavData::Pcm16(vec![10, -10, 5]), ..WavSnapshot::default() }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn variants(base: &WavSnapshot) -> Vec<WavMutation> {
    vec![
        WavMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: WavSnapshot { fmt: WavFmt { sample_rate: 48000, ..base.fmt.clone() }, ..base.clone() } }),
        WavMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch: semio_s_artifact_stdio_contract::editing::prepare_snapshot_patch(base, &semio_s_artifact_stdio_contract::editing::SnapshotEditEvent::SetValue { path: "/fmt/sampleRate".into(), value: dsl::DslValue::Number(dsl::Number::UInt(48_000)) }).unwrap() }),
        WavMutation::SetFmt(set_fmt::SetFmt { fmt: WavFmt { channels: 2, ..WavFmt::default() } }),
        WavMutation::SetData(set_data::SetData { data: WavData::Float32(vec![0.25, -0.25]) }),
        WavMutation::PatchData(patch_data::PatchData { index: 1, remove_count: 1, data: WavData::Pcm16(vec![42]), move_to: None }),
        WavMutation::SetOtherChunks(set_other_chunks::SetOtherChunks { chunks: vec![RiffChunk { fourcc: "fact".into(), data: vec![1, 2], pad_byte: 0 }] }),
    ]
}

//#region mutation_diff_law
/// 🧪️ `mutation.diff(base).diff().apply(base) == apply_wav_mutation(base, mutation)`.
#[semio_framework_async_macros::async_test]
async fn mutation_diff_law_every_variant() {
    let base = base_snapshot();
    for m in variants(&base) {
        let mut via_apply = base.clone();
        let returned = apply_wav_mutation(&mut via_apply, &m);
        let direct = m.diff(&base);
        assert_eq!(direct, returned, "diff mismatch for {m:?}");
        assert_eq!(direct.diff().apply(&base).unwrap(), via_apply, "apply mismatch for {m:?}");
    }
}
//#endregion mutation_diff_law

//#region inverse_law
/// 🧪️ Applying the inverse mutation restores base, at both the mutation and diff levels.
#[semio_framework_async_macros::async_test]
async fn inverse_law_mutation_and_diff_level() {
    let base = base_snapshot();
    for m in variants(&base) {
        let mut round = base.clone();
        apply_wav_mutation(&mut round, &m);
        for inv in m.inverse(&base) {
            apply_wav_mutation(&mut round, &inv);
        }
        assert_eq!(round, base, "mutation-level inverse failed for {m:?}");

        let d = m.diff(&base);
        let applied = d.diff().apply(&base).unwrap();
        let undone = d.diff().inverse(&base).apply(&applied).unwrap();
        assert_eq!(undone, base, "diff-level inverse failed for {m:?}");
    }
}
//#endregion inverse_law

//#region kinds_matches_enum_and_manifest
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn kind_of(m: &WavMutation) -> &'static str {
    match m {
        WavMutation::SetSnapshot(_) => "set-snapshot",
        WavMutation::PatchSnapshot(_) => "patch-snapshot",
        WavMutation::SetFmt(_) => "set-fmt",
        WavMutation::SetData(_) => "set-data",
        WavMutation::PatchData(_) => "patch-data",
        WavMutation::SetOtherChunks(_) => "set-other-chunks",
    }
}

/// 🧪️ `KINDS` must name exactly the enum's own variants (the match above is exhaustive, so a
/// variant added without updating `kind_of` fails to compile) AND every one of them must appear
/// in the mutation catalog's declared `kinds` — the framework never parses Rust, so this test is
/// what keeps the manifest honest. The check is containment, not equality: the manifest also
/// declares `no-mutation`, the identity scenario the `🥒️.feature` still exercises against the
/// independent oracle even though `NoMutation` is no longer an enum variant (`no` is not an
/// approved semantic verb for `#[derive(dsl::Mutations)]`) — the same split
/// `🎛️mutate-mp3-mpeg1-layer3`'s own `kinds_match_the_committed_catalog` makes for its sibling
/// vocabulary.
#[semio_framework_async_macros::async_test]
async fn kinds_matches_enum_variants_and_manifest() {
    let base = base_snapshot();
    let mut from_enum: Vec<&str> = variants(&base).iter().map(kind_of).collect();
    from_enum.sort_unstable();
    from_enum.dedup();
    let mut from_const = KINDS.to_vec();
    from_const.sort_unstable();
    assert_eq!(from_const, from_enum, "KINDS must name exactly the enum's variants");

    let manifest = include_str!("../../../../🔮️oracles/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "the oracle catalog manifest must declare kind {kind:?}");
    }
}
//#endregion kinds_matches_enum_and_manifest

//#region op_text_binary_roundtrip_law
#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    let base = base_snapshot();
    for m in variants(&base) {
        let printed = m.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = WavMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?}");

        let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
        let decoded = WavMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
        assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
    }
}
//#endregion op_text_binary_roundtrip_law

#[semio_framework_async_macros::async_test]
async fn text_descriptor_recognizes_every_printer_variant() {
    let base = base_snapshot();
    let grammar = dsl::parse_grammar(crate::standards::riff_pcm::subsets::any::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse WAV mutation grammar");
    let recognizer = dsl::Recognizer::compile(&grammar);
    for mutation in variants(&base) {
        let printed = mutation.print_op();
        assert!(recognizer.recognize(&printed).unwrap_or(false), "WAV mutation grammar did not recognize {printed:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn mutations_reject_states_that_cannot_round_trip_through_riff() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🧭️serialization-boundaries/🔣️.json")).expect("serialization boundary fixture is JSON");
    let invalid_even_pad = fixture["invalidEvenPadByte"].as_u64().expect("invalidEvenPadByte") as u8;
    let overflow_ext = fixture["overflowFmtExtensionBytes"].as_u64().expect("overflowFmtExtensionBytes") as usize;
    let valid_fourcc = fixture["validFourcc"].as_str().expect("validFourcc");
    let invalid_fourcc = fixture["invalidFourcc"].as_str().expect("invalidFourcc");
    let invalid_snapshots = [
        WavSnapshot { fmt_pad_byte: invalid_even_pad, ..base_snapshot() },
        WavSnapshot { data_pad_byte: invalid_even_pad, ..base_snapshot() },
        WavSnapshot {
            other_chunks: vec![RiffChunk { fourcc: valid_fourcc.into(), data: vec![1, 2], pad_byte: invalid_even_pad }],
            ..base_snapshot()
        },
        WavSnapshot { fmt: WavFmt { ext: Some(vec![0; overflow_ext]), ..WavFmt::default() }, ..base_snapshot() },
        WavSnapshot {
            other_chunks: vec![RiffChunk { fourcc: invalid_fourcc.into(), data: vec![1], pad_byte: 0 }],
            ..base_snapshot()
        },
    ];
    for invalid in invalid_snapshots {
        let mut current = base_snapshot();
        let outcome = apply_wav_mutation(&mut current, &WavMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: invalid }));
        assert!(!outcome.messages().is_empty(), "unrepresentable mutation must report a diagnostic");
        assert!(outcome.diff().is_empty(), "unrepresentable mutation must carry an empty diff");
        assert_eq!(current, base_snapshot(), "unrepresentable mutation must leave the document unchanged");
    }
}
