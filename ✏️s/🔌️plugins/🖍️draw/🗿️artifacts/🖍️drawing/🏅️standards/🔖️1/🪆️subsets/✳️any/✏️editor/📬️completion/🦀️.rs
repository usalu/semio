//! 📬️ Draw cleanup retains the actual emitted operations, host effects and typed download.
use super::DrawingPlayApp;
use semio_plugin::{ArtifactToolCompletionValue,ArtifactCompletionFault,ArtifactDownloadOutput,EditorApp,Emit,EphemeralEmit,NoConfigMutation,NoDraftMutation,VacantVec,VacantDeque,CompletionEffect};
use semio_framework_value::{ValueError,ValueRefusalKind,ErasedSnapshotRetirement,retirement::{RetireOwned,RetirementCursor,RetirementStep,admit_retirement_cursor,record_retirement_work,retirement_cursor_birth_demand,controlled::{ControlledRetirement,admit_typed_controlled_retirement}},retained_clone::RetainedCloneProgress};
use semio_framework_value::{RetainedAllocationEffect, RetirementDemand, retained_clone_grant_admits};
use std::{collections::VecDeque, mem::ManuallyDrop};
#[cfg(test)]
use semio_framework_value::RetainedCloneGrant;
pub type App=EditorApp<DrawingPlayApp>;pub type Value=ArtifactToolCompletionValue<App>;type DrawingEmit=Emit<semio_s_artifact_draw_drawing_mutations::DrawingMutation,NoConfigMutation,NoDraftMutation>;
enum Source{Emit(DrawingEmit,EphemeralEmit<App>),Download(Option<ArtifactDownloadOutput>,EphemeralEmit<App>),Fault(Option<ArtifactCompletionFault>,EphemeralEmit<App>)}
impl Source{fn ephemeral(&self)->&EphemeralEmit<App>{match self{Self::Emit(_,value)|Self::Download(_,value)|Self::Fault(_,value)=>value}}fn ephemeral_mut(&mut self)->&mut EphemeralEmit<App>{match self{Self::Emit(_,value)|Self::Download(_,value)|Self::Fault(_,value)=>value}}}
pub struct DrawingCompletionPayload(pub Value);
#[derive(semio_framework_value::RetireOwned)]
pub struct RejectedCompletion{publication:DrawingCompletionPayload,fault:semio_framework::Fault}
impl RejectedCompletion{pub fn new(original:semio_plugin::ArtifactToolCompletionRejection<App>)->Self{Self{publication:DrawingCompletionPayload(Value::Emit(original.emit.map_err(ArtifactCompletionFault::new),original.ephemeral)),fault:original.fault}}}
pub struct Cursor{original:ManuallyDrop<Option<Source>>,phase:u8,faulted:bool}
fn unsupported()->ValueError{ValueError::literal(ValueRefusalKind::UnsupportedOwner,"Draw completion retains an original opaque payload without an exact retirement facet")}
pub fn demand(value: &Value) -> Result<RetirementDemand, ValueError> {
    if matches!(value,Value::Genesis(..)){return semio_framework_value::retirement::controlled::controlled_retirement_birth_demand(value);}
    if !supported(value) { return Err(unsupported()); }
    let capacity_bytes = size_of::<ControlledRetirement<DrawingCompletionPayload>>();
    Ok(RetirementDemand {
        copy_bytes: size_of::<Option<DrawingCompletionPayload>>() + capacity_bytes,
        capacity_bytes,
        depth: 1,
        ..Default::default()
    })
}
pub fn admit(value:&mut Option<Value>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::RetainedWorkOutcome<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>>{
 let Some(original)=value.as_ref()else{return semio_framework_value::RetainedWorkOutcome{result:Ok(None),work:0};};
 if matches!(original,Value::Genesis(..)){
    let turn=admit_typed_controlled_retirement(value,effect);
    return semio_framework_value::RetainedWorkOutcome{work:turn.work,result:turn.result.map(|owner|owner.map(|(owner,progress)|(owner as Box<dyn ErasedSnapshotRetirement>,progress)))};
 }
 let selected=match demand(original){Ok(demand)=>demand,Err(error)=>return semio_framework_value::RetainedWorkOutcome{result:Err(error),work:0}};
 if effect.maximum_work_units()==0||!retained_clone_grant_admits(effect.grant(),selected){return semio_framework_value::RetainedWorkOutcome{result:Ok(None),work:0};}
 let mut payload=value.take().map(DrawingCompletionPayload);
 let turn=admit_typed_controlled_retirement(&mut payload,effect);
 *value=payload.map(|DrawingCompletionPayload(original)|original);
 semio_framework_value::RetainedWorkOutcome{work:turn.work,result:turn.result.map(|owner|owner.map(|(owner,progress)|(owner as Box<dyn ErasedSnapshotRetirement>,progress)))}
}
fn supported(value:&Value)->bool{
 let ephemeral=match value{Value::Emit(_,ephemeral)|Value::Download(_,ephemeral)=>ephemeral,Value::Genesis(..)=>return false};if !ephemeral.presence.is_empty()||!ephemeral.transient.is_empty(){return false;}
 match value{Value::Emit(Ok(emit),_)=>emit.config_mutations.is_empty()&&emit.draft_mutations.is_empty()&&emit.extension_invocations.is_empty()&&emit.events.is_empty()&&emit.child_emits.is_empty()&&emit.owned_child_emits.is_empty()&&emit.child_preparations.is_empty()&&emit.tasks.is_empty()&&emit.effects.len()<=1024&&emit.effects.iter().all(CompletionEffect::supported),_=>true}
}
impl RetireOwned for DrawingCompletionPayload {
    type RetirementCursor = Cursor;

    fn retirement_seed(self) -> Cursor {
        let original = match self.0 {
            Value::Emit(Ok(emit), ephemeral) => Source::Emit(emit, ephemeral),
            Value::Download(Ok(download), ephemeral) => Source::Download(Some(download), ephemeral),
            Value::Emit(Err(fault), ephemeral) | Value::Download(Err(fault), ephemeral) => Source::Fault(Some(fault), ephemeral),
            Value::Genesis(..) => unreachable!("typed genesis uses generic bounded retirement"),
        };
        Cursor { original: ManuallyDrop::new(Some(original)), phase: 0, faulted: false }
    }

    fn retirement_ready(&self) -> bool { supported(&self.0) }
    fn controlled_retirement_supported() -> bool { true }
}

fn wrapped_demand<T: RetireOwned>(birth: Option<usize>) -> Result<RetirementDemand, ValueError> {
    let capacity_bytes = birth.ok_or_else(unsupported)?;
    Ok(RetirementDemand {
        copy_bytes: size_of::<Option<T>>() + capacity_bytes,
        capacity_bytes,
        depth: 1,
        ..Default::default()
    })
}

fn vacant_vec_demand<T: Send + 'static>(value: &Vec<T>) -> Result<RetirementDemand, ValueError> {
    wrapped_demand::<VacantVec<T>>(VacantVec::original_birth(value))
}

