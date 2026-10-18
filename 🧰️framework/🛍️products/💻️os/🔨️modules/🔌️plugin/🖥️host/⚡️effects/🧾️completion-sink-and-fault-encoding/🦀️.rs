//! 🧾️ Completion sink and fault encoding: the envelope completion sink with its completion sink implementation and the fault, event and completed envelope encoders. See [`super`].

use super::*;

//#region 🧾️EnvelopeCompletionSink
/// 🧾️ The ONE `CompletionSink` implementation this packet ships: turns `(actor, generation,
/// event_bytes, lane)` into an `Envelope` and hands it to an [`EnvelopeInjector`] — generation-
/// gated (a completion whose snapshotted generation no longer matches the actor's CURRENT one, per
/// [`ActorScopeRegistry`], is dropped, never delivered) and mailbox-buffered (every completion
/// rides the actor's own private `EventRouter` topic honouring [`COMPLETION_MAILBOX_CAP`] via
/// `ChannelPolicy::LosslessBounded`, so a burst is bounded/rejected exactly the way any other
/// mailbox burst is, and a `Park`ed actor's completions simply accumulate there — undelivered but
/// never dropped — until [`EnvelopeCompletionSink::flush`] is called on resume).
// 🧬️ O1/R11(a) — generic over `I: EnvelopeInjector` rather than `Arc<dyn EnvelopeInjector>`: the
// whole-repo census (dyn-http-tail) found exactly ONE implementor anywhere (`RecordingEnvelopeInjector`,
// the test double the module doc already calls out — no real kernel/shard loop implements this trait
// yet) and BOTH `EnvelopeCompletionSink` and `AsyncEffectExecutor` are used ONLY inside this file
// (verified: `AsyncEffectExecutor::new` has zero production call sites repo-wide, only `mod tests`).
// Zero blast radius outside this file, so this is R11(a)'s trivial case, not R11(b) — nothing returns
// a runtime-chosen implementation, `inject` just consumes an owned `Envelope`.
pub struct EnvelopeCompletionSink<I: EnvelopeInjector> {
    pub(super) actors: ActorScopeRegistry,
    pub(super) events: Arc<EventRouter>,
    pub(super) injector: Arc<I>,
    subscribed: Mutex<std::collections::HashSet<u64>>,
}

impl<I: EnvelopeInjector> EnvelopeCompletionSink<I> {
    pub async fn new(actors: ActorScopeRegistry, events: Arc<EventRouter>, injector: Arc<I>) -> Self {
        Self { actors, events, injector, subscribed: Mutex::new(std::collections::HashSet::new()) }
    }

    fn ensure_subscribed(&self, actor: u64) {
        let mut subscribed = self.subscribed.lock().expect("EnvelopeCompletionSink subscribed mutex poisoned");
        if subscribed.insert(actor) {
            self.events.subscribe(completion_topic(actor), semio_framework_actor::ActorId(actor), ChannelPolicy::LosslessBounded { max_items: COMPLETION_MAILBOX_CAP, max_bytes: COMPLETION_MAILBOX_MAX_BYTES });
        }
    }

    /// ▶️ Delivers every currently-buffered completion for `actor`, in order, PROVIDED the actor's
    /// scope is `Live` (not `Park`ed, not `Cancelled`) — call on resume (after `CancelToken::unpark`)
    /// so a suspended actor's held completions land as soon as it comes back. Re-checks each
    /// buffered entry's generation against the CURRENT registry value (not just at `complete`-time)
    /// so an entry that went stale WHILE buffered (the actor restarted before it was ever flushed)
    /// is still dropped rather than misdelivered to the new incarnation.
    ///
    /// 🚫️async: R10 residue shape 2 — a future is consumed by a single `.await`; the codemod's
    /// insert-await pass had this awaited twice inside the loop below (E0382). Awaited once here.
    pub async fn flush(&self, actor: u64) {
        let Some(scope) = self.actors.scope_for(actor).await else { return };
        if !scope.cancel.is_live().await {
            return;
        }
        let topic = completion_topic(actor);
        let current_generation = self.actors.generation_of(actor).await;
        for wire in self.events.drain(&topic, semio_framework_actor::ActorId(actor)).await {
            let (lane, event_bytes) = decode_mailbox_entry(&wire).await;
            let Some(generation) = current_generation else {
                continue;
            };
            let envelope = build_envelope(actor, generation, lane, event_bytes.to_vec()).await;
            self.injector.inject(envelope).await;
        }
    }
}

