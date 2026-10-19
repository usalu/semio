//! 🧬️ Jack snapshot schema — artifact-lane fields only.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`: `nodes`/`edges` are gone from this STRUCT —
//! replaced by a single composed `content: JackContentChild` slot (the graph capability). See
//! `🗿️artifacts/🔌️jack/🦀️.rs`'s `🔖️ContentBridge`/`🔖️WorkingScene` regions for the
//! converter/handle/cache machinery this field depends on.

use crate::standards::v1::subsets::any::schema::content::{jack_working_scene_for_handle, JackContentChild, JackContentFactory, JackContentOwner, JackWorkingScene};
use crate::standards::v1::subsets::any::schema::errors::TrinityRamError;
use crate::standards::v1::subsets::any::schema::graph::{Camera, Edge, Node, TRINITY_GRAPH_SCHEMA, TRINITY_JACK_DEFAULT_QUERY};
use crate::standards::v1::subsets::any::schema::graph_manifest::manifest_by_id;
use crate::standards::v1::subsets::any::schema::manifest::Manifest;
use ::semio_framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted trinity graph document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.trinity.jack")]
pub struct JackSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub name: String,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub manifest_id: Option<String>,
    #[state(artifact)]
    pub manifest: Manifest,
    #[state(artifact)]
    pub camera: Camera,
    #[state(artifact)]
    #[child(capability = "s.trinity.jack.graph")]
    pub content: JackContentChild,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub root_node_id: Option<String>,
    /// 🔎️ The document's Jack query — the query editor's text, document content like the graph it runs against.
    #[state(artifact)]
    pub query: String,
}
//#endregion 🔖️Snapshot

//#region 🔖️Operations
impl JackSnapshot {
    pub const SCHEMA: &'static str = TRINITY_GRAPH_SCHEMA;

    pub fn validate_schema(&self) -> Result<(), TrinityRamError> {
        if self.schema != Self::SCHEMA {
            return Err(TrinityRamError::SchemaMismatch { expected: Self::SCHEMA, actual: self.schema.clone() });
        }
        Ok(())
    }

    /// 📤️ Encodes the actual Jack parent; composed graph content travels through its independent child owner.
    pub fn to_json(&self) -> Result<String, TrinityRamError> {
        let value = crate::standards::v1::subsets::any::schema::json_transport::convert(semio_framework_value::ToValue::to_value(self), false)?;
        Ok(semio_framework_pack_json::to_string_pretty(&semio_framework_pack_json::from_dsl_value(&value)))
    }

    pub fn resolve_manifest(&mut self) -> Result<(), TrinityRamError> {
        if let Some(id) = self.manifest_id.as_deref() {
            self.manifest = Manifest::from_graph(&manifest_by_id(id).ok_or_else(|| TrinityRamError::UnknownManifestId(id.to_string()))?);
            return Ok(());
        }
        if self.manifest.node_kinds.is_empty() && self.manifest.edge_kinds.is_empty() && self.manifest.port_kinds.is_empty() {
            return Err(TrinityRamError::ManifestMissing);
        }
        Ok(())
    }

    /// 📥️ Decodes the literal parent with its unresolved content address; the host supplies the exact child owner.
    pub fn from_json(json: &str) -> Result<Self, TrinityRamError> {
        let snapshot = crate::standards::v1::subsets::any::io::text::snapshot::decode_jack_snapshot_json(json)?;
        snapshot.validate_schema()?;
        Ok(snapshot)
    }

    /// 🏗️ Transfers one working scene into the snapshot's exact composed content owner. The query starts empty: a document
    /// constructor names its own (`Self { query, ..Self::with_content(..) }`).
    pub fn with_content<C:JackContentFactory>(schema: String, name: String, manifest_id: Option<String>, manifest: Manifest, camera: Camera, scene: JackWorkingScene, root_node_id: Option<String>) -> Self {
        Self { schema, name, manifest_id, manifest, camera, content: C::child_from_working(scene.nodes,scene.edges), root_node_id, query: String::new() }
    }

    /// 🔎 Live node list, read through the working-scene cache — replaces the old direct `.nodes`
    /// field access (see `🔖️WorkingScene`'s module doc for why this indirection exists).
    pub fn nodes(&self) -> Result<Vec<Node>, semio_framework_value::ValueError> {
        Ok(jack_working_scene(self)?.nodes)
    }

    /// 🔎 Live edge list, read through the working-scene cache.
    pub fn edges(&self) -> Result<Vec<Edge>, semio_framework_value::ValueError> {
        Ok(jack_working_scene(self)?.edges)
    }
}
//#endregion 🔖️Operations

