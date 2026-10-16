//! 🪢️ The wasm32-wasip2 translation between the WIT reactor world and the kernel types.

use semio_framework_plugin_reactor_core::ARMED_TIMERS;
use semio_framework::kernel::{ActorInstanceLifecycleReceipt, ActorUiPatchReceipt, Effect, Event, MessageEndpoint, TurnStatus, UiPatch};
use std::cell::RefCell;
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
use semio_framework::kernel::{RequestOutcome, UiPatchOp};
use semio_framework_plugin_reactor_surfaces::checkpoint;
use semio_framework_plugin_reactor_surfaces as cold_pair;
use semio_framework_plugin_reactor_turn as turn;
use semio_framework_ui_contract as ui_contract;
/// 🧭️ `reactor`/`jobs`/`checkpoint`/`describe` are the only interfaces `world actor` directly
/// `export`s, so wit-bindgen only aliases THEIR top-level types under `exports::…`. `effects`/
/// `events`/`ui`/`types` are merely `use`d by `reactor.wit` (design-abi.md §1/§4) — their own
/// payload records live at the plain (non-`exports::`) path alongside the `pure` import, one
/// level down from where the nesting stops being re-aliased. Verified empirically: a
/// deliberately wrong `wit::OpenWindowEffect` import made `cargo check --target wasm32-wasip2`
/// emit `help: consider importing … effects::OpenWindowEffect`
/// (and the `events`/`ui` siblings the same way) — not guessed.
use semio_framework_plugin_abi::wasip2::semio::framework::capabilities as wit_capabilities;
use semio_framework_plugin_abi::wasip2::semio::framework::effects as wit_effects;
use semio_framework_plugin_abi::wasip2::semio::framework::events as wit_events;
use semio_framework_plugin_abi::wasip2::semio::framework::instance_lifetime as wit_lifetime;
use semio_framework_plugin_abi::wasip2::semio::framework::types as wit_types;
use semio_framework_plugin_abi::wasip2::semio::framework::ui as wit_ui;

semio_framework_plugin_abi::component_persistent_local! {
    /// 📥️ The one command-ingress page staged for the next turn, and the one cold document-pair
    /// page beside it. Staged by their own exports and TAKEN by `poll`, never left behind.
    static STAGED_PAGES: RefCell<StagedTurnPages> = RefCell::new(StagedTurnPages { command: None, cold_pair: None });
}

/// 📦️ Both pages are BOXED. A staged `FixedCommandPage` is 4 KiB by value, and every `.with()`
/// take on the deepest frame of the turn would memcpy it across the guest's 64 KiB shadow stack —
/// measured as `memory fault at wasm address 0xffff3798`, a stack-pointer underflow, on turn 0.
#[derive(Default)]
struct StagedTurnPages {
    pub command: Option<(semio_framework::kernel::CommandPageCursor, Box<semio_framework::kernel::FixedCommandPage>)>,
    pub cold_pair: Option<Box<semio_framework::kernel::ColdDocumentPairPage>>,
}

fn staging_fault(reason: &'static str) -> semio_framework::Fault {
    semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.turn-page-staging"), reason)
}

/// 📥️ `reactor::stage-command-page` body — see the WIT for why the page is staged instead of
/// riding `poll`'s parameter list.
pub async fn stage_command_page(cursor: semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor::CommandPageCursor, bytes: Vec<u8>) -> Result<(), semio_framework::Fault> {
    let staged = wit_command_page_to_kernel(cursor, bytes)?;
    let staged = (staged.0, Box::new(staged.1));
    STAGED_PAGES.with(|pages| {
        let mut pages = pages.try_borrow_mut().map_err(|_| staging_fault("turn page staging authority busy"))?;
        if pages.command.is_some() {
            return Err(staging_fault("a command page is already staged for the next turn"));
        }
        pages.command = Some(staged);
        Ok(())
    })
}

/// 🧊️ `reactor::stage-cold-pair-page` body — the cold-pair twin of [`stage_command_page`].
pub async fn stage_cold_pair_page(page: semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor::ColdDocumentPairPage) -> Result<(), semio_framework::Fault> {
    let staged = Box::new(wit_cold_pair_page_to_kernel(page)?);
    STAGED_PAGES.with(|pages| {
        let mut pages = pages.try_borrow_mut().map_err(|_| staging_fault("turn page staging authority busy"))?;
        if pages.cold_pair.is_some() {
            return Err(staging_fault("a cold document-pair page is already staged for the next turn"));
        }
        pages.cold_pair = Some(staged);
        Ok(())
    })
}

/// ▶️ The real `reactor::poll` body — see module doc for the shape. `events`/`budget` are the
/// WIT-generated types from `exports::semio::framework::reactor`; the return is that same
/// module's `TurnResult`. The turn's pages come from [`STAGED_PAGES`], which this call empties.
pub async fn poll<PA: semio_framework_plugin_reactor_app::PluginApp + 'static>(
    runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>,
    events: Vec<semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor::Event>,
    budget: semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor::Budget,
    identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,
    cx:&mut semio_framework_job::StepContext<'_>,
) -> Result<semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor::TurnResult, semio_framework_job::OriginalTurnFailure> {
    turn::note_turn_events(events.len(), wit_events_payload_bytes(&events));
    let mut kernel_events = Vec::with_capacity(events.len());
    for event in events {
        kernel_events.push(wit_event_to_kernel(event));
    }
    let kernel_budget = semio_framework::kernel::Budget { retained:wit_retained_input_to_kernel(budget.retained)?, retained_policy:wit_retained_grant_to_kernel(budget.retained_policy)?,work_fuel:budget.work_fuel, fuel: budget.fuel, deadline_us: budget.deadline_us, max_effects: budget.max_effects, max_patch_bytes: budget.max_patch_bytes, max_frames: budget.max_frames };
    let staged = STAGED_PAGES.with(|pages| pages.try_borrow_mut().map(|mut pages| std::mem::take(&mut *pages)).map_err(|_| staging_fault("turn page staging authority busy")))?;
    turn::poll_kernel_output(runtime, kernel_events, staged.command.map(|(cursor, page)| (cursor, *page)), staged.cold_pair.map(|page| *page), kernel_budget, |result| kernel_turn_result_to_wit(result, budget), |_, prepared| prepared, identity,cx).await
}

