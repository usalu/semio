use super::*;
use crate::standards::v1::subsets::audio::schema::snapshot::{SemioAudioFormat, SemioAudioSnapshot};
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::flow::schema::snapshot::{FlowNode, SemioFlowSnapshot};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn audio_snapshot(sample_rate: u32) -> SemioSnapshot {
    SemioSnapshot { subset: SemioSubsetSnapshot::Audio(SemioAudioSnapshot { sample_rate, format: SemioAudioFormat::Pcm16, ..Default::default() }), ..Default::default() }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn flow_snapshot(node_ids: &[&str]) -> SemioSnapshot {
    SemioSnapshot {
        subset: SemioSubsetSnapshot::Flow(SemioFlowSnapshot {
            nodes: node_ids.iter().map(|id| FlowNode { id: (*id).into(), kind: "task".into(), label: (*id).into(), params: vec![], position: SemioPoint2 { x: 0.0, y: 0.0 } }).collect(),
            ..Default::default()
        }),
        ..Default::default()
    }
}

/// 🧪️ between_roundtrip_law + field_sweep, real same-kind field change (audio's
/// `sample_rate`, a genuinely mutable field — not the `schema` identity field every subset's
/// own diff module explicitly excludes from diffing).
#[semio_framework_async_macros::async_test]
async fn between_roundtrip_law_same_kind_real_field_change() {
    let a = audio_snapshot(44_100);
    let b = audio_snapshot(48_000);
    let d = <SemioDiff as DiffAlgebra<SemioSnapshot>>::between(&a, &b);
    assert!(matches!(d, SemioDiff::Audio(_)), "same-kind change must nest, not Replace: {d:?}");
    assert!(!d.is_empty());
    assert_eq!(d.apply(&a).expect("valid nested diff"), b);
    assert!(<SemioDiff as DiffAlgebra<SemioSnapshot>>::between(&a, &a).is_empty());
}

/// 🧪️ field_sweep, second real same-kind field change (flow's id-keyed `nodes`
/// collection) — sweeps a DIFFERENT subset and a DIFFERENT field shape (collection insert,
/// not a scalar) than the audio case above.
#[semio_framework_async_macros::async_test]
async fn between_roundtrip_law_flow_node_insert() {
    let a = flow_snapshot(&["n1"]);
    let b = flow_snapshot(&["n1", "n2"]);
    let d = <SemioDiff as DiffAlgebra<SemioSnapshot>>::between(&a, &b);
    assert!(matches!(d, SemioDiff::Flow(_)));
    assert_eq!(d.apply(&a).expect("valid nested diff"), b);
    let inv = d.inverse(&a);
    assert_eq!(inv.apply(&d.apply(&a).expect("valid nested diff")).expect("valid inverse"), a);
}

/// 🧪️ between_roundtrip_law, cross-kind change: no sparse representation exists, must fall
/// back to `Replace` — and still satisfy the law.
#[semio_framework_async_macros::async_test]
async fn between_roundtrip_law_cross_kind_replaces() {
    let a = audio_snapshot(44_100);
    let b = flow_snapshot(&["n1"]);
    let d = <SemioDiff as DiffAlgebra<SemioSnapshot>>::between(&a, &b);
    assert!(matches!(d, SemioDiff::Replace(_)), "cross-kind change must Replace: {d:?}");
    assert_eq!(d.apply(&a).expect("valid replacement"), b);
}

/// 🧪️ inverse_law across all 3 shapes: same-kind nested, cross-kind Replace, and NoChange.
#[semio_framework_async_macros::async_test]
async fn inverse_law_covers_nested_replace_and_no_change() {
    for (a, b) in [(audio_snapshot(44_100), audio_snapshot(96_000)), (audio_snapshot(44_100), flow_snapshot(&["n1", "n2"])), (audio_snapshot(44_100), audio_snapshot(44_100))] {
        let d = <SemioDiff as DiffAlgebra<SemioSnapshot>>::between(&a, &b);
        let applied = d.apply(&a).expect("valid diff");
        let inv = d.inverse(&a);
        assert_eq!(inv.apply(&applied).expect("valid inverse"), a, "inverse must restore base for {d:?}");
    }
}

/// 🧪️ absorb_law: same-kind sequential coalesce delegates to the nested subset's own
/// (already-proven) `absorb`.
#[semio_framework_async_macros::async_test]
async fn absorb_law_same_kind_delegates_to_nested() {
    let a = audio_snapshot(44_100);
    let mid = audio_snapshot(48_000);
    let after = audio_snapshot(96_000);
    let mut d1 = <SemioDiff as DiffAlgebra<SemioSnapshot>>::between(&a, &mid);
    let d2 = <SemioDiff as DiffAlgebra<SemioSnapshot>>::between(&mid, &after);
    let applied_before_absorb = d1.apply(&a).expect("valid first diff");
    d1.absorb(d2.clone());
    assert_eq!(d1.apply(&a).expect("valid absorbed diff"), d2.apply(&applied_before_absorb).expect("valid second diff"));
    assert_eq!(d1.apply(&a).expect("valid absorbed diff"), after);
}

/// 🧪️ absorb_law: a later `Replace` always wins outright, whatever preceded it.
#[semio_framework_async_macros::async_test]
async fn absorb_law_later_replace_wins() {
    let a = audio_snapshot(44_100);
    let mid = audio_snapshot(48_000);
    let after = flow_snapshot(&["n1"]);
    let mut d1 = <SemioDiff as DiffAlgebra<SemioSnapshot>>::between(&a, &mid);
    let d2 = <SemioDiff as DiffAlgebra<SemioSnapshot>>::between(&mid, &after);
    d1.absorb(d2);
    assert!(matches!(d1, SemioDiff::Replace(_)));
    assert_eq!(d1.apply(&a).expect("valid replacement"), after);
}

/// 🧪️ absorb_law: an earlier `Replace` absorbing a later same-kind diff folds it into the
/// replacement snapshot rather than dropping it.
#[semio_framework_async_macros::async_test]
async fn absorb_law_replace_then_nested_folds_in() {
    let a = flow_snapshot(&["n1"]);
    let replaced = audio_snapshot(44_100);
    let after = audio_snapshot(48_000);
    let mut d1 = SemioDiff::Replace(Box::new(replaced.clone()));
    let d2 = <SemioDiff as DiffAlgebra<SemioSnapshot>>::between(&replaced, &after);
    d1.absorb(d2);
    assert_eq!(d1.apply(&a).expect("valid absorbed diff"), after);
}

/// 🧪️ diff_codec_text_binary_roundtrip_law across `NoChange`, a same-kind nested diff (one
/// per subset kind, proving the dispatch table's all 13 tags), and `Replace`.
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    let a = audio_snapshot(44_100);
    let b = audio_snapshot(48_000);
    let nested = <SemioDiff as DiffAlgebra<SemioSnapshot>>::between(&a, &b);
    let replace = SemioDiff::Replace(Box::new(flow_snapshot(&["n1"])));
    for d in [SemioDiff::NoChange, nested, replace] {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = SemioDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch for {d:?}");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = SemioDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch for {d:?}");
    }
}

