//! 📂️ Native filesystem and process-observation jobs for interactive OS hosts.

use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, RetainedCloneGrant, RetainedCloneProgress, StepContext};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use std::fs::{File, ReadDir};
use std::io::{Read, Seek};
use std::path::PathBuf;

//#region 📂️Schema

pub const NATIVE_IO_PATH_CAPACITY: usize = 256;

#[derive(Debug, PartialEq, Eq)]
pub struct NativePathSet {
    entries: std::mem::ManuallyDrop<[Option<PathBuf>; NATIVE_IO_PATH_CAPACITY]>,
    length: usize,
}

impl NativePathSet {
    pub fn new() -> Self {
        Self { entries: std::mem::ManuallyDrop::new(std::array::from_fn(|_| None)), length: 0 }
    }

    pub fn try_push(&mut self, path: PathBuf) -> Result<(), PathBuf> {
        if self.length == NATIVE_IO_PATH_CAPACITY {
            return Err(path);
        }
        self.entries[self.length] = Some(path);
        self.length += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<PathBuf> {
        let index = self.length.checked_sub(1)?;
        self.length = index;
        self.entries[index].take()
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

impl Drop for NativePathSet {
    fn drop(&mut self) {
        if self.length == 0 {
            unsafe { std::mem::ManuallyDrop::drop(&mut self.entries) };
        } else {
            debug_assert!(false, "NativePathSet requires one-path close to terminal-empty; ordinary Drop preserves path owners");
        }
    }
}

impl Default for NativePathSet {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct NativeModifiedSet {
    entries: std::mem::ManuallyDrop<[Option<(PathBuf, std::time::SystemTime)>; NATIVE_IO_PATH_CAPACITY]>,
    length: usize,
}

impl NativeModifiedSet {
    fn new() -> Self {
        Self { entries: std::mem::ManuallyDrop::new(std::array::from_fn(|_| None)), length: 0 }
    }

    fn try_push(&mut self, entry: (PathBuf, std::time::SystemTime)) -> Result<(), (PathBuf, std::time::SystemTime)> {
        if self.length == NATIVE_IO_PATH_CAPACITY {
            return Err(entry);
        }
        self.entries[self.length] = Some(entry);
        self.length += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<(PathBuf, std::time::SystemTime)> {
        let index = self.length.checked_sub(1)?;
        self.length = index;
        self.entries[index].take()
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

impl Drop for NativeModifiedSet {
    fn drop(&mut self) {
        if self.length == 0 {
            unsafe { std::mem::ManuallyDrop::drop(&mut self.entries) };
        } else {
            debug_assert!(false, "NativeModifiedSet requires one-entry close to terminal-empty; ordinary Drop preserves modified-path owners");
        }
    }
}

#[derive(Debug)]
#[expect(clippy::large_enum_variant, reason = "Requests retain all 256 fixed path slots without allocating an uncredited wrapper on submission or rejection.")]
pub enum NativeIoRequest {
    ReadBytes(PathBuf),
    ReadPage { path: PathBuf, offset: u64, max_bytes: usize },
    ScanDirectory { path: PathBuf, directories_only: bool, extension: Option<String>, first_only: bool },
    Modified(NativePathSet),
    ProcessResidentBytes,
}

#[derive(Debug)]
#[expect(clippy::large_enum_variant, reason = "Completions transfer fixed path slots into one-entry retirement without allocating another result owner.")]
pub enum NativeIoValue {
    Bytes(semio_framework_job::RetainedJobPayload),
    Page { bytes: semio_framework_job::RetainedJobPayload, eof: bool },
    Paths(NativePathSet),
    Modified(NativeModifiedSet),
    ResidentBytes(Option<u64>),
}

impl NativeIoValue {
    pub fn retirement_demands(&self) -> Result<RetirementDemand, ValueError> {
        match self {
            Self::Bytes(bytes) | Self::Page { bytes, .. } if !bytes.terminal_is_empty() => native_io_child_demands(bytes.retirement_demands()?),
            Self::Paths(paths) if !paths.is_empty() => Ok(RetirementDemand { release_bytes: paths.entries[paths.length - 1].as_ref().expect("original path slot").capacity(), depth: 1, ..Default::default() }),
            Self::Modified(entries) if !entries.is_empty() => Ok(RetirementDemand { release_bytes: entries.entries[entries.length - 1].as_ref().expect("original modified slot").0.capacity(), depth: 1, ..Default::default() }),
            _ => Ok(Default::default()),
        }
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep {
        let demand = match self.retirement_demands() { Ok(demand) => demand, Err(error) => return InteractiveJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()} };
        if self.terminal_is_empty() { return InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() }; }
        if grant.maximum_items == 0 { return InteractiveJobCloseStep::Refused{kind:ValueRefusalKind::WorkLimit,progress:Default::default()}; }
        if grant.maximum_depth < demand.depth { return InteractiveJobCloseStep::Refused{kind:ValueRefusalKind::DepthLimit,progress:Default::default()}; }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return InteractiveJobCloseStep::Refused{kind:ValueRefusalKind::OwnershipLimit,progress:Default::default()}; }
        let progress = match self {
            Self::Bytes(bytes) | Self::Page { bytes, .. } => {
                let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
                let step = match bytes.close_step(child) { Ok(step) => step, Err(error) => return InteractiveJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()} };
                if let Err(error) = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, bytes.terminal_is_empty(), "original native I/O result payload") { return InteractiveJobCloseStep::Refused{kind:error.kind,progress:step.progress()}; }
                step.progress()
            }
            Self::Paths(paths) => { drop(paths.pop()); RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..RetainedCloneProgress::default() } },
            Self::Modified(entries) => { drop(entries.pop()); RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..RetainedCloneProgress::default() } },
            Self::ResidentBytes(_) => RetainedCloneProgress::default(),
        };
        InteractiveJobCloseStep::Pending { progress }.admit(grant, self.terminal_is_empty())
    }

    pub fn terminal_is_empty(&self) -> bool {
        match self {
            Self::Bytes(bytes) | Self::Page { bytes, .. } => bytes.terminal_is_empty(),
            Self::Paths(paths) => paths.is_empty(),
            Self::Modified(entries) => entries.is_empty(),
            Self::ResidentBytes(_) => true,
        }
    }
}

//#endregion 📂️Schema

fn native_io_close_grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: semio_framework_job::JOB_PAYLOAD_PAGE_BYTES, maximum_capacity_bytes: 0, maximum_release_bytes: semio_framework_job::JOB_PAYLOAD_PAGE_BYTES, maximum_depth: 3 }
}

