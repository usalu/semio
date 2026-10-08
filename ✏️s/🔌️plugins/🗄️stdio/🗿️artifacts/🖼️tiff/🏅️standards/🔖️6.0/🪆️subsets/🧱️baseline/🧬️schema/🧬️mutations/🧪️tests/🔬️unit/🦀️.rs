use super::*;
use crate::standards::v6_0::subsets::baseline::schema::{check_tiff_baseline_conformance, CODE_UNSUPPORTED_BITS_PER_SAMPLE, CODE_UNSUPPORTED_PHOTOMETRIC};
use crate::standards::v6_0::subsets::document::schema::snapshot::{TiffIfd, TiffSampleBlock, TiffTag, TiffWord64};

/// 🧫️ A conforming 4x2 RGB document: 8 bits per sample, one owned block.
fn conforming() -> TiffSnapshot {
    let tag = |tag: u16, values: TiffValues| TiffTag { tag, values };
    TiffSnapshot {
        ifds: vec![TiffIfd {
            entries: vec![tag(TAG_IMAGE_WIDTH, TiffValues::Long(vec![4])), tag(TAG_IMAGE_LENGTH, TiffValues::Long(vec![2])), tag(TAG_BITS_PER_SAMPLE, TiffValues::Short(vec![8, 8, 8])), tag(TAG_PHOTOMETRIC, TiffValues::Short(vec![2]))],
            blocks: vec![TiffSampleBlock { x: 0, y: 0, width: 4, height: 2, channels: 3, samples: vec![TiffWord64::default(); 24] }],
        }],
        ..TiffSnapshot::default()
    }
}

fn codes(snapshot: &TiffSnapshot) -> Vec<String> {
    check_tiff_baseline_conformance(snapshot).into_iter().map(|finding| finding.code.0.to_string()).collect()
}

/// 🏷️ [`KINDS`] against the committed catalog and the enum, in declaration order.
#[test]
fn kinds_match_the_committed_catalog_and_the_enum() {
    let manifest = include_str!("../../../../🔮️oracles/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
    let variants = [
        TiffBaselineMutation::SetPhotometricInterpretation(set_photometric_interpretation::SetPhotometricInterpretation { photometric: 2 }),
        TiffBaselineMutation::SetBitsPerSample(set_bits_per_sample::SetBitsPerSample { bits: vec![8] }),
    ];
    assert_eq!(variants.len(), KINDS.len(), "every variant needs exactly one KINDS entry");
    for (variant, kind) in variants.iter().zip(KINDS) {
        let tag = match serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(variant)).expect("serialize") {
            serde_json::Value::Object(members) => members.get("mutation").and_then(|value| value.as_str()).expect("tagged enum carries its own discriminant").to_string(),
            other => panic!("a tagged enum must serialize as an object, got {other:?}"),
        };
        assert_eq!(&tag.as_str(), kind, "declaration order must match KINDS");
    }
}

/// 🛡️ Every kind moves the document across the axis its own diagnostic reports, and only that axis.
#[test]
fn each_kind_moves_exactly_the_axis_its_diagnostic_reports() {
    assert!(codes(&conforming()).is_empty(), "the fixture must start conforming, got {:?}", codes(&conforming()));

    let mut snapshot = conforming();
    apply_tiff_baseline_mutation(&mut snapshot, &TiffBaselineMutation::SetPhotometricInterpretation(set_photometric_interpretation::SetPhotometricInterpretation { photometric: 6 }));
    assert_eq!(codes(&snapshot), vec![CODE_UNSUPPORTED_PHOTOMETRIC.to_string()]);

    let mut snapshot = conforming();
    apply_tiff_baseline_mutation(&mut snapshot, &TiffBaselineMutation::SetBitsPerSample(set_bits_per_sample::SetBitsPerSample { bits: vec![16, 16, 16] }));
    assert_eq!(codes(&snapshot), vec![CODE_UNSUPPORTED_BITS_PER_SAMPLE.to_string()]);
}

/// 🚫️ Setting an axis to the value it already holds produces the empty diff, and an axis the page does not carry is refused.
#[test]
fn a_no_op_is_empty_and_an_absent_axis_is_refused() {
    let base = conforming();
    let same = TiffBaselineMutation::SetPhotometricInterpretation(set_photometric_interpretation::SetPhotometricInterpretation { photometric: 2 });
    assert_eq!(<TiffBaselineMutation as Mutation<TiffSnapshot>>::diff(&same, &base).diff(), &TiffDiff::default());
    let mut bare = base.clone();
    bare.ifds[0].entries.retain(|entry| entry.tag != TAG_PHOTOMETRIC);
    assert!(!<TiffBaselineMutation as Mutation<TiffSnapshot>>::diff(&same, &bare).is_applicable(Default::default()));
}

#[semio_framework_async_macros::async_test]
async fn baseline_mutation_inverse_sum_law_holds_for_every_leaf() {
    let base = conforming();
    for mutation in [
        TiffBaselineMutation::SetPhotometricInterpretation(set_photometric_interpretation::SetPhotometricInterpretation { photometric: 0 }),
        TiffBaselineMutation::SetBitsPerSample(set_bits_per_sample::SetBitsPerSample { bits: vec![4, 4, 4] }),
    ] {
        protocol::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    }
}
