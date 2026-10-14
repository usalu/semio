//! 👷️ The tool job contract an app declares: bounded proofs, factories and registrations, instance operation owners, owned and reserved job requests, completions and faults.
use semio_framework_diagnostic::FaultFrom;
use semio_framework_value_derive::{FromValue, ToValue};
use crate::gesture_contract::GestureSlot;
use crate::gesture_contract::GestureCapture;
use crate::tool_run_contract::ToolRunView;
use semio_framework_job::JobOutcomeBorrow;
use semio_framework_job::JobOutcomeDescriptor;
use semio_framework_job::JobOutcomeKind;
use semio_framework_job::JobOutcomeView;
use semio_framework_job::JobPayloadStream;
use semio_framework_job::JobPublicationKind;
use semio_framework_job::RetainedJobPublication;
use semio_framework_job::RetainedPayloadBuilder;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use std::mem::ManuallyDrop;
use semio_framework_job::StepContext;
use crate::window_config::WindowConfigSnapshot;
use crate::window_transient::WindowTransientSnapshot;
use semio_framework_value::DslValue;
use semio_framework::Fault;
use semio_framework::FaultCode;
use semio_framework::FaultOrigin;
use semio_framework::ViewModel;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::Terminology;
use semio_framework::Media;
use semio_framework_value::RetirementDemand;
use semio_framework_value::ValueError;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneStep;
use crate::completion_fault::ArtifactCompletionFault;
use store::SnapshotReadProjection as ArtifactContextRoot;
use crate::owned_context::ArtifactOwnedContextHandle;
use semio_framework_async::PublicationClaimHandle;
use crate::app_contract::ArtifactApp;
use crate::app_emission::EphemeralEmit;
use crate::app_views::InteractionHoverState;
use crate::app_views::PresencePeersView;
use crate::app_views::PresenceView;
use crate::bounded_ownership::PluginLifecycleStep;
use crate::bounded_ownership::PluginLifecycleFailure;
use crate::child_view::ChildContentView;
use crate::history_view::HistoryView;
use crate::media_contract::ArtifactDownloadOutput;
use crate::media_contract::ArtifactOutputChunks;

use crate::sdk_fault::plugin_sdk_fault;
use crate::store_lifecycle::ArtifactDisposal;
use crate::app_emission::ArtifactMutationOutcome;
use crate::tool_cancellation::ToolCancellationLease;
use semio_framework_job::InteractiveJobCloseStep;

pub(crate) const BOUNDED_FIRST_STEP_FACTORY: &str = "BoundedFirstStepCommandJobFactory";

/// 📜️ One owner-local bounded reducer proof. The constructor derives the concrete Rust
/// owner witness; copied controller, schema, file, or tool strings cannot grant authority.
#[derive(Clone)]
pub struct ArtifactBoundedFirstStepProof {
    pub(crate) owner_file: &'static str,
    pub(crate) owner: ToolOwnerWitness,
    pub(crate) controller_id: &'static str,
    pub(crate) factory: &'static str,
    pub(crate) factory_type_id: Option<std::any::TypeId>,
    pub(crate) factory_type_name: Option<&'static str>,
    pub(crate) tool_id: &'static str,
    pub(crate) artifact_schema: &'static str,
    pub(crate) contract: semio_framework::ToolExecutionContract,
}

impl ArtifactBoundedFirstStepProof {
    /// 🏷️ Identifies the tool described by this immutable proof.
    pub fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    pub fn new<A: ArtifactApp>(owner_file: &'static str, controller_id: &'static str, factory: &'static str, tool_id: &'static str, artifact_schema: &'static str, contract: semio_framework::ToolExecutionContract) -> Self {
        Self { owner_file, owner: ToolOwnerWitness::of::<A>(), controller_id, factory, factory_type_id: None, factory_type_name: None, tool_id, artifact_schema, contract }
    }

    pub fn with_factory_type<A: ArtifactApp, F: ArtifactOwnedToolJobFactory<Owner = A>>(mut self) -> Self {
        assert_eq!(self.owner, ToolOwnerWitness::of::<A>(), "proof factory belongs to its exact concrete app owner");
        self.factory_type_id = Some(std::any::TypeId::of::<F>());
        self.factory_type_name = Some(std::any::type_name::<F>());
        self
    }
}

