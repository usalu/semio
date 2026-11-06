//! 🔌️ Extension invocation and command ingress of the reactor turn: the retained extension invocation with its receive and advance steps and the retained command ingress with its owner cursor, retirement, park and close steps. See [`super`].

use super::*;
fn retired(cx:&mut semio_framework_job::StepContext<'_>,demand:semio_framework_value::RetirementDemand,advance:impl FnOnce(semio_framework_value::RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{semio_framework_job::close_original_retained_step(cx,demand,|grant|semio_framework_value::RetainedWorkOutcome{result:advance(grant),work:1})}

#[doc(hidden)]
pub struct RetainedExtensionInvocation{
    pub req:semio_framework::kernel::RequestId,
    pub source:Option<(String,Vec<u8>,String)>,
    origin_kind:u8,
    pub result:Option<semio_framework::kernel::RequestOutcome>,
    pub failure:Option<RetainedExtensionFaultReply>,
    refused_output:Option<semio_framework_value::retirement::controlled::ControlledRetirement<Vec<u8>>>,
    pub closing:Option<semio_framework_value::retirement::controlled::ControlledRetirement<(String,Vec<u8>,String)>>,
}

impl RetainedExtensionInvocation{
    pub fn new(req:semio_framework::kernel::RequestId,from:MessageEndpoint,capability:String,payload:Vec<u8>)->Self{
        let(origin_kind,origin)=match from{MessageEndpoint::Shell{instance}=>(0,instance.0),MessageEndpoint::Backbone{uri}=>(1,uri),MessageEndpoint::PluginInstance{id}=>(2,id.0),MessageEndpoint::Extension{id}=>(3,id),MessageEndpoint::Topic{name}=>(4,name)};
        Self{req,source:Some((capability,payload,origin)),origin_kind,result:None,failure:None,refused_output:None,closing:None}
    }

    pub async fn step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework::kernel::RequestOutcome>,semio_framework_job::OriginalTurnFailure>{
        if cx.should_yield(){return Ok(None)}
        let grant=semio_framework_value::RetainedCloneGrant{maximum_items:cx.retained_grant().maximum_items.min(1),..cx.retained_grant()};
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
        if let Some(closing)=self.refused_output.as_mut(){
            if closing.terminal_is_empty(){cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);self.refused_output=None;return Ok(None)}
            retired(cx,closing.selected_demand(grant.maximum_copy_bytes)?,|loan|closing.step(loan))?;
            return Ok(None)
        }
        if let Some(failure)=self.failure.as_mut(){
            if let Some(payload)=failure.advance(cx)?{self.result=Some(semio_framework::kernel::RequestOutcome::Err(payload));self.failure=None;}
            return Ok(None)
        }
        if let Some(closing)=self.closing.as_mut(){
            if !closing.terminal_is_empty(){retired(cx,closing.selected_demand(grant.maximum_copy_bytes)?,|loan|closing.step(loan))?;return Ok(None)}
            cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);self.closing=None;return Ok(self.result.take())
        }
        if self.result.is_some(){cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);self.begin_source_close();return Ok(None)}
        let Some((capability,payload,_))=self.source.as_ref()else{return Ok(None)};
        let answer=match semio_plugin_extension::extension_runtime_lift::extension_invoke(capability,payload,cx).await{Ok(answer)=>answer,Err(cause)=>{self.failure=Some(RetainedExtensionFaultReply::new(ExtensionInvocationCause::Fault(cause)));return Ok(None)}};
        if let Some(cause)=answer.refusal{self.failure=Some(RetainedExtensionFaultReply::new(ExtensionInvocationCause::Turn(cause)));if let Some(payload)=answer.payload{self.refused_output=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(payload).unwrap_or_else(|_|unreachable!("original refused bytes declare typed retirement")))}return Ok(None)}
        let Some(payload)=answer.payload else{return Ok(None)};
        self.result=Some(semio_framework::kernel::RequestOutcome::Ok(payload));
        Ok(None)
    }
    fn begin_source_close(&mut self){let original=self.source.take().expect("original invocation retains caller fields until actual output");self.closing=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(original).unwrap_or_else(|_|unreachable!("original request fields declare complete typed retirement")));}
}

