//! 📮️ Outbox of the bridge session: the outbox item, grant and rejected publish records, the outbox state, the outbox and its receiver. See [`super`].

use super::*;

//#region 🔖️BridgeHandle
pub(crate) struct BridgeOutboxItem {
    pub lease: BridgeEncodedLease,
    pub bytes: usize,
}

#[derive(Clone, Copy)]
pub(crate) struct BridgeOutboxGrant {
    pub generation: u64,
    pub bytes: usize,
}

pub(crate) struct BridgeRejectedPublish {
    pub grant: BridgeOutboxGrant,
    pub(crate) encoded: Arc<BridgeEncodedFrame>,
}

pub(crate) struct BridgeOutboxState {
    pub slots: [Option<BridgeOutboxItem>; BRIDGE_OUTBOX_MAX_ITEMS],
    pub head: usize,
    pub len: usize,
    pub bytes: usize,
    pub generation: u64,
    pub(crate) reserved_items: usize,
    pub(crate) reserved_bytes: usize,
    pub closed: bool,
    #[cfg(test)]
    pub(crate) encode_count: usize,
    #[cfg(test)]
    pub(crate) waker: Option<std::task::Waker>,
}

impl BridgeOutboxState {
    pub(crate) fn new(generation: u64) -> Self {
        Self {
            slots: std::array::from_fn(|_| None),
            head: 0,
            len: 0,
            bytes: 0,
            generation,
            reserved_items: 0,
            reserved_bytes: 0,
            closed: false,
            #[cfg(test)]
            encode_count: 0,
            #[cfg(test)]
            waker: None,
        }
    }

    pub(crate) fn claim(&mut self, generation: u64, bytes: usize) -> Result<BridgeOutboxGrant, ()> {
        if self.closed || generation != self.generation || self.len.saturating_add(self.reserved_items) == BRIDGE_OUTBOX_MAX_ITEMS || bytes > BRIDGE_OUTBOX_MAX_BYTES.saturating_sub(self.bytes).saturating_sub(self.reserved_bytes) {
            return Err(());
        }
        self.reserved_items += 1;
        self.reserved_bytes += bytes;
        Ok(BridgeOutboxGrant { generation, bytes })
    }

    pub fn cancel(&mut self, grant: BridgeOutboxGrant) {
        if grant.generation == self.generation {
            self.reserved_items = self.reserved_items.saturating_sub(1);
            self.reserved_bytes = self.reserved_bytes.saturating_sub(grant.bytes);
        }
    }

    pub(crate) fn publish(&mut self, grant: BridgeOutboxGrant, encoded: Arc<BridgeEncodedFrame>) -> Result<(), BridgeRejectedPublish> {
        if self.closed || grant.generation != self.generation || self.reserved_items == 0 || grant.bytes > self.reserved_bytes {
            return Err(BridgeRejectedPublish { grant, encoded });
        }
        self.reserved_items -= 1;
        self.reserved_bytes -= grant.bytes;
        let index = (self.head + self.len) % BRIDGE_OUTBOX_MAX_ITEMS;
        self.len += 1;
        self.bytes += grant.bytes;
        #[cfg(test)]
        {
            self.encode_count += 1;
        }
        self.slots[index] = Some(BridgeOutboxItem { lease: BridgeEncodedLease { generation: grant.generation, encoded }, bytes: grant.bytes });
        #[cfg(test)]
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
        Ok(())
    }

    pub fn pop_front(&mut self) -> Option<BridgeOutboxItem> {
        if self.len == 0 {
            return None;
        }
        let item = self.slots[self.head].take().expect("bridge outbox FIFO slot disappeared");
        self.head = (self.head + 1) % BRIDGE_OUTBOX_MAX_ITEMS;
        self.len -= 1;
        self.bytes = self.bytes.saturating_sub(item.bytes);
        Some(item)
    }
}

pub(crate) struct BridgeOutbox {
    pub state: Mutex<BridgeOutboxState>,
}

impl BridgeOutbox {
    pub(crate) fn new(generation: u64) -> Self {
        Self { state: Mutex::new(BridgeOutboxState::new(generation)) }
    }

    pub(crate) fn try_send(&self, frame: GatewayToShell) -> Result<(), GatewayToShell> {
        let Some(bytes) = frame.encoded_len() else { return Err(frame) };
        let grant = match self.claim(bytes) {
            Ok(grant) => grant,
            Err(()) => return Err(frame),
        };
        let encoded = Arc::new(BridgeEncodedFrame::encode(&frame, bytes));
        if self.publish(grant, encoded).is_err() {
            return Err(frame);
        }
        Ok(())
    }

    pub(crate) fn claim(&self, bytes: usize) -> Result<BridgeOutboxGrant, ()> {
        let mut state = self.state.lock().expect("bridge outbox lock poisoned");
        let generation = state.generation;
        state.claim(generation, bytes)
    }

    pub(crate) fn publish(&self, grant: BridgeOutboxGrant, encoded: Arc<BridgeEncodedFrame>) -> Result<(), BridgeRejectedPublish> {
        self.state.lock().expect("bridge outbox lock poisoned").publish(grant, encoded)
    }

    pub fn cancel(&self, grant: BridgeOutboxGrant) {
        self.state.lock().expect("bridge outbox lock poisoned").cancel(grant);
    }

    pub fn try_recv(&self) -> Option<BridgeOutboxItem> {
        let mut state = self.state.lock().expect("bridge outbox lock poisoned");
        let item = state.pop_front()?;
        if item.lease.generation != state.generation {
            return None;
        }
        Some(item)
    }

    pub fn close(&self) {
        let mut state = self.state.lock().expect("bridge outbox lock poisoned");
        state.closed = true;
        state.generation = state.generation.wrapping_add(1);
        state.reserved_items = 0;
        state.reserved_bytes = 0;
        #[cfg(test)]
        if let Some(waker) = state.waker.take() {
            waker.wake();
        }
    }
}

pub struct BridgeOutboxReceiver(pub Arc<BridgeOutbox>);

impl BridgeOutboxReceiver {
    #[cfg(test)]
    pub async fn recv(&mut self) -> Option<GatewayToShell> {
        std::future::poll_fn(|context| {
            let mut state = self.0.state.lock().expect("bridge outbox lock poisoned");
            if let Some(item) = state.pop_front() {
                if item.lease.generation == state.generation {
                    return std::task::Poll::Ready(Some(item.lease.encoded.decode()));
                }
                return std::task::Poll::Ready(None);
            }
            if state.closed {
                return std::task::Poll::Ready(None);
            }
            state.waker = Some(context.waker().clone());
            std::task::Poll::Pending
        })
        .await
    }

    #[cfg(test)]
    pub fn try_recv(&mut self) -> Option<GatewayToShell> {
        self.0.try_recv().map(|item| item.lease.encoded.decode())
    }

    pub fn try_recv_encoded(&mut self) -> Option<BridgeEncodedLease> {
        self.0.try_recv().map(|item| item.lease)
    }
}
//#endregion 🔖️BridgeHandle
