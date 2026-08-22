//! 📂️ Native filesystem and process-observation jobs for interactive OS hosts.

use semio_framework_job::{CommitCandidate, InteractiveJob, StepContext, StepOutcome};
use std::fs::{File, ReadDir};
use std::future::Future;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

//#region 📂️Schema

#[derive(Clone, Debug)]
pub enum NativeIoRequest {
    ReadBytes(PathBuf),
    ScanDirectory { path: PathBuf, directories_only: bool, extension: Option<String>, first_only: bool },
    Modified(Vec<PathBuf>),
    WriteBytes { path: PathBuf, bytes: Vec<u8>, create_parent: bool },
    ProcessResidentBytes,
}

#[derive(Debug)]
pub enum NativeIoValue {
    Bytes(Vec<u8>),
    Paths(Vec<PathBuf>),
    Modified(Vec<(PathBuf, std::time::SystemTime)>),
    Written,
    ResidentBytes(Option<u64>),
}

//#endregion 📂️Schema

//#region 📬️Completion

#[derive(Default)]
struct CompletionSlot {
    result: Mutex<Option<Result<NativeIoValue, String>>>,
    waker: Mutex<Option<Waker>>,
}

impl CompletionSlot {
    fn complete(&self, result: Result<NativeIoValue, String>) {
        *self.result.lock().expect("native I/O result lock") = Some(result);
        if let Some(waker) = self.waker.lock().expect("native I/O waker lock").take() {
            waker.wake();
        }
    }
}

pub struct NativeIoCompletion {
    slot: Arc<CompletionSlot>,
}

impl NativeIoCompletion {
    pub fn try_take(&self) -> Option<Result<NativeIoValue, String>> {
        self.slot.result.lock().expect("native I/O result lock").take()
    }
}

impl Future for NativeIoCompletion {
    type Output = Result<NativeIoValue, String>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(result) = self.try_take() {
            return Poll::Ready(result);
        }
        *self.slot.waker.lock().expect("native I/O waker lock") = Some(cx.waker().clone());
        if let Some(result) = self.try_take() {
            Poll::Ready(result)
        } else {
            Poll::Pending
        }
    }
}

//#endregion 📬️Completion

//#region 👷️Job

enum NativeIoState {
    Pending(NativeIoRequest),
    Reading { file: File, bytes: Vec<u8> },
    Scanning { entries: ReadDir, paths: Vec<PathBuf>, directories_only: bool, extension: Option<String>, first_only: bool },
    Writing { file: File, bytes: Vec<u8>, cursor: usize },
    Finished,
}

pub struct NativeIoJob {
    state: NativeIoState,
    completion: Arc<CompletionSlot>,
}

impl NativeIoJob {
    pub fn new(request: NativeIoRequest) -> (Self, NativeIoCompletion) {
        let completion = Arc::new(CompletionSlot::default());
        (Self { state: NativeIoState::Pending(request), completion: completion.clone() }, NativeIoCompletion { slot: completion })
    }

    fn finish(&mut self, result: Result<NativeIoValue, String>) -> StepOutcome {
        let fault = result.as_ref().err().map(|error| semio_framework_job::JobFault { detail: error.as_bytes().to_vec() });
        self.completion.complete(result);
        self.state = NativeIoState::Finished;
        fault.map_or_else(|| StepOutcome::Complete(CommitCandidate { state: Vec::new(), output: Vec::new() }), StepOutcome::Fault)
    }

    fn start(&mut self, request: NativeIoRequest) -> StepOutcome {
        match request {
            NativeIoRequest::ReadBytes(path) => match File::open(&path) {
                Ok(file) => {
                    self.state = NativeIoState::Reading { file, bytes: Vec::new() };
                    StepOutcome::Yield
                }
                Err(error) => self.finish(Err(format!("{}: {error}", path.display()))),
            },
            NativeIoRequest::ScanDirectory { path, directories_only, extension, first_only } => match std::fs::read_dir(&path) {
                Ok(entries) => {
                    self.state = NativeIoState::Scanning { entries, paths: Vec::new(), directories_only, extension, first_only };
                    StepOutcome::Yield
                }
                Err(error) => self.finish(Err(format!("{}: {error}", path.display()))),
            },
            NativeIoRequest::Modified(paths) => {
                let modified = paths.into_iter().filter_map(|path| std::fs::metadata(&path).ok().and_then(|metadata| metadata.modified().ok()).map(|modified| (path, modified))).collect();
                self.finish(Ok(NativeIoValue::Modified(modified)))
            }
            NativeIoRequest::WriteBytes { path, bytes, create_parent } => {
                if create_parent {
                    if let Some(parent) = path.parent() {
                        if let Err(error) = std::fs::create_dir_all(parent) {
                            return self.finish(Err(format!("{}: {error}", parent.display())));
                        }
                    }
                }
                match File::create(&path) {
                    Ok(file) => {
                        self.state = NativeIoState::Writing { file, bytes, cursor: 0 };
                        StepOutcome::Yield
                    }
                    Err(error) => self.finish(Err(format!("{}: {error}", path.display()))),
                }
            }
            NativeIoRequest::ProcessResidentBytes => self.finish(Ok(NativeIoValue::ResidentBytes(process_resident_bytes()))),
        }
    }
}

