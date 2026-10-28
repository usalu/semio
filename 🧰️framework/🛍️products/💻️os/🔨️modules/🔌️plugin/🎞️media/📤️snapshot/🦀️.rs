//! 📸️ Original snapshot leases produce canonical native Pack through the existing media actor protocol.
use semio_framework_os_kernel as store;
use crate::{ArtifactApp,ArtifactMediaExportJobRequest,ArtifactReservedJob,ArtifactReservedToolJob,ArtifactOwnedToolJobFactory,ArtifactToolPublicationContract,ArtifactToolPublicationLane};
use crate::app::{ArtifactMediaExportCompletion,ArtifactMediaExportCredit,ArtifactMediaExportResult,ArtifactOutputChunks,ArtifactSnapshotCloseLease};
use semio_framework::{Fault,MediaClass,MediaForm,MediaType,ToolExecutionContract,ToolFactoryKey,ToolJobFactory,ToolJobFactoryError,InteractiveJobClassification};
use semio_framework_job::{InteractiveJob,InteractiveJobCloseStep,JobOutcomeBorrow,JobOutcomeDescriptor,JobOutcomeView,JobOutcomeKind,StepContext,OriginalTurnFailure,OriginalSourceBinding,OriginalConstructionDemand,admit_original_construction};
use semio_framework_job_publication::RetainedJobPublication;
use semio_framework_dsl_record::{BorrowedDslRecord,native_encoding::FieldProjectionSource};
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_plugin_ownership::bounded_ownership::{plugin_typed_owner_demand,plugin_typed_owner_step,plugin_grant_funds};
use semio_framework_pack::record::BorrowedPackDocumentCursor;
use std::{sync::Arc,mem::ManuallyDrop};
pub const SNAPSHOT_PACK_PORT:&str="semio.snapshot.pack";
pub const SNAPSHOT_PACK_TOOL:&str="export-media:semio.snapshot.pack";
pub const SNAPSHOT_PACK_MIME:&str="application/vnd.semio.pack";
pub const SNAPSHOT_PACK_SCHEMA:&str="semio.snapshot.pack.v1";
pub const SNAPSHOT_PACK_CONTRACT:ToolExecutionContract=ToolExecutionContract::resumable(4096,4096,4096,ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES,2000,64,1);
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn demand(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn construction(d:RetirementDemand)->OriginalConstructionDemand{OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth}}
type Outputs=(ArtifactMediaExportCredit,Option<ArtifactOutputChunks>,ArtifactMediaExportCompletion);
type Metadata=(String,String,String);
pub struct NativeSnapshotPackExportJob<A:ArtifactApp> where A::Snapshot:BorrowedDslRecord+FieldProjectionSource{
 snapshot:Option<Arc<A::Snapshot>>,lease:Option<ArtifactSnapshotCloseLease<A::Snapshot>>,cursor:Option<Box<BorrowedPackDocumentCursor>>,
 metadata:ManuallyDrop<Option<Metadata>>,metadata_close:ManuallyDrop<Option<ControlledRetirement<Metadata>>>,outputs:ManuallyDrop<Option<Outputs>>,outputs_close:ManuallyDrop<Option<ControlledRetirement<Outputs>>>,
 page:Option<Vec<u8>>,pending:ManuallyDrop<Option<Result<ArtifactMediaExportResult,semio_framework::MediaError>>>,pending_close:ManuallyDrop<Option<ControlledRetirement<Result<ArtifactMediaExportResult,semio_framework::MediaError>>>>,
 binding:Option<OriginalSourceBinding>,source_identity:(semio_framework_job::OperationId,semio_framework_job::Generation),prefix:usize,encoded:bool,published:bool,completed:bool,closing:bool,units:u64,
 publication:RetainedJobPublication,delivered:bool,publish_progress:bool,
}
impl<A:ArtifactApp> NativeSnapshotPackExportJob<A> where A::Snapshot:BorrowedDslRecord+FieldProjectionSource{
 pub fn admit_original(request:&mut Option<ArtifactMediaExportJobRequest<A>>,body:&mut Option<Box<dyn ArtifactReservedJob+Send>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  cx.original_source_parent()?;if cx.should_yield()||cx.is_cancelled(){return Ok(false)}
  if body.is_none(){let source=request.as_ref().ok_or_else(||invalid("native Pack factory requires its original request"))?;if source.port!=SNAPSHOT_PACK_PORT||source.tool_id!=SNAPSHOT_PACK_TOOL{return Err(invalid("native Pack factory received a foreign port").into())}let source_identity=source.source_identity;let bytes=size_of::<Self>();return admit_original_construction(cx,body,construction(RetirementDemand{capacity_bytes:bytes,..demand(bytes)}),|_|{let pointer=std::ptr::NonNull::new(unsafe{std::alloc::alloc(std::alloc::Layout::new::<Self>())}.cast::<Self>()).ok_or_else(||ValueError::literal(ValueRefusalKind::AllocationFailed,"native Pack body allocation refused"))?;let owner=Self{snapshot:None,lease:None,cursor:None,metadata:ManuallyDrop::new(None),metadata_close:ManuallyDrop::new(None),outputs:ManuallyDrop::new(None),outputs_close:ManuallyDrop::new(None),page:None,pending:ManuallyDrop::new(None),pending_close:ManuallyDrop::new(None),binding:None,source_identity,prefix:0,encoded:false,published:false,completed:false,closing:false,units:0,publication:RetainedJobPublication::new(),delivered:false,publish_progress:false};let boxed:Box<dyn ArtifactReservedJob+Send>=unsafe{pointer.as_ptr().write(owner);Box::from_raw(pointer.as_ptr())};Ok((boxed,RetainedCloneProgress{copied_items:1,copied_bytes:bytes,retained_capacity_bytes:bytes,..Default::default()}))}).map(|_|false)}
  let owner=(&mut**body.as_mut().unwrap()as &mut dyn std::any::Any).downcast_mut::<Self>().ok_or_else(||invalid("native Pack factory requires its exact body"))?;
  if owner.snapshot.is_some(){return if request.is_none(){Ok(true)}else{Err(invalid("native Pack source cannot be overwritten").into())}}
  let bytes=size_of::<ArtifactMediaExportJobRequest<A>>();let grant=cx.retained_grant();if !plugin_grant_funds(demand(bytes),grant)||grant.maximum_items==0{return Ok(false)}let source=request.take().ok_or_else(||invalid("native Pack source request is absent"))?;owner.snapshot=Some(source.snapshot);owner.lease=Some(source.snapshot_close);*owner.metadata=Some((source.tool_id,source.parent_document_id,source.port));*owner.outputs=Some((source.output_credit,Some(source.output_chunks),source.completion));cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(false)
 }
 fn advance(&mut self,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
  if self.binding.is_none(){self.binding=Some(OriginalSourceBinding::borrow(cx));}if !self.binding.unwrap().matches(cx)||self.source_identity!=(cx.operation(),cx.generation()){return Err(invalid("native Pack writer lost its original operation source").into())}
  if self.pending.is_some()||self.published{self.completed=self.outputs.as_ref().unwrap().2.complete_original(&mut self.pending,&mut self.published,cx)?;return Ok(())}
  if self.cursor.is_none(){let prefix=store::os_store::semio_format::declared_envelope_prefix_len(<A::Snapshot as semio_framework_pack::snapshot::ArtifactDsl>::envelope_id(),store::os_store::semio_format::Component::Pack,1)?;let maximum=ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES.checked_sub(SNAPSHOT_PACK_SCHEMA.len()).and_then(|maximum|maximum.checked_sub(prefix)).ok_or_else(||invalid("native snapshot prefix exceeds output authority"))?;let bytes=size_of::<BorrowedPackDocumentCursor>();admit_original_construction(cx,&mut self.cursor,construction(RetirementDemand{capacity_bytes:bytes,..demand(bytes)}),|_|Ok((Box::new(BorrowedPackDocumentCursor::with_maximum_file_len(maximum)),RetainedCloneProgress{copied_items:1,copied_bytes:bytes,retained_capacity_bytes:bytes,..Default::default()})))?;return Ok(())}
  if self.page.as_ref().is_some_and(|page|page.len()==ArtifactOutputChunks::CHUNK_BYTES||(self.encoded&&!page.is_empty())){let outputs=self.outputs.as_ref().unwrap();outputs.1.as_ref().unwrap().push_original(&mut self.page,&outputs.0,cx)?;return Ok(())}
  if self.encoded{
   let schema=SNAPSHOT_PACK_SCHEMA;let mime=SNAPSHOT_PACK_MIME;let capacity=schema.len()+mime.len();let bytes=size_of::<ArtifactMediaExportResult>()+capacity;
   let outputs=&mut self.outputs;admit_original_construction(cx,&mut self.pending,construction(RetirementDemand{capacity_bytes:capacity,..demand(bytes)}),|_|{let original=outputs.as_mut().unwrap();original.1.as_ref().ok_or_else(||invalid("native Pack completion lost original chunks"))?.seal().map_err(|_|invalid("native Pack output seal refused"))?;original.0.credit_original(schema.len())?;let chunks=original.1.take().unwrap();let result=ArtifactMediaExportResult{media_type:MediaType{class:MediaClass::Data,form:MediaForm::Document},schema:schema.into(),mime_type:mime.into(),chunks};Ok((Ok(result),RetainedCloneProgress{copied_items:1,copied_bytes:bytes,retained_capacity_bytes:capacity,..Default::default()}))})?;return Ok(())
  }
  if self.page.is_none(){let bytes=ArtifactOutputChunks::CHUNK_BYTES;admit_original_construction(cx,&mut self.page,construction(RetirementDemand{capacity_bytes:bytes,..demand(size_of::<Vec<u8>>())}),|_|{let mut page=Vec::new();page.try_reserve_exact(bytes).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"native Pack output page allocation refused"))?;let capacity=page.capacity();Ok((page,RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Vec<u8>>(),retained_capacity_bytes:capacity,..Default::default()}))})?;return Ok(())}
  let envelope=<A::Snapshot as semio_framework_pack::snapshot::ArtifactDsl>::envelope_id();
  if let Some(byte)=store::os_store::semio_format::declared_binary_prefix_byte(envelope,store::os_store::semio_format::Component::Pack,1,self.prefix)?{let d=demand(1+size_of::<usize>());if plugin_grant_funds(d,cx.retained_grant())&&cx.retained_grant().maximum_items>0{self.page.as_mut().unwrap().push(byte);self.prefix+=1;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})?;cx.consume_fuel(1);}return Ok(())}
  let cursor=self.cursor.as_ref().unwrap();let d=RetirementDemand{copy_bytes:cursor.next_copy_byte_demand()?.checked_add(1).ok_or_else(||invalid("native Pack copy quote overflow"))?,capacity_bytes:cursor.next_capacity_byte_demand()?,depth:cursor.next_depth_demand()?+1,..Default::default()};let grant=cx.retained_grant();let mut done=None;
  admit_original_construction(cx,&mut done,construction(d),|_|{let mut byte=[0];let step=match self.cursor.as_mut().unwrap().advance(self.snapshot.as_deref().unwrap(),<A::Snapshot as BorrowedDslRecord>::RECORD,&mut byte,RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-1,maximum_depth:grant.maximum_depth-1,..grant}){Ok(step)=>step,Err(failure)=>{let mut progress=failure.reason.retained_progress();if failure.written_bytes==1{self.page.as_mut().unwrap().push(byte[0]);progress.copied_bytes+=1;}else if failure.written_bytes!=0{return Err(invalid("native Pack refusal exposed an invalid output prefix").with_retained_progress(progress))}return Err(failure.reason.with_retained_progress(progress))}};let mut progress=step.progress;if step.written_bytes!=0{self.page.as_mut().unwrap().push(byte[0]);progress.copied_bytes+=1;}self.encoded=step.complete;Ok(((),progress))})?;if done.is_some(){self.units+=1;self.publish_progress=self.units.is_multiple_of(4096);}Ok(())
 }
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.publication.terminal_is_empty(){return self.publication.retirement_demands()}
  if self.delivered{return Ok(demand(1))}
  if let Some(cursor)=self.cursor.as_ref(){if !cursor.terminal_is_empty(){return cursor.retirement_demands()}return Ok(RetirementDemand{release_bytes:size_of::<BorrowedPackDocumentCursor>(),..demand(size_of::<Option<Box<BorrowedPackDocumentCursor>>>())})}
  if let Some(page)=self.page.as_ref(){return Ok(RetirementDemand{release_bytes:page.capacity(),..demand(size_of::<Option<Vec<u8>>>())})}
  if self.pending.is_some()||self.pending_close.is_some(){return plugin_typed_owner_demand(&self.pending,&self.pending_close,body)}
  if self.outputs.is_some()||self.outputs_close.is_some(){return plugin_typed_owner_demand(&self.outputs,&self.outputs_close,body)}
  if self.snapshot.is_some(){return Ok(demand(size_of::<Arc<A::Snapshot>>()))}
  if let Some(lease)=self.lease.as_ref(){return Ok(if lease.terminal_is_empty(){demand(1)}else{lease.retirement_demands()})}
  if self.metadata.is_some()||self.metadata_close.is_some(){return plugin_typed_owner_demand(&self.metadata,&self.metadata_close,body)}Ok(Default::default())
 }
 fn close_turn(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let d=self.close_demand(grant.maximum_copy_bytes)?;if grant.maximum_items==0||!plugin_grant_funds(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()))}let item=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
  if !self.publication.terminal_is_empty(){return self.publication.close_step(grant)}if self.delivered{self.delivered=false;return Ok(RetainedCloneStep::Progress(item))}
  if let Some(cursor)=self.cursor.as_mut(){if !cursor.terminal_is_empty(){return cursor.close(grant).map(|step|RetainedCloneStep::Progress(step.progress)).map_err(|failure|failure.reason)}self.cursor.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{released_bytes:d.release_bytes,..item}))}
  if self.page.is_some(){self.page.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{released_bytes:d.release_bytes,..item}))}
  if self.pending.is_some()||self.pending_close.is_some(){return plugin_typed_owner_step(&mut self.pending,&mut self.pending_close,grant)}if self.outputs.is_some()||self.outputs_close.is_some(){return plugin_typed_owner_step(&mut self.outputs,&mut self.outputs_close,grant)}
  if let Some(snapshot)=self.snapshot.as_ref(){if !self.lease.as_ref().is_some_and(|lease|lease.can_release(snapshot)){return Err(invalid("native Pack writer lost its original snapshot close supplier"))}self.snapshot.take();return Ok(RetainedCloneStep::Progress(item))}
  if let Some(lease)=self.lease.as_mut(){if lease.terminal_is_empty(){self.lease.take();return Ok(RetainedCloneStep::Progress(item))}return lease.close_step(grant)}
  if self.metadata.is_some()||self.metadata_close.is_some(){return plugin_typed_owner_step(&mut self.metadata,&mut self.metadata_close,grant)}Ok(RetainedCloneStep::Complete(Default::default()))
 }
}
impl<A:ArtifactApp> InteractiveJob for NativeSnapshotPackExportJob<A> where A::Snapshot:BorrowedDslRecord+FieldProjectionSource{
 fn step<'a>(&'a mut self,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure>{
  if self.delivered{let _step=semio_framework_job::close_original_retained_step(cx,self.publication.retirement_demands()?,|grant|self.publication.close_step(grant))?;if self.publication.terminal_is_empty(){self.delivered=false;self.publish_progress=false;}return Ok(None)}
  if self.closing||cx.is_cancelled(){return JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into)}if self.completed{return JobOutcomeBorrow::admit_complete(cx,None,None).map_err(Into::into)}if cx.should_yield(){return Ok(None)}
  if self.publish_progress{let result=self.publication.advance_from_source(semio_framework_job_publication::JobPublicationKind::Checkpoint{applied_progress:self.units},b"{\"stage\":\"snapshot-pack\"}",cx)?;if result.is_some(){self.delivered=true;}return Ok(result)}
  cx.set_stage("snapshot-pack");while !cx.should_yield(){let fuel=cx.fuel_remaining();self.advance(cx)?;if self.completed||self.publish_progress||cx.fuel_remaining()==fuel{break}}Ok(None)
 }
 fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{match descriptor.kind(){JobOutcomeKind::Cancelled=>descriptor.cancelled(),JobOutcomeKind::Complete if self.completed=>descriptor.complete(None,None),JobOutcomeKind::Yield=>descriptor.yielded(),_=>self.publication.borrow_outcome(descriptor)}}
 fn begin_close(&mut self){self.closing=true;}
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(0)?.copy_bytes)}
 fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demand(body)?.capacity_bytes)}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(0)?.release_bytes)}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(0)?.depth)}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{self.closing=true;let d=self.close_demand(cx.retained_grant().maximum_copy_bytes)?;let step=semio_framework_job::close_original_retained_step(cx,d,|grant|self.close_turn(grant))?;Ok(if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress:step.progress()}}else{step})}
 fn terminal_is_empty(&self)->bool{self.cursor.is_none()&&self.page.is_none()&&self.snapshot.is_none()&&self.lease.is_none()&&self.outputs.is_none()&&self.outputs_close.is_none()&&self.metadata.is_none()&&self.metadata_close.is_none()&&self.pending.is_none()&&self.pending_close.is_none()&&self.publication.terminal_is_empty()&&!self.delivered}
}
impl<A:ArtifactApp> ArtifactReservedJob for NativeSnapshotPackExportJob<A> where A::Snapshot:BorrowedDslRecord+FieldProjectionSource{}
impl<A:ArtifactApp> Drop for NativeSnapshotPackExportJob<A> where A::Snapshot:BorrowedDslRecord+FieldProjectionSource{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native Pack export requires original paid close");}}
pub struct NativeSnapshotPackExportFactory<A:ArtifactApp> where A::Snapshot:BorrowedDslRecord+FieldProjectionSource{keys:[ToolFactoryKey;1],owner:std::marker::PhantomData<fn()->A>}
impl<A:ArtifactApp> NativeSnapshotPackExportFactory<A> where A::Snapshot:BorrowedDslRecord+FieldProjectionSource{pub fn new(controller:&str)->Self{Self{keys:[ToolFactoryKey::new(controller,SNAPSHOT_PACK_TOOL)],owner:std::marker::PhantomData}}}
impl<A:ArtifactApp> ToolJobFactory for NativeSnapshotPackExportFactory<A> where A::Snapshot:BorrowedDslRecord+FieldProjectionSource{type Payload=ArtifactReservedToolJob;type Job=ArtifactReservedToolJob;fn keys(&self)->&[ToolFactoryKey]{&self.keys}fn payload_schema_id(&self)->&str{SNAPSHOT_PACK_SCHEMA}fn classification(&self)->InteractiveJobClassification{InteractiveJobClassification::Migrated}fn execution_contract(&self)->ToolExecutionContract{SNAPSHOT_PACK_CONTRACT}fn create_job(&mut self,_:semio_framework_job::Operation,payload:Self::Payload)->Result<Self::Job,ToolJobFactoryError>{Ok(payload)}}
impl<A:ArtifactApp> ArtifactOwnedToolJobFactory for NativeSnapshotPackExportFactory<A> where A::Snapshot:BorrowedDslRecord+FieldProjectionSource{type Owner=A;const TOOL_IDS:&'static[&'static str]=&[SNAPSHOT_PACK_TOOL];const DOCUMENT_SCHEMA:&'static str=A::DOCUMENT_SCHEMA;const PUBLICATION_CONTRACTS:&'static[ArtifactToolPublicationContract]=&[ArtifactToolPublicationContract{tool_id:SNAPSHOT_PACK_TOOL,lanes:&[ArtifactToolPublicationLane::HostOnly]}];}
