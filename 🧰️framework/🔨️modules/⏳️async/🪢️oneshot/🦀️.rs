//! 🪢️ Single-value handoff between one `Sender` and one `Receiver` (replaces `tokio::sync::oneshot`).

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex, PoisonError};
use std::task::{Context, Poll, Waker};

struct State<T> {
    value: Option<T>,
    sender_dropped: bool,
    receiver_dropped: bool,
    waker: Option<Waker>,
}

/// 📤️ Sending half — [`Sender::send`] never blocks: it stores the value for a still-live
/// [`Receiver`], or hands it straight back if the receiver already dropped.
pub struct Sender<T> {
    state: Arc<Mutex<State<T>>>,
}

/// 📩️ Receiving half — a [`Future`] that resolves once [`Sender::send`] runs (or reports
/// [`RecvError`] if the sender dropped first), plus a non-blocking [`Receiver::try_recv`] for a
/// caller polling without an executor (the services crate's scope-drain loop).
pub struct Receiver<T> {
    state: Arc<Mutex<State<T>>>,
}

/// 🚫️ [`Sender`] dropped before sending — the value is gone for good.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RecvError;

/// ❌️ [`Receiver::try_recv`]'s two non-blocking outcomes: nothing sent yet, or the [`Sender`]
/// dropped first.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TryRecvError {
    Empty,
    Closed,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let state = Arc::new(Mutex::new(State { value: None, sender_dropped: false, receiver_dropped: false, waker: None }));
    (Sender { state: state.clone() }, Receiver { state })
}

impl<T> Sender<T> {
    /// 📮️ `Err(value)` only if the [`Receiver`] already dropped — the value is handed back
    /// unmodified, matching `tokio::sync::oneshot::Sender::send`'s shape.
    pub fn send(self, value: T) -> Result<(), T> {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if state.receiver_dropped {
            return Err(value);
        }
        state.value = Some(value);
        let waker = state.waker.take();
        drop(state);
        if let Some(waker) = waker {
            waker.wake();
        }
        Ok(())
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if state.value.is_none() {
            state.sender_dropped = true;
            let waker = state.waker.take();
            drop(state);
            if let Some(waker) = waker {
                waker.wake();
            }
        }
    }
}

impl<T> Receiver<T> {
    /// 🆓️ Non-blocking, runtime-context-free poll — [`TryRecvError::Empty`] if the [`Sender`]
    /// hasn't sent yet and is still alive, [`TryRecvError::Closed`] if it dropped without
    /// sending.
    pub fn try_recv(&mut self) -> Result<T, TryRecvError> {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(value) = state.value.take() {
            return Ok(value);
        }
        if state.sender_dropped {
            return Err(TryRecvError::Closed);
        }
        Err(TryRecvError::Empty)
    }
}

impl<T> Future for Receiver<T> {
    type Output = Result<T, RecvError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(value) = state.value.take() {
            return Poll::Ready(Ok(value));
        }
        if state.sender_dropped {
            return Poll::Ready(Err(RecvError));
        }
        state.waker = Some(cx.waker().clone());
        Poll::Pending
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.state.lock().unwrap_or_else(PoisonError::into_inner).receiver_dropped = true;
    }
}