fn native_io_child_demands(mut demand: RetirementDemand) -> Result<RetirementDemand, ValueError> {
    demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "original native I/O child depth overflow"))?;
    Ok(demand)
}

//#region 👷️Job

enum NativeIoState {
    Pending(NativeIoRequest),
    Reading { file: File, writer: semio_framework_job::RetainedJobPayloadWriter },
    ReadingBuffered { file: File, writer: semio_framework_job::RetainedJobPayloadWriter, bytes: [u8; semio_framework_job::JOB_PAYLOAD_PAGE_BYTES], length: usize },
    ReadingPage { file: File, cursor: u64, length: u64, remaining: usize, writer: semio_framework_job::RetainedJobPayloadWriter },
    ReadingPageBuffered { file: File, cursor: u64, length: u64, remaining: usize, writer: semio_framework_job::RetainedJobPayloadWriter, bytes: [u8; semio_framework_job::JOB_PAYLOAD_PAGE_BYTES], buffered: usize },
    ClosingWriterFault { writer: semio_framework_job::RetainedJobPayloadWriter, error: String },
    Scanning { entries: ReadDir, paths: NativePathSet, directories_only: bool, extension: Option<String>, first_only: bool },
    ClosingScanFault { paths: NativePathSet, rejected: Option<PathBuf>, extension: Option<String>, error: String },
    ReadingModified { paths: NativePathSet, modified: NativeModifiedSet },
    ClosingModifiedFault { paths: NativePathSet, modified: NativeModifiedSet, rejected: Option<(PathBuf, std::time::SystemTime)>, error: String },
    Finished,
}

