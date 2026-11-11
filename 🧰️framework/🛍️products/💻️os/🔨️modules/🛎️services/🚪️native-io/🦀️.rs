//! 📂️ Native filesystem and process-observation jobs for interactive OS hosts.

use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, InteractiveJobStep, OriginalJobReturnClaim, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, RetainedCloneGrant, RetainedCloneProgress, StepContext, OriginalTurnFailure};
use semio_framework_value_retained_clone::{RetainedAllocationEffect, RetainedWorkPoll, RetirementDemand, RetainedWorkOutcome, RetainedCloneStep, ValueError, ValueRefusalKind};
use std::fs::{File, ReadDir};
use std::io::{Read, Seek};
use std::path::PathBuf;
#[path="📦️payload/🦀️.rs"]
mod native_payload;
#[path="📖️buffer/🦀️.rs"]
mod native_read_buffer;
#[path="📖️buffer/read.rs"]
mod native_read;

#[path="📮️path-return/🦀️.rs"]
mod original_path_return;
pub use original_path_return::{OriginalNativePathReturnRegistry,OriginalNativePathReservation,OriginalSuppliedNativePath};


#[cfg(test)]
#[path = "../🧪️tests/🔬️native-io-unit"]
mod tests {
    use super::*;
    use semio_framework_job_worker::OriginalWorkerFixtureCaller;

    #[path = "🦀️.rs"]
    mod unit;
}

fn extension_capacity(extension:&std::borrow::Cow<'static,str>)->usize{match extension{std::borrow::Cow::Owned(text)=>text.capacity(),std::borrow::Cow::Borrowed(_)=>0}}

//#region 📂️Schema

pub use semio_os_services_native_paths::{NativePathSet,NativeModifiedSet,NativePathInput,NATIVE_IO_PATH_CAPACITY};
#[path="📋️rows/advance.rs"]
mod native_row_work;

#[derive(Debug)]
#[expect(clippy::large_enum_variant, reason = "Requests retain all 256 fixed path slots without allocating an uncredited wrapper on submission or rejection.")]
pub enum NativeIoRequest {
    ReadBytes(PathBuf),
    ReadPage { path: PathBuf, offset: u64, max_bytes: usize },
    ScanDirectory { path: PathBuf, directories_only: bool, extension: Option<std::borrow::Cow<'static,str>>, first_only: bool },
    Modified(NativePathInput),
    ProcessResidentBytes,
}

#[derive(Debug)]
#[expect(clippy::large_enum_variant, reason = "Completions transfer fixed path slots into one-entry retirement without allocating another result owner.")]
pub enum NativeIoValue {
    Bytes(semio_framework_job_payload::RetainedJobPayload),
    Page { bytes: semio_framework_job_payload::RetainedJobPayload, eof: bool },
    Paths(NativePathSet),
    Modified(NativeModifiedSet),
    ResidentBytes(Option<u64>),
    StreamRefused(NativeIoStreamRefusal),
}

/// 📏️ The retained scalar refusal precedes page birth when the original stream cannot finish within its stock.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct NativeIoStreamRefusal{pub length:u64,pub maximum_bytes:u64}
/// 📋️ A result observation borrows only scalar shape while its producer keeps every physical owner.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum NativeIoResultStatus{Bytes{length:usize,pages:usize},Page{length:usize,pages:usize,eof:bool},Paths{count:usize},Modified{count:usize},ResidentBytes(Option<u64>),StreamRefused(NativeIoStreamRefusal),Fault}
fn progress_poll(turn:RetainedWorkOutcome<InteractiveJobCloseStep>)->RetainedWorkPoll<RetainedCloneStep,ValueError>{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work})}
fn validate_original_stream_extent(length:u64,cx:&StepContext<'_>)->Result<(),NativeIoStreamRefusal>{let stock=cx.original_total_retained_grant();let page=semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;let capacity=stock.maximum_capacity_bytes.min(stock.maximum_release_bytes).min(semio_framework_job_payload::JOB_PAYLOAD_OPERATION_BYTES);let maximum_bytes=((capacity/page)*page)as u64;if length>maximum_bytes{Err(NativeIoStreamRefusal{length,maximum_bytes})}else{Ok(())}}

impl NativeIoValue {
    pub fn retirement_demands(&self) -> Result<RetirementDemand, ValueError> {
        match self {
            Self::Bytes(bytes) | Self::Page { bytes, .. } if !bytes.terminal_is_empty() => native_io_child_demands(bytes.retirement_demands()?),
            Self::Paths(paths) if !paths.is_empty() => native_io_child_demands(paths.close_demand()),
            Self::Modified(entries) if !entries.is_empty() => native_io_child_demands(entries.close_demand()),
            _ => Ok(Default::default()),
        }
    }

    pub fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<InteractiveJobCloseStep>{
        let grant=effect.grant();let mut work=0;let result=(||{
         if self.terminal_is_empty(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}
         let demand=self.retirement_demands()?;
         if grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
         let progress=match self{
          Self::Bytes(bytes)|Self::Page{bytes,..}=>{let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};let turn=bytes.close_step(effect);work=turn.work;let step=turn.result?;semio_framework_value_retained_clone::retained_clone::admit_retained_clone_close(child,step,bytes.terminal_is_empty(),"original native I/O result payload").map_err(|error|error.with_retained_progress(step.progress()))?;step.progress()},
          Self::Paths(paths)=>{let turn=paths.close_step(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant});work=turn.work;turn.result?.progress()},
          Self::Modified(entries)=>{let turn=entries.close_step(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant});work=turn.work;turn.result?.progress()},
          Self::ResidentBytes(_)|Self::StreamRefused(_)=>Default::default(),
         };InteractiveJobCloseStep::Pending{progress}.admit(grant,self.terminal_is_empty())
        })();RetainedWorkOutcome{result,work}
    }

    pub fn terminal_is_empty(&self) -> bool {
        match self {
            Self::Bytes(bytes) | Self::Page { bytes, .. } => bytes.terminal_is_empty(),
            Self::Paths(paths) => paths.is_empty(),
            Self::Modified(entries) => entries.is_empty(),
            Self::ResidentBytes(_)|Self::StreamRefused(_) => true,
        }
    }
}

