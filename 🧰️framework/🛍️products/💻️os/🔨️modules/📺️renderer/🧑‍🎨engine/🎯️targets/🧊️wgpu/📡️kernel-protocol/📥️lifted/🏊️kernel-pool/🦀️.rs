//! 🏊️ The bounded kernel request queue the pool drains and every client enqueues into.
//!
//! 🧭️ Lifted out of the kernel pool into kernel-protocol: the queue is protocol, the pool is one consumer.

use super::*;

//#region 🎠️KernelRuntime
//#region 🔖️KernelPoolState
pub const KERNEL_REQUEST_QUEUE_CAPACITY: usize = 64;

pub struct KernelRequestQueue {
    pub state: Mutex<KernelRequestQueueState>,
}

struct KernelRequestQueueState {
    slots: Box<[Option<(KernelRequest, Arc<ResponseSlot>)>; KERNEL_REQUEST_QUEUE_CAPACITY]>,
    read: usize,
    write: usize,
    len: usize,
    command_pages: usize,
    command_bytes: usize,
    pub closing: bool,
    consumer_waker: Option<Waker>,
    producer_wakers: Vec<Waker>,
}

impl Default for KernelRequestQueue {
    fn default() -> Self {
        Self {
            state: Mutex::new(KernelRequestQueueState {
                slots: semio_framework_async::boxed_fixed_slots(|| None),
                read: 0,
                write: 0,
                len: 0,
                command_pages: 0,
                command_bytes: 0,
                closing: false,
                consumer_waker: None,
                producer_wakers: Vec::with_capacity(KERNEL_REQUEST_QUEUE_CAPACITY),
            }),
        }
    }
}

impl KernelRequestQueue {
    /// 📥️ Admits one request, or hands it back with its producer's wake already arranged: a queue
    /// that is full (or over its command credit) keeps every distinct waiting producer's wake and
    /// wakes them all when a request leaves it, and a queue whose lock is momentarily held asks the
    /// producer to retry at once. A producer parked without either wake was never polled again — a
    /// detached render hung for the rest of a 180 s law on one contended push, and a second waiting
    /// producer used to overwrite the first one's wake (ticket 26/09/23 slice WG8, session 12).
    pub fn try_push(&self, request: KernelRequest, slot: Arc<ResponseSlot>, producer: Option<&Waker>) -> Result<(), (KernelRequest, Arc<ResponseSlot>)> {
        let (pages, bytes) = request.command_credits();
        let Ok(mut state) = self.state.try_lock() else {
            if let Some(producer) = producer {
                producer.wake_by_ref();
            }
            return Err((request, slot));
        };
        let admitted_pages = state.command_pages.checked_add(pages).filter(|total| *total <= semio_framework::kernel::COMMAND_MAXIMUM_PAGES);
        let admitted_bytes = state.command_bytes.checked_add(bytes).filter(|total| *total <= semio_framework::kernel::COMMAND_MAXIMUM_BYTES);
        if state.closing || state.len == KERNEL_REQUEST_QUEUE_CAPACITY || admitted_pages.is_none() || admitted_bytes.is_none() {
            let retry = match producer {
                Some(producer) if !state.producer_wakers.iter().any(|kept| kept.will_wake(producer)) => {
                    if state.producer_wakers.len() < KERNEL_REQUEST_QUEUE_CAPACITY {
                        state.producer_wakers.push(producer.clone());
                        None
                    } else {
                        Some(producer)
                    }
                }
                _ => None,
            };
            drop(state);
            if let Some(producer) = retry {
                producer.wake_by_ref();
            }
            return Err((request, slot));
        }
        let index = state.write;
        assert!(state.slots[index].is_none(), "fixed kernel request write slot is empty after admission");
        state.slots[index] = Some((request, slot));
        state.write = (index + 1) % KERNEL_REQUEST_QUEUE_CAPACITY;
        state.len += 1;
        state.command_pages = admitted_pages.expect("command page credit was admitted");
        state.command_bytes = admitted_bytes.expect("command byte credit was admitted");
        let consumer = state.consumer_waker.take();
        drop(state);
        if let Some(waker) = consumer {
            waker.wake();
        }
        Ok(())
    }

    pub fn poll(&self, cx: &mut Context<'_>) -> Poll<(KernelRequest, Arc<ResponseSlot>)> {
        let Ok(mut state) = self.state.try_lock() else {
            cx.waker().wake_by_ref();
            return Poll::Pending;
        };
        if state.len == 0 {
            state.consumer_waker = Some(cx.waker().clone());
            return Poll::Pending;
        }
        let index = state.read;
        let request = state.slots[index].take().expect("fixed kernel request read slot is occupied");
        let (pages, bytes) = request.0.command_credits();
        state.read = (index + 1) % KERNEL_REQUEST_QUEUE_CAPACITY;
        state.len -= 1;
        state.command_pages -= pages;
        state.command_bytes -= bytes;
        let producers: Vec<Waker> = state.producer_wakers.drain(..).collect();
        drop(state);
        producers.into_iter().for_each(Waker::wake);
        Poll::Ready(request)
    }

    pub fn try_next(&self) -> Option<(KernelRequest, Arc<ResponseSlot>)> {
        let mut state = self.state.try_lock().ok()?;
        if state.len == 0 {
            return None;
        }
        let index = state.read;
        let request = state.slots[index].take().expect("fixed kernel request read slot is occupied");
        let (pages, bytes) = request.0.command_credits();
        state.read = (index + 1) % KERNEL_REQUEST_QUEUE_CAPACITY;
        state.len -= 1;
        state.command_pages -= pages;
        state.command_bytes -= bytes;
        let producers: Vec<Waker> = state.producer_wakers.drain(..).collect();
        drop(state);
        producers.into_iter().for_each(Waker::wake);
        Some(request)
    }

