//! 🚪️ IO stdio.tiff (6.0/✳️baseline) — reuses the ✳️any subset's `binary` raw-codec DAG leaf
//! rather than duplicating it (same `TiffSnapshot` type, same catalog DAG edges). Registration
//! flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level aggregator,
//! and the `SubsetValidator` directly), not per-leaf `register()` — same pattern `✳️any/🚪️io`
//! already established for this artifact.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::tiff::standards::v6_0::subsets::any::schema::snapshot::TiffSnapshot;
    use crate::artifacts::tiff::standards::v6_0::subsets::any::schema::TiffComposer as TiffAnyComposer;
    use crate::artifacts::tiff::standards::v6_0::subsets::baseline::schema::check_tiff_baseline_conformance;
    use dsl::{Diagnostic, FaultCode, Severity, TextSpan};
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_BASELINE: Dialect = Dialect { artifact_kind: "s.stdio.tiff", standard: StandardId("6.0"), subset: SubsetId("baseline") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.tiff", standard: StandardId("6.0"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct TiffBaselineComposerComposition;

    impl ArtifactComposition for TiffBaselineComposerComposition {
        type Snapshot = TiffSnapshot;
        const WRITES: Dialect = DIALECT_BASELINE;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_BASELINE, DEP_BINARY]
        }

        fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = TiffAnyComposer::compose(sources)?;
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(check_tiff_baseline_conformance(&inner.snapshot));
            Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    pub struct TiffBaselineValidator;

    impl SubsetValidator for TiffBaselineValidator {
        const DIALECT: Dialect = DIALECT_BASELINE;

        fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <TiffSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <TiffSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_tiff_baseline_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.tiff.baseline.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "Baseline TIFF (6.0) SubsetValidator: payload did not decode as a TiffSnapshot -- skipped".into(),
                    expected: None,
                    scope: dsl::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<TiffBaselineValidator>)
    }

    /// 📌️ Registers this subset's `SubsetValidator`. Called from 6.0's own `⚙️engine::register()`.
    /// The `ComposerEntry` itself is registered separately via this standard's own
    /// `composer::entries()` aggregation.
    pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
    //#endregion 🔖️SubsetValidator

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::tiff::standards::v6_0::subsets::any::schema::snapshot::{TiffByteOrder, TiffFieldType, TiffIfd, TiffTag, TiffValues};
        use semio_framework_plugin::AnalyzeSource;

        /// 🩹 `TiffSnapshot::default()` has no IFD at all, which the real encoder rejects ("tiff:
        /// encode requires an ImageWidth tag") -- `encode_pack`'s infallible convenience wrapper
        /// then panics instead of returning that `Err`. A minimal 1x1 non-degenerate image (real
        /// IFD with ImageWidth/ImageLength/StripOffsets) is the smallest real fixture the encoder
        /// accepts.
        fn minimal_non_degenerate_snapshot() -> TiffSnapshot {
            TiffSnapshot {
                byte_order: TiffByteOrder::LittleEndian,
                ifds: vec![TiffIfd { entries: vec![TiffTag { tag: 256, kind: TiffFieldType::Long, values: TiffValues::Long(vec![1]) }, TiffTag { tag: 257, kind: TiffFieldType::Long, values: TiffValues::Long(vec![1]) }] }],
                pixels: vec![0, 0, 0, 255],
                ..TiffSnapshot::default()
            }
        }

        #[test]
        fn compose_carries_no_findings_for_a_conformant_document() {
            let bytes = <TiffSnapshot as store::ArtifactPack>::encode_pack(&minimal_non_degenerate_snapshot());
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
            let composed = TiffBaselineComposerComposition::compose(&sources).expect("pass-through compose never fails on conformance grounds");
            assert!(composed.diagnostics.is_empty(), "got {:?}", composed.diagnostics);
        }

        #[test]
        fn subset_validator_carries_no_findings_for_a_conformant_document() {
            let bytes = <TiffSnapshot as store::ArtifactPack>::encode_pack(&minimal_non_degenerate_snapshot());
            let diagnostics = TiffBaselineValidator::validate(&IoPayload::Binary(bytes));
            assert!(diagnostics.is_empty(), "got {diagnostics:?}");
        }

        /// 🧭️ `TiffSnapshot::default()` (no IFD at all) can never round-trip through the real
        /// binary encoder (`encode_pack` panics -- see `minimal_non_degenerate_snapshot`'s doc), so
        /// the "no IFD" finding is only reachable by directly constructing/mutating a snapshot, not
        /// via a real decoded file (`decode_tiff` itself always guarantees `ifds` is non-empty on
        /// success). The real per-field check for that path is covered directly in `🧐️analyzer`'s
        /// own `no_ifd_is_flagged_soft` test; this composer/validator layer only needs the
        /// conformant-document path exercised above.
        #[test]
        fn no_ifd_diagnostic_is_reachable_via_direct_check_not_through_encode_pack() {
            let diagnostics = check_tiff_baseline_conformance(&TiffSnapshot::default());
            assert!(diagnostics.iter().any(|d| d.code.0 == crate::artifacts::tiff::standards::v6_0::subsets::baseline::schema::CODE_NO_IFD), "got {diagnostics:?}");
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
