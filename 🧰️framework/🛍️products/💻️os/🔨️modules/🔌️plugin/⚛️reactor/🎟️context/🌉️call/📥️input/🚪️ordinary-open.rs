//! 🚪️ Receives a canonical ordinary opening directly into its paid Poll owner.
use super::{poll_wire, ActorInput, OriginalTurnFailure, RetainedCloneProgress, StepContext, ValueError, ValueRefusalKind};
use semio_framework::kernel::{ActorInstanceOpenPurpose, ActorInstanceOpenRequest, AppInstanceId, BrokerCapabilityGrant, CapabilityId, CapabilityToken, Event, QuotaSchema};
use semio_framework_job_native_decode::OriginalNativeDecodePolicy;
use semio_framework_pack_json::JsonReadLimits;
use semio_plugin_abi::owned_abi::{OwnedCorePollLimits, PollInput};
use std::mem::size_of;

const PREFIX: &[u8] = br#"{"Poll":{"events":[{"instanceOpen":{"request":{"activationGeneration":"#;
const MAXIMUM_BYTES: usize = 2048;
const DEPTH: usize = 8;

#[derive(Clone, Copy)]
pub(super) struct Limits { wire: u64, owned: usize }

pub(super) const fn select(policy: OriginalNativeDecodePolicy, limits: JsonReadLimits) -> Option<Limits> {
    if limits.maximum_depth < DEPTH || limits.maximum_items < 32 { return None; }
    let owned = if policy.maximum_owned_bytes < limits.maximum_allocation_bytes { policy.maximum_owned_bytes } else { limits.maximum_allocation_bytes };
    Some(Limits { wire: limits.maximum_bytes, owned })
}

struct Open<'a> {
    request: ActorInstanceOpenRequest,
    app: &'a str,
    actor: &'a str,
    grant: Option<(&'a str, &'a str)>,
    limits: OwnedCorePollLimits,
}

fn text<'a>(rest: &mut &'a [u8]) -> Option<&'a str> {
    poll_wire::take(rest, b"\"")?;
    let start = *rest;
    loop {
        match poll_wire::byte(rest)? {
            b'"' => return std::str::from_utf8(&start[..start.len()-rest.len()-1]).ok(),
            b'\\' => return None,
            0x20..=0x7e => (),
            _ => return None,
        }
    }
}

