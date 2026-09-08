import { interactivityStoreSyncFailures } from "../../../../../../../../📜️script.ts";

/** 🧪️ Executes interactivity store sync policy assertions. */
export function interactivityStoreSyncSelfTests(): void {
  const good = `slots: [Option<ArtifactMailboxSlot>; ARTIFACT_MAILBOX_ITEMS] ARTIFACT_MAILBOX_BYTES.saturating_sub(state.bytes) artifact_actor_message_bytes(&message) pub fn into_message(self) -> ArtifactActorMsg wake_armed wake_requested.compare_exchange(false, true pub(super) async fn drive_one(&mut self) -> ArtifactDrive { if let Some(message) = self.cmd_rx.try_recv() self.remote.try_pop_front()? } struct OpenDocument { runner: ArtifactActorRunnerHandle } closing: std::collections::HashMap<u64, ArtifactActorRunnerHandle> runner: ArtifactActorRunnerTicket runner: std::sync::Weak<ActorRunner> returned: bool struct ActorRunner { deadline_generation deadline_armed retry_generation terminal_turn self_retained: std::sync::Mutex<Option<Arc<ActorRunner>>> external_tickets: std::sync::atomic::AtomicUsize } scheduled.compare_exchange(false, true future.as_mut().poll(&mut context) self.pool.try_submit(semio_framework_async::Lane::UserVisible, job) error.into_job() self.pool.callback_at ACTOR_RUNNER_RETRY_LIMIT fn take_terminal_job close_one_terminal_owner self.mailbox.close_one() self.terminal_is_empty() self.external_tickets.load(std::sync::atomic::Ordering::Acquire) == 0 self.self_retained.lock() Some(runner.clone()) runner.return_ticket() pub(super) async fn spawn_actor() -> ArtifactActorRunnerHandle runner.set_terminal_empty_callback closing.insert(generation, runner.clone()) runner.request_close() pub fn closing_runner pub fn close_step pub fn terminal_is_empty pub fn take_terminal_job artifact_mailbox_item_cap_plus_one_returns_exact_owner_and_preserves_fifo artifact_mailbox_byte_cap_and_plus_one_preflight_before_mutation artifact_mailbox_wake_storm_coalesces_until_fifo_becomes_empty artifact_mailbox_stale_late_send_hands_back_exact_owner_and_interrupted_close_drains_one_per_grant artifact_mailbox_nested_identifier_bytes_and_backbone_one_pop_preserve_ownership_order stale_generation_wake_cannot_schedule_or_mutate_current_turn turn_fault_and_cancel_retain_then_close_one_owner_per_grant quiet_pool_saturation_retains_exact_successor_for_timer_wheel_retry idle_runner_is_strongly_retained_and_quiet_late_wake_schedules_once idle_then_late_send_upgrades_the_host_retained_runner_once external_ticket_held_across_close_delays_completion_until_exact_return external_ticket_dropped_before_close_and_generation_aba_are_exact terminal_job_take_resume_and_close_preserve_exact_owner host_close_registry_survives_external_ticket_until_return ticket_return_before_host_close_allows_immediate_retirement detach_while_pending_retains_future_then_cancel_closes_one_owner`;
  const mutations = [
    ["pool-block-on", `${good} runtime.block_on`],
    ["tokio-spawn", `${good} tokio::spawn`],
    ["unbounded-mailbox", good.replace("slots: [Option<ArtifactMailboxSlot>; ARTIFACT_MAILBOX_ITEMS]", "UnboundedSender<ArtifactActorMsg")],
    ["mailbox-byte-preflight", good.replace("ARTIFACT_MAILBOX_BYTES.saturating_sub(state.bytes)", "usize::MAX")],
    ["wake-storm", good.replace("wake_requested.compare_exchange(false, true", "wake_requested.store(true")],
    ["multi-command-drain", good.replace("if let Some(message) = self.cmd_rx.try_recv()", "while let Some(message) = self.cmd_rx.try_recv()")],
    ["backbone-drain", `${good} self.remote.drain()`],
    ["quiet-saturation-strand", good.replace("self.pool.callback_at", "drop")],
    ["terminal-job-sink", good.replace("fn take_terminal_job", "fn inspect_terminal_job")],
    ["terminal-drain-all", good.replace("close_one_terminal_owner", "close_all_terminal_owners")],
    ["idle-poll", good.replace("deadline_generation", "deadline_generation saturating_add(4)")],
    ["missing-strong-host-handle", good.replace("runner: ArtifactActorRunnerHandle", "runner: std::sync::Weak<ActorRunner>")],
    ["quiet-runner-drop", good.replace("self_retained: std::sync::Mutex<Option<Arc<ActorRunner>>>", "self_retained: ()")],
    ["strong-external-channel-handle", good.replace("runner: ArtifactActorRunnerTicket", "runner: ArtifactActorRunnerHandle")],
    ["ticket-return-not-terminal-gated", good.replace("self.external_tickets.load(std::sync::atomic::Ordering::Acquire) == 0", "true")],
    ["missing-host-terminal-callback", good.replace("runner.set_terminal_empty_callback", "drop")],
  ] as const;
  for (const [name, source] of mutations) if (interactivityStoreSyncFailures(source).length === 0) throw new Error(`[verify interactivity] store-sync self-test ${name} was falsely accepted.`);
  if (interactivityStoreSyncFailures(good).length !== 0) throw new Error("[verify interactivity] store-sync self-test retained bounded actor turn was falsely rejected.");
}
