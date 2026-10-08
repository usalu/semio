//! 🚚 The byte transport between a client and the workspace daemon: a duplex stream whose reads and
//! writes never wait, the listener that is the daemon's one endpoint, and a single wait over every
//! stream and pseudo-terminal a thread serves. A unix socket and a pseudo-terminal master are waited
//! on with `poll`. A Windows named pipe has no readiness to wait for, so threads blocked on overlapped
//! reads and writes fill and drain bounded queues and raise the waiter instead; both halves offer the
//! same surface, so the daemon loop and the client loop are written once.
//!
//! @see https://pubs.opengroup.org/onlinepubs/9699919799/functions/poll.html
//! @see https://learn.microsoft.com/windows/win32/ipc/synchronous-and-overlapped-input-and-output

use super::ipc;
use std::path::{Path, PathBuf};
use std::time::Duration;
use ui_tui::tui::pty::Pty;

/// 🎫 What a readiness report is about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Token {
    Listener,
    Client(u64),
    Session(u64),
}

/// 🔔 One source that may be read or written without waiting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Event {
    pub token: Token,
    pub readable: bool,
    pub writable: bool,
}

fn record_endpoint(root: &Path, endpoint: &str) -> std::io::Result<()> {
    let target = ipc::endpoint_path(root);
    let temporary = target.with_extension(format!("endpoint.{}.tmp", std::process::id()));
    std::fs::write(&temporary, format!("{endpoint}\n"))?;
    std::fs::rename(&temporary, &target)
}

fn forget_endpoint(root: &Path, endpoint: &str) {
    let target = ipc::endpoint_path(root);
    if std::fs::read_to_string(&target).is_ok_and(|text| text.trim() == endpoint) { let _ = std::fs::remove_file(target); }
}

#[cfg(unix)]
mod platform {
    use super::*;
    use std::io::{Read, Write};
    use std::os::unix::fs::{DirBuilderExt, MetadataExt, PermissionsExt};
    use std::os::unix::io::AsRawFd;
    use std::os::unix::net::{UnixListener, UnixStream};
    use ui_tui::tui::pty::readiness::{self, Interest};

    /// 🔌 A connected unix socket whose reads and writes never wait.
    pub struct Stream {
        inner: UnixStream,
    }

    impl Stream {
        /// 🤝 Connects to the daemon of a workspace.
        pub fn connect(root: &Path) -> std::io::Result<Self> {
            Self::adopt(UnixStream::connect(ipc::endpoint(root))?)
        }

        pub fn adopt(inner: UnixStream) -> std::io::Result<Self> {
            inner.set_nonblocking(true)?;
            Ok(Self { inner })
        }

        /// 🧪 Two connected ends with the kernel's real buffer limits.
        pub fn pair() -> std::io::Result<(Self, Self)> {
            let (left, right) = UnixStream::pair()?;
            Ok((Self::adopt(left)?, Self::adopt(right)?))
        }

        /// 📥 `Some(0)` is the end of the stream, `None` means nothing has arrived yet.
        pub fn try_read(&mut self, buf: &mut [u8]) -> std::io::Result<Option<usize>> {
            loop {
                match self.inner.read(buf) {
                    Ok(count) => return Ok(Some(count)),
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => return Ok(None),
                    Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
                    Err(error) => return Err(error),
                }
            }
        }

        /// 📤 How many bytes the peer's buffer took; `0` means it is full for now.
        pub fn try_write(&mut self, data: &[u8]) -> std::io::Result<usize> {
            loop {
                match self.inner.write(data) {
                    Ok(count) => return Ok(count),
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => return Ok(0),
                    Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
                    Err(error) => return Err(error),
                }
            }
        }
    }

