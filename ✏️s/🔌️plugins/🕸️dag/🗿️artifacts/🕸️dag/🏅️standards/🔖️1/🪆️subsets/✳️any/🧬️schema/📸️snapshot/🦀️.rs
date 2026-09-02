//! 🧬️ DAG snapshot schema — artifact-lane fields only.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`: `nodes`/`edges` are gone from this STRUCT —
//! replaced by a single composed `content: DagContentChild` slot (`s.stdio.semio.graph`). The old
//! `DagSnapshotDsl`/`DagNodeSpecDsl`/`DagNodeKindDsl` mirror existed only to give the derive engine a
//! `Box`-wrapped path through `DagNodeSpec.kind`; since that field is now opaque (hidden inside the
//! composed child, never exposed on this struct), the mirror and its derive are both gone.
//!
//! ⚠️ **The WIRE FORMAT still carries the real `nodes`/`edges` data** (JSON-blob-encoded), not just
//! the opaque handle — matching flow's own `<flow::FlowFixture as ArtifactDsl>::parse_dsl(text).map(
//! Self::from_fixture)` precedent exactly. Reasoning: no `LinkResolver`/child-dispatch seam exists
//! yet (see `🔖️WorkingScene` in the artifact root), so the exact child's local owner is populated
//! in-process by whatever call sets the `content` field (a mutation diff, `from_fixture`, …). A
//! codec that persisted only the bare handle would produce an UNRECOVERABLE snapshot the instant a
//! fresh process parses it (confirmed by a real test failure during this migration: `default_snapshot
//! ()` came back with an empty scene on every fresh run, silently vacuous-passing several inverse-law
//! tests). `parse_dsl`/`decode_pack` therefore mint a fresh self-owned content-addressed child from the
//! decoded nodes/edges every time (deterministic — same data always re-derives the same handle, so
//! peers replaying the same bytes converge); `print_dsl`/`encode_pack` read the current owned scene
//! back out via `dag_working_scene`.
//!
//! 🚪️ The hand-rolled `impl store::ArtifactDsl`/`impl store::ArtifactPack for DagSnapshot` (the
//! codecs implementing the reasoning above) moved to `🚪️io/📸️snapshot/{📝️text,💾️binary}`
//! (design.md §1 CORRECTION) — this file keeps only the struct, its pure defaults, and the
//! framework bridge; no codec logic remains here.

use crate::artifacts::dag::{DagContentChild, DagFixtureEdge, DagNodeSpec};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted DAG document snapshot — schema tag plus the composed `graph` content child.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.dag.dag")]
pub struct DagSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.graph")]
    pub content: DagContentChild,
}

impl Default for DagSnapshot {
    fn default() -> Self {
        default_snapshot()
    }
}

/// 🌱 Canonical default document used by the play app and examples.
pub async fn default_snapshot() -> DagSnapshot {
    crate::artifacts::dag::dsl::parse_dsl(crate::artifacts::dag::dsl::DAG_EXAMPLE_TEXT).expect("bundled dag example DSL must parse")
}
//#endregion 🔖️Snapshot

//#region 🔖️FrameworkBridge
/// 🌉 `infinite_board_port_directed_dag::DagSnapshot` is the FRAMEWORK's own separate persisted
/// projection (backs `DagFixture`/`DagHost`), unrelated to and unaware of this plugin's composed
/// child — the bridge goes through the working-scene converter, never through `nodes`/`edges` fields
/// (this struct no longer has any).
impl From<DagSnapshot> for infinite_board_port_directed_dag::DagSnapshot {
    fn from(value: DagSnapshot) -> Self {
        let scene = crate::artifacts::dag::dag_working_scene(&value);
        Self { schema: value.schema, nodes: scene.nodes, edges: scene.edges }
    }
}

impl From<infinite_board_port_directed_dag::DagSnapshot> for DagSnapshot {
    fn from(value: infinite_board_port_directed_dag::DagSnapshot) -> Self {
        let content = crate::artifacts::dag::dag_content_child_with_owner(value.nodes, value.edges);
        Self { schema: value.schema, content }
    }
}

