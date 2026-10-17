//! 📢️ Broadcast cursor of the bridge session: the recipient state, the broadcast cursor, its steps and its completion. See [`super`].

use super::*;

//#region 🔖️BridgeHandle
pub(crate) enum BridgeRecipientState {
    Claimed { id: ShellConnectionId, outbox: Arc<BridgeOutbox>, grant: BridgeOutboxGrant },
    Published { id: ShellConnectionId },
    RecipientClosed { id: ShellConnectionId, grant: BridgeOutboxGrant },
}

pub(crate) struct BridgeBroadcastCursor {
    pub frame: Option<GatewayToShell>,
    pub expected: usize,
    pub offset: usize,
    pub(crate) encoded: BridgeEncodedFrame,
    pub(crate) shared: Option<Arc<BridgeEncodedFrame>>,
    pub(crate) recipients_state: [Option<BridgeRecipientState>; BRIDGE_BROADCAST_MAX_RECIPIENTS],
    pub(crate) recipients: usize,
    pub(crate) recipient_cursor: usize,
    pub(crate) delivered: usize,
    pub(crate) recipient_closed: usize,
}

pub(crate) enum BridgeBroadcastStep {
    Pending(BridgeBroadcastCursor),
    Complete(BridgeBroadcastCompletion),
}

#[derive(Debug, PartialEq)]
pub enum BridgeBroadcastCompletion {
    Delivered { delivered: usize, recipient_closed: usize },
    Undelivered { frame: GatewayToShell, recipient_closed: usize },
}

impl BridgeBroadcastCursor {
    pub fn step(mut self) -> BridgeBroadcastStep {
        if self.offset != self.expected {
            let bytes = BRIDGE_OUTBOX_PAGE_BYTES.min(self.expected - self.offset);
            let mut page = Box::new([0; BRIDGE_OUTBOX_PAGE_BYTES]);
            let written = self.frame.as_ref().expect("bridge broadcast frame disappeared").copy_encoded_page(self.offset, &mut page[..bytes]);
            assert_eq!(written, bytes, "bridge broadcast page preflight changed");
            self.encoded.pages[self.offset / BRIDGE_OUTBOX_PAGE_BYTES] = Some(page);
            self.offset += bytes;
            return BridgeBroadcastStep::Pending(self);
        }
        if self.shared.is_none() {
            self.shared = Some(Arc::new(std::mem::replace(&mut self.encoded, BridgeEncodedFrame::empty(0, None))));
            return BridgeBroadcastStep::Pending(self);
        }
        if self.recipient_cursor < self.recipients {
            let index = self.recipient_cursor;
            let state = self.recipients_state[index].take().expect("bridge recipient state disappeared");
            match state {
                BridgeRecipientState::Claimed { id, outbox, grant } => match outbox.publish(grant, Arc::clone(self.shared.as_ref().expect("shared bridge frame disappeared"))) {
                    Ok(()) => {
                        self.recipients_state[index] = Some(BridgeRecipientState::Published { id });
                        self.delivered += 1;
                    }
                    Err(rejected) => {
                        self.recipients_state[index] = Some(BridgeRecipientState::RecipientClosed { id, grant: rejected.grant });
                        self.recipient_closed += 1;
                        drop(rejected.encoded);
                    }
                },
                BridgeRecipientState::Published { id } => self.recipients_state[index] = Some(BridgeRecipientState::Published { id }),
                BridgeRecipientState::RecipientClosed { id, grant } => self.recipients_state[index] = Some(BridgeRecipientState::RecipientClosed { id, grant }),
            }
            self.recipient_cursor += 1;
            return BridgeBroadcastStep::Pending(self);
        }
        let completion = if self.delivered == 0 {
            BridgeBroadcastCompletion::Undelivered { frame: self.frame.take().expect("undelivered bridge frame disappeared"), recipient_closed: self.recipient_closed }
        } else {
            self.frame.take();
            BridgeBroadcastCompletion::Delivered { delivered: self.delivered, recipient_closed: self.recipient_closed }
        };
        BridgeBroadcastStep::Complete(completion)
    }

    pub(super) fn close_one_claim(mut self) -> BridgeBroadcastStep {
        while self.recipient_cursor < self.recipients {
            let index = self.recipient_cursor;
            self.recipient_cursor += 1;
            let state = self.recipients_state[index].take().expect("bridge recipient state disappeared");
            match state {
                BridgeRecipientState::Claimed { id, outbox, grant } => {
                    outbox.cancel(grant);
                    self.recipients_state[index] = Some(BridgeRecipientState::RecipientClosed { id, grant });
                    self.recipient_closed += 1;
                    return BridgeBroadcastStep::Pending(self);
                }
                BridgeRecipientState::Published { id } => self.recipients_state[index] = Some(BridgeRecipientState::Published { id }),
                BridgeRecipientState::RecipientClosed { id, grant } => self.recipients_state[index] = Some(BridgeRecipientState::RecipientClosed { id, grant }),
            }
        }
        let completion = if self.delivered == 0 {
            BridgeBroadcastCompletion::Undelivered { frame: self.frame.take().expect("terminal bridge frame disappeared"), recipient_closed: self.recipient_closed }
        } else {
            self.frame.take();
            BridgeBroadcastCompletion::Delivered { delivered: self.delivered, recipient_closed: self.recipient_closed }
        };
        BridgeBroadcastStep::Complete(completion)
    }
}
//#endregion 🔖️BridgeHandle
