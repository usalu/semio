//! 🔀️ Conversions between the kernel vocabulary and the generated WIT types of the actor world.

use semio_framework::kernel::{ArtifactHandle, BrokerCapabilityGrant, CapabilityId, CapabilityRequest, Effect, Event, JobPlacement, MessageEndpoint, RequestId, RequestOutcome, TurnStatus, WindowHandle, WindowKindId};
use semio_framework::DslValue;
use semio_framework_plugin_guest_contract::{PluginHostError, TurnFault, sqlite_wire};
use crate::cold_pair::kernel_cold_page_to_wit;
use crate::actor_bindings;
use crate::wit_capabilities;
use crate::wit_effects;
use crate::wit_events;
use crate::wit_lifetime;
use crate::wit_reactor;
use crate::wit_types;
use crate::wit_ui;

//#region 🎭️GuestRuntime
//#region 🐎️WasmtimeRuntime
/// 🏷️ Restores the actual codec ABI's complete closed provider authority.
pub fn wit_snapshot_retirement(value:crate::actor_bindings::exports::semio::framework::codec::SnapshotRetirement)->sqlite_wire::SnapshotRetirement {
 sqlite_wire::SnapshotRetirement{ticket:value.ticket,owned_bytes:value.owned_bytes,refusal:value.refusal,native_pending:value.native_pending,output_pending:value.output_pending,terminal:value.terminal,demands:sqlite_wire::SnapshotDemands{items:value.demands.items,copy_bytes:value.demands.copy_bytes,capacity_bytes:value.demands.capacity_bytes,release_bytes:value.demands.release_bytes,depth:value.demands.depth}}
}

pub fn wit_snapshot_rejection(value: crate::actor_bindings::exports::semio::framework::codec::SnapshotRejection) -> sqlite_wire::SnapshotRejection {
    use actor_bindings::exports::semio::framework::codec::ValueRefusalKind as Wire;
    use semio_framework_value::ValueRefusalKind as Native;
    let kind = match value.kind { Wire::InvalidValue => Native::InvalidValue, Wire::Canceled => Native::Canceled, Wire::OwnershipLimit => Native::OwnershipLimit, Wire::AllocationFailed => Native::AllocationFailed, Wire::WorkLimit => Native::WorkLimit, Wire::DepthLimit => Native::DepthLimit, Wire::UnsupportedOwner => Native::UnsupportedOwner, Wire::InvariantViolated => Native::InvariantViolated };
    sqlite_wire::SnapshotRejection { kind, message: value.message, diagnostics: value.diagnostics }
}

pub fn wit_snapshot_limits(value: semio_framework::sqlite_snapshot::SqliteDatabaseLimits) -> crate::actor_bindings::exports::semio::framework::codec::SnapshotLimits {
    let value = sqlite_wire::SnapshotLimits::from(value);
    crate::actor_bindings::exports::semio::framework::codec::SnapshotLimits { max_file_bytes: value.max_file_bytes, max_value_bytes: value.max_value_bytes, max_allocation_bytes: value.max_allocation_bytes, max_schema_bytes: value.max_schema_bytes, max_rows: value.max_rows, max_columns: value.max_columns, max_tables: value.max_tables, max_pages: value.max_pages }
}

