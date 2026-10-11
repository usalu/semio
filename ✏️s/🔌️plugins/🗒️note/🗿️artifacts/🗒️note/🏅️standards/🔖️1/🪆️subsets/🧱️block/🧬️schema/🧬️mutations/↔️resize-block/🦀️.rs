//! ↔️ Note mutation — `ResizeBlock`: changes a block's extent.

use crate::schema::mutations::NoteMutation;
use crate::{NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ↔️ `resize-block` payload — changes a block's extent.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "resize-block")]
pub struct ResizeBlock {
    pub id: String,
    pub new_width: f64,
    pub new_height: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn resize_block(id: String, new_width: f64, new_height: f64) -> NoteMutation {
    NoteMutation::ResizeBlock(ResizeBlock { id, new_width, new_height })
}

impl MutationKind<NoteSnapshot, NoteMutation> for ResizeBlock {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "resize", entity: "block", kind: "resize-block", record: "ResizedBlock" };

    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Resize block \"{}\"", self.id), &format!("Größe von Block \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
