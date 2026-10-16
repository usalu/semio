//! 🚪️ Artifact binary representations.


//#region 🧩️ArtifactItems
pub mod artifact_items {
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use std::collections::BTreeMap;

//#region 🔖️Types
pub use imperative_engine::{Path, Step};
pub use neural_engine::{Dictionary, Registry, Value};

/// 🎯️ This artifact's `✏️editor`/`👁️viewer` surface coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1) — lives at the ARTIFACT level
/// (not under `editor`/`viewer`) specifically so a viewer file can read it without ever importing
/// through the sibling editor module. `artifact_kind` matches the `#[artifact_schema(id = ..)]`
/// this artifact's own schema declares (`🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`);
/// `standard`/`subset` match this file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location — i.e. the
/// canonical surface id is `s.imperative.procedure@1/*#editor` / `s.imperative.procedure@1/*#viewer`.
pub const PROCEDURE_DIALECT: semio_framework_plugin::app::Dialect =
    semio_framework_plugin::app::Dialect { artifact_kind: "s.imperative.procedure", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };

/// 🌱️ View of a snapshot seed map as a neural [`Dictionary`] for execution — folds entries straight
/// into the dictionary's own `insert` cursor, no JSON round trip needed.
pub fn seed_dictionary(seed: &BTreeMap<String, Value>) -> Dictionary {
    seed.iter().fold(Dictionary::new(), |dictionary, (key, value)| dictionary.insert(key.clone(), value.clone()))
}

/// 🗂️ The `store::ArtifactStore` schema key — deliberately distinct from the snapshot's `schema`
/// field (`"procedure.document"`, the field inside the document itself): this one keys the store envelope.
pub use crate::schema::mutations::ProcedureMutation;

pub use crate::schema::diff::ProcedureDiff;

pub const PROCEDURE_DOCUMENT_SCHEMA: &str = "procedure.document/v1";

pub use crate::schema::snapshot::ProcedureSnapshot;

/// 📍️ Address of a nested step list inside a control step body.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct PathRef {
    #[value(default)]
    pub owner: Option<String>,
    #[value(default)]
    pub slot: Option<String>,
}
//#endregion 🔖️Types

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::editor::procedure::create_imperative_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "computation.procedure".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Procedure", "Prozedur"),
        source_format: "procedure.document".into(),
        component_kind: "procedure".into(),
        dimension: "graph".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Procedure },
        schema: PROCEDURE_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.json".into()],
        import_stdio_kinds: vec!["stdio.json".into()],
    }
}
//#endregion 🔖️ArtifactKind
}
//#endregion 🧩️ArtifactItems