/// 🔌️ How many extension invocations still own their caller fields or output: a settled reactor holds none, and a thread must not end while one still owns a controlled retirement.
pub fn retained_extension_invocation_occupancy()->usize{
    RETAINED_EXTENSION_INVOCATIONS.with(|owners|owners.borrow().iter().filter(|owner|owner.is_some()).count())
}

pub(super) async fn advance_retained_extension_invocation(cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<Effect>,semio_framework_job::OriginalTurnFailure>{
    let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
    let start=RETAINED_EXTENSION_CURSOR.get();let selected=RETAINED_EXTENSION_INVOCATIONS.with(|owners|{let mut owners=owners.borrow_mut();(0..requests::REQUEST_SLOTS).map(|offset|(start+offset)%requests::REQUEST_SLOTS).find_map(|index|owners[index].take().map(|owner|(index,owner)))});
    let Some((index,mut owner))=selected else{return Ok(None)};let result=owner.step(cx).await;RETAINED_EXTENSION_CURSOR.set((index+1)%requests::REQUEST_SLOTS);
    receive_retained_extension_invocation(index,owner,result)
}

#[doc(hidden)]
pub fn receive_retained_extension_invocation(index:usize,owner:RetainedExtensionInvocation,result:Result<Option<semio_framework::kernel::RequestOutcome>,semio_framework_job::OriginalTurnFailure>)->Result<Option<Effect>,semio_framework_job::OriginalTurnFailure>{
    match result{Ok(Some(result))=>Ok(Some(Effect::Respond{req:owner.req,result})),result=>{RETAINED_EXTENSION_INVOCATIONS.with(|owners|owners.borrow_mut()[index]=Some(owner));result.map(|_|None)}}
}

#[doc(hidden)]
pub enum CommandIngressOwner {
    ReservedPresence { cursor: semio_framework::kernel::CommandPageCursor, admission: semio_plugin_peer::PresenceRosterAdmission, page: semio_framework::kernel::FixedCommandPage },
    Presence { cursor:semio_framework::kernel::CommandPageCursor,publication_generation:u64,exchange:Option<semio_framework_plugin_runtime_ops::ops_ingress::OriginalCommandExchangeState>,exchange_close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_plugin_runtime_ops::ops_ingress::OriginalCommandExchangeState>>,terminal:bool,terminal_fault:Option<Vec<u8>> },
    PendingPresencePage { cursor: semio_framework::kernel::CommandPageCursor, publication_generation: u64, page: semio_framework::kernel::FixedCommandPage },
    GenericAssembly { cursor: semio_framework::kernel::CommandPageCursor, pages: semio_framework::kernel::CommandPageSet },
    ClosingAssembly { cursor:semio_framework::kernel::CommandPageCursor,pages:Option<semio_framework::kernel::CommandPageSet>,retirement:Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework::kernel::CommandPageSet>> },
    Generic { cursor:semio_framework::kernel::CommandPageCursor,command:Option<(u64,semio_framework_plugin_runtime_ops::ops_ingress::PluginCommandIngress)>,exchange:Option<semio_framework_plugin_runtime_ops::ops_ingress::OriginalCommandExchangeState>,exchange_close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_plugin_runtime_ops::ops_ingress::OriginalCommandExchangeState>>,output:Option<semio_framework_plugin_runtime_ops::ops_ingress::PluginExchangeOutput>,output_close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_plugin_runtime_ops::ops_ingress::PluginExchangeOutput>> },
}

#[doc(hidden)]
pub struct RetainedCommandIngress {
    pub key: instance_lifetime::NativeCloseKey,
    pub state: Option<CommandIngressOwner>,
}
pub(super) fn close_exchange_owner<T:semio_framework_value::RetireOwned>(owner:&mut Option<T>,closing:&mut Option<semio_framework_value::retirement::controlled::ControlledRetirement<T>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
 if owner.is_none()&&closing.is_none(){return Ok(true)}let bytes=cx.retained_grant().maximum_copy_bytes;let demand=semio_plugin_ownership::bounded_ownership::plugin_typed_owner_demand(owner,closing,bytes)?;let work=semio_plugin_ownership::bounded_ownership::plugin_typed_owner_work_demand(owner,closing,bytes)?;cx.with_original_retirement(|cx|semio_framework_job::close_original_quoted_retained_step(cx,demand,work,|grant|semio_plugin_ownership::bounded_ownership::plugin_typed_owner_original(owner,closing,grant)).map(|_|()))?;Ok(owner.is_none()&&closing.is_none())
}

pub(super) struct CommandIngressLoan{index:usize,entry:std::mem::ManuallyDrop<Option<RetainedCommandIngress>>,acquired:bool}
impl CommandIngressLoan{
 pub(super) fn empty(index:usize)->Self{Self{index,entry:std::mem::ManuallyDrop::new(None),acquired:false}}
 pub(super) fn lend(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{COMMAND_INGRESS.with(|slots|slots.borrow_mut()[self.index].lend_step(&mut self.entry,&mut self.acquired,cx))}
 pub(super) fn restore(&mut self)->Result<(),semio_framework_job::OriginalTurnFailure>{if self.acquired{COMMAND_INGRESS.with(|slots|slots.borrow_mut()[self.index].return_owner(&mut self.entry))?;self.acquired=false}Ok(())}
 pub(super) fn state(&self)->Option<&CommandIngressOwner>{self.entry.as_ref().and_then(|entry|entry.state.as_ref())}
 pub(super) fn state_mut(&mut self)->Option<&mut CommandIngressOwner>{self.entry.as_mut().and_then(|entry|entry.state.as_mut())}
 pub(super) fn state_slot(&mut self)->Option<&mut Option<CommandIngressOwner>>{self.entry.as_mut().map(|entry|&mut entry.state)}
 pub(super) fn take_state(&mut self)->Option<CommandIngressOwner>{self.entry.as_mut().and_then(|entry|entry.state.take())}
 pub(super) fn put_state(&mut self,state:Option<CommandIngressOwner>){if let Some(entry)=self.entry.as_mut(){entry.state=state}else{assert!(state.is_none(),"command state retains its admitted entry")}}
 pub(super) fn key(&self)->Option<instance_lifetime::NativeCloseKey>{self.entry.as_ref().map(|entry|entry.key)}
 pub(super) fn admit_entry(&mut self,key:instance_lifetime::NativeCloseKey,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{if let Some(entry)=self.entry.as_ref(){return Ok(entry.key==key)}let bytes=std::mem::size_of::<Option<RetainedCommandIngress>>();let grant=cx.retained_grant();if cx.should_yield()||cx.fuel_remaining()==0||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}*self.entry=Some(RetainedCommandIngress{key,state:None});cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)}
 pub(super) fn clear_empty_entry(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{if self.entry.as_ref().is_none_or(|entry|entry.state.is_some()){return Ok(())}let bytes=std::mem::size_of::<Option<RetainedCommandIngress>>();let grant=cx.retained_grant();if cx.retirement_should_yield()||cx.fuel_remaining()==0||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(())}self.entry.take();cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(())}
}
impl Drop for CommandIngressLoan{fn drop(&mut self){self.restore().expect("armed command return keeps its exact preissued Source target and pinned donor")}}

/// 📥️ The cursor the retained owner is holding, whatever shape it holds it in — the one answer
/// [`command_ingress_never_idle_while_owned`](crate::plugin::reactor::turn) needs to turn "this turn
/// advanced nothing" into "this command is still mine", which is the difference between a host that
/// keeps driving and a host that spins on `Idle` for a thousand crossings.
/// 📥️ The law a turn that still OWNS a command ingress answers by: `Idle` means "no owner", and
/// nothing else, so an owner that advanced nothing this turn says `CommandPending` rather than going
/// silent. Without it the host's drain cannot tell "still mine, working" from "gone", and the only
/// bound left is a crossing count — which is how `command ingress did not complete within 1024
/// continuations (observed statuses: idle)` came to be the whole diagnosis.
#[doc(hidden)]
pub fn command_ingress_while_owned(status: semio_framework::kernel::CommandIngressStatus, owner: Option<semio_framework::kernel::CommandPageCursor>) -> semio_framework::kernel::CommandIngressStatus {
    match (status, owner) {
        (semio_framework::kernel::CommandIngressStatus::Idle, Some(cursor)) => semio_framework::kernel::CommandIngressStatus::CommandPending(cursor),
        (status, _) => status,
    }
}

pub(super) fn command_ingress_owner_cursor(owner: &CommandIngressOwner) -> semio_framework::kernel::CommandPageCursor {
    match owner {
        CommandIngressOwner::ReservedPresence { cursor, .. }
        | CommandIngressOwner::Presence { cursor, .. }
        | CommandIngressOwner::PendingPresencePage { cursor, .. }
        | CommandIngressOwner::GenericAssembly { cursor, .. }
        | CommandIngressOwner::ClosingAssembly { cursor, .. }
        | CommandIngressOwner::Generic { cursor, .. } => cursor.clone(),
    }
}

pub(super) fn retire_command_ingress(slot:&mut Option<CommandIngressOwner>,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
    if matches!(slot,Some(CommandIngressOwner::GenericAssembly{..})){
        let bytes=std::mem::size_of::<Option<CommandIngressOwner>>();let p=semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};if cx.retirement_should_yield()||cx.fuel_remaining()==0||cx.retained_grant().maximum_depth==0||!p.fits(cx.retained_grant()){return Ok(())}cx.retained_progress().checked_add(p)?;
        let Some(CommandIngressOwner::GenericAssembly{cursor,pages})=slot.take()else{unreachable!()};*slot=Some(CommandIngressOwner::ClosingAssembly{cursor,pages:Some(pages),retirement:None});let posted=cx.consume_retained(p);cx.consume_fuel(1);posted?;return Ok(())
    }
    if let Some(CommandIngressOwner::ClosingAssembly{pages,retirement,..})=slot.as_mut(){if !close_exchange_owner(pages,retirement,cx)?{return Ok(())}let bytes=std::mem::size_of::<Option<CommandIngressOwner>>();let p=semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};if cx.retirement_should_yield()||cx.fuel_remaining()==0||cx.retained_grant().maximum_depth==0||!p.fits(cx.retained_grant()){return Ok(())}cx.retained_progress().checked_add(p)?;slot.take();let posted=cx.consume_retained(p);cx.consume_fuel(1);posted?;return Ok(())}
    if let Some(CommandIngressOwner::ReservedPresence{admission,..})=slot.as_mut(){admission.cancel.cancel_now();return Ok(())}
    if let Some(CommandIngressOwner::Presence{exchange,exchange_close,..})=slot.as_mut(){if !close_exchange_owner(exchange,exchange_close,cx)?{return Ok(())}return Ok(())}
    if matches!(slot,Some(CommandIngressOwner::PendingPresencePage{..})){return Ok(())}
    if let Some(CommandIngressOwner::Generic{exchange,exchange_close,output,output_close,..})=slot.as_mut(){if !close_exchange_owner(output,output_close,cx)?||!close_exchange_owner(exchange,exchange_close,cx)?{return Ok(())}}
    let Some(state)=slot.take()else{return Ok(())};*slot=match state {
        original @ (CommandIngressOwner::ReservedPresence{..}|CommandIngressOwner::Presence{..}|CommandIngressOwner::PendingPresencePage{..})=>Some(original),
        CommandIngressOwner::GenericAssembly{..}|CommandIngressOwner::ClosingAssembly{..}=>unreachable!("assembly Source retirement precedes owner extraction"),
        CommandIngressOwner::Generic { cursor, command:Some((seq,command)),exchange,exchange_close,output,output_close } if matches!(command,semio_framework_plugin_runtime_ops::ops_ingress::PluginCommandIngress::FailedDecoded{..}) => match command.retire_failed_step_on_original(cx){
            semio_framework_plugin_runtime_ops::ops_ingress::OriginalCommandIngressRetirement::Refused{owner,failure}=>{*slot=Some(CommandIngressOwner::Generic{cursor,command:Some((seq,owner)),exchange,exchange_close,output,output_close});return Err(failure)},semio_framework_plugin_runtime_ops::ops_ingress::OriginalCommandIngressRetirement::Pending(command)=>Some(CommandIngressOwner::Generic{cursor,command:Some((seq,command)),exchange,exchange_close,output,output_close}),
            semio_framework_plugin_runtime_ops::ops_ingress::OriginalCommandIngressRetirement::Failed(failure)=>return Err(failure),
            semio_framework_plugin_runtime_ops::ops_ingress::OriginalCommandIngressRetirement::Closed=>None,
        },
        CommandIngressOwner::Generic { cursor, command:Some((seq,command)),exchange,exchange_close,output,output_close } => match command.retire_original(cx){semio_framework_plugin_runtime_ops::ops_ingress::OriginalCommandIngressRetirement::Refused{owner,failure}=>{*slot=Some(CommandIngressOwner::Generic{cursor,command:Some((seq,owner)),exchange,exchange_close,output,output_close});return Err(failure)},semio_framework_plugin_runtime_ops::ops_ingress::OriginalCommandIngressRetirement::Pending(command)=>Some(CommandIngressOwner::Generic{cursor,command:Some((seq,command)),exchange,exchange_close,output,output_close}),semio_framework_plugin_runtime_ops::ops_ingress::OriginalCommandIngressRetirement::Failed(failure)=>return Err(failure),semio_framework_plugin_runtime_ops::ops_ingress::OriginalCommandIngressRetirement::Closed=>None},
        CommandIngressOwner::Generic{command:None,..}=>None,
    };Ok(())
}