#[derive(Clone, Copy)]
enum NativeIoCloseTarget { FaultWriter, ClearFaultWriter, FaultDetail, ClearFaultDetail, Writer, Paths, Modified, Path, Extension, RejectedPath, RejectedModified, Error, FinishState, ResultValue, ResultError, ClearResult, Complete }

pub struct NativeIoJob {
    state: NativeIoState,
    result: Option<Result<NativeIoValue, String>>,
    closing: bool,
    fault_writer: Option<semio_framework_job::RetainedJobPayloadWriter>,
    fault_cursor: usize,
    fault_complete: bool,
    fault_detail: Option<semio_framework_job::RetainedJobPayload>,
}

impl NativeIoJob {
    pub fn new(request: NativeIoRequest) -> Self {
        Self { state: NativeIoState::Pending(request), result: None, closing: false, fault_writer: None, fault_cursor: 0, fault_complete: false, fault_detail: None }
    }

    fn close_target(&self) -> Result<(NativeIoCloseTarget, RetirementDemand), ValueError> {
        use NativeIoCloseTarget::*;
        if let Some(writer)=self.fault_writer.as_ref(){return Ok(if writer.terminal_is_empty(){(ClearFaultWriter,RetirementDemand{copy_bytes:std::mem::size_of_val(&self.fault_writer),depth:1,..Default::default()})}else{(FaultWriter,native_io_child_demands(writer.retirement_demands()?)?)});}
        if let Some(detail)=self.fault_detail.as_ref(){return Ok(if detail.terminal_is_empty(){(ClearFaultDetail,RetirementDemand{copy_bytes:std::mem::size_of_val(&self.fault_detail),depth:1,..Default::default()})}else{(FaultDetail,native_io_child_demands(detail.retirement_demands()?)?)});}
        let release = |target, bytes, depth| Ok((target, RetirementDemand { release_bytes: bytes, depth, ..Default::default() }));
        match &self.state {
            NativeIoState::Reading { writer, .. } | NativeIoState::ReadingBuffered { writer, .. } | NativeIoState::ReadingPage { writer, .. } | NativeIoState::ReadingPageBuffered { writer, .. } | NativeIoState::ClosingWriterFault { writer, .. } if !writer.terminal_is_empty() => return Ok((Writer, native_io_child_demands(writer.retirement_demands()?)?)),
            NativeIoState::ClosingScanFault { rejected: Some(path), .. } => return release(RejectedPath,path.capacity(),1),
            NativeIoState::ClosingModifiedFault { rejected: Some((path, _)), .. } => return release(RejectedModified,path.capacity(),1),
            NativeIoState::Pending(NativeIoRequest::Modified(owner)) | NativeIoState::Scanning { paths: owner, .. } | NativeIoState::ClosingScanFault { paths: owner, .. } | NativeIoState::ReadingModified { paths: owner, .. } | NativeIoState::ClosingModifiedFault { paths: owner, .. } if !owner.is_empty() => return release(Paths,owner.entries[owner.length-1].as_ref().expect("original path slot").capacity(),1),
            NativeIoState::ReadingModified { modified: owner, .. } | NativeIoState::ClosingModifiedFault { modified: owner, .. } if !owner.is_empty() => return release(Modified,owner.entries[owner.length-1].as_ref().expect("original modified slot").0.capacity(),1),
            NativeIoState::Pending(NativeIoRequest::ScanDirectory { extension: Some(extension), .. }) | NativeIoState::Scanning { extension: Some(extension), .. } | NativeIoState::ClosingScanFault { extension: Some(extension), .. } => return release(Extension,extension.capacity(),1),
            NativeIoState::Pending(NativeIoRequest::ReadBytes(path) | NativeIoRequest::ReadPage { path, .. } | NativeIoRequest::ScanDirectory { path, .. }) if path.capacity() != 0 => return release(Path,path.capacity(),1),
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
        let path = match &self.state {
            NativeIoState::Pending(NativeIoRequest::ReadBytes(path) | NativeIoRequest::ReadPage { path, .. } | NativeIoRequest::ScanDirectory { path, .. }) => Some(path),
            NativeIoState::Pending(NativeIoRequest::Modified(paths)) => paths.entries[..paths.length].iter().flatten().next(),
            _ => None,
        }?;
        Some(path.as_os_str().as_encoded_bytes().as_ptr())
    }

    pub fn take_result(&mut self) -> Option<Result<NativeIoValue, String>> {
        self.result.take()
    }

    fn finish<'a>(&'a mut self, result: Result<NativeIoValue, String>, _cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        self.result=Some(result);self.state=NativeIoState::Finished;Ok(None)
    }

