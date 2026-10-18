//! 📬️ Artifact mailbox: the mailbox limits, send error, slots, state and authority and the bounded sender, receiver and close handle pair. See [`super`].

use super::*;

//#region 🔖️Protocol
pub const ARTIFACT_MAILBOX_ITEMS: usize = 64;

pub const ARTIFACT_MAILBOX_BYTES: usize = semio_framework_os_spr::DOCUMENT_ARCHIVE_MAXIMUM_BYTES + 65_536;

#[derive(Debug)]
pub enum ArtifactMailboxSendError {
    Full { message: ArtifactActorMsg },
    Bytes { message: ArtifactActorMsg },
    Closed { message: ArtifactActorMsg },
    Stale { message: ArtifactActorMsg },
}

impl ArtifactMailboxSendError {
    pub fn into_message(self) -> ArtifactActorMsg {
        match self {
            Self::Full { message } | Self::Bytes { message } | Self::Closed { message } | Self::Stale { message } => message,
        }
    }
}

struct ArtifactMailboxSlot {
    pub generation: u64,
    pub bytes: usize,
    pub message: ArtifactActorMsg,
}

struct ArtifactMailboxState {
    pub slots: [Option<ArtifactMailboxSlot>; ARTIFACT_MAILBOX_ITEMS],
    pub head: usize,
    pub len: usize,
    pub bytes: usize,
    pub generation: u64,
    pub closed: bool,
    wake_armed: bool,
    waker: Option<std::task::Waker>,
    pub(super) wake: Option<std::sync::Arc<dyn Fn() + Send + Sync>>,
}

impl ArtifactMailboxState {
    pub(super) fn new() -> Self {
        Self { slots: std::array::from_fn(|_| None), head: 0, len: 0, bytes: 0, generation: 1, closed: false, wake_armed: false, waker: None, wake: None }
    }
}

struct ArtifactMailboxAuthority {
    pub state: std::sync::Mutex<ArtifactMailboxState>,
}

#[derive(Clone)]
pub struct ArtifactMailboxSender {
    pub authority: std::sync::Arc<ArtifactMailboxAuthority>,
    pub(super) generation: u64,
}

pub struct ArtifactMailboxReceiver {
    pub authority: std::sync::Arc<ArtifactMailboxAuthority>,
    pub generation: u64,
}

#[derive(Clone)]
pub struct ArtifactMailboxClose {
    pub authority: std::sync::Arc<ArtifactMailboxAuthority>,
    pub(super) generation: u64,
}

impl ArtifactMailboxSender {
    pub fn send(&self, message: ArtifactActorMsg) -> Result<(), ArtifactMailboxSendError> {
        let Some(bytes) = artifact_actor_message_bytes(&message) else { return Err(ArtifactMailboxSendError::Bytes { message }) };
        let (waker, wake) = {
            let mut state = self.authority.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if self.generation != state.generation {
                return Err(ArtifactMailboxSendError::Stale { message });
            }
            if state.closed {
                return Err(ArtifactMailboxSendError::Closed { message });
            }
            if state.len == ARTIFACT_MAILBOX_ITEMS {
                return Err(ArtifactMailboxSendError::Full { message });
            }
            if bytes > ARTIFACT_MAILBOX_BYTES.saturating_sub(state.bytes) {
                return Err(ArtifactMailboxSendError::Bytes { message });
            }
            let index = (state.head + state.len) % ARTIFACT_MAILBOX_ITEMS;
            state.slots[index] = Some(ArtifactMailboxSlot { generation: self.generation, bytes, message });
            state.len += 1;
            state.bytes += bytes;
            let first_ready = !state.wake_armed;
            state.wake_armed = true;
            (state.waker.take(), first_ready.then(|| state.wake.clone()).flatten())
        };
        if let Some(waker) = waker {
            waker.wake();
        }
        if let Some(wake) = wake {
            wake();
        }
        Ok(())
    }
}

impl ArtifactMailboxReceiver {
    pub fn try_recv(&self) -> Option<ArtifactActorMsg> {
        let mut state = self.authority.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if self.generation != state.generation || state.len == 0 {
            return None;
        }
        let head = state.head;
        let slot = state.slots[head].take().expect("artifact mailbox FIFO owner disappeared");
        state.head = (head + 1) % ARTIFACT_MAILBOX_ITEMS;
        state.len -= 1;
        state.bytes -= slot.bytes;
        if state.len == 0 {
            state.wake_armed = false;
        }
        (slot.generation == self.generation).then_some(slot.message)
    }

    pub async fn recv(&self) -> Option<ArtifactActorMsg> {
        std::future::poll_fn(|context| {
            if let Some(message) = self.try_recv() {
                return std::task::Poll::Ready(Some(message));
            }
            let mut state = self.authority.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if self.generation != state.generation || state.closed {
                return std::task::Poll::Ready(None);
            }
            if state.len != 0 {
                drop(state);
                return std::task::Poll::Ready(self.try_recv());
            }
            state.waker = Some(context.waker().clone());
            std::task::Poll::Pending
        })
        .await
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_wake(&self, wake: std::sync::Arc<dyn Fn() + Send + Sync>) {
        let ready = {
            let mut state = self.authority.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if self.generation != state.generation || state.closed {
                return;
            }
            state.wake = Some(wake.clone());
            state.len != 0
        };
        if ready {
            wake();
        }
    }

    pub fn close(&self) {
        self.close_handle().close();
    }

    pub fn close_one(&self) -> bool {
        self.close_handle().close_one()
    }

    pub fn close_handle(&self) -> ArtifactMailboxClose {
        ArtifactMailboxClose { authority: self.authority.clone(), generation: self.generation }
    }
}

impl ArtifactMailboxClose {
    pub fn close(&self) {
        let waker = {
            let mut state = self.authority.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if self.generation != state.generation {
                return;
            }
            state.closed = true;
            state.generation = state.generation.wrapping_add(1);
            state.wake_armed = false;
            state.wake = None;
            state.waker.take()
        };
        if let Some(waker) = waker {
            waker.wake();
        }
    }

    pub fn close_one(&self) -> bool {
        let owner = {
            let mut state = self.authority.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if state.len == 0 {
                return false;
            }
            let head = state.head;
            let slot = state.slots[head].take();
            state.head = (head + 1) % ARTIFACT_MAILBOX_ITEMS;
            state.len -= 1;
            if let Some(slot) = &slot {
                state.bytes -= slot.bytes;
            }
            if state.len == 0 {
                state.wake_armed = false;
            }
            slot
        };
        drop(owner);
        true
    }

    pub fn has_pending(&self) -> bool {
        self.authority.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len != 0
    }
}

pub fn artifact_mailbox_pair() -> (ArtifactMailboxSender, ArtifactMailboxReceiver) {
    let authority = std::sync::Arc::new(ArtifactMailboxAuthority { state: std::sync::Mutex::new(ArtifactMailboxState::new()) });
    (ArtifactMailboxSender { authority: authority.clone(), generation: 1 }, ArtifactMailboxReceiver { authority, generation: 1 })
}
//#endregion 🔖️Protocol
