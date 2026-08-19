//! 🧬️ Imperative artifact schema — every field with its state class.

use crate::artifacts::imperative::{ImperativeFlowChild, ImperativeTextChild};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full imperative artifact state across the artifact, presence, config and transient lanes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.imperative.imperative")]
pub struct ImperativeArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.flow")]
    pub flow: ImperativeFlowChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.text")]
    pub text: ImperativeTextChild,
    #[state(presence)]
    #[serde(default)]
    pub selected_step_ids: Vec<String>,
    #[state(config)]
    pub locale: String,
    #[state(config)]
    #[serde(default = "default_contributions_json")]
    pub contributions_json: String,
    #[state(transient)]
    #[serde(default)]
    pub run_output_json: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
async fn default_contributions_json() -> String {
    "[]".into()
}

impl Default for ImperativeArtifact {
    async fn default() -> Self {
        let empty = crate::artifacts::imperative::schema::snapshot::ImperativeSnapshot::default();
        Self {
            schema: empty.schema,
            flow: empty.flow,
            text: empty.text,
            selected_step_ids: Vec::new(),
            locale: "en-US".into(),
            contributions_json: default_contributions_json(),
            run_output_json: String::new(),
        }
    }
}

impl ImperativeArtifact {
    /// 📸️ Persisted subset.
    pub async fn to_snapshot(&self) -> crate::artifacts::imperative::ImperativeSnapshot {
        crate::artifacts::imperative::ImperativeSnapshot {
            schema: self.schema.clone(),
            flow: self.flow.clone(),
            text: self.text.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub async fn from_snapshot(snapshot: crate::artifacts::imperative::ImperativeSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            flow: snapshot.flow,
            text: snapshot.text,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub async fn set_snapshot(&mut self, snapshot: crate::artifacts::imperative::ImperativeSnapshot) {
        self.schema = snapshot.schema;
        self.flow = snapshot.flow;
        self.text = snapshot.text;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.imperative.imperative` — twenty handcrafted schema leaves.
pub async fn imperative_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.imperative.imperative",
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
    use crate::artifacts::imperative::schema::diff::ImperativeDiff;
    use crate::artifacts::imperative::schema::mutations::ImperativeMutation;
    use crate::artifacts::imperative::schema::snapshot::ImperativeSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct ImperativeBuilderConstruction {
        snapshot: ImperativeSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for ImperativeBuilderConstruction {
        type Snapshot = ImperativeSnapshot;
        type Mutation = ImperativeMutation;
        type Diff = ImperativeDiff;
        async fn empty() -> Self { Self { snapshot: ImperativeSnapshot::default(), diagnostics: Vec::new() } }
        async fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<ImperativeSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<ImperativeSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <ImperativeMutation as protocol::Mutation<ImperativeSnapshot>>::diff(&mutation, &self.snapshot);
            match protocol::MutationDiff::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error(
                    "mutation.apply",
                    dsl::TextSpan::at(1, 1),
                    error.to_string(),
                )),
            }
            (self, outcome)
        }
        async fn absorb(
            mut self,
            diff: Self::Diff,
        ) -> protocol::MutationApplyResult<Self> {
            let snapshot = <ImperativeDiff as protocol::MutationDiff<ImperativeSnapshot>>::apply(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        async fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
    use crate::artifacts::imperative::ImperativeSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct ImperativeParts {
        pub snapshot: Option<ImperativeSnapshot>,
    }

    pub struct ImperativeAnalyzerAnalysis;

    impl ArtifactAnalysis for ImperativeAnalyzerAnalysis {
        type Parts = ImperativeParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.imperative", standard: StandardId("1"), subset: SubsetId("*") };

        async fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = ImperativeParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <ImperativeSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <ImperativeSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec ImperativeBuilderFacets {
        construction: ImperativeBuilderConstruction,
        analysis: ImperativeAnalyzerAnalysis,
        composition: super::super::io::derived_composition::ImperativeComposerComposition,
    }
    builder: ImperativeBuilder,
    analyzer: ImperativeAnalyzer,
    composer: ImperativeComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
/// 📄️ The default `imperative` document's live `Path` — two steps (`state.set counter=1`,
/// `log.print message="hello"`), the same content the pre-migration `.imperative`-DSL-authored
/// fixture carried. Built directly in Rust rather than recovered by parsing `IMPERATIVE_EXAMPLE_TEXT`:
/// since `flow`/`text` are now opaque content-addressed handles (ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`), a bare `parse_dsl` of persisted/fixture text
/// recovers only the handles, never the content (no `LinkResolver` exists yet — see
/// `ImperativeWorkingScene`'s doc comment) — building the canonical default directly here, then
/// printing it to regenerate the fixture text (see `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`),
/// is the honest source of truth, matching `writer`'s/`flow`'s own fixture-builder precedent.
async fn default_path() -> crate::artifacts::imperative::Path {
    use crate::artifacts::imperative::{Dictionary, Path, Step};
    use neural_engine::{Atom, Value};
    Path {
        steps: vec![
            Step {
                id: "step-1".into(),
                kind: "state.set".into(),
                params: Dictionary::new().insert("key", Value::Atom(Atom::String("counter".into()))).insert("value", Value::Atom(Atom::Integer(1))),
                bodies: Default::default(),
            },
            Step {
                id: "step-2".into(),
                kind: "log.print".into(),
                params: Dictionary::new().insert("message", Value::Atom(Atom::String("hello".into()))),
                bodies: Default::default(),
            },
        ],
    }
}

/// 📄️ The default `imperative` document — {@link default_path}'s two steps, empty seed. Relocated
/// from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — pure
/// over document types, no app-runtime parameter, so it belongs beside the schema it builds.
pub async fn default_snapshot() -> crate::artifacts::imperative::ImperativeSnapshot {
    crate::artifacts::imperative::imperative_snapshot_with_content("imperative.document", &default_path(), &std::collections::BTreeMap::new())
}
//#endregion 🔖️DocumentHelpers
