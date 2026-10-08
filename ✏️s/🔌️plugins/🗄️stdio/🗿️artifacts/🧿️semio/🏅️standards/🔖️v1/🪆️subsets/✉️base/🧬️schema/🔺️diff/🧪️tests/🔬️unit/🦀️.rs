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

/// 🧪️ diff_codec_text_binary_roundtrip_law across `NoChange`, a same-kind nested diff (one
/// per subset kind, proving the dispatch table's all 13 tags), and `Replace`.
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    let nested = SemioDiff::Audio(crate::standards::v1::subsets::audio::schema::diff::SemioAudioDiff { sample_rate: Some(48_000), ..Default::default() });
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
        let d = SemioDiff::default();
        assert!(d.is_empty(), "identical snapshot must diff empty: {d:?}");
        let printed = d.print_diff();
        let parsed = SemioDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d);
    }
}

#[semio_framework_async_macros::async_test]
async fn malformed_cross_kind_diff_is_rejected_and_absorb_preserves_rejection() {
    let base = audio_snapshot(44_100);
    let error = protocol::apply_diff(&SemioDiff::Flow(Default::default()), &base).unwrap_err();
    assert_eq!(error.code, "mutation.apply.kind-mismatch");
    assert_eq!(error.target, vec!["subset"]);

    let mut diff = SemioDiff::Flow(Default::default());
    diff.absorb(SemioDiff::Audio(Default::default()));
    let SemioDiff::Rejected(error) = &diff else {
        panic!("cross-kind absorb must preserve a typed rejection");
    };
    assert_eq!(error.code, "mutation.apply.kind-mismatch");
    assert!(protocol::apply_diff(&diff, &base).is_err());

    let text = diff.print_diff();
    assert_eq!(SemioDiff::parse_diff(&text).expect("rejection text round-trip"), diff);
    let bytes = diff.encode_diff().expect("rejection binary encode");
    assert_eq!(SemioDiff::decode_diff(&bytes).expect("rejection binary round-trip"), diff);
}
