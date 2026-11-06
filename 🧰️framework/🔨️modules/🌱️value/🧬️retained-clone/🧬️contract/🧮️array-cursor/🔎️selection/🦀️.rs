//! 🔎️ Source-free array prefix selection bounds payload, initialized-prefix metadata and actual copy work.
use super::{ValueError,RETAINED_CLONE_BULK_PAGE_BYTES,RETAINED_CLONE_BULK_PAGE_ITEMS};
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct ArrayPrefixShape{pub length:usize,pub index:usize,pub element_bytes:usize,pub index_bytes:usize,pub binding_work:u64}
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct ArrayPrefixSelection{pub elements:usize,pub copy_bytes:usize,pub work:u64}
/// 🔭️ Chooses only an admitted prefix; insufficient body or work never forces one element.
pub fn select_array_prefix(shape:ArrayPrefixShape,body:usize,maximum_work_units:u64)->Result<ArrayPrefixSelection,ValueError>{
 let remaining=shape.length.checked_sub(shape.index).ok_or_else(||ValueError::invariant("array prefix exceeds its declared length"))?;
 let fixed=shape.binding_work.checked_add(1).ok_or_else(||ValueError::invariant("array prefix fixed work overflow"))?;
 if remaining==0||shape.element_bytes>RETAINED_CLONE_BULK_PAGE_BYTES||maximum_work_units<=fixed{return Ok(Default::default())}
 let width=u64::try_from(shape.element_bytes).map_err(|_|ValueError::invariant("array element work cannot be represented"))?;
 let per_element=width.checked_add(2).ok_or_else(||ValueError::invariant("array element work overflow"))?;
 let by_work=((maximum_work_units-fixed)/per_element).min(RETAINED_CLONE_BULK_PAGE_ITEMS as u64)as usize;
 let by_copy=if shape.element_bytes==0{RETAINED_CLONE_BULK_PAGE_ITEMS}else{body.saturating_sub(shape.index_bytes).min(RETAINED_CLONE_BULK_PAGE_BYTES)/shape.element_bytes};
 let elements=remaining.min(RETAINED_CLONE_BULK_PAGE_ITEMS).min(by_work).min(by_copy);
 if elements==0{return Ok(Default::default())}
 let copy_bytes=elements.checked_mul(shape.element_bytes).and_then(|bytes|bytes.checked_add(shape.index_bytes)).ok_or_else(||ValueError::invariant("array prefix copy extent overflow"))?;
 let work=fixed.checked_add(per_element.checked_mul(elements as u64).ok_or_else(||ValueError::invariant("array prefix copy work overflow"))?).ok_or_else(||ValueError::invariant("array prefix total work overflow"))?;
 Ok(ArrayPrefixSelection{elements,copy_bytes,work})
}