/// 📥️ Which of the guest's two fixed ingress slots holds the owner the host is DRIVING, and which
/// holds one it has stopped driving.
///
/// The host drives ONE command per instance at a time and keeps a `kernel::CommandBatchDriver` alive
/// for exactly as long as that command is in flight; there is no message on this wire that tells the
/// guest a command was abandoned. The only evidence the guest ever gets is a page that names a
/// DIFFERENT command on the same instance — at which point the retained owner is provably not being
/// driven any more. It cannot simply be dropped (its pages and its decoder are released one bounded
/// step at a time, like every other close ladder here), so it moves to the retiring lane, where it
/// closes on its own budget and is invisible to this turn's ingress status. The live lane is then
/// free for the command the host IS driving.
///
/// Before this, a superseded owner sat in the one used slot forever: every page for the new command
/// answered `Backpressure` and every turn that carried no page answered `CommandPending` with the
/// STALE cursor, which the host's `validate_cursor` correctly refused with
/// `plugin.command-cursor-mismatch` (ticket 26/09/18 slice A2 §6.1, slice R2).
#[doc(hidden)]
pub const COMMAND_INGRESS_LIVE_SLOT: usize = 1;

#[doc(hidden)]
pub const COMMAND_INGRESS_RETIRING_SLOT: usize = 0;

