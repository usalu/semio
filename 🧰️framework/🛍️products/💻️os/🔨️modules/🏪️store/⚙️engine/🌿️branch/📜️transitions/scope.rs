//! 🌳️ Native scope validation hashes the genuine implicit trunk and compares original alternative rows in bounded turns.
use super::*;
use semio_framework_hash::Hasher;
/// 🧭️ Fixed probe state borrows actual Store roots and creates no trunk String or copied row.
pub struct OriginalHistoryScope{hash:Hasher,offset:usize,key:Option<ArtifactHistoryKey>,matched:usize,trunk:bool,applies:bool,phase:u8}
impl Default for OriginalHistoryScope{fn default()->Self{Self{hash:Hasher::new(),offset:0,key:None,matched:0,trunk:false,applies:false,phase:0}}}
fn integer(hash:&mut Hasher,mut value:usize){let mut bytes=[0;10];let mut count=0;loop{bytes[count]=(value as u8&127)|if value>=128{128}else{0};count+=1;value>>=7;if value==0{break}}hash.update(&bytes[..count]);}
impl OriginalHistoryScope{
 /// 🔎️ The enclosing original batch source authenticates caller, Store generation and exact immutable scope fields.
 pub fn advance<P,M>(&mut self,store:&ArtifactStore<P,M>,scope:&str,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>where P:ToValue+Send+Sync+'static,M:ToValue+FromValue+Mutation<P>+OpBinary+OpText+Send+'static{if self.ready(){return Ok(true)}let log=store.parts_ref().log;match self.phase{
  0=>{let d=fixed(size_of::<Hasher>()+128);action(cx,d,|_|{let domain="semio.history.trunk";integer(&mut self.hash,domain.len());self.hash.update(domain.as_bytes());integer(&mut self.hash,log.envelope.id.len());self.phase=1;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})})?;},
  1=>{let d=fixed(size_of::<Hasher>()+128);action(cx,d,|_|{let end=(self.offset+64).min(log.envelope.id.len());self.hash.update(&log.envelope.id.as_bytes()[self.offset..end]);self.offset=end;if end==log.envelope.id.len(){self.phase=2;}Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})})?;},
  2=>{let d=fixed(size_of::<Hasher>()+128);action(cx,d,|_|{let digest=self.hash.finalize();let mut id=[0;22];id[..6].copy_from_slice(b"trunk-");const HEX:&[u8;16]=b"0123456789abcdef";for(index,byte)in digest.as_bytes()[..8].iter().enumerate(){id[6+index*2]=HEX[(byte>>4)as usize];id[7+index*2]=HEX[(byte&15)as usize];}self.trunk=scope.as_bytes()==id;self.key=log.envelope.vcs.alternatives.last_key();self.phase=if self.trunk{4}else{3};Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})})?;},
  3=>{action(cx,fixed(256),|_|{let key=self.key.ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"history finalization scope is not an original alternative"))?;let row=log.envelope.vcs.alternatives.get_key(key).ok_or_else(||invalid("original history scope row disappeared"))?;let end=(self.matched+64).min(scope.len());if row.id.len()!=scope.len()||row.id.as_bytes()[self.matched..end]!=scope.as_bytes()[self.matched..end]{self.key=log.envelope.vcs.alternatives.previous_key(key);self.matched=0;}else if end==scope.len(){self.matched=0;self.phase=4;}else{self.matched=end;}Ok(RetainedCloneProgress{copied_items:1,copied_bytes:256,..Default::default()})})?;},
  4=>{action(cx,fixed(256),|_|{if let Some(active)=log.envelope.active_alternative_id.as_deref(){let end=(self.matched+64).min(scope.len());if active.len()!=scope.len()||active.as_bytes()[self.matched..end]!=scope.as_bytes()[self.matched..end]{self.applies=false;self.phase=5;}else if end==scope.len(){self.applies=true;self.phase=5;}else{self.matched=end;}}else{self.applies=self.trunk;self.phase=5;}Ok(RetainedCloneProgress{copied_items:1,copied_bytes:256,..Default::default()})})?;},
  _=>return Err(invalid("native history scope phase changed").into()),
 }Ok(self.ready())}
 pub fn ready(&self)->bool{self.phase==5}
 pub fn applies_to_head(&self)->bool{self.ready()&&self.applies}
}
