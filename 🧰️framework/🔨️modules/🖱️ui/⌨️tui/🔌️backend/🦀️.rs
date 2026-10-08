use crate::tui::ansi::AnsiPatch;
use crate::tui::event::Event;
use crate::tui::geometry::Size;
use crate::tui::widget::CursorSpec;
use std::io;

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

//#region ???Clipboard
/// 📋️ Completion delivered by a clipboard I/O worker mailbox.
#[derive(Debug)]
pub enum ClipboardResult {
    Copied,
    Pasted(String),
    Failed(BackendError),
}

/// 📋️ Enqueue-only clipboard access. Event callbacks submit and return; a later tick polls.
pub trait Clipboard {
    fn enqueue_copy(&mut self, text: String);
    fn enqueue_paste(&mut self);
    fn poll(&mut self) -> Option<ClipboardResult>;
}

fn base64_encode(data: &[u8]) -> String {
    const T: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity((data.len() + 2) / 3 * 4);
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

/// ?? Builds an OSC 52 clipboard-set sequence for selection `c`.
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
            if child.wait().map(|s| s.success()).unwrap_or(false) {
                return Ok(());
            }
        }
        return Err(clip_err("no native clipboard tool available"));
    }
    #[cfg(all(windows, not(target_arch = "wasm32")))]
    {
        use std::io::Write;
        use std::process as system_process;
        let mut child = system_process::Command::new("clip").stdin(system_process::Stdio::piped()).stdout(system_process::Stdio::null()).stderr(system_process::Stdio::null()).spawn().map_err(|e| clip_err(e.to_string()))?;
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes()).map_err(|e| clip_err(e.to_string()))?;
        }
        if child.wait().map(|s| s.success()).unwrap_or(false) {
            return Ok(());
        }
        return Err(clip_err("clip.exe failed"));
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

/// 📋️ Native clipboard mailbox backed by the process-wide worker pool's I/O lane.
pub struct HostClipboard {
    pool: std::sync::Arc<semio_framework_async::WorkerPool>,
    pending: std::collections::VecDeque<std::sync::mpsc::Receiver<ClipboardResult>>,
}

impl HostClipboard {
    pub fn new(pool: std::sync::Arc<semio_framework_async::WorkerPool>) -> Self {
        Self { pool, pending: std::collections::VecDeque::new() }
    }

    fn submit(&mut self, operation: impl FnOnce() -> ClipboardResult + Send + 'static) {
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

/// ?? In-memory clipboard for tests and headless hosts.
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

#[cfg(all(test, not(target_arch = "wasm32")))]
include!("../🧪️tests/🔬️backend-clipboard-mailbox/🦀️.rs");
//#endregion ???Clipboard

/// 🌈️ How many colours the attached terminal renders faithfully.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ColorDepth {
    Monochrome,
    Ansi16,
    Ansi256,
    TrueColor,
}

/// 🔠️ Which glyph repertoire the attached terminal renders at the widths the text model assumes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnicodeLevel {
    Ascii,
    Full,
}

/// 🧰️ What the attached terminal can do; emitters and painters degrade to it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Capabilities {
    pub color: ColorDepth,
    pub synchronized_output: bool,
    pub unicode: UnicodeLevel,
}

/// ⏰️ A cloneable, thread-safe handle that makes a pending or the next `TerminalBackend::wait` return `Event::Wake`.
#[derive(Clone)]
pub struct Waker(std::sync::Arc<dyn Fn() + Send + Sync>);

impl Waker {
    pub fn new(wake: impl Fn() + Send + Sync + 'static) -> Self {
        Self(std::sync::Arc::new(wake))
    }

    pub fn wake(&self) {
        (self.0)();
    }
}

/// ??? A platform terminal I/O implementation, kept out of the retained-mode core.
pub trait TerminalBackend {
    fn enter(&mut self) -> io::Result<()>;
    fn leave(&mut self) -> io::Result<()>;
    fn size(&self) -> io::Result<Size>;
    fn capabilities(&self) -> Capabilities;
    fn present(&mut self, patch: &AnsiPatch, cursor: Option<CursorSpec>) -> io::Result<()>;
    fn wait(&mut self, deadline: Option<std::time::Instant>) -> io::Result<Vec<Event>>;
    fn waker(&self) -> Waker;
    fn copy(&mut self, text: &str) -> io::Result<()>;
}

