//! 📜️ Imperative artifact — the document entity this plugin's app edits: a `Path` of control-flow
//! `Step`s (`state.set`/`log.print`/`control.if`/`control.while`/`math.add`/…), each addressable by a
//! [`PathRef`] for nested `control.*` bodies (drag-and-drop into blocks).
//!
//! `Path`/`Step` are NOT owned here — they live in the shared kernel crate `imperative_engine`
//! (`✏️s/🔨️modules/📜️imperative`, package `semio-s-kernel-imperative`; **do not confuse this kernel crate
//! with this plugin** — same "imperative" name, different crate, different location, a legitimate
//! dependency this plugin has always had). `Dictionary`/`Registry` come from the framework's
//! `neural_engine` kernel. This component re-exports the app-facing surface so every sibling taxonomy
//! node (`🔺️diff`, `🔧️op`, `🗣️dsl`, `📸️snapshot`, `📡️spr`, `⚙️engine`) names one artifact-owned symbol
//! instead of reaching into either kernel path directly.

// 🧩️ 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME D3 — the five `🧩️extensions/*/🦀️.rs`
// files mounted below each carry a `#[cfg(feature = "extension-entry")]` guard on their own
// `extension_exports!` call (this crate's `Cargo.toml` declares no such feature, so it always
// evaluates false here — see any extension's own `Cargo.toml` for the full rationale).
#![allow(unexpected_cfgs)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
mod art_procedure_demo_tests;
extern crate semio_framework_schema as framework_schema;

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use std::collections::BTreeMap;

/// 🧩️ Built-in operators mounted by the procedure runtime.
#[path = "."]
pub mod extensions {
    #[path = "../../🧩️extensions/🎮️control/🦀️.rs"]
    pub mod control;
    #[path = "../../🧩️extensions/📣️effect/🦀️.rs"]
    pub mod effect;
    #[path = "../../🧩️extensions/🧠️logic/🦀️.rs"]
    pub mod logic;
    #[path = "../../🧩️extensions/🧮️math/🦀️.rs"]
    pub mod math;
    #[path = "../../🧩️extensions/📝️text/🦀️.rs"]
    pub mod text;
}

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

//#region 🔖️ContentBridge
/// 🕸️ Owned CHILD handles of the two composed stdio subsets (design §20.15): the program lives in the `flow` member store
/// (`s.stdio.semio@v1/flow`), the initial variable dictionary in the `text` member store; the parent owns no content.
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::SemioPoint2;
pub use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::mutations::SemioFlowMutation;
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::{FlowEdge, FlowNode, FlowParam, PortRef, SemioFlowSnapshot, STDIO_SEMIOFLOW_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_semio::standards::v1::subsets::text::schema::snapshot::{SemioTextRun, SemioTextSnapshot, STDIO_SEMIOTEXT_DOCUMENT_SCHEMA};

pub type ProcedureFlowChild = store::ArtifactChild<SemioFlowSnapshot>;
pub type ProcedureTextChild = store::ArtifactChild<SemioTextSnapshot>;

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
    use store::ArtifactPack;
    let child_id = store::content_id("imperative-flow", &flow_content_snapshot_from_path(path).encode_pack());
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "flow".into() };
    store::ArtifactChild::new(child_id.clone(), store::os_io::ArtifactRef { artifact_id: child_id, dialect })
}

/// 🕸️ The content-addressed `text` CHILD handle of a seed.
pub fn procedure_text_child_handle(seed: &BTreeMap<String, Value>) -> ProcedureTextChild {
    use store::ArtifactPack;
    let child_id = store::content_id("imperative-text", &text_content_snapshot_from_seed(seed).encode_pack());
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "text".into() };
    store::ArtifactChild::new(child_id.clone(), store::os_io::ArtifactRef { artifact_id: child_id, dialect })
}