//#endregion 📂️Schema

fn native_io_child_demands(mut demand: RetirementDemand) -> Result<RetirementDemand, ValueError> {
    demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "original native I/O child depth overflow"))?;
    Ok(demand)
}

//#region 👷️Job

#[doc(hidden)] pub enum NativeIoState {
    Pending(NativeIoRequest),
    InspectingRead { file: File, page: Option<(u64,usize)> },
    SeekingRead { file: File, offset: u64, length: u64, maximum: usize },
    Reading { file: File, writer: native_payload::NativePayloadPump },
    ReadingBuffered { file: File, writer: native_payload::NativePayloadPump, length: usize },
    ReadingPage { file: File, cursor: u64, length: u64, remaining: usize, writer: native_payload::NativePayloadPump },
    ReadingPageBuffered { file: File, cursor: u64, length: u64, remaining: usize, writer: native_payload::NativePayloadPump, buffered: usize },
    ClosingReadFile { file: File, writer: native_payload::NativePayloadPump, eof: Option<bool> },
    FinishingRead { writer: native_payload::NativePayloadPump, eof: Option<bool> },
    ClosingWriterFault { writer: native_payload::NativePayloadPump, error: String },
    Scanning { entries: ReadDir, paths: NativePathSet, directories_only: bool, first_only: bool },
    StagingScan { entries: ReadDir, paths: NativePathSet, directories_only: bool, first_only: bool },
    ClosingScanFault { paths: NativePathSet, rejected: Option<PathBuf>, error: String },
    ReadingModified { paths: NativePathSet, modified: NativeModifiedSet },
    ClosingModifiedFault { paths: NativePathSet, modified: NativeModifiedSet, rejected: Option<(PathBuf, std::time::SystemTime)>, error: String },
    Finished,
}

#[derive(Clone, Copy)]
enum NativeIoCloseTarget { RequestPath, RequestExtension, RetiredPath, ScannedPath, ModifiedPath, ModifiedEntry, RetiredDirectory, FaultWriter, ClearFaultWriter, FaultDetail, ClearFaultDetail, ReadBuffer, RetiredFile, PendingPayload, ClearPendingPayload, Writer, Paths, Modified, Path, Extension, RejectedPath, RejectedModified, Error, FinishState, ResultValue, ResultError, ClearResult, Complete }

pub struct NativeIoJob {
    #[doc(hidden)] pub state: NativeIoState,
    known_file_length: Option<u64>,
    request_path: Option<PathBuf>,
    request_extension: Option<std::borrow::Cow<'static,str>>,
    retired_path: Option<PathBuf>,
    scanned_path: Option<PathBuf>,
    modified_path: Option<PathBuf>,
    modified_entry: Option<(PathBuf,std::time::SystemTime)>,
    retired_directory: Option<ReadDir>,
    #[doc(hidden)] pub result: Option<Result<NativeIoValue, String>>,
    closing: bool,
    fault_writer: Option<native_payload::NativePayloadPump>,
    read_buffer: Option<Box<native_read_buffer::NativeReadBuffer>>,
    pending_payload: Option<semio_framework_job_payload::RetainedJobPayload>,
    read_total: usize,
    retired_file: Option<File>,
    fault_complete: bool,
    fault_detail: Option<semio_framework_job_payload::RetainedJobPayload>,
}

impl NativeIoJob {
    pub fn new(request: NativeIoRequest) -> Self {
        Self { state: NativeIoState::Pending(request), known_file_length: None, request_path: None, request_extension: None, retired_path: None, scanned_path: None, modified_path: None, modified_entry: None, retired_directory: None, result: None, closing: false, fault_writer: None, read_buffer: None, pending_payload: None, read_total: 0, retired_file: None, fault_complete: false, fault_detail: None }
    }

