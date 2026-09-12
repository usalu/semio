use super::*;
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn fixture() -> SemioVideoSnapshot {
    SemioVideoSnapshot {
        schema: crate::standards::v1::subsets::video::schema::snapshot::STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(),
        streams: vec![
            SemioVideoStream {
                kind: SemioVideoStreamKind::Video,
                codec: "h264".into(),
                width: 1920,
                height: 1080,
                rate: SemioRational { num: 30, den: 1 },
                samples: vec![SemioVideoSample { pts: 0, key: true, data: vec![1, 2, 3] }, SemioVideoSample { pts: 33, key: false, data: vec![4, 5, 6] }],
            },
            SemioVideoStream { kind: SemioVideoStreamKind::Audio, codec: "aac".into(), width: 0, height: 0, rate: SemioRational { num: 48_000, den: 1_000 }, samples: Vec::new() },
        ],
    }
}

//#region 🔖️Fixtures
/// 🌱 `sweep_a`/`sweep_b` — differ in EVERY mutable field, at BOTH nesting levels. `streams`
/// uses different-length lists so the recipe's naive positional `between_indexed` shows
/// removed+modified simultaneously in one direction (a removed tail, a modified-in-every-field
/// first stream whose OWN nested `samples` shows removed+modified too) and added in the
/// reverse direction (the dropped stream, plus that same modified stream's nested `samples`
/// added) — same "known structural trap" technique docx's own sweep fixtures use.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_a() -> SemioVideoSnapshot {
    SemioVideoSnapshot {
        schema: crate::standards::v1::subsets::video::schema::snapshot::STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(),
        streams: vec![
            SemioVideoStream {
                kind: SemioVideoStreamKind::Video,
                codec: "old-codec".into(),
                width: 640,
                height: 480,
                rate: SemioRational { num: 24, den: 1 },
                samples: vec![SemioVideoSample { pts: 1, key: true, data: vec![9] }, SemioVideoSample { pts: 2, key: false, data: vec![8] }, SemioVideoSample { pts: 3, key: true, data: vec![7] }],
            },
            SemioVideoStream { kind: SemioVideoStreamKind::Audio, codec: "aac".into(), width: 0, height: 0, rate: SemioRational { num: 1, den: 1 }, samples: Vec::new() },
            SemioVideoStream { kind: SemioVideoStreamKind::Subtitle, codec: "srt".into(), width: 0, height: 0, rate: SemioRational { num: 1, den: 1 }, samples: Vec::new() },
        ],
    }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_b() -> SemioVideoSnapshot {
    SemioVideoSnapshot {
        schema: crate::standards::v1::subsets::video::schema::snapshot::STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(),
        streams: vec![
            SemioVideoStream {
                kind: SemioVideoStreamKind::Audio,
                codec: "new-codec".into(),
                width: 1280,
                height: 720,
                rate: SemioRational { num: 30, den: 1 },
                samples: vec![SemioVideoSample { pts: 1, key: true, data: vec![9] }, SemioVideoSample { pts: 22, key: true, data: vec![80] }],
            },
            SemioVideoStream { kind: SemioVideoStreamKind::Audio, codec: "aac".into(), width: 0, height: 0, rate: SemioRational { num: 1, den: 1 }, samples: Vec::new() },
        ],
    }
}
//#endregion 🔖️Fixtures

//#region 🔖️KindsLaw
/// 🧪️ `kinds_match_the_enum_and_the_catalog`: `KINDS` names every declared variant, in the
/// declaration order `variant_ordinal` assigns and the spelling `print_semio_video_mutation`
/// emits, and every one of those names also appears in the committed oracle manifest's
/// catalog. The bijection against `sample_mutations` is what makes a newly added variant fail
/// here instead of silently shrinking the vocabulary `🎥️mutate-semio-video` claims to cover.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    assert_eq!(KINDS, &OP_KEYWORDS[..], "KINDS must be exactly the op keyword table — one kebab-case name per declared variant, in declaration order");
    let mut seen = vec![false; KINDS.len()];
    for mutation in sample_mutations() {
        let ordinal = variant_ordinal(&mutation) as usize;
        assert!(!seen[ordinal], "ordinal {ordinal} is represented twice — sample_mutations must carry exactly one case per declared variant");
        seen[ordinal] = true;
        assert_eq!(KINDS[ordinal], print_semio_video_mutation(&mutation).split(' ').next().unwrap_or_default(), "KINDS[{ordinal}] must be the keyword {mutation:?} prints");
    }
    assert!(seen.iter().all(|hit| *hit), "every declared variant must be represented in sample_mutations");
    let manifest = include_str!("../../../../🔮️oracles/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
}
//#endregion 🔖️KindsLaw

