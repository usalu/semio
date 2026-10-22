//! 🚦️ `Notify` and the owned-permit `Semaphore`: the repo-owned wait primitives.

//#region 🪢️Oneshot
#[path = "../🪢️oneshot/🦀️.rs"]
pub mod oneshot;
//#endregion 🪢️Oneshot

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex, PoisonError};
use std::task::{Context, Poll, Waker};

//#region 🔔️Notify
struct NotifyState {
    permit: bool,
    pub(crate) waker: Option<Waker>,
}

/// 🔔️ Single-permit wake signal — the repo-owned replacement for `tokio::sync::Notify`
/// (INTERACTIVE-JOB-RUNTIME-REFACTOR Phase 9 packet P9a), sized for `TimerWheel`'s sole consumer
/// shape in the services crate: [`Notify::notify_one`] posts one early-wake permit that
/// [`Notify::notified`] consumes, whether the notify arrives before or after the wait starts. Only
/// ever has ONE real waiter in this repo (a timer driver loop) — multi-waiter fairness was
/// deliberately not built, since nothing here needs it.
pub struct Notify {
    pub(crate) state: Mutex<NotifyState>,
}

impl Notify {
    pub fn new() -> Notify {
        Notify { state: Mutex::new(NotifyState { permit: false, waker: None }) }
    }

    pub fn notify_one(&self) {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        state.permit = true;
        let waker = state.waker.take();
        drop(state);
        if let Some(waker) = waker {
            waker.wake();
        }
    }

    pub fn notified(&self) -> Notified<'_> {
        Notified { notify: self }
    }
}

impl Default for Notify {
    fn default() -> Notify {
        Notify::new()
    }
}

/// 🔔️ Future returned by [`Notify::notified`] — resolves once a permit is available, consuming it.
pub struct Notified<'a> {
    notify: &'a Notify,
}

impl Future for Notified<'_> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let mut state = self.notify.state.lock().unwrap_or_else(PoisonError::into_inner);
        if state.permit {
            state.permit = false;
            return Poll::Ready(());
        }
        state.waker = Some(cx.waker().clone());
        Poll::Pending
    }
}
//#endregion 🔔️Notify

//#region 🚦️Semaphore
struct SemaphoreState {
    permits: usize,
    pub(crate) next_id: u64,
    wakers: HashMap<u64, Waker>,
}

/// 🚦️ Counting admission gate — the repo-owned replacement for `tokio::sync::Semaphore`
/// (INTERACTIVE-JOB-RUNTIME-REFACTOR Phase 9 packet P9a), sized for `ComputePool`'s (services
/// crate) local admission bound independent of `WorkerPool`'s own worker count. Never `close`s —
/// nothing in this repo needs a closeable semaphore, so [`Semaphore::acquire_owned`] returns an
/// [`OwnedPermit`] directly rather than a `Result`, unlike `tokio::sync::Semaphore`. Broadcast-wake
/// on release (every pending waiter is woken and re-attempts the grab), not a fair queue — correct
/// at the small (2–8 concurrent waiter) scale every real caller here runs at; each waiter's own
/// [`Waker`] is tracked by id and removed on drop, so a cancelled (raced-away) acquire never leaks a
/// stale waker.
pub struct Semaphore {
    pub(crate) state: Mutex<SemaphoreState>,
}

impl Semaphore {
    pub fn new(permits: usize) -> Semaphore {
        Semaphore { state: Mutex::new(SemaphoreState { permits, next_id: 1, wakers: HashMap::new() }) }
    }

    pub fn acquire_owned(self: &Arc<Self>) -> AcquireOwned {
        AcquireOwned { semaphore: self.clone(), id: None }
    }
}

/// 🚦️ RAII permit returned by [`Semaphore::acquire_owned`] — returns its one permit to the
/// semaphore (and wakes every pending waiter) on drop.
pub struct OwnedPermit {
    semaphore: Arc<Semaphore>,
}

impl Drop for OwnedPermit {
    fn drop(&mut self) {
        let mut state = self.semaphore.state.lock().unwrap_or_else(PoisonError::into_inner);
        state.permits += 1;
        let wakers: Vec<Waker> = state.wakers.drain().map(|(_, waker)| waker).collect();
        drop(state);
        for waker in wakers {
            waker.wake();
        }
    }
}

/// 🚦️ Future returned by [`Semaphore::acquire_owned`].
pub struct AcquireOwned {
    semaphore: Arc<Semaphore>,
    pub(crate) id: Option<u64>,
}

impl Future for AcquireOwned {
    type Output = OwnedPermit;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<OwnedPermit> {
        let this = self.get_mut();
        let mut state = this.semaphore.state.lock().unwrap_or_else(PoisonError::into_inner);
        if state.permits > 0 {
            state.permits -= 1;
            if let Some(id) = this.id.take() {
                state.wakers.remove(&id);
            }
            return Poll::Ready(OwnedPermit { semaphore: this.semaphore.clone() });
        }
        let id = *this.id.get_or_insert_with(|| {
            let id = state.next_id;
            state.next_id += 1;
            id
        });
        state.wakers.insert(id, cx.waker().clone());
        Poll::Pending
    }
}

impl Drop for AcquireOwned {
    fn drop(&mut self) {
        if let Some(id) = self.id {
            self.semaphore.state.lock().unwrap_or_else(PoisonError::into_inner).wakers.remove(&id);
        }
    }
}
//#endregion 🚦️Semaphore
