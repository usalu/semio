//! 🧬️ Puzzle3d artifact schema — every field of the artifact with its state class.

use crate::artifacts::puzzle3d::{Puzzle3dAttraction, Puzzle3dMeta, Puzzle3dObject, Puzzle3dReference, Puzzle3dSnapshot, Puzzle3dTargetVolume};
use artifact_schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full puzzle3d artifact state across the artifact, presence and config lanes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.puzzle.puzzle3d")]
pub struct Puzzle3dArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub domain: String,
    #[state(artifact)]
    pub meta: Puzzle3dMeta,
    #[state(artifact)]
    pub objects: Vec<Puzzle3dObject>,
    #[state(artifact)]
    pub attractions: Vec<Puzzle3dAttraction>,
    #[state(artifact)]
    pub target_volumes: Vec<Puzzle3dTargetVolume>,
    #[state(artifact)]
    pub references: Vec<Puzzle3dReference>,
    #[state(presence)]
    pub selected_object_ids: Vec<String>,
    #[state(presence)]
    pub selected_vortex_ids: Vec<String>,
    #[state(presence)]
    pub selected_attraction_ids: Vec<String>,
    #[state(presence)]
    pub selected_target_volume_ids: Vec<String>,
    #[state(presence)]
    pub selected_reference_ids: Vec<String>,
    #[state(presence)]
    pub active_utility_id: String,
    #[state(config)]
    pub camera_position_x: f64,
    #[state(config)]
    pub camera_position_y: f64,
    #[state(config)]
    pub camera_position_z: f64,
    #[state(config)]
    pub camera_target_x: f64,
    #[state(config)]
    pub camera_target_y: f64,
    #[state(config)]
    pub camera_target_z: f64,
    #[state(config)]
    pub camera_zoom: f64,
    #[state(config)]
    pub selection_method: String,
    #[state(config)]
    pub selection_mode_default: String,
    #[state(config)]
    pub engagement_input: String,
    #[state(config)]
    pub grid_visible: bool,
    #[state(config)]
    pub grid_snap_enabled: bool,
    #[state(config)]
    pub grid_spacing: f64,
    #[state(config)]
    pub overlap_budget: f64,
    #[state(config)]
    pub fill_count: u32,
    #[state(config)]
    pub brush_candidate_index: u32,
    #[state(config)]
    pub lod_automatic: bool,
    #[state(config)]
    pub lod_depth_variable: bool,
    #[state(config)]
    pub lod_manual: f64,
    #[state(config)]
    pub proximity_radius: f64,
    #[state(config)]
    pub locale: String,
    #[state(config)]
    pub runtime_extras_json: String,
    #[state(artifact)]
    pub hovered_object_id: Option<String>,
    #[state(artifact)]
    pub hovered_vortex_full_id: Option<String>,
    #[state(artifact)]
    pub hovered_kind_id: Option<String>,
    #[state(artifact)]
    pub preview_seq: i64,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for Puzzle3dArtifact {
    fn default() -> Self {
        Self::from_snapshot(Puzzle3dSnapshot::default())
    }
}

