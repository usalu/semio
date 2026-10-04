//! 📖️ Playbook artifact — the document entity this plugin's app edits.
//!
//! Step/block/expr records live in the shared kernel `playbook` crate; this plugin owns
//! `PlaybookSnapshot`, `PlaybookArtifact`, facet schemas, and app-facing wrappers.
//!
//! The steps live in ONE composed CHILD, `flow` (stdio's `s.stdio.semio@v1/flow`): one `FlowNode` per step, its
//! `blocks`/`description` JSON-encoded into params, the step order the chain of `sequence` edges. Composed content is edited
//! only on the child lane and read by composing parent + child on read (design §20.15 of ticket
//! 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING) — see `🔖️ContentBridge` and `🔖️ChildLane` below.

extern crate semio_framework as semio_framework;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
mod art_playbook_demo_tests;
extern crate semio_framework_schema as framework_schema;
use semio_framework_artifact_playbook_playbook as playbook;

use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};
use semio_framework_plugin::{ChildContentView, Fault, FaultCode, FaultOrigin};
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::mutations::{insert_edge, insert_node, remove_edge, remove_node, set_node_param, SemioFlowMutation};
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::{FlowEdge as SemioFlowEdge, FlowNode as SemioFlowNode, FlowParam as SemioFlowParam, PortRef as SemioPortRef, SemioFlowSnapshot, STDIO_SEMIOFLOW_DOCUMENT_SCHEMA};
use std::collections::{BTreeMap, BTreeSet};

//#region 🔖️Types
pub use crate::playbook::{PlaybookBlock, PlaybookBlockOption, PlaybookExpr, PlaybookSpec, PlaybookStep, PlaybookVectorField, PLAYBOOK_BUILTIN_KINDS, PLAYBOOK_DOCUMENT_SCHEMA};
pub use crate::schema::diff::{PlaybookDiff, PlaybookStringList};
pub use crate::schema::mutations::PlaybookMutation;
pub use crate::schema::snapshot::PlaybookSnapshot;
pub use crate::schema::PlaybookArtifact;

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

//#region 🔖️ContentBridge
/// 🕸️ Owned CHILD handle type of the composed `s.stdio.semio@v1/flow` document that holds playbook's steps.
pub type PlaybookFlowChild = store::ArtifactChild<SemioFlowSnapshot>;

/// 🎫️ The `flow` child a new playbook composes (see [`playbook_genesis_flow`]).
pub const PLAYBOOK_GENESIS_FLOW_ID: &str = "playbook-flow";
/// 🏷️ The node kind of one step.
pub const PLAYBOOK_STEP_NODE_KIND: &str = "step";
/// 🔗️ The edge kind chaining one step to the next.
pub const PLAYBOOK_SEQUENCE_EDGE_KIND: &str = "sequence";
const PLAYBOOK_FLOW_SLOT: &str = "flow";
const PLAYBOOK_BLOCKS_PARAM: &str = "blocksJson";
const PLAYBOOK_DESCRIPTION_PARAM: &str = "description";
const PLAYBOOK_NEXT_PORT: &str = "next";
const PLAYBOOK_PREV_PORT: &str = "prev";
const PLAYBOOK_STEP_SPACING: f64 = 220.0;

/// 🧷️ The `flow` child handle named `child_id` — minted once per document and kept by every edit, since every content edit is a
/// child-lane leaf in that child's own store (`child_id` is also the child's artifact id, as `ChildRestoreProjection` requires).
pub fn playbook_flow_child(child_id: &str) -> PlaybookFlowChild {
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "flow".into() };
    store::ArtifactChild::new(child_id.to_string(), store::os_io::ArtifactRef { artifact_id: child_id.to_string(), dialect })
}

