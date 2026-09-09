//! 🔺️ S Home launcher artifact — operation diff laws (constitutional: diff).

use crate::standards::v1::subsets::any::schema::SHomeArtifact;
use crate::SHomeSnapshot;
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::standards::v1::subsets::any::schema::diff::*;

//#region 🔖️Apply
impl SHomeDiff {
    /// 🧬️ Applies sparse document changes to the artifact.
    pub fn apply_to_artifact(&self, artifact: &SHomeArtifact) -> protocol::MutationApplyResult<SHomeArtifact> {
        Ok({
            let mut next = artifact.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(value) = self.catalog_generation {
                next.catalog_generation = value;
            }
            next
        })
    }
}

impl MutationDiff<SHomeSnapshot> for SHomeDiff {
    fn apply(&self, snapshot: &SHomeSnapshot) -> protocol::MutationApplyResult<SHomeSnapshot> {
        Ok({
            let mut next = snapshot.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(value) = self.catalog_generation {
                next.catalog_generation = value;
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
        take!(catalog_generation);
    }
}
//#endregion 🔖️Apply

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type SHomeDiffText = String;
//#endregion 🚚️Carrier
