//! 📤️ Canonical SPR framing borrows original payloads without collecting frame or file shadows.
use super::*;
use semio_framework_value::{ByteReadSource,ValueError,ValueRefusalKind,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use semio_framework_replication_codec::Crc32cCursor;
use semio_framework_hash::Hasher;
use std::mem::size_of;
/// 📜️ The semantic owner lends one authentic immutable record payload at its existing ordinal.
pub struct BorrowedSprRecord<'a>{pub kind:u8,pub critical:bool,pub payload:&'a dyn ByteReadSource}
/// 🫳️ The enclosing original owner keeps every field and payload alive through the complete export or cancellation.
pub trait BorrowedSprSource{fn source_identity(&self)->usize;fn record(&self,index:usize)->Result<Option<BorrowedSprRecord<'_>>,ValueError>;}
/// 🎟️ Only initialized final octets and the actual fixed-state work receipt leave one bounded event.
#[derive(Default)]
pub struct BorrowedSprProgress{pub written:usize,pub complete:bool,pub progress:RetainedCloneProgress}
/// 🧵️ A fixed cursor writes identity frames and a single canonical commit over original source payloads.
pub struct BorrowedSprFileCursor{identity:Option<(usize,u64)>,row:usize,phase:u8,offset:usize,header:[u8;32],prefix:[u8;12],prefix_len:usize,length:usize,payload_identity:usize,kind:u8,flags:u8,trailer:[u8;8],commit:[u8;64],frame:Hasher,chain:Hasher,crc:Crc32cCursor,records_len:u64,record_count:u32,commit_frame:bool}
fn invalid(text:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,text)}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn admits(grant:RetainedCloneGrant,d:RetirementDemand)->bool{semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d)}
impl Default for BorrowedSprFileCursor{fn default()->Self{Self{identity:None,row:0,phase:0,offset:0,header:build_header_bytes(semio_framework_replication_wire_core::REQUIRED_HASH_CHAIN,semio_framework_replication_wire_core::OPTIONAL_CANONICAL),prefix:[0;12],prefix_len:0,length:0,payload_identity:0,kind:0,flags:0,trailer:[0;8],commit:[0;64],frame:Hasher::new(),chain:Hasher::new(),crc:Crc32cCursor::new(),records_len:0,record_count:0,commit_frame:false}}}
impl BorrowedSprFileCursor{
 pub fn birth_demands()->RetirementDemand{fixed(size_of::<Self>())}
 pub fn is_complete(&self)->bool{self.phase==6}
 pub fn terminal_is_empty(&self)->bool{self.phase==7&&self.identity.is_none()}
 pub fn next_demands(&self)->RetirementDemand{if self.is_complete()||self.terminal_is_empty(){return Default::default()}fixed(2*size_of::<Hasher>()+size_of::<Crc32cCursor>()+256)}
 fn start(&mut self,kind:u8,critical:bool,length:usize)->Result<(),ValueError>{let body=length.checked_add(2).ok_or_else(||invalid("original SPR frame size overflows"))?;let mut bytes=[0;10];let prefix=fixed_varint(body as u64,&mut bytes);let total=prefix.len().checked_add(body).and_then(|n|n.checked_add(8)).ok_or_else(||invalid("original SPR frame size overflows"))?;u32::try_from(total).map_err(|_|invalid("original SPR frame exceeds u32 back length"))?;self.prefix[..prefix.len()].copy_from_slice(prefix);self.prefix[prefix.len()]=kind;self.prefix[prefix.len()+1]=frame_flags(false,critical,0);self.prefix_len=prefix.len()+2;self.length=length;self.kind=kind;self.flags=self.prefix[prefix.len()+1];self.offset=0;self.frame=Hasher::new();self.crc=Crc32cCursor::new();self.phase=2;Ok(())}
 /// ✍️ The exact source identity and epoch are checked before any output or retained cursor effect.
 pub fn advance(&mut self,source:&dyn BorrowedSprSource,epoch:u64,output:&mut[u8],grant:RetainedCloneGrant)->Result<BorrowedSprProgress,ValueError>{if self.identity.is_some_and(|original|original!=(source.source_identity(),epoch)){return Err(invalid("original SPR file changed its source or frontier"))}if self.terminal_is_empty(){return Err(invalid("closed original SPR file cannot resume"))}if self.is_complete(){return Ok(BorrowedSprProgress{complete:true,..Default::default()})}let demand=self.next_demands();if output.is_empty()||!admits(grant,demand){return Ok(Default::default())}let limit=output.len().min(64);let mut written=0;match self.phase{
 0=>{let count=(32-self.offset).min(limit);output[..count].copy_from_slice(&self.header[self.offset..self.offset+count]);self.offset+=count;written=count;if self.offset==32{let hash=semio_framework_hash::hash(&self.header);self.chain.update(hash.as_bytes());self.offset=0;self.phase=1;}},
 1=>{if let Some(record)=source.record(self.row)?{if record.kind==semio_framework_replication_wire_core::REC_COMMIT{return Err(invalid("original SPR semantic source cannot inject a commit frame"))}self.payload_identity=record.payload as*const dyn ByteReadSource as*const ()as usize;self.start(record.kind,record.critical,record.payload.byte_len())?;}else{self.commit=write_commit_payload(1,0,self.records_len,self.record_count,self.chain.finalize().as_bytes());self.commit_frame=true;self.start(semio_framework_replication_wire_core::REC_COMMIT,true,64)?;}},
 2=>{let count=(self.prefix_len-self.offset).min(limit);let fragment=&self.prefix[self.offset..self.offset+count];output[..count].copy_from_slice(fragment);self.frame.update(fragment);let start=self.offset.max(self.prefix_len-2);if start<self.offset+count{self.crc.update_page(&self.prefix[start..self.offset+count]);}self.offset+=count;written=count;if self.offset==self.prefix_len{self.offset=0;self.phase=3;}},
 3=>{let count=(self.length-self.offset).min(limit);if self.commit_frame{output[..count].copy_from_slice(&self.commit[self.offset..self.offset+count]);}else{let record=source.record(self.row)?.ok_or_else(||invalid("original SPR record disappeared"))?;if record.kind!=self.kind||frame_flags(false,record.critical,0)!=self.flags||record.payload.byte_len()!=self.length||record.payload as*const dyn ByteReadSource as*const ()as usize!=self.payload_identity{return Err(invalid("original SPR record payload authority changed"))}if record.payload.read_bytes(self.offset,&mut output[..count])!=count{return Err(invalid("original SPR payload ended before its declared extent"))}}self.frame.update(&output[..count]);self.crc.update_page(&output[..count]);self.offset+=count;written=count;if self.offset==self.length{self.trailer[..4].copy_from_slice(&self.crc.finish().to_le_bytes());let length=(self.prefix_len+self.length+8)as u32;self.trailer[4..].copy_from_slice(&length.to_le_bytes());self.offset=0;self.phase=4;}},
 4=>{let count=(8-self.offset).min(limit);output[..count].copy_from_slice(&self.trailer[self.offset..self.offset+count]);self.frame.update(&output[..count]);self.offset+=count;written=count;if self.offset==8{if self.commit_frame{self.phase=6;}else{self.chain.update(self.frame.finalize().as_bytes());self.records_len=self.records_len.checked_add((self.prefix_len+self.length+8)as u64).ok_or_else(||invalid("original SPR records extent overflows"))?;self.record_count=self.record_count.checked_add(1).ok_or_else(||invalid("original SPR record count overflows"))?;self.row+=1;self.phase=1;}self.offset=0;}},
 _=>return Err(invalid("original SPR encoding phase changed")),
 }self.identity=Some((source.source_identity(),epoch));Ok(BorrowedSprProgress{written,complete:self.is_complete(),progress:RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}})}
 pub fn close_demands(&self)->RetirementDemand{if self.terminal_is_empty(){Default::default()}else{fixed(size_of::<Self>())}}
 /// ♻️ Fixed state closes without touching or releasing any borrowed semantic source.
 pub fn close(&mut self,grant:RetainedCloneGrant)->RetainedCloneStep{if self.terminal_is_empty(){return RetainedCloneStep::Complete(Default::default())}let d=self.close_demands();if !admits(grant,d){return RetainedCloneStep::Progress(Default::default())}self.identity=None;self.phase=7;self.offset=0;self.payload_identity=0;RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})}
}

#[cfg(test)]
#[path="🧪️tests.rs"]mod tests;
