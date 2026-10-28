//! 🔏️ Revision hashing streams the original edit and its operation chains under the initialization grant.
use semio_framework_os_spr::{Edit,Mutation};
#[cfg(test)]
use semio_framework_os_spr::MutationId;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetirementDemand,ValueError,ValueRefusalKind,FromValue,ToValue};
use std::mem::size_of;
#[cfg(test)]
use crate::CursorRevisionAccumulator;
fn permits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand)}
use semio_framework_os_store_canonical_json::{ArtifactCanonicalRecordDigest,ArtifactCanonicalTreeSource};
use semio_framework_hash::Sha256;
use semio_framework_pack_json::ArtifactCanonicalJsonTree;

pub struct NativeEditDigest{record:ArtifactCanonicalRecordDigest,chains:[[u8;32];3],lane:usize,row:usize,phase:u8,offset:usize,hash:Sha256,result:Option<[u8;32]>}
impl Default for NativeEditDigest{fn default()->Self{Self{record:Default::default(),chains:[[0;32];3],lane:0,row:0,phase:0,offset:0,hash:Sha256::new(),result:None}}}
fn framed_byte(domain:&[u8],parts:&[&[u8]],mut at:usize)->Option<u8>{
 for bytes in [b"semio.artifact.cursor.v2".as_slice(),&(domain.len()as u64).to_be_bytes(),domain]{if at<bytes.len(){return Some(bytes[at])}at-=bytes.len()}
 for part in parts{let length=(part.len()as u64).to_be_bytes();for bytes in [length.as_slice(),*part]{if at<bytes.len(){return Some(bytes[at])}at-=bytes.len()}}None
}
impl NativeEditDigest{
 pub fn digest(&self)->Option<[u8;32]>{self.result}
 pub fn advance<M:ArtifactCanonicalJsonTree>(&mut self,edit:&Edit<M>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{self.advance_with_inverse(edit,&edit.inverse,grant)}
 /// 🪟️ Hashes the actual staged inverse while every other field remains borrowed from the same original edit.
 pub fn advance_with_inverse<M:ArtifactCanonicalJsonTree>(&mut self,edit:&Edit<M>,inverse:&semio_framework_value_persistent::list::PagedList<M,{usize::MAX}>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  if self.result.is_some(){return Ok(Default::default())}
  if self.phase==0{
   if edit.forwards.len()<=1{let receipt=self.record.advance(&OriginalEditView{edit,inverse},b"edit",edit.id.as_bytes(),grant)?.progress();self.result=self.record.digest();return Ok(receipt)}
   let length=match self.lane{0=>edit.forwards.len(),1=>inverse.len(),_=>edit.mutation_meta.len()};
   if self.row==length{let d=RetirementDemand{copy_bytes:2*size_of::<usize>(),depth:1,..Default::default()};if !permits(grant,d){return Ok(Default::default())}self.lane+=1;self.row=0;if self.lane==3{self.phase=1}return Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})}
   if let Some(digest)=self.record.digest(){let d=RetirementDemand{copy_bytes:size_of::<ArtifactCanonicalRecordDigest>()+32+size_of::<usize>(),depth:1,..Default::default()};if !permits(grant,d){return Ok(Default::default())}self.chains[self.lane]=digest;self.row+=1;self.record=Default::default();return Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})}
   let prefix=&self.chains[self.lane];let step=match self.lane{0=>self.record.advance(ArtifactCanonicalTreeSource::view(&edit.forwards[self.row]),b"edit-forward",prefix,grant)?,1=>self.record.advance(ArtifactCanonicalTreeSource::view(&inverse[self.row]),b"edit-inverse",prefix,grant)?,_=>self.record.advance(ArtifactCanonicalTreeSource::view(&edit.mutation_meta[self.row]),b"edit-meta",prefix,grant)?};return Ok(step.progress())
  }
  let demand=RetirementDemand{copy_bytes:64,depth:1,..Default::default()};if !permits(grant,demand){return Ok(Default::default())}
  let actor=[u8::from(edit.actor.is_some())];let finished=[u8::from(edit.finished_at.is_some())];let forwards=(edit.forwards.len()as u64).to_be_bytes();let inverse=(inverse.len()as u64).to_be_bytes();let meta=(edit.mutation_meta.len()as u64).to_be_bytes();let line=[u8::from(edit.line.is_some())];
  let current=self.chains[0];let byte=match self.phase{
   1=>framed_byte(b"edit-chained",&[edit.id.as_bytes(),&actor,edit.actor.as_deref().unwrap_or_default().as_bytes(),edit.started_at.as_bytes(),&finished,edit.finished_at.as_deref().unwrap_or_default().as_bytes(),&forwards,&self.chains[0],&inverse,&self.chains[1],&meta,&self.chains[2]],self.offset),
   2=>framed_byte(b"edit-verb",&[&current,edit.verb.as_deref().unwrap_or_default().as_bytes()],self.offset),
   _=>framed_byte(b"edit-line",&[&current,&line,edit.line.as_deref().unwrap_or_default().as_bytes()],self.offset),
  };
  if let Some(byte)=byte{self.hash.update(&[byte]);self.offset+=1}else{self.chains[0]=self.hash.clone().finalize();self.hash=Sha256::new();self.offset=0;self.phase=match self.phase{1 if edit.verb.is_some()=>2,1|2=>3,_=>4};if self.phase==4{self.result=Some(self.chains[0])}}
  Ok(RetainedCloneProgress{copied_items:1,copied_bytes:64,..Default::default()})
 }
}

