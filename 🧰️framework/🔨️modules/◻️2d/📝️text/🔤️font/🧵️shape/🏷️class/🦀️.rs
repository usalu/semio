//! 🏷️ Genuine original GDEF glyph classes use bounded ordered source probes.
use super::super::{FontReader,FontFace,kerning::Index};
#[derive(Clone,Copy)]
pub struct FontGlyphClassCursor{face:FontFace,glyph:u16,started:bool,probe:Option<Index>,output:Option<u16>}
semio_framework_value::artifact_retire_leaf!(FontGlyphClassCursor);
impl FontGlyphClassCursor{
 pub fn new(face:FontFace,glyph:u16)->Result<Self,String>{if glyph>=face.glyphs{return Err("Invalid original font glyph class".into());}Ok(Self{face,glyph,started:false,probe:None,output:None})}
 pub fn step(&mut self,bytes:&[u8])->Result<bool,String>{if bytes.len()!=self.face.byte_length{return Err("Original font glyph class source changed".into());}if self.output.is_some(){return Ok(true);}let Some(table)=self.face.gdef else{self.output=Some(0);return Ok(true);};let r=FontReader::new(bytes,table)?;if !self.started{if r.u16(table.offset)?!=1{return Err("Unsupported font class version".into());}self.started=true;let offset=usize::from(r.u16(table.offset+4)?);if offset==0{self.output=Some(0);return Ok(true);}self.probe=Some(Index::new(r,table.offset+offset,self.glyph,true)?);return Ok(false);}if self.probe.as_mut().unwrap().step(r)?{let value=self.probe.as_ref().unwrap().result();if !(0..=4).contains(&value){return Err("Font glyph class exceeds authority".into());}self.output=Some(value as u16);return Ok(true);}Ok(false)}
 pub fn result(&self)->Result<u16,String>{self.output.ok_or_else(||"Font glyph class incomplete".into())}
}
