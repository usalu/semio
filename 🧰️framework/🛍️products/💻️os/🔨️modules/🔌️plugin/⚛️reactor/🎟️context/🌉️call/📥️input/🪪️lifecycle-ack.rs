//! 🪪️ A canonical single lifecycle acknowledgment builds one paid typed event in its original Poll.
use super::{ActorInput, OriginalTurnFailure, RetainedCloneProgress, StepContext, ValueError, ValueRefusalKind, poll_wire};
use semio_framework::kernel::{ActorInstanceLifecycleAck, ActorInstanceLifecycleReceipt, ActorInstanceLifetime, Event};
use semio_framework_job_native_decode::OriginalNativeDecodePolicy;
use semio_framework_pack_json::JsonReadLimits;
use semio_plugin_abi::owned_abi::{OwnedCorePollLimits, PollInput};
use std::mem::size_of;

const PREFIX: &[u8] = br#"{"Poll":{"events":[{"instanceLifecycleAck":{"receipt":{"kind":""#;
const LIFETIME: &[u8] = br#"","lifetime":{"activationGeneration":"#;
const INSTANCE: &[u8] = br#","instanceId":"#;
const GUEST: &[u8] = br#""guestLifetime":"#;
const REQUEST: &[u8] = br#"},"requestSequence":"#;
const CLOSE: &[u8] = br#""closeGeneration":"#;
const REQUIRED_DEPTH: usize = 7;
const REQUIRED_ITEMS: u64 = OwnedCorePollLimits::ARITY as u64;
const EVENT_BYTES: usize = size_of::<Event>();
const MAXIMUM_BYTES: usize = PREFIX.len() + 8 + LIFETIME.len() + 22 + INSTANCE.len() + 11
    + GUEST.len() + 22 + REQUEST.len() + 17 + CLOSE.len() + 22 + 3
    + poll_wire::LIMITS_PREFIX.len() + OwnedCorePollLimits::ARITY * 21 + 2;

pub(super) const fn select(policy: OriginalNativeDecodePolicy, limits: JsonReadLimits) -> Option<u64> {
    if limits.maximum_depth >= REQUIRED_DEPTH && limits.maximum_items >= REQUIRED_ITEMS
        && limits.maximum_allocation_bytes >= EVENT_BYTES && policy.maximum_owned_bytes >= EVENT_BYTES
    {
        Some(limits.maximum_bytes)
    } else {
        None
    }
}

fn decode(rest: &mut &[u8]) -> Option<(ActorInstanceLifecycleReceipt, OwnedCorePollLimits)> {
    poll_wire::take(rest, PREFIX)?;
    let kind = poll_wire::byte(rest)?;
    poll_wire::take(rest, match kind { b'c' => b"aptured", b'a' => b"ccepted", b'r' => b"etired", _ => return None })?;
    poll_wire::take(rest, LIFETIME)?;
    let activation_generation = poll_wire::generation(rest)?;
    poll_wire::take(rest, INSTANCE)?;
    let instance_id = u32::try_from(poll_wire::uint(rest, b',')?).ok()?;
    poll_wire::take(rest, GUEST)?;
    let guest_lifetime = poll_wire::generation(rest)?;
    let lifetime = ActorInstanceLifetime { activation_generation, instance_id, guest_lifetime };
    poll_wire::take(rest, REQUEST)?;
    let receipt = if kind == b'c' {
        let request_sequence = poll_wire::request_sequence(rest, b'}')?;
        ActorInstanceLifecycleReceipt::Captured { lifetime, request_sequence }
    } else {
        let request_sequence = poll_wire::request_sequence(rest, b',')?;
        poll_wire::take(rest, CLOSE)?;
        let close_generation = poll_wire::generation(rest)?;
        poll_wire::take(rest, b"}")?;
        if kind == b'a' {
            ActorInstanceLifecycleReceipt::Accepted { lifetime, request_sequence, close_generation }
        } else {
            ActorInstanceLifecycleReceipt::Retired { lifetime, request_sequence, close_generation }
        }
    };
    poll_wire::take(rest, b"}}")?;
    poll_wire::take(rest, poll_wire::LIMITS_PREFIX)?;
    Some((receipt, poll_wire::limits(rest)?))
}