/// 🧱️ One step as its flow node at chain position `index`: `label` = title, `blocks` JSON-encoded wholesale into the
/// `blocksJson` param (the "honest string boundary" flow's own widget converter established), `description` its own param, present
/// only when `Some`.
pub fn playbook_step_node(step: &PlaybookStep, index: usize) -> SemioFlowNode {
    let mut params = vec![SemioFlowParam { key: PLAYBOOK_BLOCKS_PARAM.into(), value: semio_framework_pack_json::to_json_string(&step.blocks) }];
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

fn playbook_chain(content: &SemioFlowSnapshot) -> impl Iterator<Item = &SemioFlowEdge> {
    content.edges.iter().filter(|edge| edge.kind == PLAYBOOK_SEQUENCE_EDGE_KIND && edge.from.port == PLAYBOOK_NEXT_PORT && edge.to.port == PLAYBOOK_PREV_PORT)
}

/// 🔓️ One step node → its step, losslessly: every `PlaybookStep` field (the full `blocks` vocabulary included) round-trips through
/// `blocksJson`/`description`; a `blocksJson` that does not decode is refused with the node named.
pub fn playbook_step_from_node(node: &SemioFlowNode) -> Result<PlaybookStep, String> {
    let param = |key: &str| node.params.iter().find(|param| param.key == key).map(|param| param.value.as_str());
    let blocks = semio_framework_pack_json::from_json_str(param(PLAYBOOK_BLOCKS_PARAM).unwrap_or("[]"), semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| format!("step {:?}: {error}", node.id))?;
    Ok(PlaybookStep { id: node.id.clone(), title: node.label.clone(), description: param(PLAYBOOK_DESCRIPTION_PARAM).map(str::to_string), blocks })
}

/// 🔁️ Flow content → its steps in step order ([`playbook_step_order`]).
pub fn steps_from_flow_content(content: &SemioFlowSnapshot) -> Result<Vec<PlaybookStep>, String> {
    let nodes: BTreeMap<&str, &SemioFlowNode> = content.nodes.iter().map(|node| (node.id.as_str(), node)).collect();
    playbook_step_order(content).into_iter().map(|id| playbook_step_from_node(nodes[id])).collect()
}
//#endregion 🔖️ContentBridge

//#region 🔖️ComposeOnRead
const PLAYBOOK_FLOW_UNAVAILABLE: &str = "playbook.flow.unavailable";
const PLAYBOOK_FLOW_DIALECT: &str = "playbook.flow.dialect";
const PLAYBOOK_FLOW_CONTENT: &str = "playbook.flow.content";
const PLAYBOOK_FLOW_PROJECTION: &str = "playbook.flow.projection";
const PLAYBOOK_STEP_MISSING: &str = "playbook.step.missing";
const PLAYBOOK_STEP_DUPLICATE: &str = "playbook.step.duplicate";
const PLAYBOOK_BLOCK_MISSING: &str = "playbook.block.missing";

fn playbook_fault(code: &'static str, message: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new(code), message)
}

/// 📣️ The localized notice of every refusal this artifact raises (design §20.12), declared by its editor and viewer.
pub fn playbook_fault_notices() -> &'static [(&'static str, semio_framework_ui_locale::LocalizedLabel)] {
    use semio_framework_ui_locale::LocalizedLabel;
    static NOTICES: std::sync::LazyLock<[(&str, LocalizedLabel); 7]> = std::sync::LazyLock::new(|| {
        [
            (PLAYBOOK_FLOW_UNAVAILABLE, LocalizedLabel::native("The playbook's steps are not loaded yet.", "Die Schritte des Playbooks sind noch nicht geladen.")),
            (PLAYBOOK_FLOW_DIALECT, LocalizedLabel::native("The playbook's steps are stored in an unsupported format.", "Die Schritte des Playbooks liegen in einem nicht unterstützten Format vor.")),
            (PLAYBOOK_FLOW_CONTENT, LocalizedLabel::native("A step of the playbook cannot be read.", "Ein Schritt des Playbooks kann nicht gelesen werden.")),
            (PLAYBOOK_FLOW_PROJECTION, LocalizedLabel::native("The playbook's steps cannot be restored.", "Die Schritte des Playbooks können nicht wiederhergestellt werden.")),
            (PLAYBOOK_STEP_MISSING, LocalizedLabel::native("The step no longer exists.", "Der Schritt existiert nicht mehr.")),
            (PLAYBOOK_STEP_DUPLICATE, LocalizedLabel::native("A step with this id already exists.", "Ein Schritt mit dieser Id existiert bereits.")),
            (PLAYBOOK_BLOCK_MISSING, LocalizedLabel::native("The block no longer exists.", "Der Baustein existiert nicht mehr.")),
        ]
    });
    &*NOTICES
}

/// 🪆️ The loaded-parent child projection: the one `flow` child `snapshot` names, so a reload restores exactly that member.
pub fn playbook_child_restore_projection(snapshot: &PlaybookSnapshot) -> Result<store::ChildRestoreProjection<'_>, Fault> {
    store::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| playbook_fault(PLAYBOOK_FLOW_PROJECTION, error.to_string()))
}