    fn close_target(&self) -> Result<(NativeIoCloseTarget, RetirementDemand), ValueError> {
        use NativeIoCloseTarget::*;
        if let Some(writer)=self.fault_writer.as_ref(){return Ok(if writer.terminal_is_empty(){(ClearFaultWriter,RetirementDemand{copy_bytes:size_of_val(&self.fault_writer),depth:1,..Default::default()})}else{(FaultWriter,native_io_child_demands(writer.retirement_demands()?)?)});}
        if let Some(detail)=self.fault_detail.as_ref(){return Ok(if detail.terminal_is_empty(){(ClearFaultDetail,RetirementDemand{copy_bytes:size_of_val(&self.fault_detail),depth:1,..Default::default()})}else{(FaultDetail,native_io_child_demands(detail.retirement_demands()?)?)});}
        let release = |target, bytes, depth| Ok((target, RetirementDemand { release_bytes: bytes, depth, ..Default::default() }));
        if let Some(path) = self.request_path.as_ref() { return release(RequestPath, path.capacity(), 1); }
        if let Some(extension) = self.request_extension.as_ref() { return release(RequestExtension, extension_capacity(extension), 1); }
        if let Some(path) = self.retired_path.as_ref() { return release(RetiredPath, path.capacity(), 1); }
        if let Some(path)=self.scanned_path.as_ref(){return release(ScannedPath,path.capacity(),1)}
        if let Some(path)=self.modified_path.as_ref(){return release(ModifiedPath,path.capacity(),1)}
        if let Some((path,_))=self.modified_entry.as_ref(){return release(ModifiedEntry,path.capacity(),1)}
        if self.retired_directory.is_some(){return release(RetiredDirectory,0,1)}
        if let Some(payload)=self.pending_payload.as_ref(){return Ok(if payload.terminal_is_empty(){(ClearPendingPayload,RetirementDemand{depth:1,..Default::default()})}else{(PendingPayload,native_io_child_demands(payload.retirement_demands()?)?)})}
        if let NativeIoState::Reading{writer,..}|NativeIoState::ReadingBuffered{writer,..}|NativeIoState::ReadingPage{writer,..}|NativeIoState::ReadingPageBuffered{writer,..}|NativeIoState::ClosingReadFile{writer,..}|NativeIoState::FinishingRead{writer,..}|NativeIoState::ClosingWriterFault{writer,..}=&self.state{if !writer.terminal_is_empty(){return Ok((Writer,native_io_child_demands(writer.retirement_demands()?)?))}}
        if self.read_buffer.is_some(){return Ok((ReadBuffer,native_read_buffer::NativeReadBuffer::release_demand(&self.read_buffer)))}
        if self.retired_file.is_some(){return Ok((RetiredFile,RetirementDemand{depth:1,..Default::default()}))}
        match &self.state {
            NativeIoState::ClosingScanFault { rejected: Some(path), .. } => return release(RejectedPath,path.capacity(),1),
            NativeIoState::ClosingModifiedFault { rejected: Some((path, _)), .. } => return release(RejectedModified,path.capacity(),1),
            NativeIoState::Pending(NativeIoRequest::Modified(NativePathInput::Rows(owner))) | NativeIoState::Scanning { paths: owner, .. } | NativeIoState::StagingScan{paths:owner,..} | NativeIoState::ClosingScanFault { paths: owner, .. } | NativeIoState::ReadingModified { paths: owner, .. } | NativeIoState::ClosingModifiedFault { paths: owner, .. } if !owner.is_empty() => return Ok((Paths,native_io_child_demands(owner.close_demand())?)),
            NativeIoState::ReadingModified { modified: owner, .. } | NativeIoState::ClosingModifiedFault { modified: owner, .. } if !owner.is_empty() => return Ok((Modified,native_io_child_demands(owner.close_demand())?)),
            NativeIoState::Pending(NativeIoRequest::ScanDirectory { extension: Some(extension), .. }) => return release(Extension,extension_capacity(extension),1),
            NativeIoState::Pending(NativeIoRequest::ReadBytes(path) | NativeIoRequest::ReadPage { path, .. } | NativeIoRequest::ScanDirectory { path, .. } | NativeIoRequest::Modified(NativePathInput::Single(path))) if path.capacity() != 0 => return release(Path,path.capacity(),1),
            NativeIoState::ClosingWriterFault { error, .. } | NativeIoState::ClosingScanFault { error, .. } | NativeIoState::ClosingModifiedFault { error, .. } if error.capacity() != 0 => return release(Error,error.capacity(),1),
            NativeIoState::Finished => {},
            _ => return release(FinishState,0,1),
        }
        match &self.result {
            Some(Ok(value)) if !value.terminal_is_empty() => Ok((ResultValue,native_io_child_demands(value.retirement_demands()?)?)),
            Some(Err(error)) if error.capacity() != 0 => release(ResultError,error.capacity(),1),
            Some(_) => release(ClearResult,0,1),
            None => release(Complete,0,0),
        }
    }

    pub fn retained_request_backing_identity(&self) -> Option<*const u8> {
        if let Some(path) = self.request_path.as_ref() { return Some(path.as_os_str().as_encoded_bytes().as_ptr()); }
        let path = match &self.state {
            NativeIoState::Pending(NativeIoRequest::ReadBytes(path) | NativeIoRequest::ReadPage { path, .. } | NativeIoRequest::ScanDirectory { path, .. } | NativeIoRequest::Modified(NativePathInput::Single(path))) => Some(path),
            NativeIoState::Pending(NativeIoRequest::Modified(NativePathInput::Rows(paths))) => paths.as_slice().first(),
            _ => None,
        }?;
        Some(path.as_os_str().as_encoded_bytes().as_ptr())
    }

