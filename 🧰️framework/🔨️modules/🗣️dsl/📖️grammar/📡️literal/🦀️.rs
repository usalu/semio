//! 📡️ Iterative literal fields and named-record protocol admission under one cumulative control.
use super::{ProtocolFile,ProtocolTrace,ProtocolMismatch,Framing,Block,Field,Prim,Count,Cond,CondOp,mismatch};
use semio_framework_value::native_decoding::NativeDecodeControl;
enum Frame<'s>{Record{fields:&'s[Field],position:usize,env:Vec<(&'s str,u64)>},Primitive{ty:&'s Prim,name:Option<&'s str>},Array{ty:&'s Prim,remaining:usize}}
struct Cursor<'s,'i,'a,'p>{spec:&'s ProtocolFile,bytes:&'i[u8],position:usize,frames:Vec<Frame<'s>>,control:&'a mut NativeDecodeControl<'p>,work:usize}
impl<'s,'i,'a,'p> Cursor<'s,'i,'a,'p>{
 fn fault(&self,message:impl Into<String>)->ProtocolMismatch{mismatch(self.position,message)}
 fn push(&mut self,frame:Frame<'s>)->Result<(),ProtocolMismatch>{if self.frames.len()==self.frames.capacity(){self.control.charge(64*std::mem::size_of::<Frame<'s>>()).map_err(|error|self.fault(error.into_message()))?;self.frames.try_reserve_exact(64).map_err(|_|self.fault("literal protocol frontier allocation"))?;}self.frames.push(frame);Ok(())}
 fn take(&mut self,length:usize)->Result<&'i[u8],ProtocolMismatch>{let end=self.position.checked_add(length).filter(|end|*end<=self.bytes.len()).ok_or_else(||self.fault("truncated literal protocol field"))?;let result=&self.bytes[self.position..end];for chunk in result.chunks(256){self.position+=chunk.len();self.control.advance(chunk.len()).map_err(|error|self.fault(error.into_message()))?;}Ok(result)}
 fn varint(&mut self)->Result<u64,ProtocolMismatch>{let mut value=0u64;for index in 0..10{let byte=self.take(1)?[0];if index==9&&byte>1{return Err(self.fault("literal varint exceeds u64"))}value|=u64::from(byte&127)<<(index*7);if byte&128==0{if index!=0&&byte==0{return Err(self.fault("literal varint is not minimally encoded"))}return Ok(value)}}Err(self.fault("unterminated literal varint"))}
 fn lookup(&self,name:&str)->Result<u64,ProtocolMismatch>{for frame in self.frames.iter().rev(){if let Frame::Record{env,..}=frame{return env.iter().rev().find(|(key,_)|*key==name).map(|(_,value)|*value).ok_or_else(||self.fault("literal condition references an absent field"))}}Err(self.fault("literal field has no record scope"))}
 fn present(&self,cond:&Cond)->Result<bool,ProtocolMismatch>{let value=self.lookup(&cond.field)?;Ok(match cond.op{CondOp::Eq=>value==cond.value,CondOp::Ne=>value!=cond.value,CondOp::Lt=>value<cond.value,CondOp::Le=>value<=cond.value,CondOp::Gt=>value>cond.value,CondOp::Ge=>value>=cond.value,CondOp::BitsSet=>value&cond.value==cond.value})}
 fn bind(&mut self,name:Option<&'s str>,value:u64)->Result<(),ProtocolMismatch>{if let Some(name)=name{for frame in self.frames.iter_mut().rev(){if let Frame::Record{env,..}=frame{if env.len()==env.capacity(){return Err(mismatch(self.position,"literal record scope exceeds declared fields"))}env.push((name,value));break}}}Ok(())}
 fn reference(&mut self,name:&str,binding:Option<&'s str>)->Result<(),ProtocolMismatch>{
  let mut definition=None;for block in &self.spec.blocks{self.control.checkpoint().map_err(|error|self.fault(error.into_message()))?;let candidate=match block{Block::Record{name:candidate,..}|Block::Struct{name:candidate,..}|Block::Enum{name:candidate,..}=>candidate,_=>continue};if candidate==name{if definition.is_some(){return Err(self.fault("duplicate literal protocol definition"))}definition=Some(block);}}
  match definition.ok_or_else(||self.fault("unresolved literal protocol definition"))?{
   Block::Record{fields,..}|Block::Struct{fields,..}=>{let env=self.control.allocate_vec::<(&'s str,u64)>(fields.len()).map_err(|error|self.fault(error.into_message()))?;self.push(Frame::Record{fields,position:0,env})},
   Block::Enum{variants,..}=>{let value=self.varint()?;let mut admitted=false;for(_,candidate)in variants{if *candidate==value{admitted=true;}self.control.checkpoint().map_err(|error|self.fault(error.into_message()))?;}if !admitted{return Err(self.fault("literal enum discriminant is undeclared"))}self.bind(binding,value)},
   _=>Err(self.fault("literal reference is not a declared type"))
  }
 }
 fn primitive(&mut self,ty:&'s Prim,name:Option<&'s str>)->Result<(),ProtocolMismatch>{
  let value=match ty{
   Prim::U8|Prim::Tag=>Some(u64::from(self.take(1)?[0])),
   Prim::U16=>Some(u64::from(u16::from_le_bytes(self.take(2)?.try_into().unwrap()))),Prim::U16Be=>Some(u64::from(u16::from_be_bytes(self.take(2)?.try_into().unwrap()))),
   Prim::U32|Prim::I32|Prim::F32=>Some(u64::from(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))),Prim::U32Be|Prim::I32Be|Prim::F32Be=>Some(u64::from(u32::from_be_bytes(self.take(4)?.try_into().unwrap()))),
   Prim::U64|Prim::I64|Prim::F64=>Some(u64::from_le_bytes(self.take(8)?.try_into().unwrap())),Prim::U64Be|Prim::I64Be|Prim::F64Be=>Some(u64::from_be_bytes(self.take(8)?.try_into().unwrap())),
   Prim::Varint|Prim::Zigzag=>Some(self.varint()?),
   Prim::Bytes|Prim::Utf8=>{let length=usize::try_from(self.varint()?).map_err(|_|self.fault("literal field width exceeds platform"))?;let bytes=self.take(length)?;if matches!(ty,Prim::Utf8){self.control.borrow_text(bytes).map_err(|error|self.fault(error.into_message()))?;}None},
   Prim::Fixed(length)=>{self.take(*length)?;None},
   Prim::Array(inner,count)=>{let length=match count{Count::Fixed(count)=>*count,Count::Varint=>usize::try_from(self.varint()?).map_err(|_|self.fault("literal array count exceeds platform"))?,Count::Field(field)=>usize::try_from(self.lookup(field)?).map_err(|_|self.fault("literal array count exceeds platform"))?};self.push(Frame::Array{ty:inner,remaining:length})?;None},
   Prim::Ref(reference)=>{self.reference(reference,name)?;None},
   Prim::MarkerScan(_)|Prim::Endian(_)=>return Err(self.fault("literal protocol forbids stream-state primitives"))
  };if let Some(value)=value{self.bind(name,value)?;}Ok(())
 }
 fn run(&mut self)->Result<ProtocolTrace,ProtocolMismatch>{
  self.control.begin_stage(self.bytes.len()).map_err(|error|self.fault(error.into_message()))?;self.reference(&self.spec.start,None)?;
  while let Some(frame)=self.frames.pop(){self.work=self.work.checked_add(1).ok_or_else(||self.fault("literal protocol work overflow"))?;if self.work%256==0{self.control.checkpoint().map_err(|error|self.fault(error.into_message()))?;}
   match frame{
    Frame::Record{fields,position,env}=>{if let Some(field)=fields.get(position){self.push(Frame::Record{fields,position:position+1,env})?;if field.cond.as_ref().map(|cond|self.present(cond)).transpose()?.unwrap_or(true){self.push(Frame::Primitive{ty:&field.ty,name:Some(&field.name)})?;}}},
    Frame::Primitive{ty,name}=>self.primitive(ty,name)?,
    Frame::Array{ty,remaining}=>{if remaining!=0{self.push(Frame::Array{ty,remaining:remaining-1})?;self.push(Frame::Primitive{ty,name:None})?;}}
   }
  }
  if self.position!=self.bytes.len(){return Err(self.fault("trailing bytes after literal protocol root"))}self.control.checkpoint().map_err(|error|self.fault(error.into_message()))?;Ok(ProtocolTrace{consumed:self.position})
 }
}
/// 🚦️ Walks the explicitly declared binary root with admitted scopes and interior cancellation.
pub fn walk_literal_protocol_controlled(spec:&ProtocolFile,bytes:&[u8],control:&mut NativeDecodeControl<'_>)->Result<ProtocolTrace,ProtocolMismatch>{
 if !matches!(spec.framing,Framing::Literal){return Err(mismatch(0,"literal cursor requires explicit literal framing"))}
 for block in &spec.blocks{control.checkpoint().map_err(|error|mismatch(0,error.into_message()))?;if !matches!(block,Block::Record{..}|Block::Struct{..}|Block::Enum{..}){return Err(mismatch(0,"literal framing only admits named type definitions"))}}
 let mut cursor=Cursor{spec,bytes,position:0,frames:Vec::new(),control,work:0};cursor.run()
}
