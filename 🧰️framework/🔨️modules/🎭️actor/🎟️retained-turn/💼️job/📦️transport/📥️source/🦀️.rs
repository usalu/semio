//! 📥️ A persistent original source walk measures once and copies only caller-funded encoded octets.
use super::{OriginalJobSource,OriginalJobSourceCursor,RetainedTurnInput,ValueError,ValueRefusalKind};
use semio_framework_job::StepContext;
use semio_framework_value::{RetainedCloneProgress,RetirementDemand};
use std::mem::ManuallyDrop;
#[path="📦️frame/🦀️.rs"]
mod frame;
pub use frame::OriginalJobSourceWireHeader;

#[derive(Clone,Copy)]
struct Position{phase:u8,index:usize}
impl Position{
 fn new()->Self{Self{phase:0,index:0}}
 fn next(&mut self,source:&OriginalJobSource)->Option<Token>{loop{let token=match self.phase{
  0=>Token::literal(b"{\"job\":"),1=>Token::number(source.job,false),2=>Token::literal(b",\"kind\":\""),
  3=>{if let Some(byte)=source.kind.as_bytes().get(self.index){self.index+=1;return Some(Token::escaped(*byte))}Token::literal(b"\",\"input\":" )},
  4=>Token::literal(if source.input.is_some(){b"["}else{b"null"}),
  5=>{if let Some(byte)=source.input.as_ref().and_then(|bytes|bytes.get(self.index)){let token=Token::number(u64::from(*byte),self.index!=0);self.index+=1;return Some(token)}Token::literal(if source.input.is_some(){b"]"}else{b""})},
  6=>Token::literal(b",\"checkpoint\":"),7=>Token::literal(if source.checkpoint.is_some(){b"["}else{b"null"}),
  8=>{if let Some(byte)=source.checkpoint.as_ref().and_then(|bytes|bytes.get(self.index)){let token=Token::number(u64::from(*byte),self.index!=0);self.index+=1;return Some(token)}Token::literal(if source.checkpoint.is_some(){b"]"}else{b""})},
  9=>Token::literal(b"}"),_=>return None,
 };self.phase+=1;self.index=0;if token.length!=0{return Some(token)}}}
}
#[derive(Clone,Copy)]
struct Token{bytes:[u8;32],length:usize,offset:usize}
impl Token{
 fn empty()->Self{Self{bytes:[0;32],length:0,offset:0}}
 fn literal(bytes:&[u8])->Self{let mut token=Self::empty();token.bytes[..bytes.len()].copy_from_slice(bytes);token.length=bytes.len();token}
 fn number(mut value:u64,comma:bool)->Self{let mut digits=[0;20];let mut start=20;loop{start-=1;digits[start]=b'0'+(value%10)as u8;value/=10;if value==0{break}}let mut token=Self::empty();let prefix=usize::from(comma);if comma{token.bytes[0]=b',';}token.bytes[prefix..prefix+20-start].copy_from_slice(&digits[start..]);token.length=prefix+20-start;token}
 fn escaped(byte:u8)->Self{match byte{b'"'=>Self::literal(b"\\\""),b'\\'=>Self::literal(b"\\\\"),8=>Self::literal(b"\\b"),9=>Self::literal(b"\\t"),10=>Self::literal(b"\\n"),12=>Self::literal(b"\\f"),13=>Self::literal(b"\\r"),0..=31=>{const HEX:&[u8;16]=b"0123456789abcdef";Self::literal(&[b'\\',b'u',b'0',b'0',HEX[(byte>>4)as usize],HEX[(byte&15)as usize]])},_=>Self::literal(&[byte])}}
}

