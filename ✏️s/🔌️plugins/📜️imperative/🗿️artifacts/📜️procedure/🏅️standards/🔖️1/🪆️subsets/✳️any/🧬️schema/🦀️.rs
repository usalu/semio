//! 🧬️ Imperative artifact schema — every field with its state class.

use crate::artifacts::procedure::{ProcedureFlowChild, ProcedureTextChild};
use schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Full imperative artifact state across the artifact, presence, config and transient lanes.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[artifact_schema(id = "s.imperative.procedure")]
pub struct ProcedureArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.flow")]
    pub flow: ProcedureFlowChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.text")]
    pub text: ProcedureTextChild,
    #[state(presence)]
    #[value(default)]
    pub selected_step_ids: Vec<String>,
    #[state(config)]
    pub locale: String,
    #[state(config)]
    #[value(default = "default_contributions_json")]
    pub contributions_json: String,
    #[state(transient)]
    #[value(default)]
    pub run_output_json: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
fn default_contributions_json() -> String {
    "[]".into()
}

impl Default for ProcedureArtifact {
    fn default() -> Self {
        let empty = crate::artifacts::procedure::schema::snapshot::ProcedureSnapshot::default();
        Self { schema: empty.schema, flow: empty.flow, text: empty.text, selected_step_ids: Vec::new(), locale: "en-US".into(), contributions_json: default_contributions_json(), run_output_json: String::new() }
    }
}

impl ProcedureArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::procedure::ProcedureSnapshot {
        crate::artifacts::procedure::ProcedureSnapshot { schema: self.schema.clone(), flow: self.flow.clone(), text: self.text.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::procedure::ProcedureSnapshot) -> Self {
        Self { schema: snapshot.schema, flow: snapshot.flow, text: snapshot.text, ..Self::default() }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::procedure::ProcedureSnapshot) {
        self.schema = snapshot.schema;
        self.flow = snapshot.flow;
        self.text = snapshot.text;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.imperative.procedure` — twenty handcrafted schema leaves.
pub fn procedure_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.imperative.procedure",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: schema::FacetLeaves {
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
    use crate::artifacts::procedure::schema::diff::ProcedureDiff;
    use crate::artifacts::procedure::schema::mutations::ProcedureMutation;
    use crate::artifacts::procedure::schema::snapshot::ProcedureSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct ProcedureBuilderConstruction {
        snapshot: ProcedureSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for ProcedureBuilderConstruction {
        type Snapshot = ProcedureSnapshot;
        type Mutation = ProcedureMutation;
        type Diff = ProcedureDiff;
        async fn empty() -> Self {
            Self { snapshot: ProcedureSnapshot::default(), diagnostics: Vec::new() }
        }
        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self { snapshot: <ProcedureSnapshot as store::ArtifactDsl>::parse_dsl(text)?, diagnostics: Vec::new() })
        }
        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self { snapshot: <ProcedureSnapshot as store::ArtifactPack>::decode_pack(bytes)?, diagnostics: Vec::new() })
        }
        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <ProcedureMutation as protocol::Mutation<ProcedureSnapshot>>::diff(&mutation, &self.snapshot);
            match protocol::MutationDiff::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        async fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <ProcedureDiff as protocol::MutationDiff<ProcedureSnapshot>>::apply(&diff, &self.snapshot)?;
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
    use crate::artifacts::procedure::ProcedureSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct ProcedureParts {
        pub snapshot: Option<ProcedureSnapshot>,
    }

    pub struct ProcedureAnalyzerAnalysis;

    impl ArtifactAnalysis for ProcedureAnalyzerAnalysis {
        type Parts = ProcedureParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.procedure", standard: StandardId("1"), subset: SubsetId("*") };

        async fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = ProcedureParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <ProcedureSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <ProcedureSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec ProcedureBuilderFacets {
        construction: ProcedureBuilderConstruction,
        analysis: ProcedureAnalyzerAnalysis,
        composition: super::super::io::derived_composition::ProcedureComposerComposition,
    }
    builder: ProcedureBuilder,
    analyzer: ProcedureAnalyzer,
    composer: ProcedureComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
/// 📄️ The default `imperative` document's live `Path` — two steps (`state.set counter=1`,
/// `log.print message="hello"`), the same content the pre-migration `.imperative`-DSL-authored
/// fixture carried. Built directly in Rust rather than recovered by parsing `PROCEDURE_EXAMPLE_TEXT`:
/// since `flow`/`text` are now opaque content-addressed handles (ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`), a bare `parse_dsl` of persisted/fixture text
/// recovers only the handles, never the content (no `LinkResolver` exists yet — see
/// `ProcedureWorkingScene`'s doc comment) — building the canonical default directly here, then
/// printing it to regenerate the fixture text (see `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`),
/// is the honest source of truth, matching `writer`'s/`flow`'s own fixture-builder precedent.
fn default_path() -> crate::artifacts::procedure::Path {
    use crate::artifacts::procedure::{Dictionary, Path, Step};
    use neural_engine::{Atom, Value};
    Path {
        steps: vec![
            Step { id: "step-1".into(), kind: "state.set".into(), params: Dictionary::new().insert("key", Value::Atom(Atom::String("counter".into()))).insert("value", Value::Atom(Atom::Integer(1))), bodies: Default::default() },
            Step { id: "step-2".into(), kind: "log.print".into(), params: Dictionary::new().insert("message", Value::Atom(Atom::String("hello".into()))), bodies: Default::default() },
        ],
    }
}

/// 📄️ The default `imperative` document — {@link default_path}'s two steps, empty seed. Relocated
/// from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — pure
/// over document types, no app-runtime parameter, so it belongs beside the schema it builds.
pub fn default_snapshot() -> crate::artifacts::procedure::ProcedureSnapshot {
    crate::artifacts::procedure::procedure_snapshot_with_content("procedure.document", &default_path(), &std::collections::BTreeMap::new())
}
//#endregion 🔖️DocumentHelpers