    /// 🔒 An exclusive lock on a file of the runtime directory that is still the file at its path.
    fn lock_runtime_file(path: &Path) -> std::io::Result<Option<std::fs::File>> {
        for _ in 0..8 {
            let file = std::fs::OpenOptions::new().create(true).read(true).write(true).truncate(false).open(path)?;
            if file.try_lock().is_err() { return Ok(None); }
            if std::fs::metadata(path).is_ok_and(|current| file.metadata().is_ok_and(|held| held.ino() == current.ino() && held.dev() == current.dev())) { return Ok(Some(file)); }
        }
        Ok(None)
    }

    /// 🧹 Removes the sockets of daemons that no longer run: their lock is free.
    fn sweep(directory: &Path, own: &Path) {
        let Ok(entries) = std::fs::read_dir(directory) else { return };
        for entry in entries.flatten().take(1024) {
            let lock = entry.path();
            if lock == own || lock.extension().and_then(std::ffi::OsStr::to_str) != Some("lock") { continue; }
            if let Ok(Some(_held)) = lock_runtime_file(&lock) {
                let _ = std::fs::remove_file(lock.with_extension("sock"));
                let _ = std::fs::remove_file(&lock);
            }
        }
    }

    /// 👂 The daemon's listening socket. Its lock file tells a later daemon that the socket is alive.
    pub struct Listener {
        inner: UnixListener,
        root: PathBuf,
        path: PathBuf,
        lock: PathBuf,
        _held: std::fs::File,
    }

    impl Listener {
        pub fn bind(root: &Path) -> std::io::Result<Self> {
            let path = ipc::socket_path(root);
            let directory = path.parent().ok_or_else(|| std::io::Error::other("dashboard socket needs a parent directory"))?.to_path_buf();
            std::fs::DirBuilder::new().recursive(true).mode(0o700).create(&directory)?;
            let metadata = std::fs::symlink_metadata(&directory)?;
            if !metadata.is_dir() || metadata.uid() != ui_tui::tui::pty::user_id() { return Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, format!("{} is not a directory of this user", directory.display()))); }
            if metadata.mode() & 0o077 != 0 { std::fs::set_permissions(&directory, std::fs::Permissions::from_mode(0o700))?; }
            let lock = path.with_extension("lock");
            let held = lock_runtime_file(&lock)?.ok_or_else(|| std::io::Error::new(std::io::ErrorKind::AlreadyExists, "workspace dashboard daemon is already listening"))?;
            sweep(&directory, &lock);
            let _ = std::fs::remove_file(&path);
            let inner = UnixListener::bind(&path)?;
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))?;
            inner.set_nonblocking(true)?;
            std::fs::create_dir_all(ipc::dashboard_cache_dir(root))?;
            record_endpoint(root, &path.display().to_string())?;
            Ok(Self { inner, root: root.to_path_buf(), path, lock, _held: held })
        }

        pub fn endpoint(&self) -> String { self.path.display().to_string() }

        pub fn accept(&mut self) -> std::io::Result<Option<Stream>> {
            loop {
                match self.inner.accept() {
                    Ok((stream, _)) => return Stream::adopt(stream).map(Some),
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => return Ok(None),
                    Err(error) if matches!(error.kind(), std::io::ErrorKind::Interrupted | std::io::ErrorKind::ConnectionAborted) => continue,
                    Err(error) => return Err(error),
                }
            }
        }
    }

    impl Drop for Listener {
        fn drop(&mut self) {
            forget_endpoint(&self.root, &self.path.display().to_string());
            let _ = std::fs::remove_file(&self.path);
            let _ = std::fs::remove_file(&self.lock);
        }
    }

    /// ⏰ Ends a wait from another thread.
    #[derive(Clone)]
    pub struct Waker {
        writer: std::sync::Arc<UnixStream>,
    }

    impl Waker {
        pub fn wake(&self) { let _ = (&*self.writer).write(&[1]); }

        pub fn watch_stream(&self, _stream: &Stream) {}

        pub fn watch_pty(&self, _pty: &Pty) {}
    }

    /// ⏳ One wait over the listener, the streams and the pseudo-terminals named since `begin`.
    pub struct Poller {
        wake: UnixStream,
        waker: Waker,
        interests: Vec<Interest>,
        tokens: Vec<Option<Token>>,
    }

    impl Poller {
        pub fn new() -> std::io::Result<Self> {
            let (wake, writer) = UnixStream::pair()?;
            wake.set_nonblocking(true)?;
            writer.set_nonblocking(true)?;
            Ok(Self { wake, waker: Waker { writer: std::sync::Arc::new(writer) }, interests: Vec::new(), tokens: Vec::new() })
        }

        pub fn waker(&self) -> Waker { self.waker.clone() }

        pub fn begin(&mut self) {
            self.interests.clear();
            self.tokens.clear();
            self.interests.push(Interest::new(self.wake.as_raw_fd(), true, false));
            self.tokens.push(None);
        }

        pub fn listener(&mut self, listener: &Listener) {
            self.interests.push(Interest::new(listener.inner.as_raw_fd(), true, false));
            self.tokens.push(Some(Token::Listener));
        }

        pub fn stream(&mut self, token: Token, stream: &Stream, read: bool, write: bool) {
            self.interests.push(Interest::new(stream.inner.as_raw_fd(), read, write));
            self.tokens.push(Some(token));
        }

        pub fn pty(&mut self, token: Token, pty: &Pty, read: bool, write: bool) {
            if !read && !write { return; }
            self.interests.push(Interest::new(pty.raw_fd(), read, write));
            self.tokens.push(Some(token));
        }

        /// ⏱️ Blocks until a named source is ready, the waker fires or the timeout passes. A source whose
        /// peer is gone reports readable, so its reader finds the end of the stream.
        pub fn wait(&mut self, timeout: Duration) -> std::io::Result<Vec<Event>> {
            readiness::wait(&mut self.interests, Some(timeout))?;
            let mut events = Vec::new();
            for (interest, token) in self.interests.iter().zip(&self.tokens) {
                match token {
                    None => if interest.readable { let mut drained = [0u8; 256]; while matches!((&self.wake).read(&mut drained), Ok(count) if count == drained.len()) {} },
                    Some(token) => if interest.readable || interest.writable || interest.closed { events.push(Event { token: *token, readable: interest.readable || interest.closed, writable: interest.writable }); },
                }
            }
            Ok(events)
        }
    }
}

