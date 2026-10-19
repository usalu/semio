//! 🤝️ Handle of the bridge session: the shared inner state, the bridge handle with its connection, outbox and agent inbox operations and the frame send helpers. See [`super`].

use super::*;

//#region 🔖️BridgeHandle
pub(crate) struct BridgeInner {
    pub next_id: AtomicU64,
    pub connections: Mutex<HashMap<ShellConnectionId, ConnectionEntry>>,
    pub(crate) asynchronous: Arc<BridgeAsyncAuthority>,
}

/// 🖇️ The seam a later packet reaches live `/bridge` connections through WITHOUT this facet
/// depending on theirs — P6's policy engine routes a parked approval to a connected shell via
/// [`send_to`](Self::send_to)/[`broadcast`](Self::broadcast); a future `ui.*` tool pushes a
/// `ShellCommand` the same way; `semio://ui/shell`/`context_resolve` read
/// [`last_shell_state`](Self::last_shell_state). None of that wiring happens in THIS file
/// (`📓️sol-P1c-packet.md` §3: "do not wire it into P6's files … just publish the API") — obtain a
/// `BridgeHandle` from the live [`crate::HttpTransportRun::bridge`] owner; the Axum router is a
/// test-only differential oracle.
#[derive(Clone)]
pub struct BridgeHandle {
    pub inner: Arc<BridgeInner>,
}

impl Default for BridgeHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl BridgeHandle {
    pub fn new() -> Self {
        let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
        Self::with_pool(semio_framework_async::process_worker_pool(WorkerPoolConfig::new(ProcessKind::InteractiveNative, cores)))
    }

    pub fn with_pool(pool: WorkerPool) -> Self {
        Self { inner: Arc::new(BridgeInner { next_id: AtomicU64::new(0), connections: Mutex::new(HashMap::new()), asynchronous: BridgeAsyncAuthority::new(pool) }) }
    }

    pub fn register(&self) -> (ShellConnectionId, BridgeOutboxReceiver) {
        let id = ShellConnectionId(self.inner.next_id.fetch_add(1, Ordering::Relaxed));
        let outbox = Arc::new(BridgeOutbox::new(id.0));
        self.inner
            .connections
            .lock()
            .expect("bridge connections lock poisoned")
            .insert(id, ConnectionEntry { generation: id.0, outbox: Arc::clone(&outbox), flags: BridgeFlags::NONE, last_shell_state: None, last_instances: None, last_command_result: None, last_app_frames: None, last_approval: None, agent_inbox: std::collections::VecDeque::new() });
        (id, BridgeOutboxReceiver(outbox))
    }

    /// 🚩️ Records the flags one shell declared in its `Hello`. `Hello` is handled inline by the
    /// read loop (it is the frame that mints the connection in the first place, so it can never
    /// reach [`BridgeHandle::record`]) — this is the one seam that carries its payload into the
    /// connection state instead of dropping it.
    pub fn record_hello(&self, id: ShellConnectionId, flags: BridgeFlags) {
        if let Some(entry) = self.inner.connections.lock().expect("bridge connections lock poisoned").get_mut(&id) {
            entry.flags = flags;
        }
    }

    pub fn unregister(&self, id: ShellConnectionId) {
        if let Some(entry) = self.inner.connections.lock().expect("bridge connections lock poisoned").remove(&id) {
            entry.outbox.close();
        }
    }

    /// 📝️ Records the effect of one received [`ShellToGateway`] frame against its connection —
    /// `Hello`/`Ping`/`Bye` never reach here (the read loop handles all three inline).
    ///
    /// 🛑️ A cancel is connection-independent and takes no connection state: it flips the SAME
    /// cooperative flag `job_cancel` flips, on the job `AgentConversation::begin_tool_call` opened
    /// under this invocation id. Handled before the connections lock so the two locks never nest.
    /// A `NotFound`/`PreconditionFailed` means the call already finished between the human's click
    /// and this frame — the honest outcome of a cancel arriving late, not an error to report.
    ///
    /// 💬️ Outside the connections lock: the subscription broker takes its own locks, and a human
    /// turn that only becomes visible on the agent's next poll is a channel with a one-poll
    /// latency floor. Subscribed clients are told now; unsubscribed ones still poll as before.
    pub fn record(&self, id: ShellConnectionId, frame: ShellToGateway) {
        if let ShellToGateway::AgentCancel { invocation_id } = &frame {
            let _ = job_registry().request_cancel(invocation_id);
            return;
        }
        let mut inbox_grew = false;
        let mut connections = self.inner.connections.lock().expect("bridge connections lock poisoned");
        let Some(entry) = connections.get_mut(&id) else { return };
        match frame {
            ShellToGateway::ShellState { .. } | ShellToGateway::ShellStatePatch { .. } => entry.last_shell_state = Some(frame),
            ShellToGateway::Instances { entries } => entry.last_instances = Some(entries),
            ShellToGateway::ShellCommandResult { in_reply_to, ok, fault } => entry.last_command_result = Some((in_reply_to, ok, fault)),
            ShellToGateway::AppFrames { in_reply_to, instance_id, frames } => entry.last_app_frames = Some((in_reply_to, instance_id, frames)),
            ShellToGateway::Approval { approval_id, decision, note } => entry.last_approval = Some((approval_id, decision, note)),
            ShellToGateway::AgentMessage { message_id, text } => {
                if entry.agent_inbox.len() == BRIDGE_AGENT_INBOX_MAX_ITEMS {
                    entry.agent_inbox.pop_front();
                }
                entry.agent_inbox.push_back(AgentInboxMessage { message_id, text, received_at_ms: bridge_wall_now_ms() });
                inbox_grew = true;
            }
            ShellToGateway::Hello { .. } | ShellToGateway::Ping | ShellToGateway::Bye | ShellToGateway::AgentCancel { .. } => {}
        }
        drop(connections);
        if inbox_grew {
            agent_messages_changed();
        }
    }

