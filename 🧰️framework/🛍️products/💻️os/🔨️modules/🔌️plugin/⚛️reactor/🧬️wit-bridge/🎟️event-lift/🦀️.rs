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

fn bounded_media_text(value: &DslValue, key: &str, maximum: usize) -> Lift<String> {
    let value = field(value, key)?.as_str().ok_or_else(outside)?;
    if value.is_empty() || value.chars().take(maximum + 1).count() > maximum { return Err(outside()); }
    Ok(value.to_owned())
}

fn original_media_control(value: &DslValue) -> Lift<ev::OriginalMediaControlEvent> {
    let loan = field(value, "loan")?;
    let operation_id = bounded_media_text(loan, "operationId", 16)?;
    let generation = bounded_media_text(loan, "generation", 16)?;
    let base_revision = bounded_media_text(loan, "baseRevision", 64)?;
    let valid_hex = |value: &str, width| value.len() == width &&
        value.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte));
    if !valid_hex(&operation_id, 16) || !valid_hex(&generation, 16) || !valid_hex(&base_revision, 64) {
        return Err(outside());
    }
    let control = field(value, "control")?;
    let phase = field(control, "tag")?.as_str().ok_or_else(outside)?;
    let control = match (phase, control.get("val")) {
        ("begin", Some(begin)) => ev::OriginalMediaControl::Begin(ev::OriginalMediaBegin {
            name: bounded_media_text(begin, "name", 255)?,
            media_type: bounded_media_text(begin, "mediaType", 255)?,
        }),
        ("complete", None) => ev::OriginalMediaControl::Complete,
        ("cancel", None) => ev::OriginalMediaControl::Cancel,
        _ => return Err(outside()),
    };
    Ok(ev::OriginalMediaControlEvent {
        loan: ev::OriginalMediaLoan {
            operation_id, generation, base_revision, instance_id: word32(loan, "instanceId")?,
            document_id: bounded_media_text(loan, "documentId", 256)?,
        },
        owner: word64(value, "owner")?, seq: word64(value, "seq")?,
        kind: u8::try_from(word64(value, "kind")?).map_err(|_| outside())?,
        extent: word32(value, "extent")?, control,
    })
}

fn closed_natural_fields(value: &DslValue, expected: &[&str]) -> Lift<()> {
    let DslValue::Object(fields) = value else { return Err(outside()); };
    if fields.len() != expected.len() || expected.iter().any(|key| value.get(key).is_none()) {
        return Err(outside());
    }
    Ok(())
}

fn natural_declaration(value: &DslValue) -> Lift<ev::NaturalSourceDeclaration> {
    closed_natural_fields(value, &["formatKind", "extension", "mediaType", "binary"])?;
    Ok(ev::NaturalSourceDeclaration {
        format_kind: bounded_media_text(value, "formatKind", 128)?,
        extension: bounded_media_text(value, "extension", 16)?,
        media_type: bounded_media_text(value, "mediaType", 128)?,
        binary: flag(value, "binary")?,
    })
}

fn natural_purpose(value: &DslValue) -> Lift<ev::ActorInstanceOpenPurpose> {
    let (tag, val) = tagged(value)?;
    Ok(match (tag.as_str(), val) {
        ("ordinary", None) => {
            closed_natural_fields(value, &["tag"])?;
            ev::ActorInstanceOpenPurpose::Ordinary
        }
        ("natural-source", Some(source)) => {
            closed_natural_fields(value, &["tag", "val"])?;
            closed_natural_fields(source, &["declaration", "extent"])?;
            ev::ActorInstanceOpenPurpose::NaturalSource(ev::NaturalSourceLaunch {
                declaration: natural_declaration(field(source, "declaration")?)?,
                extent: word32(source, "extent")?,
            })
        }
        _ => return Err(outside()),
    })
}

fn natural_prepare(value: &DslValue) -> Lift<ev::NaturalSourcePrepare> {
    closed_natural_fields(value, &["instanceId", "seq", "declaration", "extent"])?;
    Ok(ev::NaturalSourcePrepare {
        instance_id: word32(value, "instanceId")?,
        seq: word64(value, "seq")?,
        declaration: natural_declaration(field(value, "declaration")?)?,
        extent: word32(value, "extent")?,
    })
}

fn natural_control(value: &DslValue) -> Lift<ev::NaturalSourceControlEvent> {
    closed_natural_fields(value, &["loan", "owner", "seq", "kind", "extent", "control"])?;
    let loan = field(value, "loan")?;
    closed_natural_fields(loan, &["operationId", "generation", "instanceId", "declaration", "extent"])?;
    let hex = |key| {
        let value = bounded_media_text(loan, key, 16)?;
        if value.len() != 16 || value == "0000000000000000" ||
           !value.bytes().all(|ch| ch.is_ascii_digit() || (b'a'..=b'f').contains(&ch)) { return Err(outside()); }
        Ok(value)
    };
    let control = match field(value, "control")?.as_str().ok_or_else(outside)? {
        "begin" => ev::NaturalSourceControl::Begin,
        "complete" => ev::NaturalSourceControl::Complete,
        "cancel" => ev::NaturalSourceControl::Cancel,
        _ => return Err(outside()),
    };
    Ok(ev::NaturalSourceControlEvent {
        loan: ev::NaturalSourceLoan {
            operation_id: hex("operationId")?,
            generation: hex("generation")?,
            instance_id: word32(loan, "instanceId")?,
            declaration: natural_declaration(field(loan, "declaration")?)?,
            extent: word32(loan, "extent")?,
        },
        owner: word64(value, "owner")?, seq: word64(value, "seq")?,
        kind: u8::try_from(word64(value, "kind")?).map_err(|_| outside())?,
        extent: word32(value, "extent")?, control,
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
        "original-media-control" => ev::Event::OriginalMediaControl(original_media_control(val)?),
        "natural-source-prepare" => ev::Event::NaturalSourcePrepare(natural_prepare(val)?),
        "natural-source-control" => ev::Event::NaturalSourceControl(natural_control(val)?),
        "folder-archive-load" => ev::Event::FolderArchiveLoad(ev::FolderArchiveLoadEvent { instance: word32(val, "instance")?, request: pack(val, "request")? }),
        "instance-open" => ev::Event::InstanceOpen(ev::InstanceOpenEvent {
            purpose: natural_purpose(field(val, "purpose")?)?,
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