#[cfg(windows)]
mod platform {
    use super::*;
    use std::collections::VecDeque;
    use std::ffi::c_void;
    use std::os::windows::ffi::OsStrExt;
    use std::os::windows::fs::OpenOptionsExt;
    use std::os::windows::io::{AsRawHandle, FromRawHandle, OwnedHandle};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{mpsc, Arc, Condvar, Mutex};

    type Handle = *mut c_void;

    const PIPE_ACCESS_DUPLEX: u32 = 0x0000_0003;
    const FILE_FLAG_OVERLAPPED: u32 = 0x4000_0000;
    const FILE_FLAG_FIRST_PIPE_INSTANCE: u32 = 0x0008_0000;
    const PIPE_REJECT_REMOTE_CLIENTS: u32 = 0x0000_0008;
    const PIPE_UNLIMITED_INSTANCES: u32 = 255;
    const PIPE_BUFFER_BYTES: u32 = 64 * 1024;
    const ERROR_FILE_NOT_FOUND: i32 = 2;
    const ERROR_ACCESS_DENIED: i32 = 5;
    const ERROR_PIPE_BUSY: i32 = 231;
    const ERROR_PIPE_CONNECTED: i32 = 535;
    const ERROR_IO_PENDING: i32 = 997;
    const WAIT_OBJECT_0: u32 = 0;
    const INBOUND_BYTES: usize = 1024 * 1024;
    const OUTBOUND_BYTES: usize = 256 * 1024;

    /// 🧾 The Win32 record of one overlapped operation; its event is signalled when the operation completes.
    #[repr(C)]
    struct Overlapped {
        internal: usize,
        internal_high: usize,
        offset: u32,
        offset_high: u32,
        event: Handle,
    }