    pub async fn next(&self) -> (KernelRequest, Arc<ResponseSlot>) {
        std::future::poll_fn(|cx| self.poll(cx)).await
    }

    /// 🎚️ Pops the head request only when `admit` accepts it — FIFO and the command credits stay exactly as
    /// [`Self::try_next`] keeps them; a head that may not run now stays the head.
    pub fn try_next_if(&self, admit: impl Fn(&KernelRequest) -> bool) -> Option<(KernelRequest, Arc<ResponseSlot>)> {
        {
            let state = self.state.try_lock().ok()?;
            let head = state.slots[state.read].as_ref().filter(|_| state.len != 0)?;
            if !admit(&head.0) {
                return None;
            }
        }
        self.try_next()
    }

    /// 🔔️ Wakes `waker` when the next request is admitted — the loop's own wait, borrowed by a request that
    /// serves other instances while it waits for its own work ([`KernelPoolState::compile_serving_others`]).
    pub fn watch(&self, waker: &Waker) {
        if let Ok(mut state) = self.state.try_lock() {
            state.consumer_waker = Some(waker.clone());
        }
    }

    /// 🔔️ Wakes the loop so a request waiting in a gap re-reads its requester's cancel.
    pub fn wake_consumer(&self) {
        let waker = self.state.try_lock().ok().and_then(|mut state| state.consumer_waker.take());
        if let Some(waker) = waker {
            waker.wake();
        }
    }

    /// 👀️ Whether the head request satisfies `test` (`false` for an empty or contended queue).
    pub fn head_is(&self, test: impl Fn(&KernelRequest) -> bool) -> bool {
        self.state.try_lock().is_ok_and(|state| state.len != 0 && state.slots[state.read].as_ref().is_some_and(|head| test(&head.0)))
    }

    pub async fn enqueue_retained(&self, request: KernelRequest, slot: Arc<ResponseSlot>) {
        let mut owner = Some((request, slot));
        std::future::poll_fn(|cx| {
            let (request, slot) = owner.take().expect("retained queue admission owner is present");
            match self.try_push(request, slot, Some(cx.waker())) {
                Ok(()) => Poll::Ready(()),
                Err(rejected) => {
                    owner = Some(rejected);
                    Poll::Pending
                }
            }
        })
        .await;
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn begin_shutdown(&self) -> bool {
        let Ok(mut state) = self.state.try_lock() else {
            return false;
        };
        state.closing = true;
        true
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn shutdown_step(&self, maximum_bytes: usize) -> (bool, usize, usize) {
        let Ok(mut state) = self.state.try_lock() else {
            return (false, 0, 0);
        };
        if !state.closing || state.len == 0 {
            return (state.closing && state.len == 0, 0, 0);
        }
        let index = state.read;
        let (terminal, processed, released, page_released) = match &mut state.slots[index].as_mut().expect("fixed shutdown request slot is occupied").0 {
            KernelRequest::ExchangeCommands { driver, .. } => {
                let before = driver.remaining_pages();
                let (terminal, released) = driver.close_step(maximum_bytes);
                let page_released = usize::from(driver.remaining_pages() < before);
                (terminal, usize::from(terminal || page_released != 0), released, page_released)
            }
            KernelRequest::CloseRejectedCommandBuild { owner, .. } => {
                let before = owner.remaining_pages();
                let (terminal, released) = owner.close_step(maximum_bytes);
                let page_released = usize::from(owner.remaining_pages() < before);
                (terminal, usize::from(terminal || page_released != 0), released, page_released)
            }
            KernelRequest::CreateApp { owner } => {
                owner.begin_close();
                let terminal=owner.terminal_is_empty();
                (terminal,usize::from(terminal),0,0)
            }
            KernelRequest::DestroyApp { owner } => {
                owner.finish(KernelCloseStatus::Fault);
                (true, 1, 0, 0)
            }
            KernelRequest::CloseRealm { owner } => {
                owner.finish(KernelCloseStatus::Fault);
                (true, 1, 0, 0)
            }
            KernelRequest::Exchange { event, .. } => {
                let (terminal, processed, released) = event.close_step(maximum_bytes);
                (terminal, processed, released, 0)
            }
            KernelRequest::AdvanceRetained { .. } | KernelRequest::MountProductReplay { .. } | KernelRequest::RetireProductReplay { .. } | KernelRequest::RetireProductReplayRefusal { .. } | KernelRequest::AdvanceProductReplay { .. } => {
                (true, 1, 0, 0)
            }
            KernelRequest::CloseRejectedEvents { owner } => {
                let (terminal, processed) = owner.close_step();
                (terminal, processed, 0, 0)
            }
            KernelRequest::AcknowledgeJobProgress { token } => {
                let returned = job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").return_lease(*token);
                (returned, usize::from(returned), 0, 0)
            }
        };
        state.command_pages -= page_released;
        state.command_bytes -= released;
        if terminal {
            let terminal = state.slots[index].take().expect("terminal shutdown request was present");
            state.read = (index + 1) % KERNEL_REQUEST_QUEUE_CAPACITY;
            state.len -= 1;
            drop(state);
            drop(terminal);
            return (self.state.try_lock().is_ok_and(|state| state.len == 0), processed, released);
        }
        (false, processed, released)
    }
}
//#endregion 🔖️KernelPoolState
//#endregion 🎠️KernelRuntime