/// 📦️ The original source and every partial destination remain owned through guest acknowledgement and paid closure.
pub struct OriginalJobSourceWireCursor{source:OriginalJobSourceCursor,bytes:ManuallyDrop<Vec<u8>>,position:Position,token:Token,identity:(u64,u64),length:usize,stage:u8,wire_outstanding:bool,guest_outstanding:bool,closing:bool,wire_pointer:usize,wire_capacity:usize}
impl OriginalJobSourceWireCursor{
 pub fn new(source:OriginalJobSource,input:RetainedTurnInput)->Result<Self,(ValueError,OriginalJobSource)>{let source=OriginalJobSourceCursor::new(source,input)?;Ok(Self{source,bytes:ManuallyDrop::new(Vec::new()),position:Position::new(),token:Token::empty(),identity:(input.operation,input.generation),length:0,stage:0,wire_outstanding:false,guest_outstanding:false,closing:false,wire_pointer:0,wire_capacity:0})}
 pub fn original(&self)->Option<&OriginalJobSource>{self.source.original()}
 pub fn prepared_bytes(&self)->Option<&[u8]>{(self.stage==3&&!self.wire_outstanding).then_some(self.bytes.as_slice())}
 fn validate(&self,cx:&StepContext<'_>)->Result<(),ValueError>{cx.original_source_parent()?;if self.identity!=(cx.operation().0,cx.generation().0){return Err(invalid("original source wire belongs to another business caller"))}Ok(())}
 pub fn advance(&mut self,cx:&mut StepContext<'_>)->Result<bool,ValueError>{
  self.validate(cx)?;if self.closing||self.guest_outstanding{return Ok(false)}if self.stage==3{return Ok(true)}let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}let source=self.source.original().ok_or_else(||invalid("original source wire lost its source"))?;
  if self.stage==0{let mut position=self.position;let mut length=self.length;let mut ended=false;for _ in 0..32{if cx.should_yield(){break}let token=position.next(source);cx.consume_fuel(1);match token{Some(token)=>length=length.checked_add(token.length).ok_or_else(||invalid("original source wire extent overflow"))?,None=>{ended=true;break}}}cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;self.position=position;self.length=length;if ended{self.stage=1}return Ok(false)}
  if self.stage==1{if grant.maximum_capacity_bytes<self.length{return Ok(false)}self.bytes.try_reserve_exact(self.length).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original source wire allocation failed"))?;cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:self.bytes.capacity(),..Default::default()})?;cx.consume_fuel(1);self.position=Position::new();self.stage=2;return Ok(false)}
  let mut position=self.position;let mut token=self.token;let mut buffer=[0;64];let mut count=0;let limit=grant.maximum_copy_bytes.min(buffer.len());if limit==0{return Ok(false)}let mut ended=false;
  while count<limit&&!cx.should_yield(){if token.offset==token.length{if cx.fuel_remaining()<2{break}match position.next(source){Some(next)=>token=next,None=>{ended=true;break}}cx.consume_fuel(1);}let copied=(token.length-token.offset).min(limit-count);buffer[count..count+copied].copy_from_slice(&token.bytes[token.offset..token.offset+copied]);token.offset+=copied;count+=copied;}
  cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:count,..Default::default()})?;cx.consume_fuel(1);self.bytes.extend_from_slice(&buffer[..count]);self.position=position;self.token=token;if ended{if self.bytes.len()!=self.length{return Err(invalid("original source wire measurement changed"))}self.stage=3;}Ok(self.stage==3)
 }
 pub fn take_prepared(&mut self,cx:&mut StepContext<'_>)->Result<Option<Vec<u8>>,ValueError>{self.validate(cx)?;if self.stage!=3||self.wire_outstanding||self.closing||cx.should_yield(){return Ok(None)}let grant=cx.retained_grant();let copied=std::mem::size_of::<Vec<u8>>();if grant.maximum_items==0||grant.maximum_copy_bytes<copied||grant.maximum_depth==0{return Ok(None)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:copied,..Default::default()})?;cx.consume_fuel(1);self.wire_pointer=self.bytes.as_ptr()as usize;self.wire_capacity=self.bytes.capacity();self.wire_outstanding=true;self.guest_outstanding=true;Ok(Some(std::mem::take(&mut*self.bytes)))}
 pub fn return_wire(&mut self,bytes:&mut Option<Vec<u8>>,cx:&mut StepContext<'_>)->Result<bool,ValueError>{self.validate(cx)?;if !self.wire_outstanding||bytes.is_none()||self.bytes.capacity()!=0{return Err(invalid("original source wire return has no outstanding loan"))}let returning=bytes.as_ref().unwrap();if returning.as_ptr()as usize!=self.wire_pointer||returning.capacity()!=self.wire_capacity||returning.len()!=self.length{return Err(invalid("original source wire returned a different backing"))}let grant=cx.retained_grant();let copied=std::mem::size_of::<Vec<u8>>();if cx.fuel_remaining()==0||grant.maximum_items==0||grant.maximum_copy_bytes<copied||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:copied,..Default::default()})?;cx.consume_fuel(1);*self.bytes=bytes.take().unwrap();self.wire_outstanding=false;Ok(true)}
 pub fn acknowledge_source_closed(&mut self,cx:&mut StepContext<'_>)->Result<bool,ValueError>{self.validate(cx)?;if !self.guest_outstanding{return Err(invalid("original source wire has no pending guest acknowledgement"))}let grant=cx.retained_grant();if cx.fuel_remaining()==0||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);self.guest_outstanding=false;Ok(true)}
 pub fn begin_close(&mut self){self.closing=true;self.source.begin_close()}
 pub fn retirement_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if self.wire_outstanding||self.guest_outstanding{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original source wire still belongs to guest custody"))}if self.bytes.capacity()!=0{return Ok(RetirementDemand{release_bytes:self.bytes.capacity(),depth:1,..Default::default()})}self.source.retirement_demands(copy)}
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,ValueError>{self.validate(cx)?;self.begin_close();let grant=cx.retained_grant();let demand=self.retirement_demands(grant.maximum_copy_bytes)?;if self.bytes.capacity()!=0{if cx.fuel_remaining()==0||grant.maximum_items==0||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()})?;cx.consume_fuel(1);drop(std::mem::take(&mut*self.bytes));return Ok(false)}self.source.close_step(cx)}
 pub fn terminal_is_empty(&self)->bool{!self.wire_outstanding&&!self.guest_outstanding&&self.bytes.capacity()==0&&self.source.terminal_is_empty()}
}
impl Drop for OriginalJobSourceWireCursor{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original source wire dropped before guest acknowledgement and paid closure")}}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