    extern "system" {
        fn CreateNamedPipeW(name: *const u16, open_mode: u32, pipe_mode: u32, max_instances: u32, out_buffer_size: u32, in_buffer_size: u32, default_time_out: u32, security_attributes: *mut c_void) -> Handle;
        fn ConnectNamedPipe(pipe: Handle, overlapped: *mut Overlapped) -> i32;
        fn ReadFile(handle: Handle, buffer: *mut u8, bytes: u32, read: *mut u32, overlapped: *mut Overlapped) -> i32;
        fn WriteFile(handle: Handle, buffer: *const u8, bytes: u32, written: *mut u32, overlapped: *mut Overlapped) -> i32;
        fn GetOverlappedResult(handle: Handle, overlapped: *mut Overlapped, transferred: *mut u32, wait: i32) -> i32;
        fn CreateEventW(attributes: *mut c_void, manual_reset: i32, initial_state: i32, name: *const u16) -> Handle;
        fn WaitForSingleObject(handle: Handle, milliseconds: u32) -> u32;
        fn CancelIoEx(handle: Handle, overlapped: *mut Overlapped) -> i32;
    }

    /// 🚩 What the blocked threads raise and the waiter sleeps on.
    #[derive(Default)]
    struct Signal {
        raised: Mutex<bool>,
        changed: Condvar,
    }

    impl Signal {
        fn raise(&self) {
            if let Ok(mut raised) = self.raised.lock() { *raised = true; }
            self.changed.notify_all();
        }

        fn wait(&self, timeout: Duration) {
            let Ok(mut raised) = self.raised.lock() else { return };
            if !*raised {
                let Ok((waited, _)) = self.changed.wait_timeout(raised, timeout) else { return };
                raised = waited;
            }
            *raised = false;
        }
    }

    /// 🚰 A bounded byte queue between a stream and the thread blocked on its pipe.
    struct Queue {
        state: Mutex<QueueState>,
        changed: Condvar,
        capacity: usize,
    }

    #[derive(Default)]
    struct QueueState {
        bytes: VecDeque<u8>,
        closed: bool,
    }

    impl Queue {
        fn new(capacity: usize) -> Arc<Self> { Arc::new(Self { state: Mutex::default(), changed: Condvar::new(), capacity }) }

        fn close(&self) {
            if let Ok(mut state) = self.state.lock() { state.closed = true; }
            self.changed.notify_all();
        }
    }

    fn event() -> std::io::Result<OwnedHandle> {
        let handle = unsafe { CreateEventW(std::ptr::null_mut(), 1, 0, std::ptr::null()) };
        if handle.is_null() { return Err(std::io::Error::last_os_error()); }
        Ok(unsafe { OwnedHandle::from_raw_handle(handle.cast()) })
    }

    /// ⏱️ Completes one overlapped read or write and answers how many bytes it moved.
    fn complete(pipe: Handle, started: i32, overlapped: &mut Overlapped, moved: &mut u32) -> bool {
        if started != 0 { return true; }
        if std::io::Error::last_os_error().raw_os_error() != Some(ERROR_IO_PENDING) { return false; }
        unsafe { GetOverlappedResult(pipe, overlapped, moved, 1) != 0 }
    }

    type Watch = Arc<Mutex<Option<Arc<Signal>>>>;

    fn raise(watch: &Watch) {
        let signal = watch.lock().ok().and_then(|slot| slot.clone());
        if let Some(signal) = signal { signal.raise(); }
    }

    /// 🔌 A connected named pipe whose reads and writes never wait: one thread blocks on overlapped
    /// reads, one on overlapped writes, and each side of the stream only touches a bounded queue.
    pub struct Stream {
        pipe: Arc<std::fs::File>,
        inbound: Arc<Queue>,
        outbound: Arc<Queue>,
        watch: Watch,
    }

