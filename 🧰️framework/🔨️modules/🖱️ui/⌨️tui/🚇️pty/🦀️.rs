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
#[path = "🪟️windows-impl/🦀️.rs"]
mod windows_impl;

#[cfg(windows)]
pub use windows_impl::{interrupts, resolve_program, spawn_detached, Notifier, Pty};
