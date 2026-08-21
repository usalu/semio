//! @emoji 📮️ The non-blocking bounded `CommandGateway` and its `CommandSink` seam to the actor world.
//!
//! 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md. Every `fn`
//! below is plain sync by owner ruling U1.
//!
//! [`CommandGateway::try_submit`] is deliberately non-blocking and non-`async`: a UI frame must never
//! wait on an actor, since the whole point of this seam is that an unavailable or slow actor cannot
//! stall paint. On a full mailbox the caller gets [`GatewayError::Full`] back synchronously — the
//! initiating control goes to `Activity::Waiting` and the caller (`runtime-transact`'s `transact()`)
//! retries next transaction, rather than the frame blocking to wait it out.
//!
//! [`CommandSink`] (U3) is a generic parameter, never `Box<dyn CommandSink>` — the host supplies a
//! concrete sink over the kernel's existing mailbox, monomorphised at the call site.
//!
//! [`Command`]/[`CommandId`] are this crate's OWN envelope, not the os-kernel's. Pulling in the
//! kernel's `DslValue` or any actor type here would end this crate's `wasm32-wasip2` compatibility, so
//! a command's payload is the contract's neutral [`ui_contract::UiValue`] instead — conversion to
//! whatever the receiving actor actually wants happens on the far side of [`CommandSink::try_send`].
//!
//! This is the crate's DURABLE, never-drop policy: a command that clears [`CommandGateway::try_submit`]
//! is tracked until an explicit [`CommandGateway::acknowledge`]/[`CommandGateway::reject`]/
//! [`CommandGateway::mark_conflicted`] resolves it — it is never silently discarded. Contrast
//! `🦀️inbox.rs`'s [`crate::inbox::ProjectionInbox`], whose policy is the opposite and equally correct
//! for its own shape: same-key projection deltas COALESCE to the newest rather than queueing, because a
//! projection revision that has already been superseded is not durable data worth keeping around.

use std::collections::{HashMap, VecDeque};

//#region 🔖️Command

/// 🆔️ Stable identity for one submitted [`Command`], minted by the caller before [`CommandGateway::
/// try_submit`] — carried back on every [`GatewayError::Full`] and echoed by [`CommandTicket`] so a
/// later acknowledgement/rejection can name exactly which submission it resolves.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandId(pub u64);

/// 🔗️ Groups a [`Command`] with whatever caused it — e.g. the [`ui_contract::UiIntent::seq`] that
/// triggered it — so a receiver can relate an out-of-band actor response back to the originating input
/// without the gateway itself needing to understand intents.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CorrelationId(pub u64);

/// 📦️ The runtime's own outbound envelope — an id, a correlation id, and an opaque, crate-neutral
/// payload. Deliberately NOT the os-kernel's `DslValue` or any actor type (see module docs); a host's
/// [`CommandSink`] converts `payload` to whatever its own mailbox actually expects.
#[derive(Clone, Debug, PartialEq)]
pub struct Command {
    pub id: CommandId,
    pub correlation: CorrelationId,
    pub payload: ui_contract::UiValue,
}
//#endregion 🔖️Command

//#region 🔖️Sink

/// 🚧️ [`CommandSink::try_send`] could not accept a [`Command`] right now — its backing mailbox is full.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SinkFull;

/// 🕳️ The seam to the actor world. U3: a generic parameter on [`CommandGateway`], never `Box<dyn
/// CommandSink>` — the host supplies ONE concrete sink over the kernel's existing mailbox handle, so
/// this call monomorphises rather than going through a vtable.
pub trait CommandSink {
    /// 📤️ Hands `command` to the backing mailbox without blocking. `Err(SinkFull)` on a full mailbox —
    /// never panics, never waits.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn try_send(&self, command: Command) -> Result<(), SinkFull>;
}
//#endregion 🔖️Sink

//#region 🔖️Ticket

/// 🧾️ A receipt for one accepted [`Command`] — lets the issuing control later query
/// [`CommandGateway::status`] to layer an optimistic state over its own projection read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandTicket {
    pub command_id: CommandId,
    pub correlation: CorrelationId,
}

