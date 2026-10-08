//! 🧬️ Imperative artifact schema — every field with its state class.

use crate::{ProcedureFlowChild, ProcedureTextChild};
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Full imperative artifact state across the artifact, presence, config and transient lanes.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact_schema(id = "s.imperative.procedure")]
pub struct ProcedureArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub flow: ProcedureFlowChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub text: ProcedureTextChild,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for ProcedureArtifact {
    fn default() -> Self {
        let empty = crate::schema::snapshot::ProcedureSnapshot::default();
        Self { schema: empty.schema, flow: empty.flow, text: empty.text }
    }
}

impl ProcedureArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::ProcedureSnapshot {
        crate::ProcedureSnapshot { schema: self.schema.clone(), flow: self.flow.clone(), text: self.text.clone() }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: crate::ProcedureSnapshot) -> Self {
        Self { schema: snapshot.schema, flow: snapshot.flow, text: snapshot.text }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::ProcedureSnapshot) {
        self.schema = snapshot.schema;
        self.flow = snapshot.flow;
        self.text = snapshot.text;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.imperative.procedure` — twenty handcrafted schema leaves.
pub fn procedure_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.imperative.procedure",
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

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

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
pub fn default_path() -> crate::Path {
    use crate::{Dictionary, Path, Step};
    use neural_engine::{Atom, Value};
    Path {
        steps: vec![
            Step { id: "step-1".into(), kind: "state.set".into(), params: Dictionary::new().insert("key", Value::Atom(Atom::String("counter".into()))).insert("value", Value::Atom(Atom::Integer(1))), bodies: Default::default() },
            Step { id: "step-2".into(), kind: "log.print".into(), params: Dictionary::new().insert("message", Value::Atom(Atom::String("hello".into()))), bodies: Default::default() },
        ],
    }
}

/// 📄️ The default `imperative` document — the bundled demo, whose children derive {@link default_path}'s two steps and an
/// empty seed. Relocated
/// from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — pure
/// over document types, no app-runtime parameter, so it belongs beside the schema it builds.
pub fn default_snapshot() -> crate::ProcedureSnapshot {
    crate::examples::demo::snapshot()
}
//#endregion 🔖️DocumentHelpers

#[cfg(test)]
#[path = "🧪️tests/🪪️document-contract/🦀️.rs"]
mod document_contract_tests;
