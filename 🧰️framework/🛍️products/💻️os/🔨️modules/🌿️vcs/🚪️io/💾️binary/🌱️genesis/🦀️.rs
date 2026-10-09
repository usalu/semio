//! 🌱️ Exact native Pack custody and typed genesis admission.
use crate::os_vcs::genesis::{ArtifactGenesis,ArtifactGenesisWitness};
use semio_framework_value::ValueError;
use std::sync::Arc;
/// 📦️ Native binary genesis capability.
pub trait ArtifactGenesisCodec:Sized {
 fn encode_genesis_pack(&self)->Result<Vec<u8>,ValueError>;
 fn decode_genesis_pack(pack:&[u8])->Result<Self,ValueError>;
}
#[derive(Debug,PartialEq)]
pub struct AdmittedArtifactGenesis<P>{facts:ArtifactGenesis<P>,pack:Arc<Vec<u8>>}
impl<P> Clone for AdmittedArtifactGenesis<P>{fn clone(&self)->Self{Self{facts:self.facts.clone(),pack:Arc::clone(&self.pack)}}}
impl<P> AdmittedArtifactGenesis<P>{
 pub fn facts(&self)->&ArtifactGenesis<P>{&self.facts}
 pub fn pack(&self)->&[u8]{&self.pack}
 pub fn share_pack(&self)->Arc<Vec<u8>>{Arc::clone(&self.pack)}
 pub fn from_decoded_pack(snapshot:P,pack:Vec<u8>)->Self{let digest=*semio_framework_hash::hash(&pack).as_bytes();Self::from_verified_pack(snapshot,pack,digest)}
 pub fn from_verified_pack(snapshot:P,pack:Vec<u8>,digest:[u8;32])->Self{Self{facts:ArtifactGenesis::admitted(snapshot,digest),pack:Arc::new(pack)}}

 /// 🎟️ Quotes the original typed snapshot and Pack Arc shells without constructing either owner.
 pub fn verified_pack_birth_demand()->Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand,ValueError>{
  let capacity_bytes=semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<P>().checked_add(semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<Vec<u8>>()).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"verified genesis shared extent overflow"))?;
  Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand{capacity_bytes,depth:1})
 }
 /// 🫴️ Admits original verified payloads while returning both exact inputs on refusal.
 pub fn admit_verified_pack(snapshot:P,pack:Vec<u8>,digest:[u8;32],grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<(Self,semio_framework_value::retained_clone::RetainedCloneProgress),(ValueError,P,Vec<u8>)>{
  let demand=match Self::verified_pack_birth_demand(){Ok(demand)=>demand,Err(error)=>return Err((error,snapshot,pack))};
  if let Err(error)=demand.admit(grant){return Err((error, snapshot, pack));}
  Ok((Self::from_verified_pack(snapshot, pack, digest),semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,retained_capacity_bytes:demand.capacity_bytes,..Default::default()}))
 }
 pub fn into_owners(self)->(Arc<P>,Arc<Vec<u8>>){(self.facts.into_snapshot(),self.pack)}
}
impl<P:ArtifactGenesisCodec> AdmittedArtifactGenesis<P>{
 pub fn born(snapshot:P)->Result<Self,ValueError>{let pack=snapshot.encode_genesis_pack()?;Ok(Self::from_decoded_pack(snapshot,pack))}
 pub fn from_stored_pack(pack:Vec<u8>)->Result<Self,ValueError>{let snapshot=P::decode_genesis_pack(&pack)?;Ok(Self::from_decoded_pack(snapshot,pack))}
}

impl<P> ArtifactGenesisWitness for AdmittedArtifactGenesis<P>{type Snapshot=P;fn genesis_facts(&self)->&ArtifactGenesis<P>{&self.facts}}
