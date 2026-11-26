//! 📥️ Keeps an original actor event until the kernel takes its donor on a funded crossing.
use std::collections::VecDeque;

pub(crate) struct PendingEvents<T, const LIMIT: usize> {
    head: Option<T>,
    queued: VecDeque<T>,
    serving: bool,
}

impl<T, const LIMIT: usize> PendingEvents<T, LIMIT> {
    pub(crate) const fn new() -> Self {
        Self { head: None, queued: VecDeque::new(), serving: false }
    }

    fn len(&self) -> usize { usize::from(self.head.is_some()) + self.queued.len() }

    pub(crate) fn held(&self) -> bool { self.serving || self.head.is_some() }

    pub(crate) fn begin(&mut self, incoming: Option<T>) -> Result<Option<T>, Option<T>> {
        if self.serving || (incoming.is_some() && self.len() >= LIMIT) { return Err(incoming); }
        if self.head.is_some() && incoming.is_some() && self.queued.try_reserve(1).is_err() { return Err(incoming); }
        self.serving = true;
        if self.head.is_none() { return Ok(incoming); }
        if let Some(event) = incoming { self.queued.push_back(event); }
        Ok(self.head.take())
    }

    pub(crate) fn finish(&mut self, donor: Option<T>) -> bool {
        self.head = donor.or_else(|| self.queued.pop_front());
        self.serving = false;
        self.head.is_some()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
