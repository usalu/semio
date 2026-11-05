//! 🪪️ Original command identity hashing preserves the admitting store clock and streams the actual actor bytes.
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::{RetainedCloneProgress, ValueError, ValueRefusalKind};
use std::mem::size_of;

pub struct AuthoringSeedWork {
    source: semio_framework::action_bus::OriginalToolSourceBinding,
    actor: (usize, usize),
    clock: semio_framework::HybridLogicalTimestamp,
    hash: Option<semio_framework_hash::Sha256>,
    cursor: usize,
    complete: bool,
}

impl AuthoringSeedWork {
    pub(super) fn birth_bytes() -> usize { size_of::<Self>() }

    pub(super) fn new(actor: &str, clock: semio_framework::HybridLogicalTimestamp, cx: &StepContext<'_>) -> Self {
        Self { source: semio_framework::action_bus::OriginalToolSourceBinding::borrow(cx), actor: (actor.as_ptr() as usize, actor.len()), clock, hash: Some(Default::default()), cursor: 0, complete: false }
    }

    pub(super) fn advance(&mut self, actor: &str, output: &mut Option<String>, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if !self.source.matches(cx) || self.actor != (actor.as_ptr() as usize, actor.len()) {
            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "authoring seed requires its unchanged original actor and clock").into());
        }
        if self.complete { if output.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"authoring seed lost its original output receiving slot").into())}return Ok(true); }
        if output.is_some() { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "authoring seed output is already occupied").into()); }
        let grant = cx.retained_grant();
        if cx.should_yield() || cx.is_cancelled() || grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(false); }
        if self.cursor < actor.len() {
            let count = (actor.len() - self.cursor).min(4096).min(grant.maximum_copy_bytes).min(cx.fuel_remaining() as usize);
            if count == 0 { return Ok(false); }
            self.hash.as_mut().unwrap().update(&actor.as_bytes()[self.cursor..self.cursor + count]);
            self.cursor += count;
            cx.consume_fuel(count as u64);
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: count, ..Default::default() })?;
            return Ok(false);
        }
        const LENGTH: usize = 31;
        if grant.maximum_copy_bytes < 256 || grant.maximum_capacity_bytes < LENGTH || cx.fuel_remaining() < 256 { return Ok(false); }
        let mut result = String::new();
        result.try_reserve_exact(LENGTH).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "authoring seed output allocation refused"))?;
        let mut tail = [0_u8; 64];
        let count = clock_tail(self.clock, &mut tail);
        let mut hash = self.hash.take().unwrap();
        hash.update(&tail[..count]);
        let digest = hash.finalize();
        result.push_str("authoring-seed-");
        for byte in &digest[..8] {
            result.push(b"0123456789abcdef"[(byte >> 4) as usize] as char);
            result.push(b"0123456789abcdef"[(byte & 15) as usize] as char);
        }
        let capacity = result.capacity();
        *output = Some(result);
        self.complete = true;
        cx.consume_fuel(256);
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: count + LENGTH, retained_capacity_bytes: capacity, ..Default::default() })?;
        Ok(true)
    }
}

fn clock_tail(clock: semio_framework::HybridLogicalTimestamp, output: &mut [u8; 64]) -> usize {
    let mut at = 0;
    for (separator, mut number) in [(0x1f, clock.actor), (b':', clock.physical_ms), (b':', clock.logical)] {
        output[at] = separator;
        at += 1;
        let start = at;
        loop {
            output[at] = b'0' + (number % 10) as u8;
            at += 1;
            number /= 10;
            if number == 0 { break; }
        }
        output[start..at].reverse();
    }
    at
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn authoring_seed_recipe_matches_independent_sha256_vectors() {
        for (actor, clock, expected) in [("alice", semio_framework::HybridLogicalTimestamp { actor: 1, physical_ms: 42, logical: 0 }, "a8fb2c456b588351"), ("💧/Ä", semio_framework::HybridLogicalTimestamp { actor: u64::MAX, physical_ms: 9223372036854775807, logical: 65535 }, "a726d4b887f4d8b4")] {
            let mut bytes = [0; 64];
            let count = clock_tail(clock, &mut bytes);
            let mut hash = semio_framework_hash::Sha256::new();
            for page in actor.as_bytes().chunks(1) { hash.update(page); }
            hash.update(&bytes[..count]);
            assert_eq!(semio_framework_hash::hex_lower(&hash.finalize()[..8]), expected);
        }
    }
}
