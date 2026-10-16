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



//#region 🧩️ArtifactItems3
pub mod artifact_items_3 {
use crate::{Path, ProcedureMutation, ProcedureSnapshot, Value, artifact_kind};
use crate::{SemioFlowMutation, flow_content_snapshot_from_path, path_from_flow_content_snapshot, procedure_flow_child_handle, procedure_text_child_handle, seed_from_text_content_snapshot, text_content_snapshot_from_seed};
use std::collections::BTreeMap;
use semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot;
use semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Scene
/// 🌱 The program and seed as the editor reads them: composed on read from the two member stores (design §20.15).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProcedureScene {
    pub path: Path,
    pub seed: BTreeMap<String, Value>,
}

/// 🧊️ The scene is the cold boundary of its seed's neural values (a step already retires its own params on drop).
impl Drop for ProcedureScene {
    fn drop(&mut self) {
        for (_, value) in std::mem::take(&mut self.seed) {
            neural_engine::ColdRetire::retire_cold(value);
        }
    }
}

/// 🧸️ Composes the scene from the document's exact published `flow` and `text` children.
pub fn procedure_scene_from_children(snapshot: &ProcedureSnapshot, children: &semio_framework_plugin::app::ChildContentView) -> Result<ProcedureScene, semio_framework_plugin::Fault> {
    let subset = |slot: &str, child_id: &str, expected: &str| -> Result<(), semio_framework_plugin::Fault> {
        let dialect = children.dialect(slot, child_id).ok_or_else(|| semio_framework_plugin::Fault::from(format!("procedure-{slot}-child-dialect-required")))?;
        (dialect.artifact_kind == "s.stdio.semio" && dialect.standard == "v1" && dialect.subset == expected).then_some(()).ok_or_else(|| semio_framework_plugin::Fault::from(format!("procedure-{slot}-child-dialect-mismatch")))
    };
    subset("flow", &snapshot.flow.child_id, "flow")?;
    subset("text", &snapshot.text.child_id, "text")?;
    let flow = children.typed_read::<SemioFlowSnapshot>("flow", &snapshot.flow.child_id)?;
    let text = children.typed_read::<SemioTextSnapshot>("text", &snapshot.text.child_id)?;
    Ok(ProcedureScene { path: path_from_flow_content_snapshot(&flow), seed: seed_from_text_content_snapshot(&text) })
}

/// 🧸️ [`procedure_scene_from_children`] over a document view.
pub fn procedure_scene(doc: &semio_framework_plugin::ArtifactView<'_, ProcedureSnapshot>) -> Result<ProcedureScene, semio_framework_plugin::Fault> {
    procedure_scene_from_children(doc.snapshot, &doc.children)
}

/// 🌱️ The scene a document's children derive without member stores: a bundled example's or the empty program's. Readers
/// without a child view (inference, foreign serializers) read this until the framework hands them child head packs.
pub fn procedure_derivable_scene(snapshot: &ProcedureSnapshot) -> Option<ProcedureScene> {
    if (snapshot.flow.child_id.as_str(), snapshot.text.child_id.as_str()) == (crate::examples::demo::FLOW_CHILD_ID, crate::examples::demo::TEXT_CHILD_ID) {
        return Some(crate::examples::demo::scene());
    }
    let empty = ProcedureScene::default();
    (procedure_flow_child_handle(&empty.path).child_id == snapshot.flow.child_id && procedure_text_child_handle(&empty.seed).child_id == snapshot.text.child_id).then_some(empty)
}

/// 🌱️ Packs either derivable member (the react shell's `loadDocumentPair` sends `members: []`).
pub fn genesis_procedure_child_pack(snapshot: &ProcedureSnapshot, slot: &str, child_id: &str) -> Option<Vec<u8>> {
    use pack::snapshot::ArtifactPack;
    let scene = procedure_derivable_scene(snapshot)?;
    match slot {
        "flow" if child_id == snapshot.flow.child_id => Some(flow_content_snapshot_from_path(&scene.path).encode_pack()),
        "text" if child_id == snapshot.text.child_id => Some(text_content_snapshot_from_seed(&scene.seed).encode_pack()),
        _ => None,
    }
}

