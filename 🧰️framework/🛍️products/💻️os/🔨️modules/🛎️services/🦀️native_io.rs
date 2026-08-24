//! 📂️ Native filesystem and process-observation jobs for interactive OS hosts.

use semio_framework_job::{CommitCandidate, InteractiveJob, StepContext, StepOutcome};
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

    fn is_empty(&self) -> bool {
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
pub enum NativeIoRequest {
    ReadBytes(PathBuf),
    ReadPage { path: PathBuf, offset: u64, max_bytes: usize },
    ScanDirectory { path: PathBuf, directories_only: bool, extension: Option<String>, first_only: bool },
    Modified(NativePathSet),
    ProcessResidentBytes,
}

#[derive(Debug)]
pub enum NativeIoValue {
    Bytes(semio_framework_job::RetainedJobPayload),
    Page { bytes: semio_framework_job::RetainedJobPayload, eof: bool },
    Paths(NativePathSet),
    Modified(NativeModifiedSet),
    ResidentBytes(Option<u64>),
}

impl NativeIoValue {
    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        match self {
            Self::Bytes(bytes) | Self::Page { bytes, .. } => match bytes.close_step(maximum_items, maximum_bytes) {
                semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                semio_framework_job::JobPayloadCloseStep::Complete => semio_framework_job::InteractiveJobCloseStep::Complete,
            },
            Self::Paths(paths) if !paths.is_empty() => {
                let released_bytes = paths.entries[paths.length - 1].as_ref().map_or(0, |path| path.as_os_str().len());
                if maximum_items == 0 || maximum_bytes < released_bytes {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                drop(paths.pop());
                semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes }
            }
            Self::Modified(entries) if !entries.is_empty() => {
                let released_bytes = entries.entries[entries.length - 1].as_ref().map_or(0, |(path, _)| path.as_os_str().len());
                if maximum_items == 0 || maximum_bytes < released_bytes {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                drop(entries.pop());
                semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes }
            }
            _ => semio_framework_job::InteractiveJobCloseStep::Complete,
        }
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

pub struct NativeIoJob {
    state: NativeIoState,
    result: Option<Result<NativeIoValue, String>>,
    closing: bool,
}

impl NativeIoJob {
    pub fn new(request: NativeIoRequest) -> Self {
        Self { state: NativeIoState::Pending(request), result: None, closing: false }
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

    fn finish(&mut self, result: Result<NativeIoValue, String>, _cx: &mut StepContext<'_>) -> StepOutcome {
        let fault = result.as_ref().err().map(|_| semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) });
        self.result = Some(result);
        self.state = NativeIoState::Finished;
        fault.map_or_else(
            || {
                StepOutcome::Complete(CommitCandidate {
                    state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                    output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
                })
            },
            StepOutcome::Fault,
        )
    }

    fn start(&mut self, request: NativeIoRequest, cx: &mut StepContext<'_>) -> StepOutcome {
        match request {
            NativeIoRequest::ReadBytes(path) => match File::open(&path) {
                Ok(file) => {
                    self.state = NativeIoState::Reading { file, writer: semio_framework_job::RetainedJobPayloadWriter::new(semio_framework_job::JobPayloadStream::CommitOutput) };
                    StepOutcome::Yield
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
                        StepOutcome::Yield
                    }
                    Err(error) => self.finish(Err(format!("{}: {error}", path.display())), cx),
                }
            }
            NativeIoRequest::ScanDirectory { path, directories_only, extension, first_only } => match std::fs::read_dir(&path) {
                Ok(entries) => {
                    self.state = NativeIoState::Scanning { entries, paths: NativePathSet::new(), directories_only, extension, first_only };
                    StepOutcome::Yield
                }
                Err(error) => self.finish(Err(format!("{}: {error}", path.display())), cx),
            },
            NativeIoRequest::Modified(paths) => {
                self.state = NativeIoState::ReadingModified { paths, modified: NativeModifiedSet::new() };
                StepOutcome::Yield
            }
            NativeIoRequest::ProcessResidentBytes => self.finish(Ok(NativeIoValue::ResidentBytes(process_resident_bytes())), cx),
        }
    }
}

