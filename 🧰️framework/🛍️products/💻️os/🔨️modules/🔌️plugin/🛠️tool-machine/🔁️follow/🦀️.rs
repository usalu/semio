//! 🔁️ The mounted continuous-tool follower consumes genuine immutable ledger rows and publishes one complete preview epoch.
use super::*;
use semio_plugin_decl_app::ArtifactApp;
use semio_framework_job::{StepContext,OriginalTurnFailure,OriginalSourceBinding};
use semio_framework_value::{RetainedOwnedProjection,ReplyStash,RetainedAllocationEffect,SharedUtf8,ValueRefusalKind,RetirementDemand,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll};
use semio_framework_value::retained_clone::{RetainedCloneClose,RetainedCloneRef,RetainedOwnedProjectionAdmission,NativeAllocationIdentity};
use semio_plugin_job_receiving::original_steps::{begun,replied,retired,retired_poll};
use semio_framework_value::retained_clone::original_slot::RetainedOriginalSlotKey;
use semio_framework_tool_machine::{OriginalToolLedgerSource,ScrubState,GestureState,TypingState};
use semio_plugin_window_config::WindowConfigOwnerRegistry;
use semio_plugin_window_config_original::OriginalWindowPreviewEpoch;
use semio_plugin_tool_run_operations::{OriginalToolOverlayFold,OriginalToolLedgerWalk,paid,paid_effect,receive,close_paid};
use semio_framework_value_native_storage::{NativeOwnedStorage, NativeSeedTemplate};
use std::mem::{MaybeUninit,size_of};

macro_rules! follow_seed_fields {
    ($($field:ident: $value:expr),+ $(,)?) => {
        fn seed_template() -> NativeSeedTemplate<Self> {
            let _ = |Self { $($field: _,)+ }: Self| {};
            unsafe {
                NativeSeedTemplate::fields(
                    [$(stringify!($field)),+].len(),
                    |target, part| Self::seed_part::<false>(target, part),
                    |target, part| Self::seed_part::<true>(target, part).is_some(),
                )
            }
        }

        unsafe fn seed_part<const WRITE: bool>(target: *mut MaybeUninit<Self>, part: usize) -> Option<usize> {
            let fields: &[unsafe fn(*mut Self) -> usize] = &[$(
                |target| {
                    let field = unsafe { &raw mut (*target).$field };
                    if WRITE {
                        unsafe { field.write($value) };
                    }
                    fn field_size<T>(_: *mut T) -> usize { size_of::<T>() }
                    field_size(field)
                },
            )+];
            Some(unsafe { fields.get(part)?(target.cast()) })
        }
    };
}

type Machines<A>=ToolMachineRuntime<<A as ArtifactApp>::Snapshot,<A as ArtifactApp>::Mutation,<A as ArtifactApp>::Config,<A as ArtifactApp>::ConfigMutation>;
type Document<A>=store::ArtifactStore<<A as ArtifactApp>::Snapshot,<A as ArtifactApp>::Mutation>;
type Config<A>=store::ArtifactStore<<A as ArtifactApp>::Config,<A as ArtifactApp>::ConfigMutation>;
type Press<A>=ScrubState<PressLeaf<<A as ArtifactApp>::Mutation,<A as ArtifactApp>::ConfigMutation>>;
/// 🪞️ One resident projection admission with its recorded admit and take replies; an unfunded or pending turn keeps its state for the next.
struct Projection<T:?Sized+Sync>{admission:RetainedOwnedProjectionAdmission<T>,admit:ReplyStash<ValueError>,take:ReplyStash<ValueError>}
impl<T:?Sized+Sync> Default for Projection<T>{fn default()->Self{Self{admission:Default::default(),admit:Default::default(),take:Default::default()}}}
impl<T:?Sized+Sync+'static> Projection<T>{
 fn demand(&self,source:RetainedCloneRef<'_,T>)->Result<RetirementDemand,ValueError>{if self.admission.terminal_is_empty(){self.admission.demand(source)}else{let mut d=self.admission.take_demand()?;d.copy_bytes+=size_of::<u8>()+size_of::<bool>();Ok(d)}}
 fn step(&mut self,source:RetainedCloneRef<'_,T>,output:&mut Option<RetainedOwnedProjection<T>>,effect:&mut RetainedAllocationEffect<'_>)->Result<(RetainedCloneProgress,bool),ValueError>{
  let Self{admission,admit,take}=self;
  if admission.terminal_is_empty(){return Ok(match admit.run(|reply,recover|(if recover{admission.recover_admit(reply)}else{admission.admit(source,effect,reply)}).map(drop))?{Some(answer)=>(answer?.progress(),false),None=>(Default::default(),false)})}
  Ok(match take.run(|reply,recover|(if recover{admission.recover_take(reply)}else{admission.take(output,effect,reply)}).map(drop))?{Some(answer)=>{let mut progress=answer?.progress();progress.copied_bytes+=size_of::<u8>()+size_of::<bool>();(progress,true)},None=>(Default::default(),false)})
 }
 fn close_demand(&self)->Result<RetirementDemand,ValueError>{self.admission.close_demand(4096)}
 fn terminal_is_empty(&self)->bool{self.admission.terminal_is_empty()}
}
/// 🧳️ The original source fence owns every selected input, private alias and pending conflict through physical close.
pub struct OriginalToolMachineFollow<A:ArtifactApp>{binding:Option<(OriginalSourceBinding,(NativeAllocationIdentity,u64))>,request:u64,revisions:[u64;3],generations:[u64;2],contents:[[u8;32];2],phase:u8,lane:u8,index:usize,ordinal:u32,document_seen:bool,config_seen:bool,typing_started:bool,published:bool,closing:bool,seed_only:bool,
 document:OriginalToolOverlayFold<A::Snapshot,A::Mutation>,config:OriginalToolOverlayFold<A::Config,A::ConfigMutation>,presses:OriginalToolLedgerWalk<Press<A>>,gestures:OriginalToolLedgerWalk<GestureState<A::Mutation>>,typing:OriginalToolLedgerWalk<TypingState<A::Mutation>>,windows:Option<OriginalWindowPreviewEpoch>,document_input:Option<RetainedOwnedProjection<A::Mutation>>,config_input:Option<RetainedOwnedProjection<A::ConfigMutation>>,window_input:Option<RetainedOwnedProjection<WindowConfigMutation>>,document_admission:RetainedOwnedProjectionAdmission<A::Mutation>,config_admission:RetainedOwnedProjectionAdmission<A::ConfigMutation>,window_admission:RetainedOwnedProjectionAdmission<WindowConfigMutation>,document_candidate:Option<Arc<A::Snapshot>>,config_candidate:Option<Arc<A::Config>>,document_displaced:Option<Arc<A::Snapshot>>,config_displaced:Option<Arc<A::Config>>,rejected_window:Option<SharedUtf8>,rejected_key:Option<RetainedOriginalSlotKey>,rejected_state:Option<TypingState<A::Mutation>>,rejected_close:Option<ControlledRetirement<TypingState<A::Mutation>>>,address_close:RetainedCloneClose}
