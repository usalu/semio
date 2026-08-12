//! 🧬️ Puzzle5d artifact schema — every field of the artifact with its state class.

use crate::artifacts::puzzle5d::{Puzzle5dFastener, Puzzle5dKindCatalogs, Puzzle5dKindCompatibility, Puzzle5dMeta, Puzzle5dPart, Puzzle5dSnapshot, PUZZLE_5D_SCHEMA};
use artifact_schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

//#region 🔖️Artifact
/// 🧬️ Full puzzle5d artifact state across persistent, shared-ui, local-ui and preview classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.puzzle.puzzle5d")]
pub struct Puzzle5dArtifact {
    #[state(persistent)] pub schema: String,
    #[state(persistent)] pub domain: String,
    #[state(persistent)] pub label: Option<String>,
    #[state(persistent)] pub meta: Puzzle5dMeta,
    #[state(persistent)] pub kind_catalogs: Option<Puzzle5dKindCatalogs>,
    #[state(persistent)] pub kind_compatibility: Vec<Puzzle5dKindCompatibility>,
    #[state(persistent)] pub parts: Vec<Puzzle5dPart>,
    #[state(persistent)] pub fasteners: Vec<Puzzle5dFastener>,
    #[state(shared_ui)] pub selected_part_ids: Vec<String>,
    #[state(shared_ui)] pub selected_grip_ids: Vec<String>,
    #[state(shared_ui)] pub selected_fastener_ids: Vec<String>,
    #[state(shared_ui)] pub active_utility_id: String,
    #[state(local_ui)] pub camera2d_x: f64,
    #[state(local_ui)] pub camera2d_y: f64,
    #[state(local_ui)] pub camera2d_zoom: f64,
    #[state(local_ui)] pub camera3d_position_x: f64,
    #[state(local_ui)] pub camera3d_position_y: f64,
    #[state(local_ui)] pub camera3d_position_z: f64,
    #[state(local_ui)] pub camera3d_target_x: f64,
    #[state(local_ui)] pub camera3d_target_y: f64,
    #[state(local_ui)] pub camera3d_target_z: f64,
    #[state(local_ui)] pub camera3d_zoom: f64,
    #[state(local_ui)] pub selection_method: String,
    #[state(local_ui)] pub grid_snap_enabled: bool,
    #[state(local_ui)] pub grid_factor: f64,
    #[state(local_ui)] pub suggestion_offset: f64,
    #[state(local_ui)] pub overlap_budget: f64,
    #[state(local_ui)] pub fill_count: u32,
    #[state(local_ui)] pub brush_candidate_index: u32,
    #[state(local_ui)] pub lod_mode: String,
    #[state(local_ui)] pub locale: String,
    #[state(local_ui)] pub runtime_extras_json: String,
    #[state(preview)] pub hovered_part_id: Option<String>,
    #[state(preview)] pub preview_seq: i64,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for Puzzle5dArtifact {
    fn default() -> Self {
        Self::from_snapshot(Puzzle5dSnapshot::default())
    }
}

