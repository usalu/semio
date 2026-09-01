//! 🧬️ Jack snapshot schema — artifact-lane fields only.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`: `nodes`/`edges` are gone from this STRUCT —
//! replaced by a single composed `content: JackContentChild` slot (`s.stdio.semio.graph`). See
//! `🗿️artifacts/🔌️jack/🦀️component.rs`'s `🔖️ContentBridge`/`🔖️WorkingScene` regions for the
//! converter/handle/cache machinery this field depends on.

use crate::artifacts::jack::{Camera, JackContentChild, Manifest};
use schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted trinity graph document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, ArtifactSchema)]
#[artifact_schema(id = "s.trinity.jack")]
pub struct JackSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub name: String,
    #[state(artifact)]
    pub manifest_id: Option<String>,
    #[state(artifact)]
    pub manifest: Manifest,
    #[state(artifact)]
    pub camera: Camera,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.graph")]
    pub content: JackContentChild,
    #[state(artifact)]
    pub root_node_id: Option<String>,
}
//#endregion 🔖️Snapshot

//#region 🔖️ValueCodec
/// 🔀️ Hand-written, not derived: `content` is a `store::ArtifactChild<S>` composed-artifact
/// handle — see `JackArtifact`'s identical trap in the sibling `🦀️component.rs` (this struct's
/// non-child fields carried `#[serde(default, skip_serializing_if = "Option::is_none")]`
/// before this wave; `manifest_id`/`root_node_id` are `Option<String>`, and the blanket
/// `impl<T: FromValue> FromValue for Option<T>` already treats a missing key as `None` via the
/// derive macro's own generated `missing` arm being unreachable here since every field is
/// present below — no separate default handling needed in a hand-written impl).
impl dsl::ToValue for JackSnapshot {
    fn to_value(&self) -> dsl::DslValue {
        dsl::DslValue::object([
            ("schema".to_string(), dsl::ToValue::to_value(&self.schema)),
            ("name".to_string(), dsl::ToValue::to_value(&self.name)),
            ("manifestId".to_string(), dsl::ToValue::to_value(&self.manifest_id)),
            ("manifest".to_string(), dsl::ToValue::to_value(&self.manifest)),
            ("camera".to_string(), dsl::ToValue::to_value(&self.camera)),
            ("content".to_string(), dsl::to_dsl_value(&self.content).expect("ArtifactChild serializes")),
            ("rootNodeId".to_string(), dsl::ToValue::to_value(&self.root_node_id)),
        ])
    }
}
impl dsl::FromValue for JackSnapshot {
    fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        let entries = value.into_object()?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        Ok(Self {
            schema: match get("schema") { Some(v) => dsl::FromValue::from_value(v)?, None => Default::default() },
            name: match get("name") { Some(v) => dsl::FromValue::from_value(v)?, None => Default::default() },
            manifest_id: match get("manifestId") { Some(v) => dsl::FromValue::from_value(v)?, None => None },
            manifest: match get("manifest") { Some(v) => dsl::FromValue::from_value(v)?, None => Default::default() },
            camera: match get("camera") { Some(v) => dsl::FromValue::from_value(v)?, None => Default::default() },
            content: dsl::from_dsl_value(get("content").ok_or_else(|| dsl::ValueError::new("missing field `content`"))?).map_err(dsl::ValueError::new)?,
            root_node_id: match get("rootNodeId") { Some(v) => dsl::FromValue::from_value(v)?, None => None },
        })
    }
}
//#endregion 🔖️ValueCodec

impl Default for JackSnapshot {
    fn default() -> Self {
        Self {
            schema: crate::artifacts::jack::TRINITY_GRAPH_SCHEMA.into(),
            name: String::new(),
            manifest_id: None,
            manifest: Manifest::default(),
            camera: Camera::default(),
            content: crate::artifacts::jack::jack_content_child_with_owner(Vec::new(), Vec::new()),
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
/// A thin `pack::json` wrapper over [`JackSnapshot`]'s own `ToValue`, bridged through
/// `pack::json_from_dsl_value` since `DslValue` and `pack::json::Value` are sibling trees (used
/// behind this interface per CLAUDE.md's "external libraries behind an interface" rule).
pub fn encode_jack_snapshot_json(snapshot: &JackSnapshot) -> String {
    pack::json_to_string(&pack::json_from_dsl_value(&dsl::ToValue::to_value(snapshot)))
}

/// 📥️ The inverse of [`encode_jack_snapshot_json`] — decodes those committed specification vectors
/// into real [`JackSnapshot`] values, so `mutate-jack-1`'s adapter reads the committed fixture
/// rather than re-declaring it as a Rust literal beside it.
pub fn decode_jack_snapshot_json(text: &str) -> Result<JackSnapshot, String> {
    let parsed = pack::parse_json(text).map_err(|error| error.to_string())?;
    <JackSnapshot as dsl::FromValue>::from_value(pack::json_to_dsl_value(&parsed)).map_err(|error| error.to_string())
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