pub fn wit_snapshot_grant(value:semio_framework_value::RetainedCloneGrant)->crate::actor_bindings::semio::framework::types::RetainedCloneGrant{
    let value=sqlite_wire::SnapshotGrant::from(value);
    crate::actor_bindings::semio::framework::types::RetainedCloneGrant{maximum_items:value.maximum_items,maximum_copy_bytes:value.maximum_copy_bytes,maximum_capacity_bytes:value.maximum_capacity_bytes,maximum_release_bytes:value.maximum_release_bytes,maximum_depth:value.maximum_depth}
}
//#region 🔀️EffectEventMarshal
/// 🌉️ `wit_effect_to_kernel`/`kernel_event_to_wit` are the host-side half of `design-abi.md` §2's
/// "WIT variants mirror [`Effect`/`Event`] field-for-field; the guest SDK glue converts between
/// them" — this is that conversion, mirrored for the host. It is NOT fully field-for-field in
/// practice: several real shape gaps between `📜️wit/*.wit` (packet A2) and `🎠️kernel/🦀️.rs`
/// (packet A3) surfaced while writing this and are called out inline + in
/// `📓️terra-B1-host-native-report.md`'s `## blocked-on` (most notably: `📜️wit/📜️effects.wit`'s
/// `io-run` effect has no `Effect::IoRun` counterpart yet).
pub(super) async fn decode_dsl(bytes: &[u8]) -> Option<DslValue> {
    if bytes.is_empty() {
        return None;
    }
    store::pack_rt::decode_wire_value(bytes).ok()
}

pub(super) async fn decode_json<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Option<T> {
    if bytes.is_empty() {
        return None;
    }
    serde_json::from_slice(bytes).ok()
}

async fn encode_json<T: serde::Serialize>(value: &T) -> Vec<u8> {
    serde_json::to_vec(value).unwrap_or_default()
}

pub async fn wit_message_endpoint_to_kernel(endpoint: wit_types::MessageEndpoint) -> MessageEndpoint {
    match endpoint {
        wit_types::MessageEndpoint::Shell(instance) => MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) },
        wit_types::MessageEndpoint::Backbone(uri) => MessageEndpoint::Backbone { uri },
        wit_types::MessageEndpoint::PluginInstance(instance) => MessageEndpoint::PluginInstance { id: semio_framework::kernel::PluginInstanceId(instance.to_string()) },
        wit_types::MessageEndpoint::Extension(id) => MessageEndpoint::Extension { id },
        wit_types::MessageEndpoint::Topic(name) => MessageEndpoint::Topic { name },
    }
}

pub async fn kernel_message_endpoint_to_wit(endpoint: &MessageEndpoint) -> wit_types::MessageEndpoint {
    match endpoint {
        MessageEndpoint::Shell { instance } => wit_types::MessageEndpoint::Shell(instance.0.parse().unwrap_or(0)),
        MessageEndpoint::Backbone { uri } => wit_types::MessageEndpoint::Backbone(uri.clone()),
        MessageEndpoint::PluginInstance { id } => wit_types::MessageEndpoint::PluginInstance(id.0.parse().unwrap_or(0)),
        MessageEndpoint::Extension { id } => wit_types::MessageEndpoint::Extension(id.clone()),
        MessageEndpoint::Topic { name } => wit_types::MessageEndpoint::Topic(name.clone()),
    }
}

async fn kernel_request_outcome_to_wit(result: &RequestOutcome) -> wit_events::CompletionResult {
    match result {
        RequestOutcome::Ok(bytes) => wit_events::CompletionResult::Ok(bytes.clone()),
        RequestOutcome::Err(bytes) => wit_events::CompletionResult::Fault(bytes.clone()),
    }
}

pub async fn wit_turn_status_to_kernel(status: wit_reactor::TurnStatus) -> TurnStatus {
    match status {
        wit_reactor::TurnStatus::Idle => TurnStatus::Idle,
        wit_reactor::TurnStatus::MoreWork => TurnStatus::MoreWork,
        wit_reactor::TurnStatus::CheckpointReady(checkpoint) => TurnStatus::CheckpointReady { checkpoint: semio_framework_actor::JobCheckpoint { state: checkpoint.state, applied_progress: checkpoint.applied_progress } },
        wit_reactor::TurnStatus::Faulted(bytes) => TurnStatus::Faulted(bytes),
    }
}

/// 📄️ One staged command page as `reactor.stage-command-page` takes it — the cursor and the page's
/// live bytes, never the fixed 4 KiB tail. See that function's WIT doc for why the page left
/// `poll`'s parameter list.
fn kernel_command_page_to_wit(cursor: &semio_framework::kernel::CommandPageCursor, page: &semio_framework::kernel::FixedCommandPage) -> (wit_reactor::CommandPageCursor, Vec<u8>) {
    (kernel_command_cursor_to_wit(cursor), page.as_slice().to_vec())
}

