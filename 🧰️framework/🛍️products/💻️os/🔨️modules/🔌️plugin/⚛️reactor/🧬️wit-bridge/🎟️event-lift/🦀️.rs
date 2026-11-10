//! 🎟️ Lifts one admitted `{ tag, val }` actor-input event into the generated WIT event: the exact inverse of the host's `event_to_wire`, refusing any shape outside the event schema.
use semio_framework_value::{DslValue, ValueError, ValueRefusalKind};
use semio_plugin_abi::wasip2::semio::framework::{capabilities as cap, events as ev, instance_lifetime as life, types, ui};

type Lift<T> = Result<T, ValueError>;

fn outside() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "admitted actor event is outside the event schema")
}
fn field<'a>(value: &'a DslValue, key: &str) -> Lift<&'a DslValue> {
    value.get(key).ok_or_else(outside)
}
fn word64(value: &DslValue, key: &str) -> Lift<u64> {
    field(value, key)?.as_u64().ok_or_else(outside)
}
fn word32(value: &DslValue, key: &str) -> Lift<u32> {
    u32::try_from(word64(value, key)?).map_err(|_| outside())
}
fn text(value: &DslValue, key: &str) -> Lift<String> {
    field(value, key)?.as_str().map(str::to_owned).ok_or_else(outside)
}
fn flag(value: &DslValue, key: &str) -> Lift<bool> {
    field(value, key)?.as_bool().ok_or_else(outside)
}
fn octets(value: &DslValue) -> Lift<Vec<u8>> {
    if let DslValue::Bytes(bytes) = value {
        return Ok(bytes.clone());
    }
    value.as_array().ok_or_else(outside)?.iter().map(|byte| byte.as_u64().and_then(|byte| u8::try_from(byte).ok()).ok_or_else(outside)).collect()
}
fn pack(value: &DslValue, key: &str) -> Lift<Vec<u8>> {
    octets(field(value, key)?)
}
fn tagged<'a>(value: &'a DslValue) -> Lift<(String, Option<&'a DslValue>)> {
    Ok((text(value, "tag")?, value.get("val")))
}
fn payload<'a>(value: Option<&'a DslValue>) -> Lift<&'a DslValue> {
    value.ok_or_else(outside)
}
fn lifetime(value: &DslValue) -> Lift<life::Lifetime> {
    Ok(life::Lifetime { activation_generation: word64(value, "activationGeneration")?, instance_id: word32(value, "instanceId")?, guest_lifetime: word64(value, "guestLifetime")? })
}
fn patch_receipt(value: &DslValue) -> Lift<life::UiPatchReceipt> {
    Ok(life::UiPatchReceipt { lifetime: lifetime(field(value, "lifetime")?)?, patch_sequence: word64(value, "patchSequence")? })
}
fn surface(value: &DslValue) -> Lift<ui::SurfaceRef> {
    Ok(ui::SurfaceRef { instance: word32(value, "instance")?, surface: text(value, "surface")? })
}
fn endpoint(value: &DslValue) -> Lift<types::MessageEndpoint> {
    let (tag, val) = tagged(value)?;
    let val = payload(val)?;
    let id = || val.as_u64().and_then(|id| u32::try_from(id).ok()).ok_or_else(outside);
    let name = || val.as_str().map(str::to_owned).ok_or_else(outside);
    Ok(match tag.as_str() {
        "shell" => types::MessageEndpoint::Shell(id()?),
        "backbone" => types::MessageEndpoint::Backbone(name()?),
        "plugin-instance" => types::MessageEndpoint::PluginInstance(id()?),
        "extension" => types::MessageEndpoint::Extension(name()?),
        "topic" => types::MessageEndpoint::Topic(name()?),
        _ => return Err(outside()),
    })
}
fn grant(value: &DslValue) -> Lift<cap::CapabilityGrant> {
    let token = field(value, "token")?;
    let expires = field(value, "expiresMs")?;
    Ok(cap::CapabilityGrant { token: cap::CapabilityToken { id: text(token, "id")?, token: word64(token, "token")? }, scope: text(value, "scope")?, expires_ms: if expires.is_null() { None } else { Some(expires.as_i64().ok_or_else(outside)?) } })
}
fn completion(value: &DslValue) -> Lift<ev::CompletionResult> {
    let (tag, val) = tagged(value)?;
    let bytes = octets(payload(val)?)?;
    Ok(match tag.as_str() {
        "ok" => ev::CompletionResult::Ok(bytes),
        "fault" => ev::CompletionResult::Fault(bytes),
        _ => return Err(outside()),
    })
}
fn activation(value: &DslValue) -> Lift<ev::ActivationEvent> {
    let (tag, val) = tagged(value)?;
    let name = || payload(val)?.as_str().map(str::to_owned).ok_or_else(outside);
    Ok(match tag.as_str() {
        "on-command" => ev::ActivationEvent::OnCommand(name()?),
        "on-view-visible" => ev::ActivationEvent::OnViewVisible(name()?),
        "on-file-type" => ev::ActivationEvent::OnFileType(name()?),
        "on-artifact-kind" => ev::ActivationEvent::OnArtifactKind(name()?),
        "on-extension-request" => ev::ActivationEvent::OnExtensionRequest(name()?),
        "on-startup-finished" => ev::ActivationEvent::OnStartupFinished,
        _ => return Err(outside()),
    })
}
fn capability_change(value: &DslValue) -> Lift<cap::CapabilityChange> {
    let (tag, val) = tagged(value)?;
    let val = payload(val)?;
    Ok(match tag.as_str() {
        "granted" => cap::CapabilityChange::Granted(grant(val)?),
        "revoked" => cap::CapabilityChange::Revoked(val.as_str().map(str::to_owned).ok_or_else(outside)?),
        "narrowed" => cap::CapabilityChange::Narrowed(grant(val)?),
        _ => return Err(outside()),
    })
}
fn receipt(value: &DslValue) -> Lift<life::Receipt> {
    let (tag, val) = tagged(value)?;
    let val = payload(val)?;
    let (lifetime, request_sequence) = (lifetime(field(val, "lifetime")?)?, word64(val, "requestSequence")?);
    let closed = || Ok::<_, ValueError>(life::CloseReceipt { lifetime, request_sequence, close_generation: word64(val, "closeGeneration")? });
    Ok(match tag.as_str() {
        "captured" => life::Receipt::Captured(life::CapturedReceipt { lifetime, request_sequence }),
        "accepted" => life::Receipt::Accepted(closed()?),
        "retired" => life::Receipt::Retired(closed()?),
        _ => return Err(outside()),
    })
}

