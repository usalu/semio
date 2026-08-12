//! 🧬️ Puzzle3d artifact schema — every field of the artifact with its state class.

use crate::artifacts::puzzle3d::{Puzzle3dAttraction, Puzzle3dMeta, Puzzle3dObject, Puzzle3dReference, Puzzle3dTargetVolume, Puzzle3dSnapshot, PUZZLE_3D_SCHEMA};
use artifact_schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full puzzle3d artifact state across persistent, shared-ui, local-ui and preview classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.puzzle.puzzle3d")]
pub struct Puzzle3dArtifact {
    #[state(persistent)] pub schema: String,
    #[state(persistent)] pub domain: String,
    #[state(persistent)] pub meta: Puzzle3dMeta,
    #[state(persistent)] pub objects: Vec<Puzzle3dObject>,
    #[state(persistent)] pub attractions: Vec<Puzzle3dAttraction>,
    #[state(persistent)] pub target_volumes: Vec<Puzzle3dTargetVolume>,
    #[state(persistent)] pub references: Vec<Puzzle3dReference>,
    #[state(shared_ui)] pub selected_object_ids: Vec<String>,
    #[state(shared_ui)] pub selected_vortex_ids: Vec<String>,
    #[state(shared_ui)] pub selected_attraction_ids: Vec<String>,
    #[state(shared_ui)] pub selected_target_volume_ids: Vec<String>,
    #[state(shared_ui)] pub selected_reference_ids: Vec<String>,
    #[state(shared_ui)] pub active_utility_id: String,
    #[state(local_ui)] pub camera_position_x: f64,
    #[state(local_ui)] pub camera_position_y: f64,
    #[state(local_ui)] pub camera_position_z: f64,
    #[state(local_ui)] pub camera_target_x: f64,
    #[state(local_ui)] pub camera_target_y: f64,
    #[state(local_ui)] pub camera_target_z: f64,
    #[state(local_ui)] pub camera_zoom: f64,
    #[state(local_ui)] pub selection_method: String,
    #[state(local_ui)] pub selection_mode_default: String,
    #[state(local_ui)] pub engagement_input: String,
    #[state(local_ui)] pub grid_visible: bool,
    #[state(local_ui)] pub grid_snap_enabled: bool,
    #[state(local_ui)] pub grid_spacing: f64,
    #[state(local_ui)] pub overlap_budget: f64,
    #[state(local_ui)] pub fill_count: u32,
    #[state(local_ui)] pub brush_candidate_index: u32,
    #[state(local_ui)] pub lod_automatic: bool,
    #[state(local_ui)] pub lod_depth_variable: bool,
    #[state(local_ui)] pub lod_manual: f64,
    #[state(local_ui)] pub proximity_radius: f64,
    #[state(local_ui)] pub locale: String,
    #[state(local_ui)] pub runtime_extras_json: String,
    #[state(preview)] pub hovered_object_id: Option<String>,
    #[state(preview)] pub hovered_vortex_full_id: Option<String>,
    #[state(preview)] pub hovered_kind_id: Option<String>,
    #[state(preview)] pub preview_seq: i64,
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
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::puzzle3d::{Puzzle3dDiff, Puzzle3dMutation, Puzzle3dSnapshot};

    #[derive(Clone, Debug, Default)]
    pub struct Puzzle3dBuilderConstruction {
        snapshot: Puzzle3dSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for Puzzle3dBuilderConstruction {
        type Snapshot = Puzzle3dSnapshot;
        type Mutation = Puzzle3dMutation;
        type Diff = Puzzle3dDiff;
        fn empty() -> Self { Self { snapshot: Puzzle3dSnapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<Puzzle3dSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Puzzle3dSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            crate::artifacts::puzzle3d::schema::mutations::apply_puzzle3d_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <Puzzle3dDiff as protocol::MutationDiff<Puzzle3dSnapshot>>::apply(&diff, &self.snapshot);
            self
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
    use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct Puzzle3dParts {
        pub snapshot: Option<Puzzle3dSnapshot>,
    }

    pub struct Puzzle3dAnalyzerAnalysis;

    impl ArtifactAnalysis for Puzzle3dAnalyzerAnalysis {
        type Parts = Puzzle3dParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.puzzle3d", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
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
        construction: derived_construction::Puzzle3dBuilderConstruction,
        analysis: derived_analysis::Puzzle3dAnalyzerAnalysis,
        composition: super::super::io::derived_composition::Puzzle3dComposerComposition,
    }
    builder: Puzzle3dBuilder,
    analyzer: Puzzle3dAnalyzer,
    composer: Puzzle3dComposer,
);
//#endregion 🧬️DerivedArtifactFacets