#[cfg(all(feature = "tui-terminal", not(target_arch = "wasm32"), any(unix, windows)))]
const INTERIM_WAKE_SLICE: std::time::Duration = std::time::Duration::from_millis(80);

/// ⏳️ Interim wake polling: how long one platform wait may block, and whether it ends at `deadline`.
#[cfg(all(feature = "tui-terminal", not(target_arch = "wasm32"), any(unix, windows)))]
fn interim_wait_slice(deadline: Option<std::time::Instant>) -> (std::time::Duration, bool) {
    let remaining = deadline.map(|deadline| deadline.saturating_duration_since(std::time::Instant::now()));
    (remaining.map_or(INTERIM_WAKE_SLICE, |remaining| remaining.min(INTERIM_WAKE_SLICE)), remaining.is_some_and(|remaining| remaining <= INTERIM_WAKE_SLICE))
}

/// 🎚️ The capabilities every painter assumed before detection existed: truecolor, full Unicode, no synchronized output.
#[cfg(all(feature = "tui-terminal", not(target_arch = "wasm32"), any(unix, windows)))]
fn assumed_capabilities() -> Capabilities {
    Capabilities { color: ColorDepth::TrueColor, synchronized_output: false, unicode: UnicodeLevel::Full }
}

#[cfg(all(feature = "tui-terminal", not(target_arch = "wasm32"), any(unix, windows)))]
fn release_owned_terminal_cleanup(owned: &mut bool, succeeded: bool) -> bool {
    if *owned && succeeded {
        *owned = false;
    }
    !*owned
}

#[cfg(all(feature = "tui-terminal", not(target_arch = "wasm32"), any(unix, windows)))]
fn terminal_entry_is_available(entered: bool, ansi_setup_owned: bool, platform_cleanup_empty: bool) -> bool {
    !entered && !ansi_setup_owned && platform_cleanup_empty
}

#[cfg(all(feature = "tui-terminal", unix, not(target_arch = "wasm32")))]
mod native_unix {
    use super::*;
    use crate::tui::ansi::{setup_sequence, teardown_sequence, AnsiParser};
    use std::io::Write;
    use std::os::unix::io::RawFd;

    fn err(message: impl Into<String>) -> io::Error {
        io::Error::other(message.into())
    }

    fn retry_mode_restoration(raw_mode_entered: &mut bool, restore: impl FnOnce() -> bool) -> bool {
        if !*raw_mode_entered {
            return true;
        }
        if restore() {
            *raw_mode_entered = false;
            true
        } else {
            false
        }
    }

    /// ??? Raw-mode terminal backend for unix (macOS/Linux), driven by `libc` alone.
    pub struct NativeTerminal {
        fd: RawFd,
        terminal: std::fs::File,
        original: libc::termios,
        parser: AnsiParser,
        entered: bool,
        ansi_setup_owned: bool,
        raw_mode_entered: bool,
        wake: std::sync::Arc<std::sync::atomic::AtomicBool>,
    }

