//! 🔏️ Bounded record hashing reborrows the original canonical source on every turn.
use super::*;
use semio_framework_hash::Sha256;
use semio_framework_value::retained_clone::retained_clone_grant_admits;

/// 🧮️ Counts canonical bytes before hashing their exact length-framed record without a JSON allocation.
pub struct ArtifactCanonicalRecordDigest {
 cursor:ArtifactCanonicalJsonCursor,
 hash:Sha256,
 length:u64,
 offset:usize,
 phase:u8,
}
impl Default for ArtifactCanonicalRecordDigest{fn default()->Self{Self{cursor:Default::default(),hash:Sha256::new(),length:0,offset:0,phase:0}}}
impl ArtifactCanonicalRecordDigest{
 pub fn digest(&self)->Option<[u8;32]>{(self.phase==4).then(||self.hash.clone().finalize())}
 pub fn next_demand(&self)->RetirementDemand{
  match self.phase{0|3=>self.cursor.next_encode_demand(),1=>RetirementDemand{copy_bytes:size_of::<ArtifactCanonicalJsonCursor>(),depth:1,..Default::default()},2=>RetirementDemand{copy_bytes:1,depth:1,..Default::default()},_=>Default::default()}
 }
 pub fn advance(&mut self,source:&(impl ArtifactCanonicalJson+?Sized),domain:&[u8],prefix:&[u8],grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let demand=self.next_demand();if self.phase==4{return Ok(RetainedCloneStep::Complete(Default::default()))}if !retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if self.phase==1{self.cursor=Default::default();self.phase=2;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}
  if self.phase==2{
   let domain_length=(domain.len()as u64).to_be_bytes();let prefix_length=(prefix.len()as u64).to_be_bytes();let length=self.length.to_be_bytes();let mut at=self.offset;let mut byte=None;
   for part in [b"semio.artifact.cursor.v2".as_slice(),domain_length.as_slice(),domain,prefix_length.as_slice(),prefix,length.as_slice()]{if at<part.len(){byte=Some(part[at]);break}at-=part.len()}
   if let Some(byte)=byte{self.hash.update(&[byte]);self.offset+=1}else{self.phase=3}
   return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()}))
  }
  let mut output=[0];let step=self.cursor.encode_chunk_admitted(source,&mut output,grant).map_err(|error|error.reason)?;
  if self.phase==0{self.length=self.length.checked_add(step.written_bytes as u64).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"canonical record byte extent overflow"))?}else{self.hash.update(&output[..step.written_bytes])}
  if matches!(step.ownership,RetainedCloneStep::Complete(_)){self.phase=if self.phase==0{1}else{4}}
  Ok(if self.phase==4{RetainedCloneStep::Complete(step.ownership.progress())}else{RetainedCloneStep::Progress(step.ownership.progress())})
 }
}

#[cfg(test)]
#[path="🧪️.rs"]
mod tests;