/// 🧬️ Declares an exact bounded-first-step proof catalog in the owning plugin source.
#[macro_export]
macro_rules! bounded_first_step_tool_proofs {
    (
        owner: $owner:ty,
        owner_file: $owner_file:literal,
        controller: $controller:literal,
        artifact_schema: $artifact_schema:literal,
        factory: $factory:literal,
        $(factory_type: $factory_type:ty,)?
        contract: $contract:expr,
        tools: [$($tool:literal),+ $(,)?]
    ) => {
        fn bounded_first_step_tool_proofs() -> Vec<$crate::ArtifactBoundedFirstStepProof> {
            let qualify = |proof: $crate::ArtifactBoundedFirstStepProof| {
                $(let proof = proof.with_factory_type::<$owner, $factory_type>();)?
                proof
            };
            vec![$(
                qualify($crate::ArtifactBoundedFirstStepProof::new::<$owner>(
                    $owner_file,
                    $controller,
                    $factory,
                    $tool,
                    $artifact_schema,
                    $contract,
                ))
            ),+]
        }
    };
    (
        owner: $owner:ty,
        owner_file: $owner_file:literal,
        controller: $controller:literal,
        artifact_schema: $artifact_schema:literal,
        factory: $factory:literal,
        $(factory_type: $factory_type:ty,)?
        tools: { $($tool:literal => $contract:expr),+ $(,)? }
    ) => {
        fn bounded_first_step_tool_proofs() -> Vec<$crate::ArtifactBoundedFirstStepProof> {
            let qualify = |proof: $crate::ArtifactBoundedFirstStepProof| {
                $(let proof = proof.with_factory_type::<$owner, $factory_type>();)?
                proof
            };
            vec![$(
                qualify($crate::ArtifactBoundedFirstStepProof::new::<$owner>(
                    $owner_file,
                    $controller,
                    $factory,
                    $tool,
                    $artifact_schema,
                    $contract,
                ))
            ),+]
        }
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArtifactToolPublicationLane {
    HostOnly,
    Artifact,
    Config,
    Draft,
    Presence,
    Transient,
    WindowConfig,
    WindowTransient,
    Child,
    /// 🕹️ This tool's emit may carry [`InteractionWrite`]s — the framework-owned selection lane
    /// (`interaction_store`, `HistoryLane::Interaction`). Never a document/config lane: it is
    /// persisted-local and excluded from undo/redo by declaration.
    Interaction,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactToolPublicationContract {
    pub tool_id: &'static str,
    pub lanes: &'static [ArtifactToolPublicationLane],
}

/// 🔐️ A plugin factory's concrete owner is part of its Rust type, so a copied `ArtifactApp`
/// cannot reuse the factory by copying controller/schema/tool constants.
pub trait ArtifactOwnedToolJobFactory: semio_framework::ToolJobFactory {
    type Owner: ArtifactApp;
    const TOOL_IDS: &'static [&'static str];
    const DOCUMENT_SCHEMA: &'static str;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[];

    fn latest_wins_target(_command: &<Self::Owner as ArtifactApp>::Command) -> Option<&str> {
        None
    }

    fn build_latest_wins_command_disposer() -> ArtifactDisposal<<Self::Owner as ArtifactApp>::Command> {
        None
    }
}

/// 🧳️ Object-safe app-instance operation state retained and closed by [VcsArtifactApp].
pub trait ArtifactInstanceOperationOwner: std::any::Any + Send {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    /// 📏️ Exact per-axis grant the owner's next maintenance or close turn needs.
    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError>;
    fn maintenance_step(&mut self,cx:&mut StepContext<'_>,loan:RetainedCloneGrant)->Result<PluginLifecycleStep,PluginLifecycleFailure>;
    fn close_step(&mut self,cx:&mut StepContext<'_>,loan:RetainedCloneGrant)->Result<PluginLifecycleStep,PluginLifecycleFailure>;
    fn terminal_is_empty(&self) -> bool;
}

pub(crate) struct EmptyArtifactInstanceOperationOwner;

impl ArtifactInstanceOperationOwner for EmptyArtifactInstanceOperationOwner {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn retirement_demands(&self, _body: usize) -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand::default())
    }

    fn maintenance_step(&mut self,_cx:&mut StepContext<'_>,_loan:RetainedCloneGrant)->Result<PluginLifecycleStep,PluginLifecycleFailure>{
        Ok(PluginLifecycleStep::Complete(Default::default()))
    }

    fn close_step(&mut self,_cx:&mut StepContext<'_>,_loan:RetainedCloneGrant)->Result<PluginLifecycleStep,PluginLifecycleFailure>{
        Ok(PluginLifecycleStep::Complete(Default::default()))
    }

    fn terminal_is_empty(&self) -> bool {
        true
    }
}

/// 🧬️ Mandatory concrete session retirement retains original typed custody until full paid close.
trait OriginalInferenceSessionOwner:Send {
 fn original_mut(&mut self)->Option<&mut dyn std::any::Any>;
 fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,ValueError>;
 fn terminal_is_empty(&self)->bool;
}
struct OriginalInferenceSession<T:semio_framework_value::retirement::RetireOwned>{body:semio_framework_value::retirement::controlled::ControlledRetirement<T>}
impl<T:semio_framework_value::retirement::RetireOwned> OriginalInferenceSessionOwner for OriginalInferenceSession<T>{
 fn original_mut(&mut self)->Option<&mut dyn std::any::Any>{self.body.original_mut().map(|value|value as &mut dyn std::any::Any)}
 fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.body.selected_demand(body)}
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,ValueError>{self.body.step(grant)}
 fn terminal_is_empty(&self)->bool{self.body.terminal_is_empty()}
}
/// 🔮️ Original mounted sessions carry their mandatory typed close producer beside the same value.
#[derive(Default)]
pub(crate) struct InstanceInferenceSessions {
 sessions:semio_framework_value::list::PagedList<(&'static str,Box<dyn OriginalInferenceSessionOwner>),{usize::MAX}>,
 closed:bool,
 closing:bool,
}
impl InstanceInferenceSessions {
 pub(crate) fn terminal_is_empty(&self)->bool{self.closed&&self.sessions.terminal_is_empty()}
 pub(crate) fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some((_,session))=self.sessions.last(){return if session.terminal_is_empty(){Ok(RetirementDemand{release_bytes:std::mem::size_of_val(session.as_ref()),depth:self.sessions.next_pop_depth_demand().map_err(ValueError::from)?,..Default::default()})}else{session.retirement_demands(body)}}
  Ok(RetirementDemand{release_bytes:self.sessions.next_release_allocation_bytes().map_err(ValueError::from)?,depth:if self.sessions.terminal_is_empty(){usize::from(!self.closed)}else{self.sessions.next_release_depth_demand().map_err(ValueError::from)?},..Default::default()})
 }
 /// 🎟️ Advances only the same original data retirement after all independent currencies admit it.
 pub(crate) fn close_granted(&mut self,grant:RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,ValueError>{
  use semio_framework_value::RetainedCloneStep;let demand=self.retirement_demands(grant.maximum_copy_bytes)?;
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()))}
  self.closing=true;
  if let Some((_,session))=self.sessions.last_mut(){
   if session.terminal_is_empty(){let bytes=std::mem::size_of_val(session.as_ref());drop(self.sessions.pop());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}))}
   return session.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))
  }
  if !self.sessions.terminal_is_empty(){let released=self.sessions.release_empty_page(grant.maximum_release_bytes).map_err(ValueError::from)?;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:usize::from(released.progressed),released_bytes:released.released_allocation_bytes,..Default::default()}))}
  self.closed=true;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))
 }
 pub(crate) fn close_step(&mut self,cx:&mut StepContext<'_>,loan:RetainedCloneGrant)->Result<PluginLifecycleStep,PluginLifecycleFailure>{
  let demand=self.retirement_demands(loan.maximum_copy_bytes)?;
  crate::bounded_ownership::original_lifecycle_step(cx,loan,demand,|_,grant|{let step=self.close_granted(grant)?;let terminal=self.terminal_is_empty();Ok((match step{semio_framework_value::RetainedCloneStep::Complete(progress)=>PluginLifecycleStep::Complete(progress),semio_framework_value::RetainedCloneStep::Progress(progress)=>PluginLifecycleStep::Progress(progress)},terminal))})
 }

}
/// 📏️ Quotes the original shared container without reconstructing its mounted Handle.
pub(crate) fn original_inference_close_demands(original:&std::sync::Arc<std::sync::Mutex<InstanceInferenceSessions>>,body:usize)->Result<RetirementDemand,ValueError>{match original.try_lock(){Ok(sessions)=>sessions.retirement_demands(body),Err(std::sync::TryLockError::WouldBlock)=>Ok(Default::default()),Err(std::sync::TryLockError::Poisoned(_))=>Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original inferred session owner is poisoned"))}}
/// 🎟️ Closes the same shared inference owner with the actual original context.
pub(crate) fn original_inference_close_step(original:&std::sync::Arc<std::sync::Mutex<InstanceInferenceSessions>>,cx:&mut StepContext<'_>,loan:RetainedCloneGrant)->Result<PluginLifecycleStep,PluginLifecycleFailure>{match original.try_lock(){Ok(mut sessions)=>sessions.close_step(cx,loan),Err(std::sync::TryLockError::WouldBlock)=>Ok(PluginLifecycleStep::Blocked{reason:"original inferred session remains borrowed"}),Err(std::sync::TryLockError::Poisoned(_))=>Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original inferred session owner is poisoned").into())}}

/// 🧳️ Cloneable capability for one concrete app instance's retained operation owner and its inference sessions.
#[derive(Clone)]
pub struct ArtifactInstanceOperationOwnerHandle {
    inner: std::sync::Arc<semio_framework_plugin_mounted_owner::alias_retirement::MountedOwnerCellV1<dyn ArtifactInstanceOperationOwner>>,
    inference: std::sync::Arc<std::sync::Mutex<InstanceInferenceSessions>>,
}

/// 🧭️ Retains one original caller's mounted lookup and donated typed session between funded turns.
#[derive(Debug,semio_framework_value::RetireOwned)]
pub struct ArtifactInferenceSessionAccess<T:semio_framework_value::retirement::RetireOwned>{
 original:Option<T>,
 source:usize,
 operation:u64,
 generation:u64,
 name_pointer:usize,
 name_length:usize,
 index:usize,
 byte:usize,
 phase:u8,
}
impl<T:semio_framework_value::retirement::RetireOwned> Default for ArtifactInferenceSessionAccess<T>{
 fn default()->Self{Self{original:None,source:0,operation:0,generation:0,name_pointer:0,name_length:0,index:0,byte:0,phase:0}}
}
impl<T:semio_framework_value::retirement::RetireOwned> ArtifactInferenceSessionAccess<T>{
 /// 👁️ Borrows the defining original source while its typed frame awaits admission.
 pub fn original(&self)->Option<&T>{self.original.as_ref()}
 /// 🪹️ Pending typed ownership must close before the access frontier can retire.
 pub fn terminal_is_empty(&self)->bool{self.original.is_none()}
}

fn original_inference_metadata(cx:&mut StepContext<'_>,copy:usize,depth:usize)->Result<bool,semio_framework_job::OriginalTurnFailure>{
 let grant=cx.retained_grant();
 if cx.is_cancelled()||cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_depth<depth{return Ok(false)}
 cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})?;Ok(true)
}

impl ArtifactInstanceOperationOwnerHandle {
    /// 🧳️ Wraps an owner the CALLER built. It confers no authority of its own — `with_mut`
    /// downcasts to the concrete type the caller passed in — so an app's own laws can build the
    /// handle their retained work expects instead of reaching into a live instance
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn new(owner: Box<dyn ArtifactInstanceOperationOwner>) -> Self {
        Self { inner: std::sync::Arc::new(semio_framework_plugin_mounted_owner::alias_retirement::MountedOwnerCellV1::new(owner)), inference: Default::default() }
    }

    /// 🔮️ Explicit synchronous semantic facade of the same registry and typed close owner.
    pub fn with_inference_session_cold<T: semio_framework_value::retirement::RetireOwned, R>(&self, name: &'static str, init: impl FnOnce() -> T, apply: impl FnOnce(&mut T) -> Result<R, Fault>) -> Result<R, Fault> {
        if !T::controlled_retirement_supported(){return Err(Fault::new(FaultOrigin::Framework,FaultCode::new("interactive-job.instance-inference-retirement"),"mounted inference session requires its defining controlled retirement producer"))}
        let mut sessions = self.inference.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if sessions.closed || sessions.closing {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.instance-inference-closed"), "app-instance inference sessions were already closed"));
        }
        let index=match sessions.sessions.iter().position(|(known,_)|*known==name){Some(index)=>index,None=>{let body=semio_framework_value::retirement::controlled::ControlledRetirement::new(init()).unwrap_or_else(|_|panic!("mounted inference session must declare its typed retirement"));sessions.sessions.push((name,Box::new(OriginalInferenceSession{body})));sessions.sessions.len()-1}};
        let session = sessions.sessions[index].1.original_mut().and_then(|value|value.downcast_mut::<T>()).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.instance-inference-type"), "app-instance inference session type does not match its name"))?;
        apply(session)
    }

    /// 🎟️ Looks up and admits the same defining typed session with the actual caller and registry.
    pub fn with_inference_session<T:semio_framework_value::retirement::RetireOwned,R>(&self,name:&'static str,access:&mut ArtifactInferenceSessionAccess<T>,cx:&mut StepContext<'_>,init:impl FnOnce(&mut StepContext<'_>)->Result<Option<T>,semio_framework_job::OriginalTurnFailure>,apply:impl FnOnce(&mut T,&mut StepContext<'_>)->R)->Result<Option<R>,semio_framework_job::OriginalTurnFailure>{
     use semio_framework_job::OriginalTurnFailure;
     use semio_framework_value::ValueRefusalKind;
     cx.original_source_parent()?;
     let refused=|message|ValueError::literal(ValueRefusalKind::InvariantViolated,message);
     let source=std::sync::Arc::as_ptr(&self.inference)as usize;
     if access.phase!=0&&(access.source!=source||access.operation!=cx.operation().0||access.generation!=cx.generation().0||access.name_pointer!=name.as_ptr()as usize||access.name_length!=name.len()){return Err(refused("original mounted inference access source or caller changed").into())}
     if cx.is_cancelled()||cx.should_yield(){return Ok(None)}
     if !T::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"mounted inference source requires its defining controlled retirement producer").into())}
     let mut sessions=match self.inference.try_lock(){Ok(sessions)=>sessions,Err(std::sync::TryLockError::WouldBlock)=>return Ok(None),Err(std::sync::TryLockError::Poisoned(_))=>return Err(refused("original mounted inference registry is poisoned").into())};
     if sessions.closed||sessions.closing{return Err(refused("original mounted inference registry is closing").into())}
     match access.phase{
      0=>{if original_inference_metadata(cx,0,1)?{access.source=source;access.operation=cx.operation().0;access.generation=cx.generation().0;access.name_pointer=name.as_ptr()as usize;access.name_length=name.len();access.phase=1}Ok(None)},
      1=>{
       let depth=sessions.sessions.next_get_depth_demand(access.index).map_err(ValueError::from)?.max(1);
       let Some((known,_))=sessions.sessions.get(access.index)else{if original_inference_metadata(cx,0,1)?{access.phase=2}return Ok(None)};
       if access.byte<known.len().min(name.len()){
        if !original_inference_metadata(cx,1,depth)?{return Ok(None)}
        if known.as_bytes()[access.byte]==name.as_bytes()[access.byte]{access.byte+=1}else{access.index+=1;access.byte=0}return Ok(None)
       }
       if original_inference_metadata(cx,0,depth)?{if known.len()==name.len(){access.phase=5}else{access.index+=1;access.byte=0}}Ok(None)
      },
      2=>{
       if sessions.sessions.has_reserved_slot(){if original_inference_metadata(cx,0,1)?{access.phase=if access.original.is_some(){4}else{3}}return Ok(None)}
       if cx.should_yield(){return Ok(None)}
       let grant=cx.retained_grant();
       match sessions.sessions.reserve_one_funded(grant){
        Ok(progress)=>{if progress.copied_items!=0{cx.consume_fuel(1);cx.consume_retained(progress)?}Ok(None)},
        Err(error)=>{cx.consume_fuel(1);Err(OriginalTurnFailure::debit_original(ValueError::from(error.refusal()).with_retained_progress(RetainedCloneProgress{retained_capacity_bytes:error.allocated_bytes,..Default::default()}),cx))}
       }
      },
      3=>{let before=cx.fuel_remaining();if let Some(original)=init(cx)?{access.original=Some(original);access.phase=4;if cx.fuel_remaining()==before{return Err(refused("defining mounted inference constructor omitted original work receipt").into())}}Ok(None)},
      4=>{
       if !sessions.sessions.has_reserved_slot(){if original_inference_metadata(cx,0,1)?{access.phase=2}return Ok(None)}
       let grant=cx.retained_grant();let layout=std::alloc::Layout::new::<OriginalInferenceSession<T>>();
       if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth<sessions.sessions.next_push_depth_demand().map_err(ValueError::from)?.max(1)||grant.maximum_capacity_bytes<layout.size(){return Ok(None)}
       cx.consume_fuel(1);
       let Some(pointer)=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}.cast::<OriginalInferenceSession<T>>())else{return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"original mounted inference typed frame allocation refused").into())};
       let body=match semio_framework_value::retirement::controlled::ControlledRetirement::new(access.original.take().expect("original inference source retained before typed birth")){Ok(body)=>body,Err((error,original))=>{access.original=Some(original);unsafe{std::alloc::dealloc(pointer.as_ptr().cast(),layout)};return Err(OriginalTurnFailure::debit_original(error.with_retained_progress(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:layout.size(),released_bytes:layout.size(),..Default::default()}),cx))}};
       let owner=unsafe{pointer.as_ptr().write(OriginalInferenceSession{body});Box::from_raw(pointer.as_ptr())};
       access.index=sessions.sessions.len();sessions.sessions.push_reserved((name,owner)).unwrap_or_else(|_|panic!("original mounted inference row was admitted"));access.phase=5;
       cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:layout.size(),..Default::default()})?;Ok(None)
      },
      5=>{let depth=sessions.sessions.next_get_depth_demand(access.index).map_err(ValueError::from)?;let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_depth<=depth{return Ok(None)}let session=sessions.sessions.get_mut(access.index).and_then(|(_,owner)|owner.original_mut()).and_then(|original|original.downcast_mut::<T>()).ok_or_else(||refused("original mounted inference source type changed"))?;let mut output=None;cx.with_retained_loan(RetainedCloneGrant{maximum_depth:grant.maximum_depth-depth,..grant},|cx|{output=Some(apply(session,cx));Ok(())})?;Ok(output)},
      _=>Err(refused("original mounted inference admission phase is invalid").into())
     }
    }

    /// 📏️ Quotes this actual registry lookup, page and frame event before its defining constructor or child.
    pub fn inference_admission_demands<T:semio_framework_value::retirement::RetireOwned>(&self,access:&ArtifactInferenceSessionAccess<T>)->Result<RetirementDemand,ValueError>{
     let sessions=match self.inference.try_lock(){Ok(sessions)=>sessions,Err(std::sync::TryLockError::WouldBlock)=>return Ok(Default::default()),Err(std::sync::TryLockError::Poisoned(_))=>return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original mounted inference registry is poisoned"))};
     Ok(match access.phase{
      0|3=>RetirementDemand{depth:1,..Default::default()},
      1=>RetirementDemand{copy_bytes:sessions.sessions.get(access.index).filter(|(name,_)|access.byte<name.len().min(access.name_length)).map_or(0,|_|1),depth:sessions.sessions.next_get_depth_demand(access.index).map_err(ValueError::from)?.max(1),..Default::default()},
      2=>if sessions.sessions.has_reserved_slot(){RetirementDemand{depth:1,..Default::default()}}else{RetirementDemand{copy_bytes:sessions.sessions.next_reserve_copy_byte_demand().map_err(ValueError::from)?,capacity_bytes:sessions.sessions.next_allocation_bytes().map_err(ValueError::from)?,depth:sessions.sessions.next_reserve_depth_demand().map_err(ValueError::from)?,..Default::default()}},
      4=>if sessions.sessions.has_reserved_slot(){RetirementDemand{capacity_bytes:std::mem::size_of::<OriginalInferenceSession<T>>(),depth:sessions.sessions.next_push_depth_demand().map_err(ValueError::from)?.max(1),..Default::default()}}else{RetirementDemand{depth:1,..Default::default()}},
      5=>RetirementDemand{depth:sessions.sessions.next_get_depth_demand(access.index).map_err(ValueError::from)?.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original inference child depth overflow"))?,..Default::default()},
      _=>return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original inference admission phase is invalid"))
     })
    }

    pub(crate) fn inference_is_empty(&self) -> bool {
        self.inference.try_lock().is_ok_and(|sessions|sessions.terminal_is_empty())
    }

    /// 📏️ Borrows the original concrete session close producer under its existing Arc.
    pub(crate) fn inference_retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{original_inference_close_demands(&self.inference,body)}
    /// 🎟️ Advances one actual caller-funded step of the same original session owner.
    pub(crate) fn close_inference_step(&self,cx:&mut StepContext<'_>,loan:RetainedCloneGrant)->Result<PluginLifecycleStep,PluginLifecycleFailure>{original_inference_close_step(&self.inference,cx,loan)}

    pub fn with_mut<T: std::any::Any, R>(&self, apply: impl FnOnce(&mut T) -> Result<R, Fault>) -> Result<R, Fault> {
        let mut owner = self.inner.try_lock().map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.instance-owner-busy"), "app-instance operation owner is busy or poisoned"))?;
        let owner = owner.as_any_mut().downcast_mut::<T>().ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.instance-owner-type"), "app-instance operation owner type does not match its concrete app"))?;
        apply(owner)
    }

    /// 🔎️ Quotes the same original concrete owner without manufacturing a diagnostic on contention.
    pub fn inspect_original<T:std::any::Any,R>(&self,read:impl FnOnce(&T)->R)->Result<Option<R>,ValueError>{
        let mut owner=match self.inner.try_lock(){Ok(owner)=>owner,Err(semio_framework_plugin_mounted_owner::alias_retirement::MountedOwnerCellErrorV1::Busy)=>return Ok(None),Err(semio_framework_plugin_mounted_owner::alias_retirement::MountedOwnerCellErrorV1::Poisoned)=>return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original instance owner is poisoned"))};
        let source=owner.as_any_mut().downcast_ref::<T>().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original instance owner type changed"))?;
        Ok(Some(read(source)))
    }

    /// ⏳️ A WouldBlock is not a fault. A retained command job runs its `step` on a process-pool
    /// worker thread and holds this lock while it does, so the reactor's own maintenance turn on
    /// the main thread legitimately meets a busy owner — reporting that as
    /// `interactive-job.instance-owner-busy` failed the whole in-flight dispatch every time the
    /// two raced. Only a POISONED owner (a panic inside `with_mut`) is a fault; a busy one is
    /// simply no progress this turn (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    fn instance_owner_poisoned_fault() -> Fault {
        Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.instance-owner-poisoned"), "app-instance operation owner is poisoned")
    }

    pub(crate) fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        match self.inner.try_lock() {
            Ok(owner) => if owner.terminal_is_empty(){self.inference_retirement_demands(body)}else{owner.retirement_demands(body)},
            Err(semio_framework_plugin_mounted_owner::alias_retirement::MountedOwnerCellErrorV1::Busy) => Ok(RetirementDemand::default()),
            Err(semio_framework_plugin_mounted_owner::alias_retirement::MountedOwnerCellErrorV1::Poisoned) => Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "app-instance operation owner is poisoned")),
        }
    }

    pub(crate) fn maintenance_step(&self,cx:&mut StepContext<'_>,loan:RetainedCloneGrant)->Result<PluginLifecycleStep,PluginLifecycleFailure>{
        match self.inner.try_lock() {
            Ok(mut owner) => {let grant=crate::bounded_ownership::original_lifecycle_grant(cx,loan);let demand=owner.retirement_demands(grant.maximum_copy_bytes)?;crate::bounded_ownership::original_lifecycle_step(cx,loan,demand,|cx,grant|owner.maintenance_step(cx,grant).map(|step|(step,true)))},
            Err(semio_framework_plugin_mounted_owner::alias_retirement::MountedOwnerCellErrorV1::Busy) => Ok(PluginLifecycleStep::Progress(Default::default())),
            Err(semio_framework_plugin_mounted_owner::alias_retirement::MountedOwnerCellErrorV1::Poisoned) => Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"app-instance operation owner is poisoned").into()),
        }
    }

    pub(crate) fn close_step(&self,cx:&mut StepContext<'_>,loan:RetainedCloneGrant)->Result<PluginLifecycleStep,PluginLifecycleFailure>{
        match self.inner.try_lock() {
            Ok(mut owner) => {if owner.terminal_is_empty(){drop(owner);return self.close_inference_step(cx,loan)}let grant=crate::bounded_ownership::original_lifecycle_grant(cx,loan);let demand=owner.retirement_demands(grant.maximum_copy_bytes)?;crate::bounded_ownership::original_lifecycle_step(cx,loan,demand,|cx,grant|{let step=owner.close_step(cx,grant)?;let terminal=owner.terminal_is_empty();Ok((match step{PluginLifecycleStep::Complete(progress)=>PluginLifecycleStep::Progress(progress),step=>step},terminal))})},
            Err(semio_framework_plugin_mounted_owner::alias_retirement::MountedOwnerCellErrorV1::Busy) => Ok(PluginLifecycleStep::Progress(Default::default())),
            Err(semio_framework_plugin_mounted_owner::alias_retirement::MountedOwnerCellErrorV1::Poisoned) => Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"app-instance operation owner is poisoned").into()),
        }
    }

    pub(crate) fn terminal_is_empty(&self) -> Result<bool, Fault> {
        match self.inner.try_lock() {
            Ok(owner) => Ok(owner.terminal_is_empty()),
            Err(semio_framework_plugin_mounted_owner::alias_retirement::MountedOwnerCellErrorV1::Busy) => Ok(false),
            Err(semio_framework_plugin_mounted_owner::alias_retirement::MountedOwnerCellErrorV1::Poisoned) => Err(Self::instance_owner_poisoned_fault()),
        }
    }

    /// 🧹️ The close witness: the operation owner is empty AND the instance holds no inference session.
    pub(crate) fn close_terminal_is_empty(&self) -> Result<bool, Fault> {
        Ok(self.terminal_is_empty()? && self.inference_is_empty())
    }
}

