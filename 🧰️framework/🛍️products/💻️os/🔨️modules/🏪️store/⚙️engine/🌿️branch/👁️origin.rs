//! 🌱️ Native provenance reads the identity prefix of already-validated original Store transition rows.
use super::*;
use semio_framework_value::ByteReadSource;
/// 🪪️ Checkpoints originate in Commit or Repin; alternatives originate in Branch.
#[derive(Clone,Copy,PartialEq,Eq)]
pub enum OriginalTransitionOriginKind{Checkpoint,Alternative}
/// 🧭️ A borrowed index probe retains no copied transition, payload, target or mutation identifier.
pub struct OriginalTransitionOrigin{binding:Option<OriginalSourceBinding>,store:usize,generation:u64,revision:[u8;32],target:(usize,usize),kind:OriginalTransitionOriginKind,key:Option<ArtifactHistoryKey>,found:Option<ArtifactHistoryKey>,offset:usize,length:usize,matched:usize,integer:u64,shift:u8,field:u8,wanted:u8,phase:u8}
impl Default for OriginalTransitionOrigin{fn default()->Self{Self{binding:None,store:0,generation:0,revision:[0;32],target:(0,0),kind:OriginalTransitionOriginKind::Checkpoint,key:None,found:None,offset:0,length:0,matched:0,integer:0,shift:0,field:0,wanted:0,phase:0}}}
impl OriginalTransitionOrigin{
 fn varint(&mut self,payload:&impl ByteReadSource)->Result<Option<u64>,ValueError>{let byte=payload.byte_at(self.offset).ok_or_else(||invalid("validated transition identity prefix is truncated"))?;self.offset=self.offset.checked_add(1).ok_or_else(||invalid("transition identity extent overflow"))?;let bits=u64::from(byte&127);if self.shift==63&&bits>1{return Err(invalid("transition identity varint overflows"))}self.integer|=bits<<self.shift;if byte&128==0{if self.shift>0&&bits==0{return Err(invalid("transition identity varint is not minimal"))}let value=self.integer;self.integer=0;self.shift=0;Ok(Some(value))}else{if self.shift>=63{return Err(invalid("transition identity varint is unterminated"))}self.shift+=7;Ok(None)}}
 fn next<P,M>(&mut self,store:&ArtifactStore<P,M>)where P:ToValue+Send+Sync+'static,M:ToValue+FromValue+Mutation<P>+OpBinary+OpText+Send+'static{self.key=self.key.and_then(|key|store.parts_ref().log.envelope.transitions.previous_key(key));self.offset=0;self.length=0;self.matched=0;self.integer=0;self.shift=0;self.field=0;self.phase=1;}
 /// 🔍️ The actual Store already validates transition payloads at admission; this only queries their canonical identity roles.
 pub fn advance<P,M>(&mut self,store:&ArtifactStore<P,M>,target:&str,kind:OriginalTransitionOriginKind,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>where P:ToValue+Send+Sync+'static,M:ToValue+FromValue+Mutation<P>+OpBinary+OpText+Send+'static{
  if let Some(binding)=self.binding{if !binding.matches(cx)||self.store!=store as*const _ as usize||self.generation!=store.generation()||self.revision!=store.content_revision()||self.target!=(target.as_ptr()as usize,target.len())||self.kind!=kind{return Err(invalid("transition origin probe changed its original Store, target or caller").into())}}if self.ready(){return Ok(true)}let d=fixed(384);let binding=OriginalSourceBinding::borrow(cx);action(cx,d,|_|{
   if self.binding.is_none(){self.binding=Some(binding);self.store=store as*const _ as usize;self.generation=store.generation();self.revision=store.content_revision();self.target=(target.as_ptr()as usize,target.len());self.kind=kind;self.key=store.parts_ref().log.envelope.transitions.last_key();self.phase=1;}
   if self.key.is_none(){self.phase=5;return Ok(RetainedCloneProgress{copied_items:1,copied_bytes:384,..Default::default()})}let row=store.parts_ref().log.envelope.transitions.get_key(self.key.unwrap()).ok_or_else(||invalid("transition origin original row disappeared"))?;let payload=&row.diff.payload;
   match self.phase{
    1=>{let schema=semio_framework_os_spr::HISTORY_TRANSITION_SCHEMA;if row.diff.schema.0.len()!=schema.len()||row.diff.schema.0!=schema{self.next(store);}else{self.phase=2;}},
    2=>{if let Some(tag)=self.varint(payload)?{let wanted=match(kind,tag){(OriginalTransitionOriginKind::Checkpoint,2)|(OriginalTransitionOriginKind::Alternative,3)=>Some(0),(OriginalTransitionOriginKind::Checkpoint,5)=>Some(1),_=>None};if let Some(wanted)=wanted{self.wanted=wanted;self.phase=3;}else{self.next(store);}}},
    3=>{if let Some(length)=self.varint(payload)?{let length=usize::try_from(length).map_err(|_|invalid("transition identity length exceeds this platform"))?;let end=self.offset.checked_add(length).filter(|end|*end<=payload.byte_len()).ok_or_else(||invalid("validated transition identity exceeds its payload"))?;if self.field<self.wanted{self.offset=end;self.field+=1;}else if length!=target.len(){self.next(store);}else{self.length=length;self.matched=0;self.phase=4;}}},
    4=>{let count=(self.length-self.matched).min(64);let mut bytes=[0;64];if payload.read_bytes(self.offset+self.matched,&mut bytes[..count])!=count{return Err(invalid("transition identity source fragment disappeared"))}if bytes[..count]!=target.as_bytes()[self.matched..self.matched+count]{self.next(store);}else{self.matched+=count;if self.matched==self.length{self.found=self.key;self.phase=5;}}},
    _=>return Err(invalid("transition origin probe phase changed")),
   }Ok(RetainedCloneProgress{copied_items:1,copied_bytes:384,..Default::default()})
  })?;Ok(self.ready())
 }
 /// 🏁️ A completed result names an original transition ledger row rather than a cloned envelope.
 pub fn ready(&self)->bool{self.phase==5}
 pub fn found(&self)->Option<ArtifactHistoryKey>{self.ready().then_some(self.found).flatten()}
 /// 🛑️ Returning fixed probe coordinates never releases or invents custody of a Store row.
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if self.binding.is_none(){return Ok(true)}if !self.binding.is_some_and(|binding|binding.matches(cx)){return Err(invalid("transition origin closure changed its original caller").into())}close_original_retained_step(cx,fixed(128),|_|{self.binding=None;self.key=None;self.found=None;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:128,..Default::default()}))})?;Ok(self.binding.is_none())}
 pub fn terminal_is_empty(&self)->bool{self.binding.is_none()}
}