/// 🩺️ Bulk bytes a turn's events carry across the component boundary — every `pack`/`list<u8>`
/// field, which is all a turn can retain that scales with its input. Fixed-shape variants
/// (`timer`, `wake`, `surface-resized`, …) contribute nothing by construction, so a turn reading
/// `eventsBytes=0` genuinely carried no payload. Read by the per-turn guest memory trace.
fn wit_events_payload_bytes(events: &[semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor::Event]) -> usize {
    use semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor as wit;
    let completion_bytes = |outcome: &semio_framework_plugin_abi::wasip2::CompletionResult| match outcome {
        semio_framework_plugin_abi::wasip2::CompletionResult::Ok(pack) | semio_framework_plugin_abi::wasip2::CompletionResult::Fault(pack) => pack.len(),
    };
    events
        .iter()
        .map(|event| match event {
            wit::Event::InstanceOpen(open) => open.config.len() + open.quotas.len() + open.assets.iter().map(|(name, pack)| name.len() + pack.len()).sum::<usize>(),
            wit::Event::QuotaChanged(changed) => changed.quotas.len(),
            wit::Event::UiIntent(intent) => intent.intent.len(),
            wit::Event::SurfaceVisible(visible) => visible.body_key.len() + visible.view_state.len(),
            wit::Event::PatchRejected(rejected) => rejected.reason.len(),
            wit::Event::Completed(completed) => completion_bytes(&completed.outcome),
            wit::Event::HttpChunk(chunk) => chunk.params.bytes.len(),
            wit::Event::JobProgress(progress) => progress.progress.len(),
            wit::Event::JobCompleted(completed) => completion_bytes(&completed.outcome),
            wit::Event::Message(message) => message.payload.len(),
            wit::Event::Request(request) => request.params.capability.len() + request.params.payload.len(),
            wit::Event::InstanceClose(_) | wit::Event::InstanceLifecycleAck(_) | wit::Event::Activate(_) | wit::Event::SuspendRequest(_) | wit::Event::CapabilityChanged(_) => 0,
            wit::Event::SurfaceHidden(_) | wit::Event::SurfaceResized(_) | wit::Event::PatchAck(_) | wit::Event::Timer(_) | wit::Event::Wake => 0,
        })
        .sum()
}

/// 📄️ Lifts one staged command page into the kernel's fixed 4 KiB owner. The bytes arrive as a
/// `list<u8>` the guest owns and drops; the fixed page they are copied into is the authority the
/// ingress assembler reads.
fn wit_command_page_to_kernel(
    cursor: semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor::CommandPageCursor,
    bytes: Vec<u8>,
) -> Result<(semio_framework::kernel::CommandPageCursor, semio_framework::kernel::FixedCommandPage), semio_framework::Fault> {
    if bytes.len() > semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES {
        return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.command-page-lift-cap"), "fixed command page declares more than 4096 bytes"));
    }
    let length = bytes.len() as u32;
    let mut fixed = [0; semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES];
    fixed[..bytes.len()].copy_from_slice(&bytes);
    let bytes = semio_framework::kernel::FixedCommandPage::try_from_array(fixed, length)?;
    Ok((
        semio_framework::kernel::CommandPageCursor {
            owner: cursor.owner,
            generation: cursor.generation,
            command_index: cursor.command_index,
            command_count: cursor.command_count,
            instance: cursor.instance,
            seq: cursor.seq,
            kind: cursor.kind,
            page_index: cursor.page_index,
            page_count: cursor.page_count,
            item_count: cursor.item_count,
            metadata: cursor.metadata,
        },
        bytes,
    ))
}

#[doc(hidden)]
pub fn kernel_command_cursor_to_wit(cursor: &semio_framework::kernel::CommandPageCursor) -> semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor::CommandPageCursor {
    use semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor as wit;
    wit::CommandPageCursor {
        owner: cursor.owner,
        generation: cursor.generation,
        command_index: cursor.command_index,
        command_count: cursor.command_count,
        instance: cursor.instance,
        seq: cursor.seq,
        kind: cursor.kind,
        page_index: cursor.page_index,
        page_count: cursor.page_count,
        item_count: cursor.item_count,
        metadata: cursor.metadata,
    }
}

/// 💤️ Canonical cursor payload for the scalar idle command-ingress record.
fn idle_command_cursor_to_wit() -> semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor::CommandPageCursor {
    use semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor as wit;
    wit::CommandPageCursor { owner: 0, generation: 0, command_index: 0, command_count: 0, instance: 0, seq: 0, kind: 0, page_index: 0, page_count: 0, item_count: 0, metadata: 0 }
}

/// 🔢️ Kernel command ingress → scalar WIT record, avoiding nested variant discriminants in async results.
fn kernel_command_ingress_to_wit(status: semio_framework::kernel::CommandIngressStatus) -> semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor::CommandIngressStatus {
    use semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor as wit;
    let (kind, cursor, fault) = match status {
        semio_framework::kernel::CommandIngressStatus::Idle => (0, idle_command_cursor_to_wit(), Vec::new()),
        semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor) => (1, kernel_command_cursor_to_wit(&cursor), Vec::new()),
        semio_framework::kernel::CommandIngressStatus::Backpressure(cursor) => (2, kernel_command_cursor_to_wit(&cursor), Vec::new()),
        semio_framework::kernel::CommandIngressStatus::CommandPending(cursor) => (3, kernel_command_cursor_to_wit(&cursor), Vec::new()),
        semio_framework::kernel::CommandIngressStatus::CommandComplete(cursor) => (4, kernel_command_cursor_to_wit(&cursor), Vec::new()),
        semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault } => (5, kernel_command_cursor_to_wit(&cursor), fault),
    };
    wit::CommandIngressStatus { kind, cursor, fault }
}

fn wit_cold_sha256(bytes: Vec<u8>, field: &'static str) -> Result<[u8; 32], semio_framework::Fault> {
    bytes.try_into().map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.cold-pair-hash-width"), field))
}

fn wit_cold_pair_page_to_kernel(page: semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor::ColdDocumentPairPage) -> Result<semio_framework::kernel::ColdDocumentPairPage, semio_framework::Fault> {
    let header = page.header;
    let frontier = header.baseline_frontier;
    Ok(semio_framework::kernel::ColdDocumentPairPage {
        header: semio_framework::kernel::ColdDocumentPairHeader {
            lifetime: wit_lifetime_to_kernel(header.lifetime),
            transfer_generation: header.transfer_generation,
            descriptor_sha256: wit_cold_sha256(header.descriptor_sha256, "cold pair descriptor hash must be 32 bytes")?,
            baseline_frontier: semio_framework::kernel::ColdDocumentPairFrontier {
                document_id: frontier.document_id,
                head_edit_ordinal: frontier.head_edit_ordinal,
                head_edit_id: frontier.head_edit_id,
                last_commit_seq: frontier.last_commit_seq,
                chain_sha256: wit_cold_sha256(frontier.chain_sha256, "cold pair frontier hash must be 32 bytes")?,
            },
            pack_sha256: wit_cold_sha256(header.pack_sha256, "cold pair Pack hash must be 32 bytes")?,
            spr_sha256: wit_cold_sha256(header.spr_sha256, "cold pair SPR hash must be 32 bytes")?,
            aggregate_sha256: wit_cold_sha256(header.aggregate_sha256, "cold pair aggregate hash must be 32 bytes")?,
            pack_length: header.pack_length,
            spr_length: header.spr_length,
            page_count: header.page_count,
        },
        page_index: page.page_index,
        bytes: page.bytes,
    })
}