/// 🔗️ One original command alias keeps final instance payload custody until each physical backing closes.
pub struct ArtifactInstanceOperationOwnerAliasRetirement{
    original:semio_framework_plugin_mounted_owner::alias_retirement::MountedOwnerAliasRetirementV1<dyn ArtifactInstanceOperationOwner>,
    inference:ManuallyDrop<Option<std::sync::Arc<std::sync::Mutex<InstanceInferenceSessions>>>>,
    inference_owned:ManuallyDrop<Option<std::sync::Mutex<InstanceInferenceSessions>>>,
}

impl ArtifactInstanceOperationOwnerAliasRetirement {
    /// 📥️ Moves the caller's exact existing handle into an inline frontier without a new lease or frame.
    pub fn new(source: ArtifactInstanceOperationOwnerHandle) -> Self {
        Self{original:semio_framework_plugin_mounted_owner::alias_retirement::MountedOwnerAliasRetirementV1::new(source.inner),inference:ManuallyDrop::new(Some(source.inference)),inference_owned:ManuallyDrop::new(None)}
    }
    /// 📏️ Initial custody transfer needs one item and depth while all heap currencies remain zero.
    pub fn constructor_demands() -> RetirementDemand {
        semio_framework_plugin_mounted_owner::alias_retirement::MountedOwnerAliasRetirementV1::<dyn ArtifactInstanceOperationOwner>::constructor_demands()
    }
    /// 🔎️ Queries the same retained payload or actual backing without creating authority.
    pub fn demands(&self, copy: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(owner)=self.original.unique() {
            if !owner.terminal_is_empty() {
                let mut demand=owner.retirement_demands(copy)?;
                demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"instance alias payload depth overflow"))?;
                return Ok(demand);
            }
        }
        if !self.original.terminal_is_empty(){return Ok(self.original.demands())}
        if let Some(original)=self.inference_owned.as_ref(){let sessions=original.try_lock().map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original inferred alias retains its source"))?;return if sessions.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{sessions.retirement_demands(copy)}}
        self.alias_return_demands(copy)
    }
    /// 🔗️ Quotes only the original alias or terminal box shell without authorizing active payload work.
    pub fn alias_return_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if !self.original.terminal_is_empty(){return Ok(self.original.demands())}
        if self.inference.is_some(){return Ok(RetirementDemand{release_bytes:semio_framework_value::shared_retirement_allocation_bytes::<std::sync::Mutex<InstanceInferenceSessions>>(),depth:1,..Default::default()})}
        if let Some(original)=self.inference_owned.as_ref(){let sessions=original.try_lock().map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original inferred alias retains its exclusive source"))?;return if sessions.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{sessions.retirement_demands(copy)}}
        Ok(Default::default())
    }
    /// 📤️ Atomically returns the actual alias and retains any final original inferred payload.
    pub fn alias_return_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
        let demand=self.alias_return_demands(grant.maximum_copy_bytes)?;if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()))}
        if !self.original.terminal_is_empty(){return self.original.step(grant,ArtifactInstanceOperationOwner::terminal_is_empty).map(|step|RetainedCloneStep::Progress(step.progress()))}
        if let Some(original)=self.inference.as_ref(){if std::sync::Arc::weak_count(original)!=0{return Ok(RetainedCloneStep::Progress(Default::default()))}let released=if let Some(value)=std::sync::Arc::into_inner(self.inference.take().unwrap()){*self.inference_owned=Some(value);demand.release_bytes}else{0};return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:released,..Default::default()}))}
        if let Some(original)=self.inference_owned.as_mut(){let sessions=original.get_mut().map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original inferred alias source is poisoned"))?;if !sessions.terminal_is_empty(){return sessions.close_granted(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}drop(self.inference_owned.take());return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))}
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
    /// 🧾️ Closes shared backing, exact final owner and dynamic box in separate funded turns.
    pub fn step(&mut self,cx:&mut StepContext<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,PluginLifecycleFailure>{
        let demand=self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        if let Some(owner)=self.original.unique_mut() {
            if !owner.terminal_is_empty() {
                let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
                let step=owner.close_step(cx,child)?;
                let progress=step.progress().unwrap_or_default();
                let outcome=if matches!(step,PluginLifecycleStep::Complete(_)){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)};
                let accepted=semio_framework_value::retained_clone::admit_retained_clone_close(child,outcome,owner.terminal_is_empty(),"original instance alias final payload")?;
                return Ok(RetainedCloneStep::Progress(accepted.progress()));
            }
        }
        self.alias_return_step(grant).map_err(PluginLifecycleFailure::from)
    }
    /// 🏁️ The original alias, final concrete payload and its dynamic box shell are all physically gone.
    pub fn terminal_is_empty(&self)->bool{self.original.terminal_is_empty()&&self.inference.is_none()&&self.inference_owned.is_none()}
}
impl Drop for ArtifactInstanceOperationOwnerAliasRetirement{
    fn drop(&mut self){
        assert!(std::thread::panicking()||self.terminal_is_empty(),"original instance alias abandoned inferred owner custody");
        if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.inference);ManuallyDrop::drop(&mut self.inference_owned);}}
    }
}

