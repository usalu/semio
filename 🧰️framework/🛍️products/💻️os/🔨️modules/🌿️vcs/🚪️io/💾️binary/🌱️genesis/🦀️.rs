//! 🌱️ Exact native Pack custody and typed genesis admission.
use crate::genesis::{ArtifactGenesis,ArtifactGenesisWitness};
use semio_framework_value::{ByteReadSource, ValueError};
use semio_framework_value::retirement::{RetireOwned, arc_custody::ArcLease};
use std::sync::Arc;
/// 📦️ The selected native producer encodes and reconstructs its actual typed genesis snapshot.
pub trait ArtifactGenesisProducer<P> {
 fn encode_genesis_pack(&self,snapshot:&P)->Result<Vec<u8>,ValueError>;
 fn decode_genesis_pack(&self,pack:&[u8])->Result<P,ValueError>;
}
/// 🧱️ Bytes staged by the native Pack writer retain their original fixed-size pages.
pub type ArtifactGenesisPackPages = semio_framework_value::paged::PagedBytes<{isize::MAX as usize}>;

/// 🎒️ The immutable physical Pack backing, with the same byte-read contract for both sources.
#[derive(Debug, semio_framework_value::RetireOwned)]
pub enum ArtifactGenesisPack {
    Contiguous(Vec<u8>),
    Paged(ArtifactGenesisPackPages),
}

impl From<Vec<u8>> for ArtifactGenesisPack {
    fn from(bytes: Vec<u8>) -> Self { Self::Contiguous(bytes) }
}

impl From<ArtifactGenesisPackPages> for ArtifactGenesisPack {
    fn from(bytes: ArtifactGenesisPackPages) -> Self { Self::Paged(bytes) }
}

impl ArtifactGenesisPack {
    /// 🫳️ Borrows a contiguous wire source when it supplied the actual backing.
    pub fn contiguous(&self) -> Option<&[u8]> {
        match self { Self::Contiguous(bytes) => Some(bytes.as_slice()), Self::Paged(_) => None }
    }

    /// 📤️ Assembles the canonical bytes for an explicit whole-file or transport request.
    pub fn to_vec(&self) -> Vec<u8> {
        match self {
            Self::Contiguous(bytes) => bytes.clone(),
            Self::Paged(bytes) => bytes.to_vec_owner(),
        }
    }
}

impl ByteReadSource for ArtifactGenesisPack {
    fn byte_len(&self) -> usize {
        match self { Self::Contiguous(bytes) => bytes.len(), Self::Paged(bytes) => bytes.len() }
    }
    fn byte_at(&self, offset: usize) -> Option<u8> {
        match self { Self::Contiguous(bytes) => bytes.get(offset).copied(), Self::Paged(bytes) => bytes.byte_at(offset) }
    }
    fn read_bytes(&self, offset: usize, output: &mut [u8]) -> usize {
        match self {
            Self::Contiguous(bytes) => bytes.read_bytes(offset, output),
            Self::Paged(bytes) => bytes.read_bytes(offset, output),
        }
    }
}

impl PartialEq for ArtifactGenesisPack {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Contiguous(left), Self::Contiguous(right)) => left == right,
            (Self::Paged(left), Self::Paged(right)) => left == right,
            (Self::Contiguous(left), Self::Paged(right)) | (Self::Paged(right), Self::Contiguous(left)) => {
                left.iter().copied().eq(right.iter())
            }
        }
    }
}

#[derive(Debug,PartialEq)]
pub struct AdmittedArtifactGenesis<P>{facts:ArtifactGenesis<P>,pack:Arc<ArtifactGenesisPack>}
impl<P> Clone for AdmittedArtifactGenesis<P>{fn clone(&self)->Self{Self{facts:self.facts.clone(),pack:Arc::clone(&self.pack)}}}
impl<P> AdmittedArtifactGenesis<P>{
 pub fn facts(&self)->&ArtifactGenesis<P>{&self.facts}
 pub fn pack(&self)->&ArtifactGenesisPack{&self.pack}
 pub fn share_pack(&self)->Arc<ArtifactGenesisPack>{Arc::clone(&self.pack)}
 pub fn from_decoded_pack(snapshot:P,pack:Vec<u8>)->Self{let digest=*semio_framework_hash::hash(&pack).as_bytes();Self::from_verified_pack(snapshot,pack,digest)}
 pub fn from_verified_pack(snapshot:P,pack:Vec<u8>,digest:[u8;32])->Self{Self{facts:ArtifactGenesis::admitted(snapshot,digest),pack:Arc::new(ArtifactGenesisPack::Contiguous(pack))}}

