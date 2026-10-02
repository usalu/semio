//! 👁️ Note mutation — `ChangeGridVisible`: sets grid visibility.

use crate::schema::mutations::NoteMutation;
use crate::{NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 👁️ `change-grid-visible` payload — sets grid visibility.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-grid-visible")]
pub struct ChangeGridVisible {
    pub new_visible: Option<bool>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_grid_visible(new_visible: Option<bool>) -> NoteMutation {
    NoteMutation::ChangeGridVisible(ChangeGridVisible { new_visible })
}

impl MutationKind<NoteSnapshot, NoteMutation> for ChangeGridVisible {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "grid-visible", kind: "change-grid-visible", record: "ChangedGridVisible" };

    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        match self.new_visible {
            Some(true) => semio_framework_ui_locale::LocalizedLabel::native("Show grid", "Raster einblenden"),
            Some(false) => semio_framework_ui_locale::LocalizedLabel::native("Hide grid", "Raster ausblenden"),
            None => crate::schema::mutations::note_setting_label(("grid visibility", "Rastersichtbarkeit"), None),
        }
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Mutation
