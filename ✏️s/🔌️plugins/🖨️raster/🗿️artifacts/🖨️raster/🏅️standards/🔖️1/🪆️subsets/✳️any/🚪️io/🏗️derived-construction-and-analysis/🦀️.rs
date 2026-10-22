//! 🏗️ The derived builder construction and analysis, asset child minting and the artifact kind.

use super::*;

pub mod derived_construction {
    use semio_s_artifact_raster_raster_diff::RasterDiff;
    use semio_s_artifact_raster_raster_mutations::RasterMutation;
    use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct RasterBuilderConstruction {
        snapshot: RasterSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for RasterBuilderConstruction {
        type Snapshot = RasterSnapshot;
        type Mutation = RasterMutation;
        type Diff = RasterDiff;
        fn empty() -> Self {
            Self { snapshot: RasterSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<RasterSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<RasterSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Self::Mutation as protocol::ColdMutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            match protocol::apply_diff_cold(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(semio_framework_diagnostic::Diagnostic::error("build.apply", semio_framework_diagnostic::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = protocol::apply_diff_cold(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
}
pub use derived_construction::*;

pub mod derived_analysis {
    use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct RasterParts {
        pub snapshot: Option<RasterSnapshot>,
    }

    pub struct RasterAnalyzerAnalysis;

    impl ArtifactAnalysis for RasterAnalyzerAnalysis {
        type Parts = RasterParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.raster.raster", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            semio_framework_plugin::io::Confidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = RasterParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <RasterSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <RasterSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec RasterBuilderFacets {
        construction: RasterBuilderConstruction,
        analysis: RasterAnalyzerAnalysis,
        composition: RasterComposerComposition,
    }
    builder: RasterBuilder,
    analyzer: RasterAnalyzer,
    composer: RasterComposer,
);



/// 🌉️ Resolves only content owned by the exact child in this snapshot. A wire-decoded child that
/// has not yet been materialized by the host resolver fails soft without consulting shared state.
pub fn raster_asset(assets: &RasterOwnedMap<RasterAssetChild>, asset_id: &str) -> Option<RasterImageAsset> {
    let handle = assets.get(asset_id)?;
    let image = handle.local_owner::<SemioImageSnapshot>()?;
    crate::raster_asset_from_semio_image_snapshot(image.as_ref()).ok()
}

/// 🏷️ The `2d.raster` artifact kind — lifted out of `create_raster_app`'s `.artifact_kind(…)` call so
/// both the app manifest and (in the future) any other consumer can share one definition.
///
/// 🗄️ `export_stdio_kinds`/`import_stdio_kinds` are READ from `🚪️io`'s own two functions rather than
/// re-listed here (🗒️note's convention, `🗒️note/🗿️artifacts/🗒️note/🦀️.rs:101-102`). Before this the
/// two surfaces disagreed three ways — `🚪️io` listed all 9 formats, this spec listed 2, and
/// `io_registry::entries()` registered 9 — which matters because
/// `negotiate_wire_format` (`🧰️framework/🛍️products/💻️os/🖥️host/🔀️workflow/🦀️.rs`) picks a workflow wire
/// straight out of THESE lists: anything listed here that has no real codec becomes an edge that
/// always fails at run time. The io functions now list only the hops that genuinely encode/decode;
/// the composer registry deliberately still carries all 9 so a refused hop answers with its own
/// typed reason instead of a bare "no route" (block's `📓️w3-io.md` §2 records the same rule).
/// `export_formats`/`import_formats` stay empty: they are the pre-stdio format-enum peer that
/// 🗒️note, 🧱️block and every stdio artifact also leave empty.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "2d.raster".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("2D Raster", "2D-Rasterbild"),
        source_format: "raster.document".into(),
        component_kind: "raster".into(),
        dimension: "2d".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Raster },
        schema: RASTER_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: crate::export_stdio_kinds().iter().map(|kind| (*kind).to_owned()).collect(),
        import_stdio_kinds: crate::import_stdio_kinds().iter().map(|kind| (*kind).to_owned()).collect(),
    }
}