/// 🏗️ The parent document naming the content-addressed children of a program and seed (their content lives in the member
/// stores, derived at genesis for the bundled examples and the empty program).
pub fn procedure_snapshot_naming(path: &Path, seed: &BTreeMap<String, Value>) -> ProcedureSnapshot {
    ProcedureSnapshot { schema: "procedure.document".into(), flow: procedure_flow_child_handle(path), text: procedure_text_child_handle(seed) }
}
//#endregion 🔖️ContentBridge

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
        neural_engine::ColdRetire::retire_cold(std::mem::take(&mut self.seed));
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
    use store::ArtifactPack;
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
    use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::mutations as flow;
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
            leaves.push(SemioFlowMutation::SetNodeParam(flow::set_node_param::SetNodeParam { id: node.id.clone(), key: param.key.clone(), value: param.value.clone() }));
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
pub fn procedure_child_emit<C, D>(snapshot: &ProcedureSnapshot, leaves: &[SemioFlowMutation]) -> semio_framework_plugin::Emit<ProcedureMutation, C, D> {
    if leaves.is_empty() {
        return semio_framework_plugin::Emit::default();
    }
    semio_framework_plugin::Emit { child_emits: vec![semio_framework_plugin::app::ChildEmit::of::<SemioFlowSnapshot, _>("flow", &snapshot.flow.child_id, leaves)], ..Default::default() }
}

/// 🧰️ Applies one program edit to the composed scene and publishes its flow leaves as ONE child edit.
pub fn procedure_edit_emit<C, D>(doc: &semio_framework_plugin::ArtifactView<'_, ProcedureSnapshot>, edit: impl FnOnce(&mut Path)) -> Result<semio_framework_plugin::Emit<ProcedureMutation, C, D>, semio_framework_plugin::Fault> {
    let base = procedure_scene(doc)?.path.clone();
    let mut next = base.clone();
    edit(&mut next);
    Ok(procedure_child_emit(doc.snapshot, &procedure_flow_leaves(&base, &next)))
}
//#endregion 🔖️Scene

//#region 🔖️Register
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called four different global registries directly from a
/// plugin `.setup()` callback. `bootstrap_imperative_runtime()` runs here too, NOT as a §6 registrar
/// (`register_language`/`register_artifact_schema_descriptor`/… all ARE §6 and now live in the builder
/// chain below) but as this artifact's OWN native-module bootstrap
/// (`register_native_imperative_module` × 4 + `register_default_imperative_contributions`) — it has no
/// `ArtifactDeclaration` field because it isn't one of the census's global SDK registrars, it is
/// imperative's private compute-runtime setup. `Once`-guarded, so calling it eagerly here reproduces
/// the old `register()`'s timing exactly (native modules populated before any `ImperativeHost`/
/// `render()` call can observe an empty registry) without adding a second purpose to `.setup()` — see
/// the plugin root's own doc for why `.setup()` stays narrowed to `register_app_schema` alone. Lives at
/// the artifact root, not `⚙️engine` (reloc-g7 revision of that same ticket) — `declaration()` describes
/// the artifact (kind/schema/io/ownership), it is not engine behaviour.
///
/// 🔄️ UPDATE (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): `⚙️engine` is deleted.
/// `bootstrap_imperative_runtime()` and `io_registry` now live in `🚪️io` (multi-caller: this
/// `declaration()`, the app's `🎚️config`, and the app engine's `ImperativeHost::from_scene` — an
/// artifact must not depend on its app, so both stayed artifact-side rather than moving to the app),
/// reached below by their full qualified path. `bootstrap_imperative_runtime` stays `pub` (widened from
/// its former `pub(crate)`) since the app engine module now reaches it by the same long qualified path.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.imperative.procedure.standard.v1", "standard", "1", &[], None),
        ("s.imperative.procedure.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.imperative.procedure.schema.artifact", "schema", "s.imperative.procedure", &[("schema", "s.imperative.procedure")], None),
        ("s.imperative.procedure.inference.artifact", "inference", "s.imperative.procedure.inference", &[("schema", "s.imperative.procedure.inference")], None),
        ("s.imperative.procedure.composer.native", "composer", "s.imperative.procedure@1/*", &[("dialect", "s.imperative.procedure@1/*")], None),
        ("s.imperative.procedure.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.imperative.procedure.grammar.document", "grammar", "procedure.document", &[("grammar", "procedure.document")], None),
        ("s.imperative.procedure.grammar.diff", "grammar", "imperative.procedure.diff", &[("grammar", "imperative.procedure.diff")], None),
        ("s.imperative.procedure.grammar.pack", "grammar", "procedure.pack", &[("grammar", "procedure.pack")], None),
        ("s.imperative.procedure.codec.document.v1", "codec", "procedure.document/v1:imperative", &[("codec", "procedure.document/v1"), ("codec-extension", "21:procedure.document/v1:imperative")], None),
        ("s.imperative.procedure.localization.en", "localization", "Procedure", &[], Some(("en", "Procedure"))),
        ("s.imperative.procedure.localization.de", "localization", "Prozedur", &[], Some(("de", "Prozedur"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.imperative.procedure")?);
    for (identity, kind, descriptor, claims, localization) in rows {
        let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(*identity)?, ArtifactCapabilityKind::parse(*kind)?).descriptor(descriptor.as_bytes())?;
        for (namespace, value) in *claims {
            capability = capability.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::parse(*namespace)?, *value)?)?;
        }
        if let Some((locale, text)) = localization {
            capability = capability.localization(ArtifactLocalization::new(ArtifactLocale::parse(*locale)?, *text)?)?;
        }
        definition = definition.capability(capability)?;
    }
    Ok(definition)
}

pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    standards::v1::subsets::any::io::bootstrap_imperative_runtime();
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(schema::procedure_artifact_schema_descriptor())
        .inferences([standards::v1::subsets::any::schema::inferences::procedure_artifact_inference_descriptor()])
        .composers(standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::EditorApp<editor::procedure::ImperativePlayApp>>()
        .try_build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`. Private:
/// `declaration()` above is its only caller (moved here with it from `⚙️engine`, ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE reloc-g7 — kept unexported, not widened).
fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "procedure.document",
                    extension: Some("procedure"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(document_dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(document_dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("procedure.document"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "imperative.procedure.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("imperative.procedure.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "procedure.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("procedure.pack"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Register

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


//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod topology {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                        pub use text::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                    }
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs"]
                    pub mod operations;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ---- Shims: keep pre-migration module paths resolving for external callers ----
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub mod io {
    pub use super::standards::v1::subsets::any::io::*;
}
pub mod document_dsl {
    pub use crate::standards::v1::subsets::any::schema::snapshot::text::*;
}
pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::diff::*;
    }
    pub mod text {
        pub use crate::standards::v1::subsets::any::schema::diff::text::*;
    }
}
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub mod snapshot {
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::snapshot::*;
    }
    pub mod pack {
        pub use crate::standards::v1::subsets::any::schema::snapshot::binary::*;
    }
}

#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod procedure {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🦀️.rs"]
        pub mod engine;

        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕️add-step/🦀️.rs"]
            pub mod add_step;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📍️add-step-at/🦀️.rs"]
            pub mod add_step_at;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚚️move-step/🦀️.rs"]
            pub mod move_step;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧭️move-step-at/🦀️.rs"]
            pub mod move_step_at;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➖️remove-step/🦀️.rs"]
            pub mod remove_step;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✂️remove-step-at/🦀️.rs"]
            pub mod remove_step_at;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏃️run/🦀️.rs"]
            pub mod run;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️set-contributions/🦀️.rs"]
            pub mod set_contributions;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎚️set-step-params/🦀️.rs"]
            pub mod set_step_params;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-step-params-at/🦀️.rs"]
            pub mod set_step_params_at;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📋️main/🦀️.rs"]
                    pub mod main;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📝️script/🦀️.rs"]
                    pub mod script;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs"]
            pub mod document;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod procedure {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/📋️main/🦀️.rs"]
                    pub mod main;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/📝️script/🦀️.rs"]
                    pub mod script;
                }
            }
        }
    }
}

//#region 🧬️ChildRestoreProjection
/// 🧬️ The loaded-parent child projection, read off the snapshot's own derived composition fields — the ONE
/// definition the editor and the viewer both declare. The `ArtifactEditor`/`ArtifactViewer` trait default
/// refuses it (`… did not declare a loaded-parent child projection`), and since PX1 the live envelope load
/// asks for it before the decoded document may replace the store, so an undeclared app fails every live load.
pub fn procedure_child_restore_projection(snapshot: &crate::ProcedureSnapshot) -> Result<store::ChildRestoreProjection<'_>, semio_framework_plugin::Fault> {
    store::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("procedure.child-projection"), error.to_string()))
}
//#endregion 🧬️ChildRestoreProjection
