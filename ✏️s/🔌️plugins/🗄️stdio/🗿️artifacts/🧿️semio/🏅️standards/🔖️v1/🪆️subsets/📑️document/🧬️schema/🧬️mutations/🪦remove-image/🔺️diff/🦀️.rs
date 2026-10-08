//! 🔺️ Diff for `RemoveImage`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveImage, base: &SemioDocumentSnapshot) -> protocol::MutationOutcome<SemioDocumentDiff> {
    if image_at(base, &payload.id).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Image \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let super::RemoveImage { id } = payload;
    protocol::MutationOutcome::new({
        SemioDocumentDiff { styles: None, images: Some(crate::standards::v1::subsets::document::schema::diff::ImagesDiff { removed: vec![id.clone()], ..Default::default() }), blocks: None }
    })
}
//#endregion 🔖️Diff
