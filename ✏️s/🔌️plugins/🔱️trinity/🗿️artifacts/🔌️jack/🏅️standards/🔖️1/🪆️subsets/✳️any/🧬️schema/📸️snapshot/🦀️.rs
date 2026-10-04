//! 🧬️ Jack snapshot schema — artifact-lane fields only.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`: `nodes`/`edges` are gone from this STRUCT —
//! replaced by a single composed `content: JackContentChild` slot (`s.stdio.semio.graph`). See
//! `🗿️artifacts/🔌️jack/🦀️.rs`'s `🔖️ContentBridge`/`🔖️WorkingScene` regions for the
//! converter/handle/cache machinery this field depends on.

use crate::{Camera, JackContentChild, Manifest};
use ::semio_framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted trinity graph document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
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
    #[child(kind = "s.stdio.semio")]
    pub content: JackContentChild,
    #[state(artifact)]
    pub root_node_id: Option<String>,
    /// 🔎️ The document's Jack query — the query editor's text, document content like the graph it runs against.
    #[state(artifact)]
    pub query: String,
}
//#endregion 🔖️Snapshot

//#region 🔖️ValueCodec
/// 🔀️ Hand-written, not derived: `content` is a `store::ArtifactChild<S>` composed-artifact
/// handle — see `JackArtifact`'s identical trap in the sibling `🦀️.rs` (this struct's
/// non-child fields carried `#[serde(default, skip_serializing_if = "Option::is_none")]`
/// before this wave; `manifest_id`/`root_node_id` are `Option<String>`, and the blanket
/// `impl<T: FromValue> FromValue for Option<T>` already treats a missing key as `None` via the
/// derive macro's own generated `missing` arm being unreachable here since every field is
/// present below — no separate default handling needed in a hand-written impl).
impl semio_framework_value::ToValue for JackSnapshot {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_value::DslValue, semio_framework_value::ValueError> {
        c.scoped_stage(|c| {
            c.begin_stage(8)?;
            let mut fields = semio_framework_value::DslValue::object_encoding_controlled(8, c)?;
            macro_rules! field {
                ($key:literal,$value:expr) => {{
                    let child = semio_framework_value::ToValue::to_value_controlled($value, c)?;
                    semio_framework_value::DslValue::push_encoding_controlled(fields.get_mut(), $key, child, c)?;
                    c.step()?;
                }};
            }
            field!("schema", &self.schema);
            field!("name", &self.name);
            if let Some(value) = &self.manifest_id {
                field!("manifestId", value)
            } else {
                c.step()?;
            }
            field!("manifest", &self.manifest);
            field!("camera", &self.camera);
            field!("content", &self.content);
            if let Some(value) = &self.root_node_id {
                field!("rootNodeId", value)
            } else {
                c.step()?;
            }
            field!("query", &self.query);
            Ok(semio_framework_value::DslValue::Object(fields.take()))
        })
    }