#[derive(Clone)]
pub(crate) struct ArtifactToolRegistration {
    pub(crate) owner: ToolOwnerWitness,
    pub(crate) key: semio_framework::ToolFactoryKey,
    pub(crate) schema_id: String,
    pub(crate) contract: semio_framework::ToolExecutionContract,
    pub(crate) factory_type_id: std::any::TypeId,
    pub(crate) factory_type_name: &'static str,
    pub(crate) publication_lanes: &'static [ArtifactToolPublicationLane],
    pub(crate) latest_wins_target: for<'a> fn(&'a dyn std::any::Any) -> Option<&'a str>,
    pub(crate) latest_wins_command_disposer: fn() -> Option<Box<dyn std::any::Any + Send>>,
}

/// 🧾️ Public predecode projection of one exact app-contributed registration. It contains no
/// factory handle and therefore cannot grant execution authority by itself.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactToolPublicContract {
    pub(crate) owner: ToolOwnerWitness,
    pub(crate) controller_id: String,
    pub(crate) tool_id: String,
    pub(crate) schema_id: String,
    pub(crate) max_raw_wire_bytes: usize,
    pub(crate) publication_lanes: &'static [ArtifactToolPublicationLane],
}

/// 🧰️ Compiler-bound registration context passed only to `ArtifactApp`'s explicit hook.
pub struct ArtifactToolFactoryRegistry<'a, A: ArtifactApp> {
    bus: &'a semio_framework::ActionBus,
    controller_id: &'a str,
    registrations: BTreeMap<String, ArtifactToolRegistration>,
    marker: std::marker::PhantomData<A>,
}

impl<'a, A: ArtifactApp> ArtifactToolFactoryRegistry<'a, A> {
    pub(crate) fn new(bus: &'a semio_framework::ActionBus, controller_id: &'a str) -> Self {
        Self { bus, controller_id, registrations: BTreeMap::new(), marker: std::marker::PhantomData }
    }

    pub fn controller_id(&self) -> &str {
        self.controller_id
    }

    pub fn register<F>(&mut self, factory: F) -> Result<(), Fault>
    where
        F: ArtifactOwnedToolJobFactory<Owner = A>,
    {
        if F::DOCUMENT_SCHEMA != A::DOCUMENT_SCHEMA || factory.payload_schema_id().trim().is_empty() {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.owner-schema"), "app-owned factory schema does not match its concrete ArtifactApp owner"));
        }
        let expected = F::TOOL_IDS.iter().copied().collect::<BTreeSet<_>>();
        let actual = factory.keys().iter().map(|key| key.tool_id.as_str()).collect::<BTreeSet<_>>();
        if expected.is_empty() || expected != actual || factory.keys().iter().any(|key| key.controller_id != self.controller_id) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.owner-key"), "app-owned factory keys are not an exact live-controller/tool bijection"));
        }
        let publication = F::PUBLICATION_CONTRACTS.iter().map(|contract| (contract.tool_id, contract.lanes)).collect::<BTreeMap<_, _>>();
        if publication.len() != F::PUBLICATION_CONTRACTS.len()
            || publication.keys().copied().collect::<BTreeSet<_>>() != expected
            || publication.values().any(|lanes| lanes.is_empty() || (lanes.contains(&ArtifactToolPublicationLane::HostOnly) && lanes.len() != 1))
        {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.publication-contract"), "app-owned factory lacks an exact nonempty per-tool publication-lane contract"));
        }
        if factory.classification() != semio_framework::InteractiveJobClassification::Migrated {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.owner-classification"), "app-owned factory must be explicitly Migrated"));
        }
        let owner = ToolOwnerWitness::of::<A>();
        let schema_id = factory.payload_schema_id().to_string();
        let contract = factory.execution_contract();
        let factory_type_id = std::any::TypeId::of::<F>();
        let factory_type_name = std::any::type_name::<F>();
        let keys = factory.keys().to_vec();
        self.bus.register_once(factory).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.owner-registration"), error.to_string()))?;
        for key in keys {
            if self.registrations.contains_key(&key.tool_id) {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.owner-duplicate"), format!("duplicate app-owned tool {}", key.tool_id)));
            }
            let publication_lanes = publication.get(key.tool_id.as_str()).copied().expect("validated exact tool publication contract");
            self.registrations.insert(
                key.tool_id.clone(),
                ArtifactToolRegistration {
                    owner,
                    key,
                    schema_id: schema_id.clone(),
                    contract,
                    factory_type_id,
                    factory_type_name,
                    publication_lanes,
                    latest_wins_target: |command| command.downcast_ref::<<F::Owner as ArtifactApp>::Command>().and_then(F::latest_wins_target),
                    latest_wins_command_disposer: || F::build_latest_wins_command_disposer().map(|owner| Box::new(owner) as Box<dyn std::any::Any + Send>),
                },
            );
        }
        Ok(())
    }

    pub(crate) fn finish(self) -> BTreeMap<String, ArtifactToolRegistration> {
        self.registrations
    }
}

pub(crate) const TYPED_OPERATION_FAULT_BYTES: usize = 480;

pub(crate) const TYPED_OPERATION_FAULT_PAGE_BYTES: usize = TYPED_OPERATION_FAULT_BYTES;

/// 🧾️ Decodes the canonical bounded Fault carried by one typed-operation result page.
pub(crate) fn decode_typed_operation_fault_page(bytes: &[u8]) -> Fault {
    semio_framework_diagnostic::try_decode_fault_bytes(bytes).unwrap_or_else(|| Fault::new(FaultOrigin::Framework, "interactive-job.app-owned-output", String::from_utf8_lossy(bytes)))
}

#[derive(Clone,Copy)]
pub struct ArtifactBoundedToolFault {
    pub(crate) bytes: [u8; TYPED_OPERATION_FAULT_BYTES],
    pub(crate) len: usize,
}

impl ArtifactBoundedToolFault {
    /// 🧾️ Retains a terminal job fault's detail bytes (bounded) so the publication that follows
    /// a cancelled lease reports the real cause instead of a generic cancellation.
    pub(crate) fn from_payload(detail: &semio_framework_job::RetainedJobPayload) -> Self {
        let mut bounded = Self { bytes: [0; TYPED_OPERATION_FAULT_BYTES], len: 0 };
        for page in 0..detail.page_count() {
            let Some(bytes) = detail.page(page) else { break };
            for byte in bytes {
                if bounded.len == bounded.bytes.len() {
                    return bounded;
                }
                bounded.bytes[bounded.len] = *byte;
                bounded.len += 1;
            }
        }
        bounded
    }

