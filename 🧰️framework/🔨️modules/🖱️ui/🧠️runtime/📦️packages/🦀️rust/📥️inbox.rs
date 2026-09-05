//! @emoji 📥️ The bounded, coalescing `ProjectionInbox`.
//!
//! 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md. Every `fn`
//! below is plain sync by owner ruling U1.
//!
//! This is the crate's COALESCING policy — the opposite of `🦀️gateway.rs`'s [`crate::gateway::
//! CommandGateway`], and both are correct for the shape they carry. A [`crate::gateway::Command`] is
//! durable: once accepted it is tracked until an explicit resolution, never silently dropped or
//! overwritten. A projection delta pushed here is the opposite kind of thing — two deltas that share a
//! [`ProjectionDelta::key`] describe the SAME addressable slot at two different moments, and once the
//! newer one has arrived the older one is not durable data worth keeping, it is a state a subscriber
//! never needs to see. So [`ProjectionInbox::push`] coalesces same-key pushes to the newest value in
//! place rather than queueing both — a burst of same-key projection updates between two drains costs
//! exactly one slot and one eventual delivery, never a growing backlog of stale intermediate states.

use std::collections::{HashMap, VecDeque};

//#region 🔖️Inbox

/// 🔑️ The addressable identity a projection delta targets. Two [`ProjectionInbox::push`] calls whose
/// deltas report equal keys coalesce — the second overwrites the first in place, keeping the first
/// call's queue position so drain order still reflects first-seen-key order.
pub trait ProjectionDelta {
    type Key: Eq + std::hash::Hash + Clone;

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn key(&self) -> Self::Key;
}

/// 🚧️ [`ProjectionInbox::push`] could not accept a delta for a brand-new key — the inbox already holds
/// `capacity` distinct keys and none of them is this delta's key to coalesce into.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InboxOverflow;

/// 📥️ A bounded queue of the most-recent delta per [`ProjectionDelta::Key`]. `capacity` bounds the
/// number of DISTINCT keys outstanding at once, not the number of `push` calls — same-key pushes never
/// count twice against it, since they coalesce.
pub struct ProjectionInbox<T: ProjectionDelta> {
    capacity: usize,
    order: VecDeque<T::Key>,
    entries: HashMap<T::Key, T>,
}

impl<T: ProjectionDelta> ProjectionInbox<T> {
    /// 🏭️ An inbox that holds at most `capacity` distinct keys at once.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new(capacity: usize) -> Self {
        Self { capacity, order: VecDeque::new(), entries: HashMap::new() }
    }

    /// 📤️ Coalesces `delta` into its key's slot if that key is already queued; otherwise enqueues it as
    /// a new slot, or returns [`InboxOverflow`] if `capacity` distinct keys are already outstanding.
    /// [`InboxOverflow`] never discards an already-queued entry — the new delta alone is refused.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn push(&mut self, delta: T) -> Result<(), InboxOverflow> {
        let key = delta.key();
        if !self.entries.contains_key(&key) {
            if self.order.len() >= self.capacity {
                return Err(InboxOverflow);
            }
            self.order.push_back(key.clone());
        }
        self.entries.insert(key, delta);
        Ok(())
    }

    /// 📦️ Moves up to `limit` of the oldest still-queued deltas into `out`, oldest key first. A
    /// transaction bounds its own work with `limit` — draining an unbounded backlog in one call is how
    /// one slow projection turns into a dropped frame — so any remainder past `limit` stays queued
    /// exactly as it was, ready for the next `drain_into` call.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn drain_into(&mut self, limit: usize, out: &mut Vec<T>) {
        for _ in 0..limit {
            let Some(key) = self.order.pop_front() else { break };
            if let Some(delta) = self.entries.remove(&key) {
                out.push(delta);
            }
        }
    }

    /// 🔢️ Distinct keys currently queued.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn len(&self) -> usize {
        self.order.len()
    }

    /// 🈳️ `true` when [`Self::len`] is zero.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn is_empty(&self) -> bool {
        self.order.is_empty()
    }
}
//#endregion 🔖️Inbox

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq)]
    struct Delta {
        key: u32,
        revision: u32,
    }

    impl ProjectionDelta for Delta {
        type Key = u32;
        fn key(&self) -> u32 {
            self.key
        }
    }

    #[test]
    fn push_beyond_capacity_returns_overflow_without_dropping_existing_entries() {
        let mut inbox = ProjectionInbox::new(2);
        inbox.push(Delta { key: 1, revision: 1 }).expect("fits");
        inbox.push(Delta { key: 2, revision: 1 }).expect("fits");

        assert_eq!(inbox.push(Delta { key: 3, revision: 1 }), Err(InboxOverflow));
        assert_eq!(inbox.len(), 2, "the refused delta must not have displaced the queued ones");

        let mut out = Vec::new();
        inbox.drain_into(10, &mut out);
        assert_eq!(out, vec![Delta { key: 1, revision: 1 }, Delta { key: 2, revision: 1 }]);
    }

    #[test]
    fn same_key_pushes_coalesce_to_the_newest_value() {
        let mut inbox = ProjectionInbox::new(1);
        inbox.push(Delta { key: 1, revision: 1 }).expect("fits");
        inbox.push(Delta { key: 1, revision: 2 }).expect("coalesces, does not consume a second slot");
        inbox.push(Delta { key: 1, revision: 3 }).expect("coalesces again");

        assert_eq!(inbox.len(), 1);
        let mut out = Vec::new();
        inbox.drain_into(10, &mut out);
        assert_eq!(out, vec![Delta { key: 1, revision: 3 }]);
    }

    #[test]
    fn drain_into_respects_limit_and_leaves_the_remainder_queued() {
        let mut inbox = ProjectionInbox::new(3);
        inbox.push(Delta { key: 1, revision: 1 }).expect("fits");
        inbox.push(Delta { key: 2, revision: 1 }).expect("fits");
        inbox.push(Delta { key: 3, revision: 1 }).expect("fits");

        let mut out = Vec::new();
        inbox.drain_into(2, &mut out);
        assert_eq!(out, vec![Delta { key: 1, revision: 1 }, Delta { key: 2, revision: 1 }]);
        assert_eq!(inbox.len(), 1, "the third delta must remain queued, not dropped");

        inbox.drain_into(10, &mut out);
        assert_eq!(out, vec![Delta { key: 1, revision: 1 }, Delta { key: 2, revision: 1 }, Delta { key: 3, revision: 1 }]);
        assert!(inbox.is_empty());
    }

    #[test]
    fn drain_into_on_an_empty_inbox_is_a_no_op() {
        let mut inbox: ProjectionInbox<Delta> = ProjectionInbox::new(3);
        let mut out = Vec::new();
        inbox.drain_into(5, &mut out);
        assert!(out.is_empty());
    }
}
//#endregion 🧪️Tests