/// 🧪️ Dispatch-table coverage: every one of the 18 same-kind tags round-trips through
/// `print_diff`/`parse_diff` for the trivial `NoChange`-shaped nested diff (proves the print/
/// parse match is wired correctly for every subset, without re-deriving each subset's own deep
/// field grammar here).
#[semio_framework_async_macros::async_test]
async fn all_eighteen_subset_tags_round_trip_empty_nested_diff() {
    let subsets: Vec<SemioSubsetSnapshot> = vec![
        SemioSubsetSnapshot::Brep(Default::default()),
        SemioSubsetSnapshot::Mesh(Default::default()),
        SemioSubsetSnapshot::Model(Default::default()),
        SemioSubsetSnapshot::Value(Default::default()),
        SemioSubsetSnapshot::Document(Default::default()),
        SemioSubsetSnapshot::Cad(Default::default()),
        SemioSubsetSnapshot::Drawing(Default::default()),
        SemioSubsetSnapshot::Image(Default::default()),
        SemioSubsetSnapshot::Video(Default::default()),
        SemioSubsetSnapshot::Audio(Default::default()),
        SemioSubsetSnapshot::Animation(Default::default()),
        SemioSubsetSnapshot::Presentation(Default::default()),
        SemioSubsetSnapshot::Flow(Default::default()),
        SemioSubsetSnapshot::Text(Default::default()),
        SemioSubsetSnapshot::Table(Default::default()),
        SemioSubsetSnapshot::Graph(Default::default()),
        SemioSubsetSnapshot::Object(Default::default()),
        SemioSubsetSnapshot::Kit(Default::default()),
    ];
    for subset in subsets {
        let snap = SemioSnapshot { schema: "stdio.semio".into(), subset };
        let d = <SemioDiff as DiffAlgebra<SemioSnapshot>>::between(&snap, &snap);
        assert!(d.is_empty(), "identical snapshot must diff empty: {d:?}");
        let printed = d.print_diff();
        let parsed = SemioDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d);
    }
}

#[semio_framework_async_macros::async_test]
async fn malformed_cross_kind_diff_is_rejected_and_absorb_preserves_rejection() {
    let base = audio_snapshot(44_100);
    let error = SemioDiff::Flow(Default::default()).apply(&base).unwrap_err();
    assert_eq!(error.code, "mutation.apply.kind-mismatch");
    assert_eq!(error.target, vec!["subset"]);

    let mut diff = SemioDiff::Flow(Default::default());
    diff.absorb(SemioDiff::Audio(Default::default()));
    let SemioDiff::Rejected(error) = &diff else {
        panic!("cross-kind absorb must preserve a typed rejection");
    };
    assert_eq!(error.code, "mutation.absorb.kind-mismatch");
    assert!(diff.apply(&base).is_err());

    let text = diff.print_diff();
    assert_eq!(SemioDiff::parse_diff(&text).expect("rejection text round-trip"), diff);
    let bytes = diff.encode_diff().expect("rejection binary encode");
    assert_eq!(SemioDiff::decode_diff(&bytes).expect("rejection binary round-trip"), diff);
}
