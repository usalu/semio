//! 🌀️ A canonical empty Poll builds one inline typed input from the current authenticated wire.
use super::{ActorInput, OriginalTurnFailure, RetainedCloneProgress, StepContext};
use semio_framework_pack_json::JsonReadLimits;
use semio_plugin_abi::owned_abi::{OwnedCorePollLimits, PollInput};
use std::mem::size_of;

const PREFIX: &[u8] = br#"{"Poll":{"events":[],"command_page":null,"cold_pair_page":null,"limits":["#;
const SUFFIX: &[u8] = b"}}";
const MAXIMUM_BYTES: usize = PREFIX.len() + 20 * OwnedCorePollLimits::ARITY + OwnedCorePollLimits::ARITY + SUFFIX.len();
const MINIMUM_BYTES: usize = PREFIX.len() + 2 * OwnedCorePollLimits::ARITY + SUFFIX.len();
const REQUIRED_DEPTH: usize = 3;
const MINIMUM_JSON_ITEMS: u64 = OwnedCorePollLimits::ARITY as u64;

pub(super) const fn select(limits: JsonReadLimits) -> Option<u64> {
    if limits.maximum_depth >= REQUIRED_DEPTH && limits.maximum_items >= MINIMUM_JSON_ITEMS {
        Some(limits.maximum_bytes)
    } else {
        None
    }
}

pub(super) fn advance(
    wire: &[u8],
    remaining: &mut Option<u64>,
    out: &mut Option<ActorInput>,
    cx: &mut StepContext<'_>,
) -> Result<bool, OriginalTurnFailure> {
    let Some(maximum_bytes) = *remaining else { return Ok(false); };
    let grant = cx.retained_grant();
    let work = wire.len() as u64 + 1;
    let progress = RetainedCloneProgress {
        copied_items: 1,
        copied_bytes: size_of::<Option<ActorInput>>() + size_of::<Option<u64>>(),
        ..Default::default()
    };
    if !(MINIMUM_BYTES..=MAXIMUM_BYTES).contains(&wire.len()) || wire.len() as u64 > maximum_bytes
        || cx.should_yield() || cx.is_cancelled() || cx.fuel_remaining() < work
        || grant.maximum_depth < REQUIRED_DEPTH || !progress.fits(grant)
    {
        return Ok(false);
    }

    cx.retained_progress().checked_add(progress)?;
    let mut rest = wire;
    let limits = decode(&mut rest);
    let posted = if limits.is_some() {
        progress
    } else {
        RetainedCloneProgress { copied_bytes: size_of::<Option<u64>>(), ..progress }
    };
    cx.consume_fuel((wire.len() - rest.len()) as u64 + 1);
    cx.consume_retained(posted)?;
    *remaining = None;
    let Some(limits) = limits else { return Ok(false); };
    *out = Some(ActorInput::Poll(PollInput { events: Vec::new(), command_page: None, cold_pair_page: None, limits }));
    Ok(true)
}

fn byte(rest: &mut &[u8]) -> Option<u8> {
    let (head, tail) = rest.split_first()?;
    *rest = tail;
    Some(*head)
}

fn decode(rest: &mut &[u8]) -> Option<OwnedCorePollLimits> {
    for expected in PREFIX {
        if byte(rest)? != *expected { return None; }
    }
    let mut scalars = [0; OwnedCorePollLimits::ARITY];
    for (index, value) in scalars.iter_mut().enumerate() {
        let first = byte(rest)?.checked_sub(b'0').filter(|&digit| digit <= 9)?;
        *value = u64::from(first);
        let delimiter = if index + 1 == OwnedCorePollLimits::ARITY { b']' } else { b',' };
        loop {
            let next = byte(rest)?;
            if next == delimiter { break; }
            let digit = next.checked_sub(b'0').filter(|&digit| digit <= 9)?;
            if first == 0 { return None; }
            *value = value.checked_mul(10)?.checked_add(u64::from(digit))?;
        }
    }
    for expected in SUFFIX {
        if byte(rest)? != *expected { return None; }
    }
    if !rest.is_empty() { return None; }
    OwnedCorePollLimits::from_scalars(scalars)
}

