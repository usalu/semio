//! 🚪️ The host file-door call and its request trace label: one entry point to the `semioWgpuHostIo` binding the host environment installs.
//!
//! 🧭️ Lifted out of the shell's theme file import into host-base; the doors and the shell both call it. Until the shell cuts its own copy this file holds a verbatim copy of the four items.

/// 🚪️ Calls the ONE file-door binding the host environment installs (`semioWgpuHostIo`,
/// `🎯️targets/🧊️wgpu/🚪️host-io/🟦️.ts`). The PAGE installs it directly; the frame Worker installs a
/// `postMessage` bridge to the same implementation running on the page.
///
/// 🐛️ Why this indirection is the fix and not an indirection: this shell runs inside a dedicated Worker
/// (the one that owns the `OffscreenCanvas`), and a Worker has NO `window` and NO `document`. Both
/// browser halves here used to start with `web_sys::window()?` and return silently — so `Export
/// Document…` produced real bytes and handed them to nobody, and `Import Document…` opened nothing at
/// all, while every other hop of both journeys was already correct
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-end-to-end-verification-2026-09-14.md` §C).
#[cfg(target_arch = "wasm32")]
pub static HOST_IO_REQUEST_TRACE: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);

#[cfg(target_arch = "wasm32")]
pub struct HostIoRequestTraceGuard(u8);

#[cfg(target_arch = "wasm32")]
impl HostIoRequestTraceGuard {
    pub fn begin(request_json: &str) -> Self {
        let trace = if request_json.contains("\"op\":\"directory-http\"") {
            if request_json.contains("/auth/sessions/me") {
                1
            } else if request_json.contains("\"method\":\"POST\"") && request_json.contains("/directory/commands") {
                2
            } else if request_json.contains("/active-checkpoint/") {
                3
            } else if request_json.contains("/directory/spaces") {
                4
            } else {
                5
            }
        } else if request_json.contains("request-file-") || request_json.contains("download-media-export") {
            6
        } else if request_json.contains("extension-store-") {
            7
        } else if request_json.contains("format-temporal-values") {
            8
        } else if request_json.contains("\"op\":\"storage\"") {
            9
        } else if request_json.contains("\"op\":\"socket\"") {
            10
        } else {
            11
        };
        Self(HOST_IO_REQUEST_TRACE.swap(trace, std::sync::atomic::Ordering::AcqRel))
    }
}

#[cfg(target_arch = "wasm32")]
impl Drop for HostIoRequestTraceGuard {
    fn drop(&mut self) {
        HOST_IO_REQUEST_TRACE.store(self.0, std::sync::atomic::Ordering::Release);
    }
}

#[cfg(target_arch = "wasm32")]
pub fn host_io_request_trace_label() -> &'static str {
    match HOST_IO_REQUEST_TRACE.load(std::sync::atomic::Ordering::Acquire) {
        1 => "directory-get-session",
        2 => "directory-post-command",
        3 => "directory-get-checkpoint",
        4 => "directory-get-spaces",
        5 => "directory-http-other",
        6 => "file-dialog-or-download",
        7 => "extension-store",
        8 => "temporal-format",
        9 => "storage",
        10 => "socket",
        11 => "host-io-other",
        _ => "none",
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn host_io_request_trace_label() -> &'static str {
    "none"
}

#[cfg(target_arch = "wasm32")]
pub async fn host_io_call(request_json: &str, bytes: Option<&[u8]>) -> Result<String, String> {
    use wasm_bindgen::JsCast;
    let _trace = HostIoRequestTraceGuard::begin(request_json);
    let global = js_sys::global();
    let door = js_sys::Reflect::get(&global, &wasm_bindgen::JsValue::from_str("semioWgpuHostIo")).ok().and_then(|value| value.dyn_into::<js_sys::Function>().ok()).ok_or_else(|| "this isolate installs no semioWgpuHostIo door".to_string())?;
    let payload = match bytes {
        Some(bytes) => js_sys::Uint8Array::from(bytes).into(),
        None => wasm_bindgen::JsValue::NULL,
    };
    let result = door.call2(&wasm_bindgen::JsValue::NULL, &wasm_bindgen::JsValue::from_str(request_json), &payload).map_err(|error| format!("{error:?}"))?;
    let resolved = match result.dyn_ref::<js_sys::Promise>() {
        Some(promise) => semio_framework_async::browser::JsFuture::from(promise.clone()).await.map_err(|error| format!("{error:?}"))?,
        None => result,
    };
    resolved.as_string().ok_or_else(|| "semioWgpuHostIo answered a non-string".to_string())
}
