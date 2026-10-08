//! 🎞️ Incremental intrinsic Body reconstruction with explicit source and partial-value custody.
use crate::{PackLimits,PackRefusal,record::{RetainedRecordBodyCursor,RetainedRecordBodyToken,RetainedValueToken as Token,RetainedValueContainer as Container,RetainedValueRole as Role}};
use semio_framework_value::{DslValue,Number,ValueError,ValueRefusalKind,retirement::controlled::ControlledRetirement,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::{ManuallyDrop,size_of};

/// 📥️ Immutable wire custody transfers without copying its complete backing.
#[derive(Debug)]
pub enum RetainedIntrinsicInput<'a>{Borrowed(&'a[u8]),OwnedBytes(Vec<u8>)}
impl RetainedIntrinsicInput<'_>{fn bytes(&self)->&[u8]{match self{Self::Borrowed(bytes)=>bytes,Self::OwnedBytes(bytes)=>bytes}}fn capacity(&self)->usize{match self{Self::Borrowed(_)=>0,Self::OwnedBytes(bytes)=>bytes.capacity()}}}

#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct RetainedIntrinsicStep{pub units:usize,pub input_bytes:usize,pub admitted_bytes:usize,pub complete:bool}

struct Frame{value:DslValue,remaining:usize,key:Option<String>}
semio_framework_value::artifact_retire_struct!(Frame{value,remaining,key});
struct Owned{frames:Vec<Frame>,scalar:DslValue,output:Option<DslValue>,input:Vec<u8>}
semio_framework_value::artifact_retire_struct!(Owned{frames,scalar,output,input});
impl Default for Owned{fn default()->Self{Self{frames:Vec::new(),scalar:DslValue::Null,output:None,input:Vec::new()}}}
#[derive(Clone,Copy)]
enum Copy{None,Inline(usize),Bytes(usize),Symbol{symbol:u64,index:usize,count:usize},Ready}

