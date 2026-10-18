//! 🔁️ Task operation futures: the task operation impls, the retirement wait and the test pool helpers. See [`super`].

use super::*;

//#region 🔖️Limits
impl DbIoTaskOperation {
    /// 🧹 Finishes one retained cleanup opportunity per poll before storage-result handoff.
    pub async fn finish(self) -> Result<DbIoResult, DbError> {
        let handle = self.handle;
        match self.await {
            Ok(lease) => {
                let result = lease.into_result()?;
                db_io_wait_task_retirement(handle).await?;
                Ok(result)
            }
            Err(fault) => {
                let error = fault.into_db_error();
                db_io_wait_task_retirement(handle).await?;
                Err(error)
            }
        }
    }

    pub fn phase(&self) -> DbIoTaskPhase {
        let owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if db_io_slot_matches(&owner, self.handle) {
            owner.phase
        } else {
            DbIoTaskPhase::Closing
        }
    }

    pub fn cancel(&self) -> Result<(), DbError> {
        let mut owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !db_io_slot_matches(&owner, self.handle) {
            return Err(DbError::StaleGeneration { expected: GenerationId(self.handle.generation), actual: GenerationId(owner.generation) });
        }
        owner.cancelled = true;
        let mut waker = None;
        if owner.async_ready {
            if let Some(task) = owner.task.as_ref() {
                task.transition_pages(DbIoPagePhase::Executing, DbIoPagePhase::TerminalResult)?;
            }
            owner.async_ready = false;
            owner.phase = DbIoTaskPhase::Cancelled;
            owner.terminal = Some(DbIoTerminal::Cancelled(None));
            waker = db_io_wake(&mut owner);
        }
        drop(owner);
        db_io_enqueue_close(self.handle)?;
        if let Some(waker) = waker {
            waker.wake();
        }
        Ok(())
    }

    pub async fn take_async_native(&mut self) -> Result<DbIoAsyncTaskLease, DbError> {
        std::future::poll_fn(|context| {
            let (task, backend) = {
                let mut owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                if !db_io_slot_matches(&owner, self.handle) {
                    return std::task::Poll::Ready(Err(DbError::StaleGeneration { expected: GenerationId(self.handle.generation), actual: GenerationId(owner.generation) }));
                }
                if !owner.async_ready {
                    owner.waker = Some(context.waker().clone());
                    return std::task::Poll::Pending;
                }
                owner.async_ready = false;
                owner.async_detached = true;
                let Some(backend) = owner.backend else {
                    return std::task::Poll::Ready(Err(DbError::Internal("DB I/O admitted async task lost its backend".to_string())));
                };
                let Some(task) = owner.task.take() else {
                    return std::task::Poll::Ready(Err(DbError::Internal("DB I/O async-ready slot lost its typed task".to_string())));
                };
                (task, backend)
            };
            match db_io_take_async_executor(backend, self.handle.operation) {
                Ok(executor) => match executor.pin_writer_operation(self.handle.operation, &task).and_then(|()| db_io_operation_add(self.handle.operation, db_io_async_lease_credit())) {
                    Ok(()) => std::task::Poll::Ready(Ok(DbIoAsyncTaskLease { handle: self.handle, backend, task: Some(task), executor: Some(executor), completed: false, credit_returned: false })),
                    Err(error) => {
                        let _ = db_io_return_async_executor(backend, self.handle.operation, executor);
                        let mut owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                        owner.task = Some(task);
                        owner.async_ready = true;
                        owner.async_detached = false;
                        std::task::Poll::Ready(Err(error))
                    }
                },
                Err(error) => {
                    let mut owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    owner.task = Some(task);
                    owner.async_ready = true;
                    owner.async_detached = false;
                    std::task::Poll::Ready(Err(error))
                }
            }
        })
        .await
    }

    pub async fn start_async_native_on_lane_io(&mut self) -> Result<(), DbError> {
        self.take_async_native().await?.start_on_lane_io()
    }

    pub fn resume(&self) -> Result<bool, DbError> {
        let retry_generation = {
            let owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if !db_io_slot_matches(&owner, self.handle) {
                return Err(DbError::StaleGeneration { expected: GenerationId(self.handle.generation), actual: GenerationId(owner.generation) });
            }
            if owner.retry_attempt.is_none() {
                return Ok(false);
            }
            owner.retry_generation
        };
        db_io_retry(self.handle, retry_generation);
        Ok(true)
    }