fn wit_command_cursor_to_kernel(cursor: wit_reactor::CommandPageCursor) -> semio_framework::kernel::CommandPageCursor {
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
    }
}

fn kernel_command_cursor_to_wit(cursor: &semio_framework::kernel::CommandPageCursor) -> wit_reactor::CommandPageCursor {
    wit_reactor::CommandPageCursor {
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

pub fn wit_command_ingress_to_kernel(status: wit_reactor::CommandIngressStatus) -> semio_framework::kernel::CommandIngressStatus {
    let cursor = wit_command_cursor_to_kernel(status.cursor);
    match status.kind {
        0 => semio_framework::kernel::CommandIngressStatus::Idle,
        1 => semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor),
        2 => semio_framework::kernel::CommandIngressStatus::Backpressure(cursor),
        3 => semio_framework::kernel::CommandIngressStatus::CommandPending(cursor),
        4 => semio_framework::kernel::CommandIngressStatus::CommandComplete(cursor),
        5 => semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: status.fault },
        _ => semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-ingress-kind".to_vec() },
    }
}

pub async fn wit_effect_to_kernel(effect: wit_effects::Effect) -> Result<Effect, PluginHostError> {
    use wit_effects::Effect as E;
    Ok(match effect {
        E::SendMessage(inner) => Effect::SendMessage { target: kernel_message_endpoint_to_wit_reverse(inner.target).await, payload: inner.payload },
        E::PublishEvent(inner) => Effect::PublishEvent { topic: inner.topic, payload: inner.payload },
        E::BlobLoad(inner) => Effect::BlobLoad { req: RequestId(inner.req), hash: inner.params.hash },
        E::BlobWrite(inner) => Effect::BlobWrite {
            req: RequestId(inner.req),
            media_type: decode_json(&inner.params.media_type).await.unwrap_or(semio_framework::MediaType { class: semio_framework::MediaClass::Data, form: semio_framework::MediaForm::Value }),
            bytes: inner.params.bytes,
        },
        E::HttpRequest(inner) => Effect::HttpRequest { req: RequestId(inner.req), method: inner.params.method, url: inner.params.url, headers: inner.params.headers, body: inner.params.body, stream: inner.params.streaming },
        E::ArtifactRead(inner) => Effect::DocumentRead { req: RequestId(inner.req), doc: ArtifactHandle(inner.params.doc as u128), lane: inner.params.lane },
        E::ArtifactWrite(inner) => Effect::DocumentWrite { req: RequestId(inner.req), doc: ArtifactHandle(inner.params.doc as u128), lane: inner.params.lane, ops: inner.params.ops },
        E::LinkResolve(inner) => Effect::LinkResolve { req: RequestId(inner.req), link: String::from_utf8_lossy(&inner.link).into_owned() },
        E::RegistryQuery(inner) => Effect::RegistryQuery { req: RequestId(inner.req), kind: inner.params.kind, filter: decode_dsl(&inner.params.filter).await },
        E::IoCompose(inner) => Effect::IoCompose { req: RequestId(inner.req), key: String::from_utf8_lossy(&inner.params.key).into_owned(), sources: decode_json(&inner.params.sources).await.unwrap_or_default() },
        E::IoRun(_inner) => return Err(PluginHostError::Plugin("effect io-run has no semio_framework::kernel::Effect variant yet (needs A3 to add Effect::IoRun) — see 📓️terra-B1-host-native-report.md".to_string())),
        E::CacheDerive(inner) => Effect::CacheDerive { req: RequestId(inner.req), engine_id: inner.params.engine_id, input: inner.params.input },
        E::CacheRead(inner) => Effect::CacheRead { req: RequestId(inner.req), engine_id: inner.params.engine_id, key: String::from_utf8_lossy(&inner.params.key).into_owned() },
        E::OpenWindow(inner) => Effect::OpenWindow { req: RequestId(inner.req), kind: WindowKindId(inner.params.kind), params: decode_dsl(&inner.params.params).await.unwrap_or(DslValue::Null) },
        E::CloseWindow(inner) => Effect::CloseWindow { window: WindowHandle(inner.window as u128) },
        E::DispatchAction(inner) => {
            let args = match inner.params.args {
                Some(bytes) => decode_dsl(&bytes).await,
                None => None,
            };
            Effect::DispatchAction { req: RequestId(inner.req), action: inner.params.action, args, delay_ms: inner.params.delay_ms }
        }
        E::InvokeExtension(inner) => Effect::invoke_extension(RequestId(inner.req), inner.params.extension_id, inner.params.capability, String::from_utf8_lossy(&inner.params.payload).into_owned()),
        E::Notify(inner) => Effect::Notify { message: inner.message },
        E::ClipboardWrite(inner) => Effect::ClipboardWrite { fragment: decode_json(&inner.fragment).await.ok_or_else(|| PluginHostError::Plugin("clipboard-write-effect.fragment failed to decode as JSON ClipboardFragment".to_string()))? },
        E::Navigate(inner) => Effect::Navigate { uri: inner.uri },
        E::OpenExternalUrl(inner) => Effect::OpenExternalUrl { url: inner.url },
        E::SetPanel(inner) => Effect::SetPanel { panel_json: inner.panel_json },
        E::SetActiveUtility(inner) => Effect::SetActiveUtility { window_id: inner.window_id, utility_id: inner.utility_id },
        E::SetActiveTool(inner) => Effect::SetActiveTool { tool_id: inner.tool_id },
        E::ReplayShellCommand(inner) => {
            let args = match inner.args {
                Some(bytes) => decode_dsl(&bytes).await,
                None => None,
            };
            Effect::ReplayShellCommand { action_id: inner.action_id, args }
        }
        E::SpawnPluginInstance(inner) => {
            Effect::SpawnPluginInstance { req: RequestId(inner.req), plugin_id: inner.params.plugin_id, app_id: inner.params.app_id, os_instance_id: inner.params.os_instance_id, label: inner.params.label, document_json: inner.params.artifact_json }
        }
        E::OpenPluginInstance(inner) => Effect::OpenPluginInstance { plugin_id: inner.plugin_id, app_id: inner.app_id, os_instance_id: inner.os_instance_id },
        E::OpenDialog(inner) => {
            let args = match inner.params.args {
                Some(bytes) => decode_dsl(&bytes).await,
                None => None,
            };
            Effect::OpenDialog { req: RequestId(inner.req), dialog_id: inner.params.dialog_id, args }
        }
        E::IconRenderExport(inner) => Effect::IconRenderExport { items: decode_json(&inner.items).await.unwrap_or_default() },
        E::VideoRenderExport(inner) => Effect::VideoRenderExport { filename: inner.filename, program: decode_dsl(&inner.program).await.and_then(|value| semio_framework_value::FromValue::from_value(value).ok()).unwrap_or_default() },
        E::DownloadMediaExport(inner) => Effect::DownloadMediaExport { filename: inner.filename, mime_type: inner.mime_type, data: inner.data, encoding: inner.encoding },
        E::RequestFileOpen(inner) => {
            let args = match inner.params.args { Some(bytes) => decode_dsl(&bytes).await, None => None };
            Effect::RequestFileOpen { req: RequestId(inner.req), accept: inner.params.accept, read_as: inner.params.read_as, import_action: inner.params.import_action, multiple: inner.params.multiple, args }
        },
        E::RequestMediaFrames(inner) => {
            let args = match inner.params.args {
                Some(bytes) => decode_dsl(&bytes).await,
                None => None,
            };
            Effect::RequestMediaFrames {
                req: RequestId(inner.req),
                accept: inner.params.accept,
                frame_action: String::new(),
                done_action: String::new(),
                fallback_action: String::new(),
                sample_stride: inner.params.sample_stride,
                max_frames: inner.params.max_frames,
                max_long_edge_px: inner.params.max_long_edge_px,
                fps_hint: inner.params.fps_hint,
                payload: inner.params.payload,
                args,
            }
        }
        E::LoadDocument(inner) => Effect::LoadDocument { pack: inner.doc_pack, spr: inner.spr },
        E::RequestSync => Effect::RequestSync,
        E::SetTimer(inner) => Effect::SetTimer { id: inner.id, after_ms: inner.after_ms as u64, repeat: inner.repeat },
        E::SpawnJob(inner) => Effect::SpawnJob {
            job: inner.job,
            kind: inner.kind,
            input: inner.input,
            placement: match inner.placement {
                wit_effects::JobPlacement::Inline => JobPlacement::Inline,
                wit_effects::JobPlacement::Isolated => JobPlacement::Isolated,
                wit_effects::JobPlacement::Exclusive => JobPlacement::Exclusive,
            },
        },
        E::CancelJob(inner) => Effect::CancelJob { job: inner.job },
        E::Respond(inner) => Effect::Respond {
            req: RequestId(inner.req),
            result: match inner.outcome {
                wit_effects::RespondResult::Ok(bytes) => RequestOutcome::Ok(bytes),
                wit_effects::RespondResult::Fault(bytes) => RequestOutcome::Err(bytes),
            },
        },
        E::StorageRead(inner) => Effect::StorageRead { req: RequestId(inner.req), key: inner.params.key },
        E::StorageWrite(inner) => Effect::StorageWrite { req: RequestId(inner.req), key: inner.params.key, bytes: inner.params.value },
        E::StorageDelete(inner) => Effect::StorageDelete { req: RequestId(inner.req), key: inner.params.key },
        E::RequestCapability(inner) => {
            Effect::RequestCapability { req: RequestId(inner.req), capability: CapabilityRequest { id: CapabilityId(inner.params.id), scope: inner.params.scope, reason: inner.params.reason, optional: inner.params.optional } }
        }
        E::ReleaseCapability(inner) => Effect::ReleaseCapability { id: CapabilityId(inner.id) },
        E::Subscribe(inner) => Effect::Subscribe { topic: inner.topic },
        E::Unsubscribe(inner) => Effect::Unsubscribe { topic: inner.topic },
        E::RequestServiceOperation(inner) => Effect::RequestServiceOperation {
            owner: inner.owner,
            service_id: inner.service_id,
            action: inner.action,
            payload: decode_dsl(&inner.payload).await.ok_or_else(|| PluginHostError::Plugin("Invalid service operation payload".into()))?,
        },
    })
}