//#region 🔖️MutationDiffLaw
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub(crate) fn sample_mutations() -> Vec<SemioVideoMutation> {
    vec![
        SemioVideoMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: sweep_b() }),
        SemioVideoMutation::InsertStream(insert_stream::InsertStream {
            index: 1,
            stream: SemioVideoStream { kind: SemioVideoStreamKind::Subtitle, codec: "srt".into(), width: 0, height: 0, rate: SemioRational { num: 1, den: 1 }, samples: Vec::new() },
        }),
        SemioVideoMutation::RemoveStream(remove_stream::RemoveStream { index: 0 }),
        SemioVideoMutation::SetStreamMeta(set_stream_meta::SetStreamMeta { index: 0, kind: SemioVideoStreamKind::Audio, codec: "vp9".into(), width: 1280, height: 720, rate: SemioRational { num: 60, den: 1 } }),
        SemioVideoMutation::InsertSample(insert_sample::InsertSample { stream_index: 0, index: 1, sample: SemioVideoSample { pts: 99, key: true, data: vec![9, 9] } }),
        SemioVideoMutation::RemoveSample(remove_sample::RemoveSample { stream_index: 0, index: 0 }),
        SemioVideoMutation::SetSampleData(set_sample_data::SetSampleData { stream_index: 0, index: 0, data: vec![42] }),
        SemioVideoMutation::SetSampleFlags(set_sample_flags::SetSampleFlags { stream_index: 0, index: 0, pts: 500, key: true }),
    ]
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn apply_valid(diff: &SemioVideoDiff, base: &SemioVideoSnapshot) -> SemioVideoSnapshot {
    MutationDiff::apply(diff, base).expect("valid Semio video diff fixture")
}

#[semio_framework_async_macros::async_test]
async fn mutation_diff_law() {
    for mutation in sample_mutations() {
        let base = fixture();
        let diff_direct = Mutation::diff(&mutation, &base);
        let applied_via_diff = apply_valid(diff_direct.diff(), &base);

        let mut via_apply = base.clone();
        let diff_from_apply = apply_semio_video_mutation(&mut via_apply, &mutation);

        assert_eq!(applied_via_diff, via_apply, "mutation_diff_law: apply mismatch for {mutation:?}");
        assert_eq!(diff_direct, diff_from_apply, "mutation_diff_law: diff mismatch for {mutation:?}");
    }
}
//#endregion 🔖️MutationDiffLaw

//#region 🔖️InverseLaw
#[semio_framework_async_macros::async_test]
async fn inverse_law() {
    for mutation in sample_mutations() {
        let base = fixture();

        let mut round_tripped = base.clone();
        apply_semio_video_mutation(&mut round_tripped, &mutation);
        for inverse_mutation in <SemioVideoMutation as Mutation<SemioVideoSnapshot>>::inverse(&mutation, &base) {
            apply_semio_video_mutation(&mut round_tripped, &inverse_mutation);
        }
        assert_eq!(round_tripped, base, "inverse_law (mutation-level).await failed for {mutation:?}");

        let diff = Mutation::diff(&mutation, &base);
        let next = apply_valid(diff.diff(), &base);
        let inverse_diff = DiffAlgebra::inverse(diff.diff(), &base);
        let restored = apply_valid(&inverse_diff, &next);
        assert_eq!(restored, base, "inverse_law (diff-level).await failed for {mutation:?}");
    }
}
//#endregion 🔖️InverseLaw