/// 📊️ Where one [`CommandTicket`] stands — a presenter reads this to show an optimistic state (e.g. a
/// value greyed out while `Pending`, snapped back on `Rejected`/`Conflicted`) layered over whatever the
/// underlying CQRS projection has actually confirmed so far.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OptimisticStatus {
    Pending,
    Acknowledged,
    Rejected,
    Conflicted,
}
//#endregion 🔖️Ticket

//#region 🔖️Gateway

/// 🚫️ Why [`CommandGateway::try_submit`] refused a [`Command`] — today, only ever a full mailbox:
/// either this gateway's own outstanding-ticket bound was already reached, or the backing
/// [`CommandSink`] itself reported [`SinkFull`]. Both surface identically because both mean the same
/// thing to the caller: retry next transaction, do not block this frame on it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GatewayError {
    Full { command_id: CommandId },
}

/// 📮️ A non-blocking, bounded gate in front of a [`CommandSink`]. `capacity` bounds how many
/// [`Command`]s this gateway will track as outstanding (`Pending`) at once — independent of whatever
/// bound the backing mailbox itself enforces — so a UI that keeps submitting into a slow actor fails
/// fast locally instead of growing its own tracking table without limit.
pub struct CommandGateway<S: CommandSink> {
    queue: VecDeque<Command>,
    capacity: usize,
    sink: S,
    resolved: HashMap<CommandId, OptimisticStatus>,
}

impl<S: CommandSink> CommandGateway<S> {
    /// 🏭️ A gateway over `sink`, tracking at most `capacity` outstanding (unresolved) commands at once.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new(capacity: usize, sink: S) -> Self {
        Self { queue: VecDeque::new(), capacity, sink, resolved: HashMap::new() }
    }

    /// 📤️ Submits `command` without blocking. Checks THIS gateway's own outstanding-ticket bound
    /// first — fast-failing before ever touching `sink` if already at `capacity` — then forwards to
    /// [`CommandSink::try_send`]; either way a full mailbox comes back as [`GatewayError::Full`]
    /// synchronously, on this same call, never as a dropped command nobody hears about.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn try_submit(&mut self, command: Command) -> Result<CommandTicket, GatewayError> {
        if self.queue.len() >= self.capacity {
            return Err(GatewayError::Full { command_id: command.id });
        }
        let ticket = CommandTicket { command_id: command.id, correlation: command.correlation };
        self.sink.try_send(command.clone()).map_err(|SinkFull| GatewayError::Full { command_id: command.id })?;
        self.queue.push_back(command);
        Ok(ticket)
    }

    /// ✅️ Resolves `command_id` as [`OptimisticStatus::Acknowledged`]. `false` if no outstanding ticket
    /// carries that id (already resolved, or never submitted).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn acknowledge(&mut self, command_id: CommandId) -> bool {
        self.resolve(command_id, OptimisticStatus::Acknowledged)
    }

    /// ❌️ Resolves `command_id` as [`OptimisticStatus::Rejected`]. `false` if no outstanding ticket
    /// carries that id.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn reject(&mut self, command_id: CommandId) -> bool {
        self.resolve(command_id, OptimisticStatus::Rejected)
    }

    /// ⚡️ Resolves `command_id` as [`OptimisticStatus::Conflicted`] — the command reached the actor but
    /// lost to a concurrent write. `false` if no outstanding ticket carries that id.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn mark_conflicted(&mut self, command_id: CommandId) -> bool {
        self.resolve(command_id, OptimisticStatus::Conflicted)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn resolve(&mut self, command_id: CommandId, status: OptimisticStatus) -> bool {
        let Some(index) = self.queue.iter().position(|command| command.id == command_id) else { return false };
        self.queue.remove(index);
        self.resolved.insert(command_id, status);
        true
    }

    /// 📊️ The [`OptimisticStatus`] a presenter should layer over its projection read for `command_id` —
    /// `Pending` while still outstanding, the resolved status once acknowledged/rejected/conflicted, or
    /// `None` if `command_id` was never submitted through this gateway.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn status(&self, command_id: CommandId) -> Option<OptimisticStatus> {
        if self.queue.iter().any(|command| command.id == command_id) {
            return Some(OptimisticStatus::Pending);
        }
        self.resolved.get(&command_id).copied()
    }

    /// 🔢️ Outstanding (`Pending`) commands right now.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// 🈳️ `true` when [`Self::len`] is zero.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}