fn kernel_cold_frontier_to_wit(frontier: semio_framework::kernel::ColdDocumentPairFrontier) -> semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor::ColdDocumentPairFrontier {
    use semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor as wit;
    wit::ColdDocumentPairFrontier { document_id: frontier.document_id, head_edit_ordinal: frontier.head_edit_ordinal, head_edit_id: frontier.head_edit_id, last_commit_seq: frontier.last_commit_seq, chain_sha256: frontier.chain_sha256.to_vec() }
}

fn kernel_cold_cursor_to_wit(cursor: semio_framework::kernel::ColdDocumentPairCursor) -> semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor::ColdDocumentPairCursor {
    use semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor as wit;
    wit::ColdDocumentPairCursor { lifetime: kernel_lifetime_to_wit(cursor.lifetime), transfer_generation: cursor.transfer_generation, page_index: cursor.page_index, page_count: cursor.page_count }
}

fn kernel_cold_pair_ingress_to_wit(status: semio_framework::kernel::ColdPairIngressStatus) -> semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor::ColdPairIngressStatus {
    use semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor as wit;
    match status {
        semio_framework::kernel::ColdPairIngressStatus::Idle => wit::ColdPairIngressStatus::Idle,
        semio_framework::kernel::ColdPairIngressStatus::PageAccepted(cursor) => wit::ColdPairIngressStatus::PageAccepted(kernel_cold_cursor_to_wit(cursor)),
        semio_framework::kernel::ColdPairIngressStatus::Backpressure(cursor) => wit::ColdPairIngressStatus::Backpressure(kernel_cold_cursor_to_wit(cursor)),
        semio_framework::kernel::ColdPairIngressStatus::Loading(cursor) => wit::ColdPairIngressStatus::Loading(kernel_cold_cursor_to_wit(cursor)),
        semio_framework::kernel::ColdPairIngressStatus::Applied(receipt) => wit::ColdPairIngressStatus::Applied(wit::ColdDocumentPairApplied {
            lifetime: kernel_lifetime_to_wit(receipt.lifetime),
            transfer_generation: receipt.transfer_generation,
            baseline_frontier: kernel_cold_frontier_to_wit(receipt.baseline_frontier),
            aggregate_sha256: receipt.aggregate_sha256.to_vec(),
        }),
        semio_framework::kernel::ColdPairIngressStatus::Fault { cursor, fault } => wit::ColdPairIngressStatus::Fault(wit::ColdDocumentPairFault { cursor: kernel_cold_cursor_to_wit(cursor), fault }),
    }
}

fn decode_wire_quotas(bytes: &[u8]) -> semio_framework::kernel::QuotaSchema {
    store::pack_rt::decode_wire_value(bytes).ok().and_then(|value| semio_framework_value::FromValue::from_value(value).ok()).unwrap_or_default()
}

/// 🔀️ WIT `event` → kernel `Event`. Thin field-for-field translation — the WIT side already
/// mirrors the kernel shape (see `📓️design-abi.md` §2 / `events.wit`'s own doc comments).
#[doc(hidden)]
pub fn wit_lifetime_to_kernel(value: semio_framework_plugin_abi::wasip2::Lifetime) -> semio_framework::kernel::ActorInstanceLifetime {
    semio_framework::kernel::ActorInstanceLifetime { activation_generation: value.activation_generation, instance_id: value.instance_id, guest_lifetime: value.guest_lifetime }
}

#[doc(hidden)]
pub fn kernel_lifetime_to_wit(value: semio_framework::kernel::ActorInstanceLifetime) -> semio_framework_plugin_abi::wasip2::Lifetime {
    semio_framework_plugin_abi::wasip2::Lifetime { activation_generation: value.activation_generation, instance_id: value.instance_id, guest_lifetime: value.guest_lifetime }
}

#[doc(hidden)]
pub fn wit_patch_receipt_to_kernel(value: semio_framework_plugin_abi::wasip2::UiPatchReceipt) -> ActorUiPatchReceipt {
    ActorUiPatchReceipt { lifetime: wit_lifetime_to_kernel(value.lifetime), patch_sequence: value.patch_sequence }
}

#[doc(hidden)]
pub fn kernel_patch_receipt_to_wit(value: ActorUiPatchReceipt) -> semio_framework_plugin_abi::wasip2::UiPatchReceipt {
    semio_framework_plugin_abi::wasip2::UiPatchReceipt { lifetime: kernel_lifetime_to_wit(value.lifetime), patch_sequence: value.patch_sequence }
}

#[doc(hidden)]
pub fn wit_lifecycle_receipt_to_kernel(value: semio_framework_plugin_abi::wasip2::Receipt) -> ActorInstanceLifecycleReceipt {
    use semio_framework::kernel::ActorInstanceLifecycleReceipt as R;
    match value {
        semio_framework_plugin_abi::wasip2::Receipt::Captured(value) => R::Captured { lifetime: wit_lifetime_to_kernel(value.lifetime), request_sequence: value.request_sequence },
        semio_framework_plugin_abi::wasip2::Receipt::Accepted(value) => R::Accepted { lifetime: wit_lifetime_to_kernel(value.lifetime), request_sequence: value.request_sequence, close_generation: value.close_generation },
        semio_framework_plugin_abi::wasip2::Receipt::Retired(value) => R::Retired { lifetime: wit_lifetime_to_kernel(value.lifetime), request_sequence: value.request_sequence, close_generation: value.close_generation },
    }
}

#[doc(hidden)]
pub fn kernel_lifecycle_receipt_to_wit(value: ActorInstanceLifecycleReceipt) -> semio_framework_plugin_abi::wasip2::Receipt {
    use semio_framework::kernel::ActorInstanceLifecycleReceipt as R;
    match value {
        R::Captured { lifetime, request_sequence } => semio_framework_plugin_abi::wasip2::Receipt::Captured(semio_framework_plugin_abi::wasip2::CapturedReceipt { lifetime: kernel_lifetime_to_wit(lifetime), request_sequence }),
        R::Accepted { lifetime, request_sequence, close_generation } => semio_framework_plugin_abi::wasip2::Receipt::Accepted(semio_framework_plugin_abi::wasip2::CloseReceipt { lifetime: kernel_lifetime_to_wit(lifetime), request_sequence, close_generation }),
        R::Retired { lifetime, request_sequence, close_generation } => semio_framework_plugin_abi::wasip2::Receipt::Retired(semio_framework_plugin_abi::wasip2::CloseReceipt { lifetime: kernel_lifetime_to_wit(lifetime), request_sequence, close_generation }),
    }
}