/// 🌊️ The `flow` child content `snapshot` names, read through `children` (design §20.15: readers compose on read, never through
/// the handle). A child that is not composed, or composed as anything but `s.stdio.semio@v1/flow`, is a named fault.
pub fn playbook_flow_content<'a>(snapshot: &PlaybookSnapshot, children: &'a ChildContentView) -> Result<store::SnapshotReadRef<'a, SemioFlowSnapshot>, Fault> {
    let child_id = snapshot.flow.child_id.as_str();
    match children.dialect(PLAYBOOK_FLOW_SLOT, child_id) {
        Some(dialect) if dialect.artifact_kind == "s.stdio.semio" && dialect.standard == "v1" && dialect.subset == "flow" => children.typed_read::<SemioFlowSnapshot>(PLAYBOOK_FLOW_SLOT, child_id),
        Some(dialect) => Err(playbook_fault(PLAYBOOK_FLOW_DIALECT, format!("the playbook flow child {child_id:?} is {}@{}/{}, not s.stdio.semio@v1/flow", dialect.artifact_kind, dialect.standard, dialect.subset))),
        None => Err(playbook_fault(PLAYBOOK_FLOW_UNAVAILABLE, format!("the playbook flow child {child_id:?} is not composed"))),
    }
}

/// 🧮️ The playbook every reader works on: the parent's own fields composed with the steps of its `flow` child.
pub fn playbook_composed_spec(snapshot: &PlaybookSnapshot, children: &ChildContentView) -> Result<PlaybookSpec, Fault> {
    let content = playbook_flow_content(snapshot, children)?;
    let steps = steps_from_flow_content(&content).map_err(|message| playbook_fault(PLAYBOOK_FLOW_CONTENT, message))?;
    Ok(PlaybookSpec { schema: snapshot.schema.clone(), id: snapshot.id.clone(), version: snapshot.version.clone(), title: snapshot.title.clone(), steps })
}

/// 🌱️ The flow content a document's `flow` child `child_id` is composed from when no archive member carries it — answered by
/// id from this plugin's own catalogue (the empty playbook's flow, every built-in example's), never from the parent.
pub fn playbook_genesis_flow(child_id: &str) -> Option<SemioFlowSnapshot> {
    match child_id {
        PLAYBOOK_GENESIS_FLOW_ID => Some(flow_content_snapshot_from_steps(&playbook::empty_playbook_snapshot().steps)),
        examples::demo::FLOW_ID => examples::demo::flow().ok(),
        _ => None,
    }
}

/// 🧩️ The genesis pack of `snapshot`'s `flow` child ([`playbook_genesis_flow`]) for a whole-document load whose archive carries
/// no such member.
pub fn genesis_playbook_child_pack(snapshot: &PlaybookSnapshot, slot: &str, child_id: &str) -> Option<Vec<u8>> {
    use store::ArtifactPack;
    if slot != PLAYBOOK_FLOW_SLOT || child_id != snapshot.flow.child_id {
        return None;
    }
    playbook_genesis_flow(child_id).map(|content| <SemioFlowSnapshot as ArtifactPack>::encode_pack(&content))
}
//#endregion 🔖️ComposeOnRead

//#region 🔖️ChildLane
/// 🧬️ The `ChildEmit` that publishes `leaves` on `snapshot`'s `flow` child — the ONE lane every content edit takes.
pub fn playbook_flow_emit(snapshot: &PlaybookSnapshot, leaves: &[SemioFlowMutation]) -> semio_framework_plugin::app::ChildEmit {
    semio_framework_plugin::app::ChildEmit::of::<SemioFlowSnapshot, _>(PLAYBOOK_FLOW_SLOT, snapshot.flow.child_id.as_str(), leaves)
}

/// 🪢️ The leaves that make `content`'s step chain read `order`: every `sequence` edge the new chain drops is removed first, every
/// one it gains inserted after, so a reorder keeps every node and its identity.
pub fn playbook_chain_leaves(content: &SemioFlowSnapshot, order: &[&str]) -> Vec<SemioFlowMutation> {
    let wanted: BTreeSet<(&str, &str)> = order.windows(2).map(|pair| (pair[0], pair[1])).collect();
    let present: BTreeSet<(&str, &str)> = playbook_chain(content).map(|edge| (edge.from.node.as_str(), edge.to.node.as_str())).collect();
    let removed = playbook_chain(content).filter(|edge| !wanted.contains(&(edge.from.node.as_str(), edge.to.node.as_str()))).map(|edge| SemioFlowMutation::RemoveEdge(remove_edge::RemoveEdge { id: edge.id.clone() }));
    let inserted = order.windows(2).filter(|pair| !present.contains(&(pair[0], pair[1]))).map(|pair| SemioFlowMutation::InsertEdge(insert_edge::InsertEdge::new(playbook_sequence_edge(pair[0], pair[1]))));
    removed.chain(inserted).collect()
}