/// 🎟️ Lifts one event value; the guest never sees a host-built generated value, only this schema-checked lift.
pub fn event_from_value(value: &DslValue) -> Lift<ev::Event> {
    let (tag, val) = tagged(value)?;
    if tag == "wake" {
        return Ok(ev::Event::Wake);
    }
    let val = payload(val)?;
    Ok(match tag.as_str() {
        "instance-open" => ev::Event::InstanceOpen(ev::InstanceOpenEvent {
            instance: word32(val, "instance")?,
            activation_generation: word64(val, "activationGeneration")?,
            request_sequence: word64(val, "requestSequence")?,
            app_id: text(val, "appId")?,
            actor: text(val, "actor")?,
            config: pack(val, "config")?,
            assets: field(val, "assets")?.as_array().ok_or_else(outside)?.iter().map(|pair| match pair.as_array() { Some([name, bytes]) => Ok((name.as_str().map(str::to_owned).ok_or_else(outside)?, octets(bytes)?)), _ => Err(outside()) }).collect::<Lift<_>>()?,
            capabilities: field(val, "capabilities")?.as_array().ok_or_else(outside)?.iter().map(grant).collect::<Lift<_>>()?,
            quotas: pack(val, "quotas")?,
        }),
        "instance-close" => ev::Event::InstanceClose(life::CloseRequest { lifetime: lifetime(field(val, "lifetime")?)?, request_sequence: word64(val, "requestSequence")? }),
        "instance-lifecycle-ack" => ev::Event::InstanceLifecycleAck(receipt(val)?),
        "activate" => ev::Event::Activate(ev::ActivateEvent { instance: word32(val, "instance")?, reason: activation(field(val, "reason")?)? }),
        "suspend-request" => ev::Event::SuspendRequest(ev::SuspendRequestEvent { instance: word32(val, "instance")? }),
        "capability-changed" => ev::Event::CapabilityChanged(ev::CapabilityChangedEvent { instance: word32(val, "instance")?, change: capability_change(field(val, "change")?)? }),
        "quota-changed" => ev::Event::QuotaChanged(ev::QuotaChangedEvent { instance: word32(val, "instance")?, quotas: pack(val, "quotas")? }),
        "ui-intent" => ev::Event::UiIntent(ev::UiIntentEvent { instance: word32(val, "instance")?, intent: pack(val, "intent")? }),
        "surface-visible" => ev::Event::SurfaceVisible(ev::SurfaceVisibleEvent { surface: surface(field(val, "surface")?)?, body_key: text(val, "bodyKey")?, view_state: pack(val, "viewState")? }),
        "staged-form-requested" => ev::Event::StagedFormRequested(ev::StagedFormRequestedEvent { instance: word32(val, "instance")?, surface: text(val, "surface")?, body_key: text(val, "bodyKey")?, request: pack(val, "request")? }),
        "surface-hidden" => ev::Event::SurfaceHidden(ev::SurfaceHiddenEvent { surface: surface(field(val, "surface")?)? }),
        "surface-resized" => ev::Event::SurfaceResized(ev::SurfaceResizedEvent { surface: surface(field(val, "surface")?)?, width: word32(val, "width")?, height: word32(val, "height")? }),
        "patch-ack" => ev::Event::PatchAck(ev::PatchAckEvent { receipt: patch_receipt(field(val, "receipt")?)?, surface: surface(field(val, "surface")?)?, revision: word64(val, "revision")? }),
        "patch-rejected" => ev::Event::PatchRejected(ev::PatchRejectedEvent { receipt: patch_receipt(field(val, "receipt")?)?, surface: surface(field(val, "surface")?)?, revision: word64(val, "revision")?, reason: text(val, "reason")? }),
        "completed" => ev::Event::Completed(ev::CompletedEvent { req: word64(val, "req")?, outcome: completion(field(val, "outcome")?)? }),
        "http-chunk" => {
            let params = field(val, "params")?;
            ev::Event::HttpChunk(ev::HttpChunkEvent { req: word64(val, "req")?, params: ev::HttpChunkParams { bytes: pack(params, "bytes")?, done: flag(params, "done")? } })
        }
        "job-progress" => ev::Event::JobProgress(ev::JobProgressEvent { job: word64(val, "job")?, progress: pack(val, "progress")? }),
        "job-completed" => ev::Event::JobCompleted(ev::JobCompletedEvent { job: word64(val, "job")?, outcome: completion(field(val, "outcome")?)? }),
        "message" => ev::Event::Message(ev::MessageEvent { source: endpoint(field(val, "source")?)?, payload: pack(val, "payload")? }),
        "timer" => ev::Event::Timer(ev::TimerEvent { id: word64(val, "id")? }),
        "request" => {
            let params = field(val, "params")?;
            ev::Event::Request(ev::RequestEvent { req: word64(val, "req")?, params: ev::RequestParams { origin: endpoint(field(params, "origin")?)?, capability: text(params, "capability")?, payload: pack(params, "payload")? } })
        }
        _ => return Err(outside()),
    })
}