/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): previously always
/// `default()` — `spawn_task`'s quota gate is the first real reader of this field, so a
/// decode failure (malformed/empty pack) also falls back to `default()` rather than
/// failing `InstanceOpen` outright; a missing quota is "no limit declared", not a fault.
///
/// 🎬️ `wit-flip` (26/08/20): UI intents no longer masquerade as `app-command` — see kernel
/// `Event::UiIntent`'s own doc.
fn wit_event_to_kernel(event: semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor::Event) -> Event {
    use semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor::Event as W;
    match event {
        W::InstanceOpen(payload) => Event::InstanceOpen {
            request: semio_framework::kernel::ActorInstanceOpenRequest { activation_generation: payload.activation_generation, instance_id: payload.instance, request_sequence: payload.request_sequence },
            app_id: semio_framework::kernel::AppInstanceId(payload.app_id),
            actor: payload.actor,
            config: payload.config,
            assets: payload.assets,
            capabilities: Vec::new(),
            quotas: decode_wire_quotas(&payload.quotas),
        },
        W::InstanceClose(payload) => Event::InstanceClose(semio_framework::kernel::ActorInstanceCloseRequest { lifetime: wit_lifetime_to_kernel(payload.lifetime), request_sequence: payload.request_sequence }),
        W::InstanceLifecycleAck(receipt) => Event::InstanceLifecycleAck(semio_framework::kernel::ActorInstanceLifecycleAck { receipt: wit_lifecycle_receipt_to_kernel(receipt) }),
        W::Activate(payload) => Event::Activate { reason: wit_activation_to_kernel(payload.reason) },
        W::SuspendRequest(_) => Event::SuspendRequest,
        W::CapabilityChanged(payload) => Event::CapabilityChanged { change: wit_capability_change_to_kernel(payload.change) },
        W::QuotaChanged(payload) => Event::QuotaChanged { quotas: decode_wire_quotas(&payload.quotas) },
        W::UiIntent(payload) => Event::UiIntent { instance: semio_framework::kernel::PluginInstanceId(payload.instance.to_string()), intent: payload.intent },
        W::SurfaceVisible(payload) => Event::SurfaceVisible { surface: format!("{}:{}", payload.surface.instance, payload.surface.surface), body_key: payload.body_key, view_state: payload.view_state },
        W::SurfaceHidden(payload) => Event::SurfaceHidden { surface: format!("{}:{}", payload.surface.instance, payload.surface.surface) },
        W::SurfaceResized(payload) => Event::SurfaceResized { surface: format!("{}:{}", payload.surface.instance, payload.surface.surface), width: payload.width, height: payload.height },
        W::PatchAck(payload) => Event::PatchAck { receipt: wit_patch_receipt_to_kernel(payload.receipt), surface: format!("{}:{}", payload.surface.instance, payload.surface.surface), revision: payload.revision },
        W::PatchRejected(payload) => Event::PatchRejected { receipt: wit_patch_receipt_to_kernel(payload.receipt), surface: format!("{}:{}", payload.surface.instance, payload.surface.surface), revision: payload.revision, reason: payload.reason },
        W::Completed(payload) => Event::Completed { req: semio_framework::kernel::RequestId(payload.req), result: wit_completion_to_kernel(payload.outcome) },
        W::HttpChunk(payload) => Event::HttpChunk { req: semio_framework::kernel::RequestId(payload.req), bytes: payload.params.bytes, done: payload.params.done },
        W::JobProgress(payload) => Event::JobProgress { job: payload.job, progress: Some(payload.progress) },
        W::JobCompleted(payload) => Event::JobCompleted { job: payload.job, result: wit_completion_to_kernel(payload.outcome) },
        W::Message(payload) => Event::Message { source: wit_endpoint_to_kernel(payload.source), payload: payload.payload },
        W::Timer(payload) => Event::Timer { id: payload.id },
        W::Wake => Event::Wake,
        W::Request(payload) => Event::Request { req: semio_framework::kernel::RequestId(payload.req), from: wit_endpoint_to_kernel(payload.params.origin), capability: payload.params.capability, payload: payload.params.payload },
    }
}

/// 🎟️ Preserves broker changes as changes rather than turning them into resource shutdown.
fn wit_capability_change_to_kernel(change: semio_framework_plugin_abi::wasip2::CapabilityChange) -> semio_framework::kernel::CapabilityChange {
    use semio_framework::kernel::{BrokerCapabilityGrant, CapabilityChange, CapabilityId, CapabilityToken};
    let grant = |value: semio_framework_plugin_abi::wasip2::CapabilityGrant| BrokerCapabilityGrant {
        id: CapabilityId(value.token.id),
        token: CapabilityToken(u128::from(value.token.token)),
        scope: value.scope,
        expires_ms: value.expires_ms.and_then(|value| u64::try_from(value).ok()),
    };
    match change {
        semio_framework_plugin_abi::wasip2::CapabilityChange::Granted(value) => {
            let grant = grant(value);
            CapabilityChange::Granted { id: grant.id.clone(), grant }
        }
        semio_framework_plugin_abi::wasip2::CapabilityChange::Revoked(id) => CapabilityChange::Revoked { id: CapabilityId(id) },
        semio_framework_plugin_abi::wasip2::CapabilityChange::Narrowed(value) => {
            let grant = grant(value);
            CapabilityChange::Narrowed { id: grant.id.clone(), grant }
        }
    }
}

fn wit_activation_to_kernel(reason: semio_framework_plugin_abi::wasip2::ActivationEvent) -> semio_framework::kernel::ActivationEvent {
    use semio_framework_plugin_abi::wasip2::ActivationEvent as W;
    match reason {
        W::OnCommand(id) => semio_framework::kernel::ActivationEvent::OnCommand { id },
        W::OnViewVisible(id) => semio_framework::kernel::ActivationEvent::OnViewVisible { id },
        W::OnFileType(ext) => semio_framework::kernel::ActivationEvent::OnFileType { ext },
        W::OnArtifactKind(kind) => semio_framework::kernel::ActivationEvent::OnArtifactKind { kind },
        W::OnExtensionRequest(point) => semio_framework::kernel::ActivationEvent::OnExtensionRequest { point },
        W::OnStartupFinished => semio_framework::kernel::ActivationEvent::OnStartupFinished,
    }
}

fn wit_completion_to_kernel(result: semio_framework_plugin_abi::wasip2::CompletionResult) -> RequestOutcome {
    use semio_framework_plugin_abi::wasip2::CompletionResult as W;
    match result {
        W::Ok(bytes) => RequestOutcome::Ok(bytes),
        W::Fault(bytes) => RequestOutcome::Err(bytes),
    }
}