    pub fn take(&mut self) -> Result<Option<Result<DbIoResultLease, DbIoFault>>, DbError> {
        let cancelled_result_pending = {
            let owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            db_io_slot_matches(&owner, self.handle) && matches!(owner.terminal, Some(DbIoTerminal::Cancelled(Some(_))))
        };
        if cancelled_result_pending {
            db_io_enqueue_close(self.handle)?;
            return Ok(None);
        }
        {
            let owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if !db_io_slot_matches(&owner, self.handle) {
                return Err(DbError::StaleGeneration { expected: GenerationId(self.handle.generation), actual: GenerationId(owner.generation) });
            }
            if owner.terminal.is_none() {
                return Ok(None);
            }
        }
        db_io_enqueue_close(self.handle)?;
        db_io_operation_add_result_lease(self.handle.operation)?;
        let terminal = {
            let mut owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if !db_io_slot_matches(&owner, self.handle) {
                drop(owner);
                db_io_operation_return_result_lease(self.handle.operation, DbIoCredit::default())?;
                let owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                return Err(DbError::StaleGeneration { expected: GenerationId(self.handle.generation), actual: GenerationId(owner.generation) });
            }
            let Some(terminal) = owner.terminal.take() else {
                drop(owner);
                db_io_operation_return_result_lease(self.handle.operation, DbIoCredit::default())?;
                return Err(DbError::Internal("DB I/O terminal owner changed during exact take".to_string()));
            };
            let retained_credit = match &terminal {
                DbIoTerminal::Result(result) => result.retained_credit(),
                _ => DbIoCredit::default(),
            };
            owner.result_retained_credit = retained_credit;
            owner.phase = DbIoTaskPhase::Closing;
            (terminal, retained_credit)
        };
        self.resolved = true;
        let (terminal, retained_credit) = terminal;
        Ok(Some(match terminal {
            DbIoTerminal::Result(result) => Ok(DbIoResultLease { handle: self.handle, retained_credit, result: Some(result), transferred: false }),
            DbIoTerminal::Fault(mut fault) => {
                fault.result_handback = Some(DbIoResultHandback { handle: self.handle, retained_credit });
                Err(fault)
            }
            DbIoTerminal::Cancelled(None) => {
                let mut fault = db_io_literal_fault(DbIoFaultKind::Cancelled, DbIoFaultCause::Closed, "DB I/O task cancelled");
                fault.result_handback = Some(DbIoResultHandback { handle: self.handle, retained_credit });
                Err(fault)
            }
            DbIoTerminal::Cancelled(Some(_)) => return Err(DbError::Internal("DB I/O cancellation result escaped retained close".to_string())),
        }))
    }
}

impl Future for DbIoTaskOperation {
    type Output = Result<DbIoResultLease, DbIoFault>;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let _ = db_io_maintenance_step();
        match self.take() {
            Ok(Some(terminal)) => return std::task::Poll::Ready(terminal),
            Ok(None) => {}
            Err(error) => return std::task::Poll::Ready(Err(db_io_task_fault(DbIoFaultKind::Stale, &error))),
        }
        let terminal_published = {
            let mut owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if !db_io_slot_matches(&owner, self.handle) {
                return std::task::Poll::Ready(Err(db_io_literal_fault(DbIoFaultKind::Stale, DbIoFaultCause::Internal, "stale DB I/O terminal handle")));
            }
            if owner.terminal.is_some() && !matches!(owner.terminal, Some(DbIoTerminal::Cancelled(Some(_)))) {
                true
            } else {
                owner.waker = Some(context.waker().clone());
                false
            }
        };
        if terminal_published {
            return std::task::Poll::Ready(match self.take() {
                Ok(Some(terminal)) => terminal,
                Ok(None) => Err(db_io_literal_fault(DbIoFaultKind::Stale, DbIoFaultCause::Internal, "DB I/O terminal owner changed during poll")),
                Err(error) => Err(db_io_task_fault(DbIoFaultKind::Stale, &error)),
            });
        }
        std::task::Poll::Pending
    }
}

pub(super) async fn db_io_wait_task_retirement(handle: DbIoTaskHandle) -> Result<(), DbError> {
    std::future::poll_fn(move |context| {
        db_io_maintenance_step()?;
        let backend = {
            let owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if !db_io_slot_matches(&owner, handle) {
                return std::task::Poll::Ready(Ok(()));
            }
            owner.backend
        };
        if let Some(fault) = backend.and_then(db_io_take_backend_cleanup_fault) {
            return std::task::Poll::Ready(Err(fault));
        }
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    })
    .await
}

