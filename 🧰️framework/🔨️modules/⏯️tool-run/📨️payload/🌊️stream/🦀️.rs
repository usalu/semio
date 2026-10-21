//! 🌊️ Closed payload updates and immutable original page directories expose indexed reads without flattening.
use crate::ToolRunIdentity;
use semio_framework_value::{RetireOwned, ValueError, ValueRefusalKind};
use semio_framework_value_persistent::list::persistent::PersistentList;

pub const TOOL_RUN_PAYLOAD_CHUNK_BYTES:usize=65536;
pub const TOOL_RUN_PAYLOAD_READ_BYTES:usize=64;

/// 📨️ Replacement and ordered publication ranges have distinct closed ownership contracts.
#[derive(Clone,Copy,Debug,PartialEq,Eq,RetireOwned,serde::Serialize,serde::Deserialize)]
#[serde(tag="kind",rename_all="camelCase",rename_all_fields="camelCase",deny_unknown_fields)]
pub enum ToolRunPayloadUpdate{
 Replace,
 Range{publication:u64,total_bytes:u64,offset:u64,chunk_bytes:u32,#[serde(rename="final")]final_chunk:bool},
}
fn invalid(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,reason)}
impl ToolRunPayloadUpdate{
 pub fn wire_field_count(self)->usize{match self{Self::Replace=>1,Self::Range{..}=>6}}
 pub fn wire_field(self,id:usize)->Option<u64>{match(self,id){(Self::Replace,1)=>Some(0),(Self::Range{..},1)=>Some(1),(Self::Range{publication,..},2)=>Some(publication),(Self::Range{total_bytes,..},3)=>Some(total_bytes),(Self::Range{offset,..},4)=>Some(offset),(Self::Range{chunk_bytes,..},5)=>Some(u64::from(chunk_bytes)),(Self::Range{final_chunk,..},6)=>Some(u64::from(final_chunk)),_=>None}}
 /// 🛡️ A transport payload must end exactly at its declared final extent and use full intermediate pages.
 pub fn validate(self,length:usize)->Result<(),ValueError>{
  if let Self::Range{publication,total_bytes,offset,chunk_bytes,final_chunk}=self{
   let extent=usize::try_from(total_bytes).map_err(|_|invalid("payload publication extent exceeds native address range"))?;
   let offset=usize::try_from(offset).map_err(|_|invalid("payload publication offset exceeds native address range"))?;
   let stride=chunk_bytes as usize;
   let end=offset.checked_add(length).ok_or_else(||invalid("payload publication range overflow"))?;
   if publication==0||extent==0||stride==0||stride>TOOL_RUN_PAYLOAD_CHUNK_BYTES||offset%stride!=0||length==0||length>stride||end>extent||final_chunk!=(end==extent)||!final_chunk&&length!=stride{return Err(invalid("payload publication range or finality is inconsistent"));}
  }
  Ok(())
 }
}

/// 🗂️ Captured immutable directories retain every original page across successor publication and cancellation.
#[derive(RetireOwned)]
pub struct ToolRunPayloadStream{
 pub(super) identity:ToolRunIdentity,pub(super) publication:u64,pub(super) total_bytes:usize,pub(super) received_bytes:usize,
 pub(super) chunk_bytes:usize,pub(super) pages:PersistentList<Vec<u8>>,
}
impl ToolRunPayloadStream{
 /// 🪪️ A generation and model revision fence is required before the directory lends any result bytes.
 pub fn borrowed_view(&self,identity:ToolRunIdentity)->Option<ToolRunPayloadView<'_>>{(self.identity==identity).then_some(ToolRunPayloadView{source:self})}
 pub fn contiguous(&self)->Option<&[u8]>{if self.received_bytes!=self.total_bytes||self.pages.len()!=1{return None;}self.pages.get(0).map(Vec::as_slice)}
 pub(super) fn continuation(&self,identity:ToolRunIdentity,update:ToolRunPayloadUpdate,length:usize)->Result<(),ValueError>{
  update.validate(length)?;
  let ToolRunPayloadUpdate::Range{publication,total_bytes,offset,chunk_bytes,..}=update else{return Err(invalid("a replacement cannot continue a ranged publication"));};
  if self.identity!=identity||self.publication!=publication||self.total_bytes!=usize::try_from(total_bytes).map_err(|_|invalid("payload extent exceeds native range"))?||self.chunk_bytes!=chunk_bytes as usize||self.received_bytes!=usize::try_from(offset).map_err(|_|invalid("payload offset exceeds native range"))?||self.received_bytes==self.total_bytes{return Err(invalid("payload continuation changes identity, extent, order or original publication"));}
  Ok(())
 }
}

/// 🔎️ Read-only fixed spans traverse original immutable pages; no complete payload allocation is permitted.
#[derive(Clone,Copy)]
pub struct ToolRunPayloadView<'a>{source:&'a ToolRunPayloadStream}
impl ToolRunPayloadView<'_>{
 pub fn publication(&self)->u64{self.source.publication}
 pub fn total_bytes(&self)->usize{self.source.total_bytes}
 pub fn received_bytes(&self)->usize{self.source.received_bytes}
 pub fn complete(&self)->bool{self.source.received_bytes==self.source.total_bytes}
 pub fn page_count(&self)->usize{self.source.pages.len()}
 pub fn read_fixed<const N:usize>(&self,offset:usize)->Option<[u8;N]>{
  if N>TOOL_RUN_PAYLOAD_READ_BYTES||offset.checked_add(N)?>self.received_bytes(){return None;}
  let mut result=[0;N];let mut copied=0;
  while copied<N{let position=offset+copied;let page=self.source.pages.get(position/self.source.chunk_bytes)?;let start=position%self.source.chunk_bytes;let length=(N-copied).min(page.len().checked_sub(start)?);if length==0{return None;}result[copied..copied+length].copy_from_slice(&page[start..start+length]);copied+=length;}
  Some(result)
 }
}

#[path="🧳️update/🦀️.rs"]
pub mod update;
#[path="📡️wire/🦀️.rs"]pub mod wire;
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