fn wit_endpoint_to_kernel(endpoint: semio_framework_plugin_abi::wasip2::MessageEndpoint) -> MessageEndpoint {
    use semio_framework_plugin_abi::wasip2::MessageEndpoint as W;
    match endpoint {
        W::Shell(instance) => MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) },
        W::Backbone(uri) => MessageEndpoint::Backbone { uri },
        W::PluginInstance(instance) => MessageEndpoint::PluginInstance { id: semio_framework::kernel::PluginInstanceId(instance.to_string()) },
        W::Extension(id) => MessageEndpoint::Extension { id },
        W::Topic(name) => MessageEndpoint::Topic { name },
    }
}

/// 🔀️ kernel `TurnResult` → WIT `turn-result`. `budget` is currently unused beyond documenting
/// the seam — `max-effects`/`max-patch-bytes` capping is real, mechanical follow-up work (design-
/// abi.md §4's "capped by `max-effects`, overflow carries over") not yet wired into this wave.
///
/// 👥️ M2 (ticket 26/08/17 `design-unified.md`): `presence` is now real — `kernel::TurnResult`
/// gained the matching field this doc used to say it lacked (see `poll`'s own body for the
/// `PresenceHub` that fills it). Marshaled through the SAME `presence-update.peer: pack` WIT field
/// the schema already declares — that field is opaque `pack` bytes at the WIT boundary already, so
/// no STRUCTURAL wit change is needed to repoint what it carries: `wit-flip`'s doc comment there
/// still names the OLD replication `PresencePeer` payload; this packet's report carries the exact
/// (doc-comment-only, plus a field rename for clarity) WIT diff as a registrar lease-request.
fn wit_retained_grant_to_kernel(grant:semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor::RetainedGrant)->Result<semio_framework_value::RetainedCloneGrant,semio_framework::Fault>{let extent=|value|usize::try_from(value).map_err(|_|staging_fault("original policy wire extent exceeds native address space"));Ok(semio_framework_value::RetainedCloneGrant{maximum_items:extent(grant.maximum_items)?,maximum_copy_bytes:extent(grant.maximum_copy_bytes)?,maximum_capacity_bytes:extent(grant.maximum_capacity_bytes)?,maximum_release_bytes:extent(grant.maximum_release_bytes)?,maximum_depth:extent(grant.maximum_depth)?})}
fn wit_retained_input_to_kernel(input:semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor::RetainedTurnInput)->Result<semio_framework::kernel::RetainedTurnInput,semio_framework::Fault>{let extent=|value:u64|usize::try_from(value).map_err(|_|staging_fault("original retained wire extent exceeds native address space"));let grant=input.grant;let input=semio_framework::kernel::RetainedTurnInput{operation:input.operation,generation:input.generation,epoch:input.epoch,work:semio_framework_actor::OriginalActorTurnWork{clock_binding:input.work.clock_binding,absolute_deadline_us:input.work.absolute_deadline_us,fuel:input.work.fuel},grant:semio_framework_value::RetainedCloneGrant{maximum_items:extent(grant.maximum_items)?,maximum_copy_bytes:extent(grant.maximum_copy_bytes)?,maximum_capacity_bytes:extent(grant.maximum_capacity_bytes)?,maximum_release_bytes:extent(grant.maximum_release_bytes)?,maximum_depth:extent(grant.maximum_depth)?}};input.validate().map_err(|error|semio_framework_diagnostic::FaultFrom::to_fault(&error))?;Ok(input)}
fn kernel_retained_grant_to_wit(grant:semio_framework_value::RetainedCloneGrant)->semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor::RetainedGrant{use semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor as wit;wit::RetainedGrant{maximum_items:grant.maximum_items as u64,maximum_copy_bytes:grant.maximum_copy_bytes as u64,maximum_capacity_bytes:grant.maximum_capacity_bytes as u64,maximum_release_bytes:grant.maximum_release_bytes as u64,maximum_depth:grant.maximum_depth as u64}}
fn kernel_retained_receipt_to_wit(receipt:semio_framework::kernel::RetainedTurnReceipt)->semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor::RetainedTurnReceipt{use semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor as wit;wit::RetainedTurnReceipt{input:wit::RetainedTurnInput{operation:receipt.input.operation,generation:receipt.input.generation,epoch:receipt.input.epoch,work:wit::OriginalActorTurnWork{clock_binding:receipt.input.work.clock_binding,absolute_deadline_us:receipt.input.work.absolute_deadline_us,fuel:receipt.input.work.fuel},grant:kernel_retained_grant_to_wit(receipt.input.grant)},spent:wit::RetainedProgress{copied_items:receipt.spent.copied_items as u64,copied_bytes:receipt.spent.copied_bytes as u64,retained_capacity_bytes:receipt.spent.retained_capacity_bytes as u64,released_bytes:receipt.spent.released_bytes as u64},work_units_spent:receipt.work_units_spent,remaining:kernel_retained_grant_to_wit(receipt.remaining),remaining_fuel:receipt.remaining_fuel}}

fn kernel_turn_result_to_wit(
    result: &semio_framework::kernel::TurnResult,
    _budget: semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor::Budget,
) -> Result<semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor::TurnResult, semio_framework::Fault> {
    use semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor as wit;
    Ok(wit::TurnResult {
        retained_receipt:kernel_retained_receipt_to_wit(result.retained_receipt),
        ui_patches: result.ui_patches.iter().map(kernel_ui_patch_to_wit).collect(),
        effects: result.effects.iter().cloned().map(kernel_effect_to_wit).collect::<Result<Vec<_>, _>>()?,
        presence: result.presence.iter().map(kernel_presence_update_to_wit).collect(),
        next_wake: result.next_wake,
        status: match &result.status {
            TurnStatus::Idle => wit::TurnStatus::Idle,
            TurnStatus::MoreWork => wit::TurnStatus::MoreWork,
            TurnStatus::CheckpointReady { checkpoint } => wit::TurnStatus::CheckpointReady(wit::JobCheckpoint { state: checkpoint.state.clone(), applied_progress: checkpoint.applied_progress }),
            TurnStatus::Faulted(bytes) => wit::TurnStatus::Faulted(bytes.clone()),
        },
        fuel_used: result.fuel_used,
        command_ingress: kernel_command_ingress_to_wit(result.command_ingress.clone()),
        cold_pair_ingress: kernel_cold_pair_ingress_to_wit(result.cold_pair_ingress.clone()),
        lifecycle_receipt: result.lifecycle_receipt.map(kernel_lifecycle_receipt_to_wit),
        ui_patch_receipt: result.ui_patch_receipt.map(kernel_patch_receipt_to_wit),
    })
}

/// 👥️ M2: pack-encodes a whole `ui_contract::PresenceUpdate` into the WIT `presence-update.update`
/// field — same `pack_patch_field` helper every `patch-op` variant already uses, since a
/// render-plane presence update is exactly as opaque to the WIT boundary as a patch op's payload.
fn kernel_presence_update_to_wit(update: &ui_contract::PresenceUpdate) -> semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor::PresenceUpdate {
    use semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor as wit;
    wit::PresenceUpdate { update: pack_patch_field(&update) }
}