    fn publish_terminal<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        match self.result.as_ref(){
            Some(Ok(value))=>JobOutcomeBorrow::admit_complete(cx,None,match value{NativeIoValue::Bytes(bytes)|NativeIoValue::Page{bytes,..}=>Some(bytes),_=>None}),
            Some(Err(error))=>{
                if self.fault_detail.is_none(){
                    if !self.fault_complete{
                        let writer=self.fault_writer.get_or_insert_with(||semio_framework_job::RetainedJobPayloadWriter::new(semio_framework_job::JobPayloadStream::Fault));
                        match writer.write_slice_page(cx,error.as_bytes(),&mut self.fault_cursor){Ok(complete)=>self.fault_complete=complete,Err(semio_framework_job::JobPayloadAdmissionFault::OpportunityExhausted)=>{},Err(_)=>return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original native I/O fault payload exceeded received authority"))};return Ok(None);
                    }
                    let copied_bytes=std::mem::size_of::<semio_framework_job::RetainedJobPayloadWriter>()+std::mem::size_of::<semio_framework_job::RetainedJobPayload>();let grant=cx.retained_grant();
                    if grant.maximum_items==0||grant.maximum_copy_bytes<copied_bytes||grant.maximum_depth==0{return Ok(None)}
                    let original=self.fault_writer.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original native I/O fault writer absent"))?;
                    match original.finish(){Ok(detail)=>self.fault_detail=Some(detail),Err(original)=>{self.fault_writer=Some(original);return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original native I/O fault payload unfinished"))}}
                    cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes,..Default::default()})?;return Ok(None);
                }
                JobOutcomeBorrow::admit_fault(cx,self.fault_detail.as_ref().expect("original native I/O fault payload retained"))
            },
            None=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original native I/O terminal result absent")),
        }
    }

    fn start<'a>(&'a mut self, request: NativeIoRequest, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        match request {
            NativeIoRequest::ReadBytes(path) => match File::open(&path) {
                Ok(file) => {
                    self.state = NativeIoState::Reading { file, writer: semio_framework_job::RetainedJobPayloadWriter::new(semio_framework_job::JobPayloadStream::CommitOutput) };
                    Ok(None)
                }
                Err(error) => self.finish(Err(format!("{}: {error}", path.display())), cx),
            },
            NativeIoRequest::ReadPage { path, offset, max_bytes } => {
                if max_bytes == 0 || max_bytes > semio_framework_job::JOB_PAYLOAD_PAGE_BYTES {
                    return self.finish(Err("native I/O page exceeded the mounted one-page output authority".into()), cx);
                }
                match File::open(&path) {
                    Ok(mut file) => {
                        let length = match file.metadata() {
                            Ok(metadata) => metadata.len(),
                            Err(error) => return self.finish(Err(format!("{}: {error}", path.display())), cx),
                        };
                        if offset > length {
                            return self.finish(Err(format!("{}: page offset exceeds file length", path.display())), cx);
                        }
                        if let Err(error) = file.seek(std::io::SeekFrom::Start(offset)) {
                            return self.finish(Err(format!("{}: {error}", path.display())), cx);
                        }
                        self.state = NativeIoState::ReadingPage { file, cursor: offset, length, remaining: max_bytes, writer: semio_framework_job::RetainedJobPayloadWriter::new(semio_framework_job::JobPayloadStream::CommitOutput) };
                        Ok(None)
                    }
                    Err(error) => self.finish(Err(format!("{}: {error}", path.display())), cx),
                }
            }
            NativeIoRequest::ScanDirectory { path, directories_only, extension, first_only } => match std::fs::read_dir(&path) {
                Ok(entries) => {
                    self.state = NativeIoState::Scanning { entries, paths: NativePathSet::new(), directories_only, extension, first_only };
                    Ok(None)
                }
                Err(error) => self.finish(Err(format!("{}: {error}", path.display())), cx),
            },
            NativeIoRequest::Modified(paths) => {
                self.state = NativeIoState::ReadingModified { paths, modified: NativeModifiedSet::new() };
                Ok(None)
            }
            NativeIoRequest::ProcessResidentBytes => self.finish(Ok(NativeIoValue::ResidentBytes(process_resident_bytes())), cx),
        }
    }
}