    impl Stream {
        /// 🤝 Connects to the daemon of a workspace, retrying while the daemon is between two pipe instances.
        pub fn connect(root: &Path) -> std::io::Result<Self> {
            let name = ipc::endpoint(root);
            let deadline = std::time::Instant::now() + Duration::from_secs(1);
            loop {
                match std::fs::OpenOptions::new().read(true).write(true).custom_flags(FILE_FLAG_OVERLAPPED).open(&name) {
                    Ok(file) => return Self::adopt(file),
                    Err(error) if matches!(error.raw_os_error(), Some(ERROR_FILE_NOT_FOUND | ERROR_PIPE_BUSY)) && std::time::Instant::now() < deadline => std::thread::sleep(Duration::from_millis(20)),
                    Err(error) => return Err(error),
                }
            }
        }

        /// 🧵 Takes over a pipe handle opened for overlapped operation.
        pub fn adopt(file: std::fs::File) -> std::io::Result<Self> {
            let stream = Self { pipe: Arc::new(file), inbound: Queue::new(INBOUND_BYTES), outbound: Queue::new(OUTBOUND_BYTES), watch: Watch::default() };
            let (pipe, inbound, outbound, watch, done) = (stream.pipe.clone(), stream.inbound.clone(), stream.outbound.clone(), stream.watch.clone(), event()?);
            std::thread::Builder::new().name("dashboard pipe read".into()).stack_size(128 * 1024).spawn(move || {
                let mut page = vec![0u8; 64 * 1024];
                loop {
                    let mut overlapped = Overlapped { internal: 0, internal_high: 0, offset: 0, offset_high: 0, event: done.as_raw_handle().cast() };
                    let mut read = 0u32;
                    let handle: Handle = pipe.as_raw_handle().cast();
                    let started = unsafe { ReadFile(handle, page.as_mut_ptr(), page.len() as u32, &mut read, &mut overlapped) };
                    let arrived = complete(handle, started, &mut overlapped, &mut read) && read > 0;
                    let Ok(mut state) = inbound.state.lock() else { break };
                    if !arrived || state.closed {
                        state.closed = true;
                        drop(state);
                        outbound.close();
                        raise(&watch);
                        break;
                    }
                    state.bytes.extend(&page[..read as usize]);
                    while state.bytes.len() > inbound.capacity && !state.closed {
                        let Ok(waited) = inbound.changed.wait(state) else { return };
                        state = waited;
                    }
                    drop(state);
                    raise(&watch);
                }
            })?;
            let (pipe, inbound, outbound, watch, done) = (stream.pipe.clone(), stream.inbound.clone(), stream.outbound.clone(), stream.watch.clone(), event()?);
            std::thread::Builder::new().name("dashboard pipe write".into()).stack_size(128 * 1024).spawn(move || loop {
                let chunk: Vec<u8> = {
                    let Ok(mut state) = outbound.state.lock() else { break };
                    while state.bytes.is_empty() && !state.closed {
                        let Ok(waited) = outbound.changed.wait(state) else { return };
                        state = waited;
                    }
                    if state.bytes.is_empty() { break; }
                    let count = state.bytes.len().min(64 * 1024);
                    state.bytes.iter().take(count).copied().collect()
                };
                let mut rest = &chunk[..];
                let handle: Handle = pipe.as_raw_handle().cast();
                while !rest.is_empty() {
                    let mut overlapped = Overlapped { internal: 0, internal_high: 0, offset: 0, offset_high: 0, event: done.as_raw_handle().cast() };
                    let mut written = 0u32;
                    let started = unsafe { WriteFile(handle, rest.as_ptr(), rest.len() as u32, &mut written, &mut overlapped) };
                    if !complete(handle, started, &mut overlapped, &mut written) || written == 0 {
                        outbound.close();
                        inbound.close();
                        raise(&watch);
                        return;
                    }
                    rest = &rest[written as usize..];
                }
                if let Ok(mut state) = outbound.state.lock() { state.bytes.drain(..chunk.len().min(state.bytes.len())); }
                raise(&watch);
            })?;
            Ok(stream)
        }

