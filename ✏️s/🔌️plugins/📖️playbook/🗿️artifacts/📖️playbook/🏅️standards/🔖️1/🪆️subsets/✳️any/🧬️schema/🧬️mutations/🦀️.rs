//! 🧬️ playbook artifact — semantic parent-lane mutation dispatch enum. Every variant is a
//! single-field tuple wrapping a handcrafted `protocol::MutationKind` payload (see the
//! `🧬️mutations/<slug>/` triad leaves); `#[derive(dsl::Mutations)]` generates
//! `impl protocol::Mutation<PlaybookSnapshot>` and `impl protocol::SemanticMutation<PlaybookSnapshot>`
//! from those payloads — no hand-written apply/diff/inverse dispatch here.
//!
//! Moved from the framework kernel module
//! (`🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🦀️.rs`) by ticket
//! `26/08/12/SEMANTIC-MUTATIONS-OVERHAUL`'s playbook design decision: the dispatch enum cannot stay
//! in the framework and wrap plugin-local payload structs (crate dependency direction — the
//! framework cannot depend on a plugin), so it moves here, matching the other 106 mutation facets.
//! Domain types (`PlaybookStep`/`PlaybookBlock`/`PlaybookExpr`), validation, `generation_forms`, and
//! `builder_kit`'s rendering half stay in the framework kernel (`crate::playbook::*`) — only the
//! mutation vocabulary moved.

use crate::{PlaybookDiff, PlaybookSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};
// 🔬️ `Serialize`/`Deserialize` survive ONLY as a `#[cfg(test)]` differential oracle — committed
// `🧪️tests/<fixture>/🦀️.rs` fixture vectors decode/re-encode through them — never a production
// dependency of this crate.
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 🧮️ Semantic playbook PARENT-lane mutation vocabulary: the playbook's own title scalar. Steps and blocks are composed content
/// of the `flow` child and are edited only on that child's lane (stdio flow leaves, design §20.15 of ticket
/// 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING; see the artifact root's `🔖️ChildLane`).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = PlaybookSnapshot, diff = PlaybookDiff, schema = "playbook.playbook", reference = PlaybookSnapshot, reference_diff = PlaybookDiff)]
pub enum PlaybookMutation {
    ChangeTitle(ChangeTitle),
}
//#endregion 🔖️Mutations

pub use super::change_title::{change_title_operation, ChangeTitle};


/// ↩️ Computes `mutation`'s inverse from the pre-state `snapshot`.
pub fn inverse_playbook_mutation(snapshot: &PlaybookSnapshot, mutation: &PlaybookMutation) -> Result<Vec<PlaybookMutation>, semio_framework_value::ValueError> {
    Ok({
    protocol::Mutation::inverse(mutation, snapshot)?

    })
}

//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every [`PlaybookMutation`] variant, in declaration order — the vocabulary the
/// `playbook-1-any` mutation catalog (`../../🔮️oracles/🔣️.json`) declares and the exhaustive `mutate-*` case measures itself
/// against. The framework never parses Rust, so `kinds_match_the_enum_and_the_catalog` keeps this list honest against both the
/// enum and the committed catalog.
pub const KINDS: &[&str] = &["change-title"];


/// ↩️ `mutation`'s own inverse against `base`, as the step LIST `protocol::Mutation::inverse`
/// returns. Reachable from outside this crate, which `protocol::Mutation` itself is not — the
/// `protocol` extern-crate alias is private to `🦀️.rs`.
// 🚫️async: E1 pure computation over an in-memory snapshot, consumed from a synchronous external test host — see R9
pub fn inverse_playbook_mutation_steps(mutation: &PlaybookMutation, base: &PlaybookSnapshot) -> Result<Vec<PlaybookMutation>, semio_framework_value::ValueError> {
    Ok({
    <PlaybookMutation as protocol::ColdMutation<PlaybookSnapshot>>::inverse(mutation, base)?

    })
}






//#endregion 🔖️Kinds

