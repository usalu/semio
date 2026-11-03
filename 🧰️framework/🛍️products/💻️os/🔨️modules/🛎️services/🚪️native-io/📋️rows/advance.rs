//! 🚶️ Native row transfer, metadata observation and publication are distinct original-source turns.
use super::*;
fn control()->RetainedWorkOutcome{RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}}
impl NativeIoJob{
 pub(super) fn step_native_rows(&mut self,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
  cx.original_source_parent()?.validate_borrowed_context(cx)?;if cx.should_yield()||cx.is_cancelled(){return Ok(())}let g=cx.retained_grant();if g.maximum_items==0||g.maximum_depth==0{return Ok(())}
  if matches!(self.state,NativeIoState::StagingScan{..}){
   let NativeIoState::StagingScan{paths,..}=&mut self.state else{return Err(ValueError::invariant("native I/O state invariant violated").into())};let turn=paths.push_from(&mut self.scanned_path,g);
   if turn.result.is_err()||self.scanned_path.is_none(){let NativeIoState::StagingScan{entries,paths,directories_only,first_only}=std::mem::replace(&mut self.state,NativeIoState::Finished)else{return Err(ValueError::invariant("native I/O state invariant violated").into())};if turn.result.is_err(){self.retired_directory=Some(entries);self.state=NativeIoState::ClosingScanFault{paths,rejected:None,error:"native I/O directory result exceeded fixed path credits".into()};}else if first_only{self.retired_directory=Some(entries);self.result=Some(Ok(NativeIoValue::Paths(paths)));}else{self.state=NativeIoState::Scanning{entries,paths,directories_only,first_only};}}
   semio_framework_job::receive_original_retained_work(cx,turn)?;return Ok(())
  }
  let NativeIoState::ReadingModified{paths,modified}=&mut self.state else{return Err(ValueError::invariant("native I/O state invariant violated").into())};
  if self.modified_entry.is_some(){
   let turn=modified.push_from(&mut self.modified_entry,g);if turn.result.is_err(){let NativeIoState::ReadingModified{paths,modified}=std::mem::replace(&mut self.state,NativeIoState::Finished)else{return Err(ValueError::invariant("native I/O state invariant violated").into())};self.state=NativeIoState::ClosingModifiedFault{paths,modified,rejected:None,error:"native I/O modified result exceeded fixed path credits".into()};}semio_framework_job::receive_original_retained_work(cx,turn)?;return Ok(())
  }
  if let Some(path)=self.modified_path.as_ref(){
   let result=std::fs::metadata(path).and_then(|metadata|metadata.modified());
   match result{Ok(time)=>self.modified_entry=Some((self.modified_path.take().ok_or_else(||ValueError::invariant("native I/O modified path absent"))?,time)),Err(error)=>{let detail=format!("{}: {error}",path.display());self.retired_path=self.modified_path.take();let NativeIoState::ReadingModified{paths,modified}=std::mem::replace(&mut self.state,NativeIoState::Finished)else{return Err(ValueError::invariant("native I/O state invariant violated").into())};self.state=NativeIoState::ClosingModifiedFault{paths,modified,rejected:None,error:detail};}}
   semio_framework_job::receive_original_retained_work(cx,control())?;return Ok(())
  }
  if !paths.is_empty(){let turn=paths.pop_into(&mut self.modified_path,g);semio_framework_job::receive_original_retained_work(cx,turn)?;return Ok(())}
  let NativeIoState::ReadingModified{modified,..}=std::mem::replace(&mut self.state,NativeIoState::Finished)else{return Err(ValueError::invariant("native I/O state invariant violated").into())};self.result=Some(Ok(NativeIoValue::Modified(modified)));semio_framework_job::receive_original_retained_work(cx,control())?;Ok(())
 }
}