/// 🎮️ Each advance returns actual bounded work; every terminal path requires explicit close.
struct Materializer<'a,D:Driver>{
 input:Option<RetainedIntrinsicInput<'a>>,cursor:D,owned:ManuallyDrop<Owned>,retiring:Option<ControlledRetirement<Owned>>,
 offset:usize,maximum:usize,admitted:usize,maximum_frames:usize,header:u8,copy:Copy,sealed:bool,complete:bool,closing:bool,closed:bool,fault:Option<PackRefusal>,
}
fn invalid(detail:&'static str)->PackRefusal{PackRefusal::RetainedMalformed{kind:ValueRefusalKind::InvalidValue,what:"retained intrinsic Body",offset:0,detail}}
fn capacity()->PackRefusal{PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"retained intrinsic cumulative ownership"}}
fn allocation()->PackRefusal{PackRefusal::ValueRefusal(ValueError::literal(ValueRefusalKind::AllocationFailed,"retained intrinsic allocation failed"))}
impl<'a,D:Driver> Materializer<'a,D>{
 /// 🎟️ Admits transferred source backing before constructing an allocation-free parser.
 fn new(input:RetainedIntrinsicInput<'a>,mut limits:PackLimits,maximum:usize)->Result<Self,(PackRefusal,RetainedIntrinsicInput<'a>)>{
  if input.bytes().len()as u64>limits.max_file_len||input.capacity()>maximum{return Err((capacity(),input))}
  let maximum=maximum.min(usize::try_from(limits.max_total_alloc).unwrap_or(usize::MAX));let admitted=input.capacity();
  if maximum==0||admitted>maximum{return Err((capacity(),input))}
  limits.max_total_alloc=maximum as u64;let maximum_frames=usize::from(limits.max_depth)+1;
  let cursor=match D::new(limits,input.bytes().len(),maximum){Ok(cursor)=>cursor,Err(error)=>return Err((error,input))};
  Ok(Self{input:Some(input),cursor,owned:ManuallyDrop::new(Owned::default()),retiring:None,offset:0,maximum,admitted,maximum_frames,header:0,copy:Copy::None,sealed:false,complete:false,closing:false,closed:false,fault:None})
 }
 pub fn admitted_bytes(&self)->usize{self.admitted}
 /// 👁️ Borrows the retained immutable wire until close transfers it into retirement.
 pub fn input(&self)->Option<&[u8]>{self.input.as_ref().map(RetainedIntrinsicInput::bytes)}
 fn remaining(&self)->usize{self.maximum.saturating_sub(self.admitted)}
 fn admit(&mut self,bytes:usize)->Result<(),PackRefusal>{self.admitted=self.admitted.checked_add(bytes).ok_or_else(capacity)?;if self.admitted>self.maximum{Err(capacity())}else{Ok(())}}
 fn text(&mut self,length:usize)->Result<(),PackRefusal>{
  if length>self.remaining()||length>isize::MAX as usize{return Err(capacity())}
  self.owned.scalar=DslValue::String(String::new());let DslValue::String(text)=&mut self.owned.scalar else{unreachable!()};
  text.try_reserve_exact(length).map_err(|_|allocation())?;
  let bytes=text.capacity();self.admit(bytes)
 }
 fn attach(&mut self,value:DslValue)->Result<(),PackRefusal>{
  if let Some(frame)=self.owned.frames.last_mut(){
   if frame.remaining==0{self.owned.scalar=value;return Err(invalid("too many container values"))}
   match &mut frame.value{
    DslValue::Array(values)=>{values.push(value);frame.remaining-=1;},
    DslValue::Object(values)=>{if let Some(key)=frame.key.take(){values.push((key,value));frame.remaining-=1}else if let DslValue::String(key)=value{frame.key=Some(key)}else{self.owned.scalar=value;return Err(invalid("object key is not text"))}},
    _=>{self.owned.scalar=value;return Err(invalid("invalid reconstruction frame"))}
   }
  }else if self.owned.output.is_none(){self.owned.output=Some(value)}else{self.owned.scalar=value;return Err(invalid("multiple intrinsic values"))}
  Ok(())
 }
 fn finish_scalar(&mut self)->Result<(),PackRefusal>{self.copy=Copy::None;let value=std::mem::replace(&mut self.owned.scalar,DslValue::Null);self.attach(value)}
 fn begin(&mut self,kind:Container,count:u64)->Result<(),PackRefusal>{
  let count=usize::try_from(count).map_err(|_|capacity())?;let size=match kind{Container::List=>size_of::<DslValue>(),Container::Map=>size_of::<(String,DslValue)>(),_=>return Err(invalid("non-intrinsic container"))};
  if self.owned.frames.len()==self.maximum_frames{return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::DepthLimit,limit:"retained intrinsic frame depth"})}
  let bytes=count.checked_mul(size).filter(|bytes|*bytes<=self.remaining()&&*bytes<=isize::MAX as usize).ok_or_else(capacity)?;let _=bytes;
  self.owned.frames.push(Frame{value:if kind==Container::List{DslValue::Array(Vec::new())}else{DslValue::Object(Vec::new())},remaining:count,key:None});
  let frame=self.owned.frames.last_mut().unwrap();let actual=match &mut frame.value{DslValue::Array(values)=>{values.try_reserve_exact(count).map_err(|_|allocation())?;values.capacity()*size},DslValue::Object(values)=>{values.try_reserve_exact(count).map_err(|_|allocation())?;values.capacity()*size},_=>unreachable!()};self.admit(actual)
 }
 fn token(&mut self,token:Token)->Result<(),PackRefusal>{
  match(self.header,token){
   (0,Token::Begin{kind:Container::Record,count:1})=>{self.header=1;return Ok(())},
   (1,Token::Unsigned{role:Role::FieldId,value:1})=>{self.header=2;return Ok(())},
   (2,Token::Tag{value:0x11,..})=>{self.header=3;return Ok(())},
   (0..=2,_)=>return Err(invalid("expected exactly one declared Value field")),
   _=>{}
  }
  match token{
   Token::Tag{value,..}=>match value{0x01=>self.attach(DslValue::Bool(false)),0x02=>self.attach(DslValue::Bool(true)),0x12=>self.attach(DslValue::Null),0x03..=0x08|0x0c|0x10=>Ok(()),_=>Err(invalid("non-intrinsic value tag"))},
   Token::Signed(value)=>self.attach(DslValue::Number(Number::Int(value))),Token::F64(value)=>self.attach(DslValue::Number(Number::Float(f64::from_bits(value)))),
   Token::Unsigned{role:Role::Unsigned,value}=>self.attach(DslValue::Number(Number::UInt(value))),
   Token::Unsigned{role:Role::StringLength,value}=>{let length=usize::try_from(value).map_err(|_|capacity())?;self.text(length)?;self.copy=if length==0{Copy::Ready}else{Copy::Inline(length)};Ok(())},
   Token::Unsigned{role:Role::BytesLength,value}=>{let length=usize::try_from(value).map_err(|_|capacity())?;if length>self.remaining(){return Err(capacity())}self.owned.scalar=DslValue::Bytes(Vec::new());let DslValue::Bytes(bytes)=&mut self.owned.scalar else{unreachable!()};bytes.try_reserve_exact(length).map_err(|_|allocation())?;let actual=bytes.capacity();self.admit(actual)?;self.copy=if length==0{Copy::Ready}else{Copy::Bytes(length)};Ok(())},
   Token::Unsigned{role:Role::Symbol,value}=>{let bytes=self.cursor.symbol_utf8_bytes(value)?;let count=self.cursor.symbol_chars(value)?;self.text(bytes)?;self.copy=if count==0{Copy::Ready}else{Copy::Symbol{symbol:value,index:0,count}};Ok(())},
   Token::StringChar(character)=>{let Copy::Inline(remaining)=self.copy else{return Err(invalid("unexpected text scalar"))};let DslValue::String(text)=&mut self.owned.scalar else{return Err(invalid("missing text owner"))};let width=character.len_utf8();if width>remaining||text.len()+width>text.capacity(){return Err(invalid("text exceeds admitted owner"))}text.push(character);self.copy=if remaining==width{Copy::Ready}else{Copy::Inline(remaining-width)};Ok(())},
   Token::Byte(byte)=>{let Copy::Bytes(remaining)=self.copy else{return Err(invalid("unexpected octet"))};let DslValue::Bytes(bytes)=&mut self.owned.scalar else{return Err(invalid("missing octet owner"))};if remaining==0||bytes.len()==bytes.capacity(){return Err(invalid("octet exceeds admitted owner"))}bytes.push(byte);self.copy=if remaining==1{Copy::Ready}else{Copy::Bytes(remaining-1)};Ok(())},
   Token::Begin{kind,count}=>self.begin(kind,count),
   Token::End(Container::Record)=>{if !self.owned.frames.is_empty()||self.owned.output.is_none()||self.cursor.consumed_bytes()!=self.cursor.body_bytes(self.input.as_ref().unwrap().bytes()){return Err(invalid("incomplete or trailing intrinsic value"))}self.header=4;Ok(())},
   Token::End(kind)=>{let Some(frame)=self.owned.frames.last()else{return Err(invalid("unmatched container end"))};if frame.remaining!=0||frame.key.is_some()||!matches!((&frame.value,kind),(DslValue::Array(_),Container::List)|(DslValue::Object(_),Container::Map)){return Err(invalid("incomplete container"))}let frame=self.owned.frames.pop().unwrap();self.attach(frame.value)},
   Token::Complete{..}=>{if self.header!=4{return Err(invalid("incomplete intrinsic record"))}self.complete=true;Ok(())},
   _=>Err(invalid("unexpected intrinsic token"))
  }
 }
 fn one(&mut self)->Result<(),PackRefusal>{
  if self.owned.frames.capacity()==0{
   let bytes=self.maximum_frames.checked_mul(size_of::<Frame>()).filter(|bytes|*bytes<=self.remaining()).ok_or_else(capacity)?;let _=bytes;
   self.owned.frames.try_reserve_exact(self.maximum_frames).map_err(|_|allocation())?;let actual=self.owned.frames.capacity()*size_of::<Frame>();return self.admit(actual)
  }
  match self.copy{
   Copy::Ready=>return self.finish_scalar(),
   Copy::Symbol{symbol,index,count}=>{let character=self.cursor.symbol_char(symbol,index)?.ok_or_else(||invalid("missing symbol scalar"))?;let DslValue::String(text)=&mut self.owned.scalar else{return Err(invalid("missing symbol owner"))};if text.len()+character.len_utf8()>text.capacity(){return Err(invalid("symbol exceeds admitted owner"))}text.push(character);self.copy=if index+1==count{Copy::Ready}else{Copy::Symbol{symbol,index:index+1,count}};return Ok(())},
   _=>{}
  }
  let before=self.cursor.allocated_bytes();let result=self.cursor.one(self.input.as_ref().unwrap().bytes(),self.remaining());let born=self.cursor.allocated_bytes().checked_sub(before).ok_or_else(||invalid("parser allocation custody regressed"))?;self.admit(born)?;self.offset=self.cursor.input_bytes();if let Some(token)=result?{self.token(token)?}
  Ok(())
 }
 /// ⏱️ The parser and materializer retain their positions when the finite work grant ends.
 pub fn advance(&mut self,maximum_units:usize,cancelled:bool)->Result<RetainedIntrinsicStep,PackRefusal>{
  if self.closing{return Err(invalid("advance after close"))}if let Some(error)=&self.fault{return Err(error.clone())}
  if cancelled{let error=PackRefusal::ValueRefusal(ValueError::literal(ValueRefusalKind::Canceled,"retained intrinsic decoding canceled"));self.fault=Some(error.clone());return Err(error)}
  let mut units=0;while units<maximum_units&&!self.complete{units+=1;if let Err(error)=self.one(){self.fault=Some(error.clone());return Err(error)}}
  Ok(RetainedIntrinsicStep{units,input_bytes:self.offset,admitted_bytes:self.admitted,complete:self.complete})
 }
 pub fn take_output(&mut self)->Option<DslValue>{if self.complete&&!self.closing&&self.fault.is_none(){self.owned.output.take()}else{None}}
 pub fn next_close_copy_byte_demand(&self)->usize{if let Some(owner)=&self.retiring{if !owner.terminal_is_empty(){return owner.next_copy_byte_demand()}}self.cursor.next_copy_demand()}
 pub fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{if let Some(owner)=&self.retiring{if !owner.terminal_is_empty(){return owner.next_capacity_byte_demand(body)}}self.cursor.next_capacity_demand(body)}
 pub fn next_close_release_byte_demand(&self)->Result<usize,PackRefusal>{if let Some(owner)=&self.retiring{if !owner.terminal_is_empty(){return owner.next_release_byte_demand().map_err(PackRefusal::from)}}self.cursor.next_release_demand()}
 pub fn next_close_depth_demand(&self)->Result<usize,ValueError>{if let Some(owner)=&self.retiring{if !owner.terminal_is_empty(){return owner.next_depth_demand()}}self.cursor.next_depth_demand()}
 /// ♻️ Independent work, capacity, release and depth grants close every partially born owner.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,PackRefusal>{
  if self.closed{return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()))}if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()))}
  if !self.closing{if grant.maximum_depth==0{return Err(PackRefusal::ValueRefusal(ValueError::literal(ValueRefusalKind::DepthLimit,"retained intrinsic close depth")))}if let Some(RetainedIntrinsicInput::OwnedBytes(bytes))=self.input.take(){self.owned.input=bytes}let owned=std::mem::take(&mut*self.owned);match ControlledRetirement::new(owned){Ok(owner)=>self.retiring=Some(owner),Err((error,owned))=>{*self.owned=owned;return Err(PackRefusal::from(error))}}self.closing=true;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}
  if let Some(owner)=&mut self.retiring{if !owner.terminal_is_empty(){return owner.step(grant).map_err(PackRefusal::from)}}
  let step=self.cursor.close_step(grant)?;if matches!(step,RetainedCloneStep::Complete(_)){self.retiring.take();self.closed=true}Ok(step)
 }
 pub fn terminal_is_empty(&self)->bool{self.closed&&self.cursor.terminal_is_empty()&&self.retiring.is_none()&&self.input.is_none()}
}
impl<D:Driver> Drop for Materializer<'_,D>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"retained intrinsic operation abandoned before terminal close");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.owned)}}}}