/// 🐛️ `SendMessageEffect.target` is `wit_types::MessageEndpoint` (from the `types` interface via
/// `use types.{message-endpoint}` in `effects.wit`) — same generated Rust type as
/// `wit_message_endpoint_to_kernel` above takes, just named to keep the giant match arm list above
/// readable.
async fn kernel_message_endpoint_to_wit_reverse(endpoint: wit_types::MessageEndpoint) -> MessageEndpoint {
    wit_message_endpoint_to_kernel(endpoint).await
}

async fn wit_surface_ref(instance_id: u32, surface: &str) -> wit_ui::SurfaceRef {
    let body_key = surface.split_once(':').map_or(surface, |(_, body_key)| body_key);
    wit_ui::SurfaceRef { instance: instance_id, surface: body_key.to_owned() }
}

/// 🏁️ Host → guest: `semio_framework::kernel::Event` to WIT `event` (`📜️wit/📜️events.wit`).
/// `instance_id` fills the WIT `instance` field several kernel lifecycle variants dropped (see
/// `WasmtimeInstanceState::instance_id`'s docstring).
pub(crate) fn kernel_lifetime_to_wit(value: semio_framework::kernel::ActorInstanceLifetime) -> wit_lifetime::Lifetime {
    wit_lifetime::Lifetime { activation_generation: value.activation_generation, instance_id: value.instance_id, guest_lifetime: value.guest_lifetime }
}

