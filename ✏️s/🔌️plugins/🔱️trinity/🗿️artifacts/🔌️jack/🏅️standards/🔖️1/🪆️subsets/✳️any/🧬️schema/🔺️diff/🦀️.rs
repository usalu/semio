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
    pub manifest_id: Option<Option<String>>,
    #[state(artifact)]
    pub manifest: Option<crate::Manifest>,
    #[state(artifact)]
    pub camera: Option<Camera>,
    #[state(artifact)]
    pub content: Option<JackContentChild>,
    #[state(artifact)]
    pub root_node_id: Option<Option<String>>,
    #[state(artifact)]
    pub query: Option<String>,
}
//#endregion 🔖️Diff

use crate::standards::v1::subsets::any::schema::JackArtifact;
use crate::JackSnapshot;
use protocol::MutationDiff;

impl JackDiff {
    /// 🧬️ Applies document-owned sparse entries onto the artifact.
    pub fn apply_to_artifact(&self, artifact: &JackArtifact) -> protocol::MutationApplyResult<JackArtifact> {
        Ok({
            let mut next = artifact.clone();
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
            next
        })
    }
}

impl MutationDiff<JackSnapshot> for JackDiff {
    fn apply(&self, snapshot: &JackSnapshot) -> protocol::MutationApplyResult<JackSnapshot> {
        Ok({
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
            next
        })
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

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
