//! 🧾 Counts the slide, shape and word structure of authoritative OPC and retained XML.

use crate::PptxSnapshot;
use semio_framework_value::{ValueError,ValueRefusalKind};

/// 🧾 PresentationML document outline.
#[derive(Clone,Debug,Default,PartialEq,value_derive::ToValue,value_derive::FromValue)]
#[value(rename_all="camelCase")]
pub struct PptxOutline {pub slide_count:u32,pub shape_count:u32,pub word_count:u32}

impl PptxOutline {
 pub fn compute(snapshot:&PptxSnapshot)->Result<Self,ValueError>{
  let slides=crate::schema::mutations::xml_address::pptx_slides(snapshot).map_err(|message|ValueError::new(ValueRefusalKind::InvalidValue,message))?;
  let limit=||ValueError::new(ValueRefusalKind::WorkLimit,"PPTX outline count exceeds unsigned32");
  let slide_count=u32::try_from(slides.len()).map_err(|_|limit())?;
  let mut shape_count=0u32;let mut word_count=0u32;
  for slide in &slides{
   shape_count=shape_count.checked_add(u32::try_from(slide.shapes.len()).map_err(|_|limit())?).ok_or_else(limit)?;
   for text in slide.shapes.iter().filter_map(|shape|shape.text.as_deref()){
    word_count=word_count.checked_add(u32::try_from(text.split_whitespace().count()).map_err(|_|limit())?).ok_or_else(limit)?;
   }
  }
  Ok(Self{slide_count,shape_count,word_count})
 }
}

#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod tests;