//#region 🧪️KindsCatalog
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;
//#endregion 🧪️KindsCatalog

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧩️ArtifactItems
pub mod artifact_items {
use semio_framework_artifact_playbook_playbook as playbook;
use {semio_framework_plugin::ArtifactKindSpec, semio_framework_artifact_reference::Dialect, semio_framework_plugin::MediaClass, semio_framework_plugin::MediaForm, semio_framework_plugin::MediaType, semio_framework_plugin::OsMediaCapability, semio_framework_artifact_reference::StandardId, semio_framework_artifact_reference::SubsetId};

//#region 🔖️Types
pub use crate::playbook::{PlaybookBlock, PlaybookBlockOption, PlaybookExpr, PlaybookSpec, PlaybookStep, PlaybookVectorField, PLAYBOOK_BUILTIN_KINDS, PLAYBOOK_DOCUMENT_SCHEMA};
pub use crate::schema::diff::{PlaybookDiff, PlaybookStringList};
pub use crate::schema::mutations::PlaybookMutation;
pub use crate::schema::snapshot::PlaybookSnapshot;

pub const PLAYBOOK_ARTIFACT_SCHEMA_ID: &str = "s.playbook.playbook";

/// 🪪️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1/§7.4 — the canonical
/// `(artifact_kind, standard, subset)` coordinate for this artifact's `✳️any` subset, at the ARTIFACT
/// level (not under `✏️editor`/`👁️viewer`) so the viewer can read it without ever importing through
/// the sibling `editor` module. `artifact_kind` matches this file's own `#[artifact_schema(id = …)]`
/// row (`🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`); `standard`/`subset` match this
/// location on disk — the canonical surface id is `s.playbook.playbook@1/*#editor` /
/// `s.playbook.playbook@1/*#viewer`.
pub const PLAYBOOK_DIALECT: Dialect = Dialect { artifact_kind: "s.playbook.playbook", standard: StandardId("1"), subset: SubsetId::ANY };

/// 📸️ Default persisted playbook document for new stores: its `flow` child is the genesis flow ([`playbook_genesis_flow`]).
pub fn empty_playbook_snapshot() -> PlaybookSnapshot {
    PlaybookSnapshot::default()
}
//#endregion 🔖️Types

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::editor::playbook::create_playbook_play_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "text.playbook".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Playbook", "Playbook"),
        source_format: PLAYBOOK_DOCUMENT_SCHEMA.into(),
        component_kind: "playbook".into(),
        dimension: "text".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
        schema: PLAYBOOK_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
}
//#endregion 🧩️ArtifactItems

//#region 🧩️ArtifactItems2
pub mod artifact_items_2 {
use crate::{PlaybookBlock, PlaybookStep, artifact_kind};
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
use semio_s_artifact_stdio_semio_flow::{schema::snapshot::FlowEdge as SemioFlowEdge, schema::snapshot::FlowNode as SemioFlowNode, schema::snapshot::FlowParam as SemioFlowParam, schema::snapshot::PortRef as SemioPortRef, schema::snapshot::SemioFlowSnapshot, schema::snapshot::STDIO_SEMIOFLOW_DOCUMENT_SCHEMA};
use std::{collections::BTreeMap, collections::BTreeSet};

//#region 🔖️ContentBridge
/// 🕸️ Owned CHILD handle type of the composed `s.stdio.semio@v1/flow` document that holds playbook's steps.
pub type PlaybookFlowChild = store::os_store::artifact_child::ArtifactChild<SemioFlowSnapshot>;

/// 🎫️ The `flow` child a new playbook composes (see [`playbook_genesis_flow`]).
pub const PLAYBOOK_GENESIS_FLOW_ID: &str = "playbook-flow";
/// 🏷️ The node kind of one step.
pub const PLAYBOOK_STEP_NODE_KIND: &str = "step";
/// 🔗️ The edge kind chaining one step to the next.
pub const PLAYBOOK_SEQUENCE_EDGE_KIND: &str = "sequence";
pub const PLAYBOOK_FLOW_SLOT: &str = "flow";
pub const PLAYBOOK_BLOCK_PARAM_PREFIX: &str = "block.";
pub const PLAYBOOK_DESCRIPTION_PARAM: &str = "description";
pub const PLAYBOOK_NEXT_PORT: &str = "next";
pub const PLAYBOOK_PREV_PORT: &str = "prev";
pub const PLAYBOOK_STEP_SPACING: f64 = 220.0;

/// 🧷️ The `flow` child handle named `child_id` — minted once per document and kept by every edit, since every content edit is a
/// child-lane leaf in that child's own store (`child_id` is also the child's artifact id, as `ChildRestoreProjection` requires).
pub fn playbook_flow_child(child_id: &str) -> PlaybookFlowChild {
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "flow".into() };
    store::os_store::artifact_child::ArtifactChild::new(child_id.to_string(), semio_framework_artifact_reference::ArtifactRef { artifact_id: child_id.to_string(), dialect })
}

/// 🗝️ The param key of block `block_id`: one param per block, so a block is inserted, removed and moved as one concrete param row.
pub fn playbook_block_param_key(block_id: &str) -> String {
    format!("{PLAYBOOK_BLOCK_PARAM_PREFIX}{block_id}")
}

/// 🧱️ One step as its flow node at chain position `index`: `label` = title, one `block.<id>` param per block (JSON of that block, in
/// block order), `description` its own param, present only when `Some`.
pub fn playbook_step_node(step: &PlaybookStep, index: usize) -> SemioFlowNode {
    let mut params: Vec<SemioFlowParam> = step.blocks.iter().map(|block| SemioFlowParam { key: playbook_block_param_key(&block.id), value: semio_framework_pack_json::to_json_string(block) }).collect();
    if let Some(description) = &step.description {
        params.push(SemioFlowParam { key: PLAYBOOK_DESCRIPTION_PARAM.into(), value: description.clone() });
    }
    SemioFlowNode { id: step.id.clone(), kind: PLAYBOOK_STEP_NODE_KIND.into(), label: step.title.clone(), params, position: SemioPoint2 { x: index as f64 * PLAYBOOK_STEP_SPACING, y: 0.0 } }
}

