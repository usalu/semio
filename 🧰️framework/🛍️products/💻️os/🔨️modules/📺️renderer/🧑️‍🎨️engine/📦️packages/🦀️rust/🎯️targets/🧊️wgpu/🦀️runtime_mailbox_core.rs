//! 📮️ Target-neutral fixed-capacity renderer completion mailbox.
//!
//! Ready and in-flight work share one bound. Keyless work is lossless and rejects overflow;
//! replaceable keyed work may coalesce only a ready completion with the same key. One reserved slot
//! guarantees that an owned interaction state can always return from a suspended worker turn.

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

    pub(crate) fn finish(&mut self, completion: Completion<T>) {
        assert!(self.in_flight > 0, "runtime completion without reservation");
        self.in_flight -= 1;
        self.ready.push_front(completion);
        assert!(self.len() <= CAPACITY, "runtime completion mailbox capacity exceeded");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn completion(key: Option<&'static str>, revision: u64) -> Completion<()> {
        Completion { key, revision, requires_interaction: false, apply: () }
    }

    #[test]
    fn lossless_and_coalesced_work_share_a_fixed_bound_with_an_interaction_reserve() {
        let mut queue = BoundedCompletionQueue::<(), 4>::new();
        assert!(queue.enqueue(completion(None, 1)));
        assert!(queue.reserve(None));
        assert!(queue.reserve(None));
        assert!(!queue.enqueue(completion(None, 2)));
        assert!(queue.reserve_interaction());
        assert!(!queue.reserve_interaction());

        let mut queue = BoundedCompletionQueue::<(), 4>::new();
        assert!(queue.enqueue(completion(Some("preview"), 1)));
        assert!(queue.enqueue(completion(Some("preview"), 2)));
        assert!(queue.enqueue(completion(Some("preview"), 3)));
        assert!(queue.enqueue(completion(Some("preview"), 4)));
        assert_eq!(queue.len(), 3);
        assert_eq!(queue.ready.back().expect("latest preview").revision, 4);
    }
}
