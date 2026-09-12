use super::*;
use crate::standards::v_jfif_1_01::subsets::baseline::schema::{check_baseline_conformance, CODE_ARITHMETIC, CODE_COMPONENT_SAMPLING, CODE_HUFFMAN_TABLE_COUNT, CODE_PRECISION, CODE_SOF_MARKER};
use crate::standards::v_jfif_1_01::subsets::document::schema::snapshot::{JpgFrameHeader, JpgHuffmanClass};

fn table(class: JpgHuffmanClass, id: u8) -> JpgHuffmanTable {
    JpgHuffmanTable { id, class, bits: [0u8; 16], values: vec![id] }
}

/// 🧫️ A conforming 16x16 YCbCr baseline snapshot: SOF0, 8-bit, three components at 2x2/1x1/1x1,
/// two DC and two AC Huffman tables, no arithmetic conditioning.
fn conforming() -> JpgSnapshot {
    JpgSnapshot {
        frame: Some(JpgFrameHeader {
            precision: BASELINE_PRECISION,
            width: 16,
            height: 16,
            components: vec![
                JpgFrameComponent { id: 1, h_sampling: 2, v_sampling: 2, quant_table_id: 0 },
                JpgFrameComponent { id: 2, h_sampling: 1, v_sampling: 1, quant_table_id: 1 },
                JpgFrameComponent { id: 3, h_sampling: 1, v_sampling: 1, quant_table_id: 1 },
            ],
        }),
        sof_marker: SOF0,
        arithmetic: false,
        huffman_tables: vec![table(JpgHuffmanClass::Dc, 0), table(JpgHuffmanClass::Ac, 0), table(JpgHuffmanClass::Dc, 1), table(JpgHuffmanClass::Ac, 1)],
        ..JpgSnapshot::default()
    }
}

fn codes(snapshot: &JpgSnapshot) -> Vec<String> {
    check_baseline_conformance(snapshot).into_iter().map(|finding| finding.code.0.to_string()).collect()
}

/// 🏷️ [`KINDS`] against the committed catalog. The framework never parses Rust, so without this
/// the manifest could keep measuring `🟣️mutate-jpg-jfif-1-01-baseline` against a vocabulary this
/// subset no longer has — which is exactly the gap that left this vocabulary with no catalog at
/// all until the completeness gate learned to see an unregistered one.
#[test]
fn kinds_match_the_committed_catalog() {
    let manifest = include_str!("../../../../🔮️oracles/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
    assert!(manifest.contains("jpg-jfif-1-01-baseline-mutate"), "the manifest must declare this subset's OWN capability, not the 🧾️document subset's");
}

#[test]
fn kinds_match_enum_variants_in_declaration_order() {
    let variants = [
        JpgBaselineMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: JpgSnapshot::default() }),
        JpgBaselineMutation::SetSofMarker(set_sof_marker::SetSofMarker { marker: SOF0 }),
        JpgBaselineMutation::SetSamplePrecision(set_sample_precision::SetSamplePrecision { precision: BASELINE_PRECISION }),
        JpgBaselineMutation::SetArithmetic(set_arithmetic::SetArithmetic { arithmetic: false }),
        JpgBaselineMutation::InsertHuffmanTable(insert_huffman_table::InsertHuffmanTable { index: 4, table: table(JpgHuffmanClass::Dc, 2) }),
        JpgBaselineMutation::RemoveHuffmanTable(remove_huffman_table::RemoveHuffmanTable { key: JpgHuffmanTableKey { class: JpgHuffmanClass::Dc, id: 0 } }),
        JpgBaselineMutation::InsertFrameComponent(insert_frame_component::InsertFrameComponent { index: 3, component: JpgFrameComponent { id: 4, h_sampling: 1, v_sampling: 1, quant_table_id: 0 } }),
        JpgBaselineMutation::RemoveFrameComponent(remove_frame_component::RemoveFrameComponent { id: 3 }),
        JpgBaselineMutation::SetComponentSampling(set_component_sampling::SetComponentSampling { id: 1, h_sampling: 2, v_sampling: 2 }),
    ];
    assert_eq!(variants.len(), KINDS.len(), "every variant needs exactly one KINDS entry");
    for (variant, kind) in variants.iter().zip(KINDS) {
        let tag = match serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(variant)).expect("serialize") {
            serde_json::Value::Object(members) => members.get("mutation").and_then(|value| value.as_str()).expect("tagged enum carries its own discriminant").to_string(),
            other => panic!("a tagged enum must serialize as an object, got {other:?}"),
        };
        assert_eq!(&tag.as_str(), kind, "declaration order must match KINDS");
    }
}