    pub fn take_result(&mut self) -> Option<Result<NativeIoValue, String>> {
        self.result.take()
    }

    /// 👁️ Observes retained output without moving paths, payload headers or error text out of their source.
    pub fn result_status(&self)->Option<NativeIoResultStatus>{self.result.as_ref().map(|result|match result{Ok(NativeIoValue::Bytes(bytes))=>NativeIoResultStatus::Bytes{length:bytes.len(),pages:bytes.page_count()},Ok(NativeIoValue::Page{bytes,eof})=>NativeIoResultStatus::Page{length:bytes.len(),pages:bytes.page_count(),eof:*eof},Ok(NativeIoValue::Paths(paths))=>NativeIoResultStatus::Paths{count:paths.len()},Ok(NativeIoValue::Modified(paths))=>NativeIoResultStatus::Modified{count:paths.len()},Ok(NativeIoValue::ResidentBytes(bytes))=>NativeIoResultStatus::ResidentBytes(*bytes),Ok(NativeIoValue::StreamRefused(refusal))=>NativeIoResultStatus::StreamRefused(*refusal),Err(_)=>NativeIoResultStatus::Fault})}

    /// 📐️ The actual opened file supplies this scalar extent; observing it transfers no payload or path owner.
    pub fn known_file_length(&self)->Option<u64>{self.known_file_length}
    /// 📖️ A completed producer keeps its original page backing while a caller borrows bytes for controlled copying.
    pub fn result_page(&self,index:usize)->Option<&[u8]>{match self.result.as_ref(){Some(Ok(NativeIoValue::Bytes(bytes)|NativeIoValue::Page{bytes,..}))=>bytes.page(index),_=>None}}

    /// 🕰️ The retained result lends one original path and its scalar observation without moving either owner.
    pub fn result_modified(&self,index:usize)->Option<(&std::path::Path,std::time::SystemTime)>{match self.result.as_ref(){Some(Ok(NativeIoValue::Modified(entries)))if index<entries.len()=>entries.as_slice().get(index).map(|(path,time)|(path.as_path(),*time)),_=>None}}
    /// 🧭️ Only this actual producer can supply an original selected directory path to its pre-reserved supplier return.
    pub fn supply_result_last_path<const N:usize>(&mut self,reservation:&mut OriginalNativePathReservation<N>,output:&mut Option<OriginalSuppliedNativePath>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{match self.result.as_mut(){Some(Ok(NativeIoValue::Paths(paths)))=>reservation.publish_last(paths,output,cx),_=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native directory result retains no original selected path").into())}}

    /// 📮️ After semantic acknowledgement, one page moves directly to its pre-reserved original supplier return.
    pub fn supply_result_last_page<const N:usize>(&mut self,reservation:&mut semio_framework_job::OriginalJobPageReservation<N>,output:&mut Option<semio_framework_job::OriginalSuppliedJobPage>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{match self.result.as_mut(){Some(Ok(NativeIoValue::Bytes(bytes)|NativeIoValue::Page{bytes,..}))=>reservation.publish_last(bytes,output,cx),_=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native I/O result has no original payload page to supply").into())}}

    /// ♻️ Output retirement leaves the original request path available for the next bounded page of this invocation.
    pub fn close_result_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        let Some(result)=self.result.as_mut()else{return Ok(true)};
        match result{
         Ok(value)if !value.terminal_is_empty()=>{let demand=value.retirement_demands()?;semio_framework_job::close_original_effect_step(cx,demand,|effect|progress_poll(value.close_step(effect)))?;return Ok(false)},
         Err(error)if error.capacity()!=0=>{let demand=RetirementDemand{release_bytes:error.capacity(),depth:1,..Default::default()};semio_framework_job::close_original_retained_step(cx,demand,|_|{drop(std::mem::take(error));RetainedWorkOutcome::<RetainedCloneStep,ValueError>{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()})),work:1}})?;return Ok(false)},
         _=>{}
        }
        semio_framework_job::close_original_retained_step(cx,RetirementDemand{depth:1,..Default::default()},|_|{self.result=None;RetainedWorkOutcome::<RetainedCloneStep,ValueError>{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}})?;Ok(self.result.is_none())
    }

    /// 🔁️ A fully acknowledged and retired page reuses its exact path under the same incoming caller.
    pub fn restart_read_page(&mut self,offset:u64,max_bytes:usize,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        if self.closing||!matches!(self.state,NativeIoState::Finished)||self.result.is_some()||self.fault_writer.is_some()||self.fault_detail.is_some()||self.pending_payload.is_some()||self.read_buffer.is_some()||self.retired_file.is_some()||self.request_path.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native page restart requires its retained path and retired original result").into())}
        if max_bytes==0||max_bytes>semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native page restart exceeds the actual one-page bound").into())}
        let copy_bytes=size_of::<Option<PathBuf>>()+size_of::<u64>()+size_of::<usize>()+2*size_of::<usize>();let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_copy_bytes<copy_bytes||grant.maximum_depth==0{return Ok(false)}
        cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:copy_bytes,..Default::default()})?;let path=self.request_path.take().unwrap();self.read_total=0;self.state=NativeIoState::Pending(NativeIoRequest::ReadPage{path,offset,max_bytes});cx.consume_fuel(1);Ok(true)
    }

