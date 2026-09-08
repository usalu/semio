//! 🧬️ TiffSnapshot schema (6.0/🧱️baseline) — reuses the ✳️any subset's `TiffSnapshot` verbatim
//! (the SAME Rust type, same `s.stdio.tiff` schema id). A subset is a validation-gated dialect
//! STAMP on top of that existing schema, not a new one -- see D4's Tier-1 "same snapshot type,
//! subset moves" semantics (`ArtifactCommand::MigrateDialect`). This leaf exists so
//! `🪆️subsets/🧱️baseline/🧬️schema/` is present per `🔣️taxonomy.json`'s `subsetChildDirs`, without
//! duplicating the schema definition.
//!
//! Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: `TiffSnapshot`
//! now retains the REAL IFD (tag/type/count/value entries) — `Compression`/
//! `PhotometricInterpretation`/`BitsPerSample`/`StripOffsets`/`Tile*` are all genuinely present
//! and checkable. `🧐️analyzer` here now implements real Baseline TIFF conformance checks
//! against those fields (superseding the earlier ticket 26/08/11's schema-gap-only revision).
//! See `🧐️analyzer` for the full accounting.

pub use crate::standards::v6_0::subsets::document::schema::*;
//#region 🧬️Mutations
// 🧬️ This subset's OWN conformance-class vocabulary, mounted here rather than in the crate's shared
// `🦀️.rs` — the same placement, and the same rationale, the ✳️strict/✳️transitional OOXML
// subsets already use for theirs: that file is one wiring file for every stdio artifact at once,
// and an artifact owns the subtree it owns. `#[path]` on a non-inline module resolves against this
// file's own directory. The explicit declaration shadows the glob re-export of ✳️any's `mutations`
// above, which is what puts this subset's own vocabulary at
// `subsets::baseline::schema::mutations` while ✳️any's document vocabulary stays reachable at its
// own address.
#[path = "🧬️mutations/🦀️.rs"]
pub mod mutations;
//#endregion 🧬️Mutations
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::standards::v6_0::subsets::document::schema::{diff::TiffDiff, mutations::TiffMutation, snapshot::TiffSnapshot};
    use crate::standards::v6_0::subsets::baseline::schema::check_tiff_baseline_conformance;
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

        fn empty() -> Self {
            Self { snapshot: TiffSnapshot::default(), diagnostics: Vec::new() }
        }

        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }

        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<TiffSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }

        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<TiffSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }

        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::standards::v6_0::subsets::document::schema::mutations::apply_tiff_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }

        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <TiffDiff as protocol::MutationDiff<TiffSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }

        /// 🛡️ Re-runs the honestly-scope-limited Baseline TIFF check -- always SOFT at this schema,
        /// so `build()` never fails; the diagnostics still surface via the analyzer/composer/
        /// validator paths for anyone inspecting them.
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
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
    include!("🧪️tests/🔬️derived-construction-unit/🦀️.rs");
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::standards::v6_0::subsets::document::schema::snapshot::{TiffSnapshot, TiffValues, TAG_BITS_PER_SAMPLE, TAG_COMPRESSION, TAG_PHOTOMETRIC, TAG_STRIP_OFFSETS, TAG_TILE_LENGTH, TAG_TILE_WIDTH};
    use crate::standards::v6_0::subsets::document::schema::{TiffAnalyzer as TiffAnyAnalyzer, TiffParts};
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn ifd0_u32_list(snapshot: &TiffSnapshot, tag: u16) -> Vec<u32> {
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_tiff_baseline_conformance(snapshot: &TiffSnapshot) -> Vec<Diagnostic> {
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
    /// 🧐️ Analyzes `stdio.tiff` (6.0/🧱️baseline): delegates the real parse to the ✳️any subset's
    /// analyzer (same `TiffSnapshot`), then folds the real Baseline TIFF diagnostics on top.
    pub struct TiffBaselineAnalyzerAnalysis;

    impl ArtifactAnalysis for TiffBaselineAnalyzerAnalysis {
        type Parts = TiffParts;
        const DIALECT: Dialect = DIALECT;

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            TiffAnyAnalyzer::sniff(source)
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
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
    include!("🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
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
