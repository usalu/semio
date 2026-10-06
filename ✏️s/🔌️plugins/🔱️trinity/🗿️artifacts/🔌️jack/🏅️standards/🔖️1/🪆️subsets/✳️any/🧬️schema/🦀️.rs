//! 🧬️ Jack artifact schema — every field of the artifact with its state class.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`: `nodes`/`edges` replaced by a single
//! composed `content: JackContentChild` slot, matching `DagArtifact`'s own field swap exactly.

use crate::JackContentChild;
use ::semio_framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Jack document state owned by the artifact.
#[derive(Clone, Debug, PartialEq, ArtifactSchema)]
#[artifact_schema(id = "s.trinity.jack")]
pub struct JackArtifact {
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
//#endregion 🔖️Artifact

//#region 🔖️ValueCodec
/// 🔀️ Hand-written, not derived: `content` is a `store::ArtifactChild<S>` composed-artifact
/// handle, which speaks `serde` (framework-internal, unaffected by this ticket) rather than
/// `ToValue`/`FromValue` directly — bridged through the pre-existing `semio_framework_value::ToValue::to_value`/
/// `semio_framework_value::FromValue::from_value` seam instead of widening the derive macro to understand child-slot
/// handles. See `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/
/// 🔍️research/📓️serde-fanout-playbook.md`'s "composed/child-slot fields" trap.
impl semio_framework_value::ToValue for JackArtifact {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::object([
            ("schema".to_string(), semio_framework_value::ToValue::to_value(&self.schema)),
            ("name".to_string(), semio_framework_value::ToValue::to_value(&self.name)),
            ("manifestId".to_string(), semio_framework_value::ToValue::to_value(&self.manifest_id)),
            ("manifest".to_string(), semio_framework_value::ToValue::to_value(&self.manifest)),
            ("camera".to_string(), semio_framework_value::ToValue::to_value(&self.camera)),
            ("content".to_string(), semio_framework_value::ToValue::to_value(&self.content)),
            ("rootNodeId".to_string(), semio_framework_value::ToValue::to_value(&self.root_node_id)),
            ("query".to_string(), semio_framework_value::ToValue::to_value(&self.query)),
        ])
    }
}
impl semio_framework_value::FromValue for JackArtifact {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let entries = value.into_object()?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let field = |key: &str| get(key).ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("missing field `{key}`")));
        Ok(Self {
            schema: semio_framework_value::FromValue::from_value(field("schema")?)?,
            name: semio_framework_value::FromValue::from_value(field("name")?)?,
            manifest_id: semio_framework_value::FromValue::from_value(field("manifestId")?)?,
            manifest: semio_framework_value::FromValue::from_value(field("manifest")?)?,
            camera: semio_framework_value::FromValue::from_value(field("camera")?)?,
            content: semio_framework_value::FromValue::from_value(field("content")?)?,
            root_node_id: semio_framework_value::FromValue::from_value(field("rootNodeId")?)?,
            query: semio_framework_value::FromValue::from_value(field("query")?)?,
        })
    }
}
//#endregion 🔖️ValueCodec

//#region 🔖️Conversions
impl Default for JackArtifact {
    fn default() -> Self {
        Self { schema: crate::TRINITY_GRAPH_SCHEMA.into(), name: String::new(), manifest_id: None, manifest: Manifest::default(), camera: Camera::default(), content: crate::jack_content_child_with_owner(Vec::new(), Vec::new()), root_node_id: None, query: crate::TRINITY_JACK_DEFAULT_QUERY.into() }
    }
}

impl JackArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::JackSnapshot {
        crate::JackSnapshot {
            schema: self.schema.clone(),
            name: self.name.clone(),
            manifest_id: self.manifest_id.clone(),
            manifest: self.manifest.clone(),
            camera: self.camera.clone(),
            content: self.content.clone(),
            root_node_id: self.root_node_id.clone(),
            query: self.query.clone(),
        }
    }

    /// 🧬️ Builds the artifact from its document snapshot.
    pub fn from_snapshot(snapshot: crate::JackSnapshot) -> Self {
        Self { schema: snapshot.schema, name: snapshot.name, manifest_id: snapshot.manifest_id, manifest: snapshot.manifest, camera: snapshot.camera, content: snapshot.content, root_node_id: snapshot.root_node_id, query: snapshot.query }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::JackSnapshot) {
        self.schema = snapshot.schema;
        self.name = snapshot.name;
        self.manifest_id = snapshot.manifest_id;
        self.manifest = snapshot.manifest;
        self.camera = snapshot.camera;
        self.content = snapshot.content;
        self.root_node_id = snapshot.root_node_id;
        self.query = snapshot.query;
    }

    /// 🔎 Live node list, read through the working-scene cache.
    pub fn nodes(&self)->Result<Vec<crate::Node>,semio_framework_value::ValueError>{
        Ok(crate::jack_working_scene_for_handle(&self.content)?.nodes)
    }

    /// 🔎 Live edge list, read through the working-scene cache.
    pub fn edges(&self)->Result<Vec<crate::Edge>,semio_framework_value::ValueError>{
        Ok(crate::jack_working_scene_for_handle(&self.content)?.edges)
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.trinity.jack` — twenty handcrafted schema leaves.
pub fn jack_artifact_schema_descriptor() -> ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
    ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.trinity.jack",
        artifact: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto")
        },
        snapshot: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

//#region 🔖️EmptyDocument
/// 📦️ An empty trinity graph fixture — the app's zero-state initial document.
pub fn empty_jack_document() -> crate::JackSnapshot {
    crate::empty_trinity_graph_snapshot()
}
//#endregion 🔖️EmptyDocument

//#region 🧪️EmptyDocumentTests
#[cfg(test)]
#[path = "🧪️tests/🔬️empty-document/🦀️.rs"]
mod empty_document_tests;
//#endregion 🧪️EmptyDocumentTests

//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::Camera;
pub use crate::Manifest;
//#endregion 🔁️Re-exports

#[path="♻️retirement/🦀️.rs"]
mod retirement;
