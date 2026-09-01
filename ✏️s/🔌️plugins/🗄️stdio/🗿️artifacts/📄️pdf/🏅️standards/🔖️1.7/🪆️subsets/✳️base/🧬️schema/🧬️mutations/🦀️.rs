//! 🧬️ Transparent PDF 1.7/Any mutation dispatch. Every concrete payload, diff, inverse, codec,
//! schema, and test is owned by its direct semantic folder.

use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{diff::PdfDiff, snapshot::PdfSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Leaves
#[path = "➕️append-page-content/🦀️.rs"]
pub mod append_page_content;
#[path = "📦️insert-object/🦀️.rs"]
pub mod insert_object;
#[path = "📥️insert-page/🦀️.rs"]
pub mod insert_page;
#[path = "🔀️move-page/🦀️.rs"]
pub mod move_page;
#[path = "🚫️remove-dict-entry/🦀️.rs"]
pub mod remove_dict_entry;
#[path = "🧹️remove-object/🦀️.rs"]
pub mod remove_object;
#[path = "🗑️remove-page/🦀️.rs"]
pub mod remove_page;
#[path = "🧽️remove-trailer-entry/🦀️.rs"]
pub mod remove_trailer_entry;
#[path = "🔑️set-dict-entry/🦀️.rs"]
pub mod set_dict_entry;
#[path = "ℹ️set-info/🦀️.rs"]
pub mod set_info;
#[path = "🔧️set-object-value/🦀️.rs"]
pub mod set_object_value;
#[path = "✏️set-page-content/🦀️.rs"]
pub mod set_page_content;
#[path = "✂️set-page-crop-box/🦀️.rs"]
pub mod set_page_crop_box;
#[path = "📐️set-page-media-box/🦀️.rs"]
pub mod set_page_media_box;
#[path = "🔄️set-page-rotation/🦀️.rs"]
pub mod set_page_rotation;
#[path = "🧳️set-trailer-entry/🦀️.rs"]
pub mod set_trailer_entry;

pub use append_page_content::AppendPageContent;
pub use insert_object::InsertObject;
pub use insert_page::InsertPage;
pub use move_page::MovePage;
pub use remove_dict_entry::RemoveDictEntry;
pub use remove_object::RemoveObject;
pub use remove_page::RemovePage;
pub use remove_trailer_entry::RemoveTrailerEntry;
pub use set_dict_entry::SetDictEntry;
pub use set_info::SetInfo;
pub use set_object_value::SetObjectValue;
pub use set_page_content::SetPageContent;
pub use set_page_crop_box::SetPageCropBox;
pub use set_page_media_box::SetPageMediaBox;
pub use set_page_rotation::SetPageRotation;
pub use set_trailer_entry::SetTrailerEntry;
//#endregion 🔖️Leaves

//#region 🔖️Aggregate
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = PdfSnapshot, diff = PdfDiff, schema = "s.stdio.pdf.1.7")]
pub enum PdfMutation {
    InsertPage(InsertPage),
    RemovePage(RemovePage),
    SetPageMediaBox(SetPageMediaBox),
    SetPageCropBox(SetPageCropBox),
    AppendPageContent(AppendPageContent),
    SetInfo(SetInfo),
    InsertObject(InsertObject),
    RemoveObject(RemoveObject),
    SetObjectValue(SetObjectValue),
    SetDictEntry(SetDictEntry),
    RemoveDictEntry(RemoveDictEntry),
    SetTrailerEntry(SetTrailerEntry),
    RemoveTrailerEntry(RemoveTrailerEntry),
    MovePage(MovePage),
    SetPageContent(SetPageContent),
    SetPageRotation(SetPageRotation),
}
//#endregion 🔖️Aggregate

//#region 🔖️Delegation
/// ▶️ Applies one mutation through its leaf-owned diff.
pub fn apply_pdf_mutation(snapshot: &mut PdfSnapshot, mutation: &PdfMutation) -> protocol::MutationOutcome<PdfDiff> {
    use protocol::Mutation;
    let outcome = mutation.diff(snapshot);
    outcome.apply_to(snapshot)
}

/// ↩️ Delegates inverse planning to the authoritative leaf.
pub fn inverse_pdf_mutation(mutation: &PdfMutation, base: &PdfSnapshot) -> Vec<PdfMutation> {
    use protocol::Mutation;
    mutation.inverse(base)
}

/// 🧾️ Returns the derive-owned identity table in declaration and binary-tag order.
pub fn pdf_mutation_kinds() -> &'static [protocol::SemanticDescriptor] {
    use protocol::SemanticMutation;
    PdfMutation::kinds()
}
//#endregion 🔖️Delegation
