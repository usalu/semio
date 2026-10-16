//! 🚧️ The integration-fixtures checkpoint test gate: control frames and the inherited descriptor reader.

use super::*;

/// 🧪️ A bounded test-only pause placed inside the real GIS codec checkpoint callback.
#[cfg(feature = "integration-fixtures")]
pub struct InferenceCheckpointTestGateV1 {
    pub entered: AtomicBool,
    entered_notify: tokio::sync::Notify,
    pub(super) released: Mutex<bool>,
    release_notify: std::sync::Condvar,
    inherited: Option<Mutex<std::fs::File>>,
}

#[cfg(feature = "integration-fixtures")]
impl InferenceCheckpointTestGateV1 {
    pub fn new() -> Self {
        Self { entered: AtomicBool::new(false), entered_notify: tokio::sync::Notify::new(), released: Mutex::new(false), release_notify: std::sync::Condvar::new(), inherited: None }
    }

    /// 🔌️ Opens the process runner's fixed inherited checkpoint-control endpoint.
    pub fn open_inherited() -> Result<Self, InferenceRouteErrorV1> {
        Ok(Self { entered: AtomicBool::new(false), entered_notify: tokio::sync::Notify::new(), released: Mutex::new(false), release_notify: std::sync::Condvar::new(), inherited: Some(Mutex::new(inherited_inference_checkpoint_file()?)) })
    }

    pub async fn entered(&self) {
        loop {
            let entered = self.entered_notify.notified();
            tokio::pin!(entered);
            entered.as_mut().enable();
            if self.entered.load(Ordering::Acquire) {
                return;
            }
            entered.await;
        }
    }

    pub fn release(&self) {
        if let Ok(mut released) = self.released.lock() {
            *released = true;
            self.release_notify.notify_all();
        }
    }

    pub fn checkpoint(&self, control: &InferenceOperationControlV1, job_id: &str, progress_cursor: u64, completed: u64, total: u64) -> Result<(), InferenceRouteErrorV1> {
        if progress_cursor == 0 || progress_cursor > semio_hub_inference::schema::PROGRESS_MAX_CURSOR || completed == 0 || completed > total || total > semio_hub_inference::schema::SAFE_INTEGER_MAX {
            return Err(InferenceRouteErrorV1::Bounds);
        }
        self.entered.store(true, Ordering::Release);
        self.entered_notify.notify_waiters();
        let mut released = self.released.lock().map_err(|_| InferenceRouteErrorV1::Storage)?;
        if *released {
            return control.checkpoint(control.progress().0).map_err(Into::into);
        }
        if let Some(inherited) = &self.inherited {
            let mut endpoint = inherited.lock().map_err(|_| InferenceRouteErrorV1::Storage)?;
            write_inference_checkpoint_control_frame(&mut endpoint, &GisInferenceCheckpointControlFrameV1::progress_persisted(job_id, progress_cursor, completed, total))?;
            let frame = read_inference_checkpoint_control_frame(&mut endpoint)?;
            if frame != GisInferenceCheckpointControlFrameV1::release(job_id, progress_cursor, completed, total) {
                return Err(InferenceRouteErrorV1::Denied);
            }
            *released = true;
            return control.checkpoint(control.progress().0).map_err(Into::into);
        }
        while !*released {
            control.checkpoint(control.progress().0)?;
            let waited = self.release_notify.wait_timeout(released, std::time::Duration::from_millis(10)).map_err(|_| InferenceRouteErrorV1::Storage)?;
            released = waited.0;
        }
        Ok(())
    }
}

#[cfg(feature = "integration-fixtures")]
const INFERENCE_CHECKPOINT_CONTROL_FRAME_MAX_BYTES: usize = 256;
#[cfg(feature = "integration-fixtures")]
const INHERITED_INFERENCE_CHECKPOINT_DESCRIPTOR: i32 = 4;

#[cfg(feature="integration-fixtures")]
use semio_hub_inference::schema::GisInferenceCheckpointControlFrameV1;
use semio_hub_inference_runtime_contract::InferenceRouteErrorV1;

