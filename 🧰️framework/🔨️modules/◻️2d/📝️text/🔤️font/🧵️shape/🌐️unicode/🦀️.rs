//! 🌐️ Immutable original Unicode data validates one record and probes one ordered range per turn.
use super::super::{FontReader,FontProgress};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor};
pub const UNICODE_VERSION:&str="16.0.0";
#[path="🧵️normalize/🦀️.rs"]
pub mod normalize;
#[path="🧵️cluster/🦀️.rs"]
pub mod cluster;
#[path="↔️bidi/🦀️.rs"]
pub mod bidi;
pub const UNICODE_BYTES:&[u8]=include_bytes!("📦️data/🔢️properties.bin");
#[derive(Clone,Copy,Debug)]
pub struct UnicodeLayout{pub counts:[usize;6],pub offsets:[usize;6],pub byte_length:usize}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct UnicodeProperty{pub combining_class:u8,pub category:u8,pub bidi:u8,pub joining:u8,pub grapheme:u8,pub flags:u8,pub script:u32}
semio_framework_value::artifact_retire_leaf!(UnicodeLayout,UnicodeProperty);
#[derive(semio_framework_value::RetireOwned)]
pub struct AdmittedUnicode<S:RetireOwned>{pub source:S,pub layout:UnicodeLayout}
#[derive(semio_framework_value::RetireOwned)]
struct Owners<S:RetireOwned>{source:Option<S>,failure:Option<String>}
pub struct UnicodeDataAdmissionJob<S:AsRef<[u8]>+RetireOwned>{owners:Owners<S>,layout:Option<UnicodeLayout>,phase:u8,at:usize,previous:Option<u32>,previous_second:Option<u32>,work:u64,max_work:u64,cancelled:bool}
impl<S:AsRef<[u8]>+RetireOwned+'static> RetireOwned for UnicodeDataAdmissionJob<S>{fn retirement(self)->Box<dyn RetirementCursor>{self.owners.retirement()}fn retirement_birth_bytes(&self)->Option<usize>{self.owners.retirement_birth_bytes()}fn controlled_retirement_supported()->bool{true}}
impl<S:AsRef<[u8]>+RetireOwned> UnicodeDataAdmissionJob<S>{
 pub fn new(source:S,max_work:u64)->Self{let invalid=!(64..=16777216).contains(&source.as_ref().len())||!(1..=1000000000).contains(&max_work);Self{owners:Owners{source:Some(source),failure:invalid.then(||"Invalid Unicode admission limits".into())},layout:None,phase:0,at:0,previous:None,previous_second:None,work:0,max_work,cancelled:false}}
 fn next(&mut self,phase:u8){self.phase=phase;self.at=0;self.previous=None;self.previous_second=None;}
 fn step(&mut self)->Result<(),String>{let bytes=self.owners.source.as_ref().ok_or("Unicode source ownership transferred")?.as_ref();let r=FontReader::whole(bytes);match self.phase{
  0=>{if r.u32(0)?!=0x55434450||r.u32(4)?!=0x00100000||r.u32(56)?as usize!=bytes.len()||r.u32(60)?!=0{return Err("Unicode data version or extent mismatch".into());}let mut layout=UnicodeLayout{counts:[0;6],offsets:[0;6],byte_length:bytes.len()};let widths=[24,16,12,4,8,12];let mut end=64;for index in 0..6{let count=r.u32(8+index*8)?as usize;let offset=r.u32(12+index*8)?as usize;if count>100000||offset!=end{return Err("Unicode table authority exceeded or not contiguous".into());}r.range(offset,count*widths[index])?;end=offset+count*widths[index];layout.counts[index]=count;layout.offsets[index]=offset;}if end!=bytes.len()||layout.counts[0]==0{return Err("Unicode data extent or ranges invalid".into());}self.layout=Some(layout);self.next(1);},
  1=>{let l=self.layout.unwrap();if self.at==l.counts[0]{if self.previous!=Some(0x10ffff){return Err("Unicode ranges omit terminal scalars".into());}self.next(2);return Ok(());}let at=l.offsets[0]+self.at*24;self.at+=1;let first=r.u32(at)?;let last=r.u32(at+4)?;if first!=self.previous.map_or(0,|code|code+1)||last<first||last>0x10ffff||r.u8(at+9)?>29||r.u8(at+10)?>22||r.u8(at+11)?>5||r.u8(at+12)?>13||r.u8(at+13)?>127||r.u16(at+14)?!=0||r.u32(at+20)?!=0{return Err("Unicode property range invalid".into());}self.previous=Some(last);},
  2=>{let l=self.layout.unwrap();if self.at==l.counts[1]{self.next(3);return Ok(());}let at=l.offsets[1]+self.at*16;self.at+=1;let code=r.u32(at)?;let from=r.u32(at+4)?as usize;let count=r.u32(at+8)?as usize;if self.previous.is_some_and(|last|code<=last)||code>0x10ffff||count==0||count>64||from.checked_add(count).is_none_or(|end|end>l.counts[3])||r.u32(at+12)?>1{return Err("Unicode decomposition authority invalid".into());}self.previous=Some(code);},
  3=>{let l=self.layout.unwrap();if self.at==l.counts[2]{self.next(4);return Ok(());}let at=l.offsets[2]+self.at*12;self.at+=1;let a=r.u32(at)?;let b=r.u32(at+4)?;let result=r.u32(at+8)?;if self.previous.is_some_and(|last|a<last||a==last&&self.previous_second.is_some_and(|second|b<=second))||a>0x10ffff||b>0x10ffff||result>0x10ffff{return Err("Unicode composition authority invalid".into());}self.previous=Some(a);self.previous_second=Some(b);},
  4=>{let l=self.layout.unwrap();if self.at==l.counts[3]{self.next(5);return Ok(());}if r.u32(l.offsets[3]+self.at*4)?>0x10ffff{return Err("Unicode decomposition scalar invalid".into());}self.at+=1;},
  5=>{let l=self.layout.unwrap();if self.at==l.counts[4]{self.next(6);return Ok(());}let at=l.offsets[4]+self.at*8;self.at+=1;let code=r.u32(at)?;if self.previous.is_some_and(|last|code<=last)||code>0x10ffff||r.u32(at+4)?>0x10ffff{return Err("Unicode mirroring authority invalid".into());}self.previous=Some(code);},
  6=>{let l=self.layout.unwrap();if self.at==l.counts[5]{self.phase=7;return Ok(());}let at=l.offsets[5]+self.at*12;self.at+=1;let code=r.u32(at)?;let kind=r.u32(at+8)?;if self.previous.is_some_and(|last|code<=last)||code>0x10ffff||r.u32(at+4)?>0x10ffff||!matches!(kind,1|2){return Err("Unicode bracket authority invalid".into());}self.previous=Some(code);},
  _=>return Err("Unicode admission stage invalid".into()),
 }Ok(())}
 pub fn advance(&mut self,grant:usize)->Result<FontProgress,String>{if grant==0{return Err("Invalid Unicode work grant".into());}if self.cancelled{return Err("Unicode admission cancelled".into());}if let Some(error)=&self.owners.failure{return Err(error.clone());}for _ in 0..grant{if self.phase==7{break;}if self.work==self.max_work{self.owners.failure=Some("Unicode admission work limit exceeded".into());return Err(self.owners.failure.as_ref().unwrap().clone());}if let Err(error)=self.step(){self.owners.failure=Some(error.clone());return Err(error);}self.work+=1;}Ok(FontProgress{phase:match self.phase{0=>"header",1=>"ranges",2=>"decompositions",3=>"compositions",4=>"scalars",5=>"mirrors",6=>"brackets",_=>"complete"},work:self.work,done:self.phase==7})}
 pub fn result(&self)->Result<UnicodeLayout,String>{if self.phase!=7||self.cancelled||self.owners.failure.is_some()||self.owners.source.is_none(){return Err("Unicode admission incomplete".into());}Ok(self.layout.unwrap())}
 pub fn take_result(&mut self)->Result<AdmittedUnicode<S>,String>{let layout=self.result()?;Ok(AdmittedUnicode{source:self.owners.source.take().unwrap(),layout})}
 pub fn cancel(&mut self){self.cancelled=true;}
}
#[derive(Clone,Copy)]
pub struct UnicodePropertyCursor{layout:UnicodeLayout,scalar:u32,low:usize,high:usize,output:Option<UnicodeProperty>}
semio_framework_value::artifact_retire_leaf!(UnicodePropertyCursor);
impl UnicodePropertyCursor{
 pub fn new(layout:UnicodeLayout,scalar:u32)->Result<Self,String>{if scalar>0x10ffff{return Err("Unicode property scalar invalid".into());}Ok(Self{layout,scalar,low:0,high:layout.counts[0],output:None})}
 pub fn step(&mut self,bytes:&[u8])->Result<bool,String>{if self.output.is_some(){return Ok(true);}if bytes.len()!=self.layout.byte_length||self.low==self.high{return Err("Unicode source changed or scalar range missing".into());}let r=FontReader::whole(bytes);let index=(self.low+self.high)/2;let at=self.layout.offsets[0]+index*24;let first=r.u32(at)?;let last=r.u32(at+4)?;if self.scalar<first{self.high=index;}else if self.scalar>last{self.low=index+1;}else{self.output=Some(UnicodeProperty{combining_class:r.u8(at+8)?,category:r.u8(at+9)?,bidi:r.u8(at+10)?,joining:r.u8(at+11)?,grapheme:r.u8(at+12)?,flags:r.u8(at+13)?,script:r.u32(at+16)?});}Ok(self.output.is_some())}
 pub fn result(&self)->Result<UnicodeProperty,String>{self.output.ok_or_else(||"Unicode property probe incomplete".into())}
}
#[derive(Clone,Copy,PartialEq,Eq)]
pub enum UnicodeMapping{Decomposition,Composition,Mirror,Bracket}
#[derive(Clone,Copy)]
pub struct UnicodeMappingCursor{layout:UnicodeLayout,kind:UnicodeMapping,first:u32,second:u32,low:usize,high:usize,done:bool,found:Option<usize>}
semio_framework_value::artifact_retire_leaf!(UnicodeMapping,UnicodeMappingCursor);
impl UnicodeMappingCursor{
 pub fn new(layout:UnicodeLayout,kind:UnicodeMapping,first:u32,second:u32)->Self{let index=match kind{UnicodeMapping::Decomposition=>1,UnicodeMapping::Composition=>2,UnicodeMapping::Mirror=>4,UnicodeMapping::Bracket=>5};Self{layout,kind,first,second,low:0,high:layout.counts[index],done:false,found:None}}
 pub fn step(&mut self,bytes:&[u8])->Result<bool,String>{if self.done{return Ok(true);}if bytes.len()!=self.layout.byte_length{return Err("Unicode source changed".into());}if self.low==self.high{self.done=true;return Ok(true);}let(index,width)=match self.kind{UnicodeMapping::Decomposition=>(1,16),UnicodeMapping::Composition=>(2,12),UnicodeMapping::Mirror=>(4,8),UnicodeMapping::Bracket=>(5,12)};let r=FontReader::whole(bytes);let at=self.layout.offsets[index]+(self.low+self.high)/2*width;let a=r.u32(at)?;let b=if self.kind==UnicodeMapping::Composition{r.u32(at+4)?}else{0};if(self.first,self.second)<(a,b){self.high=(self.low+self.high)/2;}else if(self.first,self.second)>(a,b){self.low=(self.low+self.high)/2+1;}else{self.found=Some(at);self.done=true;}Ok(self.done)}
 pub fn result(&self)->Result<Option<usize>,String>{if !self.done{return Err("Unicode mapping probe incomplete".into());}Ok(self.found)}
}