fn kernel_ui_patch_to_wit(patch: &UiPatch) -> semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor::UiPatch {
    use semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor as wit;
    let (instance, surface) = patch.surface.0.split_once(':').unwrap_or(("0", patch.surface.0.as_str()));
    wit::UiPatch {
        surface: semio_framework_plugin_abi::wasip2::SurfaceRef { instance: instance.parse().unwrap_or(0), surface: surface.to_owned() },
        revision: patch.revision.0,
        base_revision: patch.base_revision.0,
        ops: patch.ops.iter().map(kernel_patch_op_to_wit).collect(),
    }
}

/// 🩹️ Packs any `Serialize` payload the same way `kernel_effect_to_wit`'s own `pack` helper does —
/// shared here rather than duplicated because every `patch-op` variant but `Remove`/`SetRoot` carries
/// exactly one `pack`-encoded field.
// 🚫️async: E5 executor bridge — `store::pack_rt::encode_wire_value` is genuinely `async fn` (out of
// this packet's `path_scope`, `🏪️store/**`); safe to resolve synchronously here for the same "world
// actor has no host-async import" reason as this file's other WIT-boundary bridges.
fn pack_patch_field<T: serde::Serialize>(value: &T) -> Vec<u8> {
    let value = serde_json::to_value(value).map_or(semio_framework_value::DslValue::Null, |json| semio_framework_value::DslValue::from(&json));
    store::pack_rt::encode_wire_value(&value)
}

/// 🩹️ Wire payload for `patch-set-activity`'s `activity: pack` field. `component.wit`'s
/// `patch-set-activity` (`wit-flip`, 26/08/20) carries only `activity: pack`, with no sibling
/// `disabled: bool`, while the contract crate's own `UiPatchOp::SetActivity` carries `disabled` as a
/// peer field (`🧬️contract/🦀️document.rs:148`) — a real WIT/contract mismatch, flagged rather than
/// silently resolved by `📓️terra-wit-flip-report.md`'s decisions section, and resolved HERE (the
/// first packet to actually encode this op) by folding `disabled` into the packed payload instead of
/// dropping it. A future packet editing `component.wit` may add a `disabled: bool` field there
/// instead, at which point this wrapper — and its symmetric decode on the host side — goes away.
/// 🌉️ `serde::Serialize` only — no `ToValue` derive: `pack_patch_field` bridges through
/// `serde_json` now (`ui_contract::Activity` has no `ToValue` of its own), so this payload
/// never needs one either.
#[doc(hidden)]
#[derive(serde::Serialize)]
pub struct ActivityPatchPayload<'a> {
    pub activity: &'a ui_contract::Activity,
    pub disabled: bool,
}

fn kernel_patch_op_to_wit(op: &UiPatchOp) -> semio_framework_plugin_abi::wasip2::PatchOp {
    match op {
        UiPatchOp::Upsert(record) => semio_framework_plugin_abi::wasip2::PatchOp::Upsert(semio_framework_plugin_abi::wasip2::PatchUpsert { node: pack_patch_field(&record) }),
        UiPatchOp::SetComponent { id, component } => semio_framework_plugin_abi::wasip2::PatchOp::SetComponent(semio_framework_plugin_abi::wasip2::PatchSetComponent { node: id.0, component: pack_patch_field(&component) }),
        UiPatchOp::SetLayout { id, layout } => semio_framework_plugin_abi::wasip2::PatchOp::SetLayout(semio_framework_plugin_abi::wasip2::PatchSetLayout { node: id.0, layout: pack_patch_field(&layout) }),
        UiPatchOp::SetActivity { id, activity, disabled } => semio_framework_plugin_abi::wasip2::PatchOp::SetActivity(semio_framework_plugin_abi::wasip2::PatchSetActivity { node: id.0, activity: pack_patch_field(&ActivityPatchPayload { activity, disabled: *disabled }) }),
        UiPatchOp::SetChildren { id, children } => semio_framework_plugin_abi::wasip2::PatchOp::SetChildren(semio_framework_plugin_abi::wasip2::PatchSetChildren { node: id.0, children: children.iter().map(|child| child.0).collect() }),
        UiPatchOp::SetStyle { id, style } => semio_framework_plugin_abi::wasip2::PatchOp::SetStyle(semio_framework_plugin_abi::wasip2::PatchSetStyle { node: id.0, style: pack_patch_field(&style) }),
        UiPatchOp::SetAccessibility { id, accessibility } => semio_framework_plugin_abi::wasip2::PatchOp::SetAccessibility(semio_framework_plugin_abi::wasip2::PatchSetAccessibility { node: id.0, accessibility: pack_patch_field(&accessibility) }),
        UiPatchOp::SetBindings { id, bindings } => semio_framework_plugin_abi::wasip2::PatchOp::SetBindings(semio_framework_plugin_abi::wasip2::PatchSetBindings { node: id.0, bindings: pack_patch_field(&bindings) }),
        UiPatchOp::SetMenu { id, menu } => semio_framework_plugin_abi::wasip2::PatchOp::SetMenu(semio_framework_plugin_abi::wasip2::PatchSetMenu { node: id.0, menu: pack_patch_field(&menu) }),
        UiPatchOp::Remove { id } => semio_framework_plugin_abi::wasip2::PatchOp::Remove(id.0),
        UiPatchOp::SetRoot { id } => semio_framework_plugin_abi::wasip2::PatchOp::SetRoot(id.0),
    }
}