impl InteractiveJob for NativeIoJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if cx.is_cancelled() {
            self.completion.complete(Err("native I/O cancelled".into()));
            self.state = NativeIoState::Finished;
            return StepOutcome::Cancelled;
        }
        if cx.should_yield() {
            return StepOutcome::Yield;
        }
        cx.set_stage("NativePlatformIo");
        cx.consume_fuel(1);
        match std::mem::replace(&mut self.state, NativeIoState::Finished) {
            NativeIoState::Pending(request) => self.start(request),
            NativeIoState::Reading { mut file, mut bytes } => {
                let mut chunk = [0u8; 32 * 1024];
                match file.read(&mut chunk) {
                    Ok(0) => self.finish(Ok(NativeIoValue::Bytes(bytes))),
                    Ok(count) => {
                        bytes.extend_from_slice(&chunk[..count]);
                        self.state = NativeIoState::Reading { file, bytes };
                        StepOutcome::Yield
                    }
                    Err(error) => self.finish(Err(error.to_string())),
                }
            }
            NativeIoState::Scanning { mut entries, mut paths, directories_only, extension, first_only } => {
                for _ in 0..32 {
                    let Some(entry) = entries.next() else { return self.finish(Ok(NativeIoValue::Paths(paths))) };
                    let Ok(entry) = entry else { continue };
                    let path = entry.path();
                    if directories_only && !path.is_dir() {
                        continue;
                    }
                    if extension.as_ref().is_some_and(|extension| path.extension().and_then(|value| value.to_str()) != Some(extension.as_str())) {
                        continue;
                    }
                    paths.push(path);
                    if first_only {
                        return self.finish(Ok(NativeIoValue::Paths(paths)));
                    }
                }
                self.state = NativeIoState::Scanning { entries, paths, directories_only, extension, first_only };
                StepOutcome::Yield
            }
            NativeIoState::Writing { mut file, bytes, cursor } => {
                let end = (cursor + 32 * 1024).min(bytes.len());
                match file.write_all(&bytes[cursor..end]) {
                    Ok(()) if end == bytes.len() => self.finish(Ok(NativeIoValue::Written)),
                    Ok(()) => {
                        self.state = NativeIoState::Writing { file, bytes, cursor: end };
                        StepOutcome::Yield
                    }
                    Err(error) => self.finish(Err(error.to_string())),
                }
            }
            NativeIoState::Finished => StepOutcome::Fault(semio_framework_job::JobFault { detail: b"native I/O job polled after completion".to_vec() }),
        }
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

    fn run(request: NativeIoRequest) -> Result<NativeIoValue, String> {
        let (mut job, completion) = NativeIoJob::new(request);
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
        let outcome = semio_framework_job::run_to_completion(&mut job, &params);
        assert!(matches!(outcome, StepOutcome::Complete(_)));
        completion.try_take().expect("native I/O completion")
    }

    #[test]
    fn request_and_completion_are_send() {
        fn assert_send<T: Send>() {}
        assert_send::<NativeIoJob>();
        assert_send::<NativeIoCompletion>();
    }

    #[test]
    fn resident_memory_observation_does_not_spawn_a_process() {
        let value = process_resident_bytes();
        assert!(value.is_none() || value.is_some_and(|bytes| bytes > 0));
    }

    #[test]
    fn chunked_read_write_scan_and_modified_round_trip() {
        let root = std::env::temp_dir().join(format!("semio-native-io-{}", std::process::id()));
        let path = root.join("fixture.wasm");
        let bytes = vec![0xA5; 96 * 1024 + 17];
        assert!(matches!(run(NativeIoRequest::WriteBytes { path: path.clone(), bytes: bytes.clone(), create_parent: true }).unwrap(), NativeIoValue::Written));
        let NativeIoValue::Bytes(actual) = run(NativeIoRequest::ReadBytes(path.clone())).unwrap() else { panic!("read value") };
        assert_eq!(actual, bytes);
        let NativeIoValue::Paths(paths) = run(NativeIoRequest::ScanDirectory { path: root.clone(), directories_only: false, extension: Some("wasm".into()), first_only: true }).unwrap() else { panic!("scan value") };
        assert_eq!(paths, vec![path.clone()]);
        let NativeIoValue::Modified(modified) = run(NativeIoRequest::Modified(vec![path])).unwrap() else { panic!("modified value") };
        assert_eq!(modified.len(), 1);
        std::fs::remove_dir_all(root).unwrap();
    }
}
