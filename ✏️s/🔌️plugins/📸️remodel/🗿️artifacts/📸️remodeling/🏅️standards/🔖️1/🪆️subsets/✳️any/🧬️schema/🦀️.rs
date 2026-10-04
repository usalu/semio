//! 🧬️ Remodeling artifact schema — every field of the artifact with its state class.

use crate::{RemodelingDurableArtifactStore, RemodelingSnapshot};
use framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;

//#region 🔖️Ids
/// 🆔️ The id one admitted command mints for its new `prefix` entity (a stream, a GCP): content-addressed over the
/// command's [`semio_framework_plugin::AppOperationContext::authoring_seed`] — unique per writer and admission — so two
/// writers, two sessions or a reload never mint one id twice (a process counter restarted at `stream-1` after every
/// reload), and the import transaction minted from a stream id is unique with it. A view without an admission mints from
/// the empty seed. See [`store::content_id`].
pub fn mint_remodeling_id(operation: Option<&semio_framework_plugin::AppOperationContext>, prefix: &str) -> String {
    store::content_id(prefix, format!("{}\u{1f}{prefix}", operation.map_or("", |operation| operation.authoring_seed.as_str())).as_bytes())
}
//#endregion 🔖️Ids

//#region 🔖️Codecs
/// 🎞️ Label → document `VideoCodec` — pure string parsing, no engine dependency (relocated from
/// `⚙️engine/🦀️.rs`, #2553).
pub fn video_codec_from_label(label: &str) -> VideoCodec {
    match label.to_ascii_lowercase().as_str() {
        "avc" | "h264" | "h.264" => VideoCodec::Avc,
        "hevc" | "h265" | "h.265" => VideoCodec::Hevc,
        "vp9" => VideoCodec::Vp9,
        "av1" => VideoCodec::Av1,
        "mjpeg" | "mjpg" => VideoCodec::Mjpeg,
        _ => VideoCodec::Unknown,
    }
}
//#endregion 🔖️Codecs

//#region 🔖️Artifact
/// 🧬️ remodeling document artifact state.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.remodel.remodeling")]
pub struct RemodelingArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    pub streams: Vec<MediaStream>,
    #[state(artifact)]
    pub assets: BTreeMap<String, RemodelingAssetChild>,
    #[state(artifact)]
    pub durable_artifacts: RemodelingDurableArtifactStore,
    #[state(artifact)]
    pub calibration: CalibrationState,
    #[state(artifact)]
    pub params: ReconstructionParams,
    #[state(artifact)]
    pub gcps: Vec<GroundControlPoint>,
    #[state(artifact)]
    pub results: ReconstructionResults,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for RemodelingArtifact {
    fn default() -> Self {
        Self::from_snapshot(RemodelingSnapshot::default())
    }
}

impl RemodelingArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> RemodelingSnapshot {
        RemodelingSnapshot {
            schema: self.schema.clone(),
            id: self.id.clone(),
            streams: self.streams.clone(),
            assets: self.assets.clone(),
            durable_artifacts: self.durable_artifacts.clone(),
            calibration: self.calibration.clone(),
            params: self.params.clone(),
            gcps: self.gcps.clone(),
            results: self.results.clone(),
        }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: RemodelingSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            id: snapshot.id,
            streams: snapshot.streams,
            assets: snapshot.assets,
            durable_artifacts: snapshot.durable_artifacts,
            calibration: snapshot.calibration,
            params: snapshot.params,
            gcps: snapshot.gcps,
            results: snapshot.results,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: RemodelingSnapshot) {
        self.schema = snapshot.schema;
        self.id = snapshot.id;
        self.streams = snapshot.streams;
        self.assets = snapshot.assets;
        self.durable_artifacts = snapshot.durable_artifacts;
        self.calibration = snapshot.calibration;
        self.params = snapshot.params;
        self.gcps = snapshot.gcps;
        self.results = snapshot.results;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.remodel.remodeling` — twenty handcrafted schema leaves.
pub fn remodeling_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.remodel.remodeling",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::schema::diff::RemodelingDiff;
    use crate::schema::mutations::RemodelingMutation;
    use crate::schema::snapshot::RemodelingSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct RemodelingBuilderConstruction {
        snapshot: RemodelingSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for RemodelingBuilderConstruction {
        type Snapshot = RemodelingSnapshot;
        type Mutation = RemodelingMutation;
        type Diff = RemodelingDiff;
        fn empty() -> Self {
            Self { snapshot: RemodelingSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<RemodelingSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<RemodelingSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <RemodelingMutation as protocol::Mutation<RemodelingSnapshot>>::diff(&mutation, &self.snapshot);
            match protocol::MutationDiff::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(semio_framework_diagnostic::Diagnostic::error("build.apply", semio_framework_diagnostic::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <RemodelingDiff as protocol::MutationDiff<RemodelingSnapshot>>::apply(&diff, &self.snapshot)?;
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
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::RemodelingSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct RemodelingParts {
        pub snapshot: Option<RemodelingSnapshot>,
    }

    pub struct RemodelingAnalyzerAnalysis;

    impl ArtifactAnalysis for RemodelingAnalyzerAnalysis {
        type Parts = RemodelingParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.remodel.remodeling", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = RemodelingParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <RemodelingSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <RemodelingSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
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
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec RemodelingBuilderFacets {
        construction: RemodelingBuilderConstruction,
        analysis: RemodelingAnalyzerAnalysis,
        composition: super::super::io::derived_composition::RemodelingComposerComposition,
    }
    builder: RemodelingBuilder,
    analyzer: RemodelingAnalyzer,
    composer: RemodelingComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::CalibrationState;
pub use crate::GroundControlPoint;
pub use crate::MediaStream;
pub use crate::ReconstructionParams;
pub use crate::ReconstructionResults;
pub use crate::RemodelingAssetChild;
pub use crate::VideoCodec;
//#endregion 🔁️Re-exports
