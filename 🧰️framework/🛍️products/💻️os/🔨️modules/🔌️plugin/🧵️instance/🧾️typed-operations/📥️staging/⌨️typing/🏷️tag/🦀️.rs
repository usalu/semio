//! 🏷️ Typed and host action producers share the same original bounded tag capture.
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_decl_emission::ActionMeta;
use semio_framework_plugin_instance_extension::InstanceExtension;
use semio_framework_plugin_tool_run::tool_jobs_tool_machine::{ToolTag,OriginalToolTagCapture,OriginalToolTagSource,OriginalToolTagKind,OriginalToolWindow};
use semio_framework_tool_machine::{TypingCommit,ToolAbortReason,SCRUB_GESTURE_ARG,SCRUB_COMMIT_ARG,SCRUB_ABORT_ARG,TYPING_BUFFER_ARG,TYPING_COMMIT_ARG};
use semio_framework_value::{DslValue,ValueError,ValueRefusalKind,RetainedCloneProgress,RetainedCloneStep};
use semio_framework_job::{StepContext,OriginalTurnFailure,close_original_retained_step};
use crate::typed_bundle::TypedCtx;
use store::{ReferenceSpaceMember,MemberFactory};
impl<A:ArtifactApp,M:ReferenceSpaceMember+MemberFactory+'static,X:InstanceExtension> TypedCtx<'_,A,M,X>{
 /// 🪢️ True means the retained tag output is complete; None is the explicitly plain source selection.
 pub async fn capture_original_tool_tag(&mut self,action:&str,args:Option<&DslValue>,meta:&ActionMeta,cx:&mut StepContext<'_>,output:&mut Option<ToolTag>,ready:&mut bool)->Result<bool,OriginalTurnFailure>{
  if *ready{return Ok(true)}
  let text=|key:&str|args.and_then(|args|args.get(key)).and_then(DslValue::as_str);
  let typing=text(TYPING_BUFFER_ARG).filter(|value|!value.is_empty());let commit=text(TYPING_COMMIT_ARG);
  let selected=if let Some(value)=typing.filter(|_|commit.is_none()){Some((value,OriginalToolTagKind::TypingEdit))}else if typing.is_some()&&commit.and_then(TypingCommit::parse).is_some(){None}else{match text(SCRUB_GESTURE_ARG).filter(|value|!value.is_empty()){None=>None,Some(value)=>match text(SCRUB_ABORT_ARG){Some(reason)=>ToolAbortReason::parse(reason).map(|reason|(value,OriginalToolTagKind::ScrubAbort(reason))),None=>Some((value,if args.and_then(|args|args.get(SCRUB_COMMIT_ARG)).and_then(DslValue::as_bool)==Some(true){OriginalToolTagKind::ScrubCommit}else{OriginalToolTagKind::ScrubTick}))}}};
  let Some((value,kind))=selected else{if self.tools.original_tool_tag_capture.is_some(){return Ok(false)}if output.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"plain original tool source has occupied tag output").into())}let bytes=std::mem::size_of::<bool>();let mut done=None;return semio_framework_job::admit_original_construction(cx,&mut done,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:bytes,capacity_bytes:0,release_bytes:0,depth:1},|_|{*ready=true;Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}))})};
  let window=match meta.view_state.as_ref().and_then(|view|view.window_id.as_deref()){Some(window)=>OriginalToolWindow::Window(window),None=>OriginalToolWindow::Global};
  let source=OriginalToolTagSource{window,app:self.app.instance_id().await,action,value,kind};
  if self.tools.original_tool_tag_capture.as_ref().is_some_and(|owner|!owner.accepts(source,cx)){return Ok(false)}
  if let Some(owner)=self.tools.original_tool_tag_capture.as_mut(){if owner.complete(){if output.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original captured tag output was removed before source close").into())}if !owner.terminal_is_empty(){cx.with_original_retirement(|same|owner.close_step(same))?;return Ok(false)}let mut d=owner.shell_close_demands();d.copy_bytes+=2*std::mem::size_of::<bool>();let mut done=false;cx.with_original_retirement(|same|close_original_retained_step(same,d,|_|{self.tools.original_tool_tag_capture.take();*ready=true;done=true;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))}))?;return Ok(done)}}
  if self.tools.original_tool_tag_capture.is_none(){if output.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original tag capture requires an empty retained output").into())}OriginalToolTagCapture::admit(source,cx,&mut self.tools.original_tool_tag_capture)?;return Ok(false)}
  if !self.tools.original_tool_tag_capture.as_ref().unwrap().accepts(source,cx){return Ok(false)}
  self.tools.original_tool_tag_capture.as_mut().unwrap().advance(source,output,cx)?;Ok(false)
 }
}