impl Puzzle5dArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> Puzzle5dSnapshot {
        Puzzle5dSnapshot {
            schema: self.schema.clone(),
            domain: self.domain.clone(),
            label: self.label.clone(),
            meta: self.meta.clone(),
            kind_catalogs: self.kind_catalogs.clone(),
            kind_compatibility: self.kind_compatibility.clone(),
            parts: self.parts.clone(),
            fasteners: self.fasteners.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: Puzzle5dSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            domain: snapshot.domain,
            label: snapshot.label,
            meta: snapshot.meta,
            kind_catalogs: snapshot.kind_catalogs,
            kind_compatibility: snapshot.kind_compatibility,
            parts: snapshot.parts,
            fasteners: snapshot.fasteners,
            selected_part_ids: Vec::new(),
            selected_grip_ids: Vec::new(),
            selected_fastener_ids: Vec::new(),
            active_utility_id: "select".into(),
            camera2d_x: 0.0,
            camera2d_y: 0.0,
            camera2d_zoom: 1.0,
            camera3d_position_x: 0.0,
            camera3d_position_y: 0.0,
            camera3d_position_z: 0.0,
            camera3d_target_x: 0.0,
            camera3d_target_y: 0.0,
            camera3d_target_z: 0.0,
            camera3d_zoom: 1.0,
            selection_method: "rectangle".into(),
            grid_snap_enabled: true,
            grid_factor: 1.0,
            suggestion_offset: 80.0,
            overlap_budget: 0.0,
            fill_count: 0,
            brush_candidate_index: 0,
            lod_mode: "automatic".into(),
            locale: "en-US".into(),
            runtime_extras_json: "{}".into(),
            hovered_part_id: None,
            preview_seq: 0,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: Puzzle5dSnapshot) {
        self.schema = snapshot.schema;
        self.domain = snapshot.domain;
        self.label = snapshot.label;
        self.meta = snapshot.meta;
        self.kind_catalogs = snapshot.kind_catalogs;
        self.kind_compatibility = snapshot.kind_compatibility;
        self.parts = snapshot.parts;
        self.fasteners = snapshot.fasteners;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.puzzle.puzzle5d` — twenty handcrafted schema leaves.
pub fn puzzle5d_artifact_schema_descriptor() -> artifact_schema::ArtifactSchemaDescriptor {
    artifact_schema::ArtifactSchemaDescriptor {
        id: "s.puzzle.puzzle5d",
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
    use crate::artifacts::puzzle5d::{Puzzle5dDiff, Puzzle5dMutation, Puzzle5dSnapshot};

    #[derive(Clone, Debug, Default)]
    pub struct Puzzle5dBuilderConstruction {
        snapshot: Puzzle5dSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for Puzzle5dBuilderConstruction {
        type Snapshot = Puzzle5dSnapshot;
        type Mutation = Puzzle5dMutation;
        type Diff = Puzzle5dDiff;
        fn empty() -> Self { Self { snapshot: Puzzle5dSnapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<Puzzle5dSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Puzzle5dSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            crate::artifacts::puzzle5d::schema::mutations::apply_puzzle5d_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <Puzzle5dDiff as protocol::MutationDiff<Puzzle5dSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct Puzzle5dParts {
        pub snapshot: Option<Puzzle5dSnapshot>,
    }

    pub struct Puzzle5dAnalyzerAnalysis;

    impl ArtifactAnalysis for Puzzle5dAnalyzerAnalysis {
        type Parts = Puzzle5dParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.puzzle5d", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Puzzle5dParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Puzzle5dSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Puzzle5dSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec Puzzle5dBuilderFacets {
        construction: derived_construction::Puzzle5dBuilderConstruction,
        analysis: derived_analysis::Puzzle5dAnalyzerAnalysis,
        composition: super::super::io::derived_composition::Puzzle5dComposerComposition,
    }
    builder: Puzzle5dBuilder,
    analyzer: Puzzle5dAnalyzer,
    composer: Puzzle5dComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️KindCompatibility
// 🚚️ Relocated from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
// a pure fn over document/domain data, no app/wasm dependency — a schema-side helper, not engine behaviour.
pub const PUZZLE5D_DEFAULT_MANIFEST_ID: &str = "puzzle5d-default";

/// 🧲️ Looks up whether two grip kinds are compatible per the `puzzle5d-default` manifest's
/// `kindCompatibility` rows — the single shared table both the 2D board and 3D world honor so
/// brush/fill suggestions agree across projections.
pub fn puzzle5d_grip_kinds_compatible(source_kind: &str, target_kind: &str) -> bool {
    let Some(manifest) = math::graph::manifest::manifest_by_id(PUZZLE5D_DEFAULT_MANIFEST_ID) else {
        return false;
    };
    manifest.kind_compatibility.iter().any(|row| {
        let source = row.get("source").and_then(|value| value.as_str());
        let target = row.get("target").and_then(|value| value.as_str());
        let bidirectional = row.get("bidirectional").and_then(|value| value.as_bool()).unwrap_or(false);
        (source == Some(source_kind) && target == Some(target_kind)) || (bidirectional && source == Some(target_kind) && target == Some(source_kind))
    })
}
//#endregion 🔖️KindCompatibility

//#region 🔖️DerivedDocumentHelpers
// 🚚️ Relocated from the deleted `⚙️engine` — pure document constructors/helpers with no app/wasm
// dependency, consumed by both the app's wasm bridge (`empty_puzzle5d_snapshot`) and the mutations
// binary facet, and by the transfer helpers (`next_id`).
pub fn empty_puzzle5d_snapshot() -> Puzzle5dSnapshot {
    Puzzle5dSnapshot::default()
}

/// 🪪️ Finds the smallest `"{prefix}{n}"` id not already present in `existing`.
pub fn next_id<'a>(existing: impl Iterator<Item = &'a str>, prefix: &str) -> String {
    let ids: HashSet<&str> = existing.collect();
    let mut i = ids.len();
    loop {
        let candidate = format!("{prefix}{i}");
        if !ids.iter().any(|id| *id == candidate) {
            return candidate;
        }
        i += 1;
    }
}
//#endregion 🔖️DerivedDocumentHelpers

//#region 🧪️EngineRelocationTests
#[cfg(test)]
mod engine_relocation_tests {
    use super::*;

    #[test]
    fn puzzle5d_grip_kinds_compatible_reads_manifest_rows() {
        assert!(puzzle5d_grip_kinds_compatible("port", "port"));
        assert!(puzzle5d_grip_kinds_compatible("vortex", "vortex"));
        assert!(!puzzle5d_grip_kinds_compatible("port", "vortex"));
        assert!(!puzzle5d_grip_kinds_compatible("unknown-kind", "port"));
    }
}
//#endregion 🧪️EngineRelocationTests