    /// 🧾️ Recovers the canonical reducer refusal before retaining its bounded publication frame.
    fn from_retained_fault(detail: &semio_framework_job::RetainedJobPayload) -> Self {
        let payload = Self::from_payload(detail);
        let fault = reducer_fault_of_detail(payload.as_bytes()).unwrap_or_else(|| Fault::new(FaultOrigin::Framework, "interactive-job.app-owned-output", String::from_utf8_lossy(payload.as_bytes())));
        Self::from_fault(&fault)
    }

    pub(crate) fn from_inline_report(bytes:[u8;480],len:usize)->Self{Self{bytes,len}}
    pub fn from_fault(fault: &Fault) -> Self {
        let encoded = semio_framework_diagnostic::encode_fault_bytes_bounded(fault, TYPED_OPERATION_FAULT_BYTES).unwrap_or_else(|| {
            semio_framework_diagnostic::encode_fault_bytes(&Fault::new(FaultOrigin::Framework, "interactive-job.fault-capacity", "the typed operation fault exceeded its bounded result page").with_retained_progress(fault.retained_progress()))
        });
        let mut bounded = Self { bytes: [0; TYPED_OPERATION_FAULT_BYTES], len: encoded.len() };
        bounded.bytes[..encoded.len()].copy_from_slice(&encoded);
        bounded
    }

    pub(crate) fn into_fault(self) -> Fault {
        decode_typed_operation_fault_page(&self.bytes[..self.len])
    }