        /// 📥 `Some(0)` is the end of the stream, `None` means nothing has arrived yet.
        pub fn try_read(&mut self, buf: &mut [u8]) -> std::io::Result<Option<usize>> {
            let mut state = self.inbound.state.lock().map_err(|_| std::io::Error::other("dashboard pipe queue poisoned"))?;
            if state.bytes.is_empty() { return Ok(state.closed.then_some(0)); }
            let count = state.bytes.len().min(buf.len());
            for (slot, byte) in buf.iter_mut().zip(state.bytes.drain(..count)) { *slot = byte; }
            self.inbound.changed.notify_all();
            Ok(Some(count))
        }

        /// 📤 How many bytes the outbound queue took; `0` means it is full for now. Bytes stay queued
        /// until the pipe has taken them, so the queue length is what the peer has not accepted yet.
        pub fn try_write(&mut self, data: &[u8]) -> std::io::Result<usize> {
            let mut state = self.outbound.state.lock().map_err(|_| std::io::Error::other("dashboard pipe queue poisoned"))?;
            if state.closed { return Err(std::io::ErrorKind::BrokenPipe.into()); }
            let count = self.outbound.capacity.saturating_sub(state.bytes.len()).min(data.len());
            state.bytes.extend(&data[..count]);
            self.outbound.changed.notify_all();
            Ok(count)
        }
    }

    impl Drop for Stream {
        fn drop(&mut self) {
            let deadline = std::time::Instant::now() + Duration::from_millis(50);
            while std::time::Instant::now() < deadline && self.outbound.state.lock().is_ok_and(|state| !state.bytes.is_empty() && !state.closed) { std::thread::sleep(Duration::from_millis(2)); }
            self.inbound.close();
            self.outbound.close();
            unsafe { CancelIoEx(self.pipe.as_raw_handle().cast(), std::ptr::null_mut()) };
        }
    }

    fn instance(name: &str, first: bool) -> std::io::Result<std::fs::File> {
        let wide: Vec<u16> = std::ffi::OsStr::new(name).encode_wide().chain(std::iter::once(0)).collect();
        let handle = unsafe { CreateNamedPipeW(wide.as_ptr(), PIPE_ACCESS_DUPLEX | FILE_FLAG_OVERLAPPED | if first { FILE_FLAG_FIRST_PIPE_INSTANCE } else { 0 }, PIPE_REJECT_REMOTE_CLIENTS, PIPE_UNLIMITED_INSTANCES, PIPE_BUFFER_BYTES, PIPE_BUFFER_BYTES, 0, std::ptr::null_mut()) };
        if handle.is_null() || handle as isize == -1 {
            let error = std::io::Error::last_os_error();
            return Err(if error.raw_os_error() == Some(ERROR_ACCESS_DENIED) { std::io::Error::new(std::io::ErrorKind::AlreadyExists, "workspace dashboard daemon is already listening") } else { error });
        }
        Ok(unsafe { std::fs::File::from_raw_handle(handle.cast()) })
    }

    /// 👂 The daemon's named pipe. One thread owns the wait for the next peer and hands every
    /// connected instance over; the first instance is created exclusively, so a second daemon fails.
    pub struct Listener {
        accepted: mpsc::Receiver<std::fs::File>,
        stop: Arc<AtomicBool>,
        acceptor: Option<std::thread::JoinHandle<()>>,
        watch: Watch,
        root: PathBuf,
        name: String,
    }

