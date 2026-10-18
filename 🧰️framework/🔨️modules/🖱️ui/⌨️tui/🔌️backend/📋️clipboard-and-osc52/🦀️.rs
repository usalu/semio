//! 📋️ Clipboard and osc52: the backend error, the clipboard trait and results, the base64 and osc52 copy sequences, the native copy and paste workers, the host and memory clipboards and the osc52 payload check. See [`super`].

use super::*;

#[derive(Debug)]
pub struct BackendError {
    pub message: String,
}

impl std::fmt::Display for BackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<io::Error> for BackendError {
    fn from(error: io::Error) -> Self {
        Self { message: error.to_string() }
    }
}

//#region 📋️Clipboard
/// 📋️ Completion delivered by a clipboard I/O worker mailbox.
#[derive(Debug)]
pub enum ClipboardResult {
    Copied,
    Pasted(String),
    Failed(BackendError),
}

/// 🗒️ Enqueue-only clipboard access. Event callbacks submit and return; a later tick polls.
pub trait Clipboard {
    fn enqueue_copy(&mut self, text: String);
    fn enqueue_paste(&mut self);
    fn poll(&mut self) -> Option<ClipboardResult>;
}

fn base64_encode(data: &[u8]) -> String {
    const T: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    let mut i = 0;
    while i < data.len() {
        let b0 = data[i];
        let b1 = if i + 1 < data.len() { data[i + 1] } else { 0 };
        let b2 = if i + 2 < data.len() { data[i + 2] } else { 0 };
        out.push(T[(b0 >> 2) as usize] as char);
        out.push(T[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);
        if i + 1 < data.len() {
            out.push(T[(((b1 & 0x0f) << 2) | (b2 >> 6)) as usize] as char);
        } else {
            out.push('=');
        }
        if i + 2 < data.len() {
            out.push(T[(b2 & 0x3f) as usize] as char);
        } else {
            out.push('=');
        }
        i += 3;
    }
    out
}

/// 📎️ Builds an OSC 52 clipboard-set sequence for selection `c`.
pub fn osc52_copy_sequence(text: &str) -> String {
    let mut out = String::new();
    out.push('\u{1b}');
    out.push_str("]52;c;");
    out.push_str(&base64_encode(text.as_bytes()));
    out.push('\u{07}');
    out
}

fn clip_err(message: impl Into<String>) -> BackendError {
    BackendError { message: message.into() }
}

fn native_copy_worker(text: &str) -> Result<(), BackendError> {
    #[cfg(all(unix, not(target_arch = "wasm32")))]
    {
        use std::io::Write;
        use std::process as system_process;
        let candidates: &[&[&str]] = if cfg!(target_os = "macos") { &[&["pbcopy"]] } else { &[&["wl-copy"], &["xclip", "-selection", "clipboard"], &["xsel", "--clipboard", "--input"]] };
        for argv in candidates {
            let mut child = match system_process::Command::new(argv[0]).args(&argv[1..]).stdin(system_process::Stdio::piped()).stdout(system_process::Stdio::null()).stderr(system_process::Stdio::null()).spawn() {
                Ok(c) => c,
                Err(_) => continue,
            };
            if let Some(mut stdin) = child.stdin.take() {
                if stdin.write_all(text.as_bytes()).is_err() {
                    continue;
                }
            }
            if child.wait().is_ok_and(|s| s.success()) {
                return Ok(());
            }
        }
        Err(clip_err("no native clipboard tool available"))
    }
    #[cfg(all(windows, not(target_arch = "wasm32")))]
    {
        use std::io::Write;
        use std::process as system_process;
        let mut child = system_process::Command::new("clip").stdin(system_process::Stdio::piped()).stdout(system_process::Stdio::null()).stderr(system_process::Stdio::null()).spawn().map_err(|e| clip_err(e.to_string()))?;
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes()).map_err(|e| clip_err(e.to_string()))?;
        }
        if child.wait().is_ok_and(|s| s.success()) {
            return Ok(());
        }
        Err(clip_err("clip.exe failed"))
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = text;
        Err(clip_err("native clipboard unavailable on wasm"))
    }
}

