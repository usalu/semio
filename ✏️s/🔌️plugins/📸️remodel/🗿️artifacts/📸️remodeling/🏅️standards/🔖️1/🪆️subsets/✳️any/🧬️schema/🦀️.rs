//! 🧬️ Remodeling artifact schema — every field of the artifact with its state class.

use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingDurableArtifactStore;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::VideoCodec;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::MediaStream;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingAssetChild;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::CalibrationState;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::ReconstructionParams;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::GroundControlPoint;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::ReconstructionResults;

//#region 🔖️ArtifactKind
/// 🗿️ The `3d.remodeling` artifact kind — lifted verbatim out of the manifest builder's
/// `.artifact_kind(…)` literal so the artifact node, not the app, owns its own identity.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
    ArtifactKindSpec {
        id: "3d.remodeling".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("3D Remodeling", "3D-Umbau"),
        source_format: "remodeling.scene".into(),
        component_kind: "remodeling".into(),
        dimension: "3d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
        schema: "remodeling.scene".into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.dwg".into(), "stdio.gltf".into(), "stdio.json".into(), "stdio.las".into(), "stdio.obj".into(), "stdio.ply".into(), "stdio.png".into(), "stdio.stl".into()],
        import_stdio_kinds: vec!["stdio.dwg".into(), "stdio.gltf".into(), "stdio.json".into(), "stdio.las".into(), "stdio.obj".into(), "stdio.ply".into(), "stdio.png".into(), "stdio.stl".into()],
    }
}
//#endregion 🔖️ArtifactKind

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

//#region 🔖️Leaves
/// 🧾️ The artifact facet's five format leaves.
pub const ARTIFACT_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🔖️Leaves
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
