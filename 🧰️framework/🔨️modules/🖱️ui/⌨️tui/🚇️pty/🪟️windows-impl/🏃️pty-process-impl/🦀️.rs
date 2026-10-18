//! 🏃️ Windows pty process: the `Pty` spawn, reader, writer, resize, job object and exit handling with its write and drop implementations. See [`super`].

use super::*;

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