pub(crate) fn wit_lifetime_to_kernel(value: wit_lifetime::Lifetime) -> semio_framework::kernel::ActorInstanceLifetime {
    semio_framework::kernel::ActorInstanceLifetime { activation_generation: value.activation_generation, instance_id: value.instance_id, guest_lifetime: value.guest_lifetime }
}

fn kernel_patch_receipt_to_wit(value: semio_framework::kernel::ActorUiPatchReceipt) -> wit_lifetime::UiPatchReceipt {
    wit_lifetime::UiPatchReceipt { lifetime: kernel_lifetime_to_wit(value.lifetime), patch_sequence: value.patch_sequence }
}

pub fn wit_patch_receipt_to_kernel(value: wit_lifetime::UiPatchReceipt) -> semio_framework::kernel::ActorUiPatchReceipt {
    semio_framework::kernel::ActorUiPatchReceipt { lifetime: wit_lifetime_to_kernel(value.lifetime), patch_sequence: value.patch_sequence }
}

fn kernel_lifecycle_receipt_to_wit(value: semio_framework::kernel::ActorInstanceLifecycleReceipt) -> wit_lifetime::Receipt {
    use semio_framework::kernel::ActorInstanceLifecycleReceipt as R;
    match value {
        R::Captured { lifetime, request_sequence } => wit_lifetime::Receipt::Captured(wit_lifetime::CapturedReceipt { lifetime: kernel_lifetime_to_wit(lifetime), request_sequence }),
        R::Accepted { lifetime, request_sequence, close_generation } => wit_lifetime::Receipt::Accepted(wit_lifetime::CloseReceipt { lifetime: kernel_lifetime_to_wit(lifetime), request_sequence, close_generation }),
        R::Retired { lifetime, request_sequence, close_generation } => wit_lifetime::Receipt::Retired(wit_lifetime::CloseReceipt { lifetime: kernel_lifetime_to_wit(lifetime), request_sequence, close_generation }),
    }
}

