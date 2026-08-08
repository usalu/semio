//! Draw mutation — `SetDocument` payload + builder + apply.
use crate::artifacts::draw::mutations::{apply_draw_edit_mutation, DrawMutation};
use crate::artifacts::draw::DrawDocument;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji `SetDocument` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDocument {
    pub document: DrawDocument,
}

pub fn set_document(document: DrawDocument) -> DrawMutation {
    DrawMutation::SetDocument { document }
}

pub fn apply(doc: &mut DrawDocument, document: &DrawDocument) {
    *doc = document.clone();
}
//#endregion 🔖️Mutation