    impl Listener {
        pub fn bind(root: &Path) -> std::io::Result<Self> {
            let name = ipc::pipe_name(root);
            let first = instance(&name, true)?;
            std::fs::create_dir_all(ipc::dashboard_cache_dir(root))?;
            record_endpoint(root, &name)?;
            let (sender, accepted) = mpsc::channel();
            let (stop, watch, listening, done) = (Arc::new(AtomicBool::new(false)), Watch::default(), name.clone(), event()?);
            let (stopping, watching) = (stop.clone(), watch.clone());
            let acceptor = std::thread::Builder::new().name("dashboard pipe accept".into()).stack_size(128 * 1024).spawn(move || {
                let mut next = Some(first);
                while !stopping.load(Ordering::SeqCst) {
                    let Some(pipe) = next.take().or_else(|| instance(&listening, false).ok()) else { std::thread::sleep(Duration::from_millis(50)); continue };
                    let handle: Handle = pipe.as_raw_handle().cast();
                    let mut overlapped = Overlapped { internal: 0, internal_high: 0, offset: 0, offset_high: 0, event: done.as_raw_handle().cast() };
                    let started = unsafe { ConnectNamedPipe(handle, &mut overlapped) };
                    let failure = std::io::Error::last_os_error().raw_os_error();
                    let mut connected = started != 0 || failure == Some(ERROR_PIPE_CONNECTED);
                    if !connected && failure == Some(ERROR_IO_PENDING) {
                        while !connected && !stopping.load(Ordering::SeqCst) { connected = unsafe { WaitForSingleObject(done.as_raw_handle().cast(), 200) } == WAIT_OBJECT_0; }
                        if !connected { unsafe { CancelIoEx(handle, &mut overlapped) }; }
                    }
                    if connected {
                        if sender.send(pipe).is_err() { return; }
                        raise(&watching);
                    }
                }
            })?;
            Ok(Self { accepted, stop, acceptor: Some(acceptor), watch, root: root.to_path_buf(), name })
        }

        pub fn endpoint(&self) -> String { self.name.clone() }

        pub fn accept(&mut self) -> std::io::Result<Option<Stream>> {
            match self.accepted.try_recv() { Ok(pipe) => Stream::adopt(pipe).map(Some), Err(_) => Ok(None) }
        }
    }

    impl Drop for Listener {
        fn drop(&mut self) {
            self.stop.store(true, Ordering::SeqCst);
            if let Some(acceptor) = self.acceptor.take() { let _ = acceptor.join(); }
            forget_endpoint(&self.root, &self.name);
        }
    }

    /// ⏰ Ends a wait from another thread, and is what streams and pseudo-terminals raise.
    #[derive(Clone)]
    pub struct Waker {
        signal: Arc<Signal>,
    }

    impl Waker {
        pub fn wake(&self) { self.signal.raise(); }

        pub fn watch_stream(&self, stream: &Stream) {
            if let Ok(mut slot) = stream.watch.lock() { *slot = Some(self.signal.clone()); }
        }

        pub fn watch_pty(&self, pty: &Pty) {
            let signal = self.signal.clone();
            pty.set_notifier(Arc::new(move || signal.raise()));
        }
    }

    /// ⏳ One wait over the listener, the streams and the pseudo-terminals named since `begin`. Nothing
    /// here can be asked whether it is ready, so every named source is reported and its queue answers.
    pub struct Poller {
        signal: Arc<Signal>,
        events: Vec<Event>,
    }

    impl Poller {
        pub fn new() -> std::io::Result<Self> { Ok(Self { signal: Arc::default(), events: Vec::new() }) }

        pub fn waker(&self) -> Waker { Waker { signal: self.signal.clone() } }

        pub fn begin(&mut self) { self.events.clear(); }

        pub fn listener(&mut self, listener: &Listener) {
            if let Ok(mut slot) = listener.watch.lock() { if slot.is_none() { *slot = Some(self.signal.clone()); } }
            self.events.push(Event { token: Token::Listener, readable: true, writable: false });
        }

        pub fn stream(&mut self, token: Token, _stream: &Stream, read: bool, write: bool) {
            self.events.push(Event { token, readable: read, writable: write });
        }

        pub fn pty(&mut self, token: Token, _pty: &Pty, read: bool, write: bool) {
            if read || write { self.events.push(Event { token, readable: read, writable: write }); }
        }

        pub fn wait(&mut self, timeout: Duration) -> std::io::Result<Vec<Event>> {
            self.signal.wait(timeout);
            Ok(self.events.clone())
        }
    }
}

#[cfg(any(unix, windows))]
pub use platform::{Listener, Poller, Stream, Waker};
