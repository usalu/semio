//! 📤️ Admits a registered serializer over exact captured source custody.
use super::{IoEntry,IoEntryDirection};
use semio_framework_value::{DslValue,ErasedSnapshotRetirement,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneBirthDemand,RetainedCloneGrant,RetainedCloneProgress};
use semio_framework_artifact_reference::ArtifactDialect;
use crate::os_store::ErasedSnapshotRead;

/// 🪆️ Original captured child authority with an explicitly bounded retirement lifecycle.
pub trait OwnedSerializerChildren: ErasedSnapshotRetirement {
 fn authority_matches(&self,generation:u64,revision:[u8;32])->bool;
}

/// 🧳️ Captured source and caller-owned metadata returned intact on refused admission.
pub struct OwnedSerializerRequest {
 pub source:ErasedSnapshotRead,
 pub route:(ArtifactDialect,ArtifactDialect),
 pub generation:u64,
 pub revision:[u8;32],
 pub options:DslValue,
 pub children:Option<Box<dyn OwnedSerializerChildren>>,
}

/// 🌊️ A native serializer uses the same bounded job protocol as interactive commands.
pub trait OwnedSerializerJob: semio_framework_job::InteractiveJob {
 fn source_authority_matches(&self,generation:u64,revision:[u8;32])->bool;
 fn output(&self)->Option<&OwnedSerializerOutput>;
 fn take_output(&mut self,grant:RetainedCloneGrant)->Result<Option<OwnedSerializerOutputHandoff>,ValueError>;
 fn fault(&self)->Option<&semio_framework_diagnostic::Fault>;
 fn take_fault(&mut self,grant:RetainedCloneGrant)->Result<Option<OwnedSerializerFaultHandoff>,ValueError>;
 fn terminal_frame_release_bytes(&self)->Option<usize>;
}

/// 📄️ Original formatter pages and contiguous output close through their genuine typed owners.
#[derive(semio_framework_value::RetireOwned)]
pub struct OwnedSerializerOutput {
 pub pages:semio_framework_value::list::PagedList<Vec<u8>,{usize::MAX}>,
 pub contiguous:Vec<u8>,
}
impl OwnedSerializerOutput {
 pub fn empty()->Self{Self{pages:Default::default(),contiguous:Vec::new()}}
 pub fn page_count(&self)->usize{self.pages.len()+usize::from(!self.contiguous.is_empty())}
 pub fn page(&self,index:usize)->Option<&[u8]>{if index<self.pages.len(){self.pages.get(index).map(Vec::as_slice)}else if index==self.pages.len()&&!self.contiguous.is_empty(){Some(&self.contiguous)}else{None}}
}

/// 🎁️ A funded handoff moves the original output with its actual zero-heap custody receipt.
pub struct OwnedSerializerOutputHandoff {
 pub output:OwnedSerializerOutput,
 pub progress:RetainedCloneProgress,
}

/// ⚠️ A funded handoff moves the full original diagnostic with its actual zero-heap custody receipt.
pub struct OwnedSerializerFaultHandoff {
 pub fault:semio_framework_diagnostic::Fault,
 pub progress:RetainedCloneProgress,
}

/// 🌱️ Actual admitted job custody and its physical constructor receipt.
pub struct OwnedSerializerAdmission {
 pub job:Box<dyn OwnedSerializerJob>,
 pub progress:RetainedCloneProgress,
}

/// 🛑️ Retains the entire original request together with any incurred physical receipt.
pub struct OwnedSerializerRefusal {
 pub error:ValueError,
 pub request:OwnedSerializerRequest,
 pub progress:RetainedCloneProgress,
}

/// 🎟️ Declares constructor demand before any allocation or transfer into a job.
#[derive(Clone,Copy)]
pub struct OwnedSerializerFactory {
 pub demand:fn(&OwnedSerializerRequest)->Result<RetainedCloneBirthDemand,ValueError>,
 pub admit:fn(OwnedSerializerRequest,RetainedCloneGrant)->Result<OwnedSerializerAdmission,OwnedSerializerRefusal>,
}

pub(super) fn same_owned_serializer(left:Option<OwnedSerializerFactory>,right:Option<OwnedSerializerFactory>)->bool{
 match(left,right){(None,None)=>true,(Some(left),Some(right))=>std::ptr::fn_addr_eq(left.demand,right.demand)&&std::ptr::fn_addr_eq(left.admit,right.admit),_=>false}
}

impl IoEntry {
 /// 🔐️ Revalidates exact live authority and admission before invoking this registered native factory.
 pub fn begin_owned_serialization(&self,request:OwnedSerializerRequest,grant:RetainedCloneGrant)->Result<OwnedSerializerAdmission,OwnedSerializerRefusal>{
  let refusal=|error,request|OwnedSerializerRefusal{error,request,progress:Default::default()};
  let Some(factory)=self.owned_serializer.filter(|_|self.direction==IoEntryDirection::Export&&coordinate_matches(self.from,&request.route.0)&&coordinate_matches(self.into,&request.route.1))else{return Err(OwnedSerializerRefusal{error:ValueError::literal(ValueRefusalKind::UnsupportedOwner,"registered IO entry has no matching captured serializer"),request,progress:Default::default()});};
  if !request.source.commit_authority_matches(request.generation,request.revision)||request.children.as_ref().is_some_and(|children|!children.authority_matches(request.generation,request.revision)){return Err(refusal(ValueError::literal(ValueRefusalKind::InvariantViolated,"captured serializer source authority changed"),request));}
  let demand=match(factory.demand)(&request){Ok(demand)=>demand,Err(error)=>return Err(refusal(error,request))};
  if let Err(error)=demand.admit(grant){return Err(refusal(error,request));}
  (factory.admit)(request,grant)
 }
}

fn coordinate_matches(declared:semio_framework_artifact_reference::Dialect,captured:&ArtifactDialect)->bool{
 declared.artifact_kind==captured.artifact_kind&&declared.standard.0==captured.standard&&declared.subset.0==captured.subset
}

/// 🗂️ Selects the actual registered native factory without allocating lookup keys or holding the registry during work.
pub fn io_begin_owned_serialization(request:OwnedSerializerRequest,grant:RetainedCloneGrant)->Result<OwnedSerializerAdmission,OwnedSerializerRefusal>{
 let entry={let registry=match super::io_mechanism_registry().try_read(){Ok(registry)=>registry,Err(_)=>return Err(OwnedSerializerRefusal{error:ValueError::literal(ValueRefusalKind::WorkLimit,"owned serializer registry is busy"),request,progress:Default::default()})};registry.get(&request.route).copied()};
 let Some(entry)=entry else{return Err(OwnedSerializerRefusal{error:ValueError::literal(ValueRefusalKind::UnsupportedOwner,"captured serializer entry is not registered"),request,progress:Default::default()});};
 entry.begin_owned_serialization(request,grant)
}