fn vacant_deque_demand<T: Send + 'static>(value: &VecDeque<T>) -> Result<RetirementDemand, ValueError> {
    wrapped_demand::<VacantDeque<T>>(VacantDeque::original_birth(value))
}

fn admit_child<T: RetireOwned>(slot: &mut Option<T>, effect: &mut RetainedAllocationEffect<'_>) -> RetirementStep {
    let turn = admit_retirement_cursor(slot, effect);
    match turn.result {
        Ok(Some((child, progress))) => RetirementStep::ChildProgress(child, progress, turn.work),
        Ok(None) => RetirementStep::BudgetExhausted,
        Err(error) => RetirementStep::Failure(error.into(), turn.work),
    }
}

fn admit_wrapped<T: Default, U: RetireOwned>(
    field: &mut T,
    wrap: impl FnOnce(T) -> U,
    unwrap: impl FnOnce(U) -> T,
    effect: &mut RetainedAllocationEffect<'_>,
) -> RetirementStep {
    let mut slot = Some(wrap(std::mem::take(field)));
    let step = admit_child(&mut slot, effect);
    if let Some(value) = slot { *field = unwrap(value); }
    step
}

impl Cursor {
    fn final_phase(&self) -> bool {
        let Some(source) = self.original.as_ref() else { return false; };
        self.phase >= if matches!(source, Source::Emit(..)) { 18 } else { 4 }
    }