trait Driver:Sized{
 fn new(limits:PackLimits,length:usize,maximum:usize)->Result<Self,PackRefusal>;
 fn one(&mut self,input:&[u8],maximum:usize)->Result<Option<Token>,PackRefusal>;
 fn allocated_bytes(&self)->usize;
 fn input_bytes(&self)->usize;
 fn consumed_bytes(&self)->u64;
 fn body_bytes(&self,input:&[u8])->u64;
 fn symbol_utf8_bytes(&self,symbol:u64)->Result<usize,PackRefusal>;
 fn symbol_chars(&self,symbol:u64)->Result<usize,PackRefusal>;
 fn symbol_char(&self,symbol:u64,index:usize)->Result<Option<char>,PackRefusal>;
 fn next_copy_demand(&self)->usize{0}
 fn next_capacity_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(1)}
 fn next_release_demand(&self)->Result<usize,PackRefusal>;
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,PackRefusal>;
 fn terminal_is_empty(&self)->bool;
}
fn close_receipt(step:crate::format::RetainedPackCloseStep)->RetainedCloneStep{match step{crate::format::RetainedPackCloseStep::Complete=>RetainedCloneStep::Complete(Default::default()),crate::format::RetainedPackCloseStep::Pending{released_items,released_bytes}=>RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:released_items,released_bytes,..Default::default()})}}
struct BodyDriver{cursor:RetainedRecordBodyCursor,offset:usize,sealed:bool}
impl Driver for BodyDriver{
 fn new(limits:PackLimits,length:usize,maximum:usize)->Result<Self,PackRefusal>{let symbols=length.min(limits.max_symbols as usize);Ok(Self{cursor:RetainedRecordBodyCursor::try_new_intrinsic(limits,symbols,length,length,maximum)?,offset:0,sealed:false})}
 fn one(&mut self,input:&[u8],maximum:usize)->Result<Option<Token>,PackRefusal>{
  if let Some(bytes)=self.cursor.next_allocation_bytes()?{if bytes>maximum{return Err(capacity())}self.cursor.reserve_allocation(bytes).map_err(|error|error.fault)?;return Ok(None)}
  if let Some(token)=self.cursor.grant()?{return Ok(match token{RetainedRecordBodyToken::Value(token)=>Some(token),_=>None})}
  if self.cursor.ingress_ready(){if self.offset<input.len(){self.cursor.admit_byte(self.offset as u64,input[self.offset]).map_err(|_|invalid("input handback"))?;self.offset+=1}else if !self.sealed{self.cursor.seal(self.offset as u64)?;self.sealed=true}else{return Err(invalid("terminal progress unavailable"))}}Ok(None)
 }
 fn allocated_bytes(&self)->usize{self.cursor.allocated_bytes()}
 fn input_bytes(&self)->usize{self.offset}
 fn consumed_bytes(&self)->u64{self.cursor.consumed_bytes()}
 fn body_bytes(&self,input:&[u8])->u64{input.len()as u64}
 fn symbol_utf8_bytes(&self,symbol:u64)->Result<usize,PackRefusal>{self.cursor.symbol_utf8_bytes(symbol)}
 fn symbol_chars(&self,symbol:u64)->Result<usize,PackRefusal>{self.cursor.symbol_chars(symbol)}
 fn symbol_char(&self,symbol:u64,index:usize)->Result<Option<char>,PackRefusal>{self.cursor.symbol_char(symbol,index)}
 fn next_release_demand(&self)->Result<usize,PackRefusal>{Ok(self.cursor.next_release_allocation_bytes()?.unwrap_or(0))}
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,PackRefusal>{Ok(close_receipt(self.cursor.close_step(grant.maximum_items,grant.maximum_release_bytes)?))}
 fn terminal_is_empty(&self)->bool{self.cursor.terminal_is_empty()}
}
macro_rules! operation{($name:ident,$driver:ty)=>{
 /// 🎮️ Retains explicit intrinsic framing, finite work and partial ownership through terminal close.
 pub struct $name<'a>(Materializer<'a,$driver>);
 impl<'a> $name<'a>{
  pub fn new(input:RetainedIntrinsicInput<'a>,limits:PackLimits,maximum:usize)->Result<Self,(PackRefusal,RetainedIntrinsicInput<'a>)>{Materializer::new(input,limits,maximum).map(Self)}
  pub fn advance(&mut self,units:usize,cancelled:bool)->Result<RetainedIntrinsicStep,PackRefusal>{self.0.advance(units,cancelled)}
  pub fn input(&self)->Option<&[u8]>{self.0.input()}
  pub fn admitted_bytes(&self)->usize{self.0.admitted_bytes()}
  pub fn take_output(&mut self)->Option<DslValue>{self.0.take_output()}
  pub fn next_close_copy_byte_demand(&self)->usize{self.0.next_close_copy_byte_demand()}
  pub fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{self.0.next_close_capacity_byte_demand(body)}
  pub fn next_close_release_byte_demand(&self)->Result<usize,PackRefusal>{self.0.next_close_release_byte_demand()}
  pub fn next_close_depth_demand(&self)->Result<usize,ValueError>{self.0.next_close_depth_demand()}
  pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,PackRefusal>{self.0.close_step(grant)}
  pub fn terminal_is_empty(&self)->bool{self.0.terminal_is_empty()}
 }
}}
operation!(RetainedIntrinsicBody,BodyDriver);

#[path="📦️document/🦀️.rs"]
mod document;
use document::DocumentDriver;
operation!(RetainedIntrinsicDocument,DocumentDriver);
#[cfg(test)]
pub(super) fn schema_graph()->&'static[u8]{document::SCHEMA_GRAPH}
