//! 📟️ Windows terminal backend and drop: the terminal backend implementation, the remaining `NativeTerminal` operations, the unit tests include and the drop. See [`super`].

use super::*;

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
        setup_sequence();
        arm_restore(SavedConsole { stdin: self.stdin as usize, stdout: self.stdout as usize, original_in: self.original_in, original_out: self.original_out, input_cp: self.original_input_cp, output_cp: self.original_output_cp });
        self.armed = true;
        unsafe {
            if SetConsoleOutputCP(65001) == 0 {
                self.release_restore();
                return Err(err("SetConsoleOutputCP failed"));
            }
            self.modes.output_cp_changed = true;
            if SetConsoleCP(65001) == 0 {
                self.release_restore();
                let _ = self.restore_modes();
                return Err(err("SetConsoleCP failed"));
            }
            self.modes.input_cp_changed = true;
            let out_mode = self.original_out | ENABLE_VIRTUAL_TERMINAL_PROCESSING | DISABLE_NEWLINE_AUTO_RETURN;
            let in_mode = (self.original_in | ENABLE_VIRTUAL_TERMINAL_INPUT | ENABLE_WINDOW_INPUT | ENABLE_EXTENDED_FLAGS) & !(ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT | ENABLE_PROCESSED_INPUT | ENABLE_QUICK_EDIT_MODE);
            if SetConsoleMode(self.stdout, out_mode) == 0 {
                self.release_restore();
                let _ = self.restore_modes();
                return Err(err("SetConsoleMode failed"));
            }
            self.modes.stdout_changed = true;
            if SetConsoleMode(self.stdin, in_mode) == 0 {
                self.release_restore();
                return match self.restore_modes() {
                    Ok(()) => Err(err("SetConsoleMode failed")),
                    Err(_) => Err(err("SetConsoleMode failed and rollback is pending")),
                };
            }
            self.modes.stdin_changed = true;
        }
        self.ansi_setup_owned = true;
        if let Err(error) = self.write_raw(setup_sequence().as_bytes()) {
            self.release_restore();
            let teardown = self.teardown_ansi();
            let restoration = self.restore_modes();
            self.entered = false;
            return match (teardown, restoration) {
                (Ok(()), Ok(())) => Err(error),
                _ => Err(err("Terminal setup write failed and rollback is pending")),
            };
        }
        self.presenter.reset();
        if let Ok(size) = self.size() {
            self.resize.baseline(size);
        }
        unsafe {
            SetConsoleCtrlHandler(Some(on_console_control), 1);
        }
        panic_restore::claim();
        panic_restore::install(restore_for_panic);
        self.entered = true;
        Ok(())
    }

    fn leave(&mut self) -> io::Result<()> {
        if !self.entered && !self.ansi_setup_owned && self.modes.is_empty() {
            return Ok(());
        }
        unsafe {
            SetConsoleCtrlHandler(Some(on_console_control), 0);
        }
        if !self.release_restore() {
            self.forget_external_restore();
            return Ok(());
        }
        panic_restore::release();
        let teardown = self.teardown_ansi();
        let restoration = self.restore_modes();
        self.entered = false;
        teardown.and(restoration)
    }

    fn capabilities(&self) -> Capabilities {
        self.presenter.capabilities()
    }

    fn present(&mut self, patch: &AnsiPatch, cursor: Option<CursorSpec>) -> io::Result<()> {
        let frame = self.presenter.compose(patch, cursor);
        if frame.is_empty() {
            return Ok(());
        }
        self.write_raw(frame.as_bytes())
    }

    fn wait(&mut self, deadline: Option<Instant>) -> io::Result<Vec<Event>> {
        let mut events = Vec::new();
        loop {
            let now = Instant::now();
            let timeout = Some(next_timeout(deadline, &self.input, now).map_or(SIZE_POLL, |timeout| timeout.min(SIZE_POLL)));
            match wait_signalled(self.stdin, self.wake.event.0.as_raw(), timeout)? {
                WaitOutcome::Input => self.read_records(&mut events)?,
                WaitOutcome::Wake => {
                    if self.wake.flag.swap(false, Ordering::AcqRel) {
                        events.push(Event::Wake);
                    }
                }
                WaitOutcome::Timeout => {}
            }
            self.poll_size(&mut events);
            let after = Instant::now();
            self.input.expire_due(after, &mut events);
            if !events.is_empty() || deadline.is_some_and(|deadline| after >= deadline) {
                return Ok(events);
            }
        }
    }

    fn waker(&self) -> Waker {
        let wake = Arc::clone(&self.wake);
        Waker::new(move || {
            wake.flag.store(true, Ordering::Release);
            wake.event.set();
        })
    }

    fn copy(&mut self, text: &str) -> io::Result<()> {
        if !osc52_payload_check(text)? {
            return Ok(());
        }
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

    fn write_raw(&self, bytes: &[u8]) -> io::Result<()> {
        let mut offset = 0usize;
        while offset < bytes.len() {
            let mut written = 0u32;
            let chunk = (bytes.len() - offset).min(1 << 20) as u32;
            unsafe {
                if WriteFile(self.stdout, bytes[offset..].as_ptr(), chunk, &mut written, std::ptr::null_mut()) == 0 {
                    return Err(err("WriteFile failed"));
                }
            }
            if written == 0 {
                return Err(err("WriteFile wrote nothing"));
            }
            offset += written as usize;
        }
        Ok(())
    }
}

#[cfg(test)]
include!("../../../🧪️tests/🔬️backend-native-windows-unit/🦀️.rs");

impl Drop for NativeTerminal {
    fn drop(&mut self) {
        let _ = self.leave();
    }
}
