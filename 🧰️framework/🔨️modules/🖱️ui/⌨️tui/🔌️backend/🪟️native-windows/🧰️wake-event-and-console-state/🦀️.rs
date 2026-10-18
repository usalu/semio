//! 🧰️ Windows wake event and console state: the record batch and size poll, the wake event, the wait outcome, the utf16 decoder, the input record conversion and the saved console restore. See [`super`].

use super::*;

pub(super) const RECORD_BATCH: usize = 128;

pub(super) const SIZE_POLL: Duration = Duration::from_millis(250);

pub(super) fn err(message: impl Into<String>) -> io::Error {
    io::Error::other(message.into())
}

/// 🔔️ An auto-reset Win32 event other threads set to make `wait` return.
pub(super) struct WakeEvent(pub(super) OwnedHandle);

unsafe impl Send for WakeEvent {}

unsafe impl Sync for WakeEvent {}

impl WakeEvent {
    pub(super) fn new() -> io::Result<Self> {
        let handle = unsafe { CreateEventW(std::ptr::null(), 0, 0, std::ptr::null()) };
        unsafe { OwnedHandle::from_raw(handle) }.map(Self).ok_or_else(io::Error::last_os_error)
    }

    pub(super) fn set(&self) {
        unsafe {
            SetEvent(self.0.as_raw());
        }
    }
}

pub(super) struct WakeState {
    pub(super) event: WakeEvent,
    pub(super) flag: AtomicBool,
}

/// 🕰️ What ended one multi-object wait.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in super::super) enum WaitOutcome {
    Input,
    Wake,
    Timeout,
}

/// ⏱️ Waits for the console input handle or the wake event, whichever is signalled first.
pub(in super::super) fn wait_signalled(input: HANDLE, wake: HANDLE, timeout: Option<Duration>) -> io::Result<WaitOutcome> {
    let handles = [input, wake];
    let milliseconds = timeout.map_or(INFINITE, |timeout| ceil_millis(timeout).min(u64::from(INFINITE - 1)) as u32);
    let status = unsafe { WaitForMultipleObjects(2, handles.as_ptr(), 0, milliseconds) };
    match status {
        WAIT_OBJECT_0 => Ok(WaitOutcome::Input),
        status if status == WAIT_OBJECT_0 + 1 => Ok(WaitOutcome::Wake),
        WAIT_TIMEOUT => Ok(WaitOutcome::Timeout),
        _ => Err(io::Error::last_os_error()),
    }
}

/// 🔤️ Reassembles UTF-16 console key characters, including surrogate pairs, into UTF-8 bytes.
#[derive(Default)]
pub(in super::super) struct Utf16Decoder {
    high: Option<u16>,
}

impl Utf16Decoder {
    pub(in super::super) fn push(&mut self, unit: u16, out: &mut Vec<u8>) {
        let scalar = match (self.high.take(), unit) {
            (Some(high), 0xdc00..=0xdfff) => char::from_u32(0x10000 + ((u32::from(high) - 0xd800) << 10) + (u32::from(unit) - 0xdc00)),
            (_, 0xd800..=0xdbff) => {
                self.high = Some(unit);
                None
            }
            (_, 0xdc00..=0xdfff) => None,
            (_, unit) => char::from_u32(u32::from(unit)),
        };
        if let Some(scalar) = scalar {
            out.extend_from_slice(scalar.encode_utf8(&mut [0u8; 4]).as_bytes());
        }
    }
}

/// 📜️ Turns console input records into the VT byte stream the parser decodes; reports whether the window buffer was resized.
pub(in super::super) fn records_to_bytes(records: &[INPUT_RECORD], decoder: &mut Utf16Decoder, bytes: &mut Vec<u8>) -> bool {
    let mut resized = false;
    for record in records {
        match record.EventType {
            KEY_EVENT => {
                let key = unsafe { record.Event.KeyEvent };
                let control_space = key.bKeyDown != 0 && matches!(key.wVirtualKeyCode, VK_SPACE | VK_2) && key.dwControlKeyState & CONTROL_KEYS != 0;
                if ((key.bKeyDown != 0 || key.wVirtualKeyCode == VK_MENU) && key.uChar != 0) || control_space {
                    for _ in 0..key.wRepeatCount.clamp(1, 256) {
                        decoder.push(key.uChar, bytes);
                    }
                }
            }
            WINDOW_BUFFER_SIZE_EVENT => resized = true,
            _ => {}
        }
    }
    resized
}

pub(super) struct SavedConsole {
    pub(super) stdin: usize,
    pub(super) stdout: usize,
    pub(super) original_in: u32,
    pub(super) original_out: u32,
    pub(super) input_cp: u32,
    pub(super) output_cp: u32,
}

static SAVED: Mutex<Option<SavedConsole>> = Mutex::new(None);

fn lock_saved() -> std::sync::MutexGuard<'static, Option<SavedConsole>> {
    SAVED.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

pub(super) fn arm_restore(saved: SavedConsole) {
    teardown_sequence();
    *lock_saved() = Some(saved);
}

pub(super) fn disarm_restore() -> bool {
    lock_saved().take().is_some()
}

fn restore_saved(saved: &SavedConsole) {
    let bytes = teardown_sequence().as_bytes();
    let mut written = 0u32;
    unsafe {
        WriteFile(saved.stdout as HANDLE, bytes.as_ptr(), bytes.len() as u32, &mut written, std::ptr::null_mut());
        SetConsoleMode(saved.stdin as HANDLE, saved.original_in);
        SetConsoleMode(saved.stdout as HANDLE, saved.original_out);
        SetConsoleCP(saved.input_cp);
        SetConsoleOutputCP(saved.output_cp);
    }
}

pub(super) fn restore_for_panic() {
    if let Ok(mut slot) = SAVED.try_lock() {
        if let Some(saved) = slot.take() {
            restore_saved(&saved);
        }
    }
}

pub(super) unsafe extern "system" fn on_console_control(kind: u32) -> i32 {
    if matches!(kind, CTRL_BREAK_EVENT | CTRL_CLOSE_EVENT | CTRL_LOGOFF_EVENT | CTRL_SHUTDOWN_EVENT) {
        if let Some(saved) = lock_saved().take() {
            restore_saved(&saved);
        }
    }
    0
}