//#endregion 🔖️Gateway

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    struct AlwaysAccepts;
    impl CommandSink for AlwaysAccepts {
        fn try_send(&self, _command: Command) -> Result<(), SinkFull> {
            Ok(())
        }
    }

    struct FailsAfter {
        calls: Cell<u32>,
        accepts: u32,
    }
    impl CommandSink for FailsAfter {
        fn try_send(&self, _command: Command) -> Result<(), SinkFull> {
            let seen = self.calls.get();
            self.calls.set(seen + 1);
            if seen < self.accepts {
                Ok(())
            } else {
                Err(SinkFull)
            }
        }
    }

    fn command(id: u64) -> Command {
        Command { id: CommandId(id), correlation: CorrelationId(id), payload: ui_contract::UiValue::Number(id as f64) }
    }

    #[test]
    fn full_local_capacity_returns_full_synchronously_without_dropping_the_command() {
        let mut gateway = CommandGateway::new(1, AlwaysAccepts);
        let first = gateway.try_submit(command(1)).expect("first fits within capacity");
        assert_eq!(first.command_id, CommandId(1));

        let overflow = gateway.try_submit(command(2));
        assert_eq!(overflow, Err(GatewayError::Full { command_id: CommandId(2) }));
        assert_eq!(gateway.len(), 1, "the rejected command must not have been silently tracked as sent");
        assert_eq!(gateway.status(CommandId(2)), None, "a refused submission has no ticket to look up");
    }

    #[test]
    fn full_backing_sink_returns_full_synchronously_without_dropping_the_command() {
        let sink = FailsAfter { calls: Cell::new(0), accepts: 1 };
        let mut gateway = CommandGateway::new(10, sink);
        gateway.try_submit(command(1)).expect("sink accepts the first command");

        let refused = gateway.try_submit(command(2));
        assert_eq!(refused, Err(GatewayError::Full { command_id: CommandId(2) }));
        assert_eq!(gateway.len(), 1, "a sink-refused command is never enqueued as if it were sent");
    }

    #[test]
    fn ticket_round_trips_to_acknowledged_and_to_rejected() {
        let mut gateway = CommandGateway::new(10, AlwaysAccepts);
        let acked = gateway.try_submit(command(1)).expect("submits");
        let rejected = gateway.try_submit(command(2)).expect("submits");

        assert_eq!(gateway.status(acked.command_id), Some(OptimisticStatus::Pending));
        assert!(gateway.acknowledge(acked.command_id));
        assert_eq!(gateway.status(acked.command_id), Some(OptimisticStatus::Acknowledged));

        assert_eq!(gateway.status(rejected.command_id), Some(OptimisticStatus::Pending));
        assert!(gateway.reject(rejected.command_id));
        assert_eq!(gateway.status(rejected.command_id), Some(OptimisticStatus::Rejected));

        assert_eq!(gateway.len(), 0, "resolved tickets free their capacity slot");
        assert!(!gateway.acknowledge(CommandId(404)), "resolving an unknown id is a no-op, not a panic");
    }

    #[test]
    fn resolving_a_ticket_frees_capacity_for_a_new_submission() {
        let mut gateway = CommandGateway::new(1, AlwaysAccepts);
        let first = gateway.try_submit(command(1)).expect("fits");
        assert_eq!(gateway.try_submit(command(2)), Err(GatewayError::Full { command_id: CommandId(2) }));

        assert!(gateway.acknowledge(first.command_id));
        gateway.try_submit(command(2)).expect("capacity freed by the acknowledgement");
    }
}
//#endregion 🧪️Tests