    fn close_step(&mut self, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> RetirementStep {
        if self.terminal_is_empty() { return RetirementStep::Complete(0); }
        if self.faulted { return RetirementStep::Failure(unsupported().into(), 0); }
        let demand = match self.next_demand(body) {
            Ok(demand) => demand,
            Err(error) => {
                self.faulted = true;
                return RetirementStep::Failure(error.into(), 0);
            }
        };
        if !retained_clone_grant_admits(effect.grant(), demand) { return RetirementStep::BudgetExhausted; }
        if self.final_phase() {
            let work = match record_retirement_work(effect, 1) {
                Ok(work) => work,
                Err((error, work)) => return RetirementStep::Failure(error.into(), work),
            };
            drop(self.original.take());
            return RetirementStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }, work);
        }
        let mut finish_phase = true;
        let step = match self.original.as_mut().unwrap() {
            Source::Emit(emit, ephemeral) => match self.phase {
                0 => admit_wrapped(&mut emit.artifact_mutations, |value| value, |value| value, effect),
                1 => admit_wrapped(&mut emit.config_mutations, VacantVec, |value| value.0, effect),
                2 => admit_wrapped(&mut emit.window_config_mutations, |value| value, |value| value, effect),
                3 => admit_wrapped(&mut emit.draft_mutations, VacantVec, |value| value.0, effect),
                4 => admit_wrapped(&mut emit.transaction, |value| value, |value| value, effect),
                5 => admit_wrapped(&mut emit.transaction_phase, |value| value, |value| value, effect),
                6 => if let Some(effect_value) = emit.effects.pop() { finish_phase = false; let mut slot = Some(CompletionEffect(effect_value)); let step = admit_child(&mut slot, effect); if let Some(CompletionEffect(value)) = slot { emit.effects.push(value); } step } else { admit_wrapped(&mut emit.effects, VacantVec, |value| value.0, effect) },
                7 => admit_wrapped(&mut emit.extension_invocations, VacantVec, |value| value.0, effect),
                8 => admit_wrapped(&mut emit.events, VacantVec, |value| value.0, effect),
                9 => admit_wrapped(&mut emit.ui_scope, |value| value, |value| value, effect),
                10 => admit_wrapped(&mut emit.child_emits, VacantVec, |value| value.0, effect),
                11 => admit_wrapped(&mut emit.owned_child_emits, VacantVec, |value| value.0, effect),
                12 => admit_wrapped(&mut emit.child_preparations, VacantDeque, |value| value.0, effect),
                13 => admit_wrapped(&mut emit.interaction_writes, |value| value, |value| value, effect),
                14 => admit_wrapped(&mut emit.tasks, VacantVec, |value| value.0, effect),
                15 => admit_wrapped(&mut ephemeral.presence, VacantVec, |value| value.0, effect),
                16 => admit_wrapped(&mut ephemeral.transient, VacantVec, |value| value.0, effect),
                17 => admit_wrapped(&mut ephemeral.window_transient, |value| value, |value| value, effect),
                _ => unreachable!(),
            },
            Source::Download(download, _) if self.phase == 0 => admit_wrapped(download, |value| value, |value| value, effect),
            Source::Fault(fault, _) if self.phase == 0 => admit_wrapped(fault, |value| value, |value| value, effect),
            source => match self.phase {
                1 => admit_wrapped(&mut source.ephemeral_mut().presence, VacantVec, |value| value.0, effect),
                2 => admit_wrapped(&mut source.ephemeral_mut().transient, VacantVec, |value| value.0, effect),
                3 => admit_wrapped(&mut source.ephemeral_mut().window_transient, |value| value, |value| value, effect),
                _ => unreachable!(),
            },
        };
        if finish_phase && matches!(step, RetirementStep::ChildProgress(..)) { self.phase += 1; }
        step
    }
}

impl RetirementCursor for Cursor {
    semio_framework_value::native_retirement_reply!();

    fn next_computational_work(&self, _: usize) -> Result<u64, ValueError> {
        Ok(u64::from(!self.terminal_is_empty()))
    }

    fn next_demand(&self, _: usize) -> Result<RetirementDemand, ValueError> {
        let Some(source) = self.original.as_ref() else { return Ok(Default::default()); };
        if self.final_phase() { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
        match source {
            Source::Emit(emit, ephemeral) => match self.phase {
                0 => retirement_cursor_birth_demand(&emit.artifact_mutations),
                1 => vacant_vec_demand(&emit.config_mutations),
                2 => retirement_cursor_birth_demand(&emit.window_config_mutations),
                3 => vacant_vec_demand(&emit.draft_mutations),
                4 => retirement_cursor_birth_demand(&emit.transaction),
                5 => retirement_cursor_birth_demand(&emit.transaction_phase),
                6 => emit.effects.last().map_or_else(|| vacant_vec_demand(&emit.effects), |effect| wrapped_demand::<CompletionEffect>(CompletionEffect::original_birth(effect))),
                7 => vacant_vec_demand(&emit.extension_invocations),
                8 => vacant_vec_demand(&emit.events),
                9 => retirement_cursor_birth_demand(&emit.ui_scope),
                10 => vacant_vec_demand(&emit.child_emits),
                11 => vacant_vec_demand(&emit.owned_child_emits),
                12 => vacant_deque_demand(&emit.child_preparations),
                13 => retirement_cursor_birth_demand(&emit.interaction_writes),
                14 => vacant_vec_demand(&emit.tasks),
                15 => vacant_vec_demand(&ephemeral.presence),
                16 => vacant_vec_demand(&ephemeral.transient),
                17 => retirement_cursor_birth_demand(&ephemeral.window_transient),
                _ => unreachable!(),
            },
            Source::Download(download, _) if self.phase == 0 => retirement_cursor_birth_demand(download),
            Source::Fault(fault, _) if self.phase == 0 => retirement_cursor_birth_demand(fault),
            source => match self.phase {
                1 => vacant_vec_demand(&source.ephemeral().presence),
                2 => vacant_vec_demand(&source.ephemeral().transient),
                3 => retirement_cursor_birth_demand(&source.ephemeral().window_transient),
                _ => unreachable!(),
            },
        }
    }

    fn next_selected_work_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.next_demand(body)?.copy_bytes) }
    fn next_birth_bytes(&self, body: usize) -> Option<usize> { self.next_demand(body).ok().map(|demand| demand.capacity_bytes) }
    fn next_close_byte_demand(&self) -> Option<usize> { Some(0) }
    fn terminal_is_empty(&self) -> bool { self.original.is_none() }
    fn terminal_release_bytes(&self) -> Option<usize> { self.terminal_is_empty().then_some(size_of::<Self>()) }
}
impl Drop for Cursor{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"actual Draw completion payload reached Drop before all retained fields returned");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.original);}}}}



#[cfg(test)]
#[path="🎟️close/🧪️tests/🦀️.rs"]
mod close_work_tests;
