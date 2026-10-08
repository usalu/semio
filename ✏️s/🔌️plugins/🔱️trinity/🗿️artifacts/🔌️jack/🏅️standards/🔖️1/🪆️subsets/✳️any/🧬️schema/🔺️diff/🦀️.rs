//! 🧬️ Jack diff schema — sparse field delta over the artifact.
//!
//! `content: Option<JackContentChild>` is the always-present composed child slot; scene edits never pass through it, they
//! are child-lane leaves of the shared graph vocabulary (design §20.15).

use crate::{Camera, JackContentChild};
use ::semio_framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the jack artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.trinity.jack")]
pub struct JackDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub name: Option<String>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub manifest_id: Option<Option<String>>,
    #[state(artifact)]
    pub manifest: Option<crate::Manifest>,
    #[state(artifact)]
    pub camera: Option<Camera>,
    #[state(artifact)]
    pub content: Option<JackContentChild>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub root_node_id: Option<Option<String>>,
    #[state(artifact)]
    pub query: Option<String>,
}
//#endregion 🔖️Diff

/// 🕳️ Tri-state decode of every `Option<Option<T>>` slot: a missing key is the unchanged slot (`None`) and a PRESENT `null`
/// is the clear `Some(None)`, never the unchanged slot the blanket `Option<T>` decode would fold it into.
fn deserialize_double_option<T: semio_framework_value::FromValue>(value: semio_framework_value::DslValue) -> Result<Option<Option<T>>, semio_framework_value::ValueError> {
    <Option<T> as semio_framework_value::FromValue>::from_value(value).map(Some)
}

use crate::JackSnapshot;
use protocol::{DiffAlgebra, MutationDiff};

impl MutationDiff<JackSnapshot> for JackDiff {
    fn apply(&self, snapshot: &JackSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<JackSnapshot> {
        let mut next = snapshot.clone();
        if let Some(value) = &self.schema {
            next.schema = value.clone();
        }
        if let Some(value) = &self.name {
            next.name = value.clone();
        }
        if let Some(value) = &self.manifest_id {
            next.manifest_id = value.clone();
        }
        if let Some(value) = &self.manifest {
            next.manifest = value.clone();
        }
        if let Some(value) = &self.camera {
            next.camera = value.clone();
        }
        if let Some(content) = &self.content {
            next.content = content.clone();
        }
        if let Some(value) = &self.root_node_id {
            next.root_node_id = value.clone();
        }
        if let Some(value) = &self.query {
            next.query = value.clone();
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(schema);
        take!(name);
        take!(manifest_id);
        take!(manifest);
        take!(camera);
        take!(content);
        take!(root_node_id);
        take!(query);
    }
}

impl DiffAlgebra<JackSnapshot> for JackDiff {
    fn inverse(&self, base: &JackSnapshot) -> Self {
        Self {
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            name: self.name.as_ref().map(|_| base.name.clone()),
            manifest_id: self.manifest_id.as_ref().map(|_| base.manifest_id.clone()),
            manifest: self.manifest.as_ref().map(|_| base.manifest.clone()),
            camera: self.camera.as_ref().map(|_| base.camera.clone()),
            content: self.content.as_ref().map(|_| base.content.clone()),
            root_node_id: self.root_node_id.as_ref().map(|_| base.root_node_id.clone()),
            query: self.query.as_ref().map(|_| base.query.clone()),
        }
    }
    fn between(base: &JackSnapshot, other: &JackSnapshot) -> Self {
        Self {
            schema: (base.schema != other.schema).then(|| other.schema.clone()),
            name: (base.name != other.name).then(|| other.name.clone()),
            manifest_id: (base.manifest_id != other.manifest_id).then(|| other.manifest_id.clone()),
            manifest: (base.manifest != other.manifest).then(|| other.manifest.clone()),
            camera: (base.camera != other.camera).then(|| other.camera.clone()),
            content: (base.content != other.content).then(|| other.content.clone()),
            root_node_id: (base.root_node_id != other.root_node_id).then(|| other.root_node_id.clone()),
            query: (base.query != other.query).then(|| other.query.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.schema.is_none() && self.name.is_none() && self.manifest_id.is_none() && self.manifest.is_none() && self.camera.is_none() && self.content.is_none() && self.root_node_id.is_none() && self.query.is_none()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
