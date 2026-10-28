//! 🌱️ Canonical Emit frames stream genuine borrowed byte sources without rebuilding an AppFrame or payload owner.
use semio_framework_value::{ValueError,ValueRefusalKind,paged::ByteReadSource};
/// 🎞️ Field order is the canonical channel tag-10 schema and borrows each original payload allocation.
pub struct AppEmitFrameView<'a>{pub in_reply_to:u64,pub document_ops:&'a dyn ByteReadSource,pub config_ops:&'a dyn ByteReadSource,pub draft_ops:&'a dyn ByteReadSource,pub output:&'a dyn ByteReadSource,pub diagnostics:&'a dyn ByteReadSource,pub child_ops:&'a dyn ByteReadSource}
#[derive(Clone,Copy,PartialEq,Eq)]
struct Identity{reply:u64,epoch:u64,fields:[(usize,usize);6]}
impl AppEmitFrameView<'_>{fn fields(&self)->[&dyn ByteReadSource;6]{[self.document_ops,self.config_ops,self.draft_ops,self.output,self.diagnostics,self.child_ops]}fn identity(&self,epoch:u64)->Identity{Identity{reply:self.in_reply_to,epoch,fields:self.fields().map(|source|(source as*const dyn ByteReadSource as*const()as usize,source.byte_len()))}}}
/// 📏️ Codec progress records only initialized borrowed output; the enclosing original source owns execution and output admission.
#[derive(Clone,Copy,Default,Debug,PartialEq,Eq)]
pub struct AppEmitFrameWireProgress{pub written_bytes:usize,pub complete:bool}
/// 🧵️ A fixed cursor binds every payload owner and the actual enclosing source epoch before its first byte.
#[derive(Default)]
pub struct AppEmitFrameWireCursor{identity:Option<Identity>,field:u8,offset:usize,pending:[u8;10],pending_len:usize,pending_offset:usize,complete:bool,closing:bool}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original Emit frame changed its source fields, epoch or wire extent")}
impl AppEmitFrameWireCursor{
 fn queue(&mut self,mut value:u64){self.pending_len=0;self.pending_offset=0;loop{let byte=(value&127)as u8;value>>=7;self.pending[self.pending_len]=byte|if value!=0{128}else{0};self.pending_len+=1;if value==0{break}}}
 pub fn write(&mut self,source:&AppEmitFrameView<'_>,epoch:u64,output:&mut[u8])->Result<AppEmitFrameWireProgress,ValueError>{if self.closing{return Err(invalid())}let identity=source.identity(epoch);if self.identity.is_some_and(|original|original!=identity){return Err(invalid())}if output.is_empty(){return Ok(AppEmitFrameWireProgress{complete:self.complete,..Default::default()})}self.identity=Some(identity);let maximum=output.len().min(64);let mut written=0;let fields=source.fields();while written<maximum&&!self.complete{if self.pending_offset<self.pending_len{let count=(maximum-written).min(self.pending_len-self.pending_offset);output[written..written+count].copy_from_slice(&self.pending[self.pending_offset..self.pending_offset+count]);self.pending_offset+=count;written+=count;continue}match self.field{0=>{output[written]=10;written+=1;self.field=1;},1=>{self.queue(source.in_reply_to);self.field=2;},field if field<=13=>{let index=usize::from((field-2)/2);if field%2==0{self.queue(u64::try_from(fields[index].byte_len()).map_err(|_|invalid())?);self.field+=1;}else{let count=(maximum-written).min(fields[index].byte_len()-self.offset);if fields[index].read_bytes(self.offset,&mut output[written..written+count])!=count{return Err(invalid())}written+=count;self.offset+=count;if self.offset==fields[index].byte_len(){self.offset=0;self.field+=1;}}},_=>self.complete=true}}if self.field==14&&self.pending_offset==self.pending_len{self.complete=true;}Ok(AppEmitFrameWireProgress{written_bytes:written,complete:self.complete})}
 pub fn is_complete(&self)->bool{self.complete}
 pub fn close(&mut self){self.closing=true;self.identity=None;self.pending_len=0;self.pending_offset=0;}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.identity.is_none()}
}
#[cfg(test)]#[path="🧪️.rs"]mod tests;