#[cfg(feature = "integration-fixtures")]
fn write_inference_checkpoint_control_frame(endpoint: &mut std::fs::File, frame: &GisInferenceCheckpointControlFrameV1) -> Result<(), InferenceRouteErrorV1> {
    use std::io::Write as _;
    let bytes = serde_json::to_vec(frame).map_err(|_| InferenceRouteErrorV1::Storage)?;
    if bytes.is_empty() || bytes.len() > INFERENCE_CHECKPOINT_CONTROL_FRAME_MAX_BYTES {
        return Err(InferenceRouteErrorV1::Bounds);
    }
    let length = u32::try_from(bytes.len()).map_err(|_| InferenceRouteErrorV1::Bounds)?.to_be_bytes();
    endpoint.write_all(&length).and_then(|_| endpoint.write_all(&bytes)).and_then(|_| endpoint.flush()).map_err(|_| InferenceRouteErrorV1::Storage)
}

#[cfg(feature = "integration-fixtures")]
fn read_inference_checkpoint_control_frame(endpoint: &mut std::fs::File) -> Result<GisInferenceCheckpointControlFrameV1, InferenceRouteErrorV1> {
    use std::io::Read as _;
    let mut length = [0_u8; 4];
    endpoint.read_exact(&mut length).map_err(|_| InferenceRouteErrorV1::Storage)?;
    let length = usize::try_from(u32::from_be_bytes(length)).map_err(|_| InferenceRouteErrorV1::Bounds)?;
    if length == 0 || length > INFERENCE_CHECKPOINT_CONTROL_FRAME_MAX_BYTES {
        return Err(InferenceRouteErrorV1::Bounds);
    }
    let mut bytes = vec![0_u8; length];
    endpoint.read_exact(&mut bytes).map_err(|_| InferenceRouteErrorV1::Storage)?;
    let frame = serde_json::from_slice::<GisInferenceCheckpointControlFrameV1>(&bytes).map_err(|_| InferenceRouteErrorV1::Invalid)?;
    let canonical = serde_json::to_vec(&frame).map_err(|_| InferenceRouteErrorV1::Storage)?;
    let exact = canonical == bytes;
    bytes.fill(0);
    if !exact {
        return Err(InferenceRouteErrorV1::Invalid);
    }
    Ok(frame)
}

#[cfg(all(feature = "integration-fixtures", unix))]
fn inherited_inference_checkpoint_file() -> Result<std::fs::File, InferenceRouteErrorV1> {
    use std::os::fd::FromRawFd as _;
    unsafe extern "C" {
        fn fcntl(fd: i32, command: i32, ...) -> i32;
    }
    if unsafe { fcntl(INHERITED_INFERENCE_CHECKPOINT_DESCRIPTOR, 1) } < 0 {
        return Err(InferenceRouteErrorV1::Unavailable);
    }
    Ok(unsafe { std::fs::File::from_raw_fd(INHERITED_INFERENCE_CHECKPOINT_DESCRIPTOR) })
}

#[cfg(all(feature = "integration-fixtures", windows))]
fn inherited_inference_checkpoint_file() -> Result<std::fs::File, InferenceRouteErrorV1> {
    use std::os::windows::io::FromRawHandle as _;
    unsafe extern "C" {
        fn _get_osfhandle(fd: i32) -> isize;
    }
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetCurrentProcess() -> *mut std::ffi::c_void;
        fn DuplicateHandle(source_process: *mut std::ffi::c_void, source_handle: *mut std::ffi::c_void, target_process: *mut std::ffi::c_void, target_handle: *mut *mut std::ffi::c_void, desired_access: u32, inherit_handle: i32, options: u32) -> i32;
    }
    let handle = unsafe { _get_osfhandle(INHERITED_INFERENCE_CHECKPOINT_DESCRIPTOR) };
    if handle == -1 {
        return Err(InferenceRouteErrorV1::Unavailable);
    }
    let process = unsafe { GetCurrentProcess() };
    let mut duplicate = std::ptr::null_mut();
    if unsafe { DuplicateHandle(process, handle as *mut std::ffi::c_void, process, &mut duplicate, 0, 0, 0x0000_0002) } == 0 || duplicate.is_null() {
        return Err(InferenceRouteErrorV1::Unavailable);
    }
    Ok(unsafe { std::fs::File::from_raw_handle(duplicate) })
}
