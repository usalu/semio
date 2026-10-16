//! 🧵️ The renderer worker pool and its bounded native I/O slots: one `RendererIoHandle` per submitted request, the slot ledger and the pump the winit thread and the kernel workers drive.
//!
//! 🧭️ Lifted out of the asset decode jobs into host-base: the decode jobs are one client of this transport, the kernel and the frame loop are others.
use super::*;

//#region 🧵️RendererWorkerPool
/// 🧵️ Resolves the interactive OS process's single worker pool for every renderer subsystem.
#[cfg(not(target_arch = "wasm32"))]
pub fn renderer_worker_pool() -> semio_framework_async::WorkerPool {
    let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
    semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, cores))
}

#[cfg(target_arch = "wasm32")]
pub fn renderer_worker_pool() -> semio_framework_async::WorkerPool {
    semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, 1))
}

#[cfg(not(target_arch = "wasm32"))]
pub struct RendererIoHandle {
    slot: usize,
    pub generation: u64,
    completed: bool,
}

#[cfg(not(target_arch = "wasm32"))]
impl RendererIoHandle {
    fn try_take(&mut self) -> Option<Result<semio_framework_os_services::NativeIoValue, String>> {
        let result = renderer_io_with_node(self.slot, self.generation, |node| {
            let result = node.result.take()?;
            node.result_taken = true;
            RENDERER_IO_WAKE.store(true, Ordering::Release);
            Some(result)
        })
        .flatten();
        self.completed |= result.is_some();
        result
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Future for RendererIoHandle {
    type Output = Result<semio_framework_os_services::NativeIoValue, String>;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        if let Some(result) = self.try_take() {
            return std::task::Poll::Ready(result);
        }
        let registered = renderer_io_with_node(self.slot, self.generation, |node| {
            node.waker = Some(cx.waker().clone());
            if let Some(session) = node.session.as_ref() {
                let _ = session.register_wake(cx.waker());
            }
        })
        .is_some();
        if registered {
            RENDERER_IO_WAKE.store(true, Ordering::Release);
            std::task::Poll::Pending
        } else if renderer_io_generation_live(self.slot, self.generation) {
            RENDERER_IO_WAKE.store(true, Ordering::Release);
            cx.waker().wake_by_ref();
            std::task::Poll::Pending
        } else {
            self.completed = true;
            std::task::Poll::Ready(Err("native I/O mounted session generation is stale".into()))
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for RendererIoHandle {
    fn drop(&mut self) {
        if self.completed {
            return;
        }
        if let Some(slot) = RENDERER_IO_SLOTS.get(self.slot) {
            if slot.generation.load(Ordering::Acquire) == self.generation {
                slot.cancel_requested.store(true, Ordering::Release);
            }
        }
        RENDERER_IO_WAKE.store(true, Ordering::Release);
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub const RENDERER_IO_SESSION_SLOTS: usize = semio_framework_job_worker::WORKER_JOB_SESSION_SLOTS + 1;
#[cfg(not(target_arch = "wasm32"))]
pub const RENDERER_IO_VACANT: u8 = 0;
#[cfg(not(target_arch = "wasm32"))]
pub const RENDERER_IO_LIVE: u8 = 1;
#[cfg(not(target_arch = "wasm32"))]
pub const RENDERER_IO_CHECKED_OUT: u8 = 2;
#[cfg(not(target_arch = "wasm32"))]
pub const RENDERER_IO_EXHAUSTED: u8 = 3;

#[cfg(not(target_arch = "wasm32"))]
pub struct RendererIoSlot {
    generation: AtomicU64,
    state: std::sync::atomic::AtomicU8,
    node: std::sync::atomic::AtomicPtr<RendererIoNode>,
    cancel_requested: AtomicBool,
}

#[cfg(not(target_arch = "wasm32"))]
impl RendererIoSlot {
    const fn vacant() -> Self {
        Self { generation: AtomicU64::new(0), state: std::sync::atomic::AtomicU8::new(RENDERER_IO_VACANT), node: std::sync::atomic::AtomicPtr::new(std::ptr::null_mut()), cancel_requested: AtomicBool::new(false) }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub struct RendererIoNode {
    session: Option<semio_framework_job_worker::WorkerJobSession<semio_framework_os_services::NativeIoJob>>,
    rejected: Option<semio_framework_job_worker::WorkerJobSessionAdmissionRejected<semio_framework_os_services::NativeIoJob>>,
    ticket: Option<semio_framework_job_worker::WorkerJobTicket>,
    result: Option<Result<semio_framework_os_services::NativeIoValue, String>>,
    result_taken: bool,
    detached: bool,
    cancel: semio_framework_async::CancelToken,
    waker: Option<std::task::Waker>,
}

#[cfg(not(target_arch = "wasm32"))]
impl RendererIoNode {
    fn wake(&mut self) {
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
    }

    fn close_detached_result(&mut self) -> bool {
        if !self.detached {
            return false;
        }
        let Some(result) = self.result.as_mut() else { return false };
        match result {
            Ok(value) if !value.terminal_is_empty() => {
                let _ = value.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                return true;
            }
            Ok(_) => {}
            Err(error) if !error.is_empty() => {
                if error.len() > semio_framework_job::JOB_PAYLOAD_PAGE_BYTES {
                    return false;
                }
                error.clear();
                return true;
            }
            Err(_) => {}
        }
        self.result = None;
        self.result_taken = true;
        true
    }

    fn pump_one(&mut self) {
        if self.close_detached_result() {
            self.wake();
            return;
        }
        if let Some(rejected) = self.rejected.as_mut() {
            let _ = rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            if rejected.terminal_is_empty() {
                self.rejected = None;
            }
            self.wake();
            return;
        }
        let Some(session) = self.session.as_ref() else {
            self.wake();
            return;
        };
        let _ = session.take_wake();
        if self.result.is_some() || self.detached {
            if !matches!(session.poll(), semio_framework_job_worker::WorkerJobPoll::Closing | semio_framework_job_worker::WorkerJobPoll::TerminalEmpty) {
                let _ = session.begin_close();
                self.wake();
                return;
            }
            let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            if session.terminal_is_empty() {
                self.session = None;
            }
            self.wake();
            return;
        }
        match session.poll() {
            semio_framework_job_worker::WorkerJobPoll::Idle => match session.try_submit_step(&renderer_worker_pool(), semio_framework_async::Lane::Io) {
                Ok(ticket) => self.ticket = Some(ticket),
                Err(semio_framework_job_worker::WorkerJobSubmitFault::Pool(semio_framework_async::WorkerSubmitErrorKind::Saturated | semio_framework_async::WorkerSubmitErrorKind::Contended)) => {
                    if let Ok(rejected) = session.take_rejected() {
                        rejected.resume();
                    }
                }
                Err(error) => {
                    if let Ok(rejected) = session.take_rejected() {
                        rejected.begin_close();
                    } else {
                        let _ = session.begin_close();
                    }
                    self.result = Some(Err(format!("native I/O worker submission failed: {error:?}")));
                }
            },
            semio_framework_job_worker::WorkerJobPoll::Outcome => {
                let Some(ticket) = self.ticket.take() else {
                    self.result = Some(Err("native I/O outcome lost its exact ticket".into()));
                    let _ = session.begin_close();
                    self.wake();
                    return;
                };
                match session.take_outcome(ticket) {
                    Ok(mut owner) => {
                        if matches!(owner.take_outcome(), semio_framework_job::StepOutcome::Yield) {
                            if owner.resume().is_err() {
                                self.result = Some(Err("native I/O yield could not resume exact owner".into()));
                            }
                        } else {
                            owner.begin_close();
                            self.result = Some(Err("native I/O published a nonterminal retained payload".into()));
                        }
                    }
                    Err(_) => self.result = Some(Err("native I/O outcome handback contention".into())),
                }
            }
            semio_framework_job_worker::WorkerJobPoll::Terminal => match session.take_terminal() {
                Ok(mut owner) => {
                    self.result = owner.job_mut().take_result().or_else(|| Some(Err("native I/O terminal owner had no structured result".into())));
                    owner.begin_close();
                }
                Err(_) => self.result = Some(Err("native I/O terminal handback contention".into())),
            },
            semio_framework_job_worker::WorkerJobPoll::Rejected => {
                if let Ok(rejected) = session.take_rejected() {
                    rejected.resume();
                }
            }
            semio_framework_job_worker::WorkerJobPoll::Closing => {
                let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                if session.terminal_is_empty() {
                    self.session = None;
                }
            }
            semio_framework_job_worker::WorkerJobPoll::Submitted | semio_framework_job_worker::WorkerJobPoll::CheckedOut | semio_framework_job_worker::WorkerJobPoll::TerminalEmpty => {}
        }
        self.wake();
    }

    fn terminal_is_empty(&self) -> bool {
        self.session.is_none() && self.rejected.is_none() && self.result.is_none() && (self.detached || self.result_taken)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub static RENDERER_IO_SLOTS: [RendererIoSlot; RENDERER_IO_SESSION_SLOTS] = [const { RendererIoSlot::vacant() }; RENDERER_IO_SESSION_SLOTS];
#[cfg(not(target_arch = "wasm32"))]
pub static RENDERER_IO_WAKE: AtomicBool = AtomicBool::new(false);
#[cfg(not(target_arch = "wasm32"))]
pub static RENDERER_IO_PUMP_CURSOR: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

#[cfg(not(target_arch = "wasm32"))]
pub fn renderer_io_with_node<R>(index: usize, generation: u64, use_node: impl FnOnce(&mut RendererIoNode) -> R) -> Option<R> {
    let slot = RENDERER_IO_SLOTS.get(index)?;
    if slot.generation.load(Ordering::Acquire) != generation || slot.state.compare_exchange(RENDERER_IO_LIVE, RENDERER_IO_CHECKED_OUT, Ordering::AcqRel, Ordering::Acquire).is_err() {
        return None;
    }
    if slot.generation.load(Ordering::Acquire) != generation {
        slot.state.store(RENDERER_IO_LIVE, Ordering::Release);
        return None;
    }
    let pointer = slot.node.load(Ordering::Acquire);
    let result = (!pointer.is_null()).then(|| use_node(unsafe { &mut *pointer }));
    slot.state.store(RENDERER_IO_LIVE, Ordering::Release);
    result
}

#[cfg(not(target_arch = "wasm32"))]
pub fn renderer_io_generation_live(index: usize, generation: u64) -> bool {
    RENDERER_IO_SLOTS.get(index).is_some_and(|slot| slot.generation.load(Ordering::Acquire) == generation && matches!(slot.state.load(Ordering::Acquire), RENDERER_IO_LIVE | RENDERER_IO_CHECKED_OUT) && !slot.node.load(Ordering::Acquire).is_null())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn pump_renderer_io_sessions(maximum_sessions: usize) -> usize {
    if maximum_sessions == 0 {
        return 0;
    }
    let start = RENDERER_IO_PUMP_CURSOR.fetch_add(maximum_sessions, Ordering::AcqRel);
    let mut advanced = 0;
    for offset in 0..RENDERER_IO_SESSION_SLOTS {
        if advanced == maximum_sessions {
            break;
        }
        let index = start.wrapping_add(offset) % RENDERER_IO_SESSION_SLOTS;
        let slot = &RENDERER_IO_SLOTS[index];
        if slot.state.compare_exchange(RENDERER_IO_LIVE, RENDERER_IO_CHECKED_OUT, Ordering::AcqRel, Ordering::Acquire).is_err() {
            continue;
        }
        let pointer = slot.node.load(Ordering::Acquire);
        if pointer.is_null() {
            slot.state.store(RENDERER_IO_VACANT, Ordering::Release);
            continue;
        }
        let node = unsafe { &mut *pointer };
        let cancel_advanced = slot.cancel_requested.swap(false, Ordering::AcqRel);
        if cancel_advanced {
            node.detached = true;
            node.cancel.cancel_now();
            if let Some(session) = node.session.as_ref() {
                let _ = session.begin_close();
            }
            if let Some(rejected) = node.rejected.as_mut() {
                rejected.begin_close();
            }
            node.wake();
        }
        if !cancel_advanced {
            node.pump_one();
        }
        if node.terminal_is_empty() {
            slot.node.store(std::ptr::null_mut(), Ordering::Release);
            slot.cancel_requested.store(false, Ordering::Release);
            drop(unsafe { Box::from_raw(pointer) });
            slot.state.store(if slot.generation.load(Ordering::Acquire) == u64::MAX { RENDERER_IO_EXHAUSTED } else { RENDERER_IO_VACANT }, Ordering::Release);
        } else {
            slot.state.store(RENDERER_IO_LIVE, Ordering::Release);
        }
        advanced += 1;
    }
    if RENDERER_IO_SLOTS.iter().any(|slot| slot.state.load(Ordering::Acquire) == RENDERER_IO_LIVE) {
        RENDERER_IO_WAKE.store(true, Ordering::Release);
    }
    advanced
}

#[cfg(not(target_arch = "wasm32"))]
pub fn submit_renderer_io(request: semio_framework_os_services::NativeIoRequest) -> Result<RendererIoHandle, String> {
    use semio_framework_job::{allocate_operation_id,root_cancel_token,Generation,InteractiveStage,INTERACTIVE_LANE_FUEL,INTERACTIVE_LANE_WALL_US};
use semio_framework_job_worker_authority::{BatchDriveConfig,BatchJobParams};
    let Some((slot_index, generation)) = RENDERER_IO_SLOTS.iter().enumerate().find_map(|(index, slot)| {
        if slot.state.compare_exchange(RENDERER_IO_VACANT, RENDERER_IO_CHECKED_OUT, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return None;
        }
        let Some(generation) = slot.generation.load(Ordering::Acquire).checked_add(1) else {
            slot.state.store(RENDERER_IO_EXHAUSTED, Ordering::Release);
            return None;
        };
        slot.generation.store(generation, Ordering::Release);
        slot.cancel_requested.store(false, Ordering::Release);
        Some((index, generation))
    }) else {
        return Err("native I/O mounted session registry is exhausted".into());
    };
    let job = semio_framework_os_services::NativeIoJob::new(request);
    let cancel = root_cancel_token();
    let params = BatchJobParams {
        operation: allocate_operation_id(),
        generation: Generation(generation),
        cancel: cancel.clone(),
        config: BatchDriveConfig { retained: semio_framework_job::RetainedCloneGrant::default(), site: "os_renderer_native_io", stage: InteractiveStage::InteractiveStep, fuel_per_step: INTERACTIVE_LANE_FUEL, step_budget_us: INTERACTIVE_LANE_WALL_US },
        now_us: semio_framework_job::default_now_us,
    };
    let (session, rejected, result) = match semio_framework_job_worker::WorkerJobSession::try_new(job, params) {
        Ok(session) => (Some(session), None, None),
        Err(mut rejected) => {
            rejected.begin_close();
            (None, Some(rejected), Some(Err("native I/O retained session registry is full".into())))
        }
    };
    let node = Box::new(RendererIoNode { session, rejected, ticket: None, result, result_taken: false, detached: false, cancel, waker: None });
    RENDERER_IO_SLOTS[slot_index].node.store(Box::into_raw(node), Ordering::Release);
    RENDERER_IO_SLOTS[slot_index].state.store(RENDERER_IO_LIVE, Ordering::Release);
    RENDERER_IO_WAKE.store(true, Ordering::Release);
    Ok(RendererIoHandle { slot: slot_index, generation, completed: false })
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn run_renderer_io(request: semio_framework_os_services::NativeIoRequest) -> Result<semio_framework_os_services::NativeIoValue, String> {
    submit_renderer_io(request)?.await
}

//#endregion 🧵️RendererWorkerPool