/// ⛓️ The `sequence` edge chaining step `from` to step `to`.
pub fn playbook_sequence_edge(from: &str, to: &str) -> SemioFlowEdge {
    SemioFlowEdge {
        id: format!("seq-{from}-{to}"),
        from: SemioPortRef { node: from.into(), port: PLAYBOOK_NEXT_PORT.into() },
        to: SemioPortRef { node: to.into(), port: PLAYBOOK_PREV_PORT.into() },
        kind: PLAYBOOK_SEQUENCE_EDGE_KIND.into(),
    }
}

/// 🌉 Steps → the flow content holding them: one node per step, consecutive steps chained by `sequence` edges.
pub fn flow_content_snapshot_from_steps(steps: &[PlaybookStep]) -> SemioFlowSnapshot {
    let nodes = steps.iter().enumerate().map(|(index, step)| playbook_step_node(step, index)).collect();
    let edges = steps.windows(2).map(|pair| playbook_sequence_edge(&pair[0].id, &pair[1].id)).collect();
    SemioFlowSnapshot { schema: STDIO_SEMIOFLOW_DOCUMENT_SCHEMA.into(), nodes, edges }
}

/// 🔢️ The step ids of `content` in step order: every chain of `sequence` edges walked from its head (a step node no chain edge
/// enters) in node order, then every step node no walk reached, in node order. The chain — not the node vector — is the order,
/// because `insert-node` appends and the inverse of `remove-node` re-appends: an undo never reorders the steps.
pub fn playbook_step_order(content: &SemioFlowSnapshot) -> Vec<&str> {
    let steps: Vec<&str> = content.nodes.iter().filter(|node| node.kind == PLAYBOOK_STEP_NODE_KIND).map(|node| node.id.as_str()).collect();
    let next: BTreeMap<&str, &str> = playbook_chain(content).map(|edge| (edge.from.node.as_str(), edge.to.node.as_str())).collect();
    let entered: BTreeSet<&str> = next.values().copied().collect();
    let known: BTreeSet<&str> = steps.iter().copied().collect();
    let mut seen = BTreeSet::new();
    let mut order = Vec::with_capacity(steps.len());
    for head in steps.iter().copied().filter(|id| !entered.contains(id)).chain(steps.iter().copied()) {
        let mut cursor = Some(head);
        while let Some(id) = cursor.filter(|id| known.contains(id) && seen.insert(*id)) {
            order.push(id);
            cursor = next.get(id).copied();
        }
    }
    order
}

pub fn playbook_chain(content: &SemioFlowSnapshot) -> impl Iterator<Item = &SemioFlowEdge> {
    content.edges.iter().filter(|edge| edge.kind == PLAYBOOK_SEQUENCE_EDGE_KIND && edge.from.port == PLAYBOOK_NEXT_PORT && edge.to.port == PLAYBOOK_PREV_PORT)
}

/// 🔓️ One step node → its step, losslessly: every `PlaybookStep` field (the full `blocks` vocabulary included) round-trips through
/// its `block.<id>` params (in param order) and `description`; a block param that does not decode is refused with the node named.
pub fn playbook_step_from_node(node: &SemioFlowNode) -> Result<PlaybookStep, String> {
    let param = |key: &str| node.params.iter().find(|param| param.key == key).map(|param| param.value.as_str());
    let blocks = node
        .params
        .iter()
        .filter(|param| param.key.starts_with(PLAYBOOK_BLOCK_PARAM_PREFIX))
        .map(|param| semio_framework_pack_json::from_json_str(&param.value, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| format!("step {:?}: {error}", node.id)))
        .collect::<Result<Vec<PlaybookBlock>, String>>()?;
    Ok(PlaybookStep { id: node.id.clone(), title: node.label.clone(), description: param(PLAYBOOK_DESCRIPTION_PARAM).map(str::to_string), blocks })
}

/// 🔁️ Flow content → its steps in step order ([`playbook_step_order`]).
pub fn steps_from_flow_content(content: &SemioFlowSnapshot) -> Result<Vec<PlaybookStep>, String> {
    let nodes: BTreeMap<&str, &SemioFlowNode> = content.nodes.iter().map(|node| (node.id.as_str(), node)).collect();
    playbook_step_order(content).into_iter().map(|id| playbook_step_from_node(nodes[id])).collect()
}
//#endregion 🔖️ContentBridge
}
//#endregion 🧩️ArtifactItems2