//#region 🔖️AbsorbLaw
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn assert_absorb_matches_sequential(base: &SemioVideoSnapshot, d1: &SemioVideoDiff, d2: &SemioVideoDiff) -> SemioVideoDiff {
    let sequential = apply_valid(d2, &apply_valid(d1, base));
    let mut absorbed = d1.clone();
    MutationDiff::absorb(&mut absorbed, d2.clone());
    assert_eq!(apply_valid(&absorbed, base), sequential, "absorb_law: apply(absorb(d1,d2), base) != sequential");
    absorbed
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn streams_diff(diff: &SemioVideoDiff) -> &crate::standards::v1::subsets::video::schema::diff::SemioVideoStreamsDiff {
    diff.streams.as_ref().expect("streams diff present")
}

#[semio_framework_async_macros::async_test]
async fn absorb_law() {
    // Canonical: Insert(2)+Remove(0) -> {removed:[0], added:[(1,f)]}.
    {
        let base = fixture();
        let f = SemioVideoStream { kind: SemioVideoStreamKind::Subtitle, codec: "f".into(), width: 0, height: 0, rate: SemioRational { num: 1, den: 1 }, samples: Vec::new() };
        let d1 = Mutation::diff(&SemioVideoMutation::InsertStream(insert_stream::InsertStream { index: 2, stream: f.clone() }), &base);
        let mid = apply_valid(d1.diff(), &base);
        let d2 = Mutation::diff(&SemioVideoMutation::RemoveStream(remove_stream::RemoveStream { index: 0 }), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        let triple = streams_diff(&absorbed);
        assert_eq!(triple.removed, vec![0]);
        assert_eq!(triple.added.len(), 1);
        assert_eq!(triple.added[0].index, 1);
        assert_eq!(triple.added[0].item, f);
    }

    // Canonical: Insert(2,f)+Insert(2,g) -> both survive.
    {
        let base = fixture();
        let f = SemioVideoStream { kind: SemioVideoStreamKind::Subtitle, codec: "f".into(), width: 0, height: 0, rate: SemioRational { num: 1, den: 1 }, samples: Vec::new() };
        let g = SemioVideoStream { kind: SemioVideoStreamKind::Subtitle, codec: "g".into(), width: 0, height: 0, rate: SemioRational { num: 1, den: 1 }, samples: Vec::new() };
        let d1 = Mutation::diff(&SemioVideoMutation::InsertStream(insert_stream::InsertStream { index: 2, stream: f.clone() }), &base);
        let mid = apply_valid(d1.diff(), &base);
        let d2 = Mutation::diff(&SemioVideoMutation::InsertStream(insert_stream::InsertStream { index: 2, stream: g.clone() }), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        let triple = streams_diff(&absorbed);
        assert_eq!(triple.added.len(), 2, "both inserts must survive absorb, not LWW-clobber");
        assert!(triple.added.iter().any(|a| a.item == f));
        assert!(triple.added.iter().any(|a| a.item == g));
    }

    // Canonical: Insert(1,f)+SetField(1,v) -> patch into the added payload.
    {
        let base = fixture();
        let f = SemioVideoStream { kind: SemioVideoStreamKind::Subtitle, codec: "f".into(), width: 0, height: 0, rate: SemioRational { num: 1, den: 1 }, samples: Vec::new() };
        let d1 = Mutation::diff(&SemioVideoMutation::InsertStream(insert_stream::InsertStream { index: 1, stream: f.clone() }), &base);
        let mid = apply_valid(d1.diff(), &base);
        let d2 = Mutation::diff(&SemioVideoMutation::SetStreamMeta(set_stream_meta::SetStreamMeta { index: 1, kind: SemioVideoStreamKind::Audio, codec: "patched".into(), width: 1, height: 1, rate: SemioRational { num: 2, den: 1 } }), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        let triple = streams_diff(&absorbed);
        assert!(triple.modified.is_empty(), "patch-into-added must not surface as a separate modified entry");
        assert_eq!(triple.added.len(), 1);
        assert_eq!(triple.added[0].item.codec, "patched");
        assert_eq!(triple.added[0].item.kind, SemioVideoStreamKind::Audio);
    }

    // Canonical: Modify+Remove -> the modify is annihilated by the later remove.
    {
        let base = fixture();
        let d1 = Mutation::diff(&SemioVideoMutation::SetStreamMeta(set_stream_meta::SetStreamMeta { index: 1, kind: SemioVideoStreamKind::Video, codec: "patched".into(), width: 1, height: 1, rate: SemioRational { num: 2, den: 1 } }), &base);
        let mid = apply_valid(d1.diff(), &base);
        let d2 = Mutation::diff(&SemioVideoMutation::RemoveStream(remove_stream::RemoveStream { index: 1 }), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        let triple = streams_diff(&absorbed);
        assert!(triple.modified.is_empty(), "modify of a since-removed item must not survive absorb");
        assert_eq!(triple.removed, vec![1]);
    }

    // Associativity over a triple.
    {
        let base = fixture();
        let f = SemioVideoStream { kind: SemioVideoStreamKind::Subtitle, codec: "f".into(), width: 0, height: 0, rate: SemioRational { num: 1, den: 1 }, samples: Vec::new() };
        let g = SemioVideoStream { kind: SemioVideoStreamKind::Subtitle, codec: "g".into(), width: 0, height: 0, rate: SemioRational { num: 1, den: 1 }, samples: Vec::new() };
        let d1 = Mutation::diff(&SemioVideoMutation::InsertStream(insert_stream::InsertStream { index: 2, stream: f }), &base);
        let mid1 = apply_valid(d1.diff(), &base);
        let d2 = Mutation::diff(&SemioVideoMutation::InsertStream(insert_stream::InsertStream { index: 2, stream: g }), &mid1);
        let mid2 = apply_valid(d2.diff(), &mid1);
        let d3 = Mutation::diff(&SemioVideoMutation::RemoveStream(remove_stream::RemoveStream { index: 0 }), &mid2);
        let sequential = apply_valid(d3.diff(), &mid2);

        let mut left = d1.diff().clone();
        MutationDiff::absorb(&mut left, d2.diff().clone());
        MutationDiff::absorb(&mut left, d3.diff().clone());

        let mut d2_then_d3 = d2.diff().clone();
        MutationDiff::absorb(&mut d2_then_d3, d3.diff().clone());
        let mut right = d1.diff().clone();
        MutationDiff::absorb(&mut right, d2_then_d3);

        assert_eq!(apply_valid(&left, &base), sequential, "absorb associativity (left) failed");
        assert_eq!(apply_valid(&right, &base), sequential, "absorb associativity (right) failed");
    }
}
//#endregion 🔖️AbsorbLaw

//#region 🔖️BetweenRoundtripLaw
#[semio_framework_async_macros::async_test]
async fn between_roundtrip_law() {
    let a = sweep_a();
    let b = sweep_b();
    assert_eq!(apply_valid(&<SemioVideoDiff as DiffAlgebra<SemioVideoSnapshot>>::between(&a, &b), &a), b);
    assert_eq!(apply_valid(&<SemioVideoDiff as DiffAlgebra<SemioVideoSnapshot>>::between(&b, &a), &b), a);

    let sample = fixture();
    assert_eq!(apply_valid(&<SemioVideoDiff as DiffAlgebra<SemioVideoSnapshot>>::between(&sample, &sample), &sample), sample);

    // "Real" fixture leg: a realistic 2-stream snapshot diffed against a mutated variant.
    let real = fixture();
    let mut mutated = real.clone();
    apply_semio_video_mutation(&mut mutated, &SemioVideoMutation::SetSampleFlags(set_sample_flags::SetSampleFlags { stream_index: 0, index: 0, pts: 1_000, key: true }));
    assert_ne!(real, mutated);
    assert_eq!(apply_valid(&<SemioVideoDiff as DiffAlgebra<SemioVideoSnapshot>>::between(&real, &mutated), &real), mutated);
    assert_eq!(apply_valid(&<SemioVideoDiff as DiffAlgebra<SemioVideoSnapshot>>::between(&mutated, &real), &mutated), real);
}
//#endregion 🔖️BetweenRoundtripLaw

//#region 🔖️CodecRetentionLaw
#[semio_framework_async_macros::async_test]
async fn codec_retention_law() {
    let snap = fixture();
    let bytes = store::ArtifactPack::encode_pack(&snap);
    let decoded = <SemioVideoSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(decoded, snap);
}
//#endregion 🔖️CodecRetentionLaw

//#region 🔖️FieldSweep
/// 🎯️ THE acceptance criterion: `sweep_a`/`sweep_b` differ in every mutable field at both
/// nesting levels (see the fixtures' own doc comment for exactly how removed/modified/added
/// is exercised at each level, and why the two directions of `between()` are both asserted).
#[semio_framework_async_macros::async_test]
async fn field_sweep() {
    let a = sweep_a();
    let b = sweep_b();

    let diff_ab = <SemioVideoDiff as DiffAlgebra<SemioVideoSnapshot>>::between(&a, &b);
    assert_eq!(apply_valid(&diff_ab, &a), b);
    let diff_ba = <SemioVideoDiff as DiffAlgebra<SemioVideoSnapshot>>::between(&b, &a);
    assert_eq!(apply_valid(&diff_ba, &b), a);
    assert!(<SemioVideoDiff as DiffAlgebra<SemioVideoSnapshot>>::between(&a, &a).is_empty());

    // a -> b: streams.removed (dropped subtitle stream) + streams.modified[0] (every scalar
    // field changed, incl. the SemioVideoStreamKind enum) whose OWN nested samples diff shows
    // removed + modified simultaneously.
    let streams_diff_ab = diff_ab.streams.as_ref().expect("streams diff present");
    assert!(!streams_diff_ab.removed.is_empty(), "streams: removed not exercised");
    assert_eq!(streams_diff_ab.modified.len(), 1);
    let stream_mod = &streams_diff_ab.modified[0].diff;
    assert!(stream_mod.kind.is_some(), "modified stream: kind (enum) not exercised");
    assert!(stream_mod.codec.is_some(), "modified stream: codec not exercised");
    assert!(stream_mod.width.is_some(), "modified stream: width not exercised");
    assert!(stream_mod.height.is_some(), "modified stream: height not exercised");
    assert!(stream_mod.rate.is_some(), "modified stream: rate not exercised");
    let samples_diff = stream_mod.samples.as_ref().expect("modified stream: samples diff not exercised");
    assert!(!samples_diff.removed.is_empty(), "samples: removed not exercised");
    assert!(!samples_diff.modified.is_empty(), "samples: modified not exercised");
    let sample_mod = &samples_diff.modified[0].diff;
    assert!(sample_mod.pts.is_some() && sample_mod.key.is_some() && sample_mod.data.is_some(), "modified sample: not every field exercised");

    // b -> a: the OTHER direction's top-level `added` (the very same dropped subtitle stream)
    // plus that same stream's nested `samples.added`.
    let streams_diff_ba = diff_ba.streams.as_ref().expect("streams diff (b->a) present");
    assert!(!streams_diff_ba.added.is_empty(), "streams (b->a): added not exercised");
    let stream_mod_ba = &streams_diff_ba.modified[0].diff;
    let samples_diff_ba = stream_mod_ba.samples.as_ref().expect("samples diff (b->a) present");
    assert!(!samples_diff_ba.added.is_empty(), "samples (b->a): added not exercised");
}
//#endregion 🔖️FieldSweep

//#region 🔖️OpTextBinaryRoundtripLaw
/// 🧪️ `OpText`/`OpBinary` round-trip laws for the hand-rolled `SemioVideoMutation` grammar —
/// exercises every variant, incl. `InsertStream`'s bare `SemioVideoStream` payload (with
/// nested samples), `SetSnapshot`'s whole `SemioVideoSnapshot`, and the `SemioVideoStreamKind`
/// enum tag.
#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    let stream = SemioVideoStream { kind: SemioVideoStreamKind::Subtitle, codec: "srt".into(), width: 0, height: 0, rate: SemioRational { num: 1, den: 1 }, samples: vec![SemioVideoSample { pts: 5, key: true, data: vec![1, 2] }] };
    let mutations = vec![
        SemioVideoMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: sweep_b() }),
        SemioVideoMutation::InsertStream(insert_stream::InsertStream { index: 1, stream: stream.clone() }),
        SemioVideoMutation::RemoveStream(remove_stream::RemoveStream { index: 0 }),
        SemioVideoMutation::SetStreamMeta(set_stream_meta::SetStreamMeta { index: 0, kind: SemioVideoStreamKind::Audio, codec: "hello world".into(), width: 7, height: 9, rate: SemioRational { num: 25, den: 1 } }),
        SemioVideoMutation::InsertSample(insert_sample::InsertSample { stream_index: 0, index: 0, sample: SemioVideoSample { pts: 1, key: false, data: vec![0, 255] } }),
        SemioVideoMutation::RemoveSample(remove_sample::RemoveSample { stream_index: 0, index: 0 }),
        SemioVideoMutation::SetSampleData(set_sample_data::SetSampleData { stream_index: 0, index: 0, data: vec![1, 2, 3, 4] }),
        SemioVideoMutation::SetSampleFlags(set_sample_flags::SetSampleFlags { stream_index: 0, index: 0, pts: 12345, key: true }),
    ];
    for mutation in mutations {
        let printed = mutation.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = SemioVideoMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

        let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
        let decoded = SemioVideoMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
        assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
    }
}
//#endregion 🔖️OpTextBinaryRoundtripLaw