    impl NativeTerminal {
        pub fn new() -> Result<Self, BackendError> {
            use std::os::fd::AsRawFd;
            let terminal = std::fs::OpenOptions::new().read(true).write(true).open("/dev/tty").or_else(|original| {
                use std::os::unix::ffi::OsStrExt;
                let mut device = [0 as libc::c_char; 4096];
                if unsafe { libc::ttyname_r(libc::STDIN_FILENO, device.as_mut_ptr(), device.len()) } != 0 { return Err(original); }
                let name = unsafe { std::ffi::CStr::from_ptr(device.as_ptr()) };
                std::fs::OpenOptions::new().read(true).write(true).open(std::ffi::OsStr::from_bytes(name.to_bytes()))
            }).map_err(|error| err(error.to_string()))?;
            let fd = terminal.as_raw_fd();
            let original = unsafe {
                let mut t: libc::termios = std::mem::zeroed();
                if libc::tcgetattr(fd, &mut t) != 0 {
                    return Err(err("tcgetattr failed").into());
                }
                t
            };
            Ok(Self { fd, terminal, original, parser: AnsiParser::new(), entered: false, ansi_setup_owned: false, raw_mode_entered: false, wake: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)) })
        }
    }

    impl TerminalBackend for NativeTerminal {
        fn size(&self) -> io::Result<Size> {
            unsafe {
                let mut ws: libc::winsize = std::mem::zeroed();
                if libc::ioctl(self.fd, libc::TIOCGWINSZ, &mut ws) != 0 {
                    return Err(err("TIOCGWINSZ failed"));
                }
                Ok(Size { width: ws.ws_col, height: ws.ws_row })
            }
        }

        fn enter(&mut self) -> io::Result<()> {
            if !terminal_entry_is_available(self.entered, self.ansi_setup_owned, !self.raw_mode_entered) {
                return Err(err("Terminal entry or cleanup is already active"));
            }
            let mut raw = self.original;
            raw.c_lflag &= !(libc::ECHO | libc::ICANON | libc::ISIG | libc::IEXTEN);
            raw.c_iflag &= !(libc::IXON | libc::ICRNL | libc::BRKINT | libc::INPCK | libc::ISTRIP);
            raw.c_oflag &= !libc::OPOST;
            raw.c_cc[libc::VMIN] = 0;
            raw.c_cc[libc::VTIME] = 0;
            unsafe {
                if libc::tcsetattr(self.fd, libc::TCSANOW, &raw) != 0 {
                    return Err(err("tcsetattr failed"));
                }
            }
            self.raw_mode_entered = true;
            self.ansi_setup_owned = true;
            if let Err(error) = self.terminal.write_all(setup_sequence().as_bytes()).and_then(|()| self.terminal.flush()).map_err(|e| err(e.to_string())) {
                let teardown = self.teardown_ansi();
                let restoration = self.restore_mode();
                self.entered = false;
                return match (teardown, restoration) {
                    (Ok(()), Ok(())) => Err(error),
                    _ => Err(err("Terminal setup write failed and terminal rollback is pending")),
                };
            }
            self.entered = true;
            Ok(())
        }

        fn leave(&mut self) -> io::Result<()> {
            if !self.entered && !self.ansi_setup_owned && !self.raw_mode_entered {
                return Ok(());
            }
            let teardown = self.teardown_ansi();
            let restoration = self.restore_mode();
            self.entered = false;
            teardown.and(restoration)
        }

        fn capabilities(&self) -> Capabilities {
            assumed_capabilities()
        }

        fn present(&mut self, patch: &AnsiPatch, cursor: Option<CursorSpec>) -> io::Result<()> {
            let _ = cursor;
            self.terminal.write_all(patch.0.as_bytes()).map_err(|e| err(e.to_string()))?;
            self.terminal.flush().map_err(|e| err(e.to_string()))
        }

        fn wait(&mut self, deadline: Option<std::time::Instant>) -> io::Result<Vec<Event>> {
            let mut events = Vec::new();
            loop {
                if self.wake.swap(false, std::sync::atomic::Ordering::AcqRel) {
                    events.push(Event::Wake);
                    return Ok(events);
                }
                let (timeout, last) = interim_wait_slice(deadline);
                let mut readable: libc::fd_set = unsafe { std::mem::zeroed() };
                let mut wait = libc::timeval { tv_sec: timeout.as_secs() as libc::time_t, tv_usec: timeout.subsec_micros() as libc::suseconds_t };
                let ready = unsafe {
                    libc::FD_ZERO(&mut readable);
                    libc::FD_SET(self.fd, &mut readable);
                    libc::select(self.fd + 1, &mut readable, std::ptr::null_mut(), std::ptr::null_mut(), &mut wait)
                };
                if ready > 0 && unsafe { libc::FD_ISSET(self.fd, &readable) } {
                    let mut buf = [0u8; 4096];
                    let n = unsafe { libc::read(self.fd, buf.as_mut_ptr() as *mut _, buf.len()) };
                    if n > 0 {
                        self.parser.feed(&buf[..n as usize], &mut events);
                    }
                    return Ok(events);
                }
                self.parser.flush_escape(&mut events);
                if last || ready < 0 || !events.is_empty() {
                    return Ok(events);
                }
            }
        }

        fn waker(&self) -> Waker {
            let wake = std::sync::Arc::clone(&self.wake);
            Waker::new(move || wake.store(true, std::sync::atomic::Ordering::Release))
        }

        fn copy(&mut self, text: &str) -> io::Result<()> {
            self.terminal.write_all(osc52_copy_sequence(text).as_bytes())?;
            self.terminal.flush()
        }
    }

    impl NativeTerminal {
        fn teardown_ansi(&mut self) -> io::Result<()> {
            if !self.ansi_setup_owned {
                return Ok(());
            }
            let result = self.terminal.write_all(teardown_sequence().as_bytes()).and_then(|()| self.terminal.flush()).map_err(|e| err(e.to_string()));
            release_owned_terminal_cleanup(&mut self.ansi_setup_owned, result.is_ok());
            result
        }

        fn restore_mode(&mut self) -> io::Result<()> {
            if !self.raw_mode_entered {
                return Ok(());
            }
            if !retry_mode_restoration(&mut self.raw_mode_entered, || unsafe { libc::tcsetattr(self.fd, libc::TCSANOW, &self.original) } == 0) {
                return Err(err("tcsetattr restore failed"));
            }
            Ok(())
        }
    }

    #[cfg(test)]
    include!("../🧪️tests/🔬️backend-native-unix-unit/🦀️.rs");

    impl Drop for NativeTerminal {
        fn drop(&mut self) {
            let _ = self.leave();
        }
    }
}
#[cfg(all(feature = "tui-terminal", unix, not(target_arch = "wasm32")))]
pub use native_unix::NativeTerminal;