impl InteractiveJob for NativeIoJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        if cx.is_cancelled(){self.closing=true;return JobOutcomeBorrow::admit_cancelled(cx)}
        if cx.should_yield() {
            return Ok(None);
        }
        cx.set_stage("NativePlatformIo");
        match std::mem::replace(&mut self.state, NativeIoState::Finished) {
            NativeIoState::Pending(request) => self.start(request, cx),
            NativeIoState::Reading { mut file, writer } => {
                let mut chunk = [0u8; semio_framework_job::JOB_PAYLOAD_PAGE_BYTES];
                match file.read(&mut chunk) {
                    Ok(0) => match writer.finish() {
                        Ok(bytes) => self.finish(Ok(NativeIoValue::Bytes(bytes)), cx),
                        Err(writer) => {
                            self.state = NativeIoState::ClosingWriterFault { writer, error: "native I/O byte output retained a rejected page".into() };
                            Ok(None)
                        }
                    },
                    Ok(count) => {
                        self.state = if writer.page_count() == 0 {
                            NativeIoState::ReadingBuffered { file, writer, bytes: chunk, length: count }
                        } else {
                            NativeIoState::ClosingWriterFault { writer, error: "native I/O populated read exceeds the mounted one-page consumer authority".into() }
                        };
                        Ok(None)
                    }
                    Err(error) => {
                        self.state = NativeIoState::ClosingWriterFault { writer, error: error.to_string() };
                        Ok(None)
                    }
                }
            }
            NativeIoState::ReadingBuffered { file, mut writer, bytes, length } => {
                let mut cursor = 0;
                match writer.write_slice_page(cx, &bytes[..length], &mut cursor) {
                    Ok(true) => self.state = NativeIoState::Reading { file, writer },
                    Ok(false) | Err(semio_framework_job::JobPayloadAdmissionFault::OpportunityExhausted) => {
                        self.state = NativeIoState::ReadingBuffered { file, writer, bytes, length };
                    }
                    Err(_) => self.state = NativeIoState::ClosingWriterFault { writer, error: "native I/O byte output exceeded retained page credits".into() },
                }
                Ok(None)
            }
            NativeIoState::ReadingPage { mut file, cursor, length, remaining, writer } => {
                if remaining == 0 || cursor >= length {
                    return match writer.finish() {
                        Ok(bytes) => self.finish(Ok(NativeIoValue::Page { bytes, eof: cursor >= length }), cx),
                        Err(writer) => {
                            self.state = NativeIoState::ClosingWriterFault { writer, error: "native I/O page output retained a rejected page".into() };
                            Ok(None)
                        }
                    };
                }
                let readable = remaining.min(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES).min(usize::try_from(length - cursor).unwrap_or(usize::MAX));
                let mut chunk = [0u8; semio_framework_job::JOB_PAYLOAD_PAGE_BYTES];
                match file.read(&mut chunk[..readable]) {
                    Ok(0) => match writer.finish() {
                        Ok(bytes) => self.finish(Ok(NativeIoValue::Page { bytes, eof: true }), cx),
                        Err(writer) => {
                            self.state = NativeIoState::ClosingWriterFault { writer, error: "native I/O page output retained a rejected page".into() };
                            Ok(None)
                        }
                    },
                    Ok(count) => {
                        self.state = NativeIoState::ReadingPageBuffered { file, cursor, length, remaining, writer, bytes: chunk, buffered: count };
                        Ok(None)
                    }
                    Err(error) => {
                        self.state = NativeIoState::ClosingWriterFault { writer, error: error.to_string() };
                        Ok(None)
                    }
                }
            }
            NativeIoState::ReadingPageBuffered { file, cursor, length, remaining, mut writer, bytes, buffered } => {
                let mut page_cursor = 0;
                match writer.write_slice_page(cx, &bytes[..buffered], &mut page_cursor) {
                    Ok(true) => {
                        self.state = NativeIoState::ReadingPage { file, cursor: cursor.saturating_add(buffered as u64), length, remaining: remaining - buffered, writer };
                    }
                    Ok(false) | Err(semio_framework_job::JobPayloadAdmissionFault::OpportunityExhausted) => {
                        self.state = NativeIoState::ReadingPageBuffered { file, cursor, length, remaining, writer, bytes, buffered };
                    }
                    Err(_) => self.state = NativeIoState::ClosingWriterFault { writer, error: "native I/O page output exceeded retained page credits".into() },
                }
                Ok(None)
            }
            NativeIoState::ClosingWriterFault { mut writer, error } => {
                let authority = native_io_close_grant();
                let child = RetainedCloneGrant { maximum_depth: authority.maximum_depth - 1, ..authority };
                let closed = match writer.close_step(child) {
                    Ok(step) => semio_framework_value::retained_clone::admit_retained_clone_close(child,step,writer.terminal_is_empty(),"original native I/O failed writer").is_ok() && writer.terminal_is_empty(),
                    Err(_) => false,
                };
                if closed { self.finish(Err(error),cx) }
                else { self.state = NativeIoState::ClosingWriterFault { writer,error }; Ok(None) }
            },
            NativeIoState::Scanning { mut entries, mut paths, directories_only, extension, first_only } => {
                let Some(entry) = entries.next() else { return self.finish(Ok(NativeIoValue::Paths(paths)), cx) };
                let Ok(entry) = entry else {
                    self.state = NativeIoState::Scanning { entries, paths, directories_only, extension, first_only };
                    return Ok(None);
                };
                let path = entry.path();
                if (directories_only && !path.is_dir()) || extension.as_ref().is_some_and(|extension| path.extension().and_then(|value| value.to_str()) != Some(extension.as_str())) {
                    self.state = NativeIoState::Scanning { entries, paths, directories_only, extension, first_only };
                    return Ok(None);
                }
                if let Err(rejected) = paths.try_push(path) {
                    self.state = NativeIoState::ClosingScanFault { paths, rejected: Some(rejected), extension, error: "native I/O directory result exceeded fixed path credits".into() };
                    return Ok(None);
                }
                if first_only {
                    return self.finish(Ok(NativeIoValue::Paths(paths)), cx);
                }
                self.state = NativeIoState::Scanning { entries, paths, directories_only, extension, first_only };
                Ok(None)
            }
            NativeIoState::ClosingScanFault { mut paths, mut rejected, mut extension, error } => {
                if rejected.take().is_some() {
                    self.state = NativeIoState::ClosingScanFault { paths, rejected, extension, error };
                    return Ok(None);
                }
                if paths.pop().is_some() {
                    self.state = NativeIoState::ClosingScanFault { paths, rejected, extension, error };
                    return Ok(None);
                }
                if extension.take().is_some() {
                    self.state = NativeIoState::ClosingScanFault { paths, rejected, extension, error };
                    return Ok(None);
                }
                self.finish(Err(error), cx)
            }
            NativeIoState::ReadingModified { mut paths, mut modified } => {
                let Some(path) = paths.pop() else { return self.finish(Ok(NativeIoValue::Modified(modified)), cx) };
                if let Some(modified_at) = std::fs::metadata(&path).ok().and_then(|metadata| metadata.modified().ok()) {
                    if let Err(rejected) = modified.try_push((path, modified_at)) {
                        self.state = NativeIoState::ClosingModifiedFault { paths, modified, rejected: Some(rejected), error: "native I/O modified result exceeded fixed path credits".into() };
                        return Ok(None);
                    }
                }
                self.state = NativeIoState::ReadingModified { paths, modified };
                Ok(None)
            }
            NativeIoState::ClosingModifiedFault { mut paths, mut modified, mut rejected, error } => {
                if rejected.take().is_some() {
                    self.state = NativeIoState::ClosingModifiedFault { paths, modified, rejected, error };
                    return Ok(None);
                }
                if paths.pop().is_some() {
                    self.state = NativeIoState::ClosingModifiedFault { paths, modified, rejected, error };
                    return Ok(None);
                }
                if modified.pop().is_some() {
                    self.state = NativeIoState::ClosingModifiedFault { paths, modified, rejected, error };
                    return Ok(None);
                }
                self.finish(Err(error), cx)
            }
            NativeIoState::Finished => self.publish_terminal(cx),
        }
    }

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

    fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep {
        let (target, demand) = match self.close_target() { Ok(frontier) => frontier, Err(error) => return InteractiveJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()} };
        let release = demand.release_bytes;
        if matches!(target, NativeIoCloseTarget::Complete) {
            return InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() }.admit(grant, self.terminal_is_empty());
        }
        if grant.maximum_items == 0 { return InteractiveJobCloseStep::Refused{kind:ValueRefusalKind::WorkLimit,progress:Default::default()}; }
        if grant.maximum_depth < demand.depth { return InteractiveJobCloseStep::Refused{kind:ValueRefusalKind::DepthLimit,progress:Default::default()}; }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < release { return InteractiveJobCloseStep::Refused{kind:ValueRefusalKind::OwnershipLimit,progress:Default::default()}; }
        self.closing = true;
        match target {
            NativeIoCloseTarget::FaultWriter=>{let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};let writer=self.fault_writer.as_mut().expect("original fault writer retained");return match writer.close_step(child){Ok(step)=>InteractiveJobCloseStep::Pending{progress:step.progress()}.admit(grant,false),Err(error)=>InteractiveJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()}};},
            NativeIoCloseTarget::FaultDetail=>{let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};let detail=self.fault_detail.as_mut().expect("original fault detail retained");return match detail.close_step(child){Ok(step)=>InteractiveJobCloseStep::Pending{progress:step.progress()}.admit(grant,false),Err(error)=>InteractiveJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()}};},
            NativeIoCloseTarget::ClearFaultWriter=>self.fault_writer=None,
            NativeIoCloseTarget::ClearFaultDetail=>self.fault_detail=None,
            NativeIoCloseTarget::Writer => {
                let writer = match &mut self.state {
                    NativeIoState::Reading { writer, .. } | NativeIoState::ReadingBuffered { writer, .. } | NativeIoState::ReadingPage { writer, .. } | NativeIoState::ReadingPageBuffered { writer, .. } | NativeIoState::ClosingWriterFault { writer, .. } => writer,
                    _ => return InteractiveJobCloseStep::Refused{kind:ValueRefusalKind::InvariantViolated,progress:Default::default()},
                };
                let child = RetainedCloneGrant { maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant };
                return match writer.close_step(child) {
                    Ok(step) => match semio_framework_value::retained_clone::admit_retained_clone_close(child,step,writer.terminal_is_empty(),"original native I/O writer") {
                        Ok(_) => InteractiveJobCloseStep::Pending {progress:step.progress()}.admit(grant,false),
                        Err(error) => InteractiveJobCloseStep::Refused{kind:error.kind,progress:step.progress()},
                    },
                    Err(error) => InteractiveJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()},
                };
            }
            NativeIoCloseTarget::Paths => match &mut self.state {
                NativeIoState::Pending(NativeIoRequest::Modified(paths)) | NativeIoState::Scanning { paths, .. } | NativeIoState::ClosingScanFault { paths, .. } | NativeIoState::ReadingModified { paths, .. } | NativeIoState::ClosingModifiedFault { paths, .. } => { drop(paths.pop()); },
                _ => return InteractiveJobCloseStep::Refused{kind:ValueRefusalKind::InvariantViolated,progress:Default::default()},
            },
            NativeIoCloseTarget::Modified => match &mut self.state {
                NativeIoState::ReadingModified { modified, .. } | NativeIoState::ClosingModifiedFault { modified, .. } => { drop(modified.pop()); },
                _ => return InteractiveJobCloseStep::Refused{kind:ValueRefusalKind::InvariantViolated,progress:Default::default()},
            },
            NativeIoCloseTarget::Path => match &mut self.state {
                NativeIoState::Pending(NativeIoRequest::ReadBytes(path) | NativeIoRequest::ReadPage { path, .. } | NativeIoRequest::ScanDirectory { path, .. }) => drop(std::mem::take(path)),
                _ => return InteractiveJobCloseStep::Refused{kind:ValueRefusalKind::InvariantViolated,progress:Default::default()},
            },
            NativeIoCloseTarget::Extension => match &mut self.state {
                NativeIoState::Pending(NativeIoRequest::ScanDirectory { extension, .. }) | NativeIoState::Scanning { extension, .. } | NativeIoState::ClosingScanFault { extension, .. } => drop(extension.take()),
                _ => return InteractiveJobCloseStep::Refused{kind:ValueRefusalKind::InvariantViolated,progress:Default::default()},
            },
            NativeIoCloseTarget::RejectedPath => if let NativeIoState::ClosingScanFault { rejected, .. } = &mut self.state { drop(rejected.take()); },
            NativeIoCloseTarget::RejectedModified => if let NativeIoState::ClosingModifiedFault { rejected, .. } = &mut self.state { drop(rejected.take()); },
            NativeIoCloseTarget::Error => match &mut self.state {
                NativeIoState::ClosingWriterFault { error, .. } | NativeIoState::ClosingScanFault { error, .. } | NativeIoState::ClosingModifiedFault { error, .. } => drop(std::mem::take(error)),
                _ => return InteractiveJobCloseStep::Refused{kind:ValueRefusalKind::InvariantViolated,progress:Default::default()},
            },
            NativeIoCloseTarget::FinishState => self.state = NativeIoState::Finished,
            NativeIoCloseTarget::ResultValue => {
                let Some(Ok(value)) = self.result.as_mut() else { return InteractiveJobCloseStep::Refused{kind:ValueRefusalKind::InvariantViolated,progress:Default::default()} };
                return match value.close_step(RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant }) {
                    InteractiveJobCloseStep::Complete { progress } => InteractiveJobCloseStep::Pending { progress },
                    step => step,
                }.admit(grant, self.terminal_is_empty());
            }
            NativeIoCloseTarget::ResultError => if let Some(Err(error)) = self.result.as_mut() { drop(std::mem::take(error)); },
            NativeIoCloseTarget::ClearResult => self.result = None,
            NativeIoCloseTarget::Complete => unreachable!(),
        }
        InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, copied_bytes:demand.copy_bytes, released_bytes: release, ..RetainedCloneProgress::default() } }.admit(grant, self.terminal_is_empty())
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && matches!(self.state, NativeIoState::Finished) && self.result.is_none() && self.fault_writer.is_none() && self.fault_detail.is_none()
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
fn process_resident_bytes() -> Option<u64> {
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

#[cfg(test)]
#[path = "../🧪️tests/🔬️native-io-unit/🦀️.rs"]
mod tests;