fn playbook_step_node_of<'a>(content: &'a SemioFlowSnapshot, step_id: &str) -> Result<&'a SemioFlowNode, Fault> {
    content.nodes.iter().find(|node| node.id == step_id && node.kind == PLAYBOOK_STEP_NODE_KIND).ok_or_else(|| playbook_fault(PLAYBOOK_STEP_MISSING, format!("Step \"{step_id}\" does not exist.")))
}

/// 📋️ Step `step_id` of `content`, decoded; a missing step or an undecodable `blocksJson` is a named fault.
pub fn playbook_step_of(content: &SemioFlowSnapshot, step_id: &str) -> Result<PlaybookStep, Fault> {
    playbook_step_from_node(playbook_step_node_of(content, step_id)?).map_err(|message| playbook_fault(PLAYBOOK_FLOW_CONTENT, message))
}

/// 🎛️ The ONE absolute leaf that sets step `step_id`'s blocks to `blocks`.
pub fn playbook_blocks_leaf(step_id: &str, blocks: Vec<PlaybookBlock>) -> SemioFlowMutation {
    SemioFlowMutation::SetNodeParam(set_node_param::SetNodeParam { id: step_id.into(), key: PLAYBOOK_BLOCKS_PARAM.into(), value: semio_framework_pack_json::to_json_string(&blocks) })
}

/// ➕️ Appends `step`: its node, then its chain edge; a step id already present is refused.
pub fn playbook_add_step_leaves(content: &SemioFlowSnapshot, step: &PlaybookStep) -> Result<Vec<SemioFlowMutation>, Fault> {
    if content.nodes.iter().any(|node| node.id == step.id) {
        return Err(playbook_fault(PLAYBOOK_STEP_DUPLICATE, format!("Step \"{}\" already exists.", step.id)));
    }
    let mut order = playbook_step_order(content);
    let node = SemioFlowMutation::InsertNode(insert_node::InsertNode::new(playbook_step_node(step, order.len())));
    order.push(&step.id);
    Ok(std::iter::once(node).chain(playbook_chain_leaves(content, &order)).collect())
}

/// ➖️ Removes step `step_id`: its chain edges and the bridge over it first, then its node.
pub fn playbook_remove_step_leaves(content: &SemioFlowSnapshot, step_id: &str) -> Result<Vec<SemioFlowMutation>, Fault> {
    playbook_step_node_of(content, step_id)?;
    let order: Vec<&str> = playbook_step_order(content).into_iter().filter(|id| *id != step_id).collect();
    Ok(playbook_chain_leaves(content, &order).into_iter().chain(std::iter::once(SemioFlowMutation::RemoveNode(remove_node::RemoveNode { id: step_id.into() }))).collect())
}

/// ↔️ Moves step `step_id` to position `index` (clamped) by rewiring the chain only; a step already there yields no leaf.
pub fn playbook_move_step_leaves(content: &SemioFlowSnapshot, step_id: &str, index: usize) -> Result<Vec<SemioFlowMutation>, Fault> {
    playbook_step_node_of(content, step_id)?;
    let mut order: Vec<&str> = playbook_step_order(content).into_iter().filter(|id| *id != step_id).collect();
    order.insert(index.min(order.len()), step_id);
    Ok(playbook_chain_leaves(content, &order))
}

/// ✍️ Step `step_id`'s blocks as `edit` leaves them: ONE absolute `blocksJson` set (none when `edit` changes nothing).
pub fn playbook_edit_blocks_leaves(content: &SemioFlowSnapshot, step_id: &str, edit: impl FnOnce(&mut Vec<PlaybookBlock>) -> Result<(), Fault>) -> Result<Vec<SemioFlowMutation>, Fault> {
    let step = playbook_step_of(content, step_id)?;
    let mut blocks = step.blocks.clone();
    edit(&mut blocks)?;
    Ok(if blocks == step.blocks { Vec::new() } else { vec![playbook_blocks_leaf(step_id, blocks)] })
}

