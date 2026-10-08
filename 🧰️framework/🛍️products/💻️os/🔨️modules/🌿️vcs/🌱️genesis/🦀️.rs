//! 🌱️ Pure admitted immutable genesis facts.
use semio_framework_value::{DslValue, FromValue, ToValue, ValueError, ValueRefusalKind};
use std::sync::Arc;
#[derive(Debug, PartialEq)]
pub struct ArtifactGenesis<P> { snapshot: Arc<P>, digest: [u8; 32] }
impl<P> Clone for ArtifactGenesis<P> {
 fn clone(&self)->Self { Self { snapshot: Arc::clone(&self.snapshot), digest:self.digest } }
}
impl<P> ArtifactGenesis<P> {
 pub fn admitted(snapshot:P,digest:[u8;32])->Self {Self{snapshot:Arc::new(snapshot),digest}}
 pub fn snapshot(&self)->&P {&self.snapshot}
 pub fn share_snapshot(&self)->Arc<P> {Arc::clone(&self.snapshot)}
 pub fn digest(&self)->[u8;32] {self.digest}
 pub fn into_snapshot(self)->Arc<P> {self.snapshot}
}
impl<P:ToValue> ToValue for ArtifactGenesis<P> {
 fn to_value(&self)->DslValue {DslValue::object([("snapshot".into(),self.snapshot().to_value()),("digest".into(),self.digest.to_vec().to_value())])}
}
impl<P:FromValue> FromValue for ArtifactGenesis<P> {
 fn from_value(value:DslValue)->Result<Self,ValueError>{
  let DslValue::Object(fields)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Genesis requires typed facts"));};
  let(mut snapshot,mut digest)=(None,None);
  for(key,value)in fields {match key.as_str(){"snapshot" if snapshot.is_none()=>snapshot=Some(P::from_value(value)?),"digest" if digest.is_none()=>digest=Some(Vec::<u8>::from_value(value)?.try_into().map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"Genesis digest requires 32 words"))?),_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Genesis has repeated or unknown facts"))}}
  Ok(Self::admitted(snapshot.ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Genesis missing snapshot"))?,digest.ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Genesis missing digest"))?))
 }
}

/// 🪪️ A first-party typed admission witness projecting only logical genesis facts.
pub trait ArtifactGenesisWitness {type Snapshot;fn genesis_facts(&self)->&ArtifactGenesis<Self::Snapshot>;}
impl<P> ArtifactGenesisWitness for ArtifactGenesis<P>{type Snapshot=P;fn genesis_facts(&self)->&ArtifactGenesis<P>{self}}
