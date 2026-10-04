//! 🔺️ Playbook artifact — sparse field-delta diff codec and apply/absorb.

use crate::schema::diff::PlaybookDiff;
use crate::schema::snapshot::PlaybookSnapshot;
use crate::schema::PlaybookArtifact;
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️Apply
impl PlaybookDiff {
    /// 🧬️ Applies sparse document changes to the artifact.
    pub fn apply_to_artifact(&self, artifact: &PlaybookArtifact) -> protocol::MutationApplyResult<PlaybookArtifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(id) = &self.id {
                next.id = id.clone();
            }
            if let Some(version) = &self.version {
                next.version = version.clone();
            }
            if let Some(title) = &self.title {
                next.title = title.clone();
            }
            if let Some(flow) = &self.flow {
                next.flow = flow.clone();
            }
            next
        })
    }
}

impl MutationDiff<PlaybookSnapshot> for PlaybookDiff {
    fn apply(&self, snapshot: &PlaybookSnapshot) -> protocol::MutationApplyResult<PlaybookSnapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(id) = &self.id {
                next.id = id.clone();
            }
            if let Some(version) = &self.version {
                next.version = version.clone();
            }
            if let Some(title) = &self.title {
                next.title = title.clone();
            }
            if let Some(flow) = &self.flow {
                next.flow = flow.clone();
            }
            next
        })
    }
    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(schema);
        take!(id);
        take!(version);
        take!(title);
        take!(flow);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Builders
/// 📸️ Whole-snapshot replacement diff.
pub fn diff_set_snapshot(snapshot: &PlaybookSnapshot) -> PlaybookDiff {
    PlaybookDiff { artifact: Some(Box::new(PlaybookArtifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}
//#endregion 🔖️Builders

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type PlaybookDiffText = String;
//#endregion 🚚️Carrier
