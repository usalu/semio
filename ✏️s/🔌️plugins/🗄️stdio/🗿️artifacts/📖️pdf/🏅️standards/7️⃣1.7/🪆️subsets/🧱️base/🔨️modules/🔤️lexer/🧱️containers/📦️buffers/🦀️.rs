//! 📦️ Bounded ownership for contiguous COS vectors.

use super::Turn;

pub(super) struct Growing<T> {
    current: Vec<T>,
    previous: Option<(std::vec::IntoIter<T>, usize)>,
}

impl<T> Growing<T> {
    pub(super) fn new() -> Self { Self::from_vec(Vec::new()) }
    pub(super) fn from_vec(current: Vec<T>) -> Self { Self { current, previous: None } }
    pub(super) fn into_vec(self) -> Vec<T> {
        assert!(self.previous.is_none());
        self.current
    }

    pub(super) fn len(&self) -> usize {
        self.current.len() + self.previous.as_ref().map_or(0, |(items, _)| items.len())
    }

    pub(super) fn prepare_push(&mut self, limit: usize, turn: &mut Turn) -> Result<bool, &'static str> {
        if let Some((items, bytes)) = &mut self.previous {
            if let Some(item) = items.next() {
                self.current.push(item);
            } else if turn.release(*bytes) {
                self.previous = None;
            }
            return Ok(false);
        }
        if self.len() >= limit { return Err("PDF COS item limit exceeded"); }
        if self.current.len() < self.current.capacity() { return Ok(true); }
        let count = self.current.capacity().max(1).saturating_mul(2).min(limit);
        let Some(next) = allocate::<T>(count, turn)? else { return Ok(false); };
        let previous = std::mem::replace(&mut self.current, next);
        if previous.capacity() != 0 {
            let bytes = previous.capacity() * size_of::<T>();
            self.previous = Some((previous.into_iter(), bytes));
        }
        Ok(false)
    }

    pub(super) fn push(&mut self, item: T) { self.current.push(item); }

    pub(super) fn last(&self) -> Option<&T> {
        self.current.last().or_else(|| self.previous.as_ref()?.0.as_slice().last())
    }

    pub(super) fn pop(&mut self) -> Option<T> {
        self.current.pop().or_else(|| self.previous.as_mut()?.0.next_back())
    }

    pub(super) fn retire_empty(&mut self, turn: &mut Turn) -> bool {
        if let Some((_, bytes)) = self.previous.as_ref() {
            if turn.release(*bytes) { self.previous = None; }
            return false;
        }
        if self.current.capacity() == 0 { return true; }
        if turn.release(self.current.capacity() * size_of::<T>()) {
            self.current = Vec::new();
        }
        false
    }
}

pub(super) fn allocate<T>(count: usize, turn: &mut Turn) -> Result<Option<Vec<T>>, &'static str> {
    let demand = count.checked_mul(size_of::<T>()).ok_or("PDF COS allocation overflow")?;
    if !turn.allocation_fits(demand) { return Ok(None); }
    let mut data = Vec::new();
    data.try_reserve_exact(count).map_err(|_| "PDF COS allocation failed")?;
    turn.allocated(data.capacity() * size_of::<T>());
    Ok(Some(data))
}