pub(super) fn advance(
    wire: &[u8],
    remaining: &mut Option<u64>,
    out: &mut Option<ActorInput>,
    cx: &mut StepContext<'_>,
) -> Result<Option<Result<(), ValueError>>, OriginalTurnFailure> {
    let Some(maximum_bytes) = *remaining else { return Ok(None); };
    let grant = cx.retained_grant();
    let state_bytes = size_of::<Option<u64>>();
    let progress = RetainedCloneProgress {
        copied_items: 2,
        copied_bytes: state_bytes + size_of::<Option<ActorInput>>() + EVENT_BYTES,
        retained_capacity_bytes: EVENT_BYTES,
        ..Default::default()
    };
    if wire.len() <= PREFIX.len() || wire.len() > MAXIMUM_BYTES || wire.len() as u64 > maximum_bytes
        || cx.should_yield() || cx.is_cancelled() || cx.fuel_remaining() < wire.len() as u64 + 2
        || grant.maximum_depth < REQUIRED_DEPTH || grant.maximum_release_bytes < EVENT_BYTES || !progress.fits(grant)
    {
        return Ok(None);
    }

    cx.retained_progress().checked_add(progress)?;
    let mut rest = wire;
    let decoded = decode(&mut rest);
    cx.consume_fuel((wire.len() - rest.len()) as u64 + 1);
    if cx.should_yield() || cx.is_cancelled() { return Ok(None); }
    let Some((receipt, limits)) = decoded else {
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: state_bytes, ..Default::default() })?;
        *remaining = None;
        return Ok(None);
    };
    let mut events = Vec::new();
    cx.consume_fuel(1);
    if events.try_reserve_exact(1).is_err() {
        return Ok(Some(Err(ValueError::literal(ValueRefusalKind::AllocationFailed, "lifecycle acknowledgment event allocation failed"))));
    }
    let retained_capacity_bytes = events.capacity() * EVENT_BYTES;
    events.push(Event::InstanceLifecycleAck(ActorInstanceLifecycleAck { receipt }));
    *remaining = None;
    *out = Some(ActorInput::Poll(PollInput { events, command_page: None, cold_pair_page: None, limits }));
    cx.consume_retained(RetainedCloneProgress { retained_capacity_bytes, ..progress })?;
    Ok(Some(Ok(())))
}