impl<I: EnvelopeInjector> CompletionSink for EnvelopeCompletionSink<I> {
    /// 🚫️async: E1 — `semio_framework_os_services::CompletionSink::complete` is declared sync
    /// (`🛎️services/🦀️.rs:1539`), outside this packet's `🔌️plugin/🖥️host` path_scope to
    /// change. Everything this body needs (`generation_of`, `ensure_subscribed`, `send_message`,
    /// `flush`) is genuinely `async` (the last transitively through the generic `I: EnvelopeInjector`,
    /// which per this file's own R11(a) note has exactly one implementor today, a recording test
    /// double with no real I/O) — a textbook E5 sync/async bridge, block_on'd as one unit.
    ///
    /// 🛑️ Generation gate — snapshotted at dispatch time (`OperationContext.generation`),
    /// checked here at delivery time: a stale generation (the actor restarted since this
    /// operation was dispatched) is dropped, never delivered or even buffered.
    fn complete(&self, actor: u64, generation: u16, event_bytes: Vec<u8>, lane: u8) {
        semio_framework_async::block_on(async {
            if self.actors.generation_of(actor).await != Some(generation) {
                return;
            }
            self.ensure_subscribed(actor);
            let topic = completion_topic(actor);
            let _outcome: PublishOutcome = self.events.send_message(&topic, semio_framework_actor::ActorId(actor), encode_mailbox_entry(lane, &event_bytes).await).await;
            self.flush(actor).await;
        });
    }
}

async fn build_envelope(actor_stable: u64, generation: u16, lane_byte: u8, event_bytes: Vec<u8>) -> Envelope {
    let to = addressed_actor_id(actor_stable, generation).await;
    let lane = ActorLane::ALL.get(lane_byte as usize).copied().unwrap_or(ActorLane::Maintenance);
    Envelope { to, from: Origin::Kernel, lane, seq: 0, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Event { bytes: event_bytes } }
}
//#endregion 🧾️EnvelopeCompletionSink

//#region 🧯️Fault encoding
/// 🧯️ `RequestOutcome::Err`'s own doc: "an encoded fault the SDK decodes by originating request
/// kind." Mirrors `🖥️host/🦀️.rs`'s own (private, test-only-called-today)
/// `host_fault_bytes` helper exactly — duplicated locally rather than reached via `super::`, so
/// this module stays fully self-contained (no coupling to that file's own dead-code lint state).
async fn fault_bytes(code: impl Into<String>, message: impl Into<String>) -> Vec<u8> {
    store::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(&semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Os, semio_framework_diagnostic::FaultCode::new(code), message)))
}

pub(super) async fn encode_event(event: &Event) -> Vec<u8> {
    serde_json::to_vec(event).unwrap_or_default()
}

pub(super) async fn emit_completed_ok<I: EnvelopeInjector>(sink: &Arc<EnvelopeCompletionSink<I>>, ctx: &OperationContext, req: RequestId, bytes: Vec<u8>) {
    sink.complete(ctx.actor, ctx.generation, encode_event(&Event::Completed { req, result: RequestOutcome::Ok(bytes) }).await, ctx.lane);
}

pub(super) async fn emit_completed_err<I: EnvelopeInjector>(sink: &Arc<EnvelopeCompletionSink<I>>, ctx: &OperationContext, req: RequestId, code: &str, message: impl Into<String>) {
    sink.complete(ctx.actor, ctx.generation, encode_event(&Event::Completed { req, result: RequestOutcome::Err(fault_bytes(code, message.into()).await) }).await, ctx.lane);
}
//#endregion 🧯️Fault encoding