/// 🔀️ kernel `Effect` → WIT `effect`. Field-for-field per `📓️design-abi.md` §2's table; complex
/// Rust-only field types (`WindowKindId`, `DslValue`, `MediaType`, `ClipboardFragment`, ...) are
/// wire-encoded through the SAME `store::pack_rt::encode_wire_value`/`semio_framework_value::ToValue::to_value` idiom
/// every existing host boundary in this crate already uses.
///
/// 🚫️async: E5 executor bridge — `store::pack_rt::encode_wire_value` is genuinely `async fn`
/// (out of this packet's `path_scope`, `🏪️store/**`), but every caller in this match below is
/// itself sync (R9: `kernel_effect_to_wit`'s only consumer is the WIT-fixed sync `world actor`
/// boundary, no suspension point of its own) — `resolve_ready` is safe here because `world
/// actor` imports no `host-async`, so this store call never has anything real to suspend on.
fn kernel_effect_to_wit(effect: Effect) -> Result<semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor::Effect, semio_framework::Fault> {
    use semio_framework_plugin_abi::wasip2::exports::semio::framework::reactor as wit;
    fn pack<T: serde::Serialize>(value: &T) -> Vec<u8> {
        let value = serde_json::to_value(value).map_or(semio_framework_value::DslValue::Null, |json| semio_framework_value::DslValue::from(&json));
        store::pack_rt::encode_wire_value(&value)
    }
    Ok(match effect {
        Effect::OpenWindow { req, kind, params } => wit::Effect::OpenWindow(semio_framework_plugin_abi::wasip2::OpenWindowEffect { req: req.0, params: semio_framework_plugin_abi::wasip2::OpenWindowParams { kind: kind.0, params: pack(&params) } }),
        Effect::CloseWindow { window } => wit::Effect::CloseWindow(semio_framework_plugin_abi::wasip2::CloseWindowEffect { window: window.0 as u64 }),
        Effect::Notify { message } => wit::Effect::Notify(semio_framework_plugin_abi::wasip2::NotifyEffect { message }),
        Effect::ClipboardWrite { fragment } => wit::Effect::ClipboardWrite(semio_framework_plugin_abi::wasip2::ClipboardWriteEffect { fragment: pack(&fragment) }),
        Effect::RequestSync => wit::Effect::RequestSync,
        Effect::Navigate { uri } => wit::Effect::Navigate(semio_framework_plugin_abi::wasip2::NavigateEffect { uri }),
        Effect::LoadDocument { pack: doc_pack, spr } => wit::Effect::LoadDocument(semio_framework_plugin_abi::wasip2::LoadDocumentEffect { doc_pack, spr }),
        Effect::OpenExternalUrl { url } => wit::Effect::OpenExternalUrl(semio_framework_plugin_abi::wasip2::OpenExternalUrlEffect { url }),
        Effect::SetPanel { panel_json } => wit::Effect::SetPanel(semio_framework_plugin_abi::wasip2::SetPanelEffect { panel_json }),
        Effect::DownloadMediaExport { filename, mime_type, data, encoding } => wit::Effect::DownloadMediaExport(semio_framework_plugin_abi::wasip2::DownloadMediaExportEffect { filename, mime_type, data, encoding }),
        Effect::IconRenderExport { items } => wit::Effect::IconRenderExport(semio_framework_plugin_abi::wasip2::IconRenderExportEffect { items: pack(&items) }),
        Effect::VideoRenderExport { filename, program } => wit::Effect::VideoRenderExport(semio_framework_plugin_abi::wasip2::VideoRenderExportEffect { filename, program: pack(&program) }),
        Effect::RequestFileOpen { req, accept, read_as, import_action, args, multiple } => {
            wit::Effect::RequestFileOpen(semio_framework_plugin_abi::wasip2::RequestFileOpenEffect { req: req.0, params: semio_framework_plugin_abi::wasip2::RequestFileOpenParams { accept, read_as, multiple, import_action, args: args.map(|value| pack(&value)) } })
        }
        Effect::RequestMediaFrames { req, accept, frame_action, done_action, fallback_action, sample_stride, max_frames, max_long_edge_px, fps_hint, payload, args } => wit::Effect::RequestMediaFrames(semio_framework_plugin_abi::wasip2::RequestMediaFramesEffect {
            req: req.0,
            params: semio_framework_plugin_abi::wasip2::RequestMediaFramesParams { accept, frame_action, done_action, fallback_action, sample_stride, max_frames, max_long_edge_px, fps_hint, payload, args: args.map(|value| pack(&value)) },
        }),
        Effect::SpawnPluginInstance { req, plugin_id, app_id, os_instance_id, label, document_json } => {
            wit::Effect::SpawnPluginInstance(semio_framework_plugin_abi::wasip2::SpawnPluginInstanceEffect { req: req.0, params: semio_framework_plugin_abi::wasip2::SpawnPluginInstanceParams { plugin_id, app_id, os_instance_id, label, artifact_json: document_json } })
        }
        Effect::OpenPluginInstance { plugin_id, app_id, os_instance_id } => wit::Effect::OpenPluginInstance(semio_framework_plugin_abi::wasip2::OpenPluginInstanceEffect { plugin_id, app_id, os_instance_id }),
        Effect::SetActiveUtility { window_id, utility_id } => wit::Effect::SetActiveUtility(semio_framework_plugin_abi::wasip2::SetActiveUtilityEffect { window_id, utility_id }),
        Effect::SetActiveTool { tool_id } => wit::Effect::SetActiveTool(semio_framework_plugin_abi::wasip2::SetActiveToolEffect { tool_id }),
        Effect::OpenDialog { req, dialog_id, args } => wit::Effect::OpenDialog(semio_framework_plugin_abi::wasip2::OpenDialogEffect { req: req.0, params: semio_framework_plugin_abi::wasip2::OpenDialogParams { dialog_id, args: args.map(|value| pack(&value)) } }),
        Effect::DispatchAction { req, action, args, delay_ms } => wit::Effect::DispatchAction(semio_framework_plugin_abi::wasip2::DispatchActionEffect { req: req.0, params: semio_framework_plugin_abi::wasip2::DispatchActionParams { action, args: args.map(|value| pack(&value)), delay_ms } }),
        Effect::ReplayShellCommand { action_id, args } => wit::Effect::ReplayShellCommand(semio_framework_plugin_abi::wasip2::ReplayShellCommandEffect { action_id, args: args.map(|value| pack(&value)) }),
        Effect::InvokeExtension { req, extension_id, capability, request_json, .. } => {
            wit::Effect::InvokeExtension(semio_framework_plugin_abi::wasip2::InvokeExtensionEffect { req: req.0, params: semio_framework_plugin_abi::wasip2::InvokeExtensionParams { extension_id, capability, payload: request_json.into_bytes() } })
        }
        Effect::SendMessage { target, payload } => wit::Effect::SendMessage(semio_framework_plugin_abi::wasip2::SendMessageEffect { target: kernel_endpoint_to_wit(target), payload }),
        Effect::PublishEvent { topic, payload } => wit::Effect::PublishEvent(semio_framework_plugin_abi::wasip2::PublishEventEffect { topic, payload }),
        Effect::BlobWrite { req, media_type, bytes } => wit::Effect::BlobWrite(semio_framework_plugin_abi::wasip2::BlobWriteEffect { req: req.0, params: semio_framework_plugin_abi::wasip2::BlobWriteParams { media_type: pack(&media_type), bytes } }),
        Effect::BlobLoad { req, hash } => wit::Effect::BlobLoad(semio_framework_plugin_abi::wasip2::BlobLoadEffect { req: req.0, params: semio_framework_plugin_abi::wasip2::BlobLoadParams { hash } }),
        Effect::HttpRequest { req, method, url, headers, body, stream } => wit::Effect::HttpRequest(semio_framework_plugin_abi::wasip2::HttpRequestEffect { req: req.0, params: semio_framework_plugin_abi::wasip2::HttpParams { method, url, headers, body, streaming: stream } }),
        Effect::DocumentRead { req, doc, lane } => wit::Effect::ArtifactRead(semio_framework_plugin_abi::wasip2::ArtifactReadEffect { req: req.0, params: semio_framework_plugin_abi::wasip2::ArtifactReadParams { doc: doc.0 as u64, lane } }),
        Effect::DocumentWrite { req, doc, lane, ops } => wit::Effect::ArtifactWrite(semio_framework_plugin_abi::wasip2::ArtifactWriteEffect { req: req.0, params: semio_framework_plugin_abi::wasip2::ArtifactWriteParams { doc: doc.0 as u64, lane, ops } }),
        Effect::LinkResolve { req, link } => wit::Effect::LinkResolve(semio_framework_plugin_abi::wasip2::LinkResolveEffect { req: req.0, link: link.into_bytes() }),
        Effect::RegistryQuery { req, kind, filter } => wit::Effect::RegistryQuery(semio_framework_plugin_abi::wasip2::RegistryQueryEffect { req: req.0, params: semio_framework_plugin_abi::wasip2::RegistryQueryParams { kind, filter: filter.map(|value| pack(&value)).unwrap_or_default() } }),
        Effect::IoCompose { req, key, sources } => wit::Effect::IoCompose(semio_framework_plugin_abi::wasip2::IoComposeEffect { req: req.0, params: semio_framework_plugin_abi::wasip2::IoComposeParams { key: key.into_bytes(), sources: pack(&sources) } }),
        Effect::CacheDerive { req, engine_id, input } => wit::Effect::CacheDerive(semio_framework_plugin_abi::wasip2::CacheDeriveEffect { req: req.0, params: semio_framework_plugin_abi::wasip2::CacheDeriveParams { engine_id, input } }),
        Effect::CacheRead { req, engine_id, key } => wit::Effect::CacheRead(semio_framework_plugin_abi::wasip2::CacheReadEffect { req: req.0, params: semio_framework_plugin_abi::wasip2::CacheReadParams { engine_id, key: key.into_bytes() } }),
        Effect::SetTimer { id, after_ms, repeat } => {
            if !ARMED_TIMERS.with(|timers| timers.borrow().contains(id)) {
                return Err(semio_framework::Fault::new(
                    semio_framework::FaultOrigin::Framework,
                    semio_framework::FaultCode::new("plugin.timer-owner-missing"),
                    "timer effect reached the WIT boundary without exact pre-admitted instance ownership",
                ));
            }
            wit::Effect::SetTimer(semio_framework_plugin_abi::wasip2::SetTimerEffect { id, after_ms: after_ms as u32, repeat })
        }
        Effect::SpawnJob { job, kind, input, placement } => wit::Effect::SpawnJob(semio_framework_plugin_abi::wasip2::SpawnJobEffect { job, kind, input, placement: kernel_placement_to_wit(placement) }),
        Effect::CancelJob { job } => wit::Effect::CancelJob(semio_framework_plugin_abi::wasip2::CancelJobEffect { job }),
        Effect::Respond { req, result } => wit::Effect::Respond(semio_framework_plugin_abi::wasip2::RespondEffect { req: req.0, outcome: kernel_outcome_to_wit_respond(result) }),
        Effect::StorageRead { req, key } => wit::Effect::StorageRead(semio_framework_plugin_abi::wasip2::StorageReadEffect { req: req.0, params: semio_framework_plugin_abi::wasip2::StorageReadParams { key } }),
        Effect::StorageWrite { req, key, bytes } => wit::Effect::StorageWrite(semio_framework_plugin_abi::wasip2::StorageWriteEffect { req: req.0, params: semio_framework_plugin_abi::wasip2::StorageWriteParams { key, value: bytes } }),
        Effect::StorageDelete { req, key } => wit::Effect::StorageDelete(semio_framework_plugin_abi::wasip2::StorageDeleteEffect { req: req.0, params: semio_framework_plugin_abi::wasip2::StorageDeleteParams { key } }),
        Effect::RequestCapability { req, capability } => wit::Effect::RequestCapability(semio_framework_plugin_abi::wasip2::RequestCapabilityEffect {
            req: req.0,
            params: semio_framework_plugin_abi::wasip2::RequestCapabilityParams { id: capability.id.0, scope: capability.scope, reason: capability.reason, optional: capability.optional },
        }),
        Effect::ReleaseCapability { id } => wit::Effect::ReleaseCapability(semio_framework_plugin_abi::wasip2::ReleaseCapabilityEffect { id: id.0 }),
        Effect::Subscribe { topic } => wit::Effect::Subscribe(semio_framework_plugin_abi::wasip2::SubscribeEffect { topic }),
        Effect::Unsubscribe { topic } => wit::Effect::Unsubscribe(semio_framework_plugin_abi::wasip2::SubscribeEffect { topic }),
        Effect::RequestServiceOperation { owner, service_id, action, payload } => {
            wit::Effect::RequestServiceOperation(semio_framework_plugin_abi::wasip2::RequestServiceOperationEffect { owner, service_id, action, payload: store::pack_rt::encode_wire_value(&payload) })
        }
    })
}