//#region 🔖️ValueCodec
/// 🔀️ Hand-written, not derived: `content` is a `store::os_store::artifact_child::ArtifactChild<S>` composed-artifact
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
    fn from_value_controlled(value: &semio_framework_value::DslValue, c: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<Self, semio_framework_value::ValueError> {
        c.scoped_stage(|c| {
            c.begin_stage(8)?;
            let fields = value.object_controlled(c)?;
            c.charge(size_of::<Self>())?;
            fn optional_field<T: semio_framework_value::FromValue + Default>(fields: &[(String, semio_framework_value::DslValue)], key: &str, c: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<T, semio_framework_value::ValueError> {
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
            let camera=match semio_framework_value::DslValue::field_controlled(fields,"camera",c)?{Some(value)=>crate::standards::v1::subsets::any::schema::json_transport::decode_camera(value,c)?,None=>Default::default()};c.step()?;
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
            schema: TRINITY_GRAPH_SCHEMA.into(),
            name: String::new(),
            manifest_id: None,
            manifest: Manifest::default(),
            camera: Camera::default(),
            content: store::os_store::artifact_child::ArtifactChild::new(String::new(),semio_framework_artifact_reference::ArtifactRef{artifact_id:String::new(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:String::new(),standard:String::new(),subset:String::new()}}),
            root_node_id: None,
            query: TRINITY_JACK_DEFAULT_QUERY.into(),
        }
    }
}

//#region 🌉️ExternalCodecBridge








/// 🔎️ The scene's node names and `source -> target` edge ids the document's composed child currently
/// resolves to — the readable half of a divergence message, so a failing scenario names WHICH piece
/// moved rather than only that two content digests differ.
pub fn jack_scene_summary(snapshot: &JackSnapshot) -> Result<String, semio_framework_value::ValueError> {
    let scene = jack_working_scene(snapshot)?;
    let nodes = scene.nodes.iter().map(|node| format!("{}({})", node.name, node.id)).collect::<Vec<_>>().join(" ");
    let edges = scene.edges.iter().map(|edge| edge.id.clone()).collect::<Vec<_>>().join(" ");
    Ok(format!("nodes[{nodes}] edges[{edges}]"))
}
//#endregion 🌉️ExternalCodecBridge



impl semio_framework_schema_composition::ChildFieldRefs for JackSnapshot {
    const MANY: bool = false;
    fn visit_child_field<'a,V:semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self,slot:&'static str,visitor:&mut V)->Result<(),V::Error>{
        visitor.step()?;
        semio_framework_schema_composition::ChildFieldRefs::visit_child_field(&self.content,slot,visitor)
    }
}

/// 🧱️ The working scene of the snapshot's composed content child.
pub fn jack_working_scene(snapshot:&JackSnapshot)->Result<JackWorkingScene,semio_framework_value::ValueError>{jack_working_scene_for_handle(&snapshot.content)}

/// 📦️ An empty trinity graph fixture over the installation's content factory.
pub fn empty_trinity_graph_snapshot<C:JackContentFactory>() -> JackSnapshot {
    JackSnapshot { query: TRINITY_JACK_DEFAULT_QUERY.into(), ..JackSnapshot::with_content::<C>(JackSnapshot::SCHEMA.into(), "trinity".into(), Some("nakagin".into()), Manifest::nakagin_default(), Camera::default(), JackWorkingScene { nodes: Vec::new(), edges: Vec::new() }, None) }
}

//#region 🧬️ChildRestoreProjection
/// 🧬️ The loaded-parent child projection, read off the snapshot's own derived composition fields — the ONE
/// definition the editor and the viewer both declare. The `ArtifactEditor`/`ArtifactViewer` trait default
/// refuses it (`… did not declare a loaded-parent child projection`), and since PX1 the live envelope load
/// asks for it before the decoded document may replace the store, so an undeclared app fails every live load.
pub fn jack_child_restore_projection(snapshot: &JackSnapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, semio_framework_plugin::Fault> {
    semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("jack.child-projection"), error.to_string()))
}
//#endregion 🧬️ChildRestoreProjection

//#region ♻️Retirement
/// 🔗️ Retires the metadata fields, the child handle and the shared local content owner as separate deferred fields.
impl semio_framework_value::retirement::RetireOwned for JackSnapshot {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        let Self { schema, name, manifest_id, manifest, camera, mut content, root_node_id, query } = self;
        let owner = content.take_local_owner::<JackContentOwner>().ok().flatten();
        semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(schema), semio_framework_value::retirement::deferred(name), semio_framework_value::retirement::deferred(manifest_id), semio_framework_value::retirement::deferred(manifest), semio_framework_value::retirement::deferred(camera), semio_framework_value::retirement::deferred(content), semio_framework_value::retirement::deferred(owner), semio_framework_value::retirement::deferred(root_node_id), semio_framework_value::retirement::deferred(query)])
    }
    fn retirement_birth_bytes(&self) -> Option<usize> {
        semio_framework_value::retirement::sequence_birth_bytes(&[
            semio_framework_value::retirement::deferred_birth_bytes_for(&self.schema),
            semio_framework_value::retirement::deferred_birth_bytes_for(&self.name),
            semio_framework_value::retirement::deferred_birth_bytes_for(&self.manifest_id),
            semio_framework_value::retirement::deferred_birth_bytes_for(&self.manifest),
            semio_framework_value::retirement::deferred_birth_bytes_for(&self.camera),
            semio_framework_value::retirement::deferred_birth_bytes_for(&self.content),
            semio_framework_value::retirement::deferred_birth_bytes::<Option<std::sync::Arc<JackContentOwner>>>(),
            semio_framework_value::retirement::deferred_birth_bytes_for(&self.root_node_id),
            semio_framework_value::retirement::deferred_birth_bytes_for(&self.query),
        ])
    }
    fn controlled_retirement_supported() -> bool {
        true
    }
}
//#endregion ♻️Retirement
