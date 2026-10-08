use std::io::Write;
use std::path::Path;

/// 📐 Pseudo-terminal geometry in character cells.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PtySize {
    pub cols: u16,
    pub rows: u16,
}

/// 💥 Failure from pseudo-terminal spawn or I/O.
#[derive(Debug)]
pub struct PtyError {
    pub message: String,
}

impl std::fmt::Display for PtyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for PtyError {}

fn err(message: impl Into<String>) -> PtyError {
    PtyError { message: message.into() }
}

/// 📥 What one non-blocking read of the child's output found.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PtyRead {
    Data(usize),
    Empty,
    Closed,
}

/// 🪜 The escalation steps of stopping a child: ask its foreground job, ask its whole tree, end its whole tree.
///
/// @see https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap11.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopStage {
    Interrupt,
    Terminate,
    Kill,
}

/// 🌀️ Starts an independent process in its own session without retaining the caller's terminal or output pipes.
#[cfg(all(unix, not(target_arch = "wasm32")))]
pub fn spawn_detached(cmd: &str, args: &[&str], remove_env: &[&str], cwd: &Path) -> Result<u32, PtyError> {
    use std::os::unix::process::CommandExt;
    let mut command = std::process::Command::new(cmd);
    command.args(args).current_dir(cwd).stdin(std::process::Stdio::null()).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null());
    for key in remove_env { command.env_remove(key); }
    unsafe {
        command.pre_exec(|| if libc::setsid() < 0 { Err(std::io::Error::last_os_error()) } else { Ok(()) });
    }
    command.spawn().map(|child| child.id()).map_err(|error| err(error.to_string()))
}

/// 🪪 The numeric identity of the user this process runs as; it scopes per-user runtime directories.
#[cfg(all(unix, not(target_arch = "wasm32")))]
pub fn user_id() -> u32 {
    unsafe { libc::getuid() }
}

/// 🛎️ Counts interrupt and termination requests instead of ending the process on the first one, so a
/// command can turn them into its documented cancellation steps.
///
/// @see https://man7.org/linux/man-pages/man7/signal-safety.7.html
#[cfg(all(unix, not(target_arch = "wasm32")))]
pub fn interrupts() -> &'static std::sync::atomic::AtomicUsize {
    static COUNT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    static INSTALL: std::sync::Once = std::sync::Once::new();
    extern "C" fn count(_signal: libc::c_int) {
        COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    INSTALL.call_once(|| unsafe {
        let mut action: libc::sigaction = std::mem::zeroed();
        action.sa_sigaction = count as extern "C" fn(libc::c_int) as usize;
        action.sa_flags = libc::SA_RESTART;
        libc::sigemptyset(&mut action.sa_mask);
        for signal in [libc::SIGINT, libc::SIGTERM, libc::SIGHUP] { libc::sigaction(signal, &action, std::ptr::null_mut()); }
    });
    &COUNT
}

/// ⏳ Readiness of raw descriptors: one blocking wait for the many pseudo-terminals and sockets a
/// single thread serves.
///
/// @see https://pubs.opengroup.org/onlinepubs/9699919799/functions/poll.html
#[cfg(all(unix, not(target_arch = "wasm32")))]
pub mod readiness {
    /// 🔭 One descriptor a wait observes, and what the wait found on it.
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    pub struct Interest {
        pub fd: i32,
        pub read: bool,
        pub write: bool,
        pub readable: bool,
        pub writable: bool,
        pub closed: bool,
    }

    impl Interest {
        pub fn new(fd: i32, read: bool, write: bool) -> Self {
            Self { fd, read, write, ..Self::default() }
        }
    }

    /// ⏱️ Blocks until a descriptor is ready or the timeout passes; an interrupted wait reports nothing ready.
    pub fn wait(interests: &mut [Interest], timeout: Option<std::time::Duration>) -> std::io::Result<usize> {
        let mut fds: Vec<libc::pollfd> = interests.iter().map(|interest| libc::pollfd { fd: interest.fd, events: if interest.read { libc::POLLIN } else { 0 } | if interest.write { libc::POLLOUT } else { 0 }, revents: 0 }).collect();
        let millis = timeout.map_or(-1, |timeout| timeout.as_millis().min(i32::MAX as u128).max(u128::from(!timeout.is_zero())) as libc::c_int);
        let ready = unsafe { libc::poll(fds.as_mut_ptr(), fds.len() as libc::nfds_t, millis) };
        if ready < 0 {
            let error = std::io::Error::last_os_error();
            return if error.kind() == std::io::ErrorKind::Interrupted { Ok(0) } else { Err(error) };
        }
        for (interest, fd) in interests.iter_mut().zip(&fds) {
            interest.closed = fd.revents & (libc::POLLHUP | libc::POLLERR | libc::POLLNVAL) != 0;
            interest.readable = fd.revents & libc::POLLIN != 0 || interest.closed && interest.read;
            interest.writable = fd.revents & libc::POLLOUT != 0;
        }
        Ok(ready as usize)
    }
}

#[cfg(all(unix, not(target_arch = "wasm32")))]
mod unix_impl {
    use super::*;
    use std::fs as system_fs;
    use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
    use std::os::unix::process::{CommandExt, ExitStatusExt};
    use std::process as system_process;

