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
//! yet (see `🔖️WorkingScene` in the artifact root), so the working-scene cache is only populated
//! in-process, by whatever call SET the `content` field (a mutation diff, `from_fixture`, …). A
//! codec that persisted only the bare handle would produce an UNRECOVERABLE snapshot the instant a
//! fresh process parses it (confirmed by a real test failure during this migration: `default_snapshot
//! ()` came back with an empty scene on every fresh run, silently vacuous-passing several inverse-law
//! tests). `parse_dsl`/`decode_pack` therefore mint+cache a FRESH content-addressed handle from the
//! decoded nodes/edges every time (deterministic — same data always re-derives the same handle, so
//! peers replaying the same bytes converge); `print_dsl`/`encode_pack` read the CURRENT cached scene
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
pub fn default_snapshot() -> DagSnapshot {
    crate::artifacts::dag::dsl::parse_dsl(crate::artifacts::dag::dsl::DAG_EXAMPLE_TEXT)
        .expect("bundled dag example DSL must parse")
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
        let content = crate::artifacts::dag::dag_content_child_handle_and_cache(value.nodes, value.edges);
        Self { schema: value.schema, content }
    }
}

impl From<&DagSnapshot> for infinite_board_port_directed_dag::DagSnapshot {
    fn from(value: &DagSnapshot) -> Self {
        value.clone().into()
    }
}

/// 🧾️ Node/edge accessors matching the OLD field-access call-site shape (`document.nodes`), now
/// reading through the working-scene cache. Kept as methods on `DagSnapshot` itself (rather than
/// forcing every call site to import `dag_working_scene`) to minimize the app-layer rewrite's blast
/// radius — see `crate::artifacts::dag::dag_working_scene` for the underlying cache.
impl DagSnapshot {
    pub fn nodes(&self) -> Vec<DagNodeSpec> {
        crate::artifacts::dag::dag_working_scene(self).nodes
    }
    pub fn edges(&self) -> Vec<DagFixtureEdge> {
        crate::artifacts::dag::dag_working_scene(self).edges
    }
}
//#endregion 🔖️FrameworkBridge