pub fn wit_lifecycle_receipt_to_kernel(value: wit_lifetime::Receipt) -> semio_framework::kernel::ActorInstanceLifecycleReceipt {
    use semio_framework::kernel::ActorInstanceLifecycleReceipt as R;
    match value {
        wit_lifetime::Receipt::Captured(value) => R::Captured { lifetime: wit_lifetime_to_kernel(value.lifetime), request_sequence: value.request_sequence },
        wit_lifetime::Receipt::Accepted(value) => R::Accepted { lifetime: wit_lifetime_to_kernel(value.lifetime), request_sequence: value.request_sequence, close_generation: value.close_generation },
        wit_lifetime::Receipt::Retired(value) => R::Retired { lifetime: wit_lifetime_to_kernel(value.lifetime), request_sequence: value.request_sequence, close_generation: value.close_generation },
    }
}

pub async fn kernel_turn_inputs_to_wit(events: &[Event], instance_id: u32) -> Result<(Vec<wit_events::Event>, Option<(wit_reactor::CommandPageCursor, Vec<u8>)>, Option<wit_reactor::ColdDocumentPairPage>), TurnFault> {
    let mut ordinary = Vec::with_capacity(events.len());
    let mut command = None;
    let mut cold = None;
    for event in events {
        match event {
            Event::CommandIngressPage { cursor, bytes } => {
                if command.is_some() || bytes.len() > semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES || (bytes.is_empty() && !(cursor.kind == 28 && cursor.item_count == 0)) {
                    return Err(TurnFault::Trapped("turn carries duplicate or invalid command page".into()));
                }
                command = Some(kernel_command_page_to_wit(cursor, bytes));
            }
            Event::ColdDocumentPairPage(page) => {
                if cold.is_some() {
                    return Err(TurnFault::Trapped("turn carries duplicate cold-pair page".into()));
                }
                cold = Some(kernel_cold_page_to_wit(page)?);
            }
            event => ordinary.push(kernel_event_to_wit(event, instance_id).await),
        }
    }
    Ok((ordinary, command, cold))
}