    fn finish<'a>(&'a mut self, result: Result<NativeIoValue, String>, _cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, OriginalTurnFailure> {
        self.result=Some(result);self.state=NativeIoState::Finished;Ok(None)
    }

    fn publish_terminal<'a>(&'a mut self,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure>{
        match self.result.as_ref(){
         Some(Ok(value))=>JobOutcomeBorrow::admit_complete(cx,None,match value{NativeIoValue::Bytes(bytes)|NativeIoValue::Page{bytes,..}=>Some(bytes),_=>None}).map_err(Into::into),
         Some(Err(error))=>{
          if self.fault_detail.is_none(){
           if self.fault_writer.is_none(){
            let demand=RetirementDemand{depth:1,..Default::default()};let g=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||g.maximum_items==0||g.maximum_depth<demand.depth{return Ok(None)}
            self.fault_writer=Some(native_payload::NativePayloadPump::new(semio_framework_job_payload::JobPayloadStream::Fault));semio_framework_job::receive_original_retained_work::<ValueError>(cx,RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1})?;return Ok(None)
           }
           let writer=self.fault_writer.as_mut().unwrap();
           let turn=if !self.fault_complete{if writer.chunk_complete(){let turn=writer.clear_chunk(cx.retained_grant());if matches!(&turn.result,Ok(RetainedCloneStep::Complete(_))){self.fault_complete=true}turn}else{writer.append_next(error.as_bytes(),cx)}}else{writer.finish_next(&mut self.fault_detail,cx)};
           semio_framework_job::receive_original_retained_work(cx,turn)?;return Ok(None)
          }
          JobOutcomeBorrow::admit_fault(cx,self.fault_detail.as_ref().unwrap()).map_err(Into::into)
         },
         None=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original native I/O terminal result absent").into())
        }
    }

    fn start<'a>(&'a mut self, request: NativeIoRequest, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, OriginalTurnFailure> {
        let result=match request {
            NativeIoRequest::ReadBytes(path)=>return self.open_original_read(path,None,cx),
            NativeIoRequest::ReadPage{path,offset,max_bytes}=>return self.open_original_read(path,Some((offset,max_bytes)),cx),
            NativeIoRequest::ScanDirectory { path, directories_only, extension, first_only } => {
                self.request_path = Some(path);
                self.request_extension = extension;
                let path = self.request_path.as_ref().expect("retained original native scan path");
                match std::fs::read_dir(path) {
                Ok(entries) => {
                    self.state = NativeIoState::Scanning { entries, paths: NativePathSet::new(), directories_only, first_only };
                    Ok(None)
                }
                Err(error) => self.finish(Err(format!("{}: {error}", path.display())), cx),
                }
            },
            NativeIoRequest::Modified(input) => {
                let paths=match input{NativePathInput::Single(path)=>{self.modified_path=Some(path);NativePathSet::new()},NativePathInput::Rows(paths)=>paths};
                self.state = NativeIoState::ReadingModified { paths, modified: NativeModifiedSet::new() };
                Ok(None)
            }
            NativeIoRequest::ProcessResidentBytes => self.finish(Ok(NativeIoValue::ResidentBytes(process_resident_bytes())), cx),
        };
        semio_framework_job::receive_original_retained_work::<ValueError>(cx,RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1})?;result
    }
}

impl NativeIoJob{
    fn step_owned_raw<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, OriginalTurnFailure> {
        if cx.is_cancelled(){self.closing=true;return JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into)}
        if cx.should_yield() {
            return Ok(None);
        }
        cx.set_stage("NativePlatformIo");
        let g=cx.retained_grant();if g.maximum_items==0||g.maximum_depth==0{return Ok(None)}
        if matches!(&self.state,NativeIoState::InspectingRead{..}|NativeIoState::SeekingRead{..}|NativeIoState::Reading{..}|NativeIoState::ReadingBuffered{..}|NativeIoState::ReadingPage{..}|NativeIoState::ReadingPageBuffered{..}|NativeIoState::ClosingReadFile{..}|NativeIoState::FinishingRead{..}){self.step_payload_read(cx)?;return Ok(None)}
        if matches!(&self.state,NativeIoState::StagingScan{..}|NativeIoState::ReadingModified{..}){self.step_native_rows(cx)?;return Ok(None)}
        if matches!(&self.state, NativeIoState::Scanning { .. }) { cx.consume_fuel(1); }
        match std::mem::replace(&mut self.state, NativeIoState::Finished) {
            NativeIoState::Pending(request) => self.start(request, cx),
            NativeIoState::InspectingRead{..}|NativeIoState::SeekingRead{..}|NativeIoState::Reading{..}|NativeIoState::ReadingBuffered{..}|NativeIoState::ReadingPage{..}|NativeIoState::ReadingPageBuffered{..}|NativeIoState::ClosingReadFile{..}|NativeIoState::FinishingRead{..}=>unreachable!("native read states use their retained current-turn driver"),
            NativeIoState::Scanning { mut entries, mut paths, directories_only, first_only } => {
                let Some(entry) = entries.next() else { self.retired_directory=Some(entries);return self.finish(Ok(NativeIoValue::Paths(paths)), cx) };
                let Ok(entry) = entry else {
                    self.state = NativeIoState::Scanning { entries, paths, directories_only, first_only };
                    return Ok(None);
                };
                let path = entry.path();
                if (directories_only && !path.is_dir()) || self.request_extension.as_ref().is_some_and(|extension| path.extension().and_then(|value| value.to_str()) != Some(extension.as_ref())) {
                    self.retired_path = Some(path);
                    self.state = NativeIoState::Scanning { entries, paths, directories_only, first_only };
                    return Ok(None);
                }
                self.scanned_path=Some(path);self.state=NativeIoState::StagingScan{entries,paths,directories_only,first_only};Ok(None)
            }
            NativeIoState::StagingScan{..}|NativeIoState::ReadingModified{..}=>unreachable!("native row ownership uses its actual granted driver"),
            NativeIoState::ClosingWriterFault { .. } | NativeIoState::ClosingScanFault { .. } | NativeIoState::ClosingModifiedFault { .. } => unreachable!("original fault residuals are retained by the funded driver"),
            NativeIoState::Finished => self.publish_terminal(cx),
        }
    }

}