//#region 🧩️ArtifactItems2
pub mod artifact_items_2 {
use crate::{Dictionary, Path, ProcedureSnapshot, Step, Value, artifact_kind};
use std::collections::BTreeMap;

//#region 🔖️ContentBridge
/// 🕸️ Owned CHILD handles of the two composed stdio subsets (design §20.15): the program lives in the `flow` member store
/// (`s.stdio.semio@v1/flow`), the initial variable dictionary in the `text` member store; the parent owns no content.
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
pub use semio_s_artifact_stdio_semio_flow::schema::mutations::SemioFlowMutation;
use semio_s_artifact_stdio_semio_flow::schema::snapshot::{FlowEdge, FlowNode, FlowParam, PortRef, SemioFlowSnapshot, STDIO_SEMIOFLOW_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextRun;
use semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextSnapshot;
use semio_s_artifact_stdio_semio_text_model::schema::snapshot::STDIO_SEMIOTEXT_DOCUMENT_SCHEMA;

pub type ProcedureFlowChild = store::os_store::artifact_child::ArtifactChild<SemioFlowSnapshot>;
pub type ProcedureTextChild = store::os_store::artifact_child::ArtifactChild<SemioTextSnapshot>;

/// 🔗️ The flow edge kind that chains consecutive steps of one scope.
pub const PROCEDURE_SEQUENCE_EDGE: &str = "sequence";
/// 🪆️ The flow edge kind from a control step (port = body slot) to the first step of that body.
pub const PROCEDURE_BODY_EDGE: &str = "body";

/// 🔗️ The id of the sequence edge between two consecutive steps.
pub fn procedure_sequence_edge_id(from: &str, to: &str) -> String {
    format!("s-{from}-{to}")
}

/// 🪆️ The id of the body edge of one control step's slot (stable while the slot exists).
pub fn procedure_body_edge_id(owner: &str, slot: &str) -> String {
    format!("b-{owner}-{slot}")
}

/// 🌉 The program as a flow graph (design §20.15): EVERY step — nested ones included — is one node (`id`/`kind`, params one
/// JSON-encoded value each); a scope's order is its chain of `sequence` edges, a control step's body slot is one `body`
/// edge (port = slot) to the slot's first step. Nesting, order and params are therefore each edited by one id-keyed flow
/// leaf; positions only lay the nodes out for generic flow tooling (depth-first column, nesting row).
pub fn flow_content_snapshot_from_path(path: &Path) -> SemioFlowSnapshot {
    fn walk(steps: &[Step], depth: usize, owner: Option<(&str, &str)>, nodes: &mut Vec<FlowNode>, edges: &mut Vec<FlowEdge>) {
        let edge = |id: String, from: &str, port: &str, to: &str, kind: &str| FlowEdge { id, from: PortRef { node: from.into(), port: port.into() }, to: PortRef { node: to.into(), port: "in".into() }, kind: kind.into() };
        for (index, step) in steps.iter().enumerate() {
            let params = step.params.keys().map(|key| FlowParam { key: key.clone(), value: semio_framework_pack_json::to_json_string(step.params.get(key).expect("key came from Dictionary::keys()")) }).collect();
            nodes.push(FlowNode { id: step.id.clone(), kind: step.kind.clone(), label: step.id.clone(), params, position: SemioPoint2 { x: nodes.len() as f64 * 160.0, y: depth as f64 * 120.0 } });
            match (index, owner) {
                (0, Some((owner, slot))) => edges.push(edge(procedure_body_edge_id(owner, slot), owner, slot, &step.id, PROCEDURE_BODY_EDGE)),
                (0, None) => {}
                _ => edges.push(edge(procedure_sequence_edge_id(&steps[index - 1].id, &step.id), &steps[index - 1].id, "out", &step.id, PROCEDURE_SEQUENCE_EDGE)),
            }
            for (slot, body) in &step.bodies {
                walk(&body.steps, depth + 1, Some((&step.id, slot)), nodes, edges);
            }
        }
    }
    let (mut nodes, mut edges) = (Vec::new(), Vec::new());
    walk(&path.steps, 0, None, &mut nodes, &mut edges);
    SemioFlowSnapshot { schema: STDIO_SEMIOFLOW_DOCUMENT_SCHEMA.into(), nodes, edges }
}

/// 🌉 Inverse of [`flow_content_snapshot_from_path`]: the top scope starts at the first node no edge enters; every scope
/// follows its `sequence` chain; every `body` edge opens its slot. A node no scope reaches is not part of the program
/// (a foreign author's stray node), and a cycle stops at the first revisit.
pub fn path_from_flow_content_snapshot(snapshot: &SemioFlowSnapshot) -> Path {
    let entered: std::collections::BTreeSet<&str> = snapshot.edges.iter().map(|edge| edge.to.node.as_str()).collect();
    let next = |id: &str| snapshot.edges.iter().find(|edge| edge.kind == PROCEDURE_SEQUENCE_EDGE && edge.from.node == id).map(|edge| edge.to.node.as_str());
    let mut visited = std::collections::BTreeSet::new();
    fn scope<'a>(first: Option<&'a str>, snapshot: &'a SemioFlowSnapshot, next: &dyn Fn(&str) -> Option<&'a str>, visited: &mut std::collections::BTreeSet<&'a str>) -> Path {
        let mut steps = Vec::new();
        let mut cursor = first;
        while let Some(id) = cursor.filter(|id| visited.insert(*id)) {
            let Some(node) = snapshot.nodes.iter().find(|node| node.id == id) else { break };
            let params = node.params.iter().fold(Dictionary::new(), |params, param| {
                let value: Value = semio_framework_pack_json::from_json_str(&param.value, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or(Value::Atom(neural_engine::Atom::Null));
                params.insert(param.key.clone(), value)
            });
            let bodies = snapshot.edges.iter().filter(|edge| edge.kind == PROCEDURE_BODY_EDGE && edge.from.node == id).map(|edge| (edge.from.port.clone(), scope(Some(edge.to.node.as_str()), snapshot, next, visited))).collect();
            steps.push(Step { id: node.id.clone(), kind: node.kind.clone(), params, bodies });
            cursor = next(id);
        }
        Path { steps }
    }
    let first = snapshot.nodes.iter().map(|node| node.id.as_str()).find(|id| !entered.contains(id));
    scope(first, snapshot, &next, &mut visited)
}

/// 🌉 `seed` → the text child: the WHOLE seed map JSON-encoded into ONE run (an empty seed is zero runs) — the honest,
/// lossless boundary for a dictionary with no prose (matching writer's raw text in one code block).
pub fn text_content_snapshot_from_seed(seed: &BTreeMap<String, Value>) -> SemioTextSnapshot {
    let runs = if seed.is_empty() { Vec::new() } else { vec![SemioTextRun { language: String::new(), content: semio_framework_pack_json::to_json_string(seed), marks: Vec::new() }] };
    SemioTextSnapshot { schema: STDIO_SEMIOTEXT_DOCUMENT_SCHEMA.into(), runs }
}

/// 🌉 Inverse of [`text_content_snapshot_from_seed`]; an empty or unparseable join reads as an empty seed.
pub fn seed_from_text_content_snapshot(snapshot: &SemioTextSnapshot) -> BTreeMap<String, Value> {
    let joined: String = snapshot.runs.iter().map(|run| run.content.as_str()).collect();
    if joined.is_empty() {
        return BTreeMap::new();
    }
    semio_framework_pack_json::from_json_str(&joined, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_default()
}

/// 🕸️ The content-addressed `flow` CHILD handle of a program.
pub fn procedure_flow_child_handle(path: &Path) -> ProcedureFlowChild {
    use pack::snapshot::ArtifactPack;
    let child_id = store::content_id("imperative-flow", &flow_content_snapshot_from_path(path).encode_pack());
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "flow".into() };
    store::os_store::artifact_child::ArtifactChild::new(child_id.clone(), semio_framework_artifact_reference::ArtifactRef { artifact_id: child_id, dialect })
}

/// 🕸️ The content-addressed `text` CHILD handle of a seed.
pub fn procedure_text_child_handle(seed: &BTreeMap<String, Value>) -> ProcedureTextChild {
    use pack::snapshot::ArtifactPack;
    let child_id = store::content_id("imperative-text", &text_content_snapshot_from_seed(seed).encode_pack());
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "text".into() };
    store::os_store::artifact_child::ArtifactChild::new(child_id.clone(), semio_framework_artifact_reference::ArtifactRef { artifact_id: child_id, dialect })
}

/// 🏗️ The parent document naming the content-addressed children of a program and seed (their content lives in the member
/// stores, derived at genesis for the bundled examples and the empty program).
pub fn procedure_snapshot_naming(path: &Path, seed: &BTreeMap<String, Value>) -> ProcedureSnapshot {
    ProcedureSnapshot { schema: "procedure.document".into(), flow: procedure_flow_child_handle(path), text: procedure_text_child_handle(seed) }
}
//#endregion 🔖️ContentBridge
}
//#endregion 🧩️ArtifactItems2
