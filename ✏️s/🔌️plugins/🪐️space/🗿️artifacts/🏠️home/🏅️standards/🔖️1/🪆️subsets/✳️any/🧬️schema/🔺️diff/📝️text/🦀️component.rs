//! 🔺️ S Home launcher artifact — operation diff laws (constitutional: diff).

use crate::artifacts::home::schema::SHomeArtifact;
use crate::artifacts::home::SHomeSnapshot;
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::home::schema::diff::*;

//#region 🔖️Apply
impl SHomeDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub async fn apply_to_artifact(&self, artifact: &SHomeArtifact) -> protocol::MutationApplyResult<SHomeArtifact> {
        Ok({
            let mut next = artifact.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(value) = self.catalog_generation {
                next.catalog_generation = value;
            }
            if let Some(tab) = &self.active_panel_tab {
                next.active_panel_tab = tab.clone();
            }
            if let Some(locale) = &self.locale {
                next.locale = locale.clone();
            }
            next
        })
    }
}

impl MutationDiff<SHomeSnapshot> for SHomeDiff {
    async fn apply(&self, snapshot: &SHomeSnapshot) -> protocol::MutationApplyResult<SHomeSnapshot> {
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
    async fn absorb(&mut self, other: Self) {
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(schema);
        take!(catalog_generation);
        take!(active_panel_tab);
        take!(locale);
    }
}
//#endregion 🔖️Apply