impl Puzzle3dArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> Puzzle3dSnapshot {
        Puzzle3dSnapshot {
            schema: self.schema.clone(),
            domain: self.domain.clone(),
            meta: self.meta.clone(),
            objects: self.objects.clone(),
            attractions: self.attractions.clone(),
            target_volumes: self.target_volumes.clone(),
            references: self.references.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: Puzzle3dSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            domain: snapshot.domain,
            meta: snapshot.meta,
            objects: snapshot.objects,
            attractions: snapshot.attractions,
            target_volumes: snapshot.target_volumes,
            references: snapshot.references,
            selected_object_ids: Vec::new(),
            selected_vortex_ids: Vec::new(),
            selected_attraction_ids: Vec::new(),
            selected_target_volume_ids: Vec::new(),
            selected_reference_ids: Vec::new(),
            active_utility_id: "select".into(),
            camera_position_x: 0.0,
            camera_position_y: 0.0,
            camera_position_z: 0.0,
            camera_target_x: 0.0,
            camera_target_y: 0.0,
            camera_target_z: 0.0,
            camera_zoom: 1.0,
            selection_method: "rectangle".into(),
            selection_mode_default: "default".into(),
            engagement_input: String::new(),
            grid_visible: true,
            grid_snap_enabled: false,
            grid_spacing: 1.0,
            overlap_budget: 0.0,
            fill_count: 0,
            brush_candidate_index: 0,
            lod_automatic: true,
            lod_depth_variable: false,
            lod_manual: 1.0,
            proximity_radius: 0.75,
            locale: "en-US".into(),
            runtime_extras_json: "{}".into(),
            hovered_object_id: None,
            hovered_vortex_full_id: None,
            hovered_kind_id: None,
            preview_seq: 0,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: Puzzle3dSnapshot) {
        self.schema = snapshot.schema;
        self.domain = snapshot.domain;
        self.meta = snapshot.meta;
        self.objects = snapshot.objects;
        self.attractions = snapshot.attractions;
        self.target_volumes = snapshot.target_volumes;
        self.references = snapshot.references;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.puzzle.puzzle3d` — twenty handcrafted schema leaves.
pub fn puzzle3d_artifact_schema_descriptor() -> artifact_schema::ArtifactSchemaDescriptor {
    artifact_schema::ArtifactSchemaDescriptor {
        id: "s.puzzle.puzzle3d",
        artifact: artifact_schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: artifact_schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: artifact_schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: artifact_schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::puzzle3d::{Puzzle3dDiff, Puzzle3dMutation, Puzzle3dSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct Puzzle3dBuilderConstruction {
        snapshot: Puzzle3dSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for Puzzle3dBuilderConstruction {
        type Snapshot = Puzzle3dSnapshot;
        type Mutation = Puzzle3dMutation;
        type Diff = Puzzle3dDiff;
        async fn empty() -> Self {
            Self { snapshot: Puzzle3dSnapshot::default(), diagnostics: Vec::new() }
        }
        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self { snapshot: <Puzzle3dSnapshot as store::ArtifactDsl>::parse_dsl(text)?, diagnostics: Vec::new() })
        }
        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self { snapshot: <Puzzle3dSnapshot as store::ArtifactPack>::decode_pack(bytes)?, diagnostics: Vec::new() })
        }
        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        async fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <Puzzle3dDiff as protocol::MutationDiff<Puzzle3dSnapshot>>::apply(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        async fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
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
    use crate::artifacts::puzzle3d::Puzzle3dSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct Puzzle3dParts {
        pub snapshot: Option<Puzzle3dSnapshot>,
    }

    pub struct Puzzle3dAnalyzerAnalysis;

    impl ArtifactAnalysis for Puzzle3dAnalyzerAnalysis {
        type Parts = Puzzle3dParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.puzzle3d", standard: StandardId("1"), subset: SubsetId("*") };

        async fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Puzzle3dParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Puzzle3dSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Puzzle3dSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
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
    pub spec Puzzle3dBuilderFacets {
        construction: Puzzle3dBuilderConstruction,
        analysis: Puzzle3dAnalyzerAnalysis,
        composition: super::super::io::derived_composition::Puzzle3dComposerComposition,
    }
    builder: Puzzle3dBuilder,
    analyzer: Puzzle3dAnalyzer,
    composer: Puzzle3dComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️PrecomputeModel
// ⚙️➡️🧬️ Rehomed from the former `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
// the pure data shapes the interactive brush/fill precompute session (now `crate::editor::puzzle3d::precompute`)
// exchanges with its host — the kind catalogs, the host rules/weights, the `Fixture`/`SceneConfig` wire
// projection `Puzzle3dEngineCommand::SetScene` carries, and the brush/fill readouts. An artifact is a schema
// plus an io system, never an engine — the actual stateful session lives app-side; this is its data.
pub(crate) type Quat = [f64; 4];
pub(crate) type Vec3 = [f64; 3];

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct BrushHostRules {
    #[serde(default)]
    pub(crate) reject_capital_on_tambour: bool,
    #[serde(default)]
    pub(crate) reject_last_single_storey_on_mid_tambour: bool,
    #[serde(default)]
    pub(crate) door_tambour_requires_door_capsule: bool,
    #[serde(default = "default_door_capsule_min_abs_x")]
    pub(crate) door_capsule_min_abs_x: f64,
    #[serde(default = "default_door_capsule_max_abs_y")]
    pub(crate) door_capsule_max_abs_y: f64,
}

fn default_door_capsule_min_abs_x() -> f64 {
    0.9
}

fn default_door_capsule_max_abs_y() -> f64 {
    1.6
}

impl Default for BrushHostRules {
    fn default() -> Self {
        Self {
            reject_capital_on_tambour: true,
            reject_last_single_storey_on_mid_tambour: true,
            door_tambour_requires_door_capsule: true,
            door_capsule_min_abs_x: default_door_capsule_min_abs_x(),
            door_capsule_max_abs_y: default_door_capsule_max_abs_y(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct BrushKindWeights {
    #[serde(default)]
    pub(crate) object_weights: std::collections::BTreeMap<String, f64>,
    #[serde(default)]
    pub(crate) vortex_weights: std::collections::BTreeMap<String, f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct KindCompatEntry {
    pub(crate) source: String,
    pub(crate) target: String,
    #[serde(default)]
    pub(crate) bidirectional: bool,
    #[serde(default)]
    pub(crate) important: bool,
    pub(crate) specificity: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct ObjectKindVortexTemplate {
    #[serde(default)]
    pub(crate) id: String,
    #[serde(default)]
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) label: String,
    #[serde(default)]
    pub(crate) description: String,
    #[serde(default)]
    pub(crate) icon: String,
    #[serde(rename = "vortexKind", default)]
    pub(crate) vortex_kind: Option<String>,
    #[serde(default)]
    pub(crate) point: Vec3,
    #[serde(default)]
    pub(crate) direction: Option<Vec3>,
    #[serde(default)]
    pub(crate) t: Option<f64>,
    #[serde(default)]
    pub(crate) mandatory: Option<bool>,
    #[serde(default)]
    pub(crate) radius: Option<f64>,
}

impl Default for ObjectKindVortexTemplate {
    fn default() -> Self {
        Self { id: String::new(), name: String::new(), label: String::new(), description: String::new(), icon: String::new(), vortex_kind: None, point: [0.0, 0.0, 0.0], direction: None, t: None, mandatory: None, radius: None }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, dsl::DslRecord)]
pub struct ObjectKindRepresentation {
    #[serde(default)]
    pub(crate) id: String,
    #[serde(default)]
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) url: String,
    #[serde(default)]
    pub(crate) mime: String,
    #[serde(default)]
    pub(crate) tags: Vec<String>,
    #[serde(default)]
    pub(crate) lod: Option<String>,
    #[serde(default)]
    pub(crate) description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, dsl::DslRecord)]
pub struct ObjectKind {
    pub(crate) id: String,
    #[serde(default)]
    pub(crate) representations: Vec<ObjectKindRepresentation>,
    #[serde(default)]
    pub(crate) scale: Option<dsl::DslValue>,
    #[serde(default)]
    pub(crate) vortices: Vec<ObjectKindVortexTemplate>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, dsl::DslRecord)]
pub struct VortexKindCatalog {
    pub(crate) id: String,
    #[serde(default)]
    pub(crate) code: Option<String>,
    #[serde(default)]
    pub(crate) label: Option<String>,
    #[serde(default)]
    pub(crate) order: Option<i32>,
    #[serde(default, rename = "compatibleWith")]
    pub(crate) compatible_with: Vec<String>,
    #[serde(default)]
    pub(crate) description: String,
    #[serde(default)]
    pub(crate) icon: String,
    #[serde(default)]
    pub(crate) color: String,
    #[serde(rename = "defaultCableKind", default)]
    pub(crate) default_cable_kind: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, dsl::DslRecord)]
pub struct CableKindCatalog {
    pub(crate) id: String,
    #[serde(rename = "defaultAttractionKind", default)]
    pub(crate) default_attraction_kind: Option<String>,
}

/// 🗂️ The compile-time-catalog side of a scene: object/vortex/cable kind rows, reachable through
/// `apply_brush_placement_to_fixture`'s public signature.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, dsl::DslRecord)]
pub struct KindCatalogBundle {
    #[serde(default)]
    pub(crate) objects: Vec<ObjectKind>,
    #[serde(default)]
    pub(crate) vortices: Vec<VortexKindCatalog>,
    #[serde(default)]
    pub(crate) cables: Vec<CableKindCatalog>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct VortexProps {
    pub id: String,
    #[serde(rename = "vortexKind", default)]
    pub vortex_kind: Option<String>,
    pub position: Vec3,
    pub direction: Option<Vec3>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct FixtureObject {
    pub id: String,
    #[serde(rename = "objectKind", default)]
    pub object_kind: Option<String>,
    #[serde(default)]
    pub anchor: crate::artifacts::puzzle3d::Puzzle3dObjectAnchor,
    #[serde(rename = "meshUrl", default)]
    pub mesh_url: Option<String>,
    pub origin: Vec3,
    pub orientation: Option<Quat>,
    pub scale: Option<dsl::DslValue>,
    #[serde(default)]
    pub vortices: Vec<VortexProps>,
    /// 🪣️ Live-viewport-only tag (never persisted to the document): this object's 0-based position in
    /// the fill plan's sequence, so the viewport can reveal/hide planned pieces by drag position without
    /// a WASM round trip. Set only on `compose_fill_display`'s output, stripped from committed fixtures.
    #[serde(rename = "revealIndex", default, skip_serializing_if = "Option::is_none")]
    pub reveal_index: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct AttractionProps {
    #[serde(default)]
    pub id: String,
    pub attracting: String,
    pub attracted: String,
    #[serde(default)]
    pub gap: f64,
    #[serde(default)]
    pub shift: f64,
    #[serde(default)]
    pub rise: f64,
    #[serde(default)]
    pub rotation: f64,
    #[serde(default)]
    pub turn: f64,
    #[serde(default)]
    pub tilt: f64,
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct WorldVolumeProps {
    pub id: String,
    pub origin: Vec3,
    #[serde(default)]
    pub orientation: Option<Quat>,
    #[serde(default)]
    pub scale: Option<dsl::DslValue>,
}

/// 🏗️ A puzzle-3d scene's object/attraction/target-volume state, reachable through
/// `apply_brush_placement_to_fixture`'s public signature.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct Fixture {
    #[serde(default)]
    pub attractions: Vec<AttractionProps>,
    #[serde(default)]
    pub objects: Vec<FixtureObject>,
    #[serde(default, rename = "targetVolumes")]
    pub target_volumes: Vec<WorldVolumeProps>,
}

/// 📨️ The full typed payload `Puzzle3dEngineCommand::SetScene` carries — the exact same shape
/// `Puzzle3dCollision::set_scene`'s JSON payload has always deserialized into, just reused directly
/// instead of re-declared, so the command enum's field IS this type, not a mirror of it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct SceneConfig {
    pub(crate) fixture: Fixture,
    #[serde(rename = "kindCatalogs", default)]
    pub(crate) kind_catalogs: Option<KindCatalogBundle>,
    #[serde(rename = "kindCompatibility", default)]
    pub(crate) kind_compatibility: Vec<KindCompatEntry>,
    #[serde(rename = "overlapBudget", default)]
    pub(crate) overlap_budget: f64,
    #[serde(default)]
    pub(crate) seed: u32,
    #[serde(rename = "hostRules", default)]
    pub(crate) host_rules: BrushHostRules,
    #[serde(default)]
    pub(crate) weights: BrushKindWeights,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrushCompatibleCandidate {
    pub object_kind_id: String,
    pub source_vortex_index: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrushPreviewState {
    pub target_vortex_full_id: String,
    pub object_kind_id: String,
    pub source_vortex_index: usize,
    pub mesh_url: String,
    pub origin: Vec3,
    pub orientation: Quat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<dsl::DslValue>,
}

/// 🚦️ Which background precompute lane a tick should advance — fill and brush never share one FIFO queue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrecomputeLane {
    Brush = 0,
    Fill = 1,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct BrushPlacePayload {
    pub target_vortex_full_id: String,
    pub object_kind_id: String,
    pub source_vortex_index: usize,
    pub origin: Vec3,
    pub orientation: Quat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<dsl::DslValue>,
}

/// 🎯️ A suggestion-popup preview accepted as-is becomes a placement at the exact same pose — the one
/// field `BrushPreviewState` carries that `BrushPlacePayload` doesn't (`mesh_url`, resolvable again
/// from `object_kind_id` via the kind catalog) is simply dropped.
impl From<BrushPreviewState> for BrushPlacePayload {
    fn from(preview: BrushPreviewState) -> Self {
        Self { target_vortex_full_id: preview.target_vortex_full_id, object_kind_id: preview.object_kind_id, source_vortex_index: preview.source_vortex_index, origin: preview.origin, orientation: preview.orientation, scale: preview.scale }
    }
}

/// 🎯️ Public so `Puzzle3dEngineOutcome::BrushCandidates` can hand this back to callers (the app's
/// brush slot) as a typed value instead of the JSON string the old `brush_candidates` wasm-bindgen
/// method returned.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrushCollisionFreeResult {
    pub free: Vec<BrushCompatibleCandidate>,
    pub unknown_pending: bool,
    #[serde(default)]
    pub resume_candidate_index: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FillBuildPreview {
    #[serde(default)]
    pub operation: u64,
    #[serde(default)]
    pub base_revision: u64,
    pub sequence: u64,
    pub generation: u64,
    pub stage: String,
    pub target_vortex_full_id: Option<String>,
    pub candidate_object_kind_id: Option<String>,
    #[serde(default)]
    pub candidate_ghost: Option<BrushPreviewState>,
    pub broad_phase_object_ids: Vec<String>,
    pub current_pair_object_id: Option<String>,
    #[serde(default)]
    pub colliding_object_ids: Vec<String>,
    pub sample_cursor: usize,
    pub inside_both: usize,
    pub last_sample: Option<[f32; 3]>,
    #[serde(default)]
    pub collision_samples: Vec<[f32; 3]>,
    pub rejection_reason: Option<String>,
    pub target_cursor: usize,
    pub candidate_cursor: usize,
    pub accepted_count: usize,
    #[serde(default)]
    pub accepted_prefix: Vec<BrushPlacePayload>,
    #[serde(default)]
    pub search_count: u64,
    #[serde(default)]
    pub rejected_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FillBuildProgress {
    pub(crate) count: usize,
    pub(crate) applied_count: usize,
    pub(crate) max_count: usize,
    pub(crate) done: bool,
    #[serde(default)]
    pub(crate) appended_objects: Vec<FixtureObject>,
    #[serde(default)]
    pub(crate) appended_attractions: Vec<AttractionProps>,
    #[serde(default)]
    pub(crate) sequence: Vec<BrushPlacePayload>,
    #[serde(default)]
    pub(crate) preview: Option<FillBuildPreview>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FillProgressSummary {
    pub count: usize,
    pub applied_count: usize,
    pub max_count: usize,
    pub done: bool,
}

/// 🪪️ `objectId:vortexId`, unless the vortex id already carries its owner's prefix.
pub(crate) fn puzzle3d_vortex_full_id(object_id: &str, vortex_id: &str) -> String {
    if vortex_id.contains(':') {
        vortex_id.to_string()
    } else {
        format!("{object_id}:{vortex_id}")
    }
}

pub fn empty_puzzle3d_snapshot() -> Puzzle3dSnapshot {
    Puzzle3dSnapshot::default()
}
//#endregion 🔖️PrecomputeModel

//#region 🔖️PrecomputeCommand
/// 🎯️ Typed command envelope for `crate::editor::puzzle3d::precompute::Puzzle3dPrecomputeSession::dispatch`
/// — the headless replacement for the old per-action JSON-string wasm-bindgen methods. Declared here (not
/// app-side) because `#[derive(dsl::DslEnum)]`'s generated code needs `SceneConfig`/`BrushPlacePayload` in
/// scope by value, and because `🧬️mutations/💾️binary`'s `encode_engine_command`/`decode_engine_command`
/// wrap it exactly like it already does for `Puzzle3dMutation`. Field shapes mirror the exact payload each
/// old JSON-string method parsed: `SetScene` mirrors `set_scene`'s `SceneConfig` JSON body,
/// `ApplyBrushPlacement` mirrors `apply_brush_placement_json`'s `BrushPlacePayload` body,
/// `UpdateKindWeights` mirrors `update_kind_weights`'s two JSON map bodies.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
pub enum Puzzle3dEngineCommand {
    #[dsl(key = "set-scene")]
    SetScene { scene: SceneConfig },
    #[dsl(key = "apply-brush-placement")]
    ApplyBrushPlacement { payload: BrushPlacePayload },
    #[dsl(key = "apply-fill-count")]
    ApplyFillCount { count: u32 },
    #[dsl(key = "compose-fill-display")]
    ComposeFillDisplay { count: u32 },
    #[dsl(key = "update-kind-weights")]
    UpdateKindWeights { object_weights: std::collections::BTreeMap<String, f64>, vortex_weights: std::collections::BTreeMap<String, f64> },
    #[dsl(key = "brush-preview")]
    BrushPreview { vortex_full_id: String, candidate_index: u32 },
}
//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl protocol::OpText for Puzzle3dEngineCommand {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for Puzzle3dEngineCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

/// 📬️ What `dispatch` hands back — the typed counterpart of what each old JSON-string method
/// returned (a `Fixture` JSON string, a `BrushPreviewState` JSON string, or nothing). Plain Rust, no
/// DSL/wasm-bindgen requirement — this only ever crosses the artifact <-> app boundary in-process.
#[derive(Debug, Clone, PartialEq)]
pub enum Puzzle3dEngineOutcome {
    Unit,
    Fixture(Fixture),
    BrushPreview(Option<BrushPreviewState>),
}
//#endregion 🔖️PrecomputeCommand

//#region 🧪️PrecomputeTestkit
/// 🧪️ The one puzzle3d-precompute test harness — every sibling app-side precompute test file builds on
/// it instead of re-deriving a mesh-buffer/scene/fill-plan scaffold of its own. `pub(crate)` so the app's
/// own `#[cfg(test)]` modules (session/geometry/brush) can reach it across the artifact/app boundary.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;

    pub(crate) const DEFAULT_OVERLAP_BUDGET: f64 = 0.02;

    pub(crate) fn unit_cube_mesh_buffers() -> (Vec<f32>, Vec<u32>) {
        (
            vec![-1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0],
            vec![0, 1, 2, 0, 2, 3, 4, 6, 5, 4, 7, 6, 0, 4, 5, 0, 5, 1, 2, 6, 7, 2, 7, 3, 0, 3, 7, 0, 7, 4, 1, 5, 6, 1, 6, 2],
        )
    }

    /// 🧊️ Same box as `unit_cube_mesh_buffers` but with outward-facing (CCW-from-outside) winding, needed
    /// for tests that rely on `CollisionShape::contains_point` actually reporting interior points as inside.
    pub(crate) fn outward_wound_unit_cube_mesh_buffers() -> (Vec<f32>, Vec<u32>) {
        (
            vec![-1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0],
            vec![0, 2, 1, 0, 3, 2, 4, 5, 6, 4, 6, 7, 0, 5, 4, 0, 1, 5, 2, 7, 6, 2, 3, 7, 0, 7, 3, 0, 4, 7, 1, 6, 5, 1, 2, 6],
        )
    }

    /// 🏗️ One `Host` object with a single free `port-a` vortex — the smallest scene that still schedules
    /// both precompute lanes.
    pub(crate) fn single_object_scene_json() -> String {
        let scene = SceneConfig {
            fixture: Fixture {
                attractions: vec![],
                target_volumes: vec![],
                objects: vec![FixtureObject {
                    id: "host".to_string(),
                    object_kind: Some("Host".to_string()),
                    anchor: Default::default(),
                    mesh_url: Some("/test/host.glb".to_string()),
                    origin: [0.0, 0.0, 0.0],
                    orientation: Some([0.0, 0.0, 0.0, 1.0]),
                    scale: None,
                    vortices: vec![VortexProps { id: "v0".to_string(), vortex_kind: Some("port-a".to_string()), position: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]) }],
                    reveal_index: None,
                }],
            },
            kind_catalogs: Some(KindCatalogBundle {
                objects: vec![ObjectKind {
                    id: "Host".to_string(),
                    representations: vec![ObjectKindRepresentation { id: "r0".into(), name: String::new(), url: "/test/host.glb".to_string(), mime: String::new(), tags: vec![], lod: None, description: String::new() }],
                    scale: None,
                    vortices: vec![],
                }],
                vortices: vec![VortexKindCatalog { id: "port-a".to_string(), default_cable_kind: None, ..Default::default() }],
                cables: vec![],
            }),
            kind_compatibility: vec![],
            overlap_budget: DEFAULT_OVERLAP_BUDGET,
            seed: 1,
            host_rules: BrushHostRules::default(),
            weights: BrushKindWeights::default(),
        };
        serde_json::to_string(&scene).unwrap()
    }

    /// 🪣️ One synthetic already-planned fill object / attraction / placement payload, for the fill-plan
    /// prefix-stability laws in the app's own precompute session tests.
    pub(crate) fn fill_plan_object(id: &str) -> FixtureObject {
        FixtureObject {
            id: id.to_string(),
            object_kind: Some("Placed".to_string()),
            anchor: Default::default(),
            mesh_url: Some("/test/placed.glb".to_string()),
            origin: [0.0, 0.0, 0.0],
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: None,
            vortices: vec![],
            reveal_index: None,
        }
    }

    pub(crate) fn fill_plan_attraction(index: usize) -> AttractionProps {
        AttractionProps { id: format!("a{index}"), attracting: format!("p{index}:v0"), attracted: format!("p{}:v0", index + 1), gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 0.0 }
    }

    pub(crate) fn fill_plan_payload(index: usize) -> BrushPlacePayload {
        BrushPlacePayload { target_vortex_full_id: format!("p{index}:v0"), object_kind_id: "Placed".to_string(), source_vortex_index: 0, origin: [index as f64, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None }
    }
}
//#endregion 🧪️PrecomputeTestkit

//#region 🧪️PrecomputeModelTests
#[cfg(test)]
mod precompute_model_tests {
    use super::*;

    /// 🔗️ Keeps the example fixture's scene-authored kind catalog in sync with the compile-time
    /// `puzzle3d-default` manifest.
    #[test]
    fn concrete_forest_kind_catalog_matches_puzzle3d_default_manifest() {
        let fixture = crate::artifacts::puzzle3d::dsl::parse_dsl(crate::artifacts::puzzle3d::dsl::PUZZLE3D_CONCRETE_FOREST_EXAMPLE_TEXT).expect("concrete-forest example parses as dsl");
        let catalogs: KindCatalogBundle = serde_json::from_value(serde_json::to_value(&fixture.meta.kind_catalogs).unwrap()).unwrap();
        let manifest = graph::manifest::manifest_by_id("puzzle3d-default").expect("puzzle3d-default manifest must be registered");
        let wire_kind_ids: std::collections::BTreeSet<_> = manifest.wire_kinds.iter().map(|row| row.id.as_str()).collect();
        let edge_kind_ids: std::collections::BTreeSet<_> = manifest.edge_kinds.iter().map(|row| row.id.as_str()).collect();
        for vortex in &catalogs.vortices {
            if let Some(default_cable_kind) = &vortex.default_cable_kind {
                assert!(wire_kind_ids.contains(default_cable_kind.as_str()), "vortex kind {:?} references unknown wire kind {default_cable_kind:?}", vortex.id);
            }
        }
        for cable in &catalogs.cables {
            if let Some(default_attraction_kind) = &cable.default_attraction_kind {
                assert!(edge_kind_ids.contains(default_attraction_kind.as_str()), "cable kind {:?} references unknown edge kind {default_attraction_kind:?}", cable.id);
            }
        }
    }

    /// 🪪️ A vortex id that already carries its owner's prefix is passed through untouched.
    #[test]
    fn vortex_full_id_prefixes_only_bare_ids() {
        assert_eq!(puzzle3d_vortex_full_id("host", "v0"), "host:v0");
        assert_eq!(puzzle3d_vortex_full_id("host", "other:v0"), "other:v0");
    }

    #[test]
    fn brush_preview_state_converts_into_a_placement_payload() {
        let preview = BrushPreviewState {
            target_vortex_full_id: "host:v0".into(),
            object_kind_id: "Kind".into(),
            source_vortex_index: 2,
            mesh_url: "/mesh.glb".into(),
            origin: [1.0, 2.0, 3.0],
            orientation: [0.0, 0.0, 0.0, 1.0],
            scale: Some(dsl::DslValue::Number(2.0)),
        };
        let payload = BrushPlacePayload::from(preview);
        assert_eq!(payload.target_vortex_full_id, "host:v0");
        assert_eq!(payload.object_kind_id, "Kind");
        assert_eq!(payload.source_vortex_index, 2);
        assert_eq!(payload.origin, [1.0, 2.0, 3.0]);
        assert_eq!(payload.scale, Some(dsl::DslValue::Number(2.0)));
    }
}
//#endregion 🧪️PrecomputeModelTests
