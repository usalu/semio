//! 🚪️ IO stdio.jpg (jfif-1.01/✳️baseline) — reuses the ✳️any subset's `binary` raw-codec DAG
//! leaf rather than duplicating it (same `JpgSnapshot` type, same catalog DAG edge). Registration
//! flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level aggregator,
//! and the `SubsetValidator` directly), not per-leaf `register()` — same pattern `✳️any/🚪️io`
//! already established for this artifact.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::jpg::standards::v_jfif_1_01::subsets::any::schema::JpgComposer as JpgAnyComposer;
    use crate::artifacts::jpg::standards::v_jfif_1_01::subsets::baseline::schema::check_baseline_conformance;
    use crate::artifacts::jpg::JpgSnapshot;
    use dsl::{Diagnostic, FaultCode, Severity, TextSpan};
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_BASELINE: Dialect = Dialect { artifact_kind: "s.stdio.jpg", standard: StandardId("jfif-1.01"), subset: SubsetId("baseline") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.jpg", standard: StandardId("jfif-1.01"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct JpgBaselineComposerComposition;

    impl ArtifactComposition for JpgBaselineComposerComposition {
        type Snapshot = JpgSnapshot;
        const WRITES: Dialect = DIALECT_BASELINE;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_BASELINE, DEP_BINARY]
        }

        fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = JpgAnyComposer::compose(sources)?;
            let checks = check_baseline_conformance(&inner.snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("baseline conformance violated: {} hard issue(s) -- not stamping the baseline dialect", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ The registered `SubsetValidator` for `jfif-1.01/baseline` -- see the module doc comment for
    /// how this relates to the composer's own pre-serialization hard gate above.
    pub struct JpgBaselineValidator;

    impl SubsetValidator for JpgBaselineValidator {
        const DIALECT: Dialect = DIALECT_BASELINE;

        fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <JpgSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <JpgSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_baseline_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.jpg.baseline.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "baseline SubsetValidator: payload did not decode as a JpgSnapshot -- skipped".into(),
                    expected: None,
                    scope: dsl::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<JpgBaselineValidator>)
    }

    /// 📌️ Registers this subset's `SubsetValidator` with the generic io registry (D5's
    /// validate-on-build hook). Called from the jfif-1.01 standard's own `⚙️engine::register()`. The
    /// `ComposerEntry` itself is registered separately by the standard-level composer aggregator
    /// (`crate::artifacts::jpg::standards::v_jfif_1_01::engine::io_registry::entries()`), matching how `✳️any`'s
    /// own entry is registered.
    pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
    //#endregion 🔖️SubsetValidator

    #[cfg(test)]
    mod tests {
        use super::*;
        use semio_framework_plugin::AnalyzeSource;

        fn gradient_image(w: u32, h: u32) -> Vec<u8> {
            let mut out = vec![0u8; (w * h * 4) as usize];
            for y in 0..h {
                for x in 0..w {
                    let idx = ((y * w + x) * 4) as usize;
                    out[idx] = ((x * 255) / w.max(1)) as u8;
                    out[idx + 1] = ((y * 255) / h.max(1)) as u8;
                    out[idx + 2] = 128;
                    out[idx + 3] = 255;
                }
            }
            out
        }

        #[test]
        fn engine_encoded_jpeg_composes_and_stamps_baseline() {
            let (w, h) = (32u32, 32u32);
            let snap = JpgSnapshot { width: w, height: h, pixels: gradient_image(w, h), ..JpgSnapshot::default() };
            // 🩹 `AnalyzeSource::Binary` for DIALECT_ANY expects an ALREADY pack-encoded (semio
            // envelope-wrapped) snapshot -- `store::ArtifactPack::encode_pack`, not raw
            // `engine::encode_jpg` bytes (which lack the envelope header the decode step expects).
            let bytes = <JpgSnapshot as store::ArtifactPack>::encode_pack(&snap);
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
            // 🌱 Route through the ✳️any composer first to get a real, engine-decoded snapshot (with
            // frame/sof_marker/huffman-table-count populated) the way `JpgBaselineComposerComposition::compose`
            // itself would internally.
            let composed = JpgBaselineComposerComposition::compose(&sources).expect("real baseline JPEG must compose and stamp baseline");
            assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
            assert!(composed.snapshot.frame.is_some());
            assert_eq!(composed.snapshot.sof_marker, crate::artifacts::jpg::standards::v_jfif_1_01::subsets::baseline::schema::SOF0);
        }

        #[test]
        fn subset_validator_recheck_flags_no_hard_diagnostics_for_a_real_encode() {
            let (w, h) = (16u32, 16u32);
            let snap = JpgSnapshot { width: w, height: h, pixels: gradient_image(w, h), ..JpgSnapshot::default() };
            let bytes = crate::artifacts::jpg::standards::v_jfif_1_01::engine::encode_jpg(&snap).expect("encode");
            let decoded = crate::artifacts::jpg::standards::v_jfif_1_01::engine::decode_jpg(&bytes).expect("decode");
            let packed = <JpgSnapshot as store::ArtifactPack>::encode_pack(&decoded);
            let diagnostics = JpgBaselineValidator::validate(&IoPayload::Binary(packed));
            assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "wire recheck must never report a hard violation for a real baseline encode: {diagnostics:?}");
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
