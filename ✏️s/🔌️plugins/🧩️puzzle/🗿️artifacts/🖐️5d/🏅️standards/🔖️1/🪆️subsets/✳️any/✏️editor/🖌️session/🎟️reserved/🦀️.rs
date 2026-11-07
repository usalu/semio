//! 🎟️ One closed inline work family retains all request lanes through the shared original job.
use super::{Puzzle5dClipboardJob,Puzzle5dPasteJob,Puzzle5dImportJob};
use crate::editor::puzzle5d::Puzzle5dPlayApp;
use semio_plugin::{EditorApp,ArtifactReservedToolJobRequest,plugin_app_close_prelude::ArtifactReservedWork};
use semio_framework_job::{StepContext,OriginalTurnFailure,OriginalJobReturnClaim,InteractiveJobStep,InteractiveJobCloseStep,JobOutcomeDescriptor,JobOutcomeView};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind};
type App=EditorApp<Puzzle5dPlayApp>;
#[derive(Default)]
pub struct Puzzle5dReservedWork{kind:u8,clipboard:Puzzle5dClipboardJob,paste:Puzzle5dPasteJob,import:Puzzle5dImportJob,closing:bool}
impl ArtifactReservedWork<App> for Puzzle5dReservedWork{
 fn step<'a>(&'a mut self,request:&'a mut ArtifactReservedToolJobRequest<App>,cx:&mut StepContext<'_>,claim:&mut OriginalJobReturnClaim<'_>)->InteractiveJobStep<'a>{
  if self.kind==0{let result=(||{if cx.should_yield(){return Ok(None)}let demand=RetirementDemand{copy_bytes:std::mem::size_of::<u8>(),depth:1,..Default::default()};if !cx.retained_work_can_enter(demand){return Ok(None)}if request.raw_wire.len()>request.contract.max_raw_wire_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"puzzle5d reserved wire exceeds its declared original capacity").into())}self.kind=match request.tool_id.as_str(){"copy"|"cut"=>1,"paste"=>2,"import-media"=>3,_=>return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"puzzle5d reserved route is not declared").into())};cx.consume_fuel(1);cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})?;Ok(None)})();return InteractiveJobStep::from_callback(result)}match self.kind{1=>self.clipboard.step(request,cx,claim),2=>self.paste.step(request,cx,claim),3=>self.import.step(request,cx,claim),_=>unreachable!()}
 }
 fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{match self.kind{1=>self.clipboard.borrow_outcome(descriptor),2=>self.paste.borrow_outcome(descriptor),3=>self.import.borrow_outcome(descriptor),_=>descriptor.yielded()}}
 fn begin_close(&mut self){self.closing=true;match self.kind{1=>self.clipboard.begin_close(),2=>self.paste.begin_close(),3=>self.import.begin_close(),_=>{}}}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{match self.kind{1=>self.clipboard.close_demands(body),2=>self.paste.close_demands(body),3=>self.import.close_demands(body),_=>Ok(Default::default())}}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{self.begin_close();match self.kind{1=>self.clipboard.close_step(cx),2=>self.paste.close_step(cx),3=>self.import.close_step(cx),_=>Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}}
 fn terminal_is_empty(&self)->bool{self.closing&&match self.kind{1=>self.clipboard.terminal_is_empty(),2=>self.paste.terminal_is_empty(),3=>self.import.terminal_is_empty(),_=>true}}
}