impl NativeIoJob {
    fn step_owned<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, OriginalTurnFailure> {
        if let Some(path) = self.retired_path.as_ref() {
            let demand = RetirementDemand { release_bytes: path.capacity(), depth: 1, ..Default::default() };
            semio_framework_job::close_original_retained_step(cx, demand, |_| { drop(self.retired_path.take()); RetainedWorkOutcome::<RetainedCloneStep,ValueError>{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() })),work:1} })?;
            return Ok(None);
        }
        if self.retired_file.is_some(){semio_framework_job::close_original_retained_step(cx,RetirementDemand{depth:1,..Default::default()},|_|{drop(self.retired_file.take());RetainedWorkOutcome::<RetainedCloneStep,ValueError>{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}})?;return Ok(None)}
        if self.retired_directory.is_some(){semio_framework_job::close_original_retained_step(cx,RetirementDemand{depth:1,..Default::default()},|_|{drop(self.retired_directory.take());RetainedWorkOutcome::<RetainedCloneStep,ValueError>{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}})?;return Ok(None)}
        if matches!(&self.state, NativeIoState::ClosingWriterFault { .. } | NativeIoState::ClosingScanFault { .. } | NativeIoState::ClosingModifiedFault { .. }) {
            let (target, demand) = self.close_target()?;
            if matches!(target, NativeIoCloseTarget::RequestPath | NativeIoCloseTarget::RequestExtension | NativeIoCloseTarget::RetiredPath | NativeIoCloseTarget::ScannedPath | NativeIoCloseTarget::ModifiedPath | NativeIoCloseTarget::ModifiedEntry | NativeIoCloseTarget::RetiredDirectory | NativeIoCloseTarget::ReadBuffer | NativeIoCloseTarget::RetiredFile | NativeIoCloseTarget::PendingPayload | NativeIoCloseTarget::ClearPendingPayload | NativeIoCloseTarget::Writer | NativeIoCloseTarget::Paths | NativeIoCloseTarget::Modified | NativeIoCloseTarget::Extension | NativeIoCloseTarget::RejectedPath | NativeIoCloseTarget::RejectedModified) {
                let was_closing = self.closing;
                let result = semio_framework_job::close_original_effect_step(cx, demand, |effect| {
                    let turn = self.close_granted(effect);
                    self.closing = was_closing;
                    progress_poll(turn)
                });
                result?;
                return Ok(None);
            }
            if cx.retirement_should_yield() || cx.retained_grant().maximum_items == 0 || cx.retained_grant().maximum_depth == 0 { return Ok(None); }
            let original = std::mem::replace(&mut self.state, NativeIoState::Finished);
            let error = match original { NativeIoState::ClosingWriterFault { error, .. } | NativeIoState::ClosingScanFault { error, .. } | NativeIoState::ClosingModifiedFault { error, .. } => error, _ => unreachable!() };
            self.result = Some(Err(error));
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
            cx.consume_fuel(1);
            return Ok(None);
        }
        self.step_owned_raw(cx)
    }
}