fn header<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn p(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,message)}
impl<A:ArtifactApp> OriginalToolMachineFollow<A>{
 /// 🧭️ Borrows scalar progress without materializing ledger rows or semantic state.
 pub fn progress(&self)->(u8,u8,usize,u32,bool,bool){(self.phase,self.lane,self.index,self.ordinal,self.published,self.closing)}

    follow_seed_fields! {
        binding: None,
        request: 0,
        revisions: [0; 3],
        generations: [0; 2],
        contents: [[0; 32]; 2],
        phase: 0,
        lane: 0,
        index: 0,
        ordinal: 0,
        document_seen: false,
        config_seen: false,
        typing_started: false,
        published: false,
        closing: true,
        seed_only: true,
        document: OriginalToolOverlayFold::vacant(),
        config: OriginalToolOverlayFold::vacant(),
        presses: OriginalToolLedgerWalk::vacant(),
        gestures: OriginalToolLedgerWalk::vacant(),
        typing: OriginalToolLedgerWalk::vacant(),
        windows: None,
        document_input: None,
        config_input: None,
        window_input: None,
        document_admission: RetainedOwnedProjectionAdmission::vacant(),
        config_admission: RetainedOwnedProjectionAdmission::vacant(),
        window_admission: RetainedOwnedProjectionAdmission::vacant(),
        document_candidate: None,
        config_candidate: None,
        document_displaced: None,
        config_displaced: None,
        rejected_window: None,
        rejected_key: None,
        rejected_state: None,
        rejected_close: None,
        address_close: RetainedCloneClose::vacant(),
    }

 fn bind(&mut self,binding:(OriginalSourceBinding,(NativeAllocationIdentity,u64)),machines:&Machines<A>,document:&Document<A>,config:&Config<A>){
    self.binding=Some(binding);
    self.request=machines.refold_request;
    self.revisions=[machines.presses.original_revision(),machines.gestures.original_revision(),machines.typing.original_revision()];
    self.generations=[document.generation(),config.generation()];
    self.contents=[document.content_revision(),config.content_revision()];
    self.closing=false;
    self.seed_only=false;
 }

