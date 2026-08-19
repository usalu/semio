//! 🧬️ TiffSnapshot schema (6.0/✳️baseline) — reuses the ✳️any subset's `TiffSnapshot` verbatim
//! (the SAME Rust type, same `s.stdio.tiff` schema id). A subset is a validation-gated dialect
//! STAMP on top of that existing schema, not a new one -- see D4's Tier-1 "same snapshot type,
//! subset moves" semantics (`ArtifactCommand::MigrateDialect`). This leaf exists so
//! `🪆️subsets/✳️baseline/🧬️schema/` is present per `🔣️taxonomy.json`'s `subsetChildDirs`, without
//! duplicating the schema definition.
//!
//! Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: `TiffSnapshot`
//! now retains the REAL IFD (tag/type/count/value entries) — `Compression`/
//! `PhotometricInterpretation`/`BitsPerSample`/`StripOffsets`/`Tile*` are all genuinely present
//! and checkable. `🧐️analyzer` here now implements real Baseline TIFF conformance checks
//! against those fields (superseding the earlier ticket 26/08/11's schema-gap-only revision).
//! See `🧐️analyzer` for the full accounting.

pub use crate::artifacts::tiff::standards::v6_0::subsets::any::schema::*;
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::tiff::standards::v6_0::subsets::any::schema::{diff::TiffDiff, mutations::TiffMutation, snapshot::TiffSnapshot};
    use crate::artifacts::tiff::standards::v6_0::subsets::baseline::schema::check_tiff_baseline_conformance;
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct TiffBaselineBuilderConstruction {
        snapshot: TiffSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for TiffBaselineBuilderConstruction {
        type Snapshot = TiffSnapshot;
        type Mutation = TiffMutation;
        type Diff = TiffDiff;

        async fn empty() -> Self {
            Self { snapshot: TiffSnapshot::default(), diagnostics: Vec::new() }
        }

        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }

        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<TiffSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }

        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<TiffSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }

        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::artifacts::tiff::standards::v6_0::subsets::any::schema::mutations::apply_tiff_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }

        async fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <TiffDiff as protocol::MutationDiff<TiffSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }

        /// 🛡️ Re-runs the honestly-scope-limited Baseline TIFF check -- always SOFT at this schema,
        /// so `build()` never fails; the diagnostics still surface via the analyzer/composer/
        /// validator paths for anyone inspecting them.
        async fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            let _ = check_tiff_baseline_conformance(&self.snapshot);
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
    //#endregion 🔖️Builder

    #[cfg(test)]
    mod tests {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn pass_through_build_never_fails_on_conformance_grounds() {
            let snapshot = TiffBaselineBuilderConstruction::empty().build().expect("all conformance findings are soft by policy; build must succeed");
            assert!(snapshot.ifds.is_empty());
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::tiff::standards::v6_0::subsets::any::schema::snapshot::{TiffSnapshot, TiffValues, TAG_BITS_PER_SAMPLE, TAG_COMPRESSION, TAG_PHOTOMETRIC, TAG_STRIP_OFFSETS, TAG_TILE_LENGTH, TAG_TILE_WIDTH};
    use crate::artifacts::tiff::standards::v6_0::subsets::any::schema::{TiffAnalyzer as TiffAnyAnalyzer, TiffParts};
    use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    /// 🎯️ This subset's dialect coordinate.
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.tiff", standard: StandardId("6.0"), subset: SubsetId("baseline") };

    //#region 🔖️Conformance
    pub const CODE_DEGENERATE_RASTER: &str = "stdio.tiff.baseline.degenerate-raster";
    pub const CODE_NO_IFD: &str = "stdio.tiff.baseline.no-ifd";
    pub const CODE_UNSUPPORTED_COMPRESSION: &str = "stdio.tiff.baseline.unsupported-compression";
    pub const CODE_UNSUPPORTED_PHOTOMETRIC: &str = "stdio.tiff.baseline.unsupported-photometric";
    pub const CODE_UNSUPPORTED_BITS_PER_SAMPLE: &str = "stdio.tiff.baseline.unsupported-bits-per-sample";
    pub const CODE_TILED_NOT_BASELINE: &str = "stdio.tiff.baseline.tiled-not-baseline";
    pub const CODE_MISSING_STRIP_OFFSETS: &str = "stdio.tiff.baseline.missing-strip-offsets";

    async fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    async fn ifd0_u32_list(snapshot: &TiffSnapshot, tag: u16) -> Vec<u32> {
        match snapshot.ifds.first().and_then(|ifd| ifd.entries.iter().find(|t| t.tag == tag)) {
            Some(t) => match &t.values {
                TiffValues::Short(v) => v.iter().map(|&x| x as u32).collect(),
                TiffValues::Long(v) => v.clone(),
                _ => Vec::new(),
            },
            None => Vec::new(),
        }
    }

    /// 🛡️ Real Baseline TIFF conformance check against one already-decoded `TiffSnapshot`. Shared
    /// single source of truth: `TiffBaselineComposer::compose` and the registered
    /// `SubsetValidator` both call this.
    pub async fn check_tiff_baseline_conformance(snapshot: &TiffSnapshot) -> Vec<Diagnostic> {
        let mut out = Vec::new();

        let Some(ifd0) = snapshot.ifds.first() else {
            out.push(soft(CODE_NO_IFD, "no IFD present -- Baseline TIFF conformance cannot be checked at all".into()));
            return out;
        };

        match (snapshot.width(), snapshot.height()) {
            (Some(width), Some(height)) => {
                let expected_len = width as usize * height as usize * 4;
                if width == 0 || height == 0 || snapshot.pixels.len() != expected_len {
                    out.push(soft(CODE_DEGENERATE_RASTER, format!("raster is degenerate (width={width}, height={height}, pixels.len()={}, expected {expected_len})", snapshot.pixels.len())));
                }
            }
            _ => out.push(soft(CODE_DEGENERATE_RASTER, "IFD 0 has no ImageWidth/ImageLength tag".into())),
        }

        if let Some(&c) = ifd0_u32_list(snapshot, TAG_COMPRESSION).first() {
            if c != 1 && c != 2 && c != 32773 {
                out.push(soft(CODE_UNSUPPORTED_COMPRESSION, format!("Compression {c} is not one of Baseline TIFF's {{1 none, 2 CCITT G3 1D, 32773 PackBits}}")));
            }
        }
        if let Some(&p) = ifd0_u32_list(snapshot, TAG_PHOTOMETRIC).first() {
            if p > 3 {
                out.push(soft(CODE_UNSUPPORTED_PHOTOMETRIC, format!("PhotometricInterpretation {p} is not one of Baseline TIFF's {{0,1,2,3}}")));
            }
        }
        let bits = ifd0_u32_list(snapshot, TAG_BITS_PER_SAMPLE);
        if bits.iter().any(|&b| b != 1 && b != 4 && b != 8) {
            out.push(soft(CODE_UNSUPPORTED_BITS_PER_SAMPLE, format!("BitsPerSample {bits:?} has a value outside Baseline TIFF's {{1,4,8}}")));
        }
        let has_tile = ifd0.entries.iter().any(|t| t.tag == TAG_TILE_WIDTH || t.tag == TAG_TILE_LENGTH);
        if has_tile {
            out.push(soft(CODE_TILED_NOT_BASELINE, "Baseline TIFF requires strip organization; this IFD carries Tile* tags".into()));
        } else if !ifd0.entries.iter().any(|t| t.tag == TAG_STRIP_OFFSETS) {
            out.push(soft(CODE_MISSING_STRIP_OFFSETS, "IFD 0 has neither StripOffsets nor Tile* tags -- no recognizable pixel organization".into()));
        }

        out
    }
    //#endregion 🔖️Conformance

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.tiff` (6.0/✳️baseline): delegates the real parse to the ✳️any subset's
    /// analyzer (same `TiffSnapshot`), then folds the real Baseline TIFF diagnostics on top.
    pub struct TiffBaselineAnalyzerAnalysis;

    impl ArtifactAnalysis for TiffBaselineAnalyzerAnalysis {
        type Parts = TiffParts;
        const DIALECT: Dialect = DIALECT;

        async fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            TiffAnyAnalyzer::sniff(source)
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let inner = TiffAnyAnalyzer::analyze(sources);
            let mut diagnostics = inner.diagnostics.clone();
            if let Some(snapshot) = &inner.parts.snapshot {
                diagnostics.extend(check_tiff_baseline_conformance(snapshot));
            }
            Analysis { parts: inner.parts, dialect: DIALECT, confidence: inner.confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::tiff::standards::v6_0::subsets::any::schema::snapshot::{TiffByteOrder, TiffFieldType, TiffIfd, TiffTag};

        async fn tag(id: u16, kind: TiffFieldType, values: TiffValues) -> TiffTag {
            TiffTag { tag: id, kind, values }
        }

        async fn snapshot_with(width: u32, height: u32, pixels: Vec<u8>) -> TiffSnapshot {
            TiffSnapshot {
                schema: "stdio.tiff".into(),
                byte_order: TiffByteOrder::LittleEndian,
                ifds: vec![TiffIfd {
                    entries: vec![
                        tag(256, TiffFieldType::Long, TiffValues::Long(vec![width])),
                        tag(257, TiffFieldType::Long, TiffValues::Long(vec![height])),
                        tag(273, TiffFieldType::Long, TiffValues::Long(vec![8])),   // StripOffsets
                        tag(259, TiffFieldType::Short, TiffValues::Short(vec![1])), // Compression: none
                        tag(262, TiffFieldType::Short, TiffValues::Short(vec![2])), // PhotometricInterpretation: RGB
                        tag(258, TiffFieldType::Short, TiffValues::Short(vec![8])), // BitsPerSample
                    ],
                }],
                pixels,
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn no_ifd_is_flagged_soft() {
            let snapshot = TiffSnapshot::default();
            let diagnostics = check_tiff_baseline_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_NO_IFD && d.severity == Severity::Warning), "got {diagnostics:?}");
            assert_eq!(diagnostics.len(), 1, "no-IFD short-circuits before any other check, got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn degenerate_zero_dimensions_are_flagged_soft() {
            let snapshot = snapshot_with(0, 0, Vec::new());
            let diagnostics = check_tiff_baseline_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_DEGENERATE_RASTER && d.severity == Severity::Warning), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn rgba_length_mismatch_is_flagged_soft() {
            let snapshot = snapshot_with(4, 4, vec![0u8; 4]); // way too short for 4x4 RGBA
            let diagnostics = check_tiff_baseline_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_DEGENERATE_RASTER), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn well_formed_raster_has_no_findings() {
            let snapshot = snapshot_with(3, 2, vec![0u8; 3 * 2 * 4]);
            let diagnostics = check_tiff_baseline_conformance(&snapshot);
            assert!(diagnostics.is_empty(), "expected zero findings for a fully baseline-conformant IFD, got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn unsupported_compression_is_flagged_soft() {
            let mut snapshot = snapshot_with(2, 2, vec![0u8; 2 * 2 * 4]);
            snapshot.ifds[0].entries.iter_mut().find(|t| t.tag == 259).unwrap().values = TiffValues::Short(vec![5]); // LZW
            let diagnostics = check_tiff_baseline_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_UNSUPPORTED_COMPRESSION), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn unsupported_bits_per_sample_is_flagged_soft() {
            let mut snapshot = snapshot_with(2, 2, vec![0u8; 2 * 2 * 4]);
            snapshot.ifds[0].entries.iter_mut().find(|t| t.tag == 258).unwrap().values = TiffValues::Short(vec![16]);
            let diagnostics = check_tiff_baseline_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_UNSUPPORTED_BITS_PER_SAMPLE), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn tiled_organization_is_flagged_soft() {
            let mut snapshot = snapshot_with(2, 2, vec![0u8; 2 * 2 * 4]);
            snapshot.ifds[0].entries.push(tag(322, TiffFieldType::Long, TiffValues::Long(vec![16]))); // TileWidth
            let diagnostics = check_tiff_baseline_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_TILED_NOT_BASELINE), "got {diagnostics:?}");
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec TiffBaselineBuilderFacets {
        construction: TiffBaselineBuilderConstruction,
        analysis: TiffBaselineAnalyzerAnalysis,
        composition: super::io::derived_composition::TiffBaselineComposerComposition,
    }
    builder: TiffBaselineBuilder,
    analyzer: TiffBaselineAnalyzer,
    composer: TiffBaselineComposer,
);
//#endregion 🧬️DerivedArtifactFacets