/// 🆕️ Inserts `block` into step `step_id` at `index` (appended when `None` or past the end).
pub fn playbook_add_block_leaves(content: &SemioFlowSnapshot, step_id: &str, block: PlaybookBlock, index: Option<usize>) -> Result<Vec<SemioFlowMutation>, Fault> {
    playbook_edit_blocks_leaves(content, step_id, |blocks| {
        blocks.insert(index.unwrap_or(blocks.len()).min(blocks.len()), block);
        Ok(())
    })
}

/// 🚮️ Removes block `block_id` from step `step_id`; a block the step does not hold is a named fault.
pub fn playbook_remove_block_leaves(content: &SemioFlowSnapshot, step_id: &str, block_id: &str) -> Result<Vec<SemioFlowMutation>, Fault> {
    playbook_edit_blocks_leaves(content, step_id, |blocks| {
        let position = blocks.iter().position(|block| block.id == block_id).ok_or_else(|| playbook_fault(PLAYBOOK_BLOCK_MISSING, format!("Block \"{block_id}\" is not in step \"{step_id}\".")))?;
        blocks.remove(position);
        Ok(())
    })
}

/// 🚚️ Moves block `block_id` from step `from` to position `index` (clamped) of step `to`: one leaf within a step, one per step
/// across two.
pub fn playbook_move_block_leaves(content: &SemioFlowSnapshot, block_id: &str, from: &str, to: &str, index: usize) -> Result<Vec<SemioFlowMutation>, Fault> {
    let mut source = playbook_step_of(content, from)?.blocks;
    let position = source.iter().position(|block| block.id == block_id).ok_or_else(|| playbook_fault(PLAYBOOK_BLOCK_MISSING, format!("Block \"{block_id}\" is not in step \"{from}\".")))?;
    let block = source.remove(position);
    if from == to {
        return playbook_edit_blocks_leaves(content, from, |blocks| {
            *blocks = source;
            blocks.insert(index.min(blocks.len()), block);
            Ok(())
        });
    }
    let mut target = playbook_step_of(content, to)?.blocks;
    target.insert(index.min(target.len()), block);
    Ok(vec![playbook_blocks_leaf(from, source), playbook_blocks_leaf(to, target)])
}
//#endregion 🔖️ChildLane

