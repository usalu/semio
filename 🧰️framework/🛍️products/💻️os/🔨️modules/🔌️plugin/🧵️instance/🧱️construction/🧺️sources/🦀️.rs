//! 🧺️ Four native Stores and the registration roots assemble into the actual mounted instance once.
use super::*;

/// 🧷️ The private native Store receipt binds this entire constructor input to its actual original caller.
pub struct OriginalArtifactInstanceSources<A:OriginalArtifactAppBirth,M:ReferenceSpaceMember+MemberFactory+Send+'static>{pub app:Option<A>,pub lanes:Option<OriginalArtifactStores<A>>,pub live:Option<OriginalInstanceLiveStores<A>>,pub services:OriginalInstanceServiceSources<A,M>}
/// 🌱️ A paid final allocation retains its partial initialization cursor and every original source through cancellation.
pub struct OriginalArtifactInstanceAdmission<A:OriginalArtifactAppBirth,M:ReferenceSpaceMember+MemberFactory+Send+'static>{pub(crate) sources:ManuallyDrop<Option<OriginalArtifactInstanceSources<A,M>>>,pub(crate) frame:Option<Box<std::mem::MaybeUninit<VcsArtifactApp<A,M>>>>,pub(crate) phase:u8,pub(crate) component_field:usize,pub(crate) component_byte:usize,pub(crate) candidate:ManuallyDrop<Option<Box<VcsArtifactApp<A,M>>>>,pub(crate) binding:OriginalSourceBinding,pub(crate) closing:bool,pub(crate) ready:bool}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn construction(d:RetirementDemand)->OriginalConstructionDemand{OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth}}
impl<A:OriginalArtifactAppBirth,M:ReferenceSpaceMember+MemberFactory+Send+'static> OriginalArtifactInstanceAdmission<A,M>{
 /// 📏️ Captures only this already-original source graph and its separately quoted retained constructor frame.
 pub fn birth_demands()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Self>()+2*size_of::<OriginalArtifactInstanceSources<A,M>>(),capacity_bytes:size_of::<Self>()+size_of::<VcsArtifactApp<A,M>>(),depth:1,..Default::default()}}
 /// 🎟️ The constructor cannot accept native Stores minted by another operation or decoder source.
 pub fn admit(sources:&mut Option<OriginalArtifactInstanceSources<A,M>>,cx:&mut StepContext<'_>,output:&mut Option<Box<Self>>)->Result<bool,OriginalTurnFailure>{if output.is_some()||sources.as_ref().is_none_or(|source|source.lanes.as_ref().is_none_or(|lanes|!lanes.matches(cx))){return Err(invalid("native instance admission requires its original four-lane source receipt and empty destination").into())}let d=Self::birth_demands();let binding=OriginalSourceBinding::borrow(cx);admit_original_construction(cx,output,construction(d),|_|{let result=(||Ok((Box::new(Self{sources:ManuallyDrop::new(sources.take()),frame:Some(Box::<VcsArtifactApp<A,M>>::new_uninit()),phase:0,component_field:0,component_byte:0,candidate:ManuallyDrop::new(None),binding,closing:false,ready:false}),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()})))();semio_framework_value::RetainedWorkOutcome{result,work:1}})?;Ok(output.is_some())}
 /// 📏️ Each final-address component quotes its own initialized metadata and independently funded backing.
 pub const INITIALIZATION_CURSOR_BYTES:usize=2*size_of::<usize>()+size_of::<u8>();
 fn assembly_demands(&self,maximum_copy_bytes:usize,backing:bool)->Result<RetirementDemand,ValueError>{fn field<T,U>(_:fn(&T)->&U)->usize{size_of::<U>()}let body=maximum_copy_bytes.saturating_sub(Self::INITIALIZATION_CURSOR_BYTES);let mut demand=match self.phase{
 0=>RetirementDemand{copy_bytes:2*field::<VcsArtifactApp<A,M>,_>(|owner|&owner.app),depth:1,..Default::default()},
 1=>RetirementDemand{copy_bytes:2*field::<VcsArtifactApp<A,M>,_>(|owner|&owner.mounted_policy),depth:1,..Default::default()},
 2=>RetirementDemand{copy_bytes:2*field::<VcsArtifactApp<A,M>,_>(|owner|&owner.original_controller_close),depth:1,..Default::default()},
 3=>RetirementDemand{copy_bytes:2*field::<VcsArtifactApp<A,M>,_>(|owner|&owner.constructor),depth:1,..Default::default()},
 4=>RetirementDemand{copy_bytes:2*field::<VcsArtifactApp<A,M>,_>(|owner|&owner.stores),depth:1,..Default::default()},
 5=>PeerRoster::<A>::birth_demands()?,
 6=>RetirementDemand{copy_bytes:2*field::<VcsArtifactApp<A,M>,_>(|owner|&owner.ingress),depth:1,..Default::default()},
 7=>RetirementDemand{copy_bytes:2*field::<VcsArtifactApp<A,M>,_>(|owner|&owner.original_typing),depth:1,..Default::default()},
 8=>RetirementDemand{copy_bytes:2*field::<VcsArtifactApp<A,M>,_>(|owner|&owner.original_emission),depth:1,..Default::default()},
 9=>RetirementDemand{copy_bytes:2*field::<VcsArtifactApp<A,M>,_>(|owner|&owner.original_typed_command)+2*field::<VcsArtifactApp<A,M>,_>(|owner|&owner.original_host_event_command)+2*field::<VcsArtifactApp<A,M>,_>(|owner|&owner.original_command_scope)+2*field::<VcsArtifactApp<A,M>,_>(|owner|&owner.original_host_event_scope),depth:1,..Default::default()},
 10=>RetirementDemand{copy_bytes:2*field::<VcsArtifactApp<A,M>,_>(|owner|&owner.original_typed_tag),depth:1,..Default::default()},
 11=>RetirementDemand{copy_bytes:2*field::<VcsArtifactApp<A,M>,_>(|owner|&owner.original_typed_tag_close),depth:1,..Default::default()},
 12=>RetirementDemand{copy_bytes:2*field::<VcsArtifactApp<A,M>,_>(|owner|&owner.interaction),depth:1,..Default::default()},
 13=>RetirementDemand{copy_bytes:2*field::<VcsArtifactApp<A,M>,_>(|owner|&owner.history),depth:1,..Default::default()},
 14=>RetirementDemand{copy_bytes:2*field::<VcsArtifactApp<A,M>,_>(|owner|&owner.host_events),depth:1,..Default::default()},
 15=>RetirementDemand{copy_bytes:2*field::<VcsArtifactApp<A,M>,_>(|owner|&owner.txn),depth:1,..Default::default()},
 16=>{let mut demand=ChildComposition::<M>::birth_demands()?;demand.copy_bytes=ChildComposition::<M>::initial_copy_bytes();demand},
 17=>PendingReserved::<A>::initial_field_demands(self.component_field,self.component_byte,body,backing)?,
 18=>ToolOperations::<A>::initial_field_demands(self.component_field,self.component_byte,body,backing)?,
 19=>MediaTransfers::<A>::birth_demands()?,
 20=>EnvelopeJobs::<A,M>::birth_demands()?,
 21=>RetirementDemand{copy_bytes:2*field::<VcsArtifactApp<A,M>,_>(|owner|&owner.maintenance),depth:1,..Default::default()},
 22=>RetirementDemand{copy_bytes:2*field::<VcsArtifactApp<A,M>,_>(|owner|&owner.close),depth:1,..Default::default()},
 23=>RetirementDemand{copy_bytes:2*field::<VcsArtifactApp<A,M>,_>(|owner|&owner.tool_runs),depth:1,..Default::default()},
 24=>TimeTravelLedger::<A>::initial_field_demands(self.component_field,self.component_byte,body)?,
 25=>ToolMachineRuntime::<A::Snapshot,A::Mutation,A::Config,A::ConfigMutation>::initial_field_demands(self.component_field,self.component_byte,body)?,
 _=>RetirementDemand{copy_bytes:2*size_of::<Box<VcsArtifactApp<A,M>>>()+size_of::<Option<OriginalArtifactInstanceSources<A,M>>>()+size_of::<Option<Box<std::mem::MaybeUninit<VcsArtifactApp<A,M>>>>>()+size_of::<Option<Box<VcsArtifactApp<A,M>>>>(),depth:1,..Default::default()},
 };let donor=match self.phase{0=>size_of::<Option<A>>(),4=>size_of::<Option<OriginalArtifactStores<A>>>()+size_of::<Option<OriginalInstanceLiveStores<A>>>(),22=>size_of::<Option<semio_framework_plugin_instance_app::CloseCursors<A,M>>>(),_=>0};demand.copy_bytes+=Self::INITIALIZATION_CURSOR_BYTES+donor;if !backing{demand.capacity_bytes=0;}Ok(demand)}
 /// 🖋️ Partial metadata remains private; cancellation finishes absent slots without admitting unused service backing.
 fn initialize_component(&mut self,count:usize,backing:bool)->Result<(),ValueError>{
  if self.phase==26{let frame=self.frame.take().ok_or_else(||invalid("native final allocation absent"))?;let complete=unsafe{frame.assume_init()};let VcsArtifactApp{app:_,mounted_policy:_,original_controller_close:_,constructor:_,stores:_,roster:_,ingress:_,original_typing:_,original_emission:_,original_typed_command:_,original_host_event_command:_,original_command_scope:_,original_host_event_scope:_,original_typed_tag:_,original_typed_tag_close:_,interaction:_,history:_,host_events:_,txn:_,child_composition:_,pending:_,tools:_,media:_,ingest:_,maintenance:_,close:_,tool_runs:_,time_travel:_,tool_machines:_}=complete.as_ref();*self.candidate=Some(complete);self.sources.take();self.component_field=0;self.component_byte=0;self.phase+=1;return Ok(())}
  let target=self.frame.as_mut().ok_or_else(||invalid("native final allocation absent"))?.as_mut_ptr();let source=self.sources.as_mut().ok_or_else(||invalid("native final sources absent"))?;
  let staged_fields=match self.phase{17=>Some(PendingReserved::<A>::INITIAL_FIELDS),18=>Some(ToolOperations::<A>::INITIAL_FIELDS),24=>Some(TimeTravelLedger::<A>::INITIAL_FIELDS),25=>Some(ToolMachineRuntime::<A::Snapshot,A::Mutation,A::Config,A::ConfigMutation>::INITIAL_FIELDS),_=>None};
  let field_complete=if let Some(_)=staged_fields{unsafe{match self.phase{
   17=>PendingReserved::<A>::initialize_field_at(std::ptr::addr_of_mut!((*target).pending),&mut source.services.dispatch_report,self.component_field,self.component_byte,count,backing)?,
   18=>ToolOperations::<A>::initialize_field_at(std::ptr::addr_of_mut!((*target).tools),&mut source.services.tools,self.component_field,self.component_byte,count,backing)?,
   24=>TimeTravelLedger::<A>::initialize_field_at(std::ptr::addr_of_mut!((*target).time_travel),self.component_field,self.component_byte,count)?,
   25=>ToolMachineRuntime::<A::Snapshot,A::Mutation,A::Config,A::ConfigMutation>::initialize_field_at(std::ptr::addr_of_mut!((*target).tool_machines),self.component_field,self.component_byte,count)?,
   _=>return Err(invalid("native staged component inventory changed")),
  }}}else{unsafe{match self.phase{
  0=>{std::ptr::addr_of_mut!((*target).app).write(source.app.take().ok_or_else(||invalid("native controller absent"))?);},
  1=>{std::ptr::addr_of_mut!((*target).mounted_policy).write(source.services.policy);},
  2=>{std::ptr::addr_of_mut!((*target).original_controller_close).write(Some(OriginalControllerClose::<A>::new()));},
  3=>{std::ptr::addr_of_mut!((*target).constructor).write(ConstructorState::new());},
  4=>{let lanes=source.lanes.take().ok_or_else(||invalid("native lanes absent"))?;let live=source.live.take().ok_or_else(||invalid("native live stores absent"))?;let(document,config,draft,interaction)=lanes.into_parts();std::ptr::addr_of_mut!((*target).stores).write(InstanceStores::from_original(document,config,draft,live.presence,live.transient,live.window_config,live.window_transient,live.artifact_factory,live.config_factory,live.draft_factory,live.presence_factory,live.transient_factory,live.presence_retirement,live.transient_retirement,live.unsupported,interaction,live.retired_window_transient));},
  5=>{std::ptr::addr_of_mut!((*target).roster).write(PeerRoster::from_original_with_backing(source.services.peer_presence.take().ok_or_else(||invalid("native peer root absent"))?,source.services.peer_scope.take().ok_or_else(||invalid("native peer scope absent"))?,backing));},
  6=>{std::ptr::addr_of_mut!((*target).ingress).write(ActionIngress::new());},
  7=>{std::ptr::addr_of_mut!((*target).original_typing).write(Default::default());},
  8=>{std::ptr::addr_of_mut!((*target).original_emission).write(Default::default());},
  9=>{std::ptr::addr_of_mut!((*target).original_typed_command).write(Default::default());std::ptr::addr_of_mut!((*target).original_host_event_command).write(Default::default());std::ptr::addr_of_mut!((*target).original_command_scope).write(None);std::ptr::addr_of_mut!((*target).original_host_event_scope).write(None);},
  10=>{std::ptr::addr_of_mut!((*target).original_typed_tag).write(ManuallyDrop::new(None));},
  11=>{std::ptr::addr_of_mut!((*target).original_typed_tag_close).write(Default::default());},
  12=>{std::ptr::addr_of_mut!((*target).interaction).write(LocalInteraction::new());},
  13=>{std::ptr::addr_of_mut!((*target).history).write(CommandHistory::new());},
  14=>{std::ptr::addr_of_mut!((*target).host_events).write(HostEvents::new());},
  15=>{std::ptr::addr_of_mut!((*target).txn).write(Transactions::new());},
  16=>{ChildComposition::<M>::initialize_original_at(std::ptr::addr_of_mut!((*target).child_composition),if backing{Default::default()}else{store::os_store::composition_coordinator::CompositionCoordinator::unallocated()},backing);},
  19=>{std::ptr::addr_of_mut!((*target).media).write(MediaTransfers::with_backing(backing));},
  20=>{std::ptr::addr_of_mut!((*target).ingest).write(EnvelopeJobs::from_original_with_backing(source.services.field_decoders.take().ok_or_else(||invalid("native field decoder absent"))?,source.services.completed_records.take().ok_or_else(||invalid("native completed-record issuer absent"))?,backing));},
  21=>{std::ptr::addr_of_mut!((*target).maintenance).write(MaintenanceCursors::new());},
  22=>{std::ptr::addr_of_mut!((*target).close).write(source.services.close.take().ok_or_else(||invalid("native close ports absent"))?);},
  23=>{std::ptr::addr_of_mut!((*target).tool_runs).write(ToolRunLedger::default());},
  _=>return Err(invalid("native component cursor exceeds the final field inventory")),
  }}true};
  if let Some(fields)=staged_fields{if field_complete{self.component_field+=1;self.component_byte=0;}else{self.component_byte=self.component_byte.checked_add(count).ok_or_else(||invalid("native component byte cursor overflow"))?;}if self.component_field<fields{return Ok(())}}
  self.component_field=0;self.component_byte=0;self.phase+=1;Ok(())
 }
 fn assemble(&mut self,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
  let closing=self.closing||cx.is_cancelled();if closing{self.closing=true;}let d=self.assembly_demands(cx.retained_grant().maximum_copy_bytes,!closing)?;
  let mut run=||{self.initialize_component(d.copy_bytes-Self::INITIALIZATION_CURSOR_BYTES,!closing)?;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()})};
  if closing{close_original_retained_step(cx,d,|_|semio_framework_value::RetainedWorkOutcome{result:run().map(RetainedCloneStep::Progress),work:1})?;}else{let mut done=None;admit_original_construction(cx,&mut done,construction(d),|_|semio_framework_value::RetainedWorkOutcome{result:run().map(|progress|((),progress)),work:1})?;}Ok(())
 }
 /// ▶️ No instance becomes visible before its actual native constructor recipient has admitted all four installed catalogs.
 pub fn step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if !self.binding.matches(cx){return Err(invalid("native instance constructor changed its original caller").into())}if self.closing||cx.is_cancelled(){self.closing=true;return Ok(false)}if self.ready{return Ok(true)}if self.candidate.is_none(){self.assemble(cx)?;return Ok(false)}let candidate=self.candidate.as_mut().ok_or_else(||invalid("native opening sources require their retained candidate"))?;if !candidate.stores.admit_original_constructor_recipient(&mut candidate.constructor,cx)?{return Ok(false)}let d=RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()};let mut done=None;admit_original_construction(cx,&mut done,construction(d),|_|{let result=(||{self.ready=true;Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})();semio_framework_value::RetainedWorkOutcome{result,work:1}})?;Ok(self.ready)}
 /// 📤️ Publishes the unchanged candidate allocation after its original constructor is complete.
 pub fn take(&mut self,cx:&mut StepContext<'_>,output:&mut Option<Box<VcsArtifactApp<A,M>>>)->Result<bool,OriginalTurnFailure>{if !self.binding.matches(cx)||!self.ready||self.closing||output.is_some(){return Err(invalid("native instance handoff requires its unchanged ready candidate").into())}let d=RetirementDemand{copy_bytes:2*size_of::<Box<VcsArtifactApp<A,M>>>()+size_of::<bool>(),depth:1,..Default::default()};admit_original_construction(cx,output,construction(d),|_|{let result=(||{self.closing=true;Ok((self.candidate.take().ok_or_else(||invalid("native opening sources require their retained candidate"))?,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})();semio_framework_value::RetainedWorkOutcome{result,work:1}})?;Ok(output.is_some())}
 /// ♻️ Cancellation assembles only private cleanup custody and drives the actual mounted close ladder under the same caller.
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if !self.binding.matches(cx){return Err(invalid("native instance closure changed its original caller").into())}self.closing=true;if self.sources.is_some(){self.assemble(cx)?;return Ok(false)}if let Some(candidate)=self.candidate.as_mut(){if !semio_framework_plugin_reactor_app::PluginApp::close_terminal_is_empty(candidate.as_ref()){let grant=cx.retained_grant();semio_framework_plugin_reactor_app::PluginApp::close_step(candidate.as_mut(),cx,grant)?;return Ok(false)}let d=RetirementDemand{copy_bytes:size_of::<Option<Box<VcsArtifactApp<A,M>>>>(),release_bytes:size_of::<VcsArtifactApp<A,M>>(),depth:1,..Default::default()};close_original_retained_step(cx,d,|_|{let result=(||{self.candidate.take();Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))})();semio_framework_value::RetainedWorkOutcome{result,work:1}})?;return Ok(false)}Ok(self.terminal_is_empty())}
 /// 🕳️ Every native source was handed off or closed through the original mounted instance owner.
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.sources.is_none()&&self.frame.is_none()&&self.candidate.is_none()}
}
impl<A:OriginalArtifactAppBirth,M:ReferenceSpaceMember+MemberFactory+Send+'static> Drop for OriginalArtifactInstanceAdmission<A,M>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original native instance admission must retain every source until handoff or paid closure");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.sources);ManuallyDrop::drop(&mut self.candidate);}}}}
