/// 🚧️ Blocking executors and fixture helpers: the blocking and deferred executor impls and the ledger, page and job helpers.

impl DbIoTaskExecutor for BlockingFaultLawExecutor {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn execute_step(&self, _operation: u64, _task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
        if self.panic {
            panic!("hostile DB I/O fixture panic");
        }
        Err(DbError::Internal("hostile DB I/O fixture backend fault".to_string()))
    }
    fn drive_async(self: Box<Self>, _operation: u64, task: DbIoTask) -> DbIoAsyncDriverFuture {
        Box::pin(async move {
            let executor: Box<dyn DbIoTaskExecutor> = self;
            (executor, task, Err(DbError::Internal("blocking fault fixture has no async driver".to_string())))
        })
    }
    fn close_operation_step(&self, _operation: u64, _task: &DbIoTask) -> Result<bool, DbError> {
        Ok(true)
    }
    fn close_backend_step(&mut self, _context: &mut std::task::Context<'_>) -> Result<bool, DbError> {
        self.terminal = true;
        Ok(true)
    }
    fn backend_terminal_is_empty(&self) -> bool {
        self.terminal
    }
}

impl DbIoTaskExecutor for BlockingCompleteLawExecutor {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn execute_step(&self, _operation: u64, _task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
        Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
    }
    fn drive_async(self: Box<Self>, _operation: u64, task: DbIoTask) -> DbIoAsyncDriverFuture {
        Box::pin(async move {
            let executor: Box<dyn DbIoTaskExecutor> = self;
            (executor, task, Err(DbError::Internal("blocking completion fixture has no async driver".to_string())))
        })
    }
    fn close_operation_step(&self, _operation: u64, _task: &DbIoTask) -> Result<bool, DbError> {
        Ok(true)
    }
    fn close_backend_step(&mut self, _context: &mut std::task::Context<'_>) -> Result<bool, DbError> {
        self.terminal = true;
        Ok(true)
    }
    fn backend_terminal_is_empty(&self) -> bool {
        self.terminal
    }
}

impl DbIoTaskExecutor for DeferredBackendCloseLawExecutor {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn execute_step(&self, _operation: u64, _task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
        Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
    }
    fn drive_async(self: Box<Self>, _operation: u64, task: DbIoTask) -> DbIoAsyncDriverFuture {
        Box::pin(async move {
            let executor: Box<dyn DbIoTaskExecutor> = self;
            (executor, task, Err(DbError::Internal("deferred backend close fixture has no async driver".to_string())))
        })
    }
    fn close_operation_step(&self, _operation: u64, _task: &DbIoTask) -> Result<bool, DbError> {
        Ok(true)
    }
    fn close_backend_step(&mut self, _context: &mut std::task::Context<'_>) -> Result<bool, DbError> {
        if !self.allow_close.load(std::sync::atomic::Ordering::Acquire) {
            return Ok(false);
        }
        self.terminal = true;
        Ok(true)
    }
    fn backend_terminal_is_empty(&self) -> bool {
        self.terminal
    }
}

impl DbIoTaskExecutor for BlockingOutputLifecycleLawExecutor {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn execute_step(&self, _operation: u64, task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
        let DbIoTask::PayloadGet { hash, output, .. } = task else {
            return Err(DbError::Internal("output lifecycle fixture received the wrong task".to_string()));
        };
        let counter = match hash.0[0] {
            0x71 => &self.success_steps,
            0x72 => &self.cancel_steps,
            0x73 => &self.abandon_steps,
            _ => return Err(DbError::Internal("output lifecycle fixture received an unknown scenario".to_string())),
        };
        counter.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        {
            let arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if !output.pages.iter().take(output.reserved as usize).flatten().all(|page| arena.slots[page.slot as usize].phase == DbIoPagePhase::Executing) {
                return Err(DbError::Internal("output lifecycle fixture entered a turn outside Executing".to_string()));
            }
        }
        let total = DB_IO_PAGE_BYTES + 1;
        if output.len() < total {
            let start = output.len();
            let end = start.saturating_add(DB_IO_PAGE_BYTES).min(total);
            let fragment: Vec<u8> = (start..end).map(|index| (index % 251) as u8).collect();
            if output.write_fragment(&fragment)? != fragment.len() {
                return Err(DbError::Internal("output lifecycle fixture lost an admitted fragment".to_string()));
            }
            return Ok((DbIoExecutionStep::Yield, None));
        }
        if hash.0[0] != 0x71 {
            return Ok((DbIoExecutionStep::Yield, None));
        }
        match output.seal_retained_step()? {
            Some(pages) => Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Pages(pages)))),
            None => Ok((DbIoExecutionStep::Yield, None)),
        }
    }
    fn drive_async(self: Box<Self>, _operation: u64, task: DbIoTask) -> DbIoAsyncDriverFuture {
        Box::pin(async move {
            let executor: Box<dyn DbIoTaskExecutor> = self;
            (executor, task, Err(DbError::Internal("blocking output lifecycle fixture has no async driver".to_string())))
        })
    }
    fn close_operation_step(&self, _operation: u64, _task: &DbIoTask) -> Result<bool, DbError> {
        Ok(true)
    }
    fn close_backend_step(&mut self, _context: &mut std::task::Context<'_>) -> Result<bool, DbError> {
        self.terminal = true;
        Ok(true)
    }
    fn backend_terminal_is_empty(&self) -> bool {
        self.terminal
    }
}

fn ledger_witness() -> (DbIoCredit, usize) {
    db_io_ledger_census(DB_IO_LEDGER_OWNER.with(std::cell::Cell::get))
}

fn exact_fixture_result(terminal: Result<DbIoResultLease, DbIoFault>, context: &str) -> DbIoResult {
    match terminal {
        Ok(lease) => match lease.into_result() {
            Ok(result) => result,
            Err(error) => panic!("{context} result handback failed: {error}"),
        },
        Err(mut fault) => {
            while fault.close_step() {}
            panic!("{context} task faulted")
        }
    }
}

fn pages(bytes: &[u8]) -> DbIoPages {
    let mut writer = DbIoPageWriter::try_reserve(bytes.len().div_ceil(DB_IO_PAGE_BYTES)).expect("fixture pages admitted");
    for fragment in bytes.chunks(DB_IO_PAGE_BYTES) {
        assert_eq!(writer.write_fragment(fragment).unwrap(), fragment.len());
    }
    writer.finish().unwrap()
}

fn drain_pages(mut pages: DbIoPages) {
    while pages.close_step().unwrap().is_some() {}
    while db_io_page_maintenance_step().unwrap().is_some() {}
}

fn admit_fixture_job(pool: &WorkerPool, lane: Lane, mut job: semio_framework_async::Job) {
    loop {
        match pool.try_submit(lane, job) {
            Ok(()) => return,
            Err(error) if error.kind() == WorkerSubmitErrorKind::Contended => {
                job = error.into_job();
                std::thread::yield_now();
            }
            Err(error) => panic!("fixture job admission refused: {:?}", error.kind()),
        }
    }
}

async fn drain_control_tasks(control: DbIoBackendControl) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while DB_IO_TASK_SLOTS.iter().any(|task| task.lock().unwrap_or_else(std::sync::PoisonError::into_inner).backend == Some(control)) {
        assert!(std::time::Instant::now() < deadline, "closed tasks did not retire through the backend hook or an owner maintenance opportunity");
        db_io_maintenance_step().unwrap();
        semio_framework_async::yield_once().await;
    }
}
