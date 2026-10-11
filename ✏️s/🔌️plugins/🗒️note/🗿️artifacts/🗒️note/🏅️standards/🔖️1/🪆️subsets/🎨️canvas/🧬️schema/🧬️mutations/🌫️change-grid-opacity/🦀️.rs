//! 🌫️ Note mutation — `ChangeGridOpacity`: sets grid opacity.

use crate::schema::mutations::NoteMutation;
use crate::{NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌫️ `change-grid-opacity` payload — sets grid opacity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-grid-opacity")]
pub struct ChangeGridOpacity {
    pub new_opacity: Option<f64>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_grid_opacity(new_opacity: Option<f64>) -> NoteMutation {
    NoteMutation::ChangeGridOpacity(ChangeGridOpacity { new_opacity })
}

impl MutationKind<NoteSnapshot, NoteMutation> for ChangeGridOpacity {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "grid-opacity", kind: "change-grid-opacity", record: "ChangedGridOpacity" };

    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        crate::schema::mutations::note_setting_label(("grid opacity", "Rasterdeckkraft"), self.new_opacity.map(|opacity| { let (en, de) = crate::schema::mutations::note_label_number(opacity * 100.0); (format!("{en}%"), format!("{de} %")) }))
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Mutation