/// 🚫️async: R10 residue shape 1 — `kernel_broker_grant_to_wit` is async, hoisted out of
/// the sync `Iterator::map` via a plain loop.
///
/// 🐛️ WIT `request-event.from` was renamed `origin` — `from` is WIT-reserved (the SAME
/// reserved-keyword class B1's report already fixed for `stream`/`result`; this one was
/// fixed by A2 between B1's last pass and now, per the packet brief's "guest side is green").
pub async fn kernel_event_to_wit(event: &Event, instance_id: u32) -> wit_events::Event {
    match event {
        Event::InstanceOpen { request, app_id, actor, config, assets, capabilities, quotas } => {
            let mut wit_capabilities = Vec::with_capacity(capabilities.len());
            for capability in capabilities {
                wit_capabilities.push(kernel_broker_grant_to_wit(capability).await);
            }
            wit_events::Event::InstanceOpen(wit_events::InstanceOpenEvent {
                instance: request.instance_id,
                activation_generation: request.activation_generation,
                request_sequence: request.request_sequence,
                app_id: app_id.0.clone(),
                actor: actor.clone(),
                config: config.clone(),
                assets: assets.clone(),
                capabilities: wit_capabilities,
                quotas: encode_json(quotas).await,
            })
        }
        Event::InstanceClose(request) => wit_events::Event::InstanceClose(wit_lifetime::CloseRequest { lifetime: kernel_lifetime_to_wit(request.lifetime), request_sequence: request.request_sequence }),
        Event::InstanceLifecycleAck(ack) => wit_events::Event::InstanceLifecycleAck(kernel_lifecycle_receipt_to_wit(ack.receipt)),
        Event::Activate { reason } => wit_events::Event::Activate(wit_events::ActivateEvent { instance: instance_id, reason: kernel_activation_event_to_wit(reason).await }),
        Event::SuspendRequest => wit_events::Event::SuspendRequest(wit_events::SuspendRequestEvent { instance: instance_id }),
        Event::CapabilityChanged { change } => wit_events::Event::CapabilityChanged(wit_events::CapabilityChangedEvent { instance: instance_id, change: kernel_capability_change_to_wit(change).await }),
        Event::QuotaChanged { quotas } => wit_events::Event::QuotaChanged(wit_events::QuotaChangedEvent { instance: instance_id, quotas: encode_json(quotas).await }),
        Event::ColdDocumentPairPage(_) => unreachable!("cold pages use the dedicated poll input"),
        Event::CommandIngressPage { .. } => unreachable!("command pages are lifted through reactor.poll's dedicated page argument"),
        Event::UiIntent { instance, intent } => wit_events::Event::UiIntent(wit_events::UiIntentEvent { instance: instance.0.parse().unwrap_or(instance_id), intent: intent.clone() }),
        Event::SurfaceVisible { surface, body_key, view_state } => {
            wit_events::Event::SurfaceVisible(wit_events::SurfaceVisibleEvent { surface: wit_surface_ref(instance_id, surface).await, body_key: body_key.clone(), view_state: view_state.clone() })
        }
        Event::SurfaceHidden { surface } => wit_events::Event::SurfaceHidden(wit_events::SurfaceHiddenEvent { surface: wit_surface_ref(instance_id, surface).await }),
        Event::SurfaceResized { surface, width, height } => wit_events::Event::SurfaceResized(wit_events::SurfaceResizedEvent { surface: wit_surface_ref(instance_id, surface).await, width: *width, height: *height }),
        Event::PatchAck { receipt, surface, revision } => wit_events::Event::PatchAck(wit_events::PatchAckEvent { receipt: kernel_patch_receipt_to_wit(*receipt), surface: wit_surface_ref(instance_id, surface).await, revision: *revision }),
        Event::PatchRejected { receipt, surface, revision, reason } => {
            wit_events::Event::PatchRejected(wit_events::PatchRejectedEvent { receipt: kernel_patch_receipt_to_wit(*receipt), surface: wit_surface_ref(instance_id, surface).await, revision: *revision, reason: reason.clone() })
        }
        Event::Completed { req, result } => wit_events::Event::Completed(wit_events::CompletedEvent { req: req.0, outcome: kernel_request_outcome_to_wit(result).await }),
        Event::HttpChunk { req, bytes, done } => wit_events::Event::HttpChunk(wit_events::HttpChunkEvent { req: req.0, params: wit_events::HttpChunkParams { bytes: bytes.clone(), done: *done } }),
        Event::JobProgress { job, progress } => wit_events::Event::JobProgress(wit_events::JobProgressEvent { job: *job, progress: progress.clone().unwrap_or_default() }),
        Event::JobCompleted { job, result } => wit_events::Event::JobCompleted(wit_events::JobCompletedEvent { job: *job, outcome: kernel_request_outcome_to_wit(result).await }),
        Event::Message { source, payload } => wit_events::Event::Message(wit_events::MessageEvent { source: kernel_message_endpoint_to_wit(source).await, payload: payload.clone() }),
        Event::Timer { id } => wit_events::Event::Timer(wit_events::TimerEvent { id: *id }),
        Event::Wake => wit_events::Event::Wake,
        Event::Request { req, from, capability, payload } => {
            wit_events::Event::Request(wit_events::RequestEvent { req: req.0, params: wit_events::RequestParams { origin: kernel_message_endpoint_to_wit(from).await, capability: capability.clone(), payload: payload.clone() } })
        }
    }
}

