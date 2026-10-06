//! 🚪️ IO stdio.tiff (6.0/🧱️baseline) — reuses the ✳️any subset's `binary` raw-codec DAG leaf
//! rather than duplicating it (same `TiffSnapshot` type, same catalog DAG edges). Registration
//! flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level aggregator,
//! and the `SubsetValidator` directly), not per-leaf `register()` — same pattern `✳️any/🚪️io`
//! already established for this artifact.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v6_0::subsets::baseline::schema::check_tiff_baseline_conformance;
    use crate::standards::v6_0::subsets::document::schema::snapshot::TiffSnapshot;
    use crate::standards::v6_0::subsets::document::io::TiffComposer as TiffAnyComposer;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
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

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
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

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
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
                    scope: semio_framework_diagnostic::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<TiffBaselineValidator>)
    }

    /// 📌️ Registers this subset's `SubsetValidator`. Called from 6.0's own `⚙️engine::register()`.
    /// The `ComposerEntry` itself is registered separately via this standard's own
    /// `composer::entries()` aggregation.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_subset_validator(validator_entry()).expect("static Stdio registration must be available and conflict-free");
    }
    //#endregion 🔖️SubsetValidator

    #[cfg(test)]
    include!("🧪️tests/🔬️derived-composition-unit/🦀️.rs");
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

pub mod derived_construction {
    use crate::standards::v6_0::subsets::baseline::schema::check_tiff_baseline_conformance;
    use crate::standards::v6_0::subsets::document::schema::{diff::TiffDiff, mutations::TiffMutation, snapshot::TiffSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct TiffBaselineBuilderConstruction {
        snapshot: TiffSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
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

        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
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
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
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

pub mod derived_analysis {
    use crate::standards::v6_0::subsets::document::schema::snapshot::{TiffSnapshot, TiffValues, TAG_BITS_PER_SAMPLE, TAG_COMPRESSION, TAG_PHOTOMETRIC, TAG_STRIP_OFFSETS, TAG_TILE_LENGTH, TAG_TILE_WIDTH};
    use crate::standards::v6_0::subsets::document::schema::{TiffAnalyzer as TiffAnyAnalyzer, TiffParts};
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::FaultScope;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
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
                if width == 0 || height == 0 || ifd0.storage.chunks.is_empty() {
                    out.push(soft(CODE_DEGENERATE_RASTER, format!("raster is degenerate (width={width}, height={height}, chunks={})", ifd0.storage.chunks.len())));
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
        let has_tile = ifd0.storage.kind == super::snapshot::TiffStorageKind::Tiles || ifd0.entries.iter().any(|t| t.tag == TAG_TILE_WIDTH || t.tag == TAG_TILE_LENGTH);
        if has_tile {
            out.push(soft(CODE_TILED_NOT_BASELINE, "Baseline TIFF requires strip organization; this IFD carries Tile* tags".into()));
        } else if ifd0.storage.kind != super::snapshot::TiffStorageKind::Strips {
            out.push(soft(CODE_MISSING_STRIP_OFFSETS, "IFD 0 has neither StripOffsets nor Tile* tags -- no recognizable pixel organization".into()));
        }

        out
    }

    /// 🛡️ Checks the same native baseline rules with borrowed, bounded traversal.
    pub fn check_tiff_baseline_conformance_controlled(snapshot:&TiffSnapshot,control:&mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl<'_>)->Result<Vec<Diagnostic>,semio_framework_value::ValueError>{
        use semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotPhase;
        use semio_framework_value::{ValueError,ValueRefusalKind};
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;let mut out=Vec::new();let Some(ifd)=snapshot.ifds.first()else{out.push(soft(CODE_NO_IFD,"no IFD present -- Baseline TIFF conformance cannot be checked at all".into()));return Ok(out);};
        let mut width=None;let mut height=None;let mut compression=None;let mut photometric=None;let mut bits=None;let mut strip=false;let mut tiled=false;let mut checked=0usize;
        for tag in &ifd.entries{checked=checked.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"TIFF baseline traversal count overflow"))?;if checked%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,checked,0)?;}match tag.tag{super::snapshot::TAG_IMAGE_WIDTH if width.is_none()=>width=Some(&tag.values),super::snapshot::TAG_IMAGE_LENGTH if height.is_none()=>height=Some(&tag.values),TAG_COMPRESSION if compression.is_none()=>compression=Some(&tag.values),TAG_PHOTOMETRIC if photometric.is_none()=>photometric=Some(&tag.values),TAG_BITS_PER_SAMPLE if bits.is_none()=>bits=Some(&tag.values),TAG_TILE_WIDTH|TAG_TILE_LENGTH=>tiled=true,_=>{}}}strip=ifd.storage.kind==super::snapshot::TiffStorageKind::Strips;tiled|=ifd.storage.kind==super::snapshot::TiffStorageKind::Tiles;
        match(width.and_then(TiffValues::first_u32),height.and_then(TiffValues::first_u32)){(Some(width),Some(height))=>{if width==0||height==0||ifd.storage.chunks.is_empty(){out.push(soft(CODE_DEGENERATE_RASTER,format!("raster is degenerate (width={width}, height={height}, chunks={})",ifd.storage.chunks.len())));}},_=>out.push(soft(CODE_DEGENERATE_RASTER,"IFD 0 has no ImageWidth/ImageLength tag".into()))}
        fn first(value:Option<&TiffValues>)->Option<u32>{match value{Some(TiffValues::Short(v))=>v.first().map(|&v|u32::from(v)),Some(TiffValues::Long(v))=>v.first().copied(),_=>None}}
        if let Some(c)=first(compression){if c!=1&&c!=2&&c!=32773{out.push(soft(CODE_UNSUPPORTED_COMPRESSION,format!("Compression {c} is not one of Baseline TIFF's {{1 none, 2 CCITT G3 1D, 32773 PackBits}}")));}}
        if let Some(p)=first(photometric){if p>3{out.push(soft(CODE_UNSUPPORTED_PHOTOMETRIC,format!("PhotometricInterpretation {p} is not one of Baseline TIFF's {{0,1,2,3}}")));}}
        let mut invalid=false;match bits{Some(TiffValues::Short(values))=>for &v in values{checked=checked.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"TIFF baseline traversal count overflow"))?;if checked%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,checked,0)?;}invalid|=!matches!(v,1|4|8);},Some(TiffValues::Long(values))=>for &v in values{checked=checked.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"TIFF baseline traversal count overflow"))?;if checked%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,checked,0)?;}invalid|=!matches!(v,1|4|8);},_=>{}}
        if invalid{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,checked,checked)?;let repr=match bits{Some(TiffValues::Short(v))=>format!("{v:?}"),Some(TiffValues::Long(v))=>format!("{v:?}"),_=>return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"missing TIFF bits-per-sample values"))};out.push(soft(CODE_UNSUPPORTED_BITS_PER_SAMPLE,format!("BitsPerSample {repr} has a value outside Baseline TIFF's {{1,4,8}}")));}
        if tiled{out.push(soft(CODE_TILED_NOT_BASELINE,"Baseline TIFF requires strip organization; this IFD carries Tile* tags".into()));}else if !strip{out.push(soft(CODE_MISSING_STRIP_OFFSETS,"IFD 0 has neither StripOffsets nor Tile* tags -- no recognizable pixel organization".into()));}control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,checked,checked)?;Ok(out)
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