    /// 🧬 One process of the child's tree as it was last seen: identity plus the group and session
    /// that tell a recycled identifier from the original.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Member {
        pid: libc::pid_t,
        group: libc::pid_t,
        session: libc::pid_t,
    }

    /// 🧵 Unix PTY master plus child process.
    pub struct Pty {
        master: system_fs::File,
        child: system_process::Child,
        code: Option<i32>,
        members: Vec<Member>,
        swept: bool,
    }

    impl Pty {
        /// 🌱 Starts `cmd` on a new pseudo-terminal with this process's environment, `env` added and `remove_env` dropped.
        pub fn spawn(cmd: &str, args: &[&str], env: &[(&str, &str)], remove_env: &[&str], cwd: Option<&Path>, size: PtySize) -> Result<Self, PtyError> {
            let mut command = system_process::Command::new(cmd);
            command.args(args);
            for key in remove_env { command.env_remove(key); }
            for (key, value) in env { command.env(key, value); }
            Self::start(command, cwd, size)
        }

        /// 🧾 Starts `cmd` on a new pseudo-terminal with exactly `env` as its environment; the program is
        /// looked up on the `PATH` of that environment.
        pub fn spawn_exact(cmd: &str, args: &[&str], env: &[(String, String)], cwd: Option<&Path>, size: PtySize) -> Result<Self, PtyError> {
            let mut command = system_process::Command::new(cmd);
            command.args(args).env_clear();
            for (key, value) in env { command.env(key, value); }
            Self::start(command, cwd, size)
        }

        fn start(mut command: system_process::Command, cwd: Option<&Path>, size: PtySize) -> Result<Self, PtyError> {
            let mut master: RawFd = -1;
            let mut slave: RawFd = -1;
            let mut ws: libc::winsize = unsafe { std::mem::zeroed() };
            ws.ws_col = size.cols;
            ws.ws_row = size.rows;
            unsafe {
                if libc::openpty(&mut master, &mut slave, std::ptr::null_mut(), std::ptr::null_mut(), &mut ws) != 0 {
                    return Err(err(format!("openpty failed: {}", std::io::Error::last_os_error())));
                }
            }
            let close = || unsafe {
                libc::close(master);
                libc::close(slave);
            };
            let flags = unsafe { libc::fcntl(master, libc::F_GETFL) };
            if flags < 0 || unsafe { libc::fcntl(master, libc::F_SETFL, flags | libc::O_NONBLOCK) } != 0 || unsafe { libc::fcntl(master, libc::F_SETFD, libc::FD_CLOEXEC) } != 0 || unsafe { libc::fcntl(slave, libc::F_SETFD, libc::FD_CLOEXEC) } != 0 {
                close();
                return Err(err("pseudo-terminal descriptor setup failed"));
            }
            if let Some(dir) = cwd {
                command.current_dir(dir);
            }
            command.stdin(system_process::Stdio::null());
            command.stdout(system_process::Stdio::null());
            command.stderr(system_process::Stdio::null());
            unsafe {
                command.pre_exec(move || {
                    if libc::setsid() < 0 {
                        return Err(std::io::Error::last_os_error());
                    }
                    if libc::ioctl(slave, libc::TIOCSCTTY as _, 0) < 0 {
                        return Err(std::io::Error::last_os_error());
                    }
                    if libc::dup2(slave, libc::STDIN_FILENO) < 0 || libc::dup2(slave, libc::STDOUT_FILENO) < 0 || libc::dup2(slave, libc::STDERR_FILENO) < 0 {
                        return Err(std::io::Error::last_os_error());
                    }
                    if slave <= libc::STDERR_FILENO && libc::fcntl(slave, libc::F_SETFD, 0) != 0 {
                        return Err(std::io::Error::last_os_error());
                    }
                    Ok(())
                });
            }
            let child = match command.spawn() {
                Ok(child) => child,
                Err(e) => {
                    close();
                    return Err(err(format!("spawn failed: {e}")));
                }
            };
            unsafe {
                libc::close(slave);
            }
            let master = unsafe { system_fs::File::from_raw_fd(master) };
            Ok(Self { master, child, code: None, members: Vec::new(), swept: false })
        }

        pub fn resize(&mut self, size: PtySize) -> Result<(), PtyError> {
            let mut ws: libc::winsize = unsafe { std::mem::zeroed() };
            ws.ws_col = size.cols;
            ws.ws_row = size.rows;
            unsafe {
                if libc::ioctl(self.master.as_raw_fd(), libc::TIOCSWINSZ, &ws) != 0 {
                    return Err(err(format!("TIOCSWINSZ failed: {}", std::io::Error::last_os_error())));
                }
            }
            Ok(())
        }

        pub fn writer(&mut self) -> &mut impl Write {
            self
        }

        /// 🔌 The master descriptor, for a readiness wait over many pseudo-terminals.
        pub fn raw_fd(&self) -> RawFd {
            self.master.as_raw_fd()
        }

        /// 📖️ Reads what the child has written so far without waiting for more.
        pub fn read(&mut self, buf: &mut [u8]) -> PtyRead {
            loop {
                let n = unsafe { libc::read(self.master.as_raw_fd(), buf.as_mut_ptr() as *mut _, buf.len()) };
                if n > 0 {
                    return PtyRead::Data(n as usize);
                }
                if n == 0 {
                    return PtyRead::Closed;
                }
                match std::io::Error::last_os_error().kind() {
                    std::io::ErrorKind::WouldBlock => return PtyRead::Empty,
                    std::io::ErrorKind::Interrupted => continue,
                    _ => return PtyRead::Closed,
                }
            }
        }

        pub fn try_read(&mut self, buf: &mut [u8]) -> Result<usize, PtyError> {
            Ok(match self.read(buf) { PtyRead::Data(count) => count, PtyRead::Empty | PtyRead::Closed => 0 })
        }

        /// 📤 Hands the child as much input as its terminal accepts right now and answers how much that was.
        pub fn try_write(&mut self, data: &[u8]) -> Result<usize, PtyError> {
            loop {
                let n = unsafe { libc::write(self.master.as_raw_fd(), data.as_ptr() as *const _, data.len()) };
                if n >= 0 {
                    return Ok(n as usize);
                }
                let error = std::io::Error::last_os_error();
                match error.kind() {
                    std::io::ErrorKind::WouldBlock => return Ok(0),
                    std::io::ErrorKind::Interrupted => continue,
                    _ => return Err(err(format!("write failed: {error}"))),
                }
            }
        }

        /// ✍️ Writes all of `data`, waiting while the terminal is full; a child that accepts nothing for ten seconds fails the write.
        pub fn write_all(&mut self, data: &[u8]) -> Result<(), PtyError> {
            let mut rest = data;
            let mut stalled = std::time::Instant::now();
            while !rest.is_empty() {
                let written = self.try_write(rest)?;
                if written > 0 {
                    rest = &rest[written..];
                    stalled = std::time::Instant::now();
                    continue;
                }
                if stalled.elapsed() >= std::time::Duration::from_secs(10) {
                    return Err(err("pseudo-terminal input stalled"));
                }
                let mut interest = [readiness::Interest::new(self.master.as_raw_fd(), false, true)];
                readiness::wait(&mut interest, Some(std::time::Duration::from_millis(100))).map_err(|error| err(error.to_string()))?;
            }
            Ok(())
        }

        /// 🏁 The exit code once the child has ended; a signal death reports `128 + signal`.
        pub fn try_wait(&mut self) -> Result<Option<i32>, PtyError> {
            if self.code.is_some() {
                return Ok(self.code);
            }
            match self.child.try_wait() {
                Ok(Some(status)) => {
                    self.code = Some(exit_code(status));
                    Ok(self.code)
                }
                Ok(None) => Ok(None),
                Err(e) => Err(err(format!("try_wait failed: {e}"))),
            }
        }

        pub fn pid(&self) -> u32 {
            self.child.id()
        }

        /// 🛬 Tells the terminal that its child has ended; nothing to do on Unix, where the master reports the end itself.
        pub fn finish(&mut self) {}

        /// 🦇 Whether the child is a batch job; never on Unix.
        pub fn batch(&self) -> bool {
            false
        }

        /// 🧗️ Applies one stop stage: `Interrupt` signals the terminal's foreground job the way Ctrl+C
        /// does, `Terminate` asks every process of the tree to end, `Kill` ends them. Answers whether the
        /// stage reached a process.
        pub fn signal(&mut self, stage: StopStage) -> Result<bool, PtyError> {
            let leader = self.child.id() as libc::pid_t;
            let alive = self.try_wait()?.is_none();
            self.observe(leader, alive);
            let mut reached = false;
            match stage {
                StopStage::Interrupt => {
                    if alive {
                        let foreground = unsafe { libc::tcgetpgrp(self.master.as_raw_fd()) };
                        reached = unsafe { libc::killpg(if foreground > 1 { foreground } else { leader }, libc::SIGINT) } == 0;
                    }
                }
                StopStage::Terminate | StopStage::Kill => {
                    let signal = if stage == StopStage::Kill { libc::SIGKILL } else { libc::SIGTERM };
                    for member in self.members.iter().rev() {
                        if unsafe { libc::getpgid(member.pid) } != member.group || unsafe { libc::getsid(member.pid) } != member.session {
                            continue;
                        }
                        unsafe {
                            reached |= libc::kill(member.pid, signal) == 0;
                            if stage == StopStage::Terminate { libc::kill(member.pid, libc::SIGCONT); }
                        }
                    }
                    if alive {
                        reached |= unsafe { libc::killpg(leader, signal) } == 0;
                    }
                    if stage == StopStage::Kill {
                        self.swept = true;
                    }
                }
            }
            Ok(reached)
        }

        /// 🔁 Asks the foreground job to repaint, the way a terminal does after its window changed.
        pub fn refresh(&mut self) {
            let foreground = unsafe { libc::tcgetpgrp(self.master.as_raw_fd()) };
            if foreground > 1 {
                unsafe { libc::killpg(foreground, libc::SIGWINCH) };
            }
        }

        /// 🛑 Ends the owned process tree, including descendant-created groups and sessions.
        pub fn terminate(&mut self) -> Result<(), PtyError> {
            self.signal(StopStage::Kill).map(|_| ())
        }

        pub fn kill(&mut self) -> Result<(), PtyError> {
            self.terminate()?;
            if self.code.is_none() {
                self.code = Some(exit_code(self.child.wait().map_err(|error| err(error.to_string()))?));
            }
            Ok(())
        }

        fn observe(&mut self, leader: libc::pid_t, alive: bool) {
            if self.swept {
                return;
            }
            let table: Vec<(Member, libc::pid_t)> = processes().into_iter().filter_map(|(pid, parent)| {
                let (group, session) = unsafe { (libc::getpgid(pid), libc::getsid(pid)) };
                (group > 0 && session > 0).then_some((Member { pid, group, session }, parent))
            }).collect();
            let mut owned: Vec<Member> = table.iter().filter(|(member, _)| member.session == leader || alive && member.pid == leader || self.members.contains(member)).map(|(member, _)| *member).collect();
            loop {
                let before = owned.len();
                for (member, parent) in &table {
                    if !owned.contains(member) && owned.iter().any(|known| known.pid == *parent) { owned.push(*member); }
                }
                if before == owned.len() { break; }
            }
            for member in owned {
                if !self.members.contains(&member) { self.members.push(member); }
            }
        }
    }

    fn exit_code(status: system_process::ExitStatus) -> i32 {
        status.code().or_else(|| status.signal().map(|signal| 128 + signal)).unwrap_or(-1)
    }

    /// 📋 Every process with its parent, read from the kernel's process table without starting a helper process.
    ///
    /// @see https://man7.org/linux/man-pages/man5/proc_pid_stat.5.html
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    fn processes() -> Vec<(libc::pid_t, libc::pid_t)> {
        let count = unsafe { libc::proc_listallpids(std::ptr::null_mut(), 0) };
        if count <= 0 { return Vec::new(); }
        let mut pids = vec![0 as libc::pid_t; count as usize + 256];
        let count = unsafe { libc::proc_listallpids(pids.as_mut_ptr().cast(), (pids.len() * std::mem::size_of::<libc::pid_t>()) as libc::c_int) };
        if count <= 0 { return Vec::new(); }
        pids.truncate(count as usize);
        let size = std::mem::size_of::<libc::proc_bsdinfo>() as libc::c_int;
        pids.into_iter().filter(|pid| *pid > 0).filter_map(|pid| {
            let mut info: libc::proc_bsdinfo = unsafe { std::mem::zeroed() };
            (unsafe { libc::proc_pidinfo(pid, libc::PROC_PIDTBSDINFO, 0, (&mut info as *mut libc::proc_bsdinfo).cast(), size) } == size).then_some((pid, info.pbi_ppid as libc::pid_t))
        }).collect()
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    fn processes() -> Vec<(libc::pid_t, libc::pid_t)> {
        let Ok(entries) = system_fs::read_dir("/proc") else { return Vec::new() };
        entries.flatten().filter_map(|entry| {
            let pid: libc::pid_t = entry.file_name().to_str()?.parse().ok()?;
            let stat = system_fs::read_to_string(entry.path().join("stat")).ok()?;
            let parent = stat.rsplit_once(')')?.1.split_whitespace().nth(1)?.parse().ok()?;
            Some((pid, parent))
        }).collect()
    }

    #[cfg(not(any(target_os = "macos", target_os = "ios", target_os = "linux", target_os = "android")))]
    fn processes() -> Vec<(libc::pid_t, libc::pid_t)> {
        Vec::new()
    }

    impl Write for Pty {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let n = unsafe { libc::write(self.master.as_raw_fd(), buf.as_ptr() as *const _, buf.len()) };
            if n < 0 {
                Err(std::io::Error::last_os_error())
            } else {
                Ok(n as usize)
            }
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl Drop for Pty {
        fn drop(&mut self) {
            if !self.swept {
                let _ = self.terminate();
            }
            let _ = self.child.wait();
        }
    }
}

#[cfg(all(unix, not(target_arch = "wasm32")))]
pub use unix_impl::Pty;

#[cfg(windows)]
mod windows_impl {
    use super::*;
    use crate::tui::component::windows_abi::{
        AssignProcessToJobObject, CreateJobObjectW, JOBOBJECT_EXTENDED_LIMIT_INFORMATION, ResumeThread, SetInformationJobObject, TerminateJobObject,
        CreatePipe, CreateProcessW, CreatePseudoConsole, GetExitCodeProcess, GetProcessId, OwnedHandle, OwnedPseudoConsole, ProcThreadAttributeList, ReadFile, ResizePseudoConsole, SetHandleInformation, TerminateProcess,
        WaitForSingleObject, WriteFile, COORD, CREATE_UNICODE_ENVIRONMENT, EXTENDED_STARTUPINFO_PRESENT, HANDLE_FLAG_INHERIT, INVALID_HANDLE_VALUE, PROCESS_INFORMATION, SECURITY_ATTRIBUTES, STARTUPINFOEXW, STILL_ACTIVE, WAIT_OBJECT_0,
        WAIT_TIMEOUT,
    };
    use std::collections::VecDeque;
    use std::ffi::OsStr;
    use std::mem::size_of;
    use std::os::windows::ffi::OsStrExt;
    use std::sync::{Arc, Condvar, Mutex};

    const PIPE_BACKLOG_BYTES: usize = 256 * 1024;
    const CONTROL_C_EXIT: u32 = 0xC000_013A;
    const WAIT_FOREVER: u32 = u32::MAX;
    const JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE: u32 = 0x0000_2000;
    const CTRL_BREAK_EVENT: u32 = 1;
    const ATTACH_PARENT_PROCESS: u32 = u32::MAX;

    extern "system" {
        fn SetConsoleCtrlHandler(handler: Option<unsafe extern "system" fn(u32) -> i32>, add: i32) -> i32;
        fn AttachConsole(process: u32) -> i32;
        fn FreeConsole() -> i32;
        fn GenerateConsoleCtrlEvent(event: u32, group: u32) -> i32;
        fn GetConsoleCP() -> u32;
    }

    /// 🚫 Takes every console control event, so the process that sends one to a console it shares with its
    /// target does not receive the default action itself.
    unsafe extern "system" fn swallow(_event: u32) -> i32 {
        SWALLOWED.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        1
    }

    static SWALLOWED: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

    /// 🧨️ Sends Ctrl+Break to every process attached to the console of `pid`. A process has one console at
    /// a time, so this attaches to the target's pseudo-console for the moment of the event, shielded
    /// from the event itself, and returns to the console it had.
    ///
    /// @see https://learn.microsoft.com/windows/console/generateconsolectrlevent
    fn console_event(pid: u32, event: u32) -> bool {
        static CONSOLE: Mutex<()> = Mutex::new(());
        let Ok(_exclusive) = CONSOLE.lock() else { return false };
        unsafe {
            let attached = GetConsoleCP() != 0;
            FreeConsole();
            let sent = AttachConsole(pid) != 0 && {
                SetConsoleCtrlHandler(Some(swallow), 1);
                let before = SWALLOWED.load(std::sync::atomic::Ordering::SeqCst);
                let sent = GenerateConsoleCtrlEvent(event, 0) != 0;
                let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
                while sent && SWALLOWED.load(std::sync::atomic::Ordering::SeqCst) == before && std::time::Instant::now() < deadline { std::thread::sleep(std::time::Duration::from_millis(5)); }
                SetConsoleCtrlHandler(Some(swallow), 0);
                sent
            };
            FreeConsole();
            if attached { AttachConsole(ATTACH_PARENT_PROCESS); }
            sent
        }
    }

    /// 🧮️ Counts console control events instead of ending the process on the first one, so a command can
    /// turn them into its documented cancellation steps.
    pub fn interrupts() -> &'static std::sync::atomic::AtomicUsize {
        static COUNT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        static INSTALL: std::sync::Once = std::sync::Once::new();
        unsafe extern "system" fn count(_event: u32) -> i32 {
            COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            1
        }
        INSTALL.call_once(|| unsafe {
            SetConsoleCtrlHandler(Some(count), 1);
        });
        &COUNT
    }

    /// 🪟️ Starts a detached process with handle inheritance disabled at the native boundary.
    pub fn spawn_detached(cmd: &str, args: &[&str], remove_env: &[&str], cwd: &Path) -> Result<u32, PtyError> {
        use crate::tui::component::windows_abi::STARTUPINFOW;
        let application = to_wide(cmd); let mut command = build_cmdline(cmd, args);
        let directory = to_wide(&cwd.to_string_lossy()); let environment = build_env_block(&[], remove_env).unwrap();
        let mut startup = STARTUPINFOW::default(); startup.cb = size_of::<STARTUPINFOW>() as u32;
        let mut process = PROCESS_INFORMATION::default();
        const DETACHED_PROCESS: u32 = 0x0000_0008;
        const CREATE_BREAKAWAY_FROM_JOB: u32 = 0x0100_0000;
        const ERROR_ACCESS_DENIED: i32 = 5;
        let create = |flags: u32, process: &mut PROCESS_INFORMATION, command: &mut Vec<u16>| unsafe { CreateProcessW(application.as_ptr(), command.as_mut_ptr(), std::ptr::null(), std::ptr::null(), 0, CREATE_UNICODE_ENVIRONMENT | DETACHED_PROCESS | flags, environment.as_ptr().cast(), directory.as_ptr(), &startup, process) };
        let spare = command.clone();
        if create(CREATE_BREAKAWAY_FROM_JOB, &mut process, &mut command) == 0 {
            let refused = std::io::Error::last_os_error();
            command = spare;
            if refused.raw_os_error() != Some(ERROR_ACCESS_DENIED) || create(0, &mut process, &mut command) == 0 {
                return Err(err(format!("CreateProcessW detached failed: {}", std::io::Error::last_os_error())));
            }
        }
        let _process = unsafe { OwnedHandle::from_raw(process.hProcess) };
        let _thread = unsafe { OwnedHandle::from_raw(process.hThread) };
        Ok(process.dwProcessId)
    }

    /// 📣 What a pseudo-terminal calls when its output, its input room or its exit state changed.
    pub type Notifier = Arc<dyn Fn() + Send + Sync>;

    /// 🔐 A kernel handle shared with the threads that block on it.
    struct Shared(OwnedHandle);
    unsafe impl Send for Shared {}
    unsafe impl Sync for Shared {}
    impl Shared {
        fn raw(&self) -> crate::tui::component::windows_abi::HANDLE { self.0.as_raw() }
    }

    /// 🚰 A bounded byte queue between the owner of a pseudo-terminal and the thread blocked on its pipe.
    #[derive(Default)]
    struct Pipe {
        state: Mutex<PipeState>,
        changed: Condvar,
    }

    #[derive(Default)]
    struct PipeState {
        bytes: VecDeque<u8>,
        closed: bool,
    }

    /// 🪞️ Windows ConPTY master pipes plus child process.
    ///
    /// The console's pipes have no readiness to wait for, so one thread blocks on the output pipe, one
    /// on the input pipe and one on the process; each fills or drains a bounded queue and calls the
    /// notifier. Every method of `Pty` therefore returns at once, which is what lets a single thread
    /// serve many sessions; termination closes the job, which ends all three threads.
    pub struct Pty {
        input: Arc<Pipe>,
        output: Arc<Pipe>,
        hpcon: Option<OwnedPseudoConsole>,
        process: Arc<Shared>,
        _thread: OwnedHandle,
        job: OwnedHandle,
        notifier: Arc<Mutex<Option<Notifier>>>,
        batch: bool,
    }

    fn to_wide(s: &str) -> Vec<u16> {
        OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
    }

    fn is_command_interpreter(program: &str) -> bool {
        Path::new(program).file_stem().is_some_and(|stem| stem.eq_ignore_ascii_case("cmd"))
    }

    fn is_batch_file(program: &str) -> bool {
        Path::new(program).extension().is_some_and(|extension| extension.eq_ignore_ascii_case("cmd") || extension.eq_ignore_ascii_case("bat"))
    }

    /// 🔧 Whether the program is a batch file or a command interpreter asked to run a string, which is what asks
    /// "Terminate batch job (Y/N)?" when it is interrupted.
    fn runs_batch(cmd: &str, args: &[&str]) -> bool {
        is_batch_file(cmd) || is_command_interpreter(cmd) && args.iter().any(|arg| arg.eq_ignore_ascii_case("/c") || arg.eq_ignore_ascii_case("/k"))
    }

    /// 📜 The command line of a program. `cmd.exe` is the one program that does not read its line with the C runtime's
    /// rules: asked to run a string (`/c` or `/k`) it gets `/s` and the string, joined by spaces, inside one pair of
    /// quotes that it removes and nothing else, so the string reaches the shell exactly as written. Everything else
    /// is quoted for the C runtime.
    ///
    /// @see https://learn.microsoft.com/windows-server/administration/windows-commands/cmd
    fn build_cmdline(cmd: &str, args: &[&str]) -> Vec<u16> {
        if is_command_interpreter(cmd) {
            if let Some(at) = args.iter().position(|arg| arg.eq_ignore_ascii_case("/c") || arg.eq_ignore_ascii_case("/k")) {
                let mut line = quote_argument(cmd);
                for flag in &args[..at] {
                    line.push(' ');
                    if flag.contains([' ', '\t', '"']) { line.push_str(&quote_argument(flag)); } else { line.push_str(flag); }
                }
                if !args[..at].iter().any(|flag| flag.eq_ignore_ascii_case("/s")) { line.push_str(" /s"); }
                line.push(' ');
                line.push_str(args[at]);
                line.push_str(" \"");
                line.push_str(&args[at + 1..].join(" "));
                line.push('"');
                return to_wide(&line);
            }
        }
        let mut line = String::new();
        for arg in std::iter::once(cmd).chain(args.iter().copied()) {
            if !line.is_empty() { line.push(' '); }
            line.push_str(&quote_argument(arg));
        }
        to_wide(&line)
    }

    fn quote_argument(arg: &str) -> String {
        let mut line = String::new();
        line.push('"');
        let mut slashes = 0;
        for ch in arg.chars() {
            if ch == '\\' { slashes += 1; continue; }
            line.extend(std::iter::repeat_n('\\', if ch == '"' { slashes * 2 + 1 } else { slashes }));
            line.push(ch);
            slashes = 0;
        }
        line.extend(std::iter::repeat_n('\\', slashes * 2));
        line.push('"');
        line
    }

    fn encode_env_block(mut merged: std::collections::BTreeMap<String, (std::ffi::OsString, std::ffi::OsString)>) -> Vec<u16> {
        let mut block = Vec::new();
        if let Some((_, value)) = merged.get_mut("PATH") {
            let mut seen = std::collections::HashSet::new();
            let paths: Vec<_> = std::env::split_paths(value).filter(|path| seen.insert(path.to_string_lossy().to_uppercase())).collect();
            if let Ok(path) = std::env::join_paths(paths) { *value = path; }
        }
        for (k, v) in merged.values() {
            block.extend(k.encode_wide());
            block.push(b'=' as u16);
            block.extend(v.encode_wide());
            block.push(0);
        }
        block.push(0);
        block
    }

    fn build_env_block(env: &[(&str, &str)], remove_env: &[&str]) -> Option<Vec<u16>> {
        let mut merged: std::collections::BTreeMap<String, (std::ffi::OsString, std::ffi::OsString)> = std::env::vars_os().map(|(key, value)| (key.to_string_lossy().to_uppercase(), (key.clone(), std::env::var_os(&key).unwrap_or(value)))).collect();
        if let Some(path) = std::env::var_os("PATH") { merged.insert("PATH".into(), ("PATH".into(), path)); }
        for key in remove_env { merged.remove(&key.to_uppercase()); }
        for (key, value) in env { merged.insert(key.to_uppercase(), (key.into(), value.into())); }
        Some(encode_env_block(merged))
    }

    /// 🔍 The file `cmd` names in `env`: a path as given, a bare name on the `PATH` of `env` with the
    /// extensions of its `PATHEXT`, relative paths from `cwd`.
    ///
    /// @see https://learn.microsoft.com/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw
    pub fn resolve_program(cmd: &str, env: &[(String, String)], cwd: Option<&Path>) -> Option<std::path::PathBuf> {
        let value = |name: &str| env.iter().find(|(key, _)| key.eq_ignore_ascii_case(name)).map(|(_, value)| value.as_str());
        let extensions: Vec<&str> = value("PATHEXT").unwrap_or(".COM;.EXE;.BAT;.CMD").split(';').filter(|extension| !extension.is_empty()).collect();
        let is_script = |extension: &str| extension.eq_ignore_ascii_case(".bat") || extension.eq_ignore_ascii_case(".cmd");
        let (scripts, executables): (Vec<&str>, Vec<&str>) = extensions.iter().copied().partition(|extension| is_script(extension));
        let candidate = |base: std::path::PathBuf, extensions: &[&str]| -> Option<std::path::PathBuf> {
            if base.extension().is_some() && base.is_file() { return Some(base); }
            extensions.iter().map(|extension| std::path::PathBuf::from(format!("{}{extension}", base.display()))).find(|path| path.is_file())
        };
        if cmd.contains(['\\', '/']) || Path::new(cmd).is_absolute() {
            let path = Path::new(cmd);
            let base = if path.is_absolute() { path.to_path_buf() } else { cwd.map_or_else(|| path.to_path_buf(), |cwd| cwd.join(path)) };
            return candidate(base.clone(), &executables).or_else(|| candidate(base, &scripts));
        }
        let directories: Vec<std::path::PathBuf> = std::env::split_paths(value("PATH")?).collect();
        directories.iter().find_map(|directory| candidate(directory.join(cmd), &executables)).or_else(|| directories.iter().find_map(|directory| candidate(directory.join(cmd), &scripts)))
    }

    fn build_exact_env_block(env: &[(String, String)]) -> Vec<u16> {
        encode_env_block(env.iter().map(|(key, value)| (key.to_uppercase(), (key.into(), value.into()))).collect())
    }

    impl Pty {
        /// 🚼️ Starts `cmd` on a new pseudo-console with this process's environment, `env` added and `remove_env` dropped.
        pub fn spawn(cmd: &str, args: &[&str], env: &[(&str, &str)], remove_env: &[&str], cwd: Option<&Path>, size: PtySize) -> Result<Self, PtyError> {
            Self::start(cmd, args, build_env_block(env, remove_env), cwd, size)
        }

        /// 🗝️ Starts `cmd` on a new pseudo-console with exactly `env` as its environment; the program is
        /// looked up on the `PATH` and `PATHEXT` of that environment, as the process itself would.
        pub fn spawn_exact(cmd: &str, args: &[&str], env: &[(String, String)], cwd: Option<&Path>, size: PtySize) -> Result<Self, PtyError> {
            let program = resolve_program(cmd, env, cwd).ok_or_else(|| err(format!("program {cmd:?} is not on the PATH of the requesting environment")))?;
            Self::start(&program.to_string_lossy(), args, Some(build_exact_env_block(env)), cwd, size)
        }

        fn start(cmd: &str, args: &[&str], env_block: Option<Vec<u16>>, cwd: Option<&Path>, size: PtySize) -> Result<Self, PtyError> {
            unsafe {
                let mut sa: SECURITY_ATTRIBUTES = std::mem::zeroed();
                sa.nLength = size_of::<SECURITY_ATTRIBUTES>() as u32;
                sa.bInheritHandle = 1;

                let mut input_read = INVALID_HANDLE_VALUE;
                let mut input_write = INVALID_HANDLE_VALUE;
                let mut output_read = INVALID_HANDLE_VALUE;
                let mut output_write = INVALID_HANDLE_VALUE;
                if CreatePipe(&mut input_read, &mut input_write, &sa, 0) == 0 {
                    return Err(err("CreatePipe input failed"));
                }
                let input_read = OwnedHandle::from_raw(input_read);
                let input_write = OwnedHandle::from_raw(input_write);
                let input_read = input_read.ok_or_else(|| err("CreatePipe input returned an invalid read handle"))?;
                let input_write = input_write.ok_or_else(|| err("CreatePipe input returned an invalid write handle"))?;
                if CreatePipe(&mut output_read, &mut output_write, &sa, 0) == 0 {
                    return Err(err("CreatePipe output failed"));
                }
                let output_read = OwnedHandle::from_raw(output_read);
                let output_write = OwnedHandle::from_raw(output_write);
                let output_read = output_read.ok_or_else(|| err("CreatePipe output returned an invalid read handle"))?;
                let output_write = output_write.ok_or_else(|| err("CreatePipe output returned an invalid write handle"))?;
                if SetHandleInformation(input_write.as_raw(), HANDLE_FLAG_INHERIT, 0) == 0 || SetHandleInformation(output_read.as_raw(), HANDLE_FLAG_INHERIT, 0) == 0 {
                    return Err(err(format!("SetHandleInformation failed: {}", std::io::Error::last_os_error())));
                }

                let coord = COORD { X: size.cols as i16, Y: size.rows as i16 };
                let mut raw_hpcon = 0;
                let hr = CreatePseudoConsole(coord, input_read.as_raw(), output_write.as_raw(), 0, &mut raw_hpcon);
                if hr < 0 {
                    return Err(err(format!("CreatePseudoConsole failed: HRESULT {hr}")));
                }
                let hpcon = OwnedPseudoConsole::from_raw(raw_hpcon).ok_or_else(|| err("CreatePseudoConsole returned a null handle"))?;
                drop(input_read);
                drop(output_write);

                let mut attr_list = ProcThreadAttributeList::new(1).map_err(|e| err(format!("InitializeProcThreadAttributeList failed: {e}")))?;
                attr_list.set_pseudo_console(hpcon.as_raw()).map_err(|e| err(format!("UpdateProcThreadAttribute failed: {e}")))?;

                let mut si: STARTUPINFOEXW = std::mem::zeroed();
                si.StartupInfo.cb = size_of::<STARTUPINFOEXW>() as u32;
                si.StartupInfo.dwFlags = 0x100;
                si.StartupInfo.hStdInput = INVALID_HANDLE_VALUE;
                si.StartupInfo.hStdOutput = INVALID_HANDLE_VALUE;
                si.StartupInfo.hStdError = INVALID_HANDLE_VALUE;
                si.lpAttributeList = attr_list.as_mut_ptr();

                let mut cmdline = build_cmdline(cmd, args);
                let cwd_wide = cwd.map(|p| to_wide(&p.to_string_lossy()));
                let mut pi: PROCESS_INFORMATION = std::mem::zeroed();
                let job = OwnedHandle::from_raw(CreateJobObjectW(std::ptr::null(), std::ptr::null())).ok_or_else(|| err("CreateJobObjectW failed"))?;
                let mut limits = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
                limits.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
                if SetInformationJobObject(job.as_raw(), 9, (&limits as *const JOBOBJECT_EXTENDED_LIMIT_INFORMATION).cast(), size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32) == 0 {
                    return Err(err(format!("SetInformationJobObject failed: {}", std::io::Error::last_os_error())));
                }
                let mut flags = EXTENDED_STARTUPINFO_PRESENT | 0x0000_0004;
                if env_block.is_some() {
                    flags |= CREATE_UNICODE_ENVIRONMENT;
                }
                let ok = CreateProcessW(
                    std::ptr::null(),
                    cmdline.as_mut_ptr(),
                    std::ptr::null(),
                    std::ptr::null(),
                    0,
                    flags,
                    env_block.as_ref().map(|b| b.as_ptr() as *const _).unwrap_or(std::ptr::null()),
                    cwd_wide.as_ref().map(|b| b.as_ptr()).unwrap_or(std::ptr::null()),
                    &si.StartupInfo,
                    &mut pi,
                );
                if ok == 0 {
                    return Err(err(format!("CreateProcessW failed: {}", std::io::Error::last_os_error())));
                }
                let process = OwnedHandle::from_raw(pi.hProcess);
                let thread = OwnedHandle::from_raw(pi.hThread);
                let process = process.ok_or_else(|| err("CreateProcessW returned an invalid process handle"))?;
                let thread = thread.ok_or_else(|| err("CreateProcessW returned an invalid thread handle"))?;
                if AssignProcessToJobObject(job.as_raw(), process.as_raw()) == 0 {
                    let failure = std::io::Error::last_os_error();
                    TerminateProcess(process.as_raw(), 1);
                    return Err(err(format!("AssignProcessToJobObject failed: {failure}")));
                }
                if ResumeThread(thread.as_raw()) == u32::MAX {
                    TerminateJobObject(job.as_raw(), 1);
                    return Err(err(format!("ResumeThread failed: {}", std::io::Error::last_os_error())));
                }
                let pty = Self { hpcon: Some(hpcon), input: Arc::default(), output: Arc::default(), process: Arc::new(Shared(process)), _thread: thread, job, notifier: Arc::default(), batch: runs_batch(cmd, args) };
                pty.serve(Shared(input_write), Shared(output_read));
                Ok(pty)
            }
        }

        fn serve(&self, input_write: Shared, output_read: Shared) {
            let notify = |notifier: &Arc<Mutex<Option<Notifier>>>| {
                let call = notifier.lock().ok().and_then(|slot| slot.clone());
                if let Some(call) = call { call(); }
            };
            let (output, notifier) = (self.output.clone(), self.notifier.clone());
            let _ = std::thread::Builder::new().name("ConPTY output".into()).stack_size(128 * 1024).spawn(move || {
                let mut page = vec![0u8; 64 * 1024];
                loop {
                    let mut read = 0u32;
                    let ok = unsafe { ReadFile(output_read.raw(), page.as_mut_ptr(), page.len() as u32, &mut read, std::ptr::null_mut()) };
                    let Ok(mut state) = output.state.lock() else { break };
                    if ok == 0 || read == 0 || state.closed {
                        state.closed = true;
                        drop(state);
                        notify(&notifier);
                        break;
                    }
                    state.bytes.extend(&page[..read as usize]);
                    while state.bytes.len() > PIPE_BACKLOG_BYTES && !state.closed {
                        let Ok(waited) = output.changed.wait(state) else { return };
                        state = waited;
                    }
                    drop(state);
                    notify(&notifier);
                }
            });
            let (input, notifier) = (self.input.clone(), self.notifier.clone());
            let _ = std::thread::Builder::new().name("ConPTY input".into()).stack_size(128 * 1024).spawn(move || loop {
                let chunk: Vec<u8> = {
                    let Ok(mut state) = input.state.lock() else { break };
                    while state.bytes.is_empty() && !state.closed {
                        let Ok(waited) = input.changed.wait(state) else { return };
                        state = waited;
                    }
                    if state.bytes.is_empty() { break; }
                    let count = state.bytes.len().min(16 * 1024);
                    state.bytes.drain(..count).collect()
                };
                let mut rest = &chunk[..];
                while !rest.is_empty() {
                    let mut written = 0u32;
                    if unsafe { WriteFile(input_write.raw(), rest.as_ptr(), rest.len() as u32, &mut written, std::ptr::null_mut()) } == 0 || written == 0 {
                        if let Ok(mut state) = input.state.lock() { state.closed = true; }
                        return;
                    }
                    rest = &rest[written as usize..];
                }
                notify(&notifier);
            });
            let (process, notifier) = (self.process.clone(), self.notifier.clone());
            let _ = std::thread::Builder::new().name("ConPTY exit".into()).stack_size(64 * 1024).spawn(move || {
                unsafe { WaitForSingleObject(process.raw(), WAIT_FOREVER) };
                notify(&notifier);
            });
        }

        /// 🗣️ Names what to call when output arrived, input room freed up or the child ended.
        pub fn set_notifier(&self, notifier: Notifier) {
            if let Ok(mut slot) = self.notifier.lock() { *slot = Some(notifier); }
        }

        pub fn resize(&mut self, size: PtySize) -> Result<(), PtyError> {
            let coord = COORD { X: size.cols as i16, Y: size.rows as i16 };
            let Some(hpcon) = self.hpcon.as_ref() else { return Ok(()) };
            let hr = unsafe { ResizePseudoConsole(hpcon.as_raw(), coord) };
            if hr < 0 {
                return Err(err(format!("ResizePseudoConsole failed: HRESULT {hr}")));
            }
            Ok(())
        }

        pub fn writer(&mut self) -> &mut impl Write {
            self
        }

        /// 📑️ Reads what the child has written so far without waiting for more.
        pub fn read(&mut self, buf: &mut [u8]) -> PtyRead {
            let Ok(mut state) = self.output.state.lock() else { return PtyRead::Closed };
            if state.bytes.is_empty() {
                return if state.closed { PtyRead::Closed } else { PtyRead::Empty };
            }
            let count = state.bytes.len().min(buf.len());
            for (slot, byte) in buf.iter_mut().zip(state.bytes.drain(..count)) { *slot = byte; }
            self.output.changed.notify_all();
            PtyRead::Data(count)
        }

        pub fn try_read(&mut self, buf: &mut [u8]) -> Result<usize, PtyError> {
            Ok(match self.read(buf) { PtyRead::Data(count) => count, PtyRead::Empty | PtyRead::Closed => 0 })
        }

        /// 💧️ Hands the child as much input as its console queue accepts right now and answers how much that was.
        pub fn try_write(&mut self, data: &[u8]) -> Result<usize, PtyError> {
            let mut state = self.input.state.lock().map_err(|_| err("pseudo-console input queue poisoned"))?;
            if state.closed {
                return Err(err("pseudo-console input closed"));
            }
            let count = PIPE_BACKLOG_BYTES.saturating_sub(state.bytes.len()).min(data.len());
            state.bytes.extend(&data[..count]);
            self.input.changed.notify_all();
            Ok(count)
        }

        /// 🖋️ Writes all of `data`, waiting while the console queue is full; a child that accepts nothing for ten seconds fails the write.
        pub fn write_all(&mut self, data: &[u8]) -> Result<(), PtyError> {
            let mut rest = data;
            let mut stalled = std::time::Instant::now();
            while !rest.is_empty() {
                let written = self.try_write(rest)?;
                if written > 0 {
                    rest = &rest[written..];
                    stalled = std::time::Instant::now();
                    continue;
                }
                if stalled.elapsed() >= std::time::Duration::from_secs(10) {
                    return Err(err("pseudo-console input stalled"));
                }
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
            Ok(())
        }

        /// 🎌️ The exit code once the child has ended; an interrupted child reports `130` as it does elsewhere.
        pub fn try_wait(&mut self) -> Result<Option<i32>, PtyError> {
            unsafe {
                let wait = WaitForSingleObject(self.process.raw(), 0);
                if wait == WAIT_TIMEOUT {
                    return Ok(None);
                }
                if wait != WAIT_OBJECT_0 {
                    return Err(err("WaitForSingleObject failed"));
                }
                let mut code = 0u32;
                if GetExitCodeProcess(self.process.raw(), &mut code) == 0 {
                    return Err(err("GetExitCodeProcess failed"));
                }
                if code == STILL_ACTIVE as u32 {
                    return Ok(None);
                }
                Ok(Some(if code == CONTROL_C_EXIT { 130 } else { code as i32 }))
            }
        }

        pub fn pid(&self) -> u32 {
            unsafe { GetProcessId(self.process.raw()) }
        }

        /// 🧯 Closes the pseudo-console of a child that has ended. The console writes what it still holds to the
        /// output pipe and then ends it, so reading until [`PtyRead::Closed`] returns the last output exactly.
        pub fn finish(&mut self) {
            self.hpcon.take();
        }

        /// 🧰 Whether the child is a batch job or a command interpreter running a string: a console job that does not
        /// end on an interrupt by itself, because the interpreter inherits the shell's way of ignoring it and then
        /// asks "Terminate batch job (Y/N)?".
        pub fn batch(&self) -> bool {
            self.batch
        }

        /// 🪝️ Applies one stop stage: `Interrupt` types Ctrl+C ahead of any queued input, `Terminate` sends
        /// Ctrl+Break to every process on the console, which ends a process that handles neither, `Kill`
        /// ends the job.
        pub fn signal(&mut self, stage: StopStage) -> Result<bool, PtyError> {
            match stage {
                StopStage::Interrupt => {
                    let mut state = self.input.state.lock().map_err(|_| err("pseudo-console input queue poisoned"))?;
                    state.bytes.push_front(3);
                    self.input.changed.notify_all();
                    let reached = !state.closed;
                    drop(state);
                    if self.batch { self.answer_batch_prompt(); }
                    Ok(reached)
                }
                StopStage::Terminate => Ok(self.try_wait()?.is_none() && console_event(self.pid(), CTRL_BREAK_EVENT)),
                StopStage::Kill => self.terminate().map(|()| true),
            }
        }

        /// ♻️ A pseudo-console repaints on its own when its size changes; nothing to ask for.
        pub fn refresh(&mut self) {}

        /// ⚠️ A batch job asks "Terminate batch job (Y/N)?" when it is interrupted and waits for the answer; this
        /// answers yes shortly after the interrupt if the process is still there, so that a stop does not stall at the prompt.
        fn answer_batch_prompt(&self) {
            let (input, process) = (self.input.clone(), self.process.clone());
            let _ = std::thread::Builder::new().name("ConPTY batch answer".into()).stack_size(64 * 1024).spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(400));
                if unsafe { WaitForSingleObject(process.raw(), 0) } != WAIT_TIMEOUT { return; }
                if let Ok(mut state) = input.state.lock() {
                    if !state.closed { state.bytes.extend(b"Y\r"); input.changed.notify_all(); }
                }
            });
        }


        /// 🌳 Terminates every descendant in the session's kernel-owned job.
        pub fn terminate(&mut self) -> Result<(), PtyError> {
            if unsafe { TerminateJobObject(self.job.as_raw(), 1) } == 0 {
                return Err(err(format!("TerminateJobObject failed: {}", std::io::Error::last_os_error())));
            }
            Ok(())
        }

        pub fn kill(&mut self) -> Result<(), PtyError> {
            self.terminate()?;
            unsafe {
                WaitForSingleObject(self.process.raw(), 1500);
            }
            Ok(())
        }
    }

    impl Write for Pty {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            match self.try_write(buf) {
                Ok(0) if !buf.is_empty() => Err(std::io::ErrorKind::WouldBlock.into()),
                Ok(count) => Ok(count),
                Err(error) => Err(std::io::Error::other(error.message)),
            }
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl Drop for Pty {
        fn drop(&mut self) {
            for pipe in [&self.input, &self.output] {
                if let Ok(mut state) = pipe.state.lock() { state.closed = true; }
                pipe.changed.notify_all();
            }
            let _ = self.kill();
        }
    }
}

#[cfg(windows)]
pub use windows_impl::{interrupts, resolve_program, spawn_detached, Notifier, Pty};
