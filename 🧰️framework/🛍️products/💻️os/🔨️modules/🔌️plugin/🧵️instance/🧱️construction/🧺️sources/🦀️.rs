//! 🧺️ Four native Stores and the registration roots assemble into the actual mounted instance once.
use super::*;
#[path="📦️stores/🦀️.rs"]
mod store_fields;
pub(crate) use store_fields::OriginalInstanceStoreFields;
use semio_plugin_instance_app::{OriginalTypingDispatch, OriginalEmissionSource};

/// 🧷️ The private native Store receipt binds this entire constructor input to its actual original caller.
pub struct OriginalArtifactInstanceSources<A:OriginalArtifactAppBirth,M:ReferenceSpaceMember+MemberFactory+Send+'static>{pub app:Option<A>,pub(crate) stores:OriginalInstanceStoreFields<A>,pub services:OriginalInstanceServiceSources<A,M>}
/// 🌱️ A paid final allocation retains its partial initialization cursor and every original source through cancellation.
pub struct OriginalArtifactInstanceAdmission<A:OriginalArtifactAppBirth,M:ReferenceSpaceMember+MemberFactory+Send+'static>{pub(crate) sources:ManuallyDrop<Option<OriginalArtifactInstanceSources<A,M>>>,pub(crate) frame:Option<Box<std::mem::MaybeUninit<VcsArtifactApp<A,M>>>>,pub(crate) phase:u8,pub(crate) component_field:usize,pub(crate) component_byte:usize,pub(crate) candidate:ManuallyDrop<Option<Box<VcsArtifactApp<A,M>>>>,pub(crate) binding:OriginalSourceBinding,pub(crate) closing:bool,pub(crate) ready:bool,pub(crate) birth:semio_plugin_ownership::original_store_genesis::OriginalStoreBirth,pub(crate) units:[Option<()>;3],pub(crate) handed:bool}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn construction(d:RetirementDemand)->OriginalConstructionDemand{OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth}}
impl<A:OriginalArtifactAppBirth,M:ReferenceSpaceMember+MemberFactory+Send+'static> OriginalArtifactInstanceAdmission<A,M>{
 /// 📏️ Captures only this already-original source graph and its separately quoted retained constructor frame.
 pub fn birth_demands()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Self>()+2*size_of::<OriginalArtifactInstanceSources<A,M>>(),capacity_bytes:size_of::<Self>()+size_of::<VcsArtifactApp<A,M>>(),depth:1,..Default::default()}}
 /// 📏️ Each final-address component quotes its own initialized metadata and independently funded backing.
 pub const INITIALIZATION_CURSOR_BYTES:usize=2*size_of::<usize>()+size_of::<u8>();
 /// 📏️ Quotes one selected field of the private mounted assembly, including its cursor.
 pub fn initialization_demand_at(phase:u8,component_field:usize,component_byte:usize,maximum_copy_bytes:usize,backing:bool)->Result<RetirementDemand,ValueError>{fn field<T,U>(_:fn(&T)->&U)->usize{size_of::<U>()}let body=maximum_copy_bytes.saturating_sub(Self::INITIALIZATION_CURSOR_BYTES);let mut demand=match phase{
 0=>RetirementDemand{copy_bytes:2*field::<VcsArtifactApp<A,M>,_>(|owner|&owner.app),depth:1,..Default::default()},
 1=>RetirementDemand{copy_bytes:2*field::<VcsArtifactApp<A,M>,_>(|owner|&owner.mounted_policy),depth:1,..Default::default()},
 2=>RetirementDemand{copy_bytes:2*field::<VcsArtifactApp<A,M>,_>(|owner|&owner.original_controller_close),depth:1,..Default::default()},
 3=>ConstructorState::<A>::initial_field_demands(component_field)?,
 4=>OriginalInstanceStoreFields::<A>::demand(component_field,component_byte,body)?,
 5=>PeerRoster::<A>::birth_demands()?,
 6=>RetirementDemand{copy_bytes:2*field::<VcsArtifactApp<A,M>,_>(|owner|&owner.ingress),depth:1,..Default::default()},
 7=>OriginalTypingDispatch::<A>::initial_field_demands(component_field)?,
 8=>OriginalEmissionSource::<A>::initial_field_demands(component_field)?,
 9=>RetirementDemand{copy_bytes:2*match component_field{0=>field::<VcsArtifactApp<A,M>,_>(|owner|&owner.original_typed_command),1=>field::<VcsArtifactApp<A,M>,_>(|owner|&owner.original_host_event_command),2=>field::<VcsArtifactApp<A,M>,_>(|owner|&owner.original_command_scope),3=>field::<VcsArtifactApp<A,M>,_>(|owner|&owner.original_host_event_scope),_=>return Err(invalid("native command field escaped its inventory"))},depth:1,..Default::default()},
 10=>RetirementDemand{copy_bytes:2*field::<VcsArtifactApp<A,M>,_>(|owner|&owner.original_typed_tag),depth:1,..Default::default()},
 11=>RetirementDemand{copy_bytes:2*field::<VcsArtifactApp<A,M>,_>(|owner|&owner.original_typed_tag_close),depth:1,..Default::default()},
 12=>RetirementDemand{copy_bytes:2*field::<VcsArtifactApp<A,M>,_>(|owner|&owner.interaction),depth:1,..Default::default()},
 13=>RetirementDemand{copy_bytes:2*field::<VcsArtifactApp<A,M>,_>(|owner|&owner.history),depth:1,..Default::default()},
 14=>HostEvents::initial_field_demands(component_field)?,
 15=>RetirementDemand{copy_bytes:2*field::<VcsArtifactApp<A,M>,_>(|owner|&owner.txn),depth:1,..Default::default()},
 16=>{let mut demand=ChildComposition::<M>::birth_demands()?;demand.copy_bytes=ChildComposition::<M>::initial_copy_bytes();demand},
 17=>PendingReserved::<A>::initial_field_demands(component_field,component_byte,body,backing)?,
 18=>ToolOperations::<A>::initial_field_demands(component_field,component_byte,body,backing)?,
 19=>MediaTransfers::<A>::initial_field_demands(component_field,component_byte,body,backing)?,
 20=>EnvelopeJobs::<A,M>::birth_demands()?,
 21=>RetirementDemand{copy_bytes:2*field::<VcsArtifactApp<A,M>,_>(|owner|&owner.maintenance),depth:1,..Default::default()},
 22=>RetirementDemand{copy_bytes:2*field::<VcsArtifactApp<A,M>,_>(|owner|&owner.close),depth:1,..Default::default()},
 23=>RetirementDemand{copy_bytes:2*field::<VcsArtifactApp<A,M>,_>(|owner|&owner.tool_runs),depth:1,..Default::default()},
 24=>TimeTravelLedger::<A>::initial_field_demands(component_field,component_byte,body)?,
 25=>ToolMachineRuntime::<A::Snapshot,A::Mutation,A::Config,A::ConfigMutation>::initial_field_demands(component_field,component_byte,body)?,
 _=>RetirementDemand{copy_bytes:2*size_of::<Box<VcsArtifactApp<A,M>>>()+size_of::<Option<OriginalArtifactInstanceSources<A,M>>>()+size_of::<Option<Box<std::mem::MaybeUninit<VcsArtifactApp<A,M>>>>>()+size_of::<Option<Box<VcsArtifactApp<A,M>>>>(),depth:1,..Default::default()},
 };let donor=match phase{0=>size_of::<Option<A>>(),22=>size_of::<Option<semio_plugin_instance_app::CloseCursors<A,M>>>(),_=>0};demand.copy_bytes+=Self::INITIALIZATION_CURSOR_BYTES+donor;if !backing{demand.capacity_bytes=0;}Ok(demand)}
 /// 📋️ Counts independently funded fields at a mounted assembly phase.
 pub fn initialization_fields_at(phase:u8)->usize{
     Self::staged_initial_fields(phase).unwrap_or(1)
 }
 fn staged_initial_fields(phase:u8)->Option<usize>{match phase{3=>Some(ConstructorState::<A>::INITIAL_FIELDS),4=>Some(OriginalInstanceStoreFields::<A>::FIELDS),7=>Some(OriginalTypingDispatch::<A>::INITIAL_FIELDS),8=>Some(OriginalEmissionSource::<A>::INITIAL_FIELDS),9=>Some(4),14=>Some(HostEvents::INITIAL_FIELDS),17=>Some(PendingReserved::<A>::INITIAL_FIELDS),18=>Some(ToolOperations::<A>::INITIAL_FIELDS),19=>Some(MediaTransfers::<A>::INITIAL_FIELDS),24=>Some(TimeTravelLedger::<A>::INITIAL_FIELDS),25=>Some(ToolMachineRuntime::<A::Snapshot,A::Mutation,A::Config,A::ConfigMutation>::INITIAL_FIELDS),_=>None}}
 /// 🖋️ Partial metadata remains private; cancellation finishes absent slots without admitting unused service backing.
 fn initialize_component(&mut self,count:usize,backing:bool)->Result<(),ValueError>{
  if self.phase==26{let frame=self.frame.take().ok_or_else(||invalid("native final allocation absent"))?;let complete=unsafe{frame.assume_init()};let VcsArtifactApp{app:_,mounted_policy:_,original_controller_close:_,constructor:_,stores:_,roster:_,ingress:_,original_typing:_,original_emission:_,original_typed_command:_,original_host_event_command:_,original_command_scope:_,original_host_event_scope:_,original_typed_tag:_,original_typed_tag_close:_,interaction:_,history:_,host_events:_,txn:_,child_composition:_,pending:_,tools:_,media:_,ingest:_,maintenance:_,close:_,tool_runs:_,time_travel:_,tool_machines:_}=complete.as_ref();*self.candidate=Some(complete);self.sources.take();self.component_field=0;self.component_byte=0;self.phase+=1;return Ok(())}
  let target=self.frame.as_mut().ok_or_else(||invalid("native final allocation absent"))?.as_mut_ptr();let source=self.sources.as_mut().ok_or_else(||invalid("native final sources absent"))?;
  let staged_fields=Self::staged_initial_fields(self.phase);
  let field_complete=if let Some(_)=staged_fields{unsafe{match self.phase{
   3=>ConstructorState::<A>::initialize_field_at(std::ptr::addr_of_mut!((*target).constructor),self.component_field,self.component_byte,count)?,
   4=>source.stores.place(std::ptr::addr_of_mut!((*target).stores),self.component_field,self.component_byte,count)?,
   7=>OriginalTypingDispatch::<A>::initialize_field_at(std::ptr::addr_of_mut!((*target).original_typing),self.component_field,self.component_byte,count)?,
   8=>OriginalEmissionSource::<A>::initialize_field_at(std::ptr::addr_of_mut!((*target).original_emission),self.component_field,self.component_byte,count)?,
   9=>{match self.component_field{0=>std::ptr::addr_of_mut!((*target).original_typed_command).write(Default::default()),1=>std::ptr::addr_of_mut!((*target).original_host_event_command).write(Default::default()),2=>std::ptr::addr_of_mut!((*target).original_command_scope).write(Default::default()),3=>std::ptr::addr_of_mut!((*target).original_host_event_scope).write(None),_=>return Err(invalid("native command field escaped its inventory"))}true},
   14=>HostEvents::initialize_field_at(std::ptr::addr_of_mut!((*target).host_events),self.component_field,self.component_byte,count)?,
   17=>PendingReserved::<A>::initialize_field_at(std::ptr::addr_of_mut!((*target).pending),&mut source.services.dispatch_report,self.component_field,self.component_byte,count,backing)?,
   18=>ToolOperations::<A>::initialize_field_at(std::ptr::addr_of_mut!((*target).tools),&mut source.services.tools,self.component_field,self.component_byte,count,backing)?,
   19=>MediaTransfers::<A>::initialize_field_at(std::ptr::addr_of_mut!((*target).media),self.component_field,self.component_byte,count,backing)?,
   24=>TimeTravelLedger::<A>::initialize_field_at(std::ptr::addr_of_mut!((*target).time_travel),self.component_field,self.component_byte,count)?,
   25=>ToolMachineRuntime::<A::Snapshot,A::Mutation,A::Config,A::ConfigMutation>::initialize_field_at(std::ptr::addr_of_mut!((*target).tool_machines),self.component_field,self.component_byte,count)?,
   _=>return Err(invalid("native staged component inventory changed")),
  }}}else{unsafe{match self.phase{
  0=>{std::ptr::addr_of_mut!((*target).app).write(source.app.take().ok_or_else(||invalid("native controller absent"))?);},
  1=>{std::ptr::addr_of_mut!((*target).mounted_policy).write(source.services.policy);},
  2=>{std::ptr::addr_of_mut!((*target).original_controller_close).write(Some(OriginalControllerClose::<A>::new()));},
  5=>{std::ptr::addr_of_mut!((*target).roster).write(PeerRoster::from_original_with_backing(source.services.peer_presence.take().ok_or_else(||invalid("native peer root absent"))?,source.services.peer_scope.take().ok_or_else(||invalid("native peer scope absent"))?,backing));},
  6=>{std::ptr::addr_of_mut!((*target).ingress).write(ActionIngress::new());},
  10=>{std::ptr::addr_of_mut!((*target).original_typed_tag).write(ManuallyDrop::new(None));},
  11=>{std::ptr::addr_of_mut!((*target).original_typed_tag_close).write(Default::default());},
  12=>{std::ptr::addr_of_mut!((*target).interaction).write(LocalInteraction::new());},
  13=>{std::ptr::addr_of_mut!((*target).history).write(CommandHistory::new());},
  15=>{std::ptr::addr_of_mut!((*target).txn).write(Transactions::new());},
  16=>{ChildComposition::<M>::initialize_original_at(std::ptr::addr_of_mut!((*target).child_composition),if backing{Default::default()}else{store::os_store::composition_coordinator::CompositionCoordinator::unallocated()},backing);},
  20=>{std::ptr::addr_of_mut!((*target).ingest).write(EnvelopeJobs::from_original_with_backing(source.services.field_decoders.take().ok_or_else(||invalid("native field decoder absent"))?,source.services.completed_records.take().ok_or_else(||invalid("native completed-record issuer absent"))?,backing));},
  21=>{std::ptr::addr_of_mut!((*target).maintenance).write(MaintenanceCursors::new());},
  22=>{std::ptr::addr_of_mut!((*target).close).write(source.services.close.take().ok_or_else(||invalid("native close ports absent"))?);},
  23=>{std::ptr::addr_of_mut!((*target).tool_runs).write(ToolRunLedger::default());},
  _=>return Err(invalid("native component cursor exceeds the final field inventory")),
  }}true};
  if let Some(fields)=staged_fields{if field_complete{self.component_field+=1;self.component_byte=0;}else{self.component_byte=self.component_byte.checked_add(count).ok_or_else(||invalid("native component byte cursor overflow"))?;}if self.component_field<fields{return Ok(())}}
  self.component_field=0;self.component_byte=0;self.phase+=1;Ok(())
 }
 /// 📐️ Bounds the opening field after reserving the original constructor and receiving scope.
 fn opening_copy_allowance(available_copy_bytes:usize)->Result<usize,ValueError>{
  let (reserve,_)=semio_framework_job::original_construction_settled_quote::<()>(construction(RetirementDemand::default()),1)?;
  Ok(available_copy_bytes.saturating_sub(reserve.copy_bytes))
 }
 fn assemble(&mut self,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
  let closing=self.closing||cx.is_cancelled();if closing{self.closing=true;}
  let remaining=cx.retained_grant().maximum_copy_bytes;
  let available=if closing{remaining}else{Self::opening_copy_allowance(remaining)?};
  let d=Self::initialization_demand_at(self.phase,self.component_field,self.component_byte,available,!closing)?;
  let progress=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()};let cursor=d.copy_bytes-Self::INITIALIZATION_CURSOR_BYTES;
  if closing{close_original_retained_step(cx,d,|_|semio_framework_value::RetainedWorkOutcome{result:self.initialize_component(cursor,false).map(|_|RetainedCloneStep::Progress(progress)),work:1})?;}else{let mut birth=std::mem::take(&mut self.birth);let mut unit=self.units[0].take();let ready=birth.built(cx,&mut unit,construction(d),||self.initialize_component(cursor,true).map(|_|((),progress)));self.birth=birth;let ready=ready?;self.units[0]=if ready{None}else{unit};}Ok(())
 }
 /// ▶️ No instance becomes visible before its actual native constructor recipient has admitted all four installed catalogs.
 pub fn step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if !self.binding.matches(cx){return Err(invalid("native instance constructor changed its original caller").into())}if self.closing||cx.is_cancelled(){self.closing=true;return Ok(false)}if self.ready{return Ok(true)}if self.candidate.is_none(){self.assemble(cx)?;return Ok(false)}let candidate=self.candidate.as_mut().ok_or_else(||invalid("native opening sources require their retained candidate"))?;if !candidate.stores.admit_original_constructor_recipient(&mut candidate.constructor,cx)?{return Ok(false)}let d=RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()};let ready=self.birth.built(cx,&mut self.units[1],construction(d),||{self.ready=true;Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})?;if ready{self.units[1]=None;}Ok(self.ready)}
 /// 📤️ Publishes the unchanged candidate allocation after its original constructor is complete.
 pub fn take(&mut self,cx:&mut StepContext<'_>,output:&mut Option<Box<VcsArtifactApp<A,M>>>)->Result<bool,OriginalTurnFailure>{if !self.binding.matches(cx)||!self.ready||self.closing&&!self.handed||output.is_some()&&!self.handed{return Err(invalid("native instance handoff requires its unchanged ready candidate").into())}let d=RetirementDemand{copy_bytes:2*size_of::<Box<VcsArtifactApp<A,M>>>()+size_of::<bool>(),depth:1,..Default::default()};let ready=self.birth.built(cx,output,construction(d),||{self.closing=true;self.handed=true;Ok((self.candidate.take().ok_or_else(||invalid("native opening sources require their retained candidate"))?,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})?;if ready{self.handed=false;}Ok(ready&&output.is_some())}
 /// ♻️ Cancellation assembles only private cleanup custody and drives the actual mounted close ladder under the same caller.
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if !self.binding.matches(cx){return Err(invalid("native instance closure changed its original caller").into())}self.closing=true;if self.sources.is_some(){self.assemble(cx)?;return Ok(false)}if let Some(candidate)=self.candidate.as_mut(){if !semio_plugin_reactor_app::PluginApp::close_terminal_is_empty(candidate.as_ref()){let grant=cx.retained_grant();semio_plugin_reactor_app::PluginApp::close_step(candidate.as_mut(),cx,grant)?;return Ok(false)}let d=RetirementDemand{copy_bytes:size_of::<Option<Box<VcsArtifactApp<A,M>>>>(),release_bytes:size_of::<VcsArtifactApp<A,M>>(),depth:1,..Default::default()};semio_plugin_instance_initialization::services::synchronous(cx,d,||{self.candidate.take();Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))})?;return Ok(false)}if !self.birth.terminal_is_empty(){self.units=[None;3];self.birth.close_step(cx)?;return Ok(false)}Ok(self.terminal_is_empty())}
 /// 🕳️ Every native source was handed off or closed through the original mounted instance owner.
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.sources.is_none()&&self.frame.is_none()&&self.candidate.is_none()&&self.birth.terminal_is_empty()}
}
impl<A:OriginalArtifactAppBirth,M:ReferenceSpaceMember+MemberFactory+Send+'static> Drop for OriginalArtifactInstanceAdmission<A,M>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original native instance admission must retain every source until handoff or paid closure");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.sources);ManuallyDrop::drop(&mut self.candidate);}}}}
