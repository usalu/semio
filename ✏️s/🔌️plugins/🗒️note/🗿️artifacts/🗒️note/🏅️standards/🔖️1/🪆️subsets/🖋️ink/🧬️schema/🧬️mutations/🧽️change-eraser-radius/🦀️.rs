//! 🧽 Note mutation — `ChangeEraserRadius`: sets the eraser radius.

use crate::{NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🧽 `change-eraser-radius` payload — sets the eraser radius.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-eraser-radius")]
pub struct ChangeEraserRadius {
    pub new_radius: Option<f64>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_eraser_radius<Op>(new_radius: Option<f64>) -> Op
where
    Op: From<crate::standards::v1::subsets::ink::schema::mutations::change_eraser_radius::ChangeEraserRadius>,
{
    Op::from(ChangeEraserRadius { new_radius })
}

impl<Op> MutationKind<NoteSnapshot, Op> for ChangeEraserRadius
where
    Op: protocol::Mutation<NoteSnapshot, Diff = NoteDiff> + From<ChangeEraserRadius>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "eraser-radius", kind: "change-eraser-radius", record: "ChangedEraserRadius" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        crate::schema::note_setting_label(("eraser radius", "Radiererradius"), self.new_radius.map(crate::schema::note_label_number))
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<Op> protocol::ColdMutationKind<NoteSnapshot, Op> for ChangeEraserRadius
where
    Op: protocol::ColdMutation<NoteSnapshot, ReferenceDiff = NoteDiff> + From<ChangeEraserRadius>,
{
    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