/// 🧹 One fixed mounted DB I/O retry, page, platform or terminal-close opportunity.
pub fn db_io_maintenance_step() -> Result<bool, DbError> {
    db_io_maintenance_turn(&DB_IO_MAINTENANCE_CURSOR, |class| match class {
        0 => db_io_lost_owner_close_step(),
        1 => Ok(db_io_page_maintenance_step()?.is_some()),
        2 => db_io_platform_maintenance_step(),
        3 => Ok(db_io_retry_maintenance_step()),
        4 => db_io_rejected_backend_maintenance_step(),
        5 => Ok(db_io_task_close_step()?.is_some()),
        _ => unreachable!(),
    })
}

impl Drop for DbIoTaskOperation {
    fn drop(&mut self) {
        if self.resolved {
            return;
        }
        let mut owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if db_io_slot_matches(&owner, self.handle) {
            owner.abandoned = true;
            owner.cancelled = true;
            owner.waker = owner.backend.and_then(crate::retirement_waker);
            if owner.async_ready {
                if let Some(task) = owner.task.as_ref() {
                    let _ = task.transition_pages(DbIoPagePhase::Executing, DbIoPagePhase::TerminalResult);
                }
                owner.async_ready = false;
                owner.phase = DbIoTaskPhase::Cancelled;
                owner.terminal = Some(DbIoTerminal::Cancelled(None));
            }
        }
        drop(owner);
        let _ = db_io_enqueue_close(self.handle);
    }
}
//#endregion 🔖️Limits

//#region 🔖️RetainedDbIo
/// 🏝️ A law that saturates or asserts a process-global capacity (a fixed admission table, a
/// retirement ring, the page arena, the backend registry) owns a whole process: inside a shared
/// test process it re-runs itself as the only law of a child process and reports that child's
/// verdict; a runner that already gives every law its own process (nextest) runs it in place.
/// Returns `true` where the law body must run.
#[cfg(any(test, feature = "testkit"))]
pub fn process_isolated_law(law: &str) -> bool {
    const ISOLATED_LAW: &str = "SEMIO_DB_ISOLATED_LAW";
    if std::env::var_os("NEXTEST").is_some() || std::env::var(ISOLATED_LAW).is_ok_and(|isolated| isolated == law) {
        return true;
    }
    const ISOLATED_LAW_WATCHDOG: std::time::Duration = std::time::Duration::from_secs(300);
    let executable = std::env::current_exe().expect("process-isolated law resolves its test executable");
    let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_nanos());
    let log_path = std::env::temp_dir().join(format!("semio-db-isolated-law-{}-{nonce}.log", std::process::id()));
    let log = std::fs::File::create(&log_path).expect("process-isolated law creates its child log");
    let mut child = std::process::Command::new(executable)
        .args(["--exact", law, "--test-threads=1"])
        .env(ISOLATED_LAW, law)
        .stdout(log.try_clone().expect("process-isolated law shares its child log"))
        .stderr(log)
        .spawn()
        .expect("process-isolated law spawns its child process");
    let deadline = std::time::Instant::now() + ISOLATED_LAW_WATCHDOG;
    let status = loop {
        if let Some(status) = child.try_wait().expect("process-isolated law observes its child") {
            break Some(status);
        }
        if std::time::Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            break None;
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    };
    let output = std::fs::read_to_string(&log_path).unwrap_or_default();
    let _ = std::fs::remove_file(&log_path);
    assert!(status.is_some_and(|status| status.success()) && output.contains("test result: ok. 1 passed"), "process-isolated law {law} did not pass as the only law of its own process ({status:?}):\n{output}");
    false
}

#[doc(hidden)]
pub fn db_io_test_pool() -> Arc<WorkerPool> {
    static POOL: std::sync::OnceLock<Arc<WorkerPool>> = std::sync::OnceLock::new();
    POOL.get_or_init(|| Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)))).clone()
}

/// ✅️ Test-only sync/async bridge. 🚫️async: E5 executor bridge — poll-once: every future
/// this crate's own backends (`MemoryStorage`, and `FsStorage`/`DbBackend` driven by
/// `semio_framework_async::testkit::ManualRuntime`, whose `run_blocking` executes synchronously)
/// hand back is already `Ready` the instant it's first polled — there is no real async wait
/// anywhere in a unit test. This drives one to completion without needing a real executor,
/// mirroring `semio_framework_async`'s own `ManualRuntime` test helper. The crate's single such
/// bridge (R2 E5: "at most one per crate"); [`block_on_ready`] is a thin `Result`-shaped wrapper
/// over it, not a second one.
#[cfg(any(test, feature = "testkit"))]
pub async fn poll_once<T>(fut: impl Future<Output = T>) -> T {
    fut.await
}

#[cfg(any(test, feature = "testkit"))]
pub async fn block_on_ready<T>(fut: impl Future<Output = Result<T, DbError>>) -> Result<T, DbError> {
    poll_once(fut).await
}
//#endregion 🔖️RetainedDbIo