 fn validate(&self,cx:&StepContext<'_>)->Result<(),ValueError>{if !self.binding.is_some_and(|(binding,decoder)|binding.matches(cx)&&cx.native_receiving_identity()==Some(decoder)){return Err(invalid("original tool follow changed its caller or decoder"))}Ok(())}
 fn current(&self,machines:&Machines<A>,document:&Document<A>,config:&Config<A>)->bool{self.request==machines.refold_request&&self.revisions==[machines.presses.original_revision(),machines.gestures.original_revision(),machines.typing.original_revision()]&&self.generations==[document.generation(),config.generation()]&&self.contents==[document.content_revision(),config.content_revision()]}
 fn cursor_demand()->RetirementDemand{header::<(u8,usize,u32,bool)>()}
 fn next_phase(&mut self,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{let d=Self::cursor_demand();paid(cx,d,|_|{self.phase+=1;self.index=0;self.lane=0;Ok(p(d))})?;Ok(())}
 fn complete_leaf(&mut self,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{let next=self.ordinal.checked_add(1).ok_or_else(||invalid("original tool follow input ordinal exhausted"))?;let d=Self::cursor_demand();paid(cx,d,|_|{self.index+=1;self.ordinal=next;self.lane=0;Ok(p(d))})?;Ok(())}
 fn press_window<'a>(&self,machines:&'a Machines<A>)->Result<&'a WindowConfigMutation,ValueError>{let original=self.presses.borrow(&machines.presses)?;original.get().entries.get(self.index).and_then(|(_,leaf)|leaf.window_config()).ok_or_else(||invalid("original Window preview source changed its selected press leaf"))}
 fn leaf_demands(&self,document:&Document<A>,config:&Config<A>,windows:&WindowConfigOwnerRegistry,machines:&Machines<A>,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{match self.lane{1=>if self.document_input.is_none()&&!self.document.input_pending(){Ok(if self.phase==3&&self.document.accepted()==Some(false){header::<(SharedUtf8,RetainedOriginalSlotKey)>()}else{Self::cursor_demand()})}else{self.document.demands(document,4096,maximum_work_units)},2=>if self.config_input.is_none()&&!self.config.input_pending(){Ok(Self::cursor_demand())}else{self.config.demands(config,4096,maximum_work_units)},3=>{let leaf=self.press_window(machines)?;if windows.preview_input_held(leaf.window_kind_id(),leaf.window_id(),self.windows.as_ref().unwrap().epoch()){Ok(header::<u8>())}else{windows.preview_input_demands(leaf.window_kind_id(),leaf.window_id(),self.windows.as_ref().unwrap().epoch(),4096,self.window_input.as_ref().ok_or_else(||invalid("original tool follow window input is absent"))?)}},4=>match self.window_input.as_ref(){Some(input)=>if input.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<WindowConfigMutation>>>())}else{input.close_demand(4096)},None=>Ok(header::<u8>())},5=>{let leaf=self.press_window(machines)?;if windows.preview_leaf_ready(leaf.window_kind_id(),leaf.window_id(),self.windows.as_ref().unwrap().epoch()){Ok(Self::cursor_demand())}else{windows.preview_leaf_demands(leaf.window_kind_id(),leaf.window_id(),self.windows.as_ref().unwrap().epoch(),4096,maximum_work_units)}},_=>Err(invalid("original tool follow selected no active input lane"))}}
 fn advance_leaf(&mut self,document:&Document<A>,config:&Config<A>,windows:&mut WindowConfigOwnerRegistry,machines:&Machines<A>,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{match self.lane{
  1=>{if self.document_input.is_none()&&!self.document.input_pending(){if self.phase==3&&self.document.accepted()==Some(false){let(window,key)=self.typing.resident().unwrap();let d=header::<(SharedUtf8,RetainedOriginalSlotKey)>();paid(cx,d,|_|{self.rejected_window=Some(window.clone());self.rejected_key=Some(key);Ok(p(d))})?;}else{self.complete_leaf(cx)?;}}else{self.document.advance_input(document,&mut self.document_input,self.ordinal,cx)?;}},
  2=>{if self.config_input.is_none()&&!self.config.input_pending(){self.complete_leaf(cx)?;}else{self.config.advance_input(config,&mut self.config_input,self.ordinal,cx)?;}},
  3=>{let leaf=self.press_window(machines)?;let epoch=self.windows.as_ref().unwrap().epoch();if windows.preview_input_held(leaf.window_kind_id(),leaf.window_id(),epoch){let d=header::<u8>();paid(cx,d,|_|{self.lane=4;Ok(p(d))})?;}else{let input=self.window_input.as_ref().unwrap();let d=windows.preview_input_demands(leaf.window_kind_id(),leaf.window_id(),epoch,4096,input)?;paid_effect(cx,d,|effect|Ok(windows.preview_input(input,epoch,effect)?.unwrap_or_default()))?;}},
  4=>{if let Some(input)=self.window_input.as_mut(){let d=if input.terminal_is_empty(){header::<Option<RetainedOwnedProjection<WindowConfigMutation>>>()}else{input.close_demand(4096)?};if input.terminal_is_empty(){close_paid(cx,d,|_|{self.window_input=None;Ok(p(d))})?;}else{retired_poll(cx,d,|effect|input.close_step(4096,effect))?;}}else{let d=header::<u8>();paid(cx,d,|_|{self.lane=5;Ok(p(d))})?;}},
  5=>{let leaf=self.press_window(machines)?;let epoch=self.windows.as_ref().ok_or_else(||invalid("follow windows are absent"))?.epoch();if windows.preview_leaf_ready(leaf.window_kind_id(),leaf.window_id(),epoch){self.complete_leaf(cx)?;}else{let d=windows.preview_leaf_demands(leaf.window_kind_id(),leaf.window_id(),epoch,4096,cx.fuel_remaining())?;receive(cx,d,|native,work,g|{let before=native.actual_work();let result=windows.preview_leaf_step(leaf.window_kind_id(),leaf.window_id(),epoch,4096,g,native,work).map(|(_,p)|RetainedCloneStep::Progress(p));RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work:native.actual_work().saturating_sub(before)})})?;}},_=>return Err(invalid("original tool follow has no input lane").into())}Ok(())}
 fn retain_demand<T:?Sized+Sync>(admission:&RetainedOwnedProjectionAdmission<T>,source:RetainedCloneRef<'_,T>)->Result<RetirementDemand,ValueError>{if admission.terminal_is_empty(){admission.demand(source)}else{let mut d=admission.take_demand()?;d.copy_bytes+=size_of::<u8>()+size_of::<bool>();Ok(d)}}
 fn retain<T:?Sized+Sync>(admission:&mut RetainedOwnedProjectionAdmission<T>,source:RetainedCloneRef<'_,T>,output:&mut Option<RetainedOwnedProjection<T>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let d=Self::retain_demand(admission,source)?;let mut taken=false;paid_effect(cx,d,|effect|{let adopting=admission.terminal_is_empty();let poll=if adopting{replied::<ValueError>(effect,|effect,reply|admission.admit(source,effect,reply).map(drop))}else{replied::<ValueError>(effect,|effect,reply|admission.take(output,effect,reply).map(drop))};let RetainedWorkPoll::Complete(turn)=poll else{return Ok(Default::default())};let step=turn.result?;taken=!adopting&&matches!(step,RetainedCloneStep::Complete(_));let mut progress=step.progress();if taken{progress.copied_bytes+=size_of::<u8>()+size_of::<bool>()}Ok(progress)})?;Ok(taken)}
 fn project_press(&mut self,machines:&Machines<A>,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{let source=self.presses.borrow(&machines.presses)?;let index=self.index;let leaf=&source.get().entries[index].1;match leaf{PressLeaf::Member(_)=>{if Self::retain(&mut self.document_admission,source.project(index,move|state|state.entries[index].1.member().unwrap()),&mut self.document_input,cx)?{self.document_seen=true;self.lane=1;}},PressLeaf::Config(_)=>{if Self::retain(&mut self.config_admission,source.project(index,move|state|state.entries[index].1.config().unwrap()),&mut self.config_input,cx)?{self.config_seen=true;self.lane=2;}},PressLeaf::WindowConfig(_)=>{if Self::retain(&mut self.window_admission,source.project(index,move|state|state.entries[index].1.window_config().unwrap()),&mut self.window_input,cx)?{self.lane=3;}},PressLeaf::Child(_)=>self.complete_leaf(cx)?}Ok(())}
 fn document_projection<S:RetireOwned+Send+Sync+'static>(walk:&OriginalToolLedgerWalk<S>,ledger:&impl OriginalToolLedgerSource<State=S>,index:usize,project:impl for<'a>FnOnce(&'a S)->&'a A::Mutation,admission:&mut RetainedOwnedProjectionAdmission<A::Mutation>,output:&mut Option<RetainedOwnedProjection<A::Mutation>>,lane:&mut u8,seen:&mut bool,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{let source=walk.borrow(ledger)?.project(index,project);if Self::retain(admission,source,output,cx)?{*lane=1;*seen=true;}Ok(())}
 fn return_row<S:RetireOwned+Send+Sync+'static>(walk:&mut OriginalToolLedgerWalk<S>,index:&mut usize,typing_started:&mut bool,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{let mut d=OriginalToolLedgerWalk::<S>::return_demands();d.copy_bytes+=size_of::<usize>()+size_of::<bool>();paid(cx,d,|g|{let child=RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-size_of::<usize>()-size_of::<bool>(),..g};let Some(mut receipt)=walk.finish_row_granted(child)?else{return Ok(Default::default())};*index=0;*typing_started=false;receipt.copied_bytes+=size_of::<usize>()+size_of::<bool>();Ok(receipt)})?;Ok(())}
 /// 🪜️ Drives original source selection and installed native semantic preparation, preserving the old visible epoch.
 pub fn advance(&mut self,document:&Document<A>,config:&Config<A>,windows:&mut WindowConfigOwnerRegistry,machines:&mut Machines<A>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if self.closing||cx.is_cancelled()||self.rejected_window.is_some()||!self.current(machines,document,config)||self.published{return self.close_step(document,config,windows,machines,cx)}
  match self.phase{
   0=>{if self.windows.is_none(){OriginalWindowPreviewEpoch::admit(windows,4096,cx,&mut self.windows)?;}else{self.next_phase(cx)?;}},
   1=>{if self.lane!=0{self.advance_leaf(document,config,windows,machines,cx)?;}else if self.presses.done(){self.next_phase(cx)?;}else if !self.presses.ready(){self.presses.advance(&mut machines.presses,cx)?;}else{let count=self.presses.borrow(&machines.presses)?.get().entries.len();if self.index>=count{Self::return_row(&mut self.presses,&mut self.index,&mut self.typing_started,cx)?;}else{self.project_press(machines,cx)?;}}},
   2=>{if self.lane!=0{self.advance_leaf(document,config,windows,machines,cx)?;}else if self.gestures.done(){self.next_phase(cx)?;}else if !self.gestures.ready(){self.gestures.advance(&mut machines.gestures,cx)?;}else{let count=self.gestures.borrow(&machines.gestures)?.get().entries.len();if self.index>=count{Self::return_row(&mut self.gestures,&mut self.index,&mut self.typing_started,cx)?;}else{let index=self.index;Self::document_projection(&self.gestures,&machines.gestures,index,move|state|&state.entries[index].1,&mut self.document_admission,&mut self.document_input,&mut self.lane,&mut self.document_seen,cx)?;}}},
   3=>{if self.lane!=0{self.advance_leaf(document,config,windows,machines,cx)?;}else if self.typing.done(){self.next_phase(cx)?;}else if !self.typing.ready(){self.typing.advance(&mut machines.typing,cx)?;}else if !self.typing_started{if self.document.has_checkpoint(){let d=header::<bool>();paid(cx,d,|_|{self.typing_started=true;Ok(p(d))})?;}else{self.document.checkpoint(document,cx)?;}}else{let count=self.typing.borrow(&machines.typing)?.get().net.len();if self.index>=count{if self.document.has_checkpoint(){self.document.finish_checkpoint(document,false,cx)?;}else{Self::return_row(&mut self.typing,&mut self.index,&mut self.typing_started,cx)?;}}else{let index=self.index;Self::document_projection(&self.typing,&machines.typing,index,move|state|&state.net[index],&mut self.document_admission,&mut self.document_input,&mut self.lane,&mut self.document_seen,cx)?;}}},
   4=>{if !self.document_seen||self.document_candidate.is_some(){self.next_phase(cx)?;}else{self.document.retain_candidate_alias(document,&mut self.document_candidate,cx)?;}},
   5=>{if !self.config_seen||self.config_candidate.is_some(){self.next_phase(cx)?;}else{self.config.retain_candidate_alias(config,&mut self.config_candidate,cx)?;}},
   6=>{if self.windows.as_ref().unwrap().ready(){self.next_phase(cx)?;}else{self.windows.as_mut().unwrap().advance(windows,cx)?;}},
   7=>{self.windows.as_ref().ok_or_else(||invalid("follow windows are absent"))?.validate_context(windows,cx)?;let next_generation=machines.provisional_generation.checked_add(1).ok_or_else(||invalid("original tool visible generation exhausted"))?;let d=self.publication_demands();receive(cx,d,|native,_,grant|{let result=(||->Result<RetainedCloneStep,ValueError>{let epoch=self.windows.as_mut().ok_or_else(||invalid("follow windows are absent"))?;let ep=epoch.publication_demands();let child=RetainedCloneGrant{maximum_copy_bytes:ep.copy_bytes,..grant};epoch.publish_granted(windows,child,native)?.ok_or_else(||invalid("original tool common publication lost its admitted Window epoch"))?;self.document_displaced=std::mem::replace(&mut machines.overlay,self.document_candidate.take());self.config_displaced=std::mem::replace(&mut machines.config_overlay,self.config_candidate.take());machines.overlay_generation=self.generations[0];machines.config_overlay_generation=self.generations[1];machines.provisional_generation=next_generation;machines.refold_owed=false;self.published=true;Ok(RetainedCloneStep::Complete(p(d)))})();RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome{result,work:1})})?;},_=>return Err(invalid("original tool follow phase is absent").into())}Ok(false)
 }
 fn publication_demands(&self)->RetirementDemand{let mut d=header::<(Option<Arc<A::Snapshot>>,Option<Arc<A::Snapshot>>,Option<Arc<A::Config>>,Option<Arc<A::Config>>,u64,u64,u64,bool,bool)>();d.copy_bytes+=self.windows.as_ref().unwrap().publication_demands().copy_bytes;d}
}
impl<A:ArtifactApp> OriginalToolMachineFollow<A>{
 fn return_demand()->RetirementDemand{header::<(bool,usize,bool)>()}
 /// 👆️ Every mounted maintenance turn selects one real native source event.
 pub fn demands(&self,document:&Document<A>,config:&Config<A>,windows:&WindowConfigOwnerRegistry,machines:&Machines<A>,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.closing||self.rejected_window.is_some()||self.published||!self.current(machines,document,config){return self.close_demands(document,config,windows,machines)}match self.phase{
  0=>Ok(if self.windows.is_none(){OriginalWindowPreviewEpoch::admission_demands()}else{Self::cursor_demand()}),
  1=>{if self.lane!=0{return self.leaf_demands(document,config,windows,machines,maximum_work_units)}if self.presses.done(){return Ok(Self::cursor_demand())}if !self.presses.ready(){return self.presses.demands(&machines.presses,4096)}let source=self.presses.borrow(&machines.presses)?;let index=self.index;let Some((_,leaf))=source.get().entries.get(index)else{return Ok(Self::return_demand())};Ok(match leaf{PressLeaf::Member(_)=>Self::retain_demand(&self.document_admission,source.project(index,move|state|state.entries[index].1.member().unwrap()))?,PressLeaf::Config(_)=>Self::retain_demand(&self.config_admission,source.project(index,move|state|state.entries[index].1.config().unwrap()))?,PressLeaf::WindowConfig(_)=>Self::retain_demand(&self.window_admission,source.project(index,move|state|state.entries[index].1.window_config().unwrap()))?,PressLeaf::Child(_)=>Self::cursor_demand()})},
  2=>{if self.lane!=0{return self.leaf_demands(document,config,windows,machines,maximum_work_units)}if self.gestures.done(){return Ok(Self::cursor_demand())}if !self.gestures.ready(){return self.gestures.demands(&machines.gestures,4096)}let source=self.gestures.borrow(&machines.gestures)?;let index=self.index;if index>=source.get().entries.len(){Ok(Self::return_demand())}else{Self::retain_demand(&self.document_admission,source.project(index,move|state|&state.entries[index].1))}},
  3=>{if self.lane!=0{return self.leaf_demands(document,config,windows,machines,maximum_work_units)}if self.typing.done(){return Ok(Self::cursor_demand())}if !self.typing.ready(){return self.typing.demands(&machines.typing,4096)}if !self.typing_started{return if self.document.has_checkpoint(){Ok(header::<bool>())}else{self.document.checkpoint_demands(4096,false)}}if self.index>=self.typing.borrow(&machines.typing)?.get().net.len(){if self.document.has_checkpoint(){self.document.checkpoint_demands(4096,true)}else{Ok(Self::return_demand())}}else{let source=self.typing.borrow(&machines.typing)?;let index=self.index;Self::retain_demand(&self.document_admission,source.project(index,move|state|&state.net[index]))}},
  4=>if !self.document_seen||self.document_candidate.is_some(){Ok(Self::cursor_demand())}else{self.document.candidate_alias_demands(4096)},
  5=>if !self.config_seen||self.config_candidate.is_some(){Ok(Self::cursor_demand())}else{self.config.candidate_alias_demands(4096)},
  6=>if self.windows.as_ref().unwrap().ready(){Ok(Self::cursor_demand())}else{self.windows.as_ref().unwrap().demands(windows)},
  7=>Ok(self.publication_demands()),_=>Err(invalid("original tool follow has no declared source event"))}}
 fn projection_close_demands<T:?Sized+Sync+'static>(input:&RetainedOwnedProjection<T>)->Result<RetirementDemand,ValueError>{if input.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<T>>>())}else{input.close_demand(4096)}}
 fn close_projection<T:?Sized+Sync+'static>(input:&mut Option<RetainedOwnedProjection<T>>,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{let original=input.as_mut().unwrap();let d=Self::projection_close_demands(original)?;if original.terminal_is_empty(){close_paid(cx,d,|_|{*input=None;Ok(p(d))})?;}else{retired_poll(cx,d,|effect|original.close_step(4096,effect))?;}Ok(())}
 /// 🚪️ Real semantic consumers close before their parent ledger reads and original caller loan.
 pub fn close_demands(&self,document:&Document<A>,config:&Config<A>,windows:&WindowConfigOwnerRegistry,machines:&Machines<A>)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Ok(header::<bool>())}
  if !self.document.terminal_is_empty(){return self.document.close_demands(document,4096)}if self.document_candidate.is_some(){return self.document.external_alias_close_demands(&self.document_candidate,4096)}if self.document_displaced.is_some(){return self.document.external_alias_close_demands(&self.document_displaced,4096)}
  if !self.config.terminal_is_empty(){return self.config.close_demands(config,4096)}if self.config_candidate.is_some(){return self.config.external_alias_close_demands(&self.config_candidate,4096)}if self.config_displaced.is_some(){return self.config.external_alias_close_demands(&self.config_displaced,4096)}
  if let Some(epoch)=self.windows.as_ref(){return if epoch.terminal_is_empty(){Ok(header::<Option<OriginalWindowPreviewEpoch>>())}else{epoch.close_demands(windows)}}
  if !self.document_admission.terminal_is_empty(){return self.document_admission.close_demand(4096)}if !self.config_admission.terminal_is_empty(){return self.config_admission.close_demand(4096)}if !self.window_admission.terminal_is_empty(){return self.window_admission.close_demand(4096)}
  if let Some(input)=self.document_input.as_ref(){return Self::projection_close_demands(input)}if let Some(input)=self.config_input.as_ref(){return Self::projection_close_demands(input)}if let Some(input)=self.window_input.as_ref(){return Self::projection_close_demands(input)}
  if !self.presses.terminal_is_empty(){return self.presses.close_demands(&machines.presses,4096)}if !self.gestures.terminal_is_empty(){return self.gestures.close_demands(&machines.gestures,4096)}if !self.typing.terminal_is_empty(){return self.typing.close_demands(&machines.typing,4096)}
  if self.rejected_key.is_some(){let mut d=TypingLedger::<A::Mutation>::original_transfer_demands();d.copy_bytes+=size_of::<u64>()+size_of::<Option<RetainedOriginalSlotKey>>();return Ok(d)}
  if self.rejected_state.is_some(){return Ok(header::<(TypingState<A::Mutation>,ControlledRetirement<TypingState<A::Mutation>>)>())}
  if let Some(owner)=self.rejected_close.as_ref(){return if owner.terminal_is_empty(){Ok(header::<Option<ControlledRetirement<TypingState<A::Mutation>>>>())}else{owner.selected_demand(4096)}}
  if self.rejected_window.is_some()||!self.address_close.is_empty(){return self.address_close.next_owner_demands::<SharedUtf8>(self.rejected_window.is_some(),&None,4096)}
  if self.binding.is_some(){return Ok(header::<Option<(OriginalSourceBinding,(NativeAllocationIdentity,u64))>>())}Ok(Default::default())
 }
 /// ♻️ Cancellation returns every original semantic/source alias before withdrawing a conflicted typing run.
 pub fn close_step(&mut self,document:&Document<A>,config:&Config<A>,windows:&mut WindowConfigOwnerRegistry,machines:&mut Machines<A>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.terminal_is_empty(){return Ok(true)}self.validate(cx)?;let d=self.close_demands(document,config,windows,machines)?;if !self.closing{close_paid(cx,d,|_|{self.closing=true;Ok(p(d))})?;return Ok(false)}
  if !self.document.terminal_is_empty(){self.document.close_step(document,cx)?;return Ok(false)}if self.document_candidate.is_some(){self.document.close_external_alias(document,&mut self.document_candidate,cx)?;return Ok(false)}if self.document_displaced.is_some(){self.document.close_external_alias(document,&mut self.document_displaced,cx)?;return Ok(false)}
  if !self.config.terminal_is_empty(){self.config.close_step(config,cx)?;return Ok(false)}if self.config_candidate.is_some(){self.config.close_external_alias(config,&mut self.config_candidate,cx)?;return Ok(false)}if self.config_displaced.is_some(){self.config.close_external_alias(config,&mut self.config_displaced,cx)?;return Ok(false)}
  if let Some(epoch)=self.windows.as_mut(){if epoch.terminal_is_empty(){close_paid(cx,d,|_|{self.windows=None;Ok(p(d))})?;}else{epoch.close_step(windows,cx)?;}return Ok(false)}
  if !self.document_admission.terminal_is_empty(){retired_poll(cx,d,|effect|self.document_admission.close_step(4096,effect))?;return Ok(false)}if !self.config_admission.terminal_is_empty(){retired_poll(cx,d,|effect|self.config_admission.close_step(4096,effect))?;return Ok(false)}if !self.window_admission.terminal_is_empty(){retired_poll(cx,d,|effect|self.window_admission.close_step(4096,effect))?;return Ok(false)}
  if self.document_input.is_some(){Self::close_projection(&mut self.document_input,cx)?;return Ok(false)}if self.config_input.is_some(){Self::close_projection(&mut self.config_input,cx)?;return Ok(false)}if self.window_input.is_some(){Self::close_projection(&mut self.window_input,cx)?;return Ok(false)}
  if !self.presses.terminal_is_empty(){self.presses.close_step(&mut machines.presses,cx)?;return Ok(false)}if !self.gestures.terminal_is_empty(){self.gestures.close_step(&mut machines.gestures,cx)?;return Ok(false)}if !self.typing.terminal_is_empty(){self.typing.close_step(&mut machines.typing,cx)?;return Ok(false)}
  if let Some(key)=self.rejected_key{let next=machines.refold_request.checked_add(1).ok_or_else(||invalid("original tool follow request exhausted"))?;close_paid(cx,d,|g|{let overhead=size_of::<u64>()+size_of::<Option<RetainedOriginalSlotKey>>();let child=RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-overhead,..g};let Some(mut receipt)=machines.typing.withdraw_original(self.rejected_window.as_ref().unwrap(),key,&mut self.rejected_state,child)?else{return Ok(Default::default())};machines.refold_request=next;self.rejected_key=None;receipt.copied_bytes+=overhead;Ok(receipt)})?;return Ok(false)}
  if self.rejected_state.is_some(){close_paid(cx,d,|_|{let state=self.rejected_state.take().unwrap();match ControlledRetirement::new(state){Ok(owner)=>self.rejected_close=Some(owner),Err((error,state))=>{self.rejected_state=Some(state);return Err(error)}}Ok(p(d))})?;return Ok(false)}
  if let Some(owner)=self.rejected_close.as_mut(){if owner.terminal_is_empty(){close_paid(cx,d,|_|{self.rejected_close=None;Ok(p(d))})?;}else{let selected=owner.select(4096)?;retired_poll(cx,d,|effect|selected.step(effect))?;}return Ok(false)}
  if self.rejected_window.is_some()||!self.address_close.is_empty(){if self.address_close.is_empty(){begun(cx,d,|effect|self.address_close.begin_granted(&mut self.rejected_window,effect))?;}else{let selected=self.address_close.select(4096)?;retired_poll(cx,d,|effect|selected.step(effect))?;}return Ok(false)}
  if self.binding.is_some(){close_paid(cx,d,|_|{self.binding=None;Ok(p(d))})?;return Ok(false)}Ok(true)
 }
 /// 🧺️ Neither original source custody nor unpublished/displaced aliases remain.
 pub fn terminal_is_empty(&self)->bool{self.seed_only||self.closing&&self.binding.is_none()&&self.document.terminal_is_empty()&&self.config.terminal_is_empty()&&self.presses.terminal_is_empty()&&self.gestures.terminal_is_empty()&&self.typing.terminal_is_empty()&&self.windows.is_none()&&self.document_input.is_none()&&self.config_input.is_none()&&self.window_input.is_none()&&self.document_admission.terminal_is_empty()&&self.config_admission.terminal_is_empty()&&self.window_admission.terminal_is_empty()&&self.document_candidate.is_none()&&self.config_candidate.is_none()&&self.document_displaced.is_none()&&self.config_displaced.is_none()&&self.rejected_window.is_none()&&self.rejected_key.is_none()&&self.rejected_state.is_none()&&self.rejected_close.is_none()&&self.address_close.is_empty()}
 /// 📦️ The final boxed header releases only after every original child is terminal.
 pub fn shell_close_demands(&self)->RetirementDemand{RetirementDemand{release_bytes:size_of::<Self>(),..header::<Option<Box<Self>>>()}}
}
impl<A:ArtifactApp> Drop for OriginalToolMachineFollow<A>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original tool follower retains native source custody")}}

