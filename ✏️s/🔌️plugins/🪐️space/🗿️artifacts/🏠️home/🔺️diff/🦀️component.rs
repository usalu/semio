//! 🔺️ S Home launcher artifact — operation diff laws (constitutional: diff).

use crate::artifacts::home::schema::SHomeArtifact;
use crate::artifacts::home::SHomeSnapshot;
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

pub use super::schema::*;

//#region 🔖️Apply
impl SHomeDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &SHomeArtifact) -> SHomeArtifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
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
    }
}

impl MutationDiff<SHomeSnapshot> for SHomeDiff {
    fn apply(&self, snapshot: &SHomeSnapshot) -> SHomeSnapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(value) = self.catalog_generation {
            next.catalog_generation = value;
        }
        next
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
        take!(catalog_generation);
        take!(active_panel_tab);
        take!(locale);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
/// 🖼️ Whole-snapshot replacement diff.
pub fn diff_set_snapshot(snapshot: &SHomeSnapshot) -> SHomeDiff {
    SHomeDiff {
        artifact: Some(Box::new(SHomeArtifact::from_snapshot(snapshot.clone()))),
        ..Default::default()
    }
}
//#endregion 🔖️Helpers
