//! 🧬️ JpgSnapshot schema (jfif-1.01/✳️baseline) — reuses the ✳️any subset's `JpgSnapshot`
//! verbatim (the SAME Rust type, same `s.stdio.jpg` schema id). ITU-T T.81/ISO 10918-1 baseline
//! sequential DCT conformance (in a JFIF 1.01 container) is a validation-gated dialect STAMP on
//! top of that existing schema, not a new one -- see D4's Tier-1 "same snapshot type, subset
//! moves" semantics (`ArtifactCommand::MigrateDialect`). This leaf exists so
//! `🪆️subsets/✳️baseline/🧬️schema/` is present per `🔣️taxonomy.json`'s `subsetChildDirs`, without
//! duplicating the schema definition.

pub use crate::artifacts::jpg::standards::v_jfif_1_01::subsets::any::schema::*;
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::jpg::standards::v_jfif_1_01::subsets::any::schema::JpgBuilder as JpgAnyBuilder;
    use crate::artifacts::jpg::standards::v_jfif_1_01::subsets::baseline::schema::check_baseline_conformance;
    use crate::artifacts::jpg::{JpgDiff, JpgMutation, JpgSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct JpgBaselineBuilderConstruction(JpgAnyBuilder);

    impl ArtifactBuilder for JpgBaselineBuilderConstruction {
        type Snapshot = JpgSnapshot;
        type Mutation = JpgMutation;
        type Diff = JpgDiff;

        fn empty() -> Self {
            Self(JpgAnyBuilder::empty())
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self(JpgAnyBuilder::from_snapshot(snapshot))
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self(JpgAnyBuilder::from_text(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self(JpgAnyBuilder::from_binary(bytes)?))
        }
        fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let (inner, diff) = self.0.mutate(mutation);
            (Self(inner), diff)
        }
        fn absorb(self, diff: Self::Diff) -> Self {
            Self(self.0.absorb(diff))
        }

        /// 🛡️ The real construction gate: however the wrapped snapshot got here, a hard baseline
        /// violation fails `build()` -- soft diagnostics are not surfaced here (`ArtifactBuilder`'s
        /// `build` has no diagnostics-on-success channel), matching `JpgAnyBuilder::build`'s existing
        /// contract of "diagnostics accumulated during mutation, not from validation".
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            let snapshot = self.0.build()?;
            let hard: Vec<dsl::Diagnostic> = check_baseline_conformance(&snapshot).into_iter().filter(|d| matches!(d.severity, dsl::Severity::Error | dsl::Severity::Fatal)).collect();
            if hard.is_empty() {
                Ok(snapshot)
            } else {
                Err(hard)
            }
        }
    }
    //#endregion 🔖️Builder

    #[cfg(test)]
    mod tests {
        use super::*;

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
        fn real_encoded_jpeg_builds_clean_via_from_binary() {
            let (w, h) = (24u32, 24u32);
            let snap = JpgSnapshot { width: w, height: h, pixels: gradient_image(w, h), ..JpgSnapshot::default() };
            let bytes = crate::artifacts::jpg::standards::v_jfif_1_01::engine::encode_jpg(&snap).expect("encode");
            let decoded = crate::artifacts::jpg::standards::v_jfif_1_01::engine::decode_jpg(&bytes).expect("decode");
            let packed = <JpgSnapshot as store::ArtifactPack>::encode_pack(&decoded);
            let built = JpgBaselineBuilderConstruction::from_binary(&packed).expect("from_binary").build().expect("real baseline JPEG must build clean");
            assert!(built.frame.is_some());
        }

        #[test]
        fn empty_snapshot_fails_build_with_no_frame() {
            let err = JpgBaselineBuilderConstruction::empty().build().expect_err("an empty snapshot has no SOF0 frame -- must fail build()");
            assert!(err.iter().any(|d| d.code.0 == crate::artifacts::jpg::standards::v_jfif_1_01::subsets::baseline::schema::CODE_NO_FRAME));
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::jpg::schema::snapshot::JpgHuffmanClass;
    use crate::artifacts::jpg::standards::v_jfif_1_01::subsets::any::schema::JpgAnalyzer as JpgAnyAnalyzer;
    pub use crate::artifacts::jpg::standards::v_jfif_1_01::subsets::any::schema::JpgParts;
    use crate::artifacts::jpg::JpgSnapshot;
    use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    /// 🎯️ This subset's dialect coordinate.
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.jpg", standard: StandardId("jfif-1.01"), subset: SubsetId("baseline") };

    /// 🏷️ SOF0 (baseline sequential DCT) marker byte, T.81 Table B.1.
    pub const SOF0: u8 = 0xC0;

    //#region 🔖️Conformance
    pub const CODE_NO_FRAME: &str = "stdio.jpg.baseline.no-frame";
    pub const CODE_SOF_MARKER: &str = "stdio.jpg.baseline.sof-marker";
    pub const CODE_PRECISION: &str = "stdio.jpg.baseline.precision";
    pub const CODE_ARITHMETIC: &str = "stdio.jpg.baseline.arithmetic-conditioning-present";
    pub const CODE_HUFFMAN_TABLE_COUNT: &str = "stdio.jpg.baseline.huffman-table-count";
    pub const CODE_COMPONENT_SAMPLING: &str = "stdio.jpg.baseline.component-sampling";

    fn hard(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    /// 🛡️ Real ITU-T T.81 / ISO 10918-1 Annex F baseline sequential DCT conformance checks (JFIF
    /// 1.01 container) against one already-decoded `JpgSnapshot`. Shared single source of truth:
    /// `JpgBaselineComposer::compose` hard-gates on this (pre-serialization, authoritative),
    /// `JpgBaselineBuilder::build` hard-gates on this too, and the registered `SubsetValidator`
    /// re-runs it post-hoc against the wire payload for the D5 validate-on-build hook.
    pub fn check_baseline_conformance(snapshot: &JpgSnapshot) -> Vec<Diagnostic> {
        let mut out = Vec::new();

        let Some(frame) = &snapshot.frame else {
            out.push(hard(CODE_NO_FRAME, "no SOF0 frame header retained on this snapshot -- baseline conformance cannot be certified without one (never decoded, or built without going through engine::decode_jpg)".into()));
            return out;
        };

        if snapshot.sof_marker != SOF0 {
            out.push(hard(CODE_SOF_MARKER, format!("frame marker 0x{:02X} is not SOF0 (0x{SOF0:02X}) -- T.81 Annex F baseline sequential DCT is SOF0 only (no progressive/extended/arithmetic SOFn variants)", snapshot.sof_marker)));
        }
        if frame.precision != 8 {
            out.push(hard(CODE_PRECISION, format!("sample precision {} is not 8 -- T.81 §4.2 baseline sequential DCT mandates 8-bit samples", frame.precision)));
        }
        if snapshot.arithmetic {
            out.push(hard(CODE_ARITHMETIC, "a DAC (arithmetic-coding conditioning) segment was present -- T.81 Annex F baseline sequential DCT is Huffman-entropy-coded only".into()));
        }

        let dc_count = snapshot.huffman_tables.iter().filter(|t| t.class == JpgHuffmanClass::Dc).count();
        let ac_count = snapshot.huffman_tables.iter().filter(|t| t.class == JpgHuffmanClass::Ac).count();
        if dc_count > 2 || ac_count > 2 {
            out.push(soft(CODE_HUFFMAN_TABLE_COUNT, format!("{dc_count} DC / {ac_count} AC Huffman table(s) defined -- typical JFIF baseline practice never needs more than 2 of each (one luma, one chroma)")));
        }
        if frame.components.len() > 4 {
            out.push(soft(CODE_COMPONENT_SAMPLING, format!("{} frame components -- JFIF 1.01 conventionally encodes grayscale (1) or YCbCr (3) images; more than 4 is unusual", frame.components.len())));
        }
        for c in &frame.components {
            if !(1..=4).contains(&c.h_sampling) || !(1..=4).contains(&c.v_sampling) {
                out.push(soft(CODE_COMPONENT_SAMPLING, format!("component {} has sampling factors {}x{} outside JFIF's conventional 1..=4 range", c.id, c.h_sampling, c.v_sampling)));
            }
        }
        out
    }
    //#endregion 🔖️Conformance

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.jpg` (jfif-1.01/✳️baseline): delegates the real parse to the ✳️any
    /// subset's analyzer (same `JpgSnapshot`), then folds real T.81 baseline conformance diagnostics
    /// on top. `sniff` also delegates -- a subset-level sniff for `baseline` is "is this recognizable
    /// as a JPEG at all", the same SOI magic-byte probe every jfif-1.01 dialect shares; conformance is
    /// a separate, heavier question answered by `analyze`/`check_baseline_conformance`, not by `sniff`.
    pub struct JpgBaselineAnalyzerAnalysis;

    impl ArtifactAnalysis for JpgBaselineAnalyzerAnalysis {
        type Parts = JpgParts;
        const DIALECT: Dialect = DIALECT;

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            JpgAnyAnalyzer::sniff(source)
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let inner = JpgAnyAnalyzer::analyze(sources);
            let mut diagnostics = inner.diagnostics.clone();
            let mut confidence = inner.confidence;
            if let Some(snapshot) = &inner.parts.snapshot {
                let checks = check_baseline_conformance(snapshot);
                if checks.iter().any(|d| matches!(d.severity, Severity::Error | Severity::Fatal)) {
                    confidence = IoConfidence::Low;
                }
                diagnostics.extend(checks);
            }
            Analysis { parts: inner.parts, dialect: DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::jpg::schema::snapshot::{JpgFrameComponent, JpgFrameHeader, JpgHuffmanTable};

        fn conforming_snapshot() -> JpgSnapshot {
            JpgSnapshot {
                frame: Some(JpgFrameHeader {
                    precision: 8,
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
                huffman_tables: vec![
                    JpgHuffmanTable { id: 0, class: JpgHuffmanClass::Dc, bits: [0; 16], values: vec![] },
                    JpgHuffmanTable { id: 1, class: JpgHuffmanClass::Dc, bits: [0; 16], values: vec![] },
                    JpgHuffmanTable { id: 0, class: JpgHuffmanClass::Ac, bits: [0; 16], values: vec![] },
                    JpgHuffmanTable { id: 1, class: JpgHuffmanClass::Ac, bits: [0; 16], values: vec![] },
                ],
                width: 16,
                height: 16,
                pixels: vec![0u8; 16 * 16 * 4],
                ..JpgSnapshot::default()
            }
        }

        #[test]
        fn conforming_snapshot_has_no_diagnostics() {
            let diagnostics = check_baseline_conformance(&conforming_snapshot());
            assert!(diagnostics.is_empty(), "got {diagnostics:?}");
        }

        #[test]
        fn missing_frame_is_hard() {
            let snapshot = JpgSnapshot::default();
            let diagnostics = check_baseline_conformance(&snapshot);
            assert_eq!(diagnostics.len(), 1);
            assert_eq!(diagnostics[0].code.0, CODE_NO_FRAME);
            assert_eq!(diagnostics[0].severity, Severity::Error);
        }

        #[test]
        fn non_sof0_marker_is_hard() {
            let mut snapshot = conforming_snapshot();
            snapshot.sof_marker = 0xC2; // SOF2 (progressive)
            let diagnostics = check_baseline_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_SOF_MARKER && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[test]
        fn non_8bit_precision_is_hard() {
            let mut snapshot = conforming_snapshot();
            snapshot.frame.as_mut().unwrap().precision = 12;
            let diagnostics = check_baseline_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_PRECISION && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[test]
        fn arithmetic_conditioning_present_is_hard() {
            let mut snapshot = conforming_snapshot();
            snapshot.arithmetic = true;
            let diagnostics = check_baseline_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_ARITHMETIC && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[test]
        fn excess_huffman_tables_is_soft() {
            let mut snapshot = conforming_snapshot();
            snapshot.huffman_tables.push(JpgHuffmanTable { id: 2, class: JpgHuffmanClass::Dc, bits: [0; 16], values: vec![] });
            let diagnostics = check_baseline_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_HUFFMAN_TABLE_COUNT && d.severity == Severity::Warning), "got {diagnostics:?}");
        }

        #[test]
        fn excess_components_is_soft() {
            let mut snapshot = conforming_snapshot();
            let frame = snapshot.frame.as_mut().unwrap();
            frame.components.push(JpgFrameComponent { id: 4, h_sampling: 1, v_sampling: 1, quant_table_id: 1 });
            frame.components.push(JpgFrameComponent { id: 5, h_sampling: 1, v_sampling: 1, quant_table_id: 1 });
            let diagnostics = check_baseline_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_COMPONENT_SAMPLING && d.severity == Severity::Warning), "got {diagnostics:?}");
        }

        #[test]
        fn out_of_range_sampling_is_soft() {
            let mut snapshot = conforming_snapshot();
            snapshot.frame.as_mut().unwrap().components[0].h_sampling = 8;
            let diagnostics = check_baseline_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_COMPONENT_SAMPLING && d.severity == Severity::Warning), "got {diagnostics:?}");
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec JpgBaselineBuilderFacets {
        construction: derived_construction::JpgBaselineBuilderConstruction,
        analysis: derived_analysis::JpgBaselineAnalyzerAnalysis,
        composition: super::io::derived_composition::JpgBaselineComposerComposition,
    }
    builder: JpgBaselineBuilder,
    analyzer: JpgBaselineAnalyzer,
    composer: JpgBaselineComposer,
);
//#endregion 🧬️DerivedArtifactFacets
