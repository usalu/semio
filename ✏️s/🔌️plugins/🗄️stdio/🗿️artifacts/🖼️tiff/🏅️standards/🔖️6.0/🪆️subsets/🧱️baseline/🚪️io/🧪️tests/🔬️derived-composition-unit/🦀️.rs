mod tests {
    use super::*;
    use crate::standards::v6_0::subsets::document::schema::snapshot::{TiffByteOrder, TiffFieldType, TiffIfd, TiffTag, TiffValues};
    use semio_framework_plugin::AnalyzeSource;

    /// 🩹 `TiffSnapshot::default()` has no IFD at all, which the real encoder rejects ("tiff:
    /// encode requires an ImageWidth tag") -- `encode_pack`'s infallible convenience wrapper
    /// then panics instead of returning that `Err`. A minimal 1x1 non-degenerate image (real
    /// IFD with ImageWidth/ImageLength/StripOffsets) is the smallest real fixture the encoder
    /// accepts.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn minimal_non_degenerate_snapshot() -> TiffSnapshot {
        TiffSnapshot {
            byte_order: TiffByteOrder::LittleEndian,
            ifds: vec![TiffIfd { pixels: Vec::new(), entries: vec![TiffTag { tag: 256, kind: TiffFieldType::Long, values: TiffValues::Long(vec![1]) }, TiffTag { tag: 257, kind: TiffFieldType::Long, values: TiffValues::Long(vec![1]) }] }],
            pixels: vec![0, 0, 0, 255],
            ..TiffSnapshot::default()
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn compose_carries_no_findings_for_a_conformant_document() {
        let bytes = <TiffSnapshot as store::ArtifactPack>::encode_pack(&minimal_non_degenerate_snapshot());
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
        let composed = TiffBaselineComposerComposition::compose(&sources).expect("pass-through compose never fails on conformance grounds");
        assert!(composed.diagnostics.is_empty(), "got {:?}", composed.diagnostics);
    }

    #[semio_framework_async_macros::async_test]
    async fn subset_validator_carries_no_findings_for_a_conformant_document() {
        let bytes = <TiffSnapshot as store::ArtifactPack>::encode_pack(&minimal_non_degenerate_snapshot());
        let diagnostics = TiffBaselineValidator::validate(&IoPayload::Binary(bytes)).await;
        assert!(diagnostics.is_empty(), "got {diagnostics:?}");
    }

    /// 🧭️ `TiffSnapshot::default()` (no IFD at all) can never round-trip through the real
    /// binary encoder (`encode_pack` panics -- see `minimal_non_degenerate_snapshot`'s doc), so
    /// the "no IFD" finding is only reachable by directly constructing/mutating a snapshot, not
    /// via a real decoded file (`decode_tiff` itself always guarantees `ifds` is non-empty on
    /// success). The real per-field check for that path is covered directly in `🧐️analyzer`'s
    /// own `no_ifd_is_flagged_soft` test; this composer/validator layer only needs the
    /// conformant-document path exercised above.
    #[semio_framework_async_macros::async_test]
    async fn no_ifd_diagnostic_is_reachable_via_direct_check_not_through_encode_pack() {
        let diagnostics = check_tiff_baseline_conformance(&TiffSnapshot::default());
        assert!(diagnostics.iter().any(|d| d.code.0 == crate::standards::v6_0::subsets::baseline::schema::CODE_NO_IFD), "got {diagnostics:?}");
    }
}