/// 🧮️ The flow child leaves that carry program `base` to `next` (positions are layout, never diffed): severed edges first,
/// removed steps, inserted steps, each kept step's kind/param changes, then re-pointed and inserted edges — every row
/// point-invertible, so undo and history edits replay exactly.
pub fn procedure_flow_leaves(base: &Path, next: &Path) -> Vec<SemioFlowMutation> {
    use semio_s_artifact_stdio_semio_flow::schema::mutations as flow;
    let (before, after) = (flow_content_snapshot_from_path(base), flow_content_snapshot_from_path(next));
    let mut leaves = Vec::new();
    for edge in &before.edges {
        if !after.edges.iter().any(|entry| entry.id == edge.id) {
            leaves.push(SemioFlowMutation::RemoveEdge(flow::remove_edge::RemoveEdge { id: edge.id.clone() }));
        }
    }
    for node in &before.nodes {
        if !after.nodes.iter().any(|entry| entry.id == node.id) {
            leaves.push(SemioFlowMutation::RemoveNode(flow::remove_node::RemoveNode { id: node.id.clone() }));
        }
    }
    for node in &after.nodes {
        let Some(prior) = before.nodes.iter().find(|entry| entry.id == node.id) else {
            leaves.push(SemioFlowMutation::InsertNode(flow::insert_node::InsertNode::new(node.clone())));
            continue;
        };
        if prior.kind != node.kind {
            leaves.push(SemioFlowMutation::SetNodeKind(flow::set_node_kind::SetNodeKind { id: node.id.clone(), kind: node.kind.clone() }));
        }
        for param in node.params.iter().filter(|param| !prior.params.contains(param)) {
            leaves.push(SemioFlowMutation::SetNodeParam(flow::set_node_param::SetNodeParam { id: node.id.clone(), key: param.key.clone(), value: param.value.clone(), at: None }));
        }
        for param in prior.params.iter().filter(|param| !node.params.iter().any(|entry| entry.key == param.key)) {
            leaves.push(SemioFlowMutation::RemoveNodeParam(flow::remove_node_param::RemoveNodeParam { id: node.id.clone(), key: param.key.clone() }));
        }
    }
    for edge in &after.edges {
        match before.edges.iter().find(|entry| entry.id == edge.id) {
            None => leaves.push(SemioFlowMutation::InsertEdge(flow::insert_edge::InsertEdge::new(edge.clone()))),
            Some(prior) if prior.from != edge.from || prior.to != edge.to => leaves.push(SemioFlowMutation::SetEdgeEndpoints(flow::set_edge_endpoints::SetEdgeEndpoints { id: edge.id.clone(), from: edge.from.clone(), to: edge.to.clone() })),
            Some(_) => {}
        }
    }
    leaves
}

/// 🧬️ Publishes flow child `leaves` as ONE edit of the exact composed `flow` child; no leaf is the empty emit.
pub fn procedure_child_emit<C, D>(snapshot: &ProcedureSnapshot, leaves: Vec<SemioFlowMutation>) -> semio_framework_plugin::Emit<ProcedureMutation, C, D> {
    if leaves.is_empty() {
        return semio_framework_plugin::Emit::default();
    }
    semio_framework_plugin::Emit { child_preparations: std::collections::VecDeque::from([semio_framework_plugin::app::ChildEmitPreparation::of::<SemioFlowSnapshot, _>("flow", &snapshot.flow.child_id, leaves)]), ..Default::default() }
}

/// 🧰️ Applies one program edit to the composed scene and publishes its flow leaves as ONE child edit.
pub fn procedure_edit_emit<C, D>(doc: &semio_framework_plugin::ArtifactView<'_, ProcedureSnapshot>, edit: impl FnOnce(&mut Path)) -> Result<semio_framework_plugin::Emit<ProcedureMutation, C, D>, semio_framework_plugin::Fault> {
    let base = procedure_scene(doc)?.path.clone();
    let mut next = base.clone();
    edit(&mut next);
    Ok(procedure_child_emit(doc.snapshot, procedure_flow_leaves(&base, &next)))
}
//#endregion 🔖️Scene
}
//#endregion 🧩️ArtifactItems3
