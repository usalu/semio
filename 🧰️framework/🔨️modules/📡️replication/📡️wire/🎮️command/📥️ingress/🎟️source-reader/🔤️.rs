//! 🔤️ Source-bounded scalar and UTF-8 field atoms preserve every suspended read and byte owner.
use super::*;
#[derive(Debug,Default,semio_framework_value::RetireOwned)]
#[value(crate="::semio_framework_value")]
pub struct OriginalCommandVarint{byte:Option<u8>,value:u64,shift:u8,ready:bool}
fn pending(turn:Turn<Step>)->Turn<Step>{Turn{result:turn.result.map(|step|Step::Progress(step.progress())),work:turn.work}}
fn invalid(message:&'static str)->Turn<Step>{Turn{result:Err(ValueError::literal(ValueRefusalKind::InvalidValue,message)),work:0}}
fn placement(bytes:usize,grant:Grant,work:u64)->bool{work>0&&retained_clone_grant_admits(grant,Demand{copy_bytes:bytes,depth:1,..Default::default()})}
fn placed(bytes:usize)->Turn<Step>{complete(Progress{copied_items:1,copied_bytes:bytes,..Default::default()},1)}
impl OriginalCommandVarint{
 /// 🔎️ Observes only a fully read scalar.
 pub fn value(&self)->Option<u64>{self.ready.then_some(self.value)}
 /// 📏️ A strict nested codec can reject an overlong scalar before transferring its frontier.
 pub fn canonical(&self)->Option<bool>{self.ready.then_some(self.shift==0||self.value>>self.shift!=0)}
 /// 🧮️ Reads or folds a single paid byte without discarding a partial varint on refusal.
 pub fn step(&mut self,reader:&mut impl OriginalBinaryReadSource,grant:Grant,work:u64)->Turn<Step>{
  if self.ready{return complete(Default::default(),0)}
  let Some(byte)=self.byte else{if reader.source_page_exhausted(){return pending(reader.release_consumed_source_step(grant,work))}return pending(reader.read_byte_source_step(&mut self.byte,grant,work))};
  if self.shift==63&&byte>1{return invalid("paged command varint overflowed u64")}
  if !placement(size_of::<Self>(),grant,work){return waiting()}
  let value=self.value|(u64::from(byte&127)<<self.shift);let ready=byte<128;
  *self=Self{byte:None,value,shift:if ready{self.shift}else{self.shift+7},ready};
  if ready{placed(size_of::<Self>())}else{pending(placed(size_of::<Self>()))}
 }
 /// 📬️ Transfers the completed scalar and clears its original frontier under the same grant.
 pub fn transfer_source_step(&mut self,output:&mut Option<u64>,grant:Grant,work:u64)->Turn<Step>{
  if output.is_some(){return refused("varint transfer requires an empty destination")}
  if !self.ready{return waiting()}let bytes=size_of::<Self>()+size_of::<Option<u64>>();if !placement(bytes,grant,work){return waiting()}
  *output=Some(self.value);*self=Self::default();placed(bytes)
 }
}
#[derive(Debug,Default,semio_framework_value::RetireOwned)]
#[value(crate="::semio_framework_value")]
pub struct OriginalCommandField{phase:u8,maximum:usize,text:bool,length:usize,varint:OriginalCommandVarint,bytes:Option<Vec<u8>>,index:usize,continuations:u8,lower:u8,upper:u8,characters:usize}
impl OriginalCommandField{
 /// 🏁️ Borrows validated, completely copied field bytes without transferring their owner.
 pub fn characters(&self)->Option<usize>{(self.phase==5&&self.text).then_some(self.characters)}
 /// 📚️ Borrows completed field bytes.
 pub fn bytes(&self)->Option<&[u8]>{(self.phase==5).then(||self.bytes.as_ref().unwrap().as_slice())}
 /// 📥️ Advances exactly one admitted length, backing, copy, validation or ready placement.
 pub fn step(&mut self,reader:&mut impl OriginalBinaryReadSource,maximum:usize,text:bool,grant:Grant,work:u64)->Turn<Step>{
  if self.phase!=0&&(self.maximum!=maximum||self.text!=text){return refused("command field request changed across its original frontier")}
  if self.phase==5{return complete(Default::default(),0)}
  if self.phase==0{if !placement(size_of::<Self>(),grant,work){return waiting()}*self=Self{phase:1,maximum,text,..Self::default()};return pending(placed(size_of::<Self>()))}
  if self.phase==1{let Some(value)=self.varint.value()else{return pending(self.varint.step(reader,grant,work))};let Ok(length)=usize::try_from(value)else{return invalid("command field length is not representable")};if length>self.maximum{return invalid("command field exceeds its exact bounded authority")}let bytes=size_of::<usize>()+size_of::<OriginalCommandVarint>()+size_of::<u8>();if !placement(bytes,grant,work){return waiting()}self.length=length;self.varint=OriginalCommandVarint::default();self.phase=2;return pending(placed(bytes))}
  if self.phase==2{if self.bytes.is_none(){return pending(reader.admit_field_source_step(&mut self.bytes,self.length,grant,work))}if !placement(size_of::<u8>(),grant,work){return waiting()}self.phase=3;return pending(placed(size_of::<u8>()))}
  if self.phase==3{if self.bytes.as_ref().unwrap().len()!=self.length{if reader.source_page_exhausted(){return pending(reader.release_consumed_source_step(grant,work))}return pending(reader.copy_field_source_step(self.bytes.as_mut().unwrap(),self.length,grant,work))}if !placement(size_of::<u8>(),grant,work){return waiting()}self.phase=if text{4}else{5};return if self.phase==5{placed(size_of::<u8>())}else{pending(placed(size_of::<u8>()))}}
  if self.index==self.length{if self.continuations!=0{return invalid("command field ends inside a UTF8 scalar")}if !placement(size_of::<u8>(),grant,work){return waiting()}self.phase=5;return placed(size_of::<u8>())}
  let byte=self.bytes.as_ref().unwrap()[self.index];let(next,lower,upper)=if self.continuations!=0{if byte<self.lower||byte>self.upper{return invalid("command field contains invalid UTF8 continuation")}(self.continuations-1,0x80,0xbf)}else{match byte{0..=0x7f=>(0,0x80,0xbf),0xc2..=0xdf=>(1,0x80,0xbf),0xe0=>(2,0xa0,0xbf),0xe1..=0xec|0xee..=0xef=>(2,0x80,0xbf),0xed=>(2,0x80,0x9f),0xf0=>(3,0x90,0xbf),0xf1..=0xf3=>(3,0x80,0xbf),0xf4=>(3,0x80,0x8f),_=>return invalid("command field contains invalid UTF8 leading byte")}};
  let bytes=2*size_of::<usize>()+3*size_of::<u8>();if !placement(bytes,grant,work){return waiting()}self.index+=1;self.characters+=usize::from(self.continuations==0);self.continuations=next;self.lower=lower;self.upper=upper;pending(placed(bytes))
 }
 /// 📦️ Transfers a fully copied byte field without cloning or releasing its allocation.
 pub fn transfer_bytes_source_step(&mut self,output:&mut Option<Vec<u8>>,grant:Grant,work:u64)->Turn<Step>{
  if output.is_some(){return refused("command field transfer requires an empty byte destination")}if self.phase!=5{return waiting()}
  let bytes=size_of::<Self>()+size_of::<Option<Vec<u8>>>();if !placement(bytes,grant,work){return waiting()}*output=self.bytes.take();*self=Self::default();placed(bytes)
 }
 /// 🔤️ Transfers only a field whose complete UTF8 state machine reached its accepted frontier.
 pub fn transfer_text_source_step(&mut self,output:&mut Option<String>,grant:Grant,work:u64)->Turn<Step>{
  if output.is_some(){return refused("command field transfer requires an empty text destination")}if self.phase!=5||!self.text{return waiting()}
  let bytes=size_of::<Self>()+size_of::<Option<String>>();if !placement(bytes,grant,work){return waiting()}
  *output=Some(unsafe{String::from_utf8_unchecked(self.bytes.take().unwrap())});*self=Self::default();placed(bytes)
 }
}
