//! 👁️ Note mutation — `ChangeGridVisible`: sets grid visibility.

use crate::{NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 👁️ `change-grid-visible` payload — sets grid visibility.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-grid-visible")]
pub struct ChangeGridVisible {
    pub new_visible: Option<bool>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_grid_visible<Op>(new_visible: Option<bool>) -> Op
where
    Op: From<crate::standards::v1::subsets::canvas::schema::mutations::change_grid_visible::ChangeGridVisible>,
{
    Op::from(ChangeGridVisible { new_visible })
}

impl<Op> MutationKind<NoteSnapshot, Op> for ChangeGridVisible
where
    Op: protocol::Mutation<NoteSnapshot, Diff = NoteDiff> + From<ChangeGridVisible>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "grid-visible", kind: "change-grid-visible", record: "ChangedGridVisible" };

    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        match self.new_visible {
            Some(true) => semio_framework_ui_locale::LocalizedLabel::native("Show grid", "Raster einblenden"),
            Some(false) => semio_framework_ui_locale::LocalizedLabel::native("Hide grid", "Raster ausblenden"),
            None => crate::schema::note_setting_label(("grid visibility", "Rastersichtbarkeit"), None),
        }
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Mutation
