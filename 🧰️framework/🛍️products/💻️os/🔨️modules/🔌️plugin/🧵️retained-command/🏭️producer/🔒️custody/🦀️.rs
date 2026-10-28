//! 🔒️ The admitted typed box remains sealed while original retained work borrows its payload.
use semio_framework_plugin_decl_app::tool_contract::ArtifactOriginalToolCommand;
use semio_framework_job::StepContext;
use semio_framework_value::{ValueError,retirement::RetireOwned};
#[derive(semio_framework_value::RetireOwned)]enum CommandCustody<C:RetireOwned>{Received(C),Original(ArtifactOriginalToolCommand<C>)}
#[derive(semio_framework_value::RetireOwned)]pub struct ArtifactRetainedCommandCustody<C:RetireOwned>{command:CommandCustody<C>}
impl<C:RetireOwned> ArtifactRetainedCommandCustody<C>{
 pub(crate) fn received(command:C)->Self{Self{command:CommandCustody::Received(command)}}
 pub(crate) fn original(command:ArtifactOriginalToolCommand<C>)->Self{Self{command:CommandCustody::Original(command)}}
 pub(crate) fn is_original(&self)->bool{matches!(&self.command,CommandCustody::Original(_))}
 pub(crate) fn original_mut(&mut self,cx:&StepContext<'_>)->Result<Option<&mut C>,ValueError>{match &mut self.command{CommandCustody::Original(command)=>command.original_mut(cx),CommandCustody::Received(command)=>Ok((!cx.should_yield()&&!cx.is_cancelled()&&cx.retained_grant().maximum_items>0).then_some(command))}}
}
impl<C:RetireOwned> std::ops::Deref for ArtifactRetainedCommandCustody<C>{type Target=C;fn deref(&self)->&C{match &self.command{CommandCustody::Received(command)=>command,CommandCustody::Original(command)=>command.as_ref()}}}
