//! 🔺️ Diff for `InsertImage`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertImage, base: &SemioDocumentSnapshot) -> protocol::MutationOutcome<SemioDocumentDiff> {
    if image_at(base, &payload.image.id).is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Image \"{}\" already exists.", payload.image.id), [payload.image.id.clone()]);
    }
    let super::InsertImage { image, at } = payload;
    protocol::MutationOutcome::new({
        SemioDocumentDiff { styles: None, images: Some(crate::standards::v1::subsets::document::schema::diff::ImagesDiff { added: vec![crate::standards::v1::subsets::base::schema::triples::NamedAdded { index: at.map_or(base.images.len(), |at| at.min(base.images.len())), item: image.clone() }], ..Default::default() }), blocks: None }
    })
}
//#endregion 🔖️Diff
