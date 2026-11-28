//! 🔁️ The headless channel runs the current Source-bound guest event protocol.

use super::*;
use semio_framework::kernel::{Event, KernelPollOutput, TurnResult};
use semio_framework_pack_json::JsonReadLimits;
use semio_framework_value::ValueRefusalKind;
use semio_plugin_host::GuestRelayNativeOrigin;
use semio_plugin_host_poll::{EventJsonSource, GuestPollNoReceiverState, GuestPollSession, GuestPollSessionStatus, OriginalNativeDecodePolicy};

#[path = "⚠️actor-failure.rs"]
mod actor_failure;

const POLL_STEPS_PER_YIELD: usize = 32;

#[derive(Clone, Copy)]
pub(super) enum PollPurpose { Lifecycle, Command }

pub(super) type NativePoll = GuestPollSession<KernelPollOutput, GuestPollNoReceiverState>;

impl PluginArtifactChannel {
    pub(super) fn issue_native_poll(&self, actor: semio_framework_actor::ActorId, origin: GuestRelayNativeOrigin, scope: &ActivationScope) -> Result<Box<NativePoll>, Fault> {
        let host = self.host_driver_policy;
        let cancel = scope.cancel().clone();
        let driver = self.operation_caps().issue(implicit_editor_request(host.root.maximum_lifetime_microseconds), semio_framework_job::default_now_us, Box::new(move || cancel.is_cancelled_now())).map_err(|error| Self::not_wired("guest poll Source", error))?;
        let maximum_owned_bytes = host.operation.maximum_capacity_bytes;
        let limits = JsonReadLimits {
            maximum_bytes: u64::from(host.root.max_patch_bytes),
            maximum_allocation_bytes: maximum_owned_bytes,
            maximum_depth: host.operation.maximum_depth,
            maximum_items: u64::try_from(host.operation.maximum_items).unwrap_or(u64::MAX),
        };
        Ok(Box::new(NativePoll::new(driver, actor, origin, OriginalNativeDecodePolicy { maximum_owned_bytes }, limits, GuestPollNoReceiverState::new())))
    }

    pub(super) fn with_native_turn<T>(&mut self, instance: u32, events: &[Event], receive: impl FnOnce(&mut Self, &TurnResult) -> Result<T, Fault>) -> Result<T, Fault> {
        self.with_native_turn_scoped(instance, events, &ActivationScope::detached(), receive)
    }

    pub(super) fn with_native_turn_scoped<T>(&mut self, instance: u32, events: &[Event], scope: &ActivationScope, receive: impl FnOnce(&mut Self, &TurnResult) -> Result<T, Fault>) -> Result<T, Fault> {
        if self.closing_instances.contains(&instance) { return Err(Self::not_wired("guest poll", "instance still holds a closing Source")); }
        let mut poll = self.instances.remove(&instance).ok_or_else(|| Self::not_wired("guest poll", format!("no open instance {instance}")))?;
        let result = poll_turn(&mut poll, PollPurpose::Command, events, scope, std::time::Instant::now() + COMMAND_RESUME_WALL_BUDGET, |output| receive(self, output));
        self.instances.insert(instance, poll);
        result
    }
}

/// ♻️ Returns the native guest, its receipts and its original Source before removing the instance.
pub(super) fn close_poll(poll: &mut NativePoll) -> bool {
    let status = semio_framework_async::block_on(poll.close(|_, _, _| Ok(true)));
    if status == GuestPollSessionStatus::Closed && poll.terminal_is_empty() { return true; }
    false
}

/// 🚪️ Opens the native actor while retaining its actual pending Source.
pub(super) fn open_poll(poll: &mut NativePoll, scope: &ActivationScope, deadline: std::time::Instant) -> Result<(), Fault> {
    next_poll_phase(poll, scope, deadline, NativePoll::open_step)
}

/// 🔜️ Advances one state-machine phase up to its original budget or cancellation boundary.
fn next_poll_phase(poll: &mut NativePoll, scope: &ActivationScope, deadline: std::time::Instant, step: impl Fn(&mut NativePoll) -> GuestPollSessionStatus) -> Result<(), Fault> {
    loop {
        scope.checkpoint()?;
        match step(poll) {
            GuestPollSessionStatus::Ready => return Ok(()),
            GuestPollSessionStatus::Pending if std::time::Instant::now() < deadline => std::thread::yield_now(),
            GuestPollSessionStatus::Pending => return Err(poll_deadline_fault(poll)),
            status => return Err(poll_fault(poll, status)),
        }
    }
}

