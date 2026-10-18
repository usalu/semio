//! 🎚️ Windows native terminal and console modes: the `NativeTerminal` state, the console mode ownership and the `NativeTerminal` setup and leave. See [`super`].

use super::*;

/// 🪟️ VT-mode terminal backend using the console input buffer, a wake event and the private first-party Win32 ABI.
///
/// The dashboard main loop owns this backend. `wait` blocks on the console input handle and the wake event together,
/// reads input records (never a blocking byte read), and turns window-buffer-size records into `Event::Resize`.
pub struct NativeTerminal {
    pub(super) stdin: HANDLE,
    pub(super) stdout: HANDLE,
    _console_input: std::fs::File,
    _console_output: std::fs::File,
    pub(super) original_in: u32,
    pub(super) original_out: u32,
    pub(super) original_input_cp: u32,
    pub(super) original_output_cp: u32,
    pub(super) input: InputPipeline,
    pub(super) presenter: FramePresenter,
    pub(super) resize: ResizeTracker,
    utf16: Utf16Decoder,
    pub(super) wake: Arc<WakeState>,
    pub(super) armed: bool,
    pub(super) entered: bool,
    pub(super) ansi_setup_owned: bool,
    pub(super) modes: ConsoleModeOwnership,
}

#[derive(Default)]
pub(super) struct ConsoleModeOwnership {
    pub(super) stdin_changed: bool,
    pub(super) stdout_changed: bool,
    pub(super) input_cp_changed: bool,
    pub(super) output_cp_changed: bool,
}

impl ConsoleModeOwnership {
    pub(super) fn restore_with(&mut self, mut set_mode: impl FnMut(HANDLE, u32) -> bool, stdin: HANDLE, original_in: u32, stdout: HANDLE, original_out: u32) -> bool {
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

    pub(super) fn restore_code_pages_with(&mut self, mut input: impl FnMut(u32) -> bool, mut output: impl FnMut(u32) -> bool, original_input: u32, original_output: u32) {
        if self.input_cp_changed && input(original_input) {
            self.input_cp_changed = false;
        }
        if self.output_cp_changed && output(original_output) {
            self.output_cp_changed = false;
        }
    }

    pub(super) fn is_empty(&self) -> bool {
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
            let original_input_cp = GetConsoleCP();
            let original_output_cp = GetConsoleOutputCP();
            if original_input_cp == 0 || original_output_cp == 0 {
                return Err(err("GetConsoleCP failed").into());
            }
            let wake = Arc::new(WakeState { event: WakeEvent::new()?, flag: AtomicBool::new(false) });
            let capabilities = detect_capabilities(&process_environment, true);
            Ok(Self {
                stdin,
                stdout,
                original_input_cp,
                original_output_cp,
                _console_input: console_input,
                _console_output: console_output,
                original_in,
                original_out,
                input: InputPipeline::new(),
                presenter: FramePresenter::new(capabilities),
                resize: ResizeTracker::default(),
                utf16: Utf16Decoder::default(),
                wake,
                armed: false,
                entered: false,
                ansi_setup_owned: false,
                modes: ConsoleModeOwnership::default(),
            })
        }
    }

    pub(super) fn release_restore(&mut self) -> bool {
        if !self.armed {
            return true;
        }
        self.armed = false;
        disarm_restore()
    }

    pub(super) fn forget_external_restore(&mut self) {
        self.entered = false;
        self.ansi_setup_owned = false;
        self.modes = ConsoleModeOwnership::default();
        panic_restore::release();
    }

    pub(super) fn poll_size(&mut self, events: &mut Vec<Event>) {
        if let Some(event) = self.size().ok().and_then(|size| self.resize.observe(size)) {
            events.push(event);
        }
    }

    pub(super) fn read_records(&mut self, events: &mut Vec<Event>) -> io::Result<()> {
        let mut pending = 0u32;
        if unsafe { GetNumberOfConsoleInputEvents(self.stdin, &mut pending) } == 0 {
            return Err(io::Error::last_os_error());
        }
        if pending == 0 {
            return Ok(());
        }
        let mut records = [INPUT_RECORD::default(); RECORD_BATCH];
        let mut read = 0u32;
        if unsafe { ReadConsoleInputW(self.stdin, records.as_mut_ptr(), pending.min(RECORD_BATCH as u32), &mut read) } == 0 {
            return Err(io::Error::last_os_error());
        }
        let mut bytes = Vec::new();
        let resized = records_to_bytes(&records[..read as usize], &mut self.utf16, &mut bytes);
        if !bytes.is_empty() {
            self.input.feed(&bytes, events);
        }
        if resized {
            self.poll_size(events);
        }
        Ok(())
    }
}
