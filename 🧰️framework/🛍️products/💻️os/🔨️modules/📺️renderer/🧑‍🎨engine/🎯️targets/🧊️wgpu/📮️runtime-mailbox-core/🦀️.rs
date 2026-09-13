//! 📮️ Target-neutral fixed-capacity renderer completion mailbox and its interaction-checkout ledger.
//!
//! Ready and in-flight work share one bound. Keyless work is lossless and rejects overflow;
//! replaceable keyed work may coalesce only a ready completion with the same key. One reserved slot
//! guarantees that an owned interaction state can always return from a suspended worker turn.
//!
//! ⚖️ The single interaction state is CHECKED OUT for one bounded step and returned by the completion
//! the step finishes with. Two rules keep that from becoming a permanent stall, both pinned by
//! `🧫️fixtures/🎮️wgpu-runtime-mailbox-admission/🔣️.json`:
//!
//! * a completion that does not need the checked-out state is admitted PAST one that does, so the
//!   very completion carrying the state home is never queued behind the input waiting for it;
//! * a checkout that outlives [`INTERACTION_CHECKOUT_CREDITS`] apply opportunities is a typed
//!   diagnostic — the ledger names the site that took it — instead of an invisible freeze.

use std::collections::VecDeque;

pub(crate) struct Completion<T> {
    pub(crate) key: Option<&'static str>,
    pub(crate) revision: u64,
    pub(crate) requires_interaction: bool,
    pub(crate) apply: T,
}

pub(crate) struct BoundedCompletionQueue<T, const CAPACITY: usize> {
    pub(crate) ready: VecDeque<Completion<T>>,
    in_flight: usize,
}

impl<T, const CAPACITY: usize> BoundedCompletionQueue<T, CAPACITY> {
    pub(crate) fn new() -> Self {
        assert!(CAPACITY > 1, "completion mailbox needs an interaction reserve");
        Self { ready: VecDeque::with_capacity(CAPACITY), in_flight: 0 }
    }

    pub(crate) fn len(&self) -> usize {
        self.ready.len() + self.in_flight
    }

    fn make_room_for(&mut self, key: Option<&'static str>, limit: usize) -> bool {
        if self.len() < limit {
            return true;
        }
        let Some(key) = key else { return false };
        let Some(index) = self.ready.iter().position(|queued| queued.key == Some(key)) else { return false };
        self.ready.remove(index);
        true
    }

    pub(crate) fn enqueue(&mut self, completion: Completion<T>) -> bool {
        if !self.make_room_for(completion.key, CAPACITY - 1) {
            return false;
        }
        self.ready.push_back(completion);
        true
    }

    #[cfg(any(not(target_arch = "wasm32"), test))]
    pub(crate) fn reserve(&mut self, key: Option<&'static str>) -> bool {
        if !self.make_room_for(key, CAPACITY - 1) {
            return false;
        }
        self.in_flight += 1;
        true
    }

    pub(crate) fn reserve_interaction(&mut self) -> bool {
        if self.len() == CAPACITY {
            return false;
        }
        self.in_flight += 1;
        true
    }

    #[cfg(any(not(target_arch = "wasm32"), test))]
    pub(crate) fn cancel_interaction_reservation(&mut self) -> bool {
        let Some(next) = self.in_flight.checked_sub(1) else { return false };
        self.in_flight = next;
        true
    }

    pub(crate) fn finish(&mut self, completion: Completion<T>) {
        assert!(self.in_flight > 0, "runtime completion without reservation");
        self.in_flight -= 1;
        self.ready.push_front(completion);
        assert!(self.len() <= CAPACITY, "runtime completion mailbox capacity exceeded");
    }

    /// 🚦️ Whether the queue head is one the checked-out interaction state would block.
    pub(crate) fn head_requires_interaction(&self) -> bool {
        self.ready.front().is_some_and(|completion| completion.requires_interaction)
    }

    /// 🎯️ The first completion this turn may apply.
    ///
    /// 🩸️ The gate used to read the HEAD alone and refuse the whole turn without popping, so one
    /// interaction state that was checked out and not yet returned stalled every completion behind
    /// it — including the `ResumeDispatch`/`ResumeFrameDeferred` that carries the state back whenever
    /// anything had already been pushed in front of it (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
    /// `📓️wgpu-server-input-present-2026-09-13.md` §5.2). Order among the completions that DO need the
    /// state is preserved — only work that needs none is admitted past them.
    pub(crate) fn first_applicable(&self, interaction_available: bool) -> Option<usize> {
        if interaction_available {
            return (!self.ready.is_empty()).then_some(0);
        }
        self.ready.iter().position(|completion| !completion.requires_interaction)
    }

    pub(crate) fn take_at(&mut self, index: usize) -> Option<Completion<T>> {
        self.ready.remove(index)
    }

    pub(crate) fn restore_at(&mut self, index: usize, completion: Completion<T>) {
        self.ready.insert(index.min(self.ready.len()), completion);
    }
}

/// 🎟️ Apply opportunities one interaction checkout may span before it is reported as a defect.
///
/// ⚖️ One opportunity is one `apply_pending_step` that found the head blocked, i.e. at most one per
/// frame-build advance. A legitimate checkout (one dispatched event, one deferred action) returns in
/// a handful; this bound is two orders of magnitude above that so it can only name a real leak.
pub(crate) const INTERACTION_CHECKOUT_CREDITS: u32 = 240;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum InteractionCheckoutStep {
    Admitted,
    Deferred,
    Stale,
}

/// 🎟️ Who owns the single interaction state right now, and for how long.
#[derive(Default)]
pub(crate) struct InteractionCheckoutLedger {
    site: Option<&'static str>,
    opportunities: u32,
    stale: bool,
    notified: bool,
}

impl InteractionCheckoutLedger {
    pub(crate) fn check_out(&mut self, site: &'static str) -> bool {
        if self.site.is_some() {
            return false;
        }
        self.site = Some(site);
        self.opportunities = 0;
        self.stale = false;
        self.notified = false;
        true
    }

    pub(crate) fn check_in(&mut self) -> Option<&'static str> {
        self.opportunities = 0;
        self.stale = false;
        self.notified = false;
        self.site.take()
    }

    pub(crate) fn admit(&mut self, head_requires_interaction: bool, interaction_available: bool) -> InteractionCheckoutStep {
        if interaction_available || !head_requires_interaction {
            return InteractionCheckoutStep::Admitted;
        }
        self.opportunities = self.opportunities.saturating_add(1);
        if self.opportunities <= INTERACTION_CHECKOUT_CREDITS {
            return InteractionCheckoutStep::Deferred;
        }
        self.stale = true;
        InteractionCheckoutStep::Stale
    }

    /// 🩺️ Fires once per stale episode so the diagnostic is published, never repeated per frame.
    pub(crate) fn take_stale_notice(&mut self) -> Option<(&'static str, u32)> {
        if !self.stale || self.notified {
            return None;
        }
        self.notified = true;
        Some((self.site.unwrap_or("unknown"), self.opportunities))
    }

    pub(crate) fn site(&self) -> Option<&'static str> {
        self.site
    }

    pub(crate) fn opportunities(&self) -> u32 {
        self.opportunities
    }
}

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-runtime-mailbox-core-unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "../../../🧪️tests/🎮️wgpu-runtime-mailbox-admission/🦀️.rs"]
mod admission_laws;
