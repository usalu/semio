//! 🔤️ Genuine SFNT byte sources admit Unicode mapping and metrics under bounded record turns.
use semio_framework_value::{retirement::{RetireOwned,RetirementCursor},list::PagedList};
#[derive(Clone,Copy,Default,Debug)]
pub struct FontTable{pub offset:usize,pub length:usize}
#[derive(Clone,Copy,Debug)]
pub struct FontFace{pub byte_length:usize,pub units_per_em:u16,pub glyphs:u16,pub metrics:u16,pub long_loca:bool,pub head:FontTable,pub maxp:FontTable,pub hhea:FontTable,pub hmtx:FontTable,pub loca:FontTable,pub glyf:FontTable,pub cmap:FontTable,pub kern:Option<FontTable>,pub gpos:Option<FontTable>,pub gsub:Option<FontTable>,pub gdef:Option<FontTable>,pub cmap_offset:usize,pub cmap_format:u16,pub cmap_count:usize}
#[derive(Clone,Copy,Debug)]
pub struct FontOutlineLimits{pub max_font_bytes:usize,pub max_tables:usize,pub max_glyphs:usize,pub max_points:usize,pub max_contours:usize,pub max_components:usize,pub max_depth:usize,pub max_segments:usize,pub max_work:u64}
impl Default for FontOutlineLimits{fn default()->Self{Self{max_font_bytes:67108864,max_tables:128,max_glyphs:262144,max_points:1048576,max_contours:65536,max_components:262144,max_depth:32,max_segments:1048576,max_work:1000000000}}}
#[derive(Clone,Copy,Debug)]
pub struct FontProgress{pub phase:&'static str,pub work:u64,pub done:bool}
semio_framework_value::artifact_retire_leaf!(FontTable,FontFace,FontOutlineLimits,FontProgress);
pub trait FontSource:AsRef<[u8]>+RetireOwned{}
impl<T:AsRef<[u8]>+RetireOwned> FontSource for T{}
#[derive(Clone,Copy)]
pub struct StaticFontBytes(pub &'static[u8]);
impl AsRef<[u8]> for StaticFontBytes{fn as_ref(&self)->&[u8]{self.0}}
semio_framework_value::artifact_retire_leaf!(StaticFontBytes);
#[derive(semio_framework_value::RetireOwned)]
pub struct AdmittedFont<S:RetireOwned>{pub source:S,pub face:FontFace}
#[derive(Clone,Copy)]
pub(crate) struct FontReader<'a>{bytes:&'a[u8],start:usize,end:usize}
impl<'a> FontReader<'a>{
 pub fn new(bytes:&'a[u8],table:FontTable)->Result<Self,String>{let end=table.offset.checked_add(table.length).ok_or("Font byte span exceeded")?;if end>bytes.len(){return Err("Invalid font byte span".into());}Ok(Self{bytes,start:table.offset,end})}
 pub fn whole(bytes:&'a[u8])->Self{Self{bytes,start:0,end:bytes.len()}}
 pub fn range(&self,at:usize,count:usize)->Result<(),String>{if at<self.start||at.checked_add(count).is_none_or(|end|end>self.end){return Err("Font byte span exceeded".into());}Ok(())}
 pub fn u8(&self,at:usize)->Result<u8,String>{self.range(at,1)?;Ok(self.bytes[at])}
 pub fn i8(&self,at:usize)->Result<i8,String>{Ok(self.u8(at)?as i8)}
 pub fn u16(&self,at:usize)->Result<u16,String>{self.range(at,2)?;Ok(u16::from_be_bytes([self.bytes[at],self.bytes[at+1]]))}
 pub fn i16(&self,at:usize)->Result<i16,String>{Ok(self.u16(at)?as i16)}
 pub fn u32(&self,at:usize)->Result<u32,String>{self.range(at,4)?;Ok(u32::from_be_bytes([self.bytes[at],self.bytes[at+1],self.bytes[at+2],self.bytes[at+3]]))}
}
#[derive(semio_framework_value::RetireOwned)]
struct AdmissionOwners<S:RetireOwned>{source:Option<S>,failure:Option<String>}
pub struct FontFaceAdmissionJob<S:FontSource>{owners:AdmissionOwners<S>,limits:FontOutlineLimits,phase:u8,at:usize,count:usize,tables:[(u32,FontTable);128],table_count:usize,selected:Option<FontFace>,cmap_at:usize,cmap_records:usize,cmap_rank:u8,cmap_segment:usize,previous_end:Option<u32>,work:u64,cancelled:bool}
impl<S:FontSource+'static> RetireOwned for FontFaceAdmissionJob<S>{fn retirement(self)->Box<dyn RetirementCursor>{self.owners.retirement()}fn retirement_birth_bytes(&self)->Option<usize>{self.owners.retirement_birth_bytes()}fn controlled_retirement_supported()->bool{true}}
impl<S:FontSource> FontFaceAdmissionJob<S>{
 pub fn new(source:S,limits:FontOutlineLimits)->Self{let failure=(!(64..=67108864).contains(&limits.max_font_bytes)||source.as_ref().len()>limits.max_font_bytes||!(1..=128).contains(&limits.max_tables)||!(1..=1000000000).contains(&limits.max_work)).then(||"Invalid font admission limits".into());Self{owners:AdmissionOwners{source:Some(source),failure},limits,phase:0,at:0,count:0,tables:[(0,FontTable::default());128],table_count:0,selected:None,cmap_at:0,cmap_records:0,cmap_rank:0,cmap_segment:0,previous_end:None,work:0,cancelled:false}}
 fn table(&self,tag:u32)->Option<FontTable>{self.tables[..self.table_count].iter().find(|entry|entry.0==tag).map(|entry|entry.1)}
 fn required(&self,tag:u32)->Result<FontTable,String>{self.table(tag).ok_or_else(||"Required font table missing".into())}
 fn step(&mut self)->Result<(),String>{let bytes=self.owners.source.as_ref().unwrap().as_ref();let r=FontReader::whole(bytes);match self.phase{
  0=>{if !matches!(r.u32(0)?,0x00010000|0x74727565){return Err("Font requires TrueType SFNT outlines".into());}self.count=usize::from(r.u16(4)?);if self.count==0||self.count>self.limits.max_tables{return Err("Font table limit exceeded".into());}r.range(12,self.count*16)?;self.phase=1;},
  1=>{if self.at<self.count{let at=12+self.at*16;self.at+=1;let tag=r.u32(at)?;let offset=r.u32(at+8)?as usize;let length=r.u32(at+12)?as usize;r.range(offset,length)?;if self.table(tag).is_some(){return Err("Duplicate font table".into());}self.tables[self.table_count]=(tag,FontTable{offset,length});self.table_count+=1;}else{self.phase=2;}},
  2=>{let head=self.required(0x68656164)?;let maxp=self.required(0x6d617870)?;let hhea=self.required(0x68686561)?;let hmtx=self.required(0x686d7478)?;let loca=self.required(0x6c6f6361)?;let glyf=self.required(0x676c7966)?;let cmap=self.required(0x636d6170)?;if head.length<54||maxp.length<6||hhea.length<36||cmap.length<4{return Err("Invalid font metric table spans".into());}let units=r.u16(head.offset+18)?;let glyphs=r.u16(maxp.offset+4)?;let metrics=r.u16(hhea.offset+34)?;let format=r.i16(head.offset+50)?;if !(16..=16384).contains(&units)||glyphs==0||metrics==0||metrics>glyphs||!matches!(format,0|1)||loca.length<(usize::from(glyphs)+1)*if format==1{4}else{2}||hmtx.length<usize::from(metrics)*4+usize::from(glyphs-metrics)*2{return Err("Invalid font metrics authority".into());}self.selected=Some(FontFace{byte_length:bytes.len(),units_per_em:units,glyphs,metrics,long_loca:format==1,head,maxp,hhea,hmtx,loca,glyf,cmap,kern:self.table(0x6b65726e),gpos:self.table(0x47504f53),gsub:self.table(0x47535542),gdef:self.table(0x47444546),cmap_offset:0,cmap_format:4,cmap_count:0});self.cmap_records=usize::from(r.u16(cmap.offset+2)?);if self.cmap_records>128||4+self.cmap_records*8>cmap.length{return Err("Invalid font mapping directory".into());}self.phase=3;},
  3=>{let f=self.selected.as_mut().unwrap();let span=FontReader::new(bytes,f.cmap)?;if self.cmap_at<self.cmap_records{let at=f.cmap.offset+4+self.cmap_at*8;self.cmap_at+=1;let platform=span.u16(at)?;let encoding=span.u16(at+2)?;if platform!=0&&(platform!=3||!matches!(encoding,1|10)){return Ok(());}let offset=f.cmap.offset+span.u32(at+4)?as usize;let format=span.u16(offset)?;let rank=match format{12=>2,4=>1,_=>0};if rank<=self.cmap_rank{return Ok(());}let length=if format==12{span.u32(offset+4)?as usize}else{usize::from(span.u16(offset+2)?)};span.range(offset,length)?;let count=if format==12{span.u32(offset+12)?as usize}else{let count=span.u16(offset+6)?;if count%2!=0{return Err("Invalid font Unicode segment width".into());}usize::from(count/2)};if count==0||count>65536||length<if format==12{16+count*12}else{16+count*8}{return Err("Invalid Unicode font mapping".into());}f.cmap_offset=offset;f.cmap_format=format;f.cmap_count=count;self.cmap_rank=rank;}else{if self.cmap_rank==0{return Err("Unicode font mapping missing".into());}self.phase=4;}},
  4=>{let f=self.selected.as_ref().unwrap();if self.cmap_segment==f.cmap_count{self.phase=5;return Ok(());}let r=FontReader::new(bytes,f.cmap)?;let i=self.cmap_segment;self.cmap_segment+=1;let base=f.cmap_offset;let(from,to)=if f.cmap_format==12{(r.u32(base+16+i*12)?,r.u32(base+20+i*12)?)}else{(u32::from(r.u16(base+16+f.cmap_count*2+i*2)?),u32::from(r.u16(base+14+i*2)?))};if from>to||self.previous_end.is_some_and(|end|from<=end)||to>0x10ffff{return Err("Unicode font segments are not ordered".into());}self.previous_end=Some(to);if f.cmap_format==12{let glyph=r.u32(base+24+i*12)?;if glyph.checked_add(to-from).is_none_or(|end|end>=u32::from(f.glyphs)){return Err("Unicode font segment exceeds glyph authority".into());}}else{let word=base+16+f.cmap_count*6+i*2;let offset=usize::from(r.u16(word)?);if offset>0{r.range(word+offset,(to-from+1)as usize*2)?;}}},
  _=>return Err("Font admission stage invalid".into()),
 }Ok(())}
 pub fn advance(&mut self,grant:usize)->Result<FontProgress,String>{if grant==0{return Err("Invalid font work grant".into());}if self.cancelled{return Err("Font admission cancelled".into());}if let Some(error)=&self.owners.failure{return Err(error.clone());}for _ in 0..grant{if self.phase==5{break;}if self.work>=self.limits.max_work{self.owners.failure=Some("Font admission work limit exceeded".into());return Err(self.owners.failure.as_ref().unwrap().clone());}if let Err(error)=self.step(){self.owners.failure=Some(error.clone());return Err(error);}self.work+=1;}Ok(FontProgress{phase:match self.phase{0=>"header",1=>"directory",2=>"metrics",3=>"cmap",4=>"cmapSegments",_=>"complete"},work:self.work,done:self.phase==5})}
 pub fn result(&self)->Result<FontFace,String>{if self.cancelled||self.owners.failure.is_some()||self.phase!=5||self.owners.source.is_none(){return Err("Font admission incomplete".into());}Ok(self.selected.unwrap())}
 pub fn take_result(&mut self)->Result<AdmittedFont<S>,String>{let face=self.result()?;Ok(AdmittedFont{source:self.owners.source.take().unwrap(),face})}
 pub fn cancel(&mut self){self.cancelled=true;}
}
#[derive(Clone,Copy)]
pub struct FontCmapCursor{face:FontFace,scalar:u32,low:usize,high:usize,done:bool,glyph:u16}
semio_framework_value::artifact_retire_leaf!(FontCmapCursor);
impl FontCmapCursor{
 pub fn new(face:FontFace,scalar:u32)->Result<Self,String>{if scalar>0x10ffff||(0xd800..=0xdfff).contains(&scalar){return Err("Invalid Unicode font scalar".into());}Ok(Self{face,scalar,low:0,high:face.cmap_count,done:false,glyph:0})}
 pub fn step(&mut self,bytes:&[u8])->Result<bool,String>{if self.done{return Ok(true);}if bytes.len()!=self.face.byte_length{return Err("Captured font source changed".into());}if self.low==self.high{self.done=true;return Ok(true);}let f=self.face;let r=FontReader::new(bytes,f.cmap)?;let i=(self.low+self.high)/2;let base=f.cmap_offset;let glyph=if f.cmap_format==12{let from=r.u32(base+16+i*12)?;let to=r.u32(base+20+i*12)?;if self.scalar<from{self.high=i;None}else if self.scalar>to{self.low=i+1;None}else{Some(r.u32(base+24+i*12)?.checked_add(self.scalar-from).ok_or("Font glyph offset overflow")?)}}else{if self.scalar>65535{self.done=true;return Ok(true);}let end=u32::from(r.u16(base+14+i*2)?);let start=u32::from(r.u16(base+16+f.cmap_count*2+i*2)?);if self.scalar<start{self.high=i;None}else if self.scalar>end{self.low=i+1;None}else{let delta=i32::from(r.i16(base+16+f.cmap_count*4+i*2)?);let word=base+16+f.cmap_count*6+i*2;let offset=usize::from(r.u16(word)?);let glyph=if offset>0{let glyph=r.u16(word+offset+(self.scalar-start)as usize*2)?;if glyph==0{0}else{(i32::from(glyph)+delta).rem_euclid(65536)as u32}}else{(self.scalar as i32+delta).rem_euclid(65536)as u32};Some(glyph)}};if let Some(glyph)=glyph{if glyph>=u32::from(f.glyphs){return Err("Unicode mapping exceeds font glyph authority".into());}self.glyph=glyph as u16;self.done=true;}Ok(self.done)}
 pub fn result(&self)->Result<u16,String>{if !self.done{return Err("Unicode font mapping incomplete".into());}Ok(self.glyph)}
}
pub fn font_advance(face:FontFace,bytes:&[u8],glyph:u16)->Result<u16,String>{if glyph>=face.glyphs||bytes.len()!=face.byte_length{return Err("Font glyph metric index invalid".into());}FontReader::new(bytes,face.hmtx)?.u16(face.hmtx.offset+usize::from(glyph.min(face.metrics-1))*4)}
pub(crate) fn reserve_slot<T>(values:&mut PagedList<T,{usize::MAX}>)->Result<bool,String>{if values.has_reserved_slot(){return Ok(true);}let capacity=values.next_allocation_bytes().map_err(|error|error.to_string())?;values.reserve_one(capacity).map_err(|error|error.refusal().to_string())?;Ok(false)}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
#[path="🧵️outline/🦀️.rs"]
pub mod outline;
#[path="🤝️kerning/🦀️.rs"]
pub mod kerning;
#[path="🧵️run/🦀️.rs"]
pub mod run;
#[path="📇️catalog/🦀️.rs"]
pub mod catalog;
#[path="🧵️shape/🦀️.rs"]
pub mod shape;
