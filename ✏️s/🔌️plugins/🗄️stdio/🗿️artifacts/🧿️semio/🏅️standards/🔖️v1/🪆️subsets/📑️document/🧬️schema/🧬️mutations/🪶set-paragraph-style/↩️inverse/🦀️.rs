//! ↩️ Inverse for `SetParagraphStyle`.

#[allow(unused_imports)]
use crate::schema::snapshot::DocBlock;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioDocumentSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::block_at;
#[allow(unused_imports)]
use crate::schema::mutations::set_paragraph_style;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetParagraphStyle, base: &SemioDocumentSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_paragraph_style::SetParagraphStyle>,
{
    let super::SetParagraphStyle { path, .. } = payload;
    Ok(match block_at(base, path) {
        Some(DocBlock::Paragraph { style_id, .. }) => vec![Op::from(set_paragraph_style::SetParagraphStyle { path: path.clone(), style_id: style_id.clone() })],
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