#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_job::{Generation, OperationId};
    use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
    use semio_framework_value::RetainedCloneGrant;

    fn cases() -> serde_json::Value {
        serde_json::from_str(include_str!("🧪️tests/🧫️lifecycle-ack.json")).unwrap()
    }

    #[test]
    fn canonical_lifecycle_ack_matches_serde() {
        for row in cases().as_array().unwrap() {
            let name = row["name"].as_str().unwrap();
            let mut wire = row["wire"].as_str().unwrap().as_bytes();
            let reference = serde_json::from_slice::<ActorInput>(wire);
            assert_eq!(reference.is_ok(), row["valid"].as_bool().unwrap(), "{name}");
            let parsed = decode(&mut wire);
            assert_eq!(parsed.is_some(), row["canonical"].as_bool().unwrap(), "{name}");
            if let Some((receipt, limits)) = parsed {
                let ActorInput::Poll(expected) = reference.unwrap() else { panic!("{name}: expected Poll"); };
                let [Event::InstanceLifecycleAck(ack)] = expected.events.as_slice() else { panic!("{name}: expected one acknowledgment"); };
                assert_eq!((receipt, limits), (ack.receipt, expected.limits), "{name}");
                assert!(expected.command_page.is_none() && expected.cold_pair_page.is_none(), "{name}");
            }
        }
    }

    #[test]
    fn lifecycle_ack_denies_short_sources_before_allocating() {
        let corpus = cases();
        let wire = corpus[0]["wire"].as_str().unwrap().as_bytes();
        let stock = RetainedCloneGrant {
            maximum_items: 10_000, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20,
            maximum_release_bytes: 1 << 20, maximum_depth: 64,
        };
        let limits = JsonReadLimits {
            maximum_bytes: wire.len() as u64, maximum_allocation_bytes: EVENT_BYTES,
            maximum_depth: REQUIRED_DEPTH, maximum_items: REQUIRED_ITEMS,
        };
        let policy = OriginalNativeDecodePolicy { maximum_owned_bytes: EVENT_BYTES };
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(954), Generation(2), stock, || Some(1), 1_000_000, 100_000);
        caller.audit_heap();
        assert!(select(OriginalNativeDecodePolicy { maximum_owned_bytes: EVENT_BYTES - 1 }, limits).is_none());
        assert!(select(policy, JsonReadLimits { maximum_allocation_bytes: EVENT_BYTES - 1, ..limits }).is_none());
        assert!(select(policy, JsonReadLimits { maximum_items: REQUIRED_ITEMS - 1, ..limits }).is_none());
        assert!(select(policy, JsonReadLimits { maximum_depth: REQUIRED_DEPTH - 1, ..limits }).is_none());
        for (grant, limits) in [
            (RetainedCloneGrant { maximum_items: 1, ..stock }, limits),
            (RetainedCloneGrant { maximum_copy_bytes: size_of::<Option<ActorInput>>() + size_of::<Option<u64>>() + EVENT_BYTES - 1, ..stock }, limits),
            (RetainedCloneGrant { maximum_capacity_bytes: EVENT_BYTES - 1, ..stock }, limits),
            (RetainedCloneGrant { maximum_release_bytes: EVENT_BYTES - 1, ..stock }, limits),
            (RetainedCloneGrant { maximum_depth: REQUIRED_DEPTH - 1, ..stock }, limits),
            (stock, JsonReadLimits { maximum_bytes: limits.maximum_bytes - 1, ..limits }),
        ] {
            let mut selected = select(policy, limits);
            let mut input = None;
            caller.bare_turn(grant, |cx| {
                let before = (cx.retained_progress(), cx.original_work_units_spent());
                assert!(advance(wire, &mut selected, &mut input, cx)?.is_none());
                assert_eq!((cx.retained_progress(), cx.original_work_units_spent()), before);
                Ok(())
            }).unwrap();
            assert!(input.is_none());
            assert_eq!(selected, select(policy, limits));
        }
        let mut selected = select(policy, limits);
        let mut input = None;
        caller.bare_turn(stock, |cx| {
            let deadline = cx.deadline_us();
            cx.with_original_work_loan(wire.len() as u64 + 1, deadline, |cx| {
                assert!(advance(wire, &mut selected, &mut input, cx)?.is_none());
                Ok(())
            })
        }).unwrap();
        caller.cancel().cancel_now();
        assert!(caller.bare_turn(stock, |cx| advance(wire, &mut selected, &mut input, cx)).unwrap().is_none());
        assert!(input.is_none());
        assert_eq!(selected, select(policy, limits));
        caller.retire(stock);
    }

    #[test]
    fn lifecycle_ack_posts_exact_event_birth_and_release() {
        use semio_framework_value::retirement::controlled::ControlledRetirement;

        let corpus = cases();
        let wire = corpus[0]["wire"].as_str().unwrap().as_bytes();
        let stock = RetainedCloneGrant {
            maximum_items: 10_000, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20,
            maximum_release_bytes: 1 << 20, maximum_depth: 64,
        };
        let mut selected = Some(wire.len() as u64);
        let mut input = None;
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(955), Generation(2), stock, || Some(1), 1_000_000, 100_000);
        caller.audit_heap();
        let copy = size_of::<Option<ActorInput>>() + size_of::<Option<u64>>() + EVENT_BYTES;
        let exact = RetainedCloneGrant {
            maximum_items: 2, maximum_copy_bytes: copy, maximum_capacity_bytes: EVENT_BYTES,
            maximum_release_bytes: EVENT_BYTES, maximum_depth: REQUIRED_DEPTH,
        };
        caller.bare_turn(exact, |cx| {
            let before = cx.retained_progress();
            let work = cx.original_work_units_spent();
            assert!(matches!(advance(wire, &mut selected, &mut input, cx)?, Some(Ok(()))));
            assert_eq!(cx.original_work_units_spent() - work, wire.len() as u128 + 2);
            assert_eq!(cx.retained_progress(), before.checked_add(RetainedCloneProgress {
                copied_items: 2, copied_bytes: copy, retained_capacity_bytes: EVENT_BYTES, released_bytes: 0,
            }).unwrap());
            Ok(())
        }).unwrap();
        assert!(selected.is_none());
        let mut owner = ControlledRetirement::retain(input.take().unwrap());
        let mut retirement = RetainedCloneProgress::default();
        for _ in 0..128 {
            if owner.terminal_is_empty() { break; }
            caller.retirement_turn(stock, |cx| super::super::retire_step(&mut owner, cx)).unwrap();
            retirement = retirement.checked_add(caller.receipt()).unwrap();
        }
        assert!(owner.terminal_is_empty());
        assert_eq!(retirement.released_bytes, retirement.retained_capacity_bytes + EVENT_BYTES);
        caller.retire(stock);
    }
}