impl NativeIoJob{
    fn close_granted(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> RetainedWorkOutcome<InteractiveJobCloseStep> {
        let grant=effect.grant();let mut work=0;let result=(||{
        let (target,demand)=self.close_target()?;
        let release = demand.release_bytes;
        if matches!(target, NativeIoCloseTarget::Complete) {
            return InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() }.admit(grant, self.terminal_is_empty());
        }
        if grant.maximum_items == 0 { return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()}); }
        if grant.maximum_depth < demand.depth { return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()}); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < release { return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()}); }
        self.closing = true;
        match target {
            NativeIoCloseTarget::RequestPath => drop(self.request_path.take()),
            NativeIoCloseTarget::RequestExtension => drop(self.request_extension.take()),
            NativeIoCloseTarget::RetiredPath => drop(self.retired_path.take()),
            NativeIoCloseTarget::ScannedPath=>drop(self.scanned_path.take()),
            NativeIoCloseTarget::ModifiedPath=>drop(self.modified_path.take()),
            NativeIoCloseTarget::ModifiedEntry=>drop(self.modified_entry.take()),
            NativeIoCloseTarget::RetiredDirectory=>drop(self.retired_directory.take()),
            NativeIoCloseTarget::FaultWriter=>{let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};let turn=self.fault_writer.as_mut().unwrap().close_step(effect);work=turn.work;return turn.result.and_then(|step|InteractiveJobCloseStep::Pending{progress:step.progress()}.admit(grant,false));},
            NativeIoCloseTarget::FaultDetail=>{let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};let turn=self.fault_detail.as_mut().unwrap().close_step(effect);work=turn.work;return turn.result.and_then(|step|InteractiveJobCloseStep::Pending{progress:step.progress()}.admit(grant,false));},
            NativeIoCloseTarget::PendingPayload=>{let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};let turn=self.pending_payload.as_mut().unwrap().close_step(effect);work=turn.work;return turn.result.and_then(|step|InteractiveJobCloseStep::Pending{progress:step.progress()}.admit(grant,false));},
            NativeIoCloseTarget::ReadBuffer=>{let turn=native_read_buffer::NativeReadBuffer::release(&mut self.read_buffer,grant);work=turn.work;return turn.result.and_then(|step|InteractiveJobCloseStep::Pending{progress:step.progress()}.admit(grant,false));},
            NativeIoCloseTarget::RetiredFile=>drop(self.retired_file.take()),
            NativeIoCloseTarget::ClearPendingPayload=>self.pending_payload=None,
            NativeIoCloseTarget::ClearFaultWriter=>self.fault_writer=None,
            NativeIoCloseTarget::ClearFaultDetail=>self.fault_detail=None,
            NativeIoCloseTarget::Writer => {
                let writer = match &mut self.state {
                    NativeIoState::Reading { writer, .. } | NativeIoState::ReadingBuffered { writer, .. } | NativeIoState::ReadingPage { writer, .. } | NativeIoState::ReadingPageBuffered { writer, .. } | NativeIoState::ClosingReadFile{writer,..} | NativeIoState::FinishingRead{writer,..} | NativeIoState::ClosingWriterFault { writer, .. } => writer,
                    _ => return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original native I/O close target changed custody")),
                };
                let child = RetainedCloneGrant { maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant };
                let turn=writer.close_step(effect);work=turn.work;return turn.result.and_then(|step|{semio_framework_value_retained_clone::retained_clone::admit_retained_clone_close(child,step,writer.terminal_is_empty(),"original native I/O payload builder").map_err(|error|error.with_retained_progress(step.progress()))?;InteractiveJobCloseStep::Pending{progress:step.progress()}.admit(grant,false)});

            }
            NativeIoCloseTarget::Paths => match &mut self.state {
                NativeIoState::Pending(NativeIoRequest::Modified(NativePathInput::Rows(paths))) | NativeIoState::Scanning { paths, .. } | NativeIoState::StagingScan{paths,..} | NativeIoState::ClosingScanFault { paths, .. } | NativeIoState::ReadingModified { paths, .. } | NativeIoState::ClosingModifiedFault { paths, .. } => {let turn=paths.close_step(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant});work=turn.work;return turn.result.and_then(|step|InteractiveJobCloseStep::Pending{progress:step.progress()}.admit(grant,false));},
                _ => return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original native I/O close target changed custody")),
            },
            NativeIoCloseTarget::Modified => match &mut self.state {
                NativeIoState::ReadingModified { modified, .. } | NativeIoState::ClosingModifiedFault { modified, .. } => {let turn=modified.close_step(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant});work=turn.work;return turn.result.and_then(|step|InteractiveJobCloseStep::Pending{progress:step.progress()}.admit(grant,false));},
                _ => return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original native I/O close target changed custody")),
            },
            NativeIoCloseTarget::Path => match &mut self.state {
                NativeIoState::Pending(NativeIoRequest::ReadBytes(path) | NativeIoRequest::ReadPage { path, .. } | NativeIoRequest::ScanDirectory { path, .. } | NativeIoRequest::Modified(NativePathInput::Single(path))) => drop(std::mem::take(path)),
                _ => return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original native I/O close target changed custody")),
            },
            NativeIoCloseTarget::Extension => match &mut self.state {
                NativeIoState::Pending(NativeIoRequest::ScanDirectory { extension, .. }) => drop(extension.take()),
                _ => return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original native I/O close target changed custody")),
            },
            NativeIoCloseTarget::RejectedPath => if let NativeIoState::ClosingScanFault { rejected, .. } = &mut self.state { drop(rejected.take()); },
            NativeIoCloseTarget::RejectedModified => if let NativeIoState::ClosingModifiedFault { rejected, .. } = &mut self.state { drop(rejected.take()); },
            NativeIoCloseTarget::Error => match &mut self.state {
                NativeIoState::ClosingWriterFault { error, .. } | NativeIoState::ClosingScanFault { error, .. } | NativeIoState::ClosingModifiedFault { error, .. } => drop(std::mem::take(error)),
                _ => return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original native I/O close target changed custody")),
            },
            NativeIoCloseTarget::FinishState => self.state = NativeIoState::Finished,
            NativeIoCloseTarget::ResultValue => {
                let Some(Ok(value)) = self.result.as_mut() else { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original native I/O close target changed custody")) };
                let turn=value.close_step(effect);work=turn.work;return turn.result.and_then(|step|InteractiveJobCloseStep::Pending{progress:step.progress()}.admit(grant,false));

            }
            NativeIoCloseTarget::ResultError => if let Some(Err(error)) = self.result.as_mut() { drop(std::mem::take(error)); },
            NativeIoCloseTarget::ClearResult => self.result = None,
            NativeIoCloseTarget::Complete => unreachable!(),
        }
        work=1;InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, copied_bytes:demand.copy_bytes, released_bytes: release, ..RetainedCloneProgress::default() } }.admit(grant, self.terminal_is_empty())
        })();RetainedWorkOutcome{result,work}
    }

}

