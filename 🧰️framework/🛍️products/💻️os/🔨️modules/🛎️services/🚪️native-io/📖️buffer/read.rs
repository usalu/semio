//! 📂️ A stable original read backing and payload cursor survive every native poll and receipt refusal.
use super::*;
use native_read_buffer::NativeReadBuffer;
fn fixed()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
fn permits(g:RetainedCloneGrant,d:RetirementDemand)->bool{g.maximum_items>0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}
fn control()->RetainedWorkOutcome{RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}}
impl NativeIoJob{
 pub(super) fn open_original_read<'a>(&'a mut self,path:PathBuf,page:Option<(u64,usize)>,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure>{
  cx.original_source_parent()?.validate_borrowed_context(cx)?;
  self.request_path=Some(path);
  if page.is_some_and(|(_,maximum)|maximum==0||maximum>semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES){self.result=Some(Err("native I/O page exceeded the mounted one-page output authority".into()));self.state=NativeIoState::Finished;semio_framework_job::receive_original_retained_work(cx,control())?;return Ok(None)}
  let path=self.request_path.as_ref().ok_or_else(||ValueError::invariant("request_path absent"))?;match File::open(path){Ok(file)=>self.state=NativeIoState::InspectingRead{file,page},Err(error)=>{self.result=Some(Err(format!("{}: {error}",path.display())));self.state=NativeIoState::Finished}}
  semio_framework_job::receive_original_retained_work(cx,control())?;Ok(None)
 }
 fn observe_original_file(&mut self,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
  let g=cx.retained_grant();if !permits(g,fixed()){return Ok(())}
  let state=std::mem::replace(&mut self.state,NativeIoState::Finished);
  self.state=match state{
   NativeIoState::InspectingRead{file,page}=>{
    match file.metadata(){
     Ok(metadata)=>{let length=metadata.len();self.known_file_length=Some(length);match page{
      Some((offset,maximum))if offset<=length=>NativeIoState::SeekingRead{file,offset,length,maximum},
      Some(_)=>{self.result=Some(Err("native I/O page offset exceeds file length".into()));self.retired_file=Some(file);NativeIoState::Finished},
      None=>{if let Err(refusal)=validate_original_stream_extent(length,cx){self.result=Some(Ok(NativeIoValue::StreamRefused(refusal)));self.retired_file=Some(file);NativeIoState::Finished}else{NativeIoState::Reading{file,writer:native_payload::NativePayloadPump::new(semio_framework_job_payload::JobPayloadStream::CommitOutput)}}}
     }},
     Err(error)=>{self.result=Some(Err(error.to_string()));self.retired_file=Some(file);NativeIoState::Finished}
    }
   },
   NativeIoState::SeekingRead{mut file,offset,length,maximum}=>{match file.seek(std::io::SeekFrom::Start(offset)){Ok(_)=>NativeIoState::ReadingPage{file,cursor:offset,length,remaining:maximum,writer:native_payload::NativePayloadPump::new(semio_framework_job_payload::JobPayloadStream::CommitOutput)},Err(error)=>{self.result=Some(Err(error.to_string()));self.retired_file=Some(file);NativeIoState::Finished}}},
   _=>return Err(ValueError::invariant("native I/O state invariant violated").into())
  };
  semio_framework_job::receive_original_retained_work(cx,control())?;Ok(())
 }
 pub(super) fn step_payload_read(&mut self,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
  cx.original_source_parent()?.validate_borrowed_context(cx)?;
  if cx.should_yield()||cx.is_cancelled(){return Ok(())}
  let g=cx.retained_grant();
  if matches!(self.state,NativeIoState::InspectingRead{..}|NativeIoState::SeekingRead{..}){return self.observe_original_file(cx)}
  if matches!(self.state,NativeIoState::FinishingRead{..}){
   if self.pending_payload.is_none(){let NativeIoState::FinishingRead{writer,..}=&mut self.state else{return Err(ValueError::invariant("native I/O state invariant violated").into())};let turn=writer.finish_next(&mut self.pending_payload,cx);semio_framework_job::receive_original_retained_work(cx,turn)?;return Ok(())}
   if self.read_buffer.is_some(){let d=NativeReadBuffer::release_demand(&self.read_buffer);if !permits(g,d){return Ok(())}let turn=NativeReadBuffer::release(&mut self.read_buffer,g);semio_framework_job::receive_original_retained_work(cx,turn)?;return Ok(())}
   if !permits(g,fixed()){return Ok(())}
   let NativeIoState::FinishingRead{eof,..}=std::mem::replace(&mut self.state,NativeIoState::Finished)else{return Err(ValueError::invariant("state invariant violated").into())};let bytes=self.pending_payload.take().ok_or_else(||ValueError::invariant("original finished payload slot"))?;self.result=Some(Ok(match eof{Some(eof)=>NativeIoValue::Page{bytes,eof},None=>NativeIoValue::Bytes(bytes)}));semio_framework_job::receive_original_retained_work(cx,control())?;return Ok(())
  }
  if matches!(self.state,NativeIoState::ClosingReadFile{..}){
   if !permits(g,fixed()){return Ok(())}
   let NativeIoState::ClosingReadFile{file,writer,eof}=std::mem::replace(&mut self.state,NativeIoState::Finished)else{return Err(ValueError::invariant("native I/O state invariant violated").into())};drop(file);self.state=NativeIoState::FinishingRead{writer,eof};semio_framework_job::receive_original_retained_work(cx,control())?;return Ok(())
  }
  if matches!(self.state,NativeIoState::ReadingBuffered{..}|NativeIoState::ReadingPageBuffered{..}){
   let (writer,length)=match &mut self.state{NativeIoState::ReadingBuffered{writer,length,..}=>(writer,*length),NativeIoState::ReadingPageBuffered{writer,buffered,..}=>(writer,*buffered),_=>return Err(ValueError::invariant("native I/O state invariant violated").into())};
   if writer.chunk_complete(){
    let turn=writer.clear_chunk(g);if matches!(&turn.result,Ok(RetainedCloneStep::Complete(_))){let state=std::mem::replace(&mut self.state,NativeIoState::Finished);self.state=match state{NativeIoState::ReadingBuffered{file,writer,..}=>NativeIoState::Reading{file,writer},NativeIoState::ReadingPageBuffered{file,writer,cursor,length,remaining,buffered}=>NativeIoState::ReadingPage{file,writer,cursor:cursor+buffered as u64,length,remaining:remaining-buffered},_=>return Err(ValueError::invariant("native I/O state invariant violated").into())};}
    semio_framework_job::receive_original_retained_work(cx,turn)?;return Ok(())
   }
   let bytes=self.read_buffer.as_ref().ok_or_else(||ValueError::invariant("original buffered read backing"))?.initialized_prefix(length)?;let turn=writer.append_next(bytes,cx);semio_framework_job::receive_original_retained_work(cx,turn)?;return Ok(())
  }
  let (maximum,eof)=match &self.state{NativeIoState::Reading{..}=>(self.known_file_length.unwrap_or(0).saturating_sub(self.read_total as u64).max(1).min(semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES as u64)as usize,None),NativeIoState::ReadingPage{cursor,length,remaining,..}=>((*remaining).min(usize::try_from(length.saturating_sub(*cursor)).unwrap_or(usize::MAX)),Some(*cursor>=*length)),_=>return Err(ValueError::invariant("native I/O state invariant violated").into())};
  if maximum==0||eof==Some(true){if !permits(g,fixed()){return Ok(())}let NativeIoState::ReadingPage{file,writer,..}=std::mem::replace(&mut self.state,NativeIoState::Finished)else{return Err(ValueError::invariant("native I/O state invariant violated").into())};self.state=NativeIoState::ClosingReadFile{file,writer,eof};semio_framework_job::receive_original_retained_work(cx,control())?;return Ok(())}
  if self.read_buffer.is_none(){let d=NativeReadBuffer::birth_demand();if !permits(g,d){return Ok(())}let turn=NativeReadBuffer::admit(&mut self.read_buffer,g);semio_framework_job::receive_original_retained_work(cx,turn)?;return Ok(())}
  if g.maximum_copy_bytes==0||!permits(g,fixed()){return Ok(())}
  let readable=maximum.min(semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES).min(g.maximum_copy_bytes);
  let buffer=self.read_buffer.as_mut().ok_or_else(||ValueError::invariant("read_buffer absent"))?;
  if buffer.initialized_len()<readable{let turn=buffer.initialize_prefix(readable,g.maximum_copy_bytes,g);semio_framework_job::receive_original_retained_work(cx,turn)?;return Ok(())}
  let bytes=buffer.initialized_prefix_mut(readable)?;
  let result=match &mut self.state{NativeIoState::Reading{file,..}|NativeIoState::ReadingPage{file,..}=>file.read(bytes),_=>return Err(ValueError::invariant("native I/O state invariant violated").into())};
  let count=result.as_ref().copied().unwrap_or(0);if matches!(self.state,NativeIoState::Reading{..}){self.read_total+=count}let state=std::mem::replace(&mut self.state,NativeIoState::Finished);
  self.state=match(state,result){
   (NativeIoState::Reading{file,writer},Ok(0))=>NativeIoState::ClosingReadFile{file,writer,eof:None},
   (NativeIoState::Reading{file,writer},Ok(length))=>NativeIoState::ReadingBuffered{file,writer,length},
   (NativeIoState::ReadingPage{file,writer,..},Ok(0))=>NativeIoState::ClosingReadFile{file,writer,eof:Some(true)},
   (NativeIoState::ReadingPage{file,writer,cursor,length,remaining},Ok(buffered))=>NativeIoState::ReadingPageBuffered{file,writer,cursor,length,remaining,buffered},
   (NativeIoState::Reading{file,writer},Err(error))|(NativeIoState::ReadingPage{file,writer,..},Err(error))=>{self.retired_file=Some(file);NativeIoState::ClosingWriterFault{writer,error:error.to_string()}},
   _=>return Err(ValueError::invariant("native I/O state invariant violated").into())
  };
  semio_framework_job::receive_original_retained_work::<ValueError>(cx,RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:count,..Default::default()})),work:1})?;Ok(())
 }
}
