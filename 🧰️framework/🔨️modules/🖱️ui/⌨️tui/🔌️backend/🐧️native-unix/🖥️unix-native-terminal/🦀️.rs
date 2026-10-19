//! 🖥️ Unix native terminal: the `NativeTerminal` with its setup, leave, size and event reads, the terminal backend implementation and the drop. See [`super`].

use super::*;

/// 🐧️ Raw-mode terminal backend for unix (macOS/Linux), driven by `libc` alone.
pub struct NativeTerminal {
    pub(super) fd: RawFd,
    terminal: std::fs::File,
    pub(super) original: libc::termios,
    pub(super) input: InputPipeline,
    pub(super) presenter: FramePresenter,
    resize: ResizeTracker,
    pub(super) wake: Arc<WakeState>,
    signals: Option<SignalGuard>,
    armed: bool,
    entered: bool,
    pub(super) ansi_setup_owned: bool,
    pub(super) raw_mode_entered: bool,
}

impl NativeTerminal {
    pub fn new() -> Result<Self, BackendError> {
        use std::os::fd::AsRawFd;
        let terminal = std::fs::OpenOptions::new().read(true).write(true).open("/dev/tty").or_else(|original| {
            use std::os::unix::ffi::OsStrExt;
            let mut device = [0 as libc::c_char; 4096];
            if unsafe { libc::ttyname_r(libc::STDIN_FILENO, device.as_mut_ptr(), device.len()) } != 0 {
                return Err(original);
            }
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
        let wake = Arc::new(WakeState { pipe: WakePipe::new()?, flag: AtomicBool::new(false) });
        let capabilities = detect_capabilities(&process_environment, false);
        Ok(Self { fd, terminal, original, input: InputPipeline::new(), presenter: FramePresenter::new(capabilities), resize: ResizeTracker::default(), wake, signals: None, armed: false, entered: false, ansi_setup_owned: false, raw_mode_entered: false })
    }

    fn release_restore(&mut self) -> bool {
        if !self.armed {
            return true;
        }
        self.armed = false;
        disarm_restore()
    }

    fn forget_external_restore(&mut self) {
        self.entered = false;
        self.ansi_setup_owned = false;
        self.raw_mode_entered = false;
        self.signals = None;
        SIGNAL_PIPE.store(-1, Ordering::Release);
        panic_restore::release();
    }

    fn drain_signals(&mut self, events: &mut Vec<Event>) {
        if PENDING_RESIZE.swap(false, Ordering::AcqRel) {
            if let Some(event) = self.size().ok().and_then(|size| self.resize.observe(size)) {
                events.push(event);
            }
        }
    }

    fn read_available(&mut self, events: &mut Vec<Event>) -> io::Result<()> {
        let mut total = 0usize;
        loop {
            let mut buf = [0u8; 4096];
            let n = unsafe { libc::read(self.fd, buf.as_mut_ptr().cast(), buf.len()) };
            if n > 0 {
                self.input.feed(&buf[..n as usize], events);
                total += n as usize;
                if (n as usize) < buf.len() || total >= READ_BURST {
                    return Ok(());
                }
            } else if n == 0 {
                return if total == 0 { Err(io::Error::new(io::ErrorKind::UnexpectedEof, "terminal input closed")) } else { Ok(()) };
            } else {
                let error = io::Error::last_os_error();
                match error.kind() {
                    io::ErrorKind::Interrupted => continue,
                    io::ErrorKind::WouldBlock => return Ok(()),
                    _ => return Err(error),
                }
            }
        }
    }
}

impl TerminalBackend for NativeTerminal {
            fn width_mode(&self) -> crate::tui::text::WidthMode { crate::tui::text::WidthMode::WindowsConsole }

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
        setup_sequence();
        arm_restore(self.fd, &self.original);
        self.armed = true;
        unsafe {
            if libc::tcsetattr(self.fd, libc::TCSANOW, &raw) != 0 {
                self.release_restore();
                return Err(err("tcsetattr failed"));
            }
        }
        self.raw_mode_entered = true;
        self.ansi_setup_owned = true;
        if let Err(error) = self.terminal.write_all(setup_sequence().as_bytes()).and_then(|()| self.terminal.flush()).map_err(|e| err(e.to_string())) {
            self.release_restore();
            let teardown = self.teardown_ansi();
            let restoration = self.restore_mode();
            self.entered = false;
            return match (teardown, restoration) {
                (Ok(()), Ok(())) => Err(error),
                _ => Err(err("Terminal setup write failed and terminal rollback is pending")),
            };
        }
        self.presenter.reset();
        if let Ok(size) = self.size() {
            self.resize.baseline(size);
        }
        PENDING_RESIZE.store(false, Ordering::Release);
        SIGNAL_PIPE.store(self.wake.pipe.write, Ordering::Release);
        self.signals = SignalGuard::install().ok();
        panic_restore::claim();
        panic_restore::install(restore_for_panic);
        self.entered = true;
        Ok(())
    }

    fn leave(&mut self) -> io::Result<()> {
        if !self.entered && !self.ansi_setup_owned && !self.raw_mode_entered {
            return Ok(());
        }
        if !self.release_restore() {
            self.forget_external_restore();
            return Ok(());
        }
        SIGNAL_PIPE.store(-1, Ordering::Release);
        self.signals = None;
        panic_restore::release();
        let teardown = self.teardown_ansi();
        let restoration = self.restore_mode();
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
        self.terminal.write_all(frame.as_bytes()).map_err(|e| err(e.to_string()))?;
        self.terminal.flush().map_err(|e| err(e.to_string()))
    }

    fn wait(&mut self, deadline: Option<Instant>) -> io::Result<Vec<Event>> {
        let mut events = Vec::new();
        loop {
            let now = Instant::now();
            let timeout = next_timeout(deadline, &self.input, now);
            let (tty_ready, wake_ready) = wait_readable(self.fd, self.wake.pipe.read, timeout)?;
            if wake_ready {
                self.wake.pipe.drain();
            }
            self.drain_signals(&mut events);
            if self.wake.flag.swap(false, Ordering::AcqRel) {
                events.push(Event::Wake);
            }
            if tty_ready {
                self.read_available(&mut events)?;
            }
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
            write_byte(wake.pipe.write);
        })
    }

    fn copy(&mut self, text: &str) -> io::Result<()> {
        if !osc52_payload_check(text)? {
            return Ok(());
        }
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
include!("../../../🧪️tests/🔬️backend-native-unix-unit/🦀️.rs");

impl Drop for NativeTerminal {
    fn drop(&mut self) {
        let _ = self.leave();
    }
}
