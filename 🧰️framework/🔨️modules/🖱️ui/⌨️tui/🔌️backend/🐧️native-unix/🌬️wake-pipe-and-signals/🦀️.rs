//! 🌬️ Unix wake pipe and signals: the read burst, the mode restoration, the wake pipe, the saved termios and restore arming, the resize and terminate handlers, the signal guard and the readiness waits. See [`super`].

use super::*;

pub(super) const READ_BURST: usize = 1 << 18;

pub(super) fn err(message: impl Into<String>) -> io::Error {
    io::Error::other(message.into())
}

pub(super) fn retry_mode_restoration(raw_mode_entered: &mut bool, restore: impl FnOnce() -> bool) -> bool {
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

pub(super) fn write_byte(fd: RawFd) {
    let byte = 1u8;
    unsafe {
        libc::write(fd, (&byte as *const u8).cast(), 1);
    }
}

/// 🪢️ A non-blocking self-pipe: any thread or signal handler writes a byte to make `wait` return.
pub(super) struct WakePipe {
    pub(super) read: RawFd,
    pub(super) write: RawFd,
}

impl WakePipe {
    pub(super) fn new() -> io::Result<Self> {
        let mut fds = [0 as libc::c_int; 2];
        if unsafe { libc::pipe(fds.as_mut_ptr()) } != 0 {
            return Err(io::Error::last_os_error());
        }
        for fd in fds {
            unsafe {
                let status = libc::fcntl(fd, libc::F_GETFL);
                libc::fcntl(fd, libc::F_SETFL, status | libc::O_NONBLOCK);
                let descriptor = libc::fcntl(fd, libc::F_GETFD);
                libc::fcntl(fd, libc::F_SETFD, descriptor | libc::FD_CLOEXEC);
            }
        }
        Ok(Self { read: fds[0], write: fds[1] })
    }

    pub(super) fn drain(&self) {
        let mut buf = [0u8; 64];
        loop {
            let n = unsafe { libc::read(self.read, buf.as_mut_ptr().cast(), buf.len()) };
            if n <= 0 || (n as usize) < buf.len() {
                break;
            }
        }
    }
}

impl Drop for WakePipe {
    fn drop(&mut self) {
        unsafe {
            libc::close(self.read);
            libc::close(self.write);
        }
    }
}

pub(super) struct WakeState {
    pub(super) pipe: WakePipe,
    pub(super) flag: AtomicBool,
}

pub(super) static SIGNAL_PIPE: AtomicI32 = AtomicI32::new(-1);

pub(super) static PENDING_RESIZE: AtomicBool = AtomicBool::new(false);

static RESTORE_FD: AtomicI32 = AtomicI32::new(-1);

struct SavedTermios(UnsafeCell<MaybeUninit<libc::termios>>);

unsafe impl Sync for SavedTermios {}

static SAVED_TERMIOS: SavedTermios = SavedTermios(UnsafeCell::new(MaybeUninit::uninit()));

pub(super) fn arm_restore(fd: RawFd, original: &libc::termios) {
    teardown_sequence();
    unsafe {
        (*SAVED_TERMIOS.0.get()).write(*original);
    }
    RESTORE_FD.store(fd, Ordering::Release);
}

pub(super) fn disarm_restore() -> bool {
    RESTORE_FD.swap(-1, Ordering::AcqRel) >= 0
}

pub(super) fn restore_now() -> bool {
    let fd = RESTORE_FD.swap(-1, Ordering::AcqRel);
    if fd < 0 {
        return false;
    }
    let bytes = teardown_sequence().as_bytes();
    let mut written = 0;
    while written < bytes.len() {
        let n = unsafe { libc::write(fd, bytes[written..].as_ptr().cast(), bytes.len() - written) };
        if n <= 0 {
            break;
        }
        written += n as usize;
    }
    unsafe {
        libc::tcsetattr(fd, libc::TCSANOW, (*SAVED_TERMIOS.0.get()).as_ptr());
    }
    true
}

pub(super) fn restore_for_panic() {
    restore_now();
}

extern "C" fn on_resize(_signal: libc::c_int) {
    PENDING_RESIZE.store(true, Ordering::Release);
    let fd = SIGNAL_PIPE.load(Ordering::Acquire);
    if fd >= 0 {
        write_byte(fd);
    }
}

extern "C" fn on_terminate(signal: libc::c_int) {
    restore_now();
    unsafe {
        libc::signal(signal, libc::SIG_DFL);
        libc::raise(signal);
    }
}

/// 🚦️ Installs the resize and terminate handlers and puts back the previous dispositions on drop.
pub(super) struct SignalGuard {
    previous: Vec<(libc::c_int, libc::sigaction)>,
}

impl SignalGuard {
    pub(super) fn install() -> io::Result<Self> {
        let table: [(libc::c_int, extern "C" fn(libc::c_int)); 5] = [(libc::SIGWINCH, on_resize), (libc::SIGINT, on_terminate), (libc::SIGTERM, on_terminate), (libc::SIGHUP, on_terminate), (libc::SIGQUIT, on_terminate)];
        let mut guard = Self { previous: Vec::new() };
        for (signal, handler) in table {
            unsafe {
                let mut old: libc::sigaction = std::mem::zeroed();
                if libc::sigaction(signal, std::ptr::null(), &mut old) != 0 {
                    return Err(io::Error::last_os_error());
                }
                if signal != libc::SIGWINCH && old.sa_sigaction == libc::SIG_IGN {
                    continue;
                }
                let mut action: libc::sigaction = std::mem::zeroed();
                action.sa_sigaction = handler as usize as libc::sighandler_t;
                libc::sigemptyset(&mut action.sa_mask);
                action.sa_flags = libc::SA_RESTART;
                if libc::sigaction(signal, &action, std::ptr::null_mut()) != 0 {
                    return Err(io::Error::last_os_error());
                }
                guard.previous.push((signal, old));
            }
        }
        Ok(guard)
    }
}

impl Drop for SignalGuard {
    fn drop(&mut self) {
        for (signal, old) in &self.previous {
            unsafe {
                libc::sigaction(*signal, old, std::ptr::null_mut());
            }
        }
    }
}

/// 👂️ Blocks until `a` or `b` is readable or the timeout passes; `(false, false)` on timeout or interruption.
#[cfg(not(target_os = "macos"))]
pub(in super::super) fn wait_readable(a: RawFd, b: RawFd, timeout: Option<Duration>) -> io::Result<(bool, bool)> {
    let mut fds = [libc::pollfd { fd: a, events: libc::POLLIN, revents: 0 }, libc::pollfd { fd: b, events: libc::POLLIN, revents: 0 }];
    let milliseconds = timeout.map_or(-1, |timeout| i32::try_from(ceil_millis(timeout)).unwrap_or(i32::MAX));
    let ready = unsafe { libc::poll(fds.as_mut_ptr(), 2, milliseconds) };
    if ready < 0 {
        let error = io::Error::last_os_error();
        return if error.kind() == io::ErrorKind::Interrupted { Ok((false, false)) } else { Err(error) };
    }
    let hit = |entry: &libc::pollfd| entry.revents & (libc::POLLIN | libc::POLLHUP | libc::POLLERR) != 0;
    Ok((hit(&fds[0]), hit(&fds[1])))
}

/// 🎧️ Blocks until `a` or `b` is readable or the timeout passes; `(false, false)` on timeout or interruption.
#[cfg(target_os = "macos")]
pub(in super::super) fn wait_readable(a: RawFd, b: RawFd, timeout: Option<Duration>) -> io::Result<(bool, bool)> {
    let highest = a.max(b);
    if highest as usize >= libc::FD_SETSIZE {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "file descriptor exceeds FD_SETSIZE"));
    }
    let mut set: libc::fd_set = unsafe { std::mem::zeroed() };
    unsafe {
        libc::FD_ZERO(&mut set);
        libc::FD_SET(a, &mut set);
        libc::FD_SET(b, &mut set);
    }
    let mut limit = libc::timeval { tv_sec: 0, tv_usec: 0 };
    let limit_ptr = match timeout {
        Some(timeout) => {
            limit = libc::timeval { tv_sec: timeout.as_secs() as libc::time_t, tv_usec: timeout.subsec_micros() as libc::suseconds_t };
            &mut limit as *mut libc::timeval
        }
        None => std::ptr::null_mut(),
    };
    let ready = unsafe { libc::select(highest + 1, &mut set, std::ptr::null_mut(), std::ptr::null_mut(), limit_ptr) };
    if ready < 0 {
        let error = io::Error::last_os_error();
        return if error.kind() == io::ErrorKind::Interrupted { Ok((false, false)) } else { Err(error) };
    }
    unsafe { Ok((libc::FD_ISSET(a, &set), libc::FD_ISSET(b, &set))) }
}