#[cfg(all(feature = "tui-terminal", windows))]
mod native_windows {
    use super::*;
    use crate::tui::ansi::{setup_sequence, teardown_sequence, AnsiParser};
    use crate::tui::component::windows_abi::{
        GetConsoleMode, GetConsoleCP, GetConsoleOutputCP, SetConsoleCP, SetConsoleOutputCP, GetConsoleScreenBufferInfo, ReadFile, SetConsoleMode, WaitForSingleObject, WriteFile, CONSOLE_SCREEN_BUFFER_INFO, DISABLE_NEWLINE_AUTO_RETURN, ENABLE_ECHO_INPUT, ENABLE_LINE_INPUT, ENABLE_PROCESSED_INPUT,
        ENABLE_VIRTUAL_TERMINAL_INPUT, ENABLE_VIRTUAL_TERMINAL_PROCESSING, HANDLE, WAIT_OBJECT_0, WAIT_TIMEOUT,
    };

    fn err(message: impl Into<String>) -> io::Error {
        io::Error::other(message.into())
    }

    /// 🪟️ VT-mode terminal backend using borrowed standard handles and the private first-party Win32 ABI.
    ///
    /// The dashboard main loop owns this backend. Polling waits once and admits at most one
    /// 4 KiB input page; presenting performs one retained-patch write.
    pub struct NativeTerminal {
        stdin: HANDLE,
        stdout: HANDLE,
        _console_input: std::fs::File,
        _console_output: std::fs::File,
        original_in: u32,
        original_out: u32,
        original_input_cp: u32,
        original_output_cp: u32,
        parser: AnsiParser,
        entered: bool,
        ansi_setup_owned: bool,
        modes: ConsoleModeOwnership,
        wake: std::sync::Arc<std::sync::atomic::AtomicBool>,
    }

    #[derive(Default)]
    struct ConsoleModeOwnership {
        stdin_changed: bool,
        stdout_changed: bool,
        input_cp_changed: bool,
        output_cp_changed: bool,
    }

    impl ConsoleModeOwnership {
        fn restore_with(&mut self, mut set_mode: impl FnMut(HANDLE, u32) -> bool, stdin: HANDLE, original_in: u32, stdout: HANDLE, original_out: u32) -> bool {
            let mut restored = true;
            if self.stdin_changed && set_mode(stdin, original_in) {
                self.stdin_changed = false;
            } else if self.stdin_changed {
                restored = false;
            }
            if self.stdout_changed && set_mode(stdout, original_out) {
                self.stdout_changed = false;
            } else if self.stdout_changed {
                restored = false;
            }
            restored
        }

