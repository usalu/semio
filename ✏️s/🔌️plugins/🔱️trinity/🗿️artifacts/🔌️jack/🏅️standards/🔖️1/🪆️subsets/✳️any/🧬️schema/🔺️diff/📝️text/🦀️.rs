//! 🔺️ Jack artifact — sparse field-delta diff codec and apply/absorb.
//!
//! `JackDiff.content` is the whole-handle slot of the composed `content` child; no parent leaf writes it, because scene
//! edits are child-lane leaves of `s.stdio.semio@v1/graph` (design §20.15).

use crate::standards::v1::subsets::any::schema::diff::JackDiff;
use crate::standards::v1::subsets::any::schema::JackArtifact;
use crate::JackSnapshot;
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️Apply
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
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion ️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type JackDiffText = String;
//#endregion 🚚️Carrier