#[cfg(test)]
#[path="🧪️.rs"]
mod tests;

/// 🪪️ Borrows authored identities and bounds the only synthesized identifier on the stack.
pub fn with_operation_identity<P,M:Mutation<P>,R>(edit:&Edit<M>,row:usize,work:impl FnOnce(&str)->Result<R,ValueError>)->Result<R,ValueError>{
 if let Some(id)=edit.mutation_meta.get(row).and_then(|meta|meta.mutation_id.as_ref()).or_else(||edit.forwards.get(row).and_then(M::mutation_id)){return work(&id.0)}
 let mut bytes=[0;256];let mut digits=[0;20];let(mut at,mut value)=(digits.len(),row);loop{at-=1;digits[at]=b'0'+(value%10)as u8;value/=10;if value==0{break}}let length=edit.id.len()+1+digits.len()-at;if length>bytes.len(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original operation identity exceeds bounded native capacity"))}bytes[..edit.id.len()].copy_from_slice(edit.id.as_bytes());bytes[edit.id.len()]=b'#';bytes[edit.id.len()+1..length].copy_from_slice(&digits[at..]);work(std::str::from_utf8(&bytes[..length]).unwrap())
}
/// ♻️ Hashes effective replacement bytes directly from their retained map without building identity or payload copies.
pub struct NativeEffectiveDigest{row:usize,phase:u8,offset:usize,hash:Sha256,one:[u8;32],chain:Option<[u8;32]>,result:Option<[u8;32]>}
impl Default for NativeEffectiveDigest{fn default()->Self{Self{row:0,phase:0,offset:0,hash:Sha256::new(),one:[0;32],chain:None,result:None}}}
impl NativeEffectiveDigest{
 pub fn digest(&self)->Option<[u8;32]>{self.result}
 pub fn advance<P,M:Mutation<P>>(&mut self,edit:&Edit<M>,supersessions:&crate::lifted_replay::EffectiveSupersessions,original:[u8;32],grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  if self.result.is_some(){return Ok(Default::default())}let demand=RetirementDemand{copy_bytes:256+size_of::<Self>(),depth:1,..Default::default()};if !permits(grant,demand){return Ok(Default::default())}
  if self.row==edit.forwards.len()&&self.phase==0{if self.chain.is_none(){self.result=Some(original)}else{self.phase=2}return Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})}
  let byte=if self.phase==0{with_operation_identity::<P,M,_>(edit,self.row,|id|{let Some(value)=supersessions.get(id)else{return Ok(None)};let(kind,schema,payload)=match &value.replacement{semio_framework_os_spr::InputReplacement::Input{schema,payload}=>(b"input".as_slice(),schema.as_bytes(),payload.as_slice()),_=>(b"withdrawn".as_slice(),b"".as_slice(),b"".as_slice())};Ok(Some(framed_byte(b"supersession",&[id.as_bytes(),value.transition_id.as_bytes(),kind,schema,payload],self.offset)))})?}else if self.phase==1{Some(framed_byte(b"edit-supersessions",&[&self.chain.unwrap_or([0;32]),&(self.row as u64).to_be_bytes(),&self.one],self.offset))}else{Some(framed_byte(b"edit-superseded",&[&original,&self.chain.unwrap()],self.offset))};
  match byte{None=>self.row+=1,Some(Some(byte))=>{self.hash.update(&[byte]);self.offset+=1},Some(None)=>{let digest=self.hash.clone().finalize();self.hash=Sha256::new();self.offset=0;match self.phase{0=>{self.one=digest;self.phase=1},1=>{self.chain=Some(digest);self.row+=1;self.phase=0},_=>self.result=Some(digest)}}}
  Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})
 }
}

struct OriginalEditView<'a,M>{edit:&'a Edit<M>,inverse:&'a semio_framework_value_persistent::list::PagedList<M,{usize::MAX}>}
impl<M:ArtifactCanonicalJsonTree> OriginalEditView<'_,M>{fn descend(&self,path:&[usize])->Result<&dyn ArtifactCanonicalJsonTree,ValueError>{let inverse_ordinal=2+usize::from(self.edit.actor.is_some());let(mut node,rest):(&dyn ArtifactCanonicalJsonTree,&[usize])=if path.first()==Some(&inverse_ordinal){(self.inverse,&path[1..])}else{(self.edit,path)};for ordinal in rest{node=node.canonical_tree_child(*ordinal)?;}Ok(node)}}
impl<M:ArtifactCanonicalJsonTree> semio_framework_os_store_canonical_json::ArtifactCanonicalJson for OriginalEditView<'_,M>{fn canonical_json_node(&self,path:&[usize])->Result<semio_framework_pack_json::ArtifactCanonicalJsonNode<'_>,ValueError>{self.descend(path)?.canonical_tree_node()}fn canonical_json_key(&self,path:&[usize],index:usize)->Result<semio_framework_pack_json::ArtifactCanonicalJsonText<'_>,ValueError>{self.descend(path)?.canonical_tree_key(index)}}
