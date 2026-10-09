//! 📬️ Draw cleanup retains the actual emitted operations, host effects and typed download.
use super::DrawingPlayApp;
use semio_framework_plugin::{ArtifactToolCompletionValue,ArtifactCompletionFault,ArtifactDownloadOutput,EditorApp,Emit,EphemeralEmit,NoConfigMutation,NoDraftMutation,VacantVec,VacantDeque,CompletionEffect};
use semio_framework_value::{ValueError,ValueRefusalKind,ErasedSnapshotRetirement,retirement::{RetireOwned,RetirementCursor,RetirementStep,controlled::{ControlledRetirement,admit_typed_controlled_retirement}},retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
use std::mem::ManuallyDrop;
type App=EditorApp<DrawingPlayApp>;type Value=ArtifactToolCompletionValue<App>;type DrawingEmit=Emit<crate::DrawingMutation,NoConfigMutation,NoDraftMutation>;
enum Source{Emit(DrawingEmit,EphemeralEmit<App>),Download(Option<ArtifactDownloadOutput>,EphemeralEmit<App>),Fault(Option<ArtifactCompletionFault>,EphemeralEmit<App>)}
impl Source{fn ephemeral(&self)->&EphemeralEmit<App>{match self{Self::Emit(_,value)|Self::Download(_,value)|Self::Fault(_,value)=>value}}fn ephemeral_mut(&mut self)->&mut EphemeralEmit<App>{match self{Self::Emit(_,value)|Self::Download(_,value)|Self::Fault(_,value)=>value}}}
pub(crate) struct DrawingCompletionPayload(pub(crate) Value);
#[derive(semio_framework_value::RetireOwned)]
pub(crate) struct RejectedCompletion{publication:DrawingCompletionPayload,fault:semio_framework::Fault}
impl RejectedCompletion{pub(crate) fn new(original:semio_framework_plugin::ArtifactToolCompletionRejection<App>)->Self{Self{publication:DrawingCompletionPayload(Value::Emit(original.emit.map_err(ArtifactCompletionFault::new),original.ephemeral)),fault:original.fault}}}
struct Cursor{original:ManuallyDrop<Option<Source>>,phase:u8}
fn unsupported()->ValueError{ValueError::literal(ValueRefusalKind::UnsupportedOwner,"Draw completion retains an original opaque payload without an exact retirement facet")}
pub(super) fn birth(value:&Value)->Option<usize>{supported(value).then_some(std::mem::size_of::<ControlledRetirement<DrawingCompletionPayload>>())}
pub(super) fn admit(value:&mut Option<Value>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>{
 let Some(original)=value.as_ref()else{return Ok(None);};let bytes=birth(original).ok_or_else(unsupported)?;if grant.maximum_items==0||grant.maximum_capacity_bytes<bytes||grant.maximum_depth==0{return Ok(None);}
 match admit_typed_controlled_retirement(DrawingCompletionPayload(value.take().unwrap()),grant){Ok((owner,progress))=>Ok(Some((owner as Box<dyn ErasedSnapshotRetirement>,progress))),Err((error,DrawingCompletionPayload(original)))=>{*value=Some(original);Err(error)}}
}
fn supported(value:&Value)->bool{
 let ephemeral=match value{Value::Emit(_,ephemeral)|Value::Download(_,ephemeral)=>ephemeral};if !ephemeral.presence.is_empty()||!ephemeral.transient.is_empty(){return false;}
 match value{Value::Emit(Ok(emit),_)=>emit.config_mutations.is_empty()&&emit.draft_mutations.is_empty()&&emit.extension_invocations.is_empty()&&emit.events.is_empty()&&emit.child_emits.is_empty()&&emit.owned_child_emits.is_empty()&&emit.child_preparations.is_empty()&&emit.tasks.is_empty()&&emit.effects.len()<=1024&&emit.effects.iter().all(CompletionEffect::supported),_=>true}
}
impl RetireOwned for DrawingCompletionPayload{
 fn retirement(self)->Box<dyn RetirementCursor>{let original=match self.0{Value::Emit(Ok(emit),ephemeral)=>Source::Emit(emit,ephemeral),Value::Download(Ok(download),ephemeral)=>Source::Download(Some(download),ephemeral),Value::Emit(Err(fault),ephemeral)|Value::Download(Err(fault),ephemeral)=>Source::Fault(Some(fault),ephemeral)};Box::new(Cursor{original:ManuallyDrop::new(Some(original)),phase:0})}
 fn retirement_birth_bytes(&self)->Option<usize>{supported(&self.0).then_some(std::mem::size_of::<Cursor>())}
 fn controlled_retirement_supported()->bool{true}
}
impl Cursor{
 fn birth(&self)->Option<usize>{let Some(source)=self.original.as_ref()else{return Some(0);};match source{
 Source::Emit(emit,_)=>match self.phase{0=>emit.artifact_mutations.retirement_birth_bytes(),1=>VacantVec::original_birth(&emit.config_mutations),2=>emit.window_config_mutations.retirement_birth_bytes(),3=>VacantVec::original_birth(&emit.draft_mutations),4=>emit.transaction.retirement_birth_bytes(),5=>emit.transaction_phase.retirement_birth_bytes(),6=>emit.effects.last().map_or_else(||VacantVec::original_birth(&emit.effects),CompletionEffect::original_birth),7=>VacantVec::original_birth(&emit.extension_invocations),8=>VacantVec::original_birth(&emit.events),9=>emit.ui_scope.retirement_birth_bytes(),10=>VacantVec::original_birth(&emit.child_emits),11=>VacantVec::original_birth(&emit.owned_child_emits),12=>VacantDeque::original_birth(&emit.child_preparations),13=>emit.interaction_writes.retirement_birth_bytes(),14=>VacantVec::original_birth(&emit.tasks),15=>VacantVec::original_birth(&source.ephemeral().presence),16=>VacantVec::original_birth(&source.ephemeral().transient),17=>source.ephemeral().window_transient.retirement_birth_bytes(),_=>Some(0)},
 Source::Download(download,_)if self.phase==0=>download.retirement_birth_bytes(),Source::Fault(fault,_)if self.phase==0=>fault.retirement_birth_bytes(),
 _=>match self.phase{1=>VacantVec::original_birth(&source.ephemeral().presence),2=>VacantVec::original_birth(&source.ephemeral().transient),3=>source.ephemeral().window_transient.retirement_birth_bytes(),_=>Some(0)}
 }}
}
impl RetirementCursor for Cursor{
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{if self.terminal_is_empty(){return RetirementStep::Complete;}if grant.maximum_items==0{return RetirementStep::BudgetExhausted;}let Some(bytes)=self.birth()else{return RetirementStep::Failure(unsupported());};if grant.maximum_capacity_bytes<bytes||grant.maximum_depth==0{return RetirementStep::BudgetExhausted;}
  let source=self.original.as_mut().unwrap();let child=match source{
   Source::Emit(emit,ephemeral)=>match self.phase{0=>std::mem::take(&mut emit.artifact_mutations).retirement(),1=>VacantVec(std::mem::take(&mut emit.config_mutations)).retirement(),2=>std::mem::take(&mut emit.window_config_mutations).retirement(),3=>VacantVec(std::mem::take(&mut emit.draft_mutations)).retirement(),4=>std::mem::take(&mut emit.transaction).retirement(),5=>std::mem::take(&mut emit.transaction_phase).retirement(),6=>{if let Some(effect)=emit.effects.pop(){self.phase-=1;CompletionEffect(effect).retirement()}else{VacantVec(std::mem::take(&mut emit.effects)).retirement()}},7=>VacantVec(std::mem::take(&mut emit.extension_invocations)).retirement(),8=>VacantVec(std::mem::take(&mut emit.events)).retirement(),9=>std::mem::take(&mut emit.ui_scope).retirement(),10=>VacantVec(std::mem::take(&mut emit.child_emits)).retirement(),11=>VacantVec(std::mem::take(&mut emit.owned_child_emits)).retirement(),12=>VacantDeque(std::mem::take(&mut emit.child_preparations)).retirement(),13=>std::mem::take(&mut emit.interaction_writes).retirement(),14=>VacantVec(std::mem::take(&mut emit.tasks)).retirement(),15=>VacantVec(std::mem::take(&mut ephemeral.presence)).retirement(),16=>VacantVec(std::mem::take(&mut ephemeral.transient)).retirement(),17=>std::mem::take(&mut ephemeral.window_transient).retirement(),_=>{drop(self.original.take());return RetirementStep::Complete;}},
   Source::Download(download,_)if self.phase==0=>download.take().retirement(),Source::Fault(fault,_)if self.phase==0=>fault.take().retirement(),
   _=>match self.phase{1=>VacantVec(std::mem::take(&mut source.ephemeral_mut().presence)).retirement(),2=>VacantVec(std::mem::take(&mut source.ephemeral_mut().transient)).retirement(),3=>std::mem::take(&mut source.ephemeral_mut().window_transient).retirement(),_=>{drop(self.original.take());return RetirementStep::Complete;}}
  };self.phase+=1;RetirementStep::Child(child)
 }
 fn terminal_is_empty(&self)->bool{self.original.is_none()}
 fn next_birth_bytes(&self,_copy:usize)->Option<usize>{self.birth()}
 fn next_close_byte_demand(&self)->Option<usize>{Some(0)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
}
impl Drop for Cursor{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"actual Draw completion payload reached Drop before all retained fields returned");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.original);}}}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

