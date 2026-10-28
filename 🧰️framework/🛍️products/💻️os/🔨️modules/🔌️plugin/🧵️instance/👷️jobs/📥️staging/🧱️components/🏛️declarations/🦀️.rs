//! 🏛️ Declaration commit produces one original registered catalog before mounted requests retain paid lower leases.
use super::*;
use semio_framework::Fault;
use semio_framework_value_persistent::list::PagedList;
use std::mem::{ManuallyDrop,size_of};
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,ValueError,ValueRefusalKind};
use semio_framework_plugin_window_kits::RegisteredDefinitionSource;
use semio_framework_plugin_decl_app::tool_contract::{ArtifactToolPublicationLane,ArtifactToolFactoryRegistry};
pub use semio_framework_plugin_job::original_declared_tools::{DeclaredToolCatalog,DeclaredToolCatalogSource,DeclaredToolCatalogAlias};
use semio_framework_plugin_job::original_declared_tools::OriginalDeclaredToolCatalog;
 /// 🏗️ This producer runs at declaration commit before any mounted request exists.
pub fn produce_declared_tool_catalog<A:ArtifactApp>(source:RegisteredDefinitionSource<'_>)->Result<DeclaredToolCatalog,Fault>{
  if source.dialect.artifact_kind!=A::DIALECT.artifact_kind||source.dialect.standard!=A::DIALECT.standard.0||source.dialect.subset!=A::DIALECT.subset.0||source.io.artifact_schema!=A::DOCUMENT_SCHEMA{return Err(semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault("declared tool catalog does not name its typed original artifact owner"))}
  let registry=AppActionRegistry::from_definition(&source);
  let bus_source=semio_framework::RegisteredActionBusSource::production();
  let bus=bus_source.bus();
  let mut framework=ArtifactToolFactoryRegistry::<A>::new(&bus,&source.id);
  semio_framework_plugin_job::reserved_jobs::register_framework_reserved_factories(&mut framework)?;
  let framework=framework.finish();
  let mut app=ArtifactToolFactoryRegistry::<A>::new(&bus,&source.id);
  A::register_tool_job_factories(&mut app)?;
  let app=app.finish();
  let(controller_id,proofs)=semio_framework_plugin_job::reserved_jobs::tool_job_registration::<A>(&registry,&source.id,A::DOCUMENT_SCHEMA,<A::Command as protocol::OpBinary>::TOOL_JOB_IDS,&bus,&app)?;
  let mut bounded_proofs=PagedList::new();let mut bounded_contracts=PagedList::new();
  for proof in proofs{
   let tool_id=proof.row.tool_id.to_string();
   if app.contains_key(&tool_id)||framework.contains_key(&tool_id){continue}
   bounded_contracts.push(ArtifactToolPublicContract{owner:proof.owner,controller_id:proof.row.controller_id.to_string(),tool_id:tool_id.clone(),schema_id:proof.schema_id(),max_raw_wire_bytes:proof.contract().max_raw_wire_bytes,publication_lanes:&[ArtifactToolPublicationLane::HostOnly]});
   let factory=semio_framework_plugin_job::lifted_full_operation_job::TypedCommandFullOperationJobFactory::<A>::from_proof(proof).ok_or_else(||semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault("original bounded proof lost its installed concrete factory witness"))?;
   bus.register_once(factory).map_err(|_|semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault("declared bounded factory conflicts with its original controller registration"))?;
   bounded_proofs.push((tool_id,proof));
  }
  Ok(DeclaredToolCatalog{controller_id,bus:bus_source,bounded_proofs,bounded_contracts,framework:framework.into_iter().collect(),app:app.into_iter().collect()})
 }
/// 🏡️ The program registration keeps the actual lower catalog producer through every mounted metadata lease.
pub struct OriginalDeclaredTools{catalog:OriginalDeclaredToolCatalog}
impl OriginalDeclaredTools{
 pub fn new(catalog:DeclaredToolCatalog)->Self{Self{catalog:OriginalDeclaredToolCatalog::new(catalog)}}
 pub fn commit_producer(&mut self){self.catalog.commit_producer();}
 pub fn source(&self)->Option<DeclaredToolCatalogSource<'_>>{self.catalog.source()}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.catalog.close_demands(body)}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.catalog.close_step(grant)}
 pub fn terminal_is_empty(&self)->bool{self.catalog.terminal_is_empty()}
}
impl semio_framework_plugin_reactor_app::OriginalPluginAppRegistration for OriginalDeclaredTools{
 fn as_any(&self)->&dyn std::any::Any{self}
 fn commit_producer(&mut self){OriginalDeclaredTools::commit_producer(self);}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{OriginalDeclaredTools::close_demands(self,body)}
 fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{let d=OriginalDeclaredTools::close_demands(self,cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,d,|g|OriginalDeclaredTools::close_step(self,g))?;Ok(OriginalDeclaredTools::terminal_is_empty(self))}
 fn terminal_is_empty(&self)->bool{OriginalDeclaredTools::terminal_is_empty(self)}
 fn boxed_release_demands(&self)->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<Box<dyn semio_framework_plugin_reactor_app::OriginalPluginAppRegistration>>>(),release_bytes:size_of::<Self>(),depth:1,..Default::default()}}
}

/// 🎟️ The static declaration producer validates the concrete App factories before retaining their sole metadata owner.
pub fn produce_original_tool_registration<A:ArtifactApp>(source:RegisteredDefinitionSource<'_>)->Result<Box<dyn semio_framework_plugin_reactor_app::OriginalPluginAppRegistration>,Fault>{Ok(Box::new(OriginalDeclaredTools::new(produce_declared_tool_catalog::<A>(source)?)))}

#[path="🌱️capture/🦀️.rs"]mod original_capture;
pub use original_capture::{OriginalDeclaredEmptyOperationSource,OriginalDeclaredOperationTools};