    /// 📤️ Pushes one frame to exactly one live connection — `false` if that connection no longer
    /// exists or its outbox is closed (the connection's own task will unregister it shortly after).
    pub fn send_to(&self, id: ShellConnectionId, frame: GatewayToShell) -> bool {
        self.try_send_to(id, frame).is_ok()
    }

    /// 📦️ Fixed-credit bridge admission with exact rejected-frame handback.
    pub fn try_send_to(&self, id: ShellConnectionId, frame: GatewayToShell) -> Result<(), GatewayToShell> {
        let connections = self.inner.connections.lock().expect("bridge connections lock poisoned");
        match connections.get(&id) {
            Some(entry) => try_send_bridge_frame(entry, frame),
            None => Err(frame),
        }
    }

    /// 📢️ Atomically admits one shared encoded frame to every live connection.
    pub fn broadcast(&self, frame: GatewayToShell) -> Result<usize, GatewayToShell> {
        let Some(bytes) = frame.encoded_len() else { return Err(frame) };
        let connections = self.inner.connections.lock().expect("bridge connections lock poisoned");
        let recipients = connections.len();
        if recipients > BRIDGE_BROADCAST_MAX_RECIPIENTS || bytes.checked_mul(recipients).map_or(true, |total| total > BRIDGE_BROADCAST_MAX_BYTES) {
            return Err(frame);
        }
        if recipients == 0 {
            return Ok(0);
        }
        if self.inner.asynchronous.reserve_broadcast().is_err() {
            return Err(frame);
        }
        let retirement = match self.inner.asynchronous.reserve_retirement() {
            Ok(grant) => grant,
            Err(()) => {
                self.inner.asynchronous.cancel_broadcast_reservation();
                return Err(frame);
            }
        };
        let mut recipient_ids: [Option<ShellConnectionId>; BRIDGE_BROADCAST_MAX_RECIPIENTS] = [None; BRIDGE_BROADCAST_MAX_RECIPIENTS];
        for (index, id) in connections.keys().copied().enumerate() {
            recipient_ids[index] = Some(id);
        }
        recipient_ids[..recipients].sort_unstable();
        let mut recipients_state: [Option<BridgeRecipientState>; BRIDGE_BROADCAST_MAX_RECIPIENTS] = std::array::from_fn(|_| None);
        let mut admitted = 0;
        for id in recipient_ids[..recipients].iter().flatten().copied() {
            let entry = connections.get(&id).expect("stable bridge recipient disappeared under admission lock");
            let grant = match entry.outbox.claim(bytes) {
                Ok(grant) if grant.generation == entry.generation => grant,
                Ok(grant) => {
                    entry.outbox.cancel(grant);
                    for state in recipients_state[..admitted].iter_mut().filter_map(Option::take) {
                        if let BridgeRecipientState::Claimed { outbox, grant, .. } = state {
                            outbox.cancel(grant);
                        }
                    }
                    self.inner.asynchronous.cancel_retirement(retirement);
                    self.inner.asynchronous.cancel_broadcast_reservation();
                    return Err(frame);
                }
                Err(()) => {
                    for state in recipients_state[..admitted].iter_mut().filter_map(Option::take) {
                        if let BridgeRecipientState::Claimed { outbox, grant, .. } = state {
                            outbox.cancel(grant);
                        }
                    }
                    self.inner.asynchronous.cancel_retirement(retirement);
                    self.inner.asynchronous.cancel_broadcast_reservation();
                    return Err(frame);
                }
            };
            recipients_state[admitted] = Some(BridgeRecipientState::Claimed { id, outbox: Arc::clone(&entry.outbox), grant });
            admitted += 1;
        }
        let cursor = BridgeBroadcastCursor {
            frame: Some(frame),
            expected: bytes,
            offset: 0,
            encoded: BridgeEncodedFrame::empty(bytes, Some((Arc::clone(&self.inner.asynchronous), retirement))),
            shared: None,
            recipients_state,
            recipients,
            recipient_cursor: 0,
            delivered: 0,
            recipient_closed: 0,
        };
        drop(connections);
        self.inner.asynchronous.enqueue_broadcast(cursor);
        Ok(recipients)
    }

