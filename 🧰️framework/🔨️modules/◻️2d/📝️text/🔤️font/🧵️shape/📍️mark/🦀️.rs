//! 📍️ Actual unhinted GPOS mark-to-base anchor authority advances under original font byte custody.
use super::FontLookup;
use super::super::{FontFace,FontProgress,FontReader,kerning::Index};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor};
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct FontMarkAttachment{pub x:i32,pub y:i32}
semio_framework_value::artifact_retire_leaf!(FontMarkAttachment);
pub struct FontMarkCursor{face:FontFace,lookup:FontLookup,base:u16,mark:u16,max_work:u64,phase:u8,sub:usize,at:usize,probe:Option<Index>,mark_index:usize,base_index:usize,mark_class:usize,mark_anchor:usize,base_anchor:usize,mark_x:i32,mark_y:i32,output:Option<FontMarkAttachment>,work:u64,cancelled:bool,failure:Option<String>}
impl RetireOwned for FontMarkCursor{fn retirement(self)->Box<dyn RetirementCursor>{self.failure.retirement()}fn retirement_birth_bytes(&self)->Option<usize>{self.failure.retirement_birth_bytes()}fn controlled_retirement_supported()->bool{true}}
impl FontMarkCursor{
 pub fn new(face:FontFace,lookup:FontLookup,base:u16,mark:u16,max_work:u64)->Self{let invalid=face.gpos.is_none()||lookup.kind!=4||lookup.flags!=0||base>=face.glyphs||mark>=face.glyphs||!(1..=1000000000).contains(&max_work);Self{face,lookup,base,mark,max_work,phase:0,sub:0,at:0,probe:None,mark_index:0,base_index:0,mark_class:0,mark_anchor:0,base_anchor:0,mark_x:0,mark_y:0,output:None,work:0,cancelled:false,failure:invalid.then(||"Invalid font mark authority".into())}}
 fn next_sub(&mut self){self.sub+=1;self.probe=None;self.phase=0;}
 fn anchor(r:FontReader<'_>,at:usize)->Result<(i32,i32),String>{let format=r.u16(at)?;if !matches!(format,1..=3){return Err("Unsupported font anchor format".into());}r.range(at,match format{1=>6,2=>8,_=>10})?;Ok((i32::from(r.i16(at+2)?),i32::from(r.i16(at+4)?)))}
 fn step(&mut self,bytes:&[u8])->Result<(),String>{if bytes.len()!=self.face.byte_length{return Err("Captured font source changed".into());}let r=FontReader::new(bytes,self.face.gpos.ok_or("Font mark authority missing")?)?;match self.phase{
  0=>{if self.sub==self.lookup.subtables{self.phase=7;return Ok(());}self.at=self.lookup.offset+usize::from(r.u16(self.lookup.offset+6+self.sub*2)?);if r.u16(self.at)?!=1{return Err("Unsupported font mark attachment format".into());}self.probe=Some(Index::new(r,self.at+usize::from(r.u16(self.at+2)?),self.mark,false)?);self.phase=1;},
  1=>{if !self.probe.as_mut().unwrap().step(r)?{return Ok(());}let index=self.probe.as_ref().unwrap().result();self.probe=None;if index<0{self.next_sub();return Ok(());}self.mark_index=index as usize;self.probe=Some(Index::new(r,self.at+usize::from(r.u16(self.at+4)?),self.base,false)?);self.phase=2;},
  2=>{if !self.probe.as_mut().unwrap().step(r)?{return Ok(());}let index=self.probe.as_ref().unwrap().result();self.probe=None;if index<0{self.next_sub();return Ok(());}self.base_index=index as usize;self.phase=3;},
  3=>{let array=self.at+usize::from(r.u16(self.at+8)?);let count=usize::from(r.u16(array)?);if self.mark_index>=count{return Err("Font mark index exceeds array authority".into());}let record=array+2+self.mark_index*4;self.mark_class=usize::from(r.u16(record)?);let offset=usize::from(r.u16(record+2)?);if self.mark_class>=usize::from(r.u16(self.at+6)?)||offset==0{return Err("Font mark anchor authority invalid".into());}self.mark_anchor=array+offset;self.phase=4;},
  4=>{let array=self.at+usize::from(r.u16(self.at+10)?);let count=usize::from(r.u16(array)?);let classes=usize::from(r.u16(self.at+6)?);if self.base_index>=count||classes==0{return Err("Font base index exceeds array authority".into());}let offset=usize::from(r.u16(array+2+(self.base_index*classes+self.mark_class)*2)?);if offset==0{self.next_sub();return Ok(());}self.base_anchor=array+offset;self.phase=5;},
  5=>{(self.mark_x,self.mark_y)=Self::anchor(r,self.mark_anchor)?;self.phase=6;},
  6=>{let(x,y)=Self::anchor(r,self.base_anchor)?;self.output=Some(FontMarkAttachment{x:x-self.mark_x,y:y-self.mark_y});self.phase=7;},
  _=>return Err("Font mark attachment stage invalid".into()),
 }Ok(())}
 pub fn advance(&mut self,bytes:&[u8],grant:usize)->Result<FontProgress,String>{if grant==0{return Err("Invalid font mark work grant".into());}if self.cancelled{return Err("Font mark cancelled".into());}if let Some(error)=&self.failure{return Err(error.clone());}for _ in 0..grant{if self.phase==7{break;}if self.work==self.max_work{self.failure=Some("Font mark work limit exceeded".into());return Err(self.failure.as_ref().unwrap().clone());}if let Err(error)=self.step(bytes){self.failure=Some(error.clone());return Err(error);}self.work+=1;}Ok(FontProgress{phase:match self.phase{0=>"subtable",1=>"markCoverage",2=>"baseCoverage",3=>"markRecord",4=>"baseRecord",5=>"markAnchor",6=>"baseAnchor",_=>"complete"},work:self.work,done:self.phase==7})}
 pub fn result(&self)->Result<Option<FontMarkAttachment>,String>{if self.phase!=7||self.cancelled||self.failure.is_some(){return Err("Font mark attachment incomplete".into());}Ok(self.output)}
 pub fn cancel(&mut self){self.cancelled=true;}
}
