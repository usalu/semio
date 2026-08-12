//! 🧬️ Rewrite artifact schema — every field of the artifact with its state class.

use crate::artifacts::jack::{Camera, PropertyValue};
use crate::artifacts::rewrite::LayoutPoint;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Artifact
/// 🧬️ Full rewrite artifact state across persistent, shared-ui and local-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.trinity.rewrite")]
pub struct RewriteArtifact {
    #[state(persistent)] pub before_fixture_json: String,
    #[state(persistent)] pub lhs_json: String,
    #[state(persistent)] pub rhs_json: String,
    #[state(persistent)] pub parameter_bindings: BTreeMap<String, PropertyValue>,
    #[state(persistent)] pub rule_layout: BTreeMap<String, LayoutPoint>,
    #[state(shared_ui)] pub selected_node_ids: Vec<String>,
    #[state(shared_ui)] pub active_hover_var: String,
    #[state(shared_ui)] pub active_select_var: String,
    #[state(shared_ui)] pub lod_mode_by_window: BTreeMap<String, String>,
    #[state(local_ui)] pub before_pane_camera: Camera,
    #[state(local_ui)] pub reorganize_epoch: u64,
    #[state(local_ui)] pub hover_epoch: u64,
    #[state(local_ui)] pub select_epoch: u64,
    #[state(local_ui)] pub locale: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for RewriteArtifact {
    fn default() -> Self {
        Self {
            before_fixture_json: String::new(),
            lhs_json: String::new(),
            rhs_json: String::new(),
            parameter_bindings: BTreeMap::new(),
            rule_layout: BTreeMap::new(),
            selected_node_ids: Vec::new(),
            active_hover_var: String::new(),
            active_select_var: String::new(),
            lod_mode_by_window: BTreeMap::new(),
            before_pane_camera: Camera::default(),
            reorganize_epoch: 0,
            hover_epoch: 0,
            select_epoch: 0,
            locale: "en-US".into(),
        }
    }
}

impl RewriteArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::rewrite::RewriteSnapshot {
        crate::artifacts::rewrite::RewriteSnapshot {
            before_fixture_json: self.before_fixture_json.clone(),
            lhs_json: self.lhs_json.clone(),
            rhs_json: self.rhs_json.clone(),
            parameter_bindings: self.parameter_bindings.clone(),
            rule_layout: self.rule_layout.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::rewrite::RewriteSnapshot) -> Self {
        Self {
            before_fixture_json: snapshot.before_fixture_json,
            lhs_json: snapshot.lhs_json,
            rhs_json: snapshot.rhs_json,
            parameter_bindings: snapshot.parameter_bindings,
            rule_layout: snapshot.rule_layout,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::rewrite::RewriteSnapshot) {
        self.before_fixture_json = snapshot.before_fixture_json;
        self.lhs_json = snapshot.lhs_json;
        self.rhs_json = snapshot.rhs_json;
        self.parameter_bindings = snapshot.parameter_bindings;
        self.rule_layout = snapshot.rule_layout;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.trinity.rewrite` — twenty handcrafted schema leaves.
pub fn rewrite_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.trinity.rewrite",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
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
    use crate::artifacts::rewrite::{RewriteDiff, RewriteRuleMutation, RewriteSnapshot};

    #[derive(Clone, Debug, Default)]
    pub struct RewriteBuilderConstruction {
        snapshot: RewriteSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for RewriteBuilderConstruction {
        type Snapshot = RewriteSnapshot;
        type Mutation = RewriteRuleMutation;
        type Diff = RewriteDiff;
        fn empty() -> Self { Self { snapshot: RewriteSnapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<RewriteSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<RewriteSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            crate::artifacts::rewrite::schema::mutations::apply_rewrite_rule_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <RewriteDiff as protocol::MutationDiff<RewriteSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::rewrite::RewriteSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct RewriteParts {
        pub snapshot: Option<RewriteSnapshot>,
    }

    pub struct RewriteAnalyzerAnalysis;

    impl ArtifactAnalysis for RewriteAnalyzerAnalysis {
        type Parts = RewriteParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.rewrite", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = RewriteParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <RewriteSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <RewriteSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec RewriteBuilderFacets {
        construction: derived_construction::RewriteBuilderConstruction,
        analysis: derived_analysis::RewriteAnalyzerAnalysis,
        composition: super::super::io::derived_composition::RewriteComposerComposition,
    }
    builder: RewriteBuilder,
    analyzer: RewriteAnalyzer,
    composer: RewriteComposer,
);
//#endregion 🧬️DerivedArtifactFacets