async fn kernel_activation_event_to_wit(reason: &semio_framework::kernel::ActivationEvent) -> wit_events::ActivationEvent {
    use semio_framework::kernel::ActivationEvent as A;
    match reason {
        A::OnCommand { id } => wit_events::ActivationEvent::OnCommand(id.clone()),
        A::OnViewVisible { id } => wit_events::ActivationEvent::OnViewVisible(id.clone()),
        A::OnFileType { ext } => wit_events::ActivationEvent::OnFileType(ext.clone()),
        A::OnArtifactKind { kind } => wit_events::ActivationEvent::OnArtifactKind(kind.clone()),
        A::OnExtensionRequest { point } => wit_events::ActivationEvent::OnExtensionRequest(point.clone()),
        A::OnStartupFinished => wit_events::ActivationEvent::OnStartupFinished,
    }
}

async fn kernel_capability_change_to_wit(change: &semio_framework::kernel::CapabilityChange) -> wit_capabilities::CapabilityChange {
    use semio_framework::kernel::CapabilityChange as C;
    match change {
        C::Granted { id: _, grant } => wit_capabilities::CapabilityChange::Granted(kernel_broker_grant_to_wit(grant).await),
        C::Revoked { id } => wit_capabilities::CapabilityChange::Revoked(id.0.clone()),
        C::Narrowed { id: _, grant } => wit_capabilities::CapabilityChange::Narrowed(kernel_broker_grant_to_wit(grant).await),
    }
}

async fn kernel_broker_grant_to_wit(grant: &BrokerCapabilityGrant) -> wit_capabilities::CapabilityGrant {
    wit_capabilities::CapabilityGrant { token: wit_capabilities::CapabilityToken { id: grant.id.0.clone(), token: grant.token.0 as u64 }, scope: grant.scope.clone(), expires_ms: grant.expires_ms.map(|value| value as i64) }
}
//#endregion 🔀️EffectEventMarshal
//#endregion 🐎️WasmtimeRuntime
//#endregion 🎭️GuestRuntime