impl InteractiveJob for NativeIoJob {
    fn step<'a>(&'a mut self,cx:&mut StepContext<'_>,_return_claim:&mut OriginalJobReturnClaim<'_>)->InteractiveJobStep<'a>{InteractiveJobStep::from_callback(self.step_owned(cx))}

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        match descriptor.kind(){
            JobOutcomeKind::Yield=>descriptor.yielded(),JobOutcomeKind::Cancelled=>descriptor.cancelled(),
            JobOutcomeKind::Complete=>{let Some(Ok(value))=self.result.as_ref()else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original native I/O completion custody absent"))};descriptor.complete(None,match value{NativeIoValue::Bytes(bytes)|NativeIoValue::Page{bytes,..}=>Some(bytes),_=>None})},
            JobOutcomeKind::Fault=>descriptor.fault(self.fault_detail.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original native I/O fault custody absent"))?),
            _=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native I/O cannot publish a preview or checkpoint")),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_target()?.1.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, _body: usize) -> Result<usize, ValueError> { Ok(self.close_target()?.1.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_target()?.1.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.close_target()?.1.depth) }

    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
        if self.terminal_is_empty(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}
        let demand=self.close_target().map_err(|error|OriginalTurnFailure::debit_original(error,cx))?.1;
        let mut blocked=false;
        let step=semio_framework_job::close_original_effect_step(cx,demand,|effect|{let turn=self.close_granted(effect);RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(|step|match step{InteractiveJobCloseStep::Pending{progress}=>RetainedCloneStep::Progress(progress),InteractiveJobCloseStep::Complete{progress}=>RetainedCloneStep::Complete(progress),InteractiveJobCloseStep::Blocked=>{blocked=true;RetainedCloneStep::Progress(Default::default())}}),work:turn.work})})?;
        if blocked{Ok(InteractiveJobCloseStep::Blocked)}else if self.terminal_is_empty(){Ok(InteractiveJobCloseStep::Complete{progress:step.progress()})}else{Ok(step)}
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.request_path.is_none() && self.request_extension.is_none() && self.retired_path.is_none() && self.scanned_path.is_none() && self.modified_path.is_none() && self.modified_entry.is_none() && self.retired_directory.is_none() && matches!(self.state, NativeIoState::Finished) && self.result.is_none() && self.fault_writer.is_none() && self.fault_detail.is_none() && self.read_buffer.is_none() && self.pending_payload.is_none() && self.retired_file.is_none()
    }
}

//#endregion 👷️Job

//#region 📊️ProcessMemory

#[cfg(target_os = "linux")]
fn process_resident_bytes() -> Option<u64> {
    let pages = std::fs::read_to_string("/proc/self/statm").ok()?.split_whitespace().nth(1)?.parse::<u64>().ok()?;
    Some(pages.saturating_mul(4096))
}

#[cfg(target_os = "macos")]
#[doc(hidden)] pub fn process_resident_bytes() -> Option<u64> {
    #[repr(C)]
    struct TimeValue {
        seconds: i32,
        microseconds: i32,
    }
    #[repr(C)]
    struct MachTaskBasicInfo {
        virtual_size: u64,
        resident_size: u64,
        resident_size_max: u64,
        user_time: TimeValue,
        system_time: TimeValue,
        policy: i32,
        suspend_count: i32,
    }
    unsafe extern "C" {
        fn mach_task_self() -> u32;
        fn task_info(target_task: u32, flavor: u32, task_info_out: *mut i32, task_info_out_count: *mut u32) -> i32;
    }
    let mut info = MachTaskBasicInfo { virtual_size: 0, resident_size: 0, resident_size_max: 0, user_time: TimeValue { seconds: 0, microseconds: 0 }, system_time: TimeValue { seconds: 0, microseconds: 0 }, policy: 0, suspend_count: 0 };
    let mut count = (size_of::<MachTaskBasicInfo>() / size_of::<u32>()) as u32;
    let status = unsafe { task_info(mach_task_self(), 20, (&mut info as *mut MachTaskBasicInfo).cast(), &mut count) };
    (status == 0).then_some(info.resident_size)
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn process_resident_bytes() -> Option<u64> {
    None
}

//#endregion 📊️ProcessMemory