/// 🧹️ One bounded close step for a superseded owner. It owns no ingress status: the host has no
/// driver left for it, so anything this lane said would name a command the host cannot acknowledge.
#[doc(hidden)]
pub fn step_retiring_command_ingress(cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
 retire_command_slot(COMMAND_INGRESS_RETIRING_SLOT,cx)
}
fn retire_command_slot(index:usize,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
 if !COMMAND_INGRESS.with(|slots|slots.borrow_mut()[index].settle_step(cx))?{return Ok(())}
 if COMMAND_INGRESS.with(|slots|slots.borrow()[index].peek().is_none()){return Ok(())}
 let mut loan=std::pin::pin!(CommandIngressLoan::empty(index));if !loan.lend(cx)?{return Ok(())}
 if let Some(state)=loan.state_slot(){retire_command_ingress(state,cx)?;}
 loan.clear_empty_entry(cx)?;loan.restore()
}
pub(super) fn retiring_command_ingress_is_free()->bool{COMMAND_INGRESS.with(|slots|slots.borrow()[COMMAND_INGRESS_RETIRING_SLOT].peek().is_none())}
pub(super) fn park_retiring_command_ingress(key:instance_lifetime::NativeCloseKey,state:&mut Option<CommandIngressOwner>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
 if !COMMAND_INGRESS.with(|slots|slots.borrow_mut()[COMMAND_INGRESS_RETIRING_SLOT].settle_step(cx))?{return Ok(false)}
 if !retiring_command_ingress_is_free(){return Ok(false)}let mut loan=std::pin::pin!(CommandIngressLoan::empty(COMMAND_INGRESS_RETIRING_SLOT));if !loan.lend(cx)?||!loan.admit_entry(key,cx)?{return Ok(false)}
 let bytes=2*std::mem::size_of::<Option<CommandIngressOwner>>();let grant=cx.retained_grant();if cx.should_yield()||cx.fuel_remaining()<2||grant.maximum_items<2||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}
 loan.put_state(state.take());cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:2,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(2);loan.restore()?;Ok(true)
}
#[doc(hidden)]
pub fn close_command_ingress_step(key:instance_lifetime::NativeCloseKey,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
 for index in 0..2{let selected=COMMAND_INGRESS.with(|slots|slots.borrow()[index].peek().is_some_and(|entry|entry.key==key));if selected{retire_command_slot(index,cx)?;}else{COMMAND_INGRESS.with(|slots|slots.borrow_mut()[index].settle_step(cx))?;}}
 Ok(COMMAND_INGRESS.with(|slots|slots.borrow().iter().all(|slot|match slot.peek(){Some(entry)=>entry.key!=key,None=>slot.terminal_is_empty()})))
}
pub fn retained_command_ingress_occupancy()->usize{COMMAND_INGRESS.with(|slots|slots.borrow().iter().filter(|slot|slot.peek().is_some()).count())}