impl From<&DagSnapshot> for infinite_board_port_directed_dag::DagSnapshot {
    fn from(value: &DagSnapshot) -> Self {
        value.clone().into()
    }
}

/// 🧾️ Node/edge accessors matching the OLD field-access call-site shape (`document.nodes`), now
/// reading through the exact child owner. Kept as methods on `DagSnapshot` itself so call sites do
/// not need to import `dag_working_scene`.
impl DagSnapshot {
    pub async fn nodes(&self) -> Vec<DagNodeSpec> {
        crate::artifacts::dag::dag_working_scene(self).nodes
    }
    pub async fn edges(&self) -> Vec<DagFixtureEdge> {
        crate::artifacts::dag::dag_working_scene(self).edges
    }
}
//#endregion 🔖️FrameworkBridge

//#region 🌉️ExternalCodecBridge
/// 📤️ Renders a [`DagSnapshot`] as this facet's own camelCase JSON projection — the comparison
/// surface `mutate-dag-1`'s scenarios are measured through, and the shape the committed
/// `../🧬️mutations/<slug>/🧪️tests/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors are written in. It carries `content` as a HANDLE, never as a graph, which
/// is exactly what makes it a usable observability surface here: the handle's `childId` is a digest
/// of the child's content, so it moves if and only if the working scene moved.
///
/// A thin `serde_json` wrapper (already a direct dependency of this crate, used behind this
/// interface per CLAUDE.md's "external libraries behind an interface" rule, never a new one).
pub fn encode_dag_snapshot_json(snapshot: &DagSnapshot) -> String {
    serde_json::to_string(snapshot).expect("DagSnapshot serialization is infallible")
}

/// 📥️ The inverse of [`encode_dag_snapshot_json`] — decodes those committed specification vectors
/// into real [`DagSnapshot`] values, so `mutate-dag-1`'s adapter reads the committed fixture rather
/// than re-declaring it as a Rust literal beside it. Reaching `serde_json` from that adapter is
/// impossible: the generated test host links only this crate and `semio-repo-test-host`.
pub fn decode_dag_snapshot_json(text: &str) -> Result<DagSnapshot, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

/// 📝️ Parses `.dag.dsl.semio` text into a [`DagSnapshot`], attaching the working scene to the child
/// handle it mints — a named, non-async pass-through of this type's own `store::ArtifactDsl` impl,
/// whose trait and error type are both unnameable outside this crate. This is the only way an
/// external caller can obtain a dag document whose composed `s.stdio.semio.graph` child actually
/// resolves, which is what `mutate-dag-1` needs before any kind can have a visible effect.
pub fn parse_dag_dsl(text: &str) -> Result<DagSnapshot, String> {
    <DagSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 📝️ Renders a [`DagSnapshot`] back as `.dag.dsl.semio` text — the inverse of [`parse_dag_dsl`],
/// preamble included, which is what makes a printed document comparable to the committed one byte
/// for byte.
pub fn print_dag_dsl(snapshot: &DagSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

/// 🔎️ The node ids and `source -> target` endpoints the document's composed child currently
/// resolves to, in scene order — the human-readable half of a divergence message, so a failing
/// `mutate-<kind>` or `inverse-<kind>` names WHICH node moved rather than only that two content
/// digests differ.
pub fn dag_scene_summary(snapshot: &DagSnapshot) -> String {
    let scene = crate::artifacts::dag::dag_working_scene(snapshot);
    let nodes = scene.nodes.iter().map(|node| format!("{}({},{})", node.id, node.x, node.y)).collect::<Vec<_>>().join(" ");
    let edges = scene.edges.iter().map(|edge| format!("{}:{}->{}", edge.id, edge.source, edge.target)).collect::<Vec<_>>().join(" ");
    format!("nodes[{nodes}] edges[{edges}]")
}
//#endregion 🌉️ExternalCodecBridge