type FollowBinding=(OriginalSourceBinding,(NativeAllocationIdentity,u64));

/// 📦️ Retains one native backing across funding turns and publishes only a complete mounted follower.
pub struct OriginalToolMachineFollowSlot<A:ArtifactApp>{
    backing:NativeOwnedStorage<OriginalToolMachineFollow<A>>,
    birth_binding:Option<FollowBinding>,
    ready:Option<Box<OriginalToolMachineFollow<A>>>,
}
impl<A:ArtifactApp> OriginalToolMachineFollowSlot<A>{
    pub const fn vacant()->Self{Self{backing:NativeOwnedStorage::empty(),birth_binding:None,ready:None}}
    pub fn is_some(&self)->bool{self.backing.reserved()||self.ready.is_some()}
    pub fn as_ref(&self)->Option<&OriginalToolMachineFollow<A>>{self.ready.as_deref()}
    pub fn as_mut(&mut self)->Option<&mut OriginalToolMachineFollow<A>>{self.ready.as_deref_mut()}
    pub fn take(&mut self)->Option<Box<OriginalToolMachineFollow<A>>>{self.ready.take()}

    fn binding_bytes()->usize{size_of::<Option<FollowBinding>>()}
    fn frontier_bytes()->usize{size_of::<(Option<FollowBinding>,u64,[u64;3],[u64;2],[[u8;32];2],bool,bool)>()}

