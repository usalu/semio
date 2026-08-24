//! 🧬️ Jack snapshot schema — artifact-lane fields only.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`: `nodes`/`edges` are gone from this STRUCT —
//! replaced by a single composed `content: JackContentChild` slot (`s.stdio.semio.graph`). See
//! `🗿️artifacts/🔌️jack/🦀️component.rs`'s `🔖️ContentBridge`/`🔖️WorkingScene` regions for the
//! converter/handle/cache machinery this field depends on.

use crate::artifacts::jack::{Camera, JackContentChild, Manifest};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted trinity graph document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.trinity.jack")]
pub struct JackSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub name: String,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest_id: Option<String>,
    #[state(artifact)]
    #[serde(default)]
    pub manifest: Manifest,
    #[state(artifact)]
    pub camera: Camera,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.graph")]
    pub content: JackContentChild,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root_node_id: Option<String>,
}
//#endregion 🔖️Snapshot

impl Default for JackSnapshot {
    fn default() -> Self {
        Self {
            schema: crate::artifacts::jack::TRINITY_GRAPH_SCHEMA.into(),
            name: String::new(),
            manifest_id: None,
            manifest: Manifest::default(),
            camera: Camera::default(),
            content: crate::artifacts::jack::jack_content_child_handle_and_cache(Vec::new(), Vec::new()),
            root_node_id: None,
        }
    }
}

//#region 🌉️ExternalCodecBridge
/// 📤️ Renders a [`JackSnapshot`] as this facet's own camelCase JSON projection — the comparison
/// surface `mutate-jack-1`'s scenarios are measured through, and the shape the committed
/// `../🧬️mutations/<slug>/🧪️tests/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️component.json`
/// specification vectors are written in. It carries `content` as a HANDLE, never as a scene, and
/// that handle's `childId` is a digest of the child — so it moves if and only if the working scene
/// moved, which is what makes it a usable observability surface here.
///
/// A thin `serde_json` wrapper (already a direct dependency of this crate, used behind this
/// interface per CLAUDE.md's "external libraries behind an interface" rule, never a new one).
pub fn encode_jack_snapshot_json(snapshot: &JackSnapshot) -> String {
    serde_json::to_string(snapshot).expect("JackSnapshot serialization is infallible")
}

/// 📥️ The inverse of [`encode_jack_snapshot_json`] — decodes those committed specification vectors
/// into real [`JackSnapshot`] values, so `mutate-jack-1`'s adapter reads the committed fixture
/// rather than re-declaring it as a Rust literal beside it. Reaching `serde_json` from that adapter
/// is impossible: the generated test host links only this crate and `semio-repo-test-host`.
pub fn decode_jack_snapshot_json(text: &str) -> Result<JackSnapshot, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

/// 📝️ Parses `.jack.dsl.semio` text into a [`JackSnapshot`], SEEDING the working-scene cache for the
/// handle it mints — a named, non-async pass-through of this type's own `store::ArtifactDsl` impl
/// (`📝️text/🦀️component.rs`), whose trait and error type are both unnameable outside this crate.
/// This is the only way an external caller can obtain a jack document whose composed
/// `s.stdio.semio.graph` child actually resolves, which is what `mutate-jack-1` needs before any
/// kind can have a visible effect.
pub fn parse_jack_dsl(text: &str) -> Result<JackSnapshot, String> {
    <JackSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 📝️ Renders a [`JackSnapshot`] back as `.jack.dsl.semio` text — the inverse of [`parse_jack_dsl`],
/// preamble and hex-encoded field lines included.
pub fn print_jack_dsl(snapshot: &JackSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

/// 🔎️ The scene's node names and `source -> target` edge ids the document's composed child currently
/// resolves to — the readable half of a divergence message, so a failing scenario names WHICH piece
/// moved rather than only that two content digests differ.
pub fn jack_scene_summary(snapshot: &JackSnapshot) -> String {
    let scene = crate::artifacts::jack::jack_working_scene(snapshot);
    let nodes = scene.nodes.iter().map(|node| format!("{}({})", node.name, node.id)).collect::<Vec<_>>().join(" ");
    let edges = scene.edges.iter().map(|edge| edge.id.clone()).collect::<Vec<_>>().join(" ");
    format!("nodes[{nodes}] edges[{edges}]")
}
//#endregion 🌉️ExternalCodecBridge