    pub fn take_broadcast_completion(&self) -> Option<BridgeBroadcastCompletion> {
        self.inner.asynchronous.take_broadcast_completion()
    }

    pub fn close_one_terminal_broadcast_claim(&self) -> bool {
        self.inner.asynchronous.close_one_terminal_broadcast_claim()
    }

    pub fn cancel_broadcasts(&self) {
        self.inner.asynchronous.cancel();
    }

    pub(crate) fn take_terminal_broadcast_job(&self) -> Option<(WorkerSubmitErrorKind, Job)> {
        self.inner.asynchronous.take_terminal_job()
    }

    pub fn close_one_terminal_retired_page(&self) -> bool {
        self.inner.asynchronous.close_one_terminal_retired_page()
    }

    pub fn connections(&self) -> Vec<ShellConnectionId> {
        self.inner.connections.lock().expect("bridge connections lock poisoned").keys().copied().collect()
    }

    /// 🧭️ The last `ShellState`/`ShellStatePatch` frame received on this connection. Applying a patch
    /// onto a base state is `💻️os/🔨️modules/🖥️shell`'s reducer's job, not this facet's — a caller that
    /// needs the merged/canonical state feeds whatever this returns through that reducer.
    pub fn last_shell_state(&self, id: ShellConnectionId) -> Option<ShellToGateway> {
        self.inner.connections.lock().expect("bridge connections lock poisoned").get(&id).and_then(|entry| entry.last_shell_state.clone())
    }

    pub fn last_instances(&self, id: ShellConnectionId) -> Option<Vec<BridgeInstanceRef>> {
        self.inner.connections.lock().expect("bridge connections lock poisoned").get(&id).and_then(|entry| entry.last_instances.clone())
    }

    /// 🚩️ The flags the shell on `id` declared in its `Hello` — `BridgeFlags::NONE` for a
    /// connection that no longer exists, which is the same answer as "claims nothing".
    pub fn shell_flags(&self, id: ShellConnectionId) -> BridgeFlags {
        self.inner.connections.lock().expect("bridge connections lock poisoned").get(&id).map_or(BridgeFlags::NONE, |entry| entry.flags)
    }

    /// 🗿️ The shell's most recent `AppFrames` reply on `id`. The caller MUST check `in_reply_to`
    /// against the `seq` it sent — a stale reply from an earlier exchange is otherwise
    /// indistinguishable from its own.
    pub fn last_app_frames(&self, id: ShellConnectionId) -> Option<(u64, String, Vec<Vec<u8>>)> {
        self.inner.connections.lock().expect("bridge connections lock poisoned").get(&id).and_then(|entry| entry.last_app_frames.clone())
    }

    pub fn last_command_result(&self, id: ShellConnectionId) -> Option<(u64, bool, Option<String>)> {
        self.inner.connections.lock().expect("bridge connections lock poisoned").get(&id).and_then(|entry| entry.last_command_result.clone())
    }

    pub fn last_approval(&self, id: ShellConnectionId) -> Option<(String, ApprovalDecision, Option<String>)> {
        self.inner.connections.lock().expect("bridge connections lock poisoned").get(&id).and_then(|entry| entry.last_approval.clone())
    }

    /// 📥️ Drains every unread human turn this connection typed at the agent, oldest first. Read-once
    /// by design: the agent polls `semio://ui/agent-messages`, and a turn it has been handed is not
    /// handed to it again.
    pub fn take_agent_messages(&self, id: ShellConnectionId) -> Vec<AgentInboxMessage> {
        let mut connections = self.inner.connections.lock().expect("bridge connections lock poisoned");
        match connections.get_mut(&id) {
            Some(entry) => entry.agent_inbox.drain(..).collect(),
            None => Vec::new(),
        }
    }

    /// 👀️ How many unread human turns this connection is holding — a non-destructive peek, for a
    /// resource listing or a test that must not consume the queue.
    pub fn pending_agent_message_count(&self, id: ShellConnectionId) -> usize {
        self.inner.connections.lock().expect("bridge connections lock poisoned").get(&id).map(|entry| entry.agent_inbox.len()).unwrap_or(0)
    }
}

/// 🕐️ Wall-clock milliseconds — this facet's own single clock read site, same convention every
/// other facet in this crate restates rather than sharing a private helper across module roots.
fn bridge_wall_now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u64
}

fn try_send_bridge_frame(entry: &ConnectionEntry, frame: GatewayToShell) -> Result<(), GatewayToShell> {
    entry.outbox.try_send(frame)
}
//#endregion 🔖️BridgeHandle