    /// 📏️ Quotes one allocation, template page, pointer transfer or cancellation release.
    pub fn birth_demand(&self,closing:bool)->Result<RetirementDemand,ValueError>{
        if self.backing.initialized(){
            let mut demand=NativeOwnedStorage::<OriginalToolMachineFollow<A>>::box_demand();
            demand.copy_bytes+=Self::binding_bytes();
            if !closing{demand.copy_bytes+=Self::frontier_bytes()}
            return Ok(demand)
        }
        if closing{
            let mut demand=self.backing.release_demand();
            if self.birth_binding.is_some(){demand.copy_bytes+=Self::binding_bytes()}
            return Ok(demand)
        }
        if !self.backing.reserved(){
            let mut demand=NativeOwnedStorage::<OriginalToolMachineFollow<A>>::reserve_demand();
            demand.copy_bytes+=Self::binding_bytes();
            return Ok(demand)
        }
        self.backing.template_seed_demand(OriginalToolMachineFollow::<A>::seed_template())
    }

    /// 🌱️ Initializes native storage in bounded Source-bound turns, then captures the current frontier.
    pub fn birth_step(&mut self,machines:&Machines<A>,document:&Document<A>,config:&Config<A>,cx:&mut StepContext<'_>,closing:bool)->Result<RetainedCloneStep,OriginalTurnFailure>{
        if self.ready.is_some()||!machines.refold_owed{
            return Err(invalid("original tool follow requires its pending request and empty source slot").into())
        }
        let demand=self.birth_demand(closing)?;
        if !cx.retained_work_can_enter(demand)||(!closing&&cx.should_yield()){
            return Ok(RetainedCloneStep::Progress(Default::default()))
        }
        let decoder=cx.native_receiving_identity().ok_or_else(||invalid("original tool follow requires its actual decoder"))?;
        let binding=(OriginalSourceBinding::borrow(cx),decoder);
        if self.birth_binding.is_some_and(|original|original!=binding){
            return Err(invalid("original tool follow birth changed its caller or decoder").into())
        }
        let grant=cx.retained_grant();
        let mut advance=|cx:&mut StepContext<'_>|{
            if closing&&cx.retirement_should_yield(){return Ok(RetainedCloneStep::Progress(Default::default()))}
            let turn=cx.with_original_source_effect(grant,|_,effect|{
                let transfer=self.backing.initialized();
                let was_reserved=self.backing.reserved();
                let poll=if transfer{
                    self.backing.take_box_into(&mut self.ready,effect)
                }else if closing{
                    RetainedWorkPoll::Complete(self.backing.release_between_source_actions(effect))
                }else if was_reserved{
                    RetainedWorkPoll::Complete(self.backing.initialize_template_between_source_actions(OriginalToolMachineFollow::<A>::seed_template(),effect))
                }else{
                    self.backing.reserve_empty(effect)
                };
                let RetainedWorkPoll::Complete(turn)=poll else{return poll};
                let result=turn.result.map(|step|{
                    let mut progress=step.progress();
                    if let Some(owner)=self.ready.as_mut(){
                        self.birth_binding=None;
                        progress.copied_bytes+=Self::binding_bytes();
                        if !closing{
                            owner.bind(binding,machines,document,config);
                            progress.copied_bytes+=Self::frontier_bytes();
                        }
                    }else if !was_reserved&&self.backing.reserved(){
                        self.birth_binding=Some(binding);
                        progress.copied_bytes+=Self::binding_bytes();
                    }else if was_reserved&&!self.backing.reserved(){
                        self.birth_binding=None;
                        progress.copied_bytes+=Self::binding_bytes();
                    }
                    if matches!(step,RetainedCloneStep::Complete(_)){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)}
                });
                RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work:turn.work})
            })?;
            let Some((poll,work))=turn else{return Ok(RetainedCloneStep::Progress(Default::default()))};
            let result=match poll{
                RetainedWorkPoll::Complete(turn)=>turn.result,
                RetainedWorkPoll::Pending{..}=>Ok(RetainedCloneStep::Progress(Default::default())),
            };
            semio_framework_job::receive_original_retained_work(cx,RetainedWorkOutcome{result,work})
        };
        if closing{cx.with_original_retirement(advance)}else{advance(cx)}
    }
}