 /// 🎟️ Quotes the original typed snapshot and Pack Arc shells without constructing either owner.
 pub fn verified_pack_birth_demand()->Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand,ValueError>{
  let capacity_bytes=semio_framework_value::retirement::arc_custody::shared_retirement_allocation_bytes::<P>().checked_add(semio_framework_value::retirement::arc_custody::shared_retirement_allocation_bytes::<ArtifactGenesisPack>()).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"verified genesis shared extent overflow"))?;
  Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand{capacity_bytes,depth:1})
 }
 /// 🫴️ Admits original verified payloads while returning both exact inputs on refusal.
 pub fn admit_verified_pack(snapshot:P,pack:Vec<u8>,digest:[u8;32],grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<(Self,semio_framework_value::retained_clone::RetainedCloneProgress),(ValueError,P,Vec<u8>)>{
  Self::admit_verified_physical_pack(snapshot, ArtifactGenesisPack::Contiguous(pack), digest, grant)
    .map_err(|(error, snapshot, pack)| match pack {
      ArtifactGenesisPack::Contiguous(pack) => (error, snapshot, pack),
      ArtifactGenesisPack::Paged(_) => unreachable!("verified contiguous Pack admission changed representation"),
    })
 }

 /// 🫴️ Transfers original verified pages and typed state without copying either payload.
 pub fn admit_verified_paged_pack(snapshot: P, pack: ArtifactGenesisPackPages, digest: [u8; 32], grant: semio_framework_value::RetainedCloneGrant)
     -> Result<(Self, semio_framework_value::RetainedCloneProgress), (ValueError, P, ArtifactGenesisPackPages)>
 {
    Self::admit_verified_physical_pack(snapshot, ArtifactGenesisPack::Paged(pack), digest, grant)
        .map_err(|(error, snapshot, pack)| match pack {
            ArtifactGenesisPack::Paged(pack) => (error, snapshot, pack),
            ArtifactGenesisPack::Contiguous(_) => unreachable!("verified paged Pack admission changed representation"),
        })
 }

 /// 📥 Admits an already verified physical Pack without flattening its backing.
 pub fn admit_verified_physical_pack(snapshot: P, pack: ArtifactGenesisPack, digest: [u8; 32], grant: semio_framework_value::RetainedCloneGrant)
     -> Result<(Self, semio_framework_value::RetainedCloneProgress), (ValueError, P, ArtifactGenesisPack)>
 {
     let demand = match Self::verified_pack_birth_demand() {
         Ok(demand) => demand,
         Err(error) => return Err((error, snapshot, pack)),
     };
     if let Err(error) = demand.admit(grant) { return Err((error, snapshot, pack)); }
     let genesis = Self { facts: ArtifactGenesis::admitted(snapshot, digest), pack: Arc::new(pack) };
     let receipt = semio_framework_value::RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: demand.capacity_bytes, ..Default::default() };
     Ok((genesis, receipt))
 }

 pub fn into_owners(self)->(Arc<P>,Arc<ArtifactGenesisPack>){(self.facts.into_snapshot(),self.pack)}
}
impl<P> AdmittedArtifactGenesis<P>{
 pub fn born<G:ArtifactGenesisProducer<P>>(snapshot:P,producer:&G)->Result<Self,ValueError>{let pack=producer.encode_genesis_pack(&snapshot)?;Ok(Self::from_decoded_pack(snapshot,pack))}
 pub fn from_stored_pack<G:ArtifactGenesisProducer<P>>(pack:Vec<u8>,producer:&G)->Result<Self,ValueError>{let snapshot=producer.decode_genesis_pack(&pack)?;Ok(Self::from_decoded_pack(snapshot,pack))}
}


#[doc(hidden)]
#[derive(semio_framework_value::RetireOwned)]
pub struct ArtifactGenesisRetirementOwners<P: RetireOwned + Sync> {
    snapshot: ArcLease<P>,
    pack: ArcLease<ArtifactGenesisPack>,
}

impl<P: RetireOwned + Sync> RetireOwned for AdmittedArtifactGenesis<P> {
    type RetirementCursor = <ArtifactGenesisRetirementOwners<P> as RetireOwned>::RetirementCursor;

    fn retirement_seed(self) -> Self::RetirementCursor {
        let (snapshot, pack) = self.into_owners();
        ArtifactGenesisRetirementOwners { snapshot: ArcLease::new(snapshot), pack: ArcLease::new(pack) }.retirement_seed()
    }

    fn controlled_retirement_supported() -> bool {
        ArtifactGenesisRetirementOwners::<P>::controlled_retirement_supported()
    }
}

impl<P> ArtifactGenesisWitness for AdmittedArtifactGenesis<P>{type Snapshot=P;fn genesis_facts(&self)->&ArtifactGenesis<P>{&self.facts}}

use pack::snapshot::ArtifactPack;
use pack::record::EncodeOptions as PackEncodeOptions;
/// 🎒️ The native Pack producer uses the authored snapshot Pack representation.
pub struct ArtifactPackGenesisProducer;
impl<P:ArtifactPack> ArtifactGenesisProducer<P> for ArtifactPackGenesisProducer {
 fn encode_genesis_pack(&self,snapshot:&P)->Result<Vec<u8>,ValueError>{snapshot.encode_pack_with(&PackEncodeOptions::default()).map_err(|error|ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error.to_string()))}
 fn decode_genesis_pack(&self,pack:&[u8])->Result<P,ValueError>{P::decode_pack(pack).map_err(|error|ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error.to_string()))}
}