fn kernel_endpoint_to_wit(endpoint: MessageEndpoint) -> semio_framework_plugin_abi::wasip2::MessageEndpoint {
    match endpoint {
        MessageEndpoint::Shell { instance } => semio_framework_plugin_abi::wasip2::MessageEndpoint::Shell(instance.0.parse().unwrap_or(0)),
        MessageEndpoint::Backbone { uri } => semio_framework_plugin_abi::wasip2::MessageEndpoint::Backbone(uri),
        MessageEndpoint::PluginInstance { id } => semio_framework_plugin_abi::wasip2::MessageEndpoint::PluginInstance(id.0.parse().unwrap_or(0)),
        MessageEndpoint::Extension { id } => semio_framework_plugin_abi::wasip2::MessageEndpoint::Extension(id),
        MessageEndpoint::Topic { name } => semio_framework_plugin_abi::wasip2::MessageEndpoint::Topic(name),
    }
}

fn kernel_placement_to_wit(placement: semio_framework::kernel::JobPlacement) -> semio_framework_plugin_abi::wasip2::JobPlacement {
    match placement {
        semio_framework::kernel::JobPlacement::Inline => semio_framework_plugin_abi::wasip2::JobPlacement::Inline,
        semio_framework::kernel::JobPlacement::Isolated => semio_framework_plugin_abi::wasip2::JobPlacement::Isolated,
        semio_framework::kernel::JobPlacement::Exclusive => semio_framework_plugin_abi::wasip2::JobPlacement::Exclusive,
    }
}

fn kernel_outcome_to_wit_respond(result: RequestOutcome) -> semio_framework_plugin_abi::wasip2::RespondResult {
    match result {
        RequestOutcome::Ok(bytes) => semio_framework_plugin_abi::wasip2::RespondResult::Ok(bytes),
        RequestOutcome::Err(bytes) => semio_framework_plugin_abi::wasip2::RespondResult::Fault(bytes),
    }
}