/// 🛡️ The point of the whole vocabulary: every kind moves the document across the axis its own
/// diagnostic reports, and only that axis.
#[test]
fn each_kind_moves_exactly_the_axis_its_diagnostic_reports() {
    assert!(codes(&conforming()).is_empty(), "the fixture must start conforming, got {:?}", codes(&conforming()));

    let mut snapshot = conforming();
    apply_jpg_baseline_mutation(&mut snapshot, &JpgBaselineMutation::SetSofMarker(set_sof_marker::SetSofMarker { marker: 0xC2 }));
    assert_eq!(codes(&snapshot), vec![CODE_SOF_MARKER.to_string()]);

    let mut snapshot = conforming();
    apply_jpg_baseline_mutation(&mut snapshot, &JpgBaselineMutation::SetSamplePrecision(set_sample_precision::SetSamplePrecision { precision: 12 }));
    assert_eq!(codes(&snapshot), vec![CODE_PRECISION.to_string()]);

    let mut snapshot = conforming();
    apply_jpg_baseline_mutation(&mut snapshot, &JpgBaselineMutation::SetArithmetic(set_arithmetic::SetArithmetic { arithmetic: true }));
    assert_eq!(codes(&snapshot), vec![CODE_ARITHMETIC.to_string()]);

    let mut snapshot = conforming();
    apply_jpg_baseline_mutation(&mut snapshot, &JpgBaselineMutation::InsertHuffmanTable(insert_huffman_table::InsertHuffmanTable { index: 4, table: table(JpgHuffmanClass::Dc, 2) }));
    assert_eq!(codes(&snapshot), vec![CODE_HUFFMAN_TABLE_COUNT.to_string()]);

    let mut snapshot = conforming();
    apply_jpg_baseline_mutation(&mut snapshot, &JpgBaselineMutation::SetComponentSampling(set_component_sampling::SetComponentSampling { id: 1, h_sampling: 5, v_sampling: 2 }));
    assert_eq!(codes(&snapshot), vec![CODE_COMPONENT_SAMPLING.to_string()]);
}

/// 🔢️ The component-count axis needs TWO insertions to cross its line (the check reports more
/// than four), which is exactly why the kind counts rather than replaces.
#[test]
fn a_fifth_frame_component_is_what_crosses_the_component_count_line() {
    let mut snapshot = conforming();
    apply_jpg_baseline_mutation(&mut snapshot, &JpgBaselineMutation::InsertFrameComponent(insert_frame_component::InsertFrameComponent { index: 3, component: JpgFrameComponent { id: 4, h_sampling: 1, v_sampling: 1, quant_table_id: 1 } }));
    assert!(codes(&snapshot).is_empty(), "four components is still inside the line");
    apply_jpg_baseline_mutation(&mut snapshot, &JpgBaselineMutation::InsertFrameComponent(insert_frame_component::InsertFrameComponent { index: 4, component: JpgFrameComponent { id: 5, h_sampling: 1, v_sampling: 1, quant_table_id: 1 } }));
    assert_eq!(codes(&snapshot), vec![CODE_COMPONENT_SAMPLING.to_string()]);
}

/// ↩️ `apply(inverse(m), apply(m, base))` must land back on `base` for every kind.
#[test]
fn every_kind_is_inverted_by_its_own_inverse() {
    let cases = [
        JpgBaselineMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: JpgSnapshot::default() }),
        JpgBaselineMutation::SetSofMarker(set_sof_marker::SetSofMarker { marker: 0xC1 }),
        JpgBaselineMutation::SetSamplePrecision(set_sample_precision::SetSamplePrecision { precision: 12 }),
        JpgBaselineMutation::SetArithmetic(set_arithmetic::SetArithmetic { arithmetic: true }),
        JpgBaselineMutation::InsertHuffmanTable(insert_huffman_table::InsertHuffmanTable { index: 4, table: table(JpgHuffmanClass::Ac, 3) }),
        JpgBaselineMutation::RemoveHuffmanTable(remove_huffman_table::RemoveHuffmanTable { key: JpgHuffmanTableKey { class: JpgHuffmanClass::Dc, id: 1 } }),
        JpgBaselineMutation::InsertFrameComponent(insert_frame_component::InsertFrameComponent { index: 3, component: JpgFrameComponent { id: 9, h_sampling: 1, v_sampling: 1, quant_table_id: 0 } }),
        JpgBaselineMutation::RemoveFrameComponent(remove_frame_component::RemoveFrameComponent { id: 2 }),
        JpgBaselineMutation::SetComponentSampling(set_component_sampling::SetComponentSampling { id: 1, h_sampling: 4, v_sampling: 4 }),
    ];
    for mutation in cases {
        let base = conforming();
        let mut snapshot = base.clone();
        apply_jpg_baseline_mutation(&mut snapshot, &mutation);
        for undo in inverse_jpg_baseline_mutation(&mutation, &base) {
            apply_jpg_baseline_mutation(&mut snapshot, &undo);
        }
        assert_eq!(snapshot.sof_marker, base.sof_marker, "inverse of {mutation:?} left sof_marker moved");
        assert_eq!(snapshot.arithmetic, base.arithmetic, "inverse of {mutation:?} left arithmetic moved");
        assert_eq!(snapshot.frame, base.frame, "inverse of {mutation:?} left the frame moved");
        assert_eq!(snapshot.huffman_tables, base.huffman_tables, "inverse of {mutation:?} left the Huffman tables moved");
    }
}

/// 🚫️ An addition whose target already exists, and a removal whose target does not, are both
/// no-ops — and must produce the EMPTY diff rather than a change nothing made.
#[test]
fn an_insertion_that_finds_its_target_present_produces_an_empty_diff() {
    let base = conforming();
    let already = JpgBaselineMutation::InsertHuffmanTable(insert_huffman_table::InsertHuffmanTable { index: 0, table: table(JpgHuffmanClass::Dc, 0) });
    assert_eq!(<JpgBaselineMutation as Mutation<JpgSnapshot>>::diff(&already, &base).diff(), &JpgDiff::default());
    assert_eq!(inverse_jpg_baseline_mutation(&already, &base), Vec::new());

    let absent = JpgBaselineMutation::RemoveFrameComponent(remove_frame_component::RemoveFrameComponent { id: 42 });
    assert_eq!(<JpgBaselineMutation as Mutation<JpgSnapshot>>::diff(&absent, &base).diff(), &JpgDiff::default());
    assert_eq!(inverse_jpg_baseline_mutation(&absent, &base), Vec::new());
}