/// 🔌️ Advances the retained extension invocation and the extension retirement under the mounted close policy; reports whether extension retirement still has work.
///
/// ↩️ One `Effect::Respond` per inbound `Event::Request` this turn served — kept apart from the
/// document-backbone effects so the answer's ordering against them is explicit rather than
/// incidental (answers trail, so a request that also wrote the document publishes the write first).
///
/// 📥️ The inbound half of the `request`/`respond` seam (`📜️.wit`'s `request-event`,
/// "every 'someone else calls INTO this actor' seam is now one inbound `request`,
/// answered with the `respond` effect within a bounded number of turns"). This actor's
/// installed `ExtensionBundle` is the ONE capability table that answers it, so the
/// request is served inline and answered on the SAME turn — an extension handler is a
/// pure `Fn(&[u8]) -> Result<Vec<u8>, Fault>` with nothing to await. An actor with no
/// bundle answers the bundle's own typed refusal (`extension.inactive`/
/// `extension.missing`), never silence: dropping the event stranded every caller's
/// parked request forever (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub(super) async fn drive_extension_turn<PA: semio_plugin_reactor_app::PluginApp>(runtime: &semio_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>, effects: &mut Vec<Effect>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
    if let Some(effect)=advance_retained_extension_invocation(cx).await?{effects.push(effect);}
    let original_extension_grant = runtime.mounted_owner_policy().close;
    let remaining=cx.retained_grant();
    let extension_grant = semio_framework_value::RetainedCloneGrant { maximum_items: original_extension_grant.maximum_items.min(remaining.maximum_items),maximum_copy_bytes:original_extension_grant.maximum_copy_bytes.min(remaining.maximum_copy_bytes),maximum_capacity_bytes:original_extension_grant.maximum_capacity_bytes.min(remaining.maximum_capacity_bytes),maximum_release_bytes:original_extension_grant.maximum_release_bytes.min(remaining.maximum_release_bytes),maximum_depth:original_extension_grant.maximum_depth.min(remaining.maximum_depth) };
    let (extension_retirement_work, _) = cx.with_retained_loan(extension_grant,|same|semio_plugin_extension::extension_runtime_lift::extension_retirement_turn(same))?;
    Ok(extension_retirement_work)
}
