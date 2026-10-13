//! 🔺️ Diff for `InsertStyle`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertStyle, base: &SemioDocumentSnapshot) -> protocol::MutationOutcome<SemioDocumentDiff> {
    if style_at(base, &payload.style.id).is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Style \"{}\" already exists.", payload.style.id), [payload.style.id.clone()]);
    }
    let super::InsertStyle { style, at } = payload;
    protocol::MutationOutcome::new({
        SemioDocumentDiff { styles: Some(crate::schema::diff::StylesDiff { added: vec![semio_s_artifact_stdio_semio_base::schema::triples::NamedAdded { index: at.map_or(base.styles.len(), |at| at.min(base.styles.len())), item: style.clone() }], ..Default::default() }), images: None, blocks: None }
    })
}
//#endregion 🔖️Diff