/// 👂️ Borrows the received kernel output before acknowledging it back to its own Source.
pub(super) fn poll_turn<T>(poll: &mut NativePoll, purpose: PollPurpose, events: &[Event], scope: &ActivationScope, deadline: std::time::Instant, receive: impl FnOnce(&TurnResult) -> Result<T, Fault>) -> Result<T, Fault> {
    let mut receive = Some(receive);
    loop {
        next_poll_phase(poll, scope, deadline, NativePoll::next_input_step)?;
        let (ordinary, page) = match events {
            [page @ (Event::CommandIngressPage { .. } | Event::ColdDocumentPairPage(_))] => (&[][..], Some(page)),
            _ => (events, None),
        };
        let source = EventJsonSource(ordinary);
        let input = poll.event_input(&source, page).map_err(|error| PluginArtifactChannel::not_wired("guest poll input", error))?;
        let input = std::pin::pin!(input);
        let mut result = None;
        let mut steps_since_yield = 0;
        loop {
            scope.checkpoint()?;
            let status = poll.drive_step(input.as_ref(), &mut |output, _, _| {
                if let Some(receive) = receive.take() {
                    result = Some(match &output.result.status {
                        semio_framework::kernel::TurnStatus::Faulted(detail) => Err(decode_guest_fault(detail)),
                        _ => receive(&output.result),
                    });
                }
                Ok(true)
            });
            match status {
                GuestPollSessionStatus::Complete => {
                    if let Some(bytes) = poll.take_actor_fault_bytes() {
                        if result.is_some() { return Err(PluginArtifactChannel::not_wired("guest failure", "turn returned both a kernel output and an actor failure")); }
                        actor_failure::retry_lifecycle(&bytes, matches!(purpose, PollPurpose::Lifecycle).then_some(events))?;
                        scope.checkpoint()?;
                        if std::time::Instant::now() >= deadline { return Err(poll_deadline_fault(poll)); }
                        break;
                    }
                    return result.unwrap_or_else(|| Err(poll_fault(poll, status)));
                },
                GuestPollSessionStatus::Pending | GuestPollSessionStatus::Delivered if std::time::Instant::now() < deadline => {
                    steps_since_yield += 1;
                    if steps_since_yield == POLL_STEPS_PER_YIELD {
                        std::thread::yield_now();
                        steps_since_yield = 0;
                    }
                }
                GuestPollSessionStatus::Pending | GuestPollSessionStatus::Delivered => return Err(poll_deadline_fault(poll)),
                status => return Err(poll_fault(poll, status)),
            }
        }
    }
}

fn poll_deadline_fault(poll: &NativePoll) -> Fault {
    let mut fault = PluginArtifactChannel::budget_fault("guest poll");
    fault.message.push_str(&format!(" ({})", poll.pending_diagnostic()));
    fault
}

fn poll_fault(poll: &NativePoll, status: GuestPollSessionStatus) -> Fault {
    let failure = poll.failure().or_else(|| poll.cleanup_failure());
    if let Some(value) = failure.and_then(|failure| failure.primary.as_ref()).and_then(|primary| primary.primary_value()) {
        match value.kind {
            ValueRefusalKind::WorkLimit | ValueRefusalKind::OwnershipLimit => return Fault { message: format!("guest poll: {}", value.message), ..PluginArtifactChannel::budget_fault("guest poll") },
            ValueRefusalKind::Canceled => return Fault { code: ACTIVATION_CANCELLED_FAULT_CODE.to_string(), message: format!("guest poll: {}", value.message) },
            kind => return guest_refused(format!("{status:?}: {kind:?}: {}", value.message)),
        }
    }
    guest_refused(format!("{status:?} with {failure:?}; {}", poll.pending_diagnostic()))
}

/// 🧱️ A guest refusal as its typed [`GUEST_REFUSED_FAULT_CODE`] fault, never an unnamed internal error.
fn guest_refused(detail: String) -> Fault {
    Fault { code: GUEST_REFUSED_FAULT_CODE.to_string(), message: format!("guest poll: {detail}") }
}
