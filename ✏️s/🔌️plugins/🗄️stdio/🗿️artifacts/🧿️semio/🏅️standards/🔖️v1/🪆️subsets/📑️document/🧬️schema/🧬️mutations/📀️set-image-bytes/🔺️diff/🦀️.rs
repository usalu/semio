//! 🔺️ Diff for `SetImageBytes`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetImageBytes, base: &SemioDocumentSnapshot) -> protocol::MutationOutcome<SemioDocumentDiff> {
    if image_at(base, &payload.id).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Image \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let super::SetImageBytes { id, mime, bytes } = payload;
    protocol::MutationOutcome::new(match image_at(base, id) {
        Some(old) if &old.mime != mime || &old.bytes != bytes => SemioDocumentDiff {
            styles: None,
            images: Some(crate::standards::v1::subsets::document::schema::diff::ImagesDiff {
                modified: vec![crate::standards::v1::subsets::base::schema::triples::NamedModified {
                    key: id.clone(),
                    diff: crate::standards::v1::subsets::document::schema::diff::DocImageDiff { mime: Some(mime.clone()), bytes: Some(bytes.clone()) },
                }],
                ..Default::default()
            }),
            blocks: None,
        },
        _ => SemioDocumentDiff::default(),
    })
}
//#endregion 🔖️Diff