fn decode<'a>(rest: &mut &'a [u8]) -> Option<Open<'a>> {
    use poll_wire::{take, uint};
    take(rest, PREFIX)?;
    let activation_generation = poll_wire::generation(rest)?;
    take(rest, br#","instanceId":"#)?;
    let instance_id = u32::try_from(uint(rest, b',')?).ok()?;
    take(rest, br#""requestSequence":"#)?;
    let request_sequence = poll_wire::request_sequence(rest, b'}')?;
    take(rest, br#","purpose":"ordinary","appId":"#)?;
    let app = text(rest)?;
    take(rest, br#","actor":"#)?;
    let actor = text(rest)?;
    take(rest, br#","config":[],"assets":[],"capabilities":["#)?;
    let grant = if rest.first()==Some(&b'{') {
        take(rest, br#"{"token":"0","id":"#)?;
        let id = text(rest)?;
        take(rest, br#","scope":"#)?;
        let scope = text(rest)?;
        take(rest, b"}")?;
        Some((id,scope))
    } else { None };
    take(rest, br#"],"quotas":{}}}"#)?;
    take(rest, poll_wire::LIMITS_PREFIX)?;
    let limits = poll_wire::limits(rest)?;
    Some(Open { request: ActorInstanceOpenRequest { activation_generation, instance_id, request_sequence }, app, actor, grant, limits })
}

fn allocation_failed() -> ValueError {
    ValueError::literal(ValueRefusalKind::AllocationFailed, "ordinary Poll opening allocation failed")
}

fn post(cx: &mut StepContext<'_>, bytes: usize, capacity: usize) -> Result<(), ValueError> {
    cx.consume_fuel(1);
    cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: capacity, ..Default::default() })
}

fn copy_text(target: &mut String, source: &str, cx: &mut StepContext<'_>) -> Result<(), ValueError> {
    target.try_reserve_exact(source.len()).map_err(|_| allocation_failed())?;
    target.push_str(source);
    cx.consume_fuel(source.len() as u64);
    post(cx, size_of::<String>() + source.len(), target.capacity())
}

impl Open<'_> {
    fn quote(&self) -> RetainedCloneProgress {
        let text_bytes = self.app.len() + self.actor.len() + self.grant.map_or(0, |(id, scope)| id.len() + scope.len());
        let grant_bytes = usize::from(self.grant.is_some()) * size_of::<BrokerCapabilityGrant>();
        let capacity = size_of::<Event>() + grant_bytes + text_bytes;
        let strings = if self.grant.is_some() { 4 } else { 2 };
        RetainedCloneProgress {
            copied_items: if self.grant.is_some() { 7 } else { 4 },
            copied_bytes: size_of::<Option<ActorInput>>() + size_of::<Option<Limits>>() + capacity + strings * size_of::<String>(),
            retained_capacity_bytes: capacity, ..Default::default()
        }
    }

    fn publish(self, out: &mut Option<ActorInput>, cx: &mut StepContext<'_>) -> Result<(), ValueError> {
        *out = Some(ActorInput::Poll(PollInput { events: Vec::new(), command_page: None, cold_pair_page: None, limits: self.limits }));
        post(cx, size_of::<Option<ActorInput>>() + size_of::<Option<Limits>>(), 0)?;
        let Some(ActorInput::Poll(poll)) = out else { unreachable!() };
        poll.events.try_reserve_exact(1).map_err(|_| allocation_failed())?;
        poll.events.push(Event::InstanceOpen {
            request: self.request, purpose: ActorInstanceOpenPurpose::Ordinary, app_id: AppInstanceId(String::new()),
            actor: String::new(), config: Vec::new(), assets: Vec::new(), capabilities: Vec::new(), quotas: QuotaSchema::default(),
        });
        post(cx, size_of::<Event>(), poll.events.capacity() * size_of::<Event>())?;
        let Event::InstanceOpen { app_id, actor, capabilities, .. } = &mut poll.events[0] else { unreachable!() };
        copy_text(&mut app_id.0, self.app, cx)?;
        copy_text(actor, self.actor, cx)?;
        if let Some((id, scope)) = self.grant {
            capabilities.try_reserve_exact(1).map_err(|_| allocation_failed())?;
            capabilities.push(BrokerCapabilityGrant { token: CapabilityToken(0), id: CapabilityId(String::new()), scope: String::new(), expires_ms: None });
            post(cx, size_of::<BrokerCapabilityGrant>(), capabilities.capacity() * size_of::<BrokerCapabilityGrant>())?;
            copy_text(&mut capabilities[0].id.0, id, cx)?;
            copy_text(&mut capabilities[0].scope, scope, cx)?;
        }
        Ok(())
    }
}

pub(super) fn advance(wire: &[u8], remaining: &mut Option<Limits>, out: &mut Option<ActorInput>, cx: &mut StepContext<'_>) -> Result<Option<Result<(), ValueError>>, OriginalTurnFailure> {
    let Some(limits) = *remaining else { return Ok(None); };
    let grant = cx.retained_grant();
    if wire.len() < PREFIX.len() || wire.len() > MAXIMUM_BYTES || wire.len() as u64 > limits.wire
        || grant.maximum_items == 0 || grant.maximum_copy_bytes < size_of::<Option<Limits>>()
        || grant.maximum_depth < DEPTH || cx.should_yield() || cx.is_cancelled() || cx.fuel_remaining() < wire.len() as u64 + 2 {
        return Ok(None);
    }
    let mut rest = wire;
    let opening = decode(&mut rest);
    cx.consume_fuel((wire.len()-rest.len()) as u64 + 1);
    if cx.should_yield() || cx.is_cancelled() { return Ok(None); }
    if let Some(opening) = opening {
        let quote = opening.quote();
        if quote.retained_capacity_bytes <= limits.owned && quote.retained_capacity_bytes <= grant.maximum_release_bytes
            && cx.fuel_remaining() >= quote.copied_items as u64 + wire.len() as u64 && quote.fits(grant) {
            cx.retained_progress().checked_add(quote)?;
            *remaining = None;
            return Ok(Some(opening.publish(out, cx)));
        }
    }
    cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Option<Limits>>(), ..Default::default() })?;
    *remaining = None;
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ordinary_open_matches_serde_and_preserves_every_field() {
        let cases: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/ordinary-open.json")).unwrap();
        for row in cases.as_array().unwrap() {
            let name = row["name"].as_str().unwrap();
            let mut wire = row["wire"].as_str().unwrap().as_bytes();
            let expected = serde_json::from_slice::<ActorInput>(wire);
            assert_eq!(expected.is_ok(), row["valid"].as_bool().unwrap(), "{name}");
            let opening = decode(&mut wire);
            assert_eq!(opening.is_some(), row["canonical"].as_bool().unwrap(), "{name}");
            if let Some(opening) = opening {
                let ActorInput::Poll(poll) = expected.unwrap() else { panic!("{name}") };
                let [Event::InstanceOpen { request, purpose, app_id, actor, config, assets, capabilities, quotas }] = &poll.events[..] else { panic!("{name}") };
                assert_eq!((opening.request, opening.app, opening.actor, opening.limits), (*request, app_id.0.as_str(), actor.as_str(), poll.limits));
                assert!(matches!(purpose, ActorInstanceOpenPurpose::Ordinary));
                assert!(assets.is_empty() && config.is_empty() && *quotas==QuotaSchema::default());
                assert_eq!(opening.grant, capabilities.first().map(|c| (c.id.0.as_str(),c.scope.as_str())));
            }
        }
    }
}
