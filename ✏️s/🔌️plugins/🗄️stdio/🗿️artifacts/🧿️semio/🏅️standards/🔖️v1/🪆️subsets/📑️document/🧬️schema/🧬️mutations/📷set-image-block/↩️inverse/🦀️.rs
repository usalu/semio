//! ↩️ Inverse for `SetImageBlock`.

#[allow(unused_imports)]
use crate::schema::snapshot::DocBlock;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioDocumentSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::block_at;
#[allow(unused_imports)]
use crate::schema::mutations::set_image_block;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetImageBlock, base: &SemioDocumentSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_image_block::SetImageBlock>,
{
    let super::SetImageBlock { path, .. } = payload;
    Ok(match block_at(base, path) {
        Some(DocBlock::Image { image_id, alt, width, height }) => {
            vec![Op::from(set_image_block::SetImageBlock { path: path.clone(), image_id: image_id.clone(), alt: alt.clone(), width: *width, height: *height })]
        }
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