    /// 🕳️ `manifestId`/`rootNodeId` are SKIPPED while `None` (the old `skip_serializing_if` law the
    /// committed `📸️snapshot` fixture vectors are written against): decode→encode of a committed
    /// snapshot is a fixed point only if an absent id stays absent instead of surfacing as `null`.
    fn to_value(&self) -> semio_framework_value::DslValue {
        let mut entries: Vec<(String, semio_framework_value::DslValue)> = Vec::with_capacity(8);
        entries.push(("schema".to_string(), semio_framework_value::ToValue::to_value(&self.schema)));
        entries.push(("name".to_string(), semio_framework_value::ToValue::to_value(&self.name)));
        if let Some(manifest_id) = self.manifest_id.as_ref() {
            entries.push(("manifestId".to_string(), semio_framework_value::ToValue::to_value(manifest_id)));
        }
        entries.push(("manifest".to_string(), semio_framework_value::ToValue::to_value(&self.manifest)));
        entries.push(("camera".to_string(), semio_framework_value::ToValue::to_value(&self.camera)));
        entries.push(("content".to_string(), semio_framework_value::ToValue::to_value(&self.content)));
        if let Some(root_node_id) = self.root_node_id.as_ref() {
            entries.push(("rootNodeId".to_string(), semio_framework_value::ToValue::to_value(root_node_id)));
        }
        entries.push(("query".to_string(), semio_framework_value::ToValue::to_value(&self.query)));
        semio_framework_value::DslValue::object(entries)
    }
}
impl semio_framework_value::FromValue for JackSnapshot {
    fn from_value_controlled(value: &semio_framework_value::DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, semio_framework_value::ValueError> {
        c.scoped_stage(|c| {
            c.begin_stage(8)?;
            let fields = value.object_controlled(c)?;
            c.charge(size_of::<Self>())?;
            fn optional_field<T: semio_framework_value::FromValue + Default>(fields: &[(String, semio_framework_value::DslValue)], key: &str, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<T, semio_framework_value::ValueError> {
                let value = match semio_framework_value::DslValue::field_controlled(fields, key, c)? {
                    Some(value) => T::from_value_controlled(value, c)?,
                    None => T::default(),
                };
                let value = semio_framework_value::DecodedValue::new(value, T::retire_decoded);
                c.step()?;
                Ok(value.take())
            }
            let schema = optional_field(fields, "schema", c)?;
            let name = optional_field(fields, "name", c)?;
            let manifest_id = optional_field(fields, "manifestId", c)?;
            let manifest = semio_framework_value::DecodedValue::new(optional_field::<Manifest>(fields, "manifest", c)?, <Manifest as semio_framework_value::FromValue>::retire_decoded);
            let camera = optional_field(fields, "camera", c)?;
            let child = semio_framework_value::DslValue::field_controlled(fields, "content", c)?.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing field content"))?;
            let content = semio_framework_value::DecodedValue::new(<JackContentChild as semio_framework_value::FromValue>::from_value_controlled(child, c)?, <JackContentChild as semio_framework_value::FromValue>::retire_decoded);
            c.step()?;
            let root_node_id = optional_field(fields, "rootNodeId", c)?;
            let query = semio_framework_value::DslValue::field_controlled(fields, "query", c)?.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing field query"))?;
            let query = <String as semio_framework_value::FromValue>::from_value_controlled(query, c)?;
            let output = semio_framework_value::DecodedValue::new(Self { schema, name, manifest_id, manifest: manifest.take(), camera, content: content.take(), root_node_id, query }, Self::retire_decoded);
            c.step()?;
            Ok(output.take())
        })
    }
    fn retire_decoded(self) {
        <Self as semio_framework_dsl_record::DslField>::retire_decoded(self)
    }

    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let entries = value.into_object()?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        Ok(Self {
            schema: match get("schema") {
                Some(v) => semio_framework_value::FromValue::from_value(v)?,
                None => Default::default(),
            },
            name: match get("name") {
                Some(v) => semio_framework_value::FromValue::from_value(v)?,
                None => Default::default(),
            },
            manifest_id: match get("manifestId") {
                Some(v) => semio_framework_value::FromValue::from_value(v)?,
                None => None,
            },
            manifest: match get("manifest") {
                Some(v) => semio_framework_value::FromValue::from_value(v)?,
                None => Default::default(),
            },
            camera: match get("camera") {
                Some(v) => semio_framework_value::FromValue::from_value(v)?,
                None => Default::default(),
            },
            content: semio_framework_value::FromValue::from_value(get("content").ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing field `content`"))?)?,
            root_node_id: match get("rootNodeId") {
                Some(v) => semio_framework_value::FromValue::from_value(v)?,
                None => None,
            },
            query: semio_framework_value::FromValue::from_value(get("query").ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing field `query`"))?)?,
        })
    }
}
//#endregion 🔖️ValueCodec

impl Default for JackSnapshot {
    fn default() -> Self {
        Self {
            schema: crate::TRINITY_GRAPH_SCHEMA.into(),
            name: String::new(),
            manifest_id: None,
            manifest: Manifest::default(),
            camera: Camera::default(),
            content: crate::jack_content_child_with_owner(Vec::new(), Vec::new()),
            root_node_id: None,
            query: crate::TRINITY_JACK_DEFAULT_QUERY.into(),
        }
    }
}

//#region 🌉️ExternalCodecBridge
/// 📤️ Renders a [`JackSnapshot`] as this facet's own camelCase JSON projection — the comparison
/// surface `🔌️mutate-jack-1`'s scenarios are measured through, and the shape the committed
/// `../🧫️fixtures/🧬️mutations/<slug>/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors are written in. It carries `content` as a HANDLE, never as a scene, and
/// that handle's `childId` is a digest of the child — so it moves if and only if the working scene
/// moved, which is what makes it a usable observability surface here.
///
/// A thin `pack::json` wrapper over [`JackSnapshot`]'s own `ToValue`, bridged through
/// `pack::json_from_dsl_value` since `DslValue` and `pack::json::Value` are sibling trees (used
/// behind this interface per CLAUDE.md's "external libraries behind an interface" rule).
pub fn encode_jack_snapshot_json(snapshot: &JackSnapshot) -> Result<String, semio_framework_value::ValueError> {
    let value = crate::standards::v1::subsets::any::io::json_native::convert(semio_framework_value::ToValue::to_value(snapshot), false)?;
    Ok(semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&value)))
}

/// 📥️ The inverse of [`encode_jack_snapshot_json`] — decodes those committed specification vectors
/// into real [`JackSnapshot`] values, so `🔌️mutate-jack-1`'s adapter reads the committed fixture
/// rather than re-declaring it as a Rust literal beside it.
pub fn decode_jack_snapshot_json(text: &str) -> Result<JackSnapshot, semio_framework_value::ValueError> {
    let parsed = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))?;
    <JackSnapshot as semio_framework_value::FromValue>::from_value(crate::standards::v1::subsets::any::io::json_native::convert(semio_framework_pack_json::to_dsl_value(&parsed), true)?)
}

/// 📝️ Parses the literal Jack parent and its independent content-child address.
/// Child materialization belongs to the host's composed artifact boundary.
pub fn parse_jack_dsl(text: &str) -> Result<JackSnapshot, String> {
    <JackSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 📝️ Renders the literal parent record with its native document preamble.
pub fn print_jack_dsl(snapshot: &JackSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

/// 🔎️ The scene's node names and `source -> target` edge ids the document's composed child currently
/// resolves to — the readable half of a divergence message, so a failing scenario names WHICH piece
/// moved rather than only that two content digests differ.
pub fn jack_scene_summary(snapshot: &JackSnapshot) -> Result<String, semio_framework_value::ValueError> {
    let scene = crate::jack_working_scene(snapshot)?;
    let nodes = scene.nodes.iter().map(|node| format!("{}({})", node.name, node.id)).collect::<Vec<_>>().join(" ");
    let edges = scene.edges.iter().map(|edge| edge.id.clone()).collect::<Vec<_>>().join(" ");
    Ok(format!("nodes[{nodes}] edges[{edges}]"))
}
//#endregion 🌉️ExternalCodecBridge

#[cfg(test)]
#[path = "🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_snapshot_tests;

impl semio_framework_schema_composition::ChildFieldRefs for JackSnapshot {
    const MANY: bool = false;
    fn visit_child_field<'a,V:semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self,slot:&'static str,visitor:&mut V)->Result<(),V::Error>{
        visitor.step()?;
        semio_framework_schema_composition::ChildFieldRefs::visit_child_field(&self.content,slot,visitor)
    }
}