    /// 🧾️ Copies the canonical Fault wire into the result page that crosses the app frame.
    pub(crate) fn framed_page_bytes(&self, buffer: &mut [u8; TYPED_OPERATION_FAULT_PAGE_BYTES]) -> usize {
        buffer[..self.len].copy_from_slice(&self.bytes[..self.len]);
        self.len
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

semio_framework_value::artifact_retire_leaf!(ArtifactBoundedToolFault);

pub enum ArtifactToolCompletionValue<A: ArtifactApp> {
    Emit(ArtifactMutationOutcome<<A as ArtifactApp>::Mutation, <A as ArtifactApp>::ConfigMutation, <A as ArtifactApp>::DraftMutation, ArtifactCompletionFault>, EphemeralEmit<A>),
    Download(Result<ArtifactDownloadOutput, ArtifactCompletionFault>, EphemeralEmit<A>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub enum TypedOperationResultLane {
    Artifact,
    Config,
    Draft,
    Presence,
    Transient,
    WindowConfig,
    WindowTransient,
    Child,
    Interaction,
    Effect,
    Event,
    Ui,
    Download,
    Terminal,
    Fault,
}

pub(crate) struct OriginalAuthoringEffect {
    receipt: Option<semio_framework_value::native_encoding::NativeEncodeContinuation>,
    token: semio_framework_job::CancelToken,
    publication: PublicationClaimHandle,
    progress: semio_framework_value::native_encoding::NativeEncodeProgress,
}

impl OriginalAuthoringEffect {
    fn run<T>(&mut self, operation: impl FnOnce(&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<T, Fault>) -> Result<T, Fault> {
        let Self { receipt, token, publication, progress } = self;
        let original = receipt.take().ok_or_else(|| plugin_sdk_fault("app-owned effect lacks its original authoring receipt"))?;
        let mut observed = *progress;
        let mut observer = |next: semio_framework_value::native_encoding::NativeEncodeProgress| { observed = next; !token.is_cancelled_now() && !publication.is_cancelled() };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::resume(original, &mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>);
        let result = operation(&mut identity);
        *receipt = Some(identity.pause().map_err(|error| plugin_sdk_fault(error.to_string()))?);
        drop(observer);
        *progress = observed;
        result
    }
}

/// 📬️ Operation-owned completion cell shared by an app factory's job and the exact framework
/// commit path. It is single-assignment and consumed exactly once after freshness validation.
pub struct ArtifactToolCompletion<A: ArtifactApp> {
    inner: super::completion_retirement::CompletionCell<A>,

}

/// 🧬️ Shared retained raw-page lease. Application jobs may inspect one page per step;
/// the mounted operation keeps the sole close authority until every job clone is returned.
#[derive(Clone)]
pub struct ArtifactToolRawInput {
    inner: std::sync::Arc<std::sync::Mutex<Option<semio_framework::action_bus::RetainedToolWireInput>>>,
}

impl ArtifactToolRawInput {
    pub(crate) fn new(input: semio_framework::action_bus::RetainedToolWireInput) -> Self {
        Self { inner: std::sync::Arc::new(std::sync::Mutex::new(Some(input))) }
    }

    pub(crate) fn transferred_to_factory() -> Self {
        Self { inner: std::sync::Arc::new(std::sync::Mutex::new(None)) }
    }

    pub fn page<R>(&self, index: usize, read: impl FnOnce(Option<&[u8]>) -> R) -> Result<R, Fault> {
        let input = self.inner.try_lock().map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.raw-page-busy"), "typed-operation raw page owner is busy or poisoned"))?;
        Ok(read(input.as_ref().and_then(|input| input.page(index))))
    }

    pub fn page_count(&self) -> Result<usize, Fault> {
        let input = self.inner.try_lock().map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.raw-page-busy"), "typed-operation raw page owner is busy or poisoned"))?;
        Ok(input.as_ref().map_or(0, semio_framework::action_bus::RetainedToolWireInput::page_count))
    }

    /// 📏️ Quotes releasing the retained raw page backing whole; a held or busy lease quotes nothing.
    pub(crate) fn retirement_demands(&self) -> Result<RetirementDemand, ValueError> {
        if std::sync::Arc::strong_count(&self.inner) != 1 {
            return Ok(RetirementDemand::default());
        }
        let Ok(input) = self.inner.try_lock() else { return Ok(RetirementDemand::default()) };
        let Some(owner) = input.as_ref() else { return Ok(RetirementDemand::default()) };
        Ok(RetirementDemand { copy_bytes: owner.next_close_copy_byte_demand()?, capacity_bytes: owner.next_close_capacity_byte_demand(0)?, release_bytes: owner.next_close_release_byte_demand()?, depth: owner.next_close_depth_demand()? })
    }

    pub(crate) fn close_step(&self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        if std::sync::Arc::strong_count(&self.inner) != 1 {
            return Ok(PluginLifecycleStep::Blocked { reason: "typed-operation raw page lease remains held by its app-owned decoder job" });
        }
        let mut input = self.inner.try_lock().map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.raw-page-busy"), "typed-operation raw page owner is busy or poisoned"))?;
        let Some(owner) = input.as_mut() else { return Ok(PluginLifecycleStep::Complete(Default::default())) };
        let step = owner.close_step(grant).map_err(crate::bounded_ownership::plugin_retirement_fault)?;
        if owner.terminal_is_empty() {
            *input = None;
        }
        match step {
            InteractiveJobCloseStep::Pending { progress } => Ok(PluginLifecycleStep::Progress(progress)),
            InteractiveJobCloseStep::Blocked => Ok(PluginLifecycleStep::Blocked { reason: "typed-operation raw page close is blocked" }),
            InteractiveJobCloseStep::Complete { progress } => Ok(PluginLifecycleStep::Complete(progress)),
        }
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        std::sync::Arc::strong_count(&self.inner) == 1 && std::sync::Arc::weak_count(&self.inner) == 0 && self.inner.try_lock().is_ok_and(|input| input.is_none())
    }
}

impl<A: ArtifactApp> Clone for ArtifactToolCompletion<A> {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

pub struct ArtifactDownloadCompletionRejection<A:ArtifactApp>{pub download:Result<ArtifactDownloadOutput,Fault>,pub ephemeral:EphemeralEmit<A>,pub fault:Fault}

impl<A:ArtifactApp> std::fmt::Debug for ArtifactDownloadCompletionRejection<A>{fn fmt(&self,formatter:&mut std::fmt::Formatter<'_>)->std::fmt::Result{formatter.debug_struct("ArtifactDownloadCompletionRejection").field("fault",&self.fault).finish_non_exhaustive()}}

pub struct ArtifactToolCompletionRejection<A: ArtifactApp> {
    pub emit: ArtifactMutationOutcome<A::Mutation, A::ConfigMutation, A::DraftMutation>,
    pub ephemeral: EphemeralEmit<A>,
    pub fault: Fault,
}

impl<A: ArtifactApp> std::fmt::Debug for ArtifactToolCompletionRejection<A> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("ArtifactToolCompletionRejection").field("fault", &self.fault).finish_non_exhaustive()
    }
}

impl<A: ArtifactApp> ArtifactToolCompletion<A> {
    /// 🪪️ Runs app-owned physical IO with the mounted operation's original lease and receipt.
    pub fn with_authoring_identity<T>(&self, operation: impl FnOnce(&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<T, Fault>) -> Result<T, Fault> {
        let mut owner = self.inner.try_lock_authoring().map_err(|_| plugin_sdk_fault("original app-owned effect authority is busy or poisoned"))?;
        owner.as_mut().ok_or_else(|| plugin_sdk_fault("app-owned creation requires its original mounted effect authority"))?.run(operation)
    }

    pub(crate) fn install_authoring_identity(&self, receipt: semio_framework_value::native_encoding::NativeEncodeContinuation, lease: &ToolCancellationLease) -> Result<(), Fault> {
        let mut owner = self.inner.try_lock_authoring().map_err(|_| plugin_sdk_fault("original app-owned effect authority is busy or poisoned"))?;
        if owner.is_some() { return Err(plugin_sdk_fault("original app-owned effect authority was already installed")); }
        *owner = Some(OriginalAuthoringEffect { receipt: Some(receipt), token: lease.token.clone(), publication: lease.publication_claim.clone(), progress: semio_framework_value::native_encoding::NativeEncodeProgress { completed: 0, total: 0, owned_bytes: 0 } });
        Ok(())
    }

    pub(crate) fn take_authoring_identity(&self) -> Result<Option<(semio_framework_value::native_encoding::NativeEncodeContinuation, semio_framework_value::native_encoding::NativeEncodeProgress)>, Fault> {
        let mut owner = self.inner.try_lock_authoring().map_err(|_| plugin_sdk_fault("original app-owned effect authority is busy or poisoned"))?;
        let Some(mut original) = owner.take() else { return Ok(None) };
        let receipt = original.receipt.take().ok_or_else(|| plugin_sdk_fault("original app-owned effect receipt is absent"))?;
        Ok(Some((receipt, original.progress)))
    }

    pub(crate) fn retirement_cursor(self)->super::completion_retirement::CompletionRetirement<A>{super::completion_retirement::CompletionRetirement::new(self.inner)}
    /// 🔎️ Compares genuine completion identity without exposing an Arc or Weak capability.
    fn same_original_cell(&self,other:&Self)->bool{self.inner.same_cell(&other.inner)}
    /// 🧪️ Creates a completion cell for an external artifact's registered factory laws.
    #[cfg(feature = "artifact-app-testing")]
    pub fn test_new() -> Self {
        Self::new()
    }

    /// 🧪️ Observes a factory result without granting production code completion consumption.
    #[cfg(feature = "artifact-app-testing")]
    pub fn test_take_emit(&self) -> Result<Option<ArtifactToolEmission<A>>, Fault> {
        self.take_emit()
    }

    pub(crate) fn new() -> Self {
        Self { inner: super::completion_retirement::CompletionCell::new() }
    }

    pub fn has_mounted_consumer(&self) -> bool {
        self.inner.strong_count() > 1
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the original fixed-capacity input so its caller can retry or retire that exact owner without an extra allocation.")]
    pub fn complete(&self, emit: ArtifactMutationOutcome<A::Mutation, A::ConfigMutation, A::DraftMutation>, ephemeral: EphemeralEmit<A>) -> Result<(), ArtifactToolCompletionRejection<A>> {
        let mut value = match self.inner.try_lock() {
            Ok(value) => value,
            Err(_) => {
                return Err(ArtifactToolCompletionRejection { emit, ephemeral, fault: Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.media-completion-busy"), "media completion authority is busy or poisoned") });
            }
        };
        if value.is_some() {
            return Err(ArtifactToolCompletionRejection { emit, ephemeral, fault: Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.duplicate-output"), "app-owned tool job attempted to complete more than once") });
        }
        *value = Some(ArtifactToolCompletionValue::Emit(emit.map_err(ArtifactCompletionFault::new), ephemeral));
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn with_busy_test_lock<R>(&self, test: impl FnOnce() -> R) -> R {
        let _guard = self.inner.lock().expect("completion test lock");
        test()
    }

    pub fn complete_download(&self,download:Result<ArtifactDownloadOutput,Fault>,ephemeral:EphemeralEmit<A>)->Result<(),ArtifactDownloadCompletionRejection<A>>{
        let mut value=match self.inner.try_lock(){Ok(value)=>value,Err(_)=>return Err(ArtifactDownloadCompletionRejection{download,ephemeral,fault:Fault::new(FaultOrigin::Framework,FaultCode::new("interactive-job.tool-completion-busy"),"tool completion authority is busy or poisoned")})};
        if value.is_some(){return Err(ArtifactDownloadCompletionRejection{download,ephemeral,fault:Fault::new(FaultOrigin::Framework,FaultCode::new("interactive-job.duplicate-output"),"app-owned tool job attempted to complete more than once")})}
        *value=Some(ArtifactToolCompletionValue::Download(download.map_err(ArtifactCompletionFault::new),ephemeral));Ok(())
    }

    #[cfg(test)]
    fn test_take_download(&self)->Result<Option<(Result<ArtifactDownloadOutput,Fault>,EphemeralEmit<A>)>,Fault>{match self.take()?{Some(ArtifactToolCompletionValue::Download(output,ephemeral))=>Ok(Some((output.map_err(ArtifactCompletionFault::into_fault),ephemeral))),None=>Ok(None),_=>Err(Fault::from("expected original download completion"))}}

    pub(crate) fn take(&self) -> Result<Option<ArtifactToolCompletionValue<A>>, Fault> {
        self.inner.try_lock().map(|mut value| value.take()).map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.tool-completion-busy"), "tool completion authority is busy or poisoned"))
    }

    pub(crate) fn take_emit(&self) -> Result<Option<ArtifactToolEmission<A>>, Fault> {
        match self.take()? {
            Some(ArtifactToolCompletionValue::Emit(emit, ephemeral)) => Ok(Some((emit.map_err(ArtifactCompletionFault::into_fault), ephemeral))),
            Some(ArtifactToolCompletionValue::Download(_, _)) => Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.unexpected-download-output"), "reserved clipboard/import route completed with a segmented download")),
            None => Ok(None),
        }
    }
}

/// 📸️ State snapshots retained together by one app-owned tool job.
pub struct ArtifactOwnedToolJobSnapshots<A: ArtifactApp> {
    pub children: std::sync::Arc<ChildContentView>,
    pub draft: std::sync::Arc<A::Draft>,
    pub transient: std::sync::Arc<A::Transient>,
    pub window_config: Option<WindowConfigSnapshot>,
    pub window_transient: Option<WindowTransientSnapshot>,
}

/// 🧬 Immutable app-owned state captured by the scheduler for one retained job lifetime.
pub struct ArtifactOwnedToolJobContext<A: ArtifactApp> {
    presence_local: Option<store::os_store::snapshot_read::SnapshotRead<A::Presence>>,
    presence_peers: Option<std::sync::Arc<store::os_store::presence::PresencePeersRoot<A::Presence>>>,
    presence_peer_factory: Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<A::Presence>>>,
    pub app_instance_id: u32,
    pub view_state: Option<ViewModel>,
    pub canonical_base_revision: [u8; 32],
    pub draft_generation: u64,
    pub transient_generation: u64,
    pub children: std::sync::Arc<ChildContentView>,
    pub draft: ArtifactContextRoot<A::Draft>,
    pub transient: ArtifactContextRoot<A::Transient>,
    pub window_config: Option<WindowConfigSnapshot>,
    pub window_transient: Option<WindowTransientSnapshot>,
    source_capture_started: bool,
    draft_source_fault: Option<String>,
    transient_source_fault: Option<String>,
    refused_source_reads: [Option<store::os_store::snapshot_read::ErasedSnapshotRead>; 2],
    identity_digest: u64,
    tool_run: Option<ToolRunView>,
    provisional: Vec<DslValue>,
    provisional_generation: u64,
    gesture: GestureCapture<A::Mutation>,
}

pub(crate) fn artifact_owned_tool_job_context_identity_digest(
    app_instance_id: u32,
    view_state: Option<&ViewModel>,
    canonical_base_revision: [u8; 32],
    draft_generation: u64,
    transient_generation: u64,
    children_digest: u64,
    windows: (Option<&WindowConfigSnapshot>, Option<&WindowTransientSnapshot>),
) -> u64 {
    let (window_config, window_transient) = windows;
    fn extend(mut digest: u64, bytes: impl IntoIterator<Item = u8>) -> u64 {
        for byte in bytes {
            digest = (digest ^ u64::from(byte)).wrapping_mul(0x1000_0000_01b3);
        }
        digest
    }

    let mut digest = extend(0xcbf2_9ce4_8422_2325, b"ARC-CONTEXT-4".iter().copied());
    digest = extend(digest, app_instance_id.to_le_bytes());
    digest = extend(digest, view_state.map(semio_framework_pack_json::to_json_string).unwrap_or_default().bytes());
    digest = extend(digest, canonical_base_revision);
    digest = extend(digest, draft_generation.to_le_bytes());
    digest = extend(digest, transient_generation.to_le_bytes());
    digest = extend(digest, children_digest.to_le_bytes());
    digest = extend(digest, [u8::from(window_config.is_some())]);
    if let Some(window) = window_config {
        for value in [window.window_id(), window.window_kind_id()] {
            digest = extend(digest, (value.len() as u64).to_le_bytes());
            digest = extend(digest, value.bytes());
        }
        digest = extend(digest, window.generation().to_le_bytes());
        digest = extend(digest, window.revision());
    }
    digest = extend(digest, [u8::from(window_transient.is_some())]);
    if let Some(window) = window_transient {
        for value in [window.window_id(), window.window_kind_id()] {
            digest = extend(digest, (value.len() as u64).to_le_bytes());
            digest = extend(digest, value.bytes());
        }
        digest = extend(digest, window.generation().to_le_bytes());
        digest = extend(digest, window.document_generation().to_le_bytes());
    }
    digest
}

impl<A: ArtifactApp> ArtifactOwnedToolJobContext<A> {
    pub fn new(app_instance_id: u32, view_state: Option<ViewModel>, canonical_base_revision: [u8; 32], draft_generation: u64, transient_generation: u64, snapshots: ArtifactOwnedToolJobSnapshots<A>) -> Self {
        let ArtifactOwnedToolJobSnapshots { children, draft, transient, window_config, window_transient } = snapshots;
        let identity_digest =
            artifact_owned_tool_job_context_identity_digest(app_instance_id, view_state.as_ref(), canonical_base_revision, draft_generation, transient_generation, children.identity_digest(), (window_config.as_ref(), window_transient.as_ref()));
        Self {
            app_instance_id,
            view_state,
            canonical_base_revision,
            draft_generation,
            transient_generation,
            children,
            draft: ArtifactContextRoot::new(draft),
            transient: ArtifactContextRoot::new(transient),
            window_config,
            window_transient,
            source_capture_started: false,
            draft_source_fault: None,
            transient_source_fault: None,
            refused_source_reads: [None, None],
            identity_digest,
            tool_run: None,
            presence_local: None,
            presence_peers: None,
            presence_peer_factory: None,
            provisional: Vec::new(),
            provisional_generation: 0,
            gesture: GestureCapture::detached(),
        }
    }

    /// 🧳️ Transfers every original captured field into its genuine known or unresolved owner group.
    pub(crate) fn into_retirement_parts(self)->crate::owned_context::ContextRetirementParts<A>{
        let Self{presence_local,presence_peers,presence_peer_factory,app_instance_id:_,view_state,canonical_base_revision:_,draft_generation:_,transient_generation:_,children,draft,transient,window_config,window_transient,source_capture_started:_,draft_source_fault,transient_source_fault,refused_source_reads,identity_digest:_,tool_run,provisional,provisional_generation:_,gesture}=self;
        let (tool_run,tool_run_sources)=tool_run.map(|view|{let ToolRunView{tool_id,identity,state,provisional_entities,progress,payload}=view;(crate::owned_context::ContextToolRunMetadata{tool_id,identity,state,progress},crate::owned_context::ContextToolRunSources{entities:provisional_entities,payload})}).unzip();
        crate::owned_context::ContextRetirementParts{metadata:crate::owned_context::ContextOwnedFields{view_state,tool_run,draft,transient,presence:presence_local.map(store::os_store::snapshot_read::SnapshotRead::into_erased),window_config,window_transient,provisional,gesture,refused_reads:refused_source_reads,transient_source_fault,draft_source_fault},residual:crate::owned_context::ContextOriginalResidual{children,peers:presence_peers,peer_factory:presence_peer_factory,tool_run:tool_run_sources}}
    }
    /// 🔗️ Captures the exact draft and transient source issuers while preserving every refusal.
    pub fn with_source_issuers(mut self,draft:&store::DraftStore<A::Draft,A::DraftMutation>,transient:&store::TransientStore<A::Transient,A::TransientMutation>)->Self {
        if self.source_capture_started{return self;}
        self.source_capture_started=true;
        match draft.snapshot_read(){Ok(read)=>{if let Err(read)=self.draft.bind(read){self.refused_source_reads[0]=Some(read.into_erased());}},Err(error)=>self.draft_source_fault=Some(format!("{error:?}"))}
        match transient.current_read(){Ok(read)=>{if let Err(read)=self.transient.bind(read){self.refused_source_reads[1]=Some(read.into_erased());}},Err(error)=>self.transient_source_fault=Some(error)}
        self
    }
    /// 🪪️ Reports only successfully captured original source authority.
    pub fn source_issuers_ready(&self)->bool{self.source_capture_started&&self.draft_source_fault.is_none()&&self.transient_source_fault.is_none()&&self.refused_source_reads.iter().all(Option::is_none)&&self.draft.has_original_issuer()&&self.transient.has_original_issuer()}

    pub fn identity_digest(&self) -> u64 {
        self.identity_digest
    }

    /// 👥️ Retains the exact O(1) presence read authority for an app-owned worker stage.
    pub fn with_presence(mut self, local: store::os_store::snapshot_read::SnapshotRead<A::Presence>, peers: std::sync::Arc<store::os_store::presence::PresencePeersRoot<A::Presence>>, peer_factory: Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<A::Presence>>>) -> Self {
        self.presence_local = Some(local);
        self.presence_peers = Some(peers);
        self.presence_peer_factory = peer_factory;
        self
    }

    /// 👥️ Borrows immutable captured presence without projecting or granting publication authority.
    pub fn presence_view(&self) -> Option<PresenceView<'_, A::Presence>> {
        Some(PresenceView { local: self.presence_local.as_ref()?.get(), peers: PresencePeersView { root: self.presence_peers.as_deref()? } })
    }

    /// ⏯️ Binds the instance's tool run as of command admission (identity, state, provisional entities).
    pub fn with_tool_run(mut self, tool_run: Option<ToolRunView>) -> Self {
        self.tool_run = tool_run;
        self
    }

    /// ⏯️ The instance's tool run as of command admission, the identity a command targets a run action at —
    /// never a renderer-echoed cursor. Ephemeral local state, so it is not part of the identity digest.
    pub fn tool_run(&self) -> Option<&ToolRunView> {
        self.tool_run.as_ref()
    }

    /// 🪞️ Binds the instance's open continuous-tool transactions as of command admission: every open press's and
    /// run's provisional leaves in value form (overlay fold order) and the overlay generation. Ephemeral local state,
    /// folded into the identity digest so a job captured on another overlay is never mistaken for this one.
    pub fn with_provisional(mut self, provisional: Vec<DslValue>, generation: u64) -> Self {
        self.identity_digest = (self.identity_digest ^ generation).wrapping_mul(0x1000_0000_01b3);
        self.provisional = provisional;
        self.provisional_generation = generation;
        self
    }

    /// 🪞️ The provisional leaves a derived view (a preview a retained job evaluates) folds onto the committed
    /// snapshot it was handed, so it follows an open scrub or typing run instead of freezing until the release;
    /// empty while nothing is open. Commands decide over the committed document and never read it.
    pub fn provisional(&self) -> &[DslValue] {
        &self.provisional
    }

    /// 🔢️ The overlay generation as of admission: it moves on every tick, release and zero-trace abort.
    pub fn provisional_generation(&self) -> u64 {
        self.provisional_generation
    }

    /// 🎰️ Binds the dispatching window's gesture slot as of command admission (design §22.10). Ephemeral local tool
    /// state, so it is not part of the identity digest.
    pub fn with_gesture(mut self, gesture: GestureCapture<A::Mutation>) -> Self {
        self.gesture = gesture;
        self
    }

    /// 🖐️ The dispatching window's gesture slot: a streamed tool drives its window's ONE gesture through it
    /// (`GestureSlot::drive`) instead of persisting a gesture of its own; the runtime keeps what the dispatch decided
    /// when it publishes and ends the gesture on host facts.
    pub fn gesture(&self) -> &GestureSlot<A::Mutation> {
        &self.gesture
    }
}

/// 🎒️ Fully owned input handed to an app-defined factory builder after exact raw admission.
pub struct ArtifactOwnedToolJobRequest<A: ArtifactApp> {
    pub retained:RetainedCloneGrant,
    pub command: Box<A::Command>,
    pub raw_wire: ArtifactToolRawInput,
    pub operation: semio_framework_job::Operation,
    pub controller_id: String,
    pub tool_id: String,
    pub payload_schema_id: String,
    pub contract: semio_framework::ToolExecutionContract,
    pub decoded_items: usize,
    pub app_instance_id: u32,
    pub parent_document_id: String,
    pub canonical_base_revision: [u8; 32],
    pub authoring_seed: String,
    pub snapshot: std::sync::Arc<A::Snapshot>,
    /// 🔐️ Exact immutable store authority retained by producers until their private owners close.
    pub snapshot_read: store::os_store::snapshot_read::SnapshotRead<A::Snapshot>,
    pub config: std::sync::Arc<A::Config>,
    pub window_config: Option<WindowConfigSnapshot>,
    pub history: std::sync::Arc<HistoryView>,
    pub interaction_state: std::sync::Arc<protocol::InteractionState>,
    pub interaction_hover: std::sync::Arc<InteractionHoverState>,
    pub context: ArtifactOwnedContextHandle<A>,
    pub instance_operation_owner: ArtifactInstanceOperationOwnerHandle,
    pub output_chunks: ArtifactOutputChunks,
    pub completion: ArtifactToolCompletion<A>,
}

/// 🧭️ Owned input for an app-defined implementation of a framework-reserved route. The runtime
/// selects the exact owner/factory/tool/schema proof before constructing this value.
pub enum ArtifactReservedToolInput {
    Action { args: Option<DslValue>, interaction: protocol::InteractionState, hover: InteractionHoverState },
    Media { port: String, media: Media },
}

/// 🎒️ Immutable app state plus an owned reserved-route input. Implementations only construct a
/// persistent job here; producer work belongs in that job's bounded `step` calls.
pub struct ArtifactReservedToolJobRequest<A: ArtifactApp> {
    pub retained:RetainedCloneGrant,
    pub controller_id: String,
    pub tool_id: String,
    pub payload_schema_id: String,
    pub contract: semio_framework::ToolExecutionContract,
    pub app_instance_id: u32,
    /// 🌐️ Caller-selected ephemeral presentation axes, when a host context is present.
    pub ui_axes: Option<(Locale, Terminology)>,
    pub parent_document_id: String,
    pub canonical_base_revision: [u8; 32],
    pub authoring_seed: String,
    pub snapshot: std::sync::Arc<A::Snapshot>,
    /// 🧾️ Exact immutable store authority for retained reserved-route source traversal.
    pub snapshot_read: store::os_store::snapshot_read::SnapshotRead<A::Snapshot>,
    pub config: std::sync::Arc<A::Config>,
    pub history: std::sync::Arc<HistoryView>,
    /// 🧩️ The parent's composed children, the same view `ArtifactView` hands the synchronous
    /// `A::import_media`. A composing app's LIVE scene is its children, not its own snapshot
    /// (sequence's steps live on the `content` Flow child), and the mounted route builds the emit
    /// exclusively from the resumable job — so without this a composed app could not write a
    /// resumable importer at all and every delivery died `interactive-job.missing-reserved-builder`
    /// (ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP, media §8). Cheap to clone
    /// (`Option<Arc<ChildContentRoot>>`); a job releases it as soon as it has decoded.
    pub children: ChildContentView,
    pub raw_wire: Vec<u8>,
    pub input: ArtifactReservedToolInput,
    pub completion: ArtifactToolCompletion<A>,
}

/// 📬️ Completed document mutations paired with their ephemeral output.
type ArtifactToolEmission<A> = (ArtifactMutationOutcome<<A as ArtifactApp>::Mutation, <A as ArtifactApp>::ConfigMutation, <A as ArtifactApp>::DraftMutation>, EphemeralEmit<A>);

include!("♻️alias/🦀️.rs");

/// 🧹️ Required disposal side of an app-owned persistent job. `Complete` is an assertion
/// that dropping the concrete job immediately afterward is constant-time; no blanket/default
/// implementation exists because an erased job may own arbitrarily deep snapshots or queues.
pub trait ArtifactReservedJob: semio_framework_job::InteractiveJob {}

/// 📦️ Sized ActionBus payload around one app-owned concrete resumable state machine.
pub(crate) struct ArtifactReservedToolJobState {
    inner:ManuallyDrop<Option<Box<dyn ArtifactReservedJob+Send>>>,
    operation:Option<semio_framework_job::Operation>,
}

#[derive(Clone)]
pub struct ArtifactReservedToolJob {
    state:ManuallyDrop<Option<std::sync::Arc<semio_framework_value::retirement::controlled::RetainedOwnerGate<ArtifactReservedToolJobState>>>>,
}

impl ArtifactReservedToolJob {
    pub fn new<J:ArtifactReservedJob+Send+'static>(job:J)->Self {
        Self{state:ManuallyDrop::new(Some(std::sync::Arc::new(semio_framework_value::retirement::controlled::RetainedOwnerGate::new(ArtifactReservedToolJobState{inner:ManuallyDrop::new(Some(Box::new(job))),operation:None}))))}
    }

    pub fn bind_operation(&mut self,operation:semio_framework_job::Operation)->Result<(),semio_framework::ToolJobFactoryError>{
        let source=self.state.as_ref().ok_or_else(||semio_framework::ToolJobFactoryError::new("reserved job state already returned"))?;
        let mut state=source.try_lock().map_err(|_|semio_framework::ToolJobFactoryError::new("reserved job operation authority is busy"))?;
        if state.operation.is_some(){return Err(semio_framework::ToolJobFactoryError::new("reserved job operation authority was already bound"))}
        state.operation=Some(operation);Ok(())
    }

    fn close_demand(&self,kind:u8,copy:usize)->Result<usize,ValueError>{
        let Some(source)=self.state.as_ref()else{return Ok(0)};
        let state=source.try_lock().map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"reserved producer demand authority is borrowed"))?;
        let Some(inner)=state.inner.as_ref()else{return Ok(if state.operation.is_some(){match kind{0=>size_of::<semio_framework_job::Operation>(),3=>1,_=>0}}else{match kind{0=>size_of::<Self>(),2=>Self::original_shared_frame_bytes(),3=>1,_=>0}})};
        if semio_framework_job::InteractiveJob::terminal_is_empty(&**inner){return Ok(match kind{2=>size_of_val(&**inner),3=>1,_=>0})}
        match kind{0=>inner.next_close_copy_byte_demand(),1=>inner.next_close_capacity_byte_demand(copy),2=>inner.next_close_release_byte_demand(),_=>inner.next_close_depth_demand()}
    }

    pub(crate) fn terminal_is_empty(&self)->bool{self.state.is_none()}

    pub(crate) fn begin_close(&mut self){if let Some(source)=self.state.as_ref(){if let Ok(mut state)=source.try_lock(){if let Some(inner)=state.inner.as_mut(){inner.begin_close()}}}}
}

impl semio_framework_job::InteractiveJob for ArtifactReservedToolJob {
    fn step<'a>(&'a mut self,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{
        let Some(source)=self.state.as_ref()else{return JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into)};
        match source.try_lock(){
            Ok(mut state)=>match state.inner.as_mut(){Some(inner)=>{let inner:*mut(dyn ArtifactReservedJob+Send)=&mut**inner;drop(state);unsafe{(&mut*inner).step(cx)}},None=>JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into)},
            Err(_)=>JobOutcomeBorrow::admit_yield(cx).map_err(Into::into),
        }
    }

    fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{
        let Some(source)=self.state.as_ref()else{return match descriptor.kind(){JobOutcomeKind::Cancelled=>descriptor.cancelled(),_=>Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"reserved producer state returned before outcome handback"))}};
        let state=source.try_lock().map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"reserved producer outcome authority is borrowed"))?;
        match state.inner.as_ref(){Some(inner)=>{let inner:*const(dyn ArtifactReservedJob+Send)=&**inner;drop(state);unsafe{(&*inner).borrow_outcome(descriptor)}},None=>match descriptor.kind(){JobOutcomeKind::Cancelled=>descriptor.cancelled(),_=>Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"reserved producer released before its outcome borrow"))}}
    }

    fn begin_close(&mut self){ArtifactReservedToolJob::begin_close(self)}

    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_job::InteractiveJobCloseStep as Step;
        let Some(source)=self.state.as_ref()else{return Ok(Step::Complete{progress:Default::default()})};
        if cx.should_yield(){return Ok(Step::Pending{progress:Default::default()})}let grant=cx.retained_grant();
        let mut state=match source.try_lock(){Ok(state)=>state,Err(_)=>return Ok(Step::Blocked)};
        if let Some(inner)=state.inner.as_mut(){
            if semio_framework_job::InteractiveJob::terminal_is_empty(&**inner){let demand=RetirementDemand{release_bytes:size_of_val(&**inner),depth:1,..Default::default()};return semio_framework_job::close_original_retained_step(cx,demand,|_|{drop(state.inner.take());Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}))})}
            let step=semio_framework_job::InteractiveJob::close_step(&mut**inner,cx)?.admit(grant,semio_framework_job::InteractiveJob::terminal_is_empty(&**inner))?;return Ok(match step{Step::Complete{progress}=>Step::Pending{progress},step=>step})
        }
        if state.operation.is_some(){let demand=RetirementDemand{copy_bytes:size_of::<semio_framework_job::Operation>(),depth:1,..Default::default()};return semio_framework_job::close_original_retained_step(cx,demand,|_|{state.operation.take();Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))})}
        drop(state);
        let demand=RetirementDemand{copy_bytes:size_of::<Self>(),release_bytes:Self::original_shared_frame_bytes(),depth:1,..Default::default()};
        semio_framework_job::close_original_retained_step(cx,demand,|_|{let original=std::sync::Arc::into_inner(self.state.take().unwrap());let released_bytes=if let Some(original)=original{let original=original.into_inner();assert!(original.inner.is_none()&&original.operation.is_none(),"original shared producer frame still retains a body");drop(original);demand.release_bytes}else{0};Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes,..Default::default()}))})
    }

    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{self.close_demand(0,0)}
    fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.close_demand(1,copy)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{self.close_demand(2,0)}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{self.close_demand(3,0)}
    fn terminal_is_empty(&self)->bool{ArtifactReservedToolJob::terminal_is_empty(self)}
}