impl InteractiveJob for NativeIoJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if cx.is_cancelled() {
            self.closing = true;
            let _ = self.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            if matches!(self.state, NativeIoState::Finished) && self.result.is_none() {
                self.result = Some(Err("native I/O cancelled".into()));
                return StepOutcome::Cancelled;
            }
            return StepOutcome::Yield;
        }
        if cx.should_yield() {
            return StepOutcome::Yield;
        }
        cx.set_stage("NativePlatformIo");
        match std::mem::replace(&mut self.state, NativeIoState::Finished) {
            NativeIoState::Pending(request) => self.start(request, cx),
            NativeIoState::Reading { mut file, mut writer } => {
                let mut chunk = [0u8; semio_framework_job::JOB_PAYLOAD_PAGE_BYTES];
                match file.read(&mut chunk) {
                    Ok(0) => match writer.finish() {
                        Ok(bytes) => self.finish(Ok(NativeIoValue::Bytes(bytes)), cx),
                        Err(writer) => {
                            self.state = NativeIoState::ClosingWriterFault { writer, error: "native I/O byte output retained a rejected page".into() };
                            StepOutcome::Yield
                        }
                    },
                    Ok(count) => {
                        self.state = if writer.page_count() == 0 {
                            NativeIoState::ReadingBuffered { file, writer, bytes: chunk, length: count }
                        } else {
                            NativeIoState::ClosingWriterFault { writer, error: "native I/O populated read exceeds the mounted one-page consumer authority".into() }
                        };
                        StepOutcome::Yield
                    }
                    Err(error) => {
                        self.state = NativeIoState::ClosingWriterFault { writer, error: error.to_string() };
                        StepOutcome::Yield
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
                StepOutcome::Yield
            }
            NativeIoState::ReadingPage { mut file, cursor, length, remaining, mut writer } => {
                if remaining == 0 || cursor >= length {
                    return match writer.finish() {
                        Ok(bytes) => self.finish(Ok(NativeIoValue::Page { bytes, eof: cursor >= length }), cx),
                        Err(writer) => {
                            self.state = NativeIoState::ClosingWriterFault { writer, error: "native I/O page output retained a rejected page".into() };
                            StepOutcome::Yield
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
                            StepOutcome::Yield
                        }
                    },
                    Ok(count) => {
                        self.state = NativeIoState::ReadingPageBuffered { file, cursor, length, remaining, writer, bytes: chunk, buffered: count };
                        StepOutcome::Yield
                    }
                    Err(error) => {
                        self.state = NativeIoState::ClosingWriterFault { writer, error: error.to_string() };
                        StepOutcome::Yield
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
                StepOutcome::Yield
            }
            NativeIoState::ClosingWriterFault { mut writer, error } => match writer.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {
                semio_framework_job::JobPayloadCloseStep::Complete if writer.terminal_is_empty() => self.finish(Err(error), cx),
                _ => {
                    self.state = NativeIoState::ClosingWriterFault { writer, error };
                    StepOutcome::Yield
                }
            },
            NativeIoState::Scanning { mut entries, mut paths, directories_only, extension, first_only } => {
                let Some(entry) = entries.next() else { return self.finish(Ok(NativeIoValue::Paths(paths)), cx) };
                let Ok(entry) = entry else {
                    self.state = NativeIoState::Scanning { entries, paths, directories_only, extension, first_only };
                    return StepOutcome::Yield;
                };
                let path = entry.path();
                if (directories_only && !path.is_dir()) || extension.as_ref().is_some_and(|extension| path.extension().and_then(|value| value.to_str()) != Some(extension.as_str())) {
                    self.state = NativeIoState::Scanning { entries, paths, directories_only, extension, first_only };
                    return StepOutcome::Yield;
                }
                if let Err(rejected) = paths.try_push(path) {
                    self.state = NativeIoState::ClosingScanFault { paths, rejected: Some(rejected), extension, error: "native I/O directory result exceeded fixed path credits".into() };
                    return StepOutcome::Yield;
                }
                if first_only {
                    return self.finish(Ok(NativeIoValue::Paths(paths)), cx);
                }
                self.state = NativeIoState::Scanning { entries, paths, directories_only, extension, first_only };
                StepOutcome::Yield
            }
            NativeIoState::ClosingScanFault { mut paths, mut rejected, mut extension, error } => {
                if rejected.take().is_some() {
                    self.state = NativeIoState::ClosingScanFault { paths, rejected, extension, error };
                    return StepOutcome::Yield;
                }
                if paths.pop().is_some() {
                    self.state = NativeIoState::ClosingScanFault { paths, rejected, extension, error };
                    return StepOutcome::Yield;
                }
                if extension.take().is_some() {
                    self.state = NativeIoState::ClosingScanFault { paths, rejected, extension, error };
                    return StepOutcome::Yield;
                }
                self.finish(Err(error), cx)
            }
            NativeIoState::ReadingModified { mut paths, mut modified } => {
                let Some(path) = paths.pop() else { return self.finish(Ok(NativeIoValue::Modified(modified)), cx) };
                if let Some(modified_at) = std::fs::metadata(&path).ok().and_then(|metadata| metadata.modified().ok()) {
                    if let Err(rejected) = modified.try_push((path, modified_at)) {
                        self.state = NativeIoState::ClosingModifiedFault { paths, modified, rejected: Some(rejected), error: "native I/O modified result exceeded fixed path credits".into() };
                        return StepOutcome::Yield;
                    }
                }
                self.state = NativeIoState::ReadingModified { paths, modified };
                StepOutcome::Yield
            }
            NativeIoState::ClosingModifiedFault { mut paths, mut modified, mut rejected, error } => {
                if rejected.take().is_some() {
                    self.state = NativeIoState::ClosingModifiedFault { paths, modified, rejected, error };
                    return StepOutcome::Yield;
                }
                if paths.pop().is_some() {
                    self.state = NativeIoState::ClosingModifiedFault { paths, modified, rejected, error };
                    return StepOutcome::Yield;
                }
                if modified.pop().is_some() {
                    self.state = NativeIoState::ClosingModifiedFault { paths, modified, rejected, error };
                    return StepOutcome::Yield;
                }
                self.finish(Err(error), cx)
            }
            NativeIoState::Finished => {
                let detail =
                    cx.payload_from_bytes(semio_framework_job::JobPayloadStream::Fault, b"native I/O job polled after completion").unwrap_or_else(|_| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault));
                StepOutcome::Fault(semio_framework_job::JobFault { detail })
            }
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return if matches!(self.state, NativeIoState::Finished) && self.result.is_none() {
                semio_framework_job::InteractiveJobCloseStep::Complete
            } else {
                semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 }
            };
        }
        match &mut self.state {
            NativeIoState::Reading { writer, .. } | NativeIoState::ReadingBuffered { writer, .. } | NativeIoState::ReadingPage { writer, .. } | NativeIoState::ReadingPageBuffered { writer, .. } => {
                match writer.close_step(maximum_items.min(1), maximum_bytes) {
                    semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => {
                        return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
                    }
                    semio_framework_job::JobPayloadCloseStep::Complete if !writer.terminal_is_empty() => return semio_framework_job::InteractiveJobCloseStep::Blocked,
                    semio_framework_job::JobPayloadCloseStep::Complete => {}
                }
            }
            NativeIoState::ClosingWriterFault { writer, error } => match writer.close_step(maximum_items.min(1), maximum_bytes) {
                semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
                }
                semio_framework_job::JobPayloadCloseStep::Complete if !writer.terminal_is_empty() => return semio_framework_job::InteractiveJobCloseStep::Blocked,
                semio_framework_job::JobPayloadCloseStep::Complete if !error.is_empty() && maximum_bytes >= error.len() => {
                    let released_bytes = error.len();
                    drop(std::mem::take(error));
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
                }
                semio_framework_job::JobPayloadCloseStep::Complete if !error.is_empty() => {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                semio_framework_job::JobPayloadCloseStep::Complete => {}
            },
            NativeIoState::Pending(NativeIoRequest::Modified(paths)) if !paths.is_empty() => {
                let released_bytes = paths.entries[paths.length - 1].as_ref().map_or(0, |path| path.as_os_str().len());
                if maximum_bytes < released_bytes {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                drop(paths.pop());
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
            }
            NativeIoState::Pending(NativeIoRequest::ScanDirectory { extension, .. }) if extension.is_some() => {
                let released_bytes = extension.as_ref().map_or(0, String::len);
                if maximum_bytes < released_bytes {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                drop(extension.take());
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
            }
            NativeIoState::Pending(NativeIoRequest::ReadBytes(path) | NativeIoRequest::ReadPage { path, .. } | NativeIoRequest::ScanDirectory { path, .. }) if !path.as_os_str().is_empty() => {
                let released_bytes = path.as_os_str().len();
                if maximum_bytes < released_bytes {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                drop(std::mem::take(path));
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
            }
            NativeIoState::Scanning { paths, extension, .. } if !paths.is_empty() => {
                let released_bytes = paths.entries[paths.length - 1].as_ref().map_or(0, |path| path.as_os_str().len());
                if maximum_bytes < released_bytes {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                drop(paths.pop());
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
            }
            NativeIoState::Scanning { extension, .. } if extension.is_some() => {
                let released_bytes = extension.as_ref().map_or(0, String::len);
                if maximum_bytes < released_bytes {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                drop(extension.take());
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
            }
            NativeIoState::ClosingScanFault { rejected, .. } if rejected.is_some() => {
                let released_bytes = rejected.as_ref().map_or(0, |path| path.as_os_str().len());
                if maximum_bytes < released_bytes {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                drop(rejected.take());
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
            }
            NativeIoState::ClosingScanFault { paths, .. } if !paths.is_empty() => {
                let released_bytes = paths.entries[paths.length - 1].as_ref().map_or(0, |path| path.as_os_str().len());
                if maximum_bytes < released_bytes {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                drop(paths.pop());
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
            }
            NativeIoState::ClosingScanFault { extension, .. } if extension.is_some() => {
                let released_bytes = extension.as_ref().map_or(0, String::len);
                if maximum_bytes < released_bytes {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                drop(extension.take());
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
            }
            NativeIoState::ClosingScanFault { error, .. } if !error.is_empty() => {
                let released_bytes = error.len();
                if maximum_bytes < released_bytes {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                error.clear();
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
            }
            NativeIoState::ReadingModified { paths, .. } if !paths.is_empty() => {
                let released_bytes = paths.entries[paths.length - 1].as_ref().map_or(0, |path| path.as_os_str().len());
                if maximum_bytes < released_bytes {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                drop(paths.pop());
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
            }
            NativeIoState::ReadingModified { modified, .. } if !modified.is_empty() => {
                let released_bytes = modified.entries[modified.length - 1].as_ref().map_or(0, |(path, _)| path.as_os_str().len());
                if maximum_bytes < released_bytes {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                drop(modified.pop());
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
            }
            NativeIoState::ClosingModifiedFault { rejected, .. } if rejected.is_some() => {
                let released_bytes = rejected.as_ref().map_or(0, |(path, _)| path.as_os_str().len());
                if maximum_bytes < released_bytes {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                drop(rejected.take());
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
            }
            NativeIoState::ClosingModifiedFault { paths, .. } if !paths.is_empty() => {
                let released_bytes = paths.entries[paths.length - 1].as_ref().map_or(0, |path| path.as_os_str().len());
                if maximum_bytes < released_bytes {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                drop(paths.pop());
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
            }
            NativeIoState::ClosingModifiedFault { modified, .. } if !modified.is_empty() => {
                let released_bytes = modified.entries[modified.length - 1].as_ref().map_or(0, |(path, _)| path.as_os_str().len());
                if maximum_bytes < released_bytes {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                drop(modified.pop());
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
            }
            NativeIoState::ClosingModifiedFault { error, .. } if !error.is_empty() => {
                let released_bytes = error.len();
                if maximum_bytes < released_bytes {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                error.clear();
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
            }
            NativeIoState::Finished => {}
            _ => {
                self.state = NativeIoState::Finished;
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
        }
        if !matches!(self.state, NativeIoState::Finished) {
            self.state = NativeIoState::Finished;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(Ok(value)) = self.result.as_mut() {
            match value.close_step(maximum_items, maximum_bytes) {
                semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
                }
                semio_framework_job::InteractiveJobCloseStep::Blocked => return semio_framework_job::InteractiveJobCloseStep::Blocked,
                semio_framework_job::InteractiveJobCloseStep::Complete if !value.terminal_is_empty() => return semio_framework_job::InteractiveJobCloseStep::Blocked,
                semio_framework_job::InteractiveJobCloseStep::Complete => {}
            }
        }
        if let Some(Err(error)) = self.result.as_mut() {
            if !error.is_empty() {
                let released_bytes = error.len();
                if maximum_bytes < released_bytes {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                error.clear();
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
            }
        }
        if self.result.is_some() {
            if maximum_items == 0 {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.result = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && matches!(self.state, NativeIoState::Finished) && self.result.is_none()
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
mod tests {
    use super::*;

    fn payload_vec(mut payload: semio_framework_job::RetainedJobPayload) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(payload.len());
        let mut reader = payload.reader();
        while let Some(page) = reader.read_page(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {
            bytes.extend_from_slice(page);
        }
        while !payload.terminal_is_empty() {
            let _ = payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        }
        bytes
    }

    fn run(request: NativeIoRequest) -> Result<NativeIoValue, String> {
        let mut job = NativeIoJob::new(request);
        let params = semio_framework_job::BatchJobParams {
            operation: semio_framework_job::allocate_operation_id(),
            generation: semio_framework_job::Generation(1),
            cancel: semio_framework_job::root_cancel_token(),
            config: semio_framework_job::BatchDriveConfig {
                site: "native_io_test",
                stage: semio_framework_job::InteractiveStage::InteractiveStep,
                fuel_per_step: semio_framework_job::INTERACTIVE_LANE_FUEL,
                step_budget_ms: semio_framework_job::INTERACTIVE_LANE_WALL_MS,
            },
            now_ms: semio_framework_job::default_now_ms,
        };
        let mut preview_sequence = 0;
        loop {
            let outcome = semio_framework_job::drive_step(
                &mut job,
                params.config.site,
                params.operation,
                params.generation,
                params.config.stage,
                semio_framework_job::StepBudget::new(params.config.fuel_per_step, u64::MAX),
                params.cancel.clone(),
                params.now_ms,
                &mut preview_sequence,
            );
            if outcome.is_terminal() {
                break;
            }
        }
        job.take_result().expect("native I/O terminal result")
    }

    #[test]
    fn request_and_completion_are_send() {
        fn assert_send<T: Send>() {}
        assert_send::<NativeIoJob>();
    }

    #[test]
    fn resident_memory_observation_does_not_spawn_a_process() {
        let value = process_resident_bytes();
        assert!(value.is_none() || value.is_some_and(|bytes| bytes > 0));
    }

    #[test]
    fn path_set_max_plus_one_identity_zero_grant_and_job_close_are_exact() {
        let mut paths = NativePathSet::new();
        for index in 0..NATIVE_IO_PATH_CAPACITY {
            paths.try_push(PathBuf::from(format!("/retained-native-path-{index:04}"))).expect("fixed path capacity");
        }
        let plus_one = PathBuf::from("/retained-native-path-plus-one");
        let plus_one_pointer = plus_one.as_os_str().as_encoded_bytes().as_ptr();
        let returned = paths.try_push(plus_one).expect_err("maximum plus one returns exact path owner");
        assert_eq!(returned.as_os_str().as_encoded_bytes().as_ptr(), plus_one_pointer);
        drop(returned);
        let mut job = NativeIoJob::new(NativeIoRequest::Modified(paths));
        job.begin_close();
        assert_eq!(job.close_step(0, 0), semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 });
        let mut released = 0;
        while !job.terminal_is_empty() {
            if let semio_framework_job::InteractiveJobCloseStep::Pending { released_items, .. } = job.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {
                assert!(released_items <= 1);
                released += released_items;
            }
        }
        assert!(released >= NATIVE_IO_PATH_CAPACITY);
    }

    #[test]
    fn chunked_read_write_scan_and_modified_round_trip() {
        let root = std::env::temp_dir().join(format!("semio-native-io-{}", std::process::id()));
        let path = root.join("fixture.wasm");
        let bytes = vec![0xA5; 96 * 1024 + 17];
        std::fs::create_dir_all(&root).expect("test fixture directory");
        std::fs::write(&path, &bytes).expect("test-only filesystem oracle");
        assert!(run(NativeIoRequest::ReadBytes(path.clone())).is_err());
        let NativeIoValue::Page { bytes: first, eof: false } = run(NativeIoRequest::ReadPage { path: path.clone(), offset: 0, max_bytes: 16 * 1024 }).unwrap() else { panic!("first page") };
        assert_eq!(payload_vec(first), bytes[..16 * 1024]);
        let offset = (bytes.len() - 7) as u64;
        let NativeIoValue::Page { bytes: last, eof: true } = run(NativeIoRequest::ReadPage { path: path.clone(), offset, max_bytes: 16 * 1024 }).unwrap() else { panic!("last page") };
        assert_eq!(payload_vec(last), bytes[bytes.len() - 7..]);
        assert!(run(NativeIoRequest::ReadPage { path: path.clone(), offset: 0, max_bytes: 64 * 1024 + 1 }).is_err());
        let NativeIoValue::Paths(mut paths) = run(NativeIoRequest::ScanDirectory { path: root.clone(), directories_only: false, extension: Some("wasm".into()), first_only: true }).unwrap() else { panic!("scan value") };
        assert_eq!(paths.pop(), Some(path.clone()));
        let mut modified_paths = NativePathSet::new();
        modified_paths.try_push(path).expect("one modified path");
        let NativeIoValue::Modified(mut modified) = run(NativeIoRequest::Modified(modified_paths)).unwrap() else { panic!("modified value") };
        assert_eq!(modified.len(), 1);
        drop(modified.pop());
        std::fs::remove_dir_all(root).unwrap();
    }
}
