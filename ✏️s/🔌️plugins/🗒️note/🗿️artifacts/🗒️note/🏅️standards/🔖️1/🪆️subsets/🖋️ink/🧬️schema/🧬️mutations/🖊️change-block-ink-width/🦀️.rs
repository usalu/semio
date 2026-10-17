//! 🖊️ Note mutation — `ChangeBlockInkWidth`: sets an ink block's stroke width.

use crate::{NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🖊️ `change-block-ink-width` payload — sets an ink block's stroke width.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-block-ink-width")]
pub struct ChangeBlockInkWidth {
    pub id: String,
    pub new_stroke_width: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_block_ink_width<Op>(id: String, new_stroke_width: f64) -> Op
where
    Op: From<crate::standards::v1::subsets::ink::schema::mutations::change_block_ink_width::ChangeBlockInkWidth>,
{
    Op::from(ChangeBlockInkWidth { id, new_stroke_width })
}

impl<Op> MutationKind<NoteSnapshot, Op> for ChangeBlockInkWidth
where
    Op: protocol::Mutation<NoteSnapshot, Diff = NoteDiff> + From<ChangeBlockInkWidth>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "block-ink-width", kind: "change-block-ink-width", record: "ChangedBlockInkWidth" };

    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change block \"{}\" ink width to {}", self.id, self.new_stroke_width), &format!("Strichbreite von Block \"{}\" auf {} ändern", self.id, self.new_stroke_width))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