fn native_paste_worker() -> Result<String, BackendError> {
    #[cfg(all(unix, not(target_arch = "wasm32")))]
    {
        use std::process as system_process;
        let candidates: &[&[&str]] = if cfg!(target_os = "macos") { &[&["pbpaste"]] } else { &[&["wl-paste", "--no-newline"], &["xclip", "-selection", "clipboard", "-o"], &["xsel", "--clipboard", "--output"]] };
        for argv in candidates {
            if let Ok(out) = system_process::Command::new(argv[0]).args(&argv[1..]).output() {
                if out.status.success() {
                    return Ok(String::from_utf8_lossy(&out.stdout).into_owned());
                }
            }
        }
        Err(clip_err("no native clipboard tool available"))
    }
    #[cfg(all(windows, not(target_arch = "wasm32")))]
    {
        use std::process as system_process;
        let out = system_process::Command::new("powershell").args(["-NoProfile", "-Command", "Get-Clipboard"]).output().map_err(|e| clip_err(e.to_string()))?;
        if out.status.success() {
            Ok(String::from_utf8_lossy(&out.stdout).trim_end_matches(&['\r', '\n'][..]).to_string())
        } else {
            Err(clip_err("Get-Clipboard failed"))
        }
    }
    #[cfg(target_arch = "wasm32")]
    {
        Err(clip_err("native clipboard unavailable on wasm"))
    }
}

/// 🗃️ Native clipboard mailbox backed by the process-wide worker pool's I/O lane.
pub struct HostClipboard {
    pub(super) pool: std::sync::Arc<semio_framework_async::WorkerPool>,
    pending: std::collections::VecDeque<std::sync::mpsc::Receiver<ClipboardResult>>,
}

impl HostClipboard {
    pub fn new(pool: std::sync::Arc<semio_framework_async::WorkerPool>) -> Self {
        Self { pool, pending: std::collections::VecDeque::new() }
    }

    pub(super) fn submit(&mut self, operation: impl FnOnce() -> ClipboardResult + Send + 'static) {
        let (sender, receiver) = std::sync::mpsc::channel();
        self.pool.submit(
            semio_framework_async::Lane::Io,
            Box::new(move || {
                let _ = sender.send(operation());
            }),
        );
        self.pending.push_back(receiver);
    }
}

impl Clipboard for HostClipboard {
    fn enqueue_copy(&mut self, text: String) {
        self.submit(move || native_copy_worker(&text).map_or_else(ClipboardResult::Failed, |_| ClipboardResult::Copied));
    }

    fn enqueue_paste(&mut self) {
        self.submit(|| native_paste_worker().map_or_else(ClipboardResult::Failed, ClipboardResult::Pasted));
    }

    fn poll(&mut self) -> Option<ClipboardResult> {
        let result = self.pending.front()?.try_recv();
        match result {
            Ok(result) => {
                self.pending.pop_front();
                Some(result)
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => None,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                self.pending.pop_front();
                Some(ClipboardResult::Failed(clip_err("clipboard worker disconnected")))
            }
        }
    }
}

/// 🧠️ In-memory clipboard for tests and headless hosts.
#[derive(Default)]
pub struct MemoryClipboard {
    pub text: String,
    pending: std::collections::VecDeque<ClipboardResult>,
}

impl Clipboard for MemoryClipboard {
    fn enqueue_copy(&mut self, text: String) {
        self.text = text;
        self.pending.push_back(ClipboardResult::Copied);
    }

    fn enqueue_paste(&mut self) {
        self.pending.push_back(ClipboardResult::Pasted(self.text.clone()));
    }

    fn poll(&mut self) -> Option<ClipboardResult> {
        self.pending.pop_front()
    }
}
//#endregion 📋️Clipboard

/// 🪣️ Largest clipboard payload `copy` sends as one OSC 52 sequence.
pub const OSC52_LIMIT: usize = 1 << 20;

#[cfg(any(test, all(feature = "native", not(target_arch = "wasm32"), any(unix, windows))))]
pub(super) fn osc52_payload_check(text: &str) -> io::Result<bool> {
    if text.is_empty() {
        return Ok(false);
    }
    if text.len() > OSC52_LIMIT {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "clipboard text exceeds the OSC 52 limit"));
    }
    Ok(true)
}