impl Drop for ArtifactReservedToolJob {
    fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original reserved producer requires its paid shared frame return")}
}

include!("🎟️birth/🦀️.rs");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToolOwnerWitness {
    owner_type_id: std::any::TypeId,
    pub(crate) owner_type_name: &'static str,
}

impl ToolOwnerWitness {
    pub(crate) fn of<A: ArtifactApp>() -> Self {
        Self { owner_type_id: std::any::TypeId::of::<A>(), owner_type_name: std::any::type_name::<A>() }
    }

    #[cfg(test)]
    fn test_with_name(owner_type_name: &'static str) -> Self {
        Self { owner_type_id: std::any::TypeId::of::<()>(), owner_type_name }
    }
}

/// 🚦️ The semantic outcome whose original source bytes the job keeps stable until it is delivered.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ReservedOutcomeKind {
    Preview,
    Checkpoint { applied_progress: u64 },
    Fault,
    Commit,
}

/// 🧳️ Enclosing job owns this inline storage; each turn funds one original publication frontier.
pub(crate) struct ReservedJobOutcomes {
    publication: RetainedJobPublication,
    commit: RetainedPayloadBuilder,
    commit_cursor: usize,
    delivered: bool,
}

/// 🧾️ Admits one received step receipt into the caller's wallet, refused or not.
pub(crate) fn receive_step(cx: &mut StepContext<'_>, step: Result<RetainedCloneStep, ValueError>) -> Result<RetainedCloneStep, ValueError> {
    match step {
        Ok(step) => {
            cx.consume_retained(step.progress())?;
            Ok(step)
        }
        Err(error) => {
            cx.consume_retained(error.retained_progress())?;
            Err(error)
        }
    }
}