        fn restore_code_pages_with(&mut self, mut input: impl FnMut(u32) -> bool, mut output: impl FnMut(u32) -> bool, original_input: u32, original_output: u32) {
            if self.input_cp_changed && input(original_input) { self.input_cp_changed = false; }
            if self.output_cp_changed && output(original_output) { self.output_cp_changed = false; }
        }

        fn is_empty(&self) -> bool {
            !self.stdin_changed && !self.stdout_changed && !self.input_cp_changed && !self.output_cp_changed
        }
    }

    impl NativeTerminal {
        pub fn new() -> Result<Self, BackendError> {
            unsafe {
                use std::os::windows::io::AsRawHandle;
                let console_input = std::fs::OpenOptions::new().read(true).write(true).open("CONIN$").map_err(|error| err(error.to_string()))?;
                let console_output = std::fs::OpenOptions::new().read(true).write(true).open("CONOUT$").map_err(|error| err(error.to_string()))?;
                let stdin = console_input.as_raw_handle();
                let stdout = console_output.as_raw_handle();
                let mut original_in = 0u32;
                let mut original_out = 0u32;
                if GetConsoleMode(stdin, &mut original_in) == 0 || GetConsoleMode(stdout, &mut original_out) == 0 {
                    return Err(err("GetConsoleMode failed").into());
                }
                let original_input_cp = GetConsoleCP(); let original_output_cp = GetConsoleOutputCP();
                if original_input_cp == 0 || original_output_cp == 0 { return Err(err("GetConsoleCP failed").into()); }
                Ok(Self { stdin, stdout, original_input_cp, original_output_cp, _console_input: console_input, _console_output: console_output, original_in, original_out, parser: AnsiParser::new(), entered: false, ansi_setup_owned: false, modes: ConsoleModeOwnership::default(), wake: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)) })
            }
        }
    }

    impl TerminalBackend for NativeTerminal {
        fn size(&self) -> io::Result<Size> {
            unsafe {
                let mut info: CONSOLE_SCREEN_BUFFER_INFO = std::mem::zeroed();
                if GetConsoleScreenBufferInfo(self.stdout, &mut info) == 0 {
                    return Err(err("GetConsoleScreenBufferInfo failed"));
                }
                let width = (info.srWindow.Right - info.srWindow.Left + 1).max(0) as u16;
                let height = (info.srWindow.Bottom - info.srWindow.Top + 1).max(0) as u16;
                Ok(Size { width, height })
            }
        }

        fn enter(&mut self) -> io::Result<()> {
            if !terminal_entry_is_available(self.entered, self.ansi_setup_owned, self.modes.is_empty()) {
                return Err(err("Terminal entry or cleanup is already active"));
            }
            unsafe {
                if SetConsoleOutputCP(65001) == 0 { return Err(err("SetConsoleOutputCP failed")); }
                self.modes.output_cp_changed = true;
                if SetConsoleCP(65001) == 0 { let _ = self.restore_modes(); return Err(err("SetConsoleCP failed")); }
                self.modes.input_cp_changed = true;
                let out_mode = self.original_out | ENABLE_VIRTUAL_TERMINAL_PROCESSING | DISABLE_NEWLINE_AUTO_RETURN;
                let in_mode = (self.original_in | ENABLE_VIRTUAL_TERMINAL_INPUT) & !(ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT | ENABLE_PROCESSED_INPUT);
                if SetConsoleMode(self.stdout, out_mode) == 0 {
                    let _ = self.restore_modes();
                    return Err(err("SetConsoleMode failed"));
                }
                self.modes.stdout_changed = true;
                if SetConsoleMode(self.stdin, in_mode) == 0 {
                    return match self.restore_modes() {
                        Ok(()) => Err(err("SetConsoleMode failed")),
                        Err(_) => Err(err("SetConsoleMode failed and rollback is pending")),
                    };
                }
                self.modes.stdin_changed = true;
            }
            self.ansi_setup_owned = true;
            if let Err(error) = self.write_raw(setup_sequence().as_bytes()) {
                let teardown = self.teardown_ansi();
                let restoration = self.restore_modes();
                self.entered = false;
                return match (teardown, restoration) {
                    (Ok(()), Ok(())) => Err(error),
                    _ => Err(err("Terminal setup write failed and rollback is pending")),
                };
            }
            self.entered = true;
            Ok(())
        }

        fn leave(&mut self) -> io::Result<()> {
            if !self.entered && !self.ansi_setup_owned && self.modes.is_empty() {
                return Ok(());
            }
            let teardown = self.teardown_ansi();
            let restoration = self.restore_modes();
            self.entered = false;
            teardown.and(restoration)
        }

        fn capabilities(&self) -> Capabilities {
            assumed_capabilities()
        }

        fn present(&mut self, patch: &AnsiPatch, cursor: Option<CursorSpec>) -> io::Result<()> {
            let _ = cursor;
            self.write_raw(patch.0.as_bytes())
        }

        fn wait(&mut self, deadline: Option<std::time::Instant>) -> io::Result<Vec<Event>> {
            let mut events = Vec::new();
            loop {
                if self.wake.swap(false, std::sync::atomic::Ordering::AcqRel) {
                    events.push(Event::Wake);
                    return Ok(events);
                }
                let (timeout, last) = interim_wait_slice(deadline);
                let wait = unsafe { WaitForSingleObject(self.stdin, timeout.as_millis() as u32) };
                if wait == WAIT_OBJECT_0 {
                    let mut buf = [0u8; 4096];
                    let mut read = 0u32;
                    unsafe {
                        if ReadFile(self.stdin, buf.as_mut_ptr(), buf.len() as u32, &mut read, std::ptr::null_mut()) != 0 && read > 0 {
                            self.parser.feed(&buf[..read as usize], &mut events);
                        }
                    }
                    return Ok(events);
                }
                self.parser.flush_escape(&mut events);
                if last || wait != WAIT_TIMEOUT || !events.is_empty() {
                    return Ok(events);
                }
            }
        }

        fn waker(&self) -> Waker {
            let wake = std::sync::Arc::clone(&self.wake);
            Waker::new(move || wake.store(true, std::sync::atomic::Ordering::Release))
        }

        fn copy(&mut self, text: &str) -> io::Result<()> {
            self.write_raw(osc52_copy_sequence(text).as_bytes())
        }
    }

    impl NativeTerminal {
        fn teardown_ansi(&mut self) -> io::Result<()> {
            if !self.ansi_setup_owned {
                return Ok(());
            }
            let result = self.write_raw(teardown_sequence().as_bytes());
            release_owned_terminal_cleanup(&mut self.ansi_setup_owned, result.is_ok());
            result
        }

        fn restore_modes(&mut self) -> io::Result<()> {
            unsafe {
                let restored = self.modes.restore_with(|handle, mode| SetConsoleMode(handle, mode) != 0, self.stdin, self.original_in, self.stdout, self.original_out);
                self.modes.restore_code_pages_with(|page| SetConsoleCP(page) != 0, |page| SetConsoleOutputCP(page) != 0, self.original_input_cp, self.original_output_cp);
                if restored && self.modes.is_empty() {
                    Ok(())
                } else {
                    Err(err("SetConsoleMode restore failed"))
                }
            }
        }
    }

    #[cfg(test)]
    include!("../🧪️tests/🔬️backend-native-windows-unit/🦀️.rs");

    impl NativeTerminal {
        fn write_raw(&self, bytes: &[u8]) -> io::Result<()> {
            let mut written = 0u32;
            unsafe {
                if WriteFile(self.stdout, bytes.as_ptr(), bytes.len() as u32, &mut written, std::ptr::null_mut()) == 0 {
                    return Err(err("WriteFile failed"));
                }
            }
            Ok(())
        }
    }

    impl Drop for NativeTerminal {
        fn drop(&mut self) {
            let _ = self.leave();
        }
    }
}
#[cfg(all(feature = "tui-terminal", windows))]
pub use native_windows::NativeTerminal;