#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_job::{Generation, OperationId};
    use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
    use semio_framework_value::RetainedCloneGrant;

    #[test]
    fn canonical_empty_poll_matches_the_serde_actor_wire() {
        let fixtures: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/🧫️empty-poll.json")).unwrap();
        for row in fixtures.as_array().unwrap() {
            let name = row["name"].as_str().unwrap();
            let mut wire = row["wire"].as_str().unwrap().as_bytes();
            let oracle = serde_json::from_slice::<ActorInput>(wire);
            assert_eq!(oracle.is_ok(), row["valid"].as_bool().unwrap(), "{name}");
            let decoded = decode(&mut wire);
            assert_eq!(decoded.is_some(), row["canonical"].as_bool().unwrap(), "{name}");
            if let Some(decoded) = decoded {
                let ActorInput::Poll(expected) = oracle.unwrap() else { panic!("{name}: expected Poll"); };
                assert_eq!(decoded, expected.limits, "{name}");
                assert!(expected.events.is_empty() && expected.command_page.is_none() && expected.cold_pair_page.is_none(), "{name}");
            }
        }
    }

    #[test]
    fn empty_poll_does_not_take_input_without_the_required_limits_or_grant() {
        let fixtures: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/🧫️empty-poll.json")).unwrap();
        let wire = fixtures[1]["wire"].as_str().unwrap().as_bytes();
        let stock = RetainedCloneGrant {
            maximum_items: 10_000, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20,
            maximum_release_bytes: 1 << 20, maximum_depth: 64,
        };
        let limits = JsonReadLimits {
            maximum_bytes: wire.len() as u64, maximum_allocation_bytes: stock.maximum_capacity_bytes,
            maximum_depth: stock.maximum_depth, maximum_items: stock.maximum_items as u64,
        };
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(953), Generation(2), stock, || Some(1), 1_000_000, 100_000);
        for (grant, limits) in [
            (RetainedCloneGrant { maximum_items: 0, ..stock }, limits),
            (RetainedCloneGrant { maximum_copy_bytes: size_of::<Option<ActorInput>>() + size_of::<Option<u64>>() - 1, ..stock }, limits),
            (RetainedCloneGrant { maximum_depth: REQUIRED_DEPTH - 1, ..stock }, limits),
            (stock, JsonReadLimits { maximum_depth: REQUIRED_DEPTH - 1, ..limits }),
            (stock, JsonReadLimits { maximum_items: MINIMUM_JSON_ITEMS - 1, ..limits }),
            (stock, JsonReadLimits { maximum_bytes: limits.maximum_bytes - 1, ..limits }),
        ] {
            let mut selected = select(limits);
            let mut input = None;
            let before = selected;
            let granted = caller.bare_turn(grant, |cx| {
                let receipt = cx.retained_progress();
                let work = cx.original_work_units_spent();
                let ready = advance(wire, &mut selected, &mut input, cx)?;
                assert_eq!(cx.retained_progress(), receipt);
                assert_eq!(cx.original_work_units_spent(), work);
                Ok(ready)
            }).unwrap();
            assert!(!granted && input.is_none());
            assert_eq!(selected, before);
        }
        let mut selected = select(limits);
        let mut input = None;
        assert!(!caller.bare_turn(stock, |cx| {
            let deadline = cx.deadline_us();
            cx.with_original_work_loan(wire.len() as u64, deadline, |cx| advance(wire, &mut selected, &mut input, cx))
        }).unwrap());
        assert_eq!(selected, select(limits));
        caller.cancel().cancel_now();
        assert!(!caller.bare_turn(stock, |cx| advance(wire, &mut selected, &mut input, cx)).unwrap());
        assert!(input.is_none());
        caller.retire(stock);
    }
}