//#region 🔖️Register
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called five different global registries directly from
/// a plugin `.setup()` callback. `crate::editor::playbook::config::schema::register_app_schema()` is the
/// one exception, still called from this file's own `.setup()`: it registers the `PlaybookPlayApp`
/// CONFIG/PRESENCE schema, an app-scope concern `ArtifactDeclaration` deliberately has no field for
/// (see that struct's own doc) — `register_app_schema_descriptor` is not in §6's artifact-scoped
/// function set. Lives at the artifact root, not `⚙️engine` (reloc-g7 revision of that same ticket) —
/// `declaration()` describes the artifact (kind/schema/io/ownership), it is not engine behaviour.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    type CapabilityRow<'a> = (&'a str, &'a str, &'a str, &'a [(&'a str, &'a str)], Option<(&'a str, &'a str)>);
    let rows: &[CapabilityRow<'_>] = &[
        ("s.playbook.playbook.standard.v1", "standard", "1", &[], None),
        ("s.playbook.playbook.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.playbook.playbook.schema.artifact", "schema", "s.playbook.playbook", &[("schema", "s.playbook.playbook")], None),
        ("s.playbook.playbook.inference.artifact", "inference", "s.playbook.playbook.inference", &[("schema", "s.playbook.playbook.inference")], None),
        // 🐛️ D2-capability-claim-repairs: `io_registry::entries()` registers SIX composer rows, not
        // five — the five below plus `composer_entry_of::<PlaybookAnyComposer>()` (`🚪️io/🦀️.rs`),
        // whose `writes` is this artifact's own native dialect (`PLAYBOOK_DIALECT`, `s.playbook@1/*`),
        // the same gap class `🗒️note` hit first (see that file's own `definition()` doc comment).
        ("s.playbook.playbook.composer.playbook", "composer", "s.playbook.playbook@1/*", &[("dialect", "s.playbook.playbook@1/*")], None),
        ("s.playbook.playbook.composer.txt", "composer", "s.stdio.txt@utf-8/*", &[("dialect", "s.stdio.txt@utf-8/*")], None),
        ("s.playbook.playbook.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.playbook.playbook.grammar.document", "grammar", "playbook.playbook", &[("grammar", "playbook.playbook")], None),
        ("s.playbook.playbook.grammar.op", "grammar", "playbook.playbook.op", &[("grammar", "playbook.playbook.op")], None),
        ("s.playbook.playbook.grammar.diff", "grammar", "playbook.playbook.diff", &[("grammar", "playbook.playbook.diff")], None),
        ("s.playbook.playbook.grammar.pack", "grammar", "playbook.pack", &[("grammar", "playbook.pack")], None),
        ("s.playbook.playbook.grammar.spr", "grammar", "playbook.spr", &[("grammar", "playbook.spr")], None),
        ("s.playbook.playbook.codec.document.v1", "codec", "playbook.playbook:playbook", &[("codec", "playbook.playbook"), ("codec-extension", "17:playbook.playbook:playbook")], None),
        ("s.playbook.playbook.localization.en", "localization", "Playbook", &[], Some(("en", "Playbook"))),
        ("s.playbook.playbook.localization.de", "localization", "Playbook", &[], Some(("de", "Playbook"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.playbook.playbook")?);
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

/// 🌳️ This artifact's declaration tree root (ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-
/// RUNTIME`, `terra-descriptors` packet, following the `terra-fleet-trinity-recipe` recipe) —
/// replaces the old `declaration()` (`ArtifactDeclaration::builder(...).schema(...).inferences(...)
/// .composers(...).languages(...).document_codec(...)` chain, deleted outright, no dual channel) as
/// the ONLY registration channel for schema/io/viewer/editor rows. `definition()` (old
/// `ArtifactDefinition`/capability rows, above) is kept per debt D1.
pub fn artifact<A: PlaybookApplication>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<A> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.playbook.playbook").expect("canonical playbook kind"), localization: &[], standards: vec![standards::v1::standard()] }
}

/// 🧰️ Application variants required to assemble the Playbook artifact.
pub trait PlaybookApplication:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<editor::playbook::PlaybookPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>>
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<viewer::playbook::PlaybookViewer>, semio_s_artifact_stdio_semio::SemioMembers>>
{
}

impl<A> PlaybookApplication for A where
    A: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<editor::playbook::PlaybookPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>>
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<viewer::playbook::PlaybookViewer>, semio_s_artifact_stdio_semio::SemioMembers>>
{
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`. Private:
/// `declaration()` above is its only caller (moved here with it from `⚙️engine`, ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE reloc-g7 — kept unexported, not widened).
pub fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "playbook.playbook",
                    extension: Some("playbook"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(document_dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(document_dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("playbook.playbook"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "playbook.playbook.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(spr::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("playbook.playbook.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "playbook.playbook.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(schema::diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("playbook.playbook.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "playbook.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("playbook.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "playbook.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(spr::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("playbook.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Register

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

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "🏅️standards/🔖️1/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs"]
                mod component;
                pub use component::*;

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
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod change_title {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️change-title/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️change-title/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️change-title/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️change-title/🧪️tests/🧪️changes/🦀️.rs"]
                            mod tests_changes_the_playbook_title;
                        }
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
pub mod op {
    pub use crate::standards::v1::subsets::any::schema::mutations::text::*;
    pub use crate::standards::v1::subsets::any::schema::mutations::{apply_playbook_mutation, PlaybookMutation};
}
pub mod document_dsl {
    pub use crate::standards::v1::subsets::any::schema::snapshot::text::*;
}
pub mod spr {
    pub use crate::standards::v1::subsets::any::schema::mutations::binary::*;
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
    pub mod playbook {
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

        #[path = "."]
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧱️add-block/🦀️.rs"]
            pub mod add_block;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪜️add-step/🦀️.rs"]
            pub mod add_step;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚚️move-block/🦀️.rs"]
            pub mod move_block;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↔️move-step/🦀️.rs"]
            pub mod move_step;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚮️remove-block/🦀️.rs"]
            pub mod remove_block;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➖️remove-step/🦀️.rs"]
            pub mod remove_step;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️set-contributions/🦀️.rs"]
            pub mod set_contributions;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/♻️update-playbook/🦀️.rs"]
            pub mod update_playbook;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod builder {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🏗️builder/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod builder {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🏗️builder/🪟️windows/🏗️builder/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod changes {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🏗️builder/🪟️windows/🔺️changes/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod steps {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🏗️builder/🪟️windows/📋️steps/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod activity {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🏗️builder/🪟️windows/📰️activity/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod source {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🏗️builder/🪟️windows/📜️source/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod files {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🏗️builder/🪟️windows/🗂️files/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod playbook {
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
                    #[path = "."]
                    pub mod steps {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🌳️steps/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