/// 🔁️ Maps one paid close receipt onto the job close protocol; completion is the job's own terminal witness.
pub(crate) fn close_result(step:Result<RetainedCloneStep,ValueError>)->Result<InteractiveJobCloseStep,ValueError>{
    step.map(|step|InteractiveJobCloseStep::Pending{progress:step.progress()})
}

impl ReservedJobOutcomes {
    pub(crate) fn new() -> Self {
        Self { publication: RetainedJobPublication::new(), commit: RetainedPayloadBuilder::new(JobPayloadStream::CommitOutput), commit_cursor: 0, delivered: false }
    }

    /// 📬️ Whether a lent outcome still owns its payload pages and must be retired before the next one.
    pub(crate) fn is_delivered(&self) -> bool {
        self.delivered
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.publication.terminal_is_empty() && self.commit.terminal_is_empty()
    }

    /// ♻️ Retires the previous publication one paid turn at a time inside the wallet of a running step.
    pub(crate) fn retire(&mut self, cx: &mut StepContext<'_>) -> Result<(), ValueError> {
        if !self.publication.terminal_is_empty() {
            let step = self.publication.close_step(cx.retained_grant());
            receive_step(cx, step)?;
        }
        if self.publication.terminal_is_empty() {
            self.delivered = false;
        }
        Ok(())
    }

    /// ✍️ Advances one paid frontier of the pending outcome and lends it once its original pages are sealed.
    pub(crate) fn advance<'a>(&'a mut self, kind: ReservedOutcomeKind, source: &[u8], cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        let publication_kind = match kind {
            ReservedOutcomeKind::Preview => JobPublicationKind::Preview,
            ReservedOutcomeKind::Checkpoint { applied_progress } => JobPublicationKind::Checkpoint { applied_progress },
            ReservedOutcomeKind::Fault => JobPublicationKind::Fault,
            ReservedOutcomeKind::Commit => {
                if !self.commit.is_initialized() {
                    self.commit.advance_initialization(cx)?;
                    return Ok(None);
                }
                if self.commit_cursor < source.len() {
                    self.commit.append_original(cx, source, &mut self.commit_cursor)?;
                    return Ok(None);
                }
                if self.commit.published().is_none() {
                    self.commit.seal(cx)?;
                    return Ok(None);
                }
                let original = JobOutcomeBorrow::admit_complete(cx, None, self.commit.published())?;
                self.delivered |= original.is_some();
                return Ok(original);
            }
        };
        let original = self.publication.advance_from_source(publication_kind, source, cx)?;
        self.delivered |= original.is_some();
        Ok(original)
    }

    /// 🤝️ Resolves a descriptor against the same original pages this job lent.
    pub(crate) fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Complete => descriptor.complete(None, self.commit.published()),
            JobOutcomeKind::PreviewReady | JobOutcomeKind::CheckpointReady { .. } | JobOutcomeKind::Fault => self.publication.borrow_outcome(descriptor),
        }
    }

    /// 📏️ Quotes the next original page, builder or metadata turn without granting it.
    pub(crate) fn close_demands(&self) -> Result<RetirementDemand, ValueError> {
        if !self.publication.terminal_is_empty() {
            return self.publication.retirement_demands();
        }
        if !self.commit.terminal_is_empty() {
            return self.commit.retirement_demands();
        }
        Ok(RetirementDemand::default())
    }

    /// ♻️ Closes the publication before the commit pages, never both in one turn.
    pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.publication.terminal_is_empty() {
            return self.publication.close_step(grant);
        }
        if !self.commit.terminal_is_empty() {
            let step = self.commit.close_step_granted(grant)?;
            if self.commit.terminal_is_empty() {
                self.commit_cursor = 0;
                self.delivered = false;
            }
            return Ok(step);
        }
        self.delivered = false;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()))
    }
}

impl Default for ReservedJobOutcomes {
    fn default() -> Self {
        Self::new()
    }
}

/// 🧯️ One bounded fault text kept inline so its publication source never moves or allocates.
pub(crate) struct BoundedFaultText {
    bytes: [u8; Self::MAXIMUM_BYTES],
    length: usize,
}

impl BoundedFaultText {
    const MAXIMUM_BYTES: usize = 480;

    pub(crate) fn new() -> Self {
        Self { bytes: [0; Self::MAXIMUM_BYTES], length: 0 }
    }

    /// ✍️ Stores the longest whole-character prefix of `text` that fits.
    pub(crate) fn set(&mut self, text: &str) {
        let mut length = text.len().min(Self::MAXIMUM_BYTES);
        while !text.is_char_boundary(length) {
            length -= 1;
        }
        self.bytes[..length].copy_from_slice(&text.as_bytes()[..length]);
        self.length = length;
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.length]
    }
}

impl Default for BoundedFaultText {
    fn default() -> Self {
        Self::new()
    }
}

/// 🔎️ Recovers the exact typed reducer fault while leaving framework-authored plain fault pages distinct.
pub fn reducer_fault_of_detail(detail: &[u8]) -> Option<Fault> {
    semio_framework_diagnostic::try_decode_fault_bytes(detail)
}

#[cfg(test)]
#[path="🧪️inference-close/🦀️.rs"]
mod original_inference_close_tests;

#[cfg(test)]
#[path="🧪️inference-admission/🦀️.rs"]
mod original_inference_admission_tests;
