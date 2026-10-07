//! 🎮️ Moves ordered native list slots one admitted step at a time.

use super::{PagedList, PagedListError, PagedListProgress, PagedListRefusalKind};
use std::mem::size_of;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PagedListEditStep {
    pub complete: bool,
    pub moved_items: usize,
    pub progress: PagedListProgress,
}

pub struct PagedListEditCursor {
    insertion: bool,
    target: usize,
    position: usize,
    expected_length: usize,
    owner: Option<usize>,
    started: bool,
    complete: bool,
}

impl PagedListEditCursor {
    pub fn insert(index: usize, original_length: usize) -> Self {
        Self { insertion: true, target: index, position: original_length, expected_length: original_length, owner: None, started: false, complete: false }
    }

    pub fn remove(index: usize, original_length: usize) -> Self {
        Self { insertion: false, target: index, position: index, expected_length: original_length, owner: None, started: false, complete: false }
    }

    pub fn is_finished(&self) -> bool { self.complete }

    pub fn step<T, const N: usize>(&mut self, owner: &mut PagedList<T, N>, item: &mut Option<T>, maximum_items: usize, maximum_bytes: usize) -> Result<PagedListEditStep, PagedListError> {
        let refuse = |reason| PagedListError { kind: PagedListRefusalKind::InvariantViolated, reason };
        let address = owner as *mut PagedList<T, N> as usize;
        if self.owner.is_some_and(|expected| expected != address) || owner.len() != self.expected_length {
            return Err(refuse("ordered list edit owner or length changed"));
        }
        if self.complete { return Ok(PagedListEditStep { complete: true, ..Default::default() }); }
        if self.target > owner.len() || (!self.insertion && self.target == owner.len()) {
            return Err(refuse("ordered list edit index is outside its owner"));
        }
        if maximum_items == 0 { return Ok(Default::default()); }
        self.owner = Some(address);
        if self.insertion && !self.started {
            if item.is_none() { return Err(refuse("ordered list insertion lost its source owner")); }
            if !owner.has_reserved_slot() {
                let progress = owner.reserve_one(maximum_bytes).map_err(|error| error.refusal())?;
                return Ok(PagedListEditStep { moved_items: usize::from(progress.progressed), progress, complete: false });
            }
            let progress = owner.place_reserved(item, maximum_bytes)?;
            if progress.progressed {
                self.expected_length += 1;
                self.started = true;
                self.complete = self.position == self.target;
            }
            return Ok(PagedListEditStep { complete: self.complete, moved_items: usize::from(progress.progressed), progress });
        }
        if !self.insertion && item.is_some() { return Err(refuse("ordered list removal destination already owns an item")); }
        let shifting = if self.insertion { self.position > self.target } else { self.position + 1 < owner.len() };
        if shifting {
            let moved_bytes = size_of::<T>().checked_mul(3).ok_or_else(|| refuse("ordered list slot move byte count overflow"))?;
            if moved_bytes > maximum_bytes { return Ok(Default::default()); }
            if self.insertion { owner.swap(self.position - 1, self.position); self.position -= 1; }
            else { owner.swap(self.position, self.position + 1); self.position += 1; }
            self.started = true;
            self.complete = self.insertion && self.position == self.target;
            return Ok(PagedListEditStep { complete: self.complete, moved_items: 1, progress: PagedListProgress { progressed: true, placed_bytes: moved_bytes, ..Default::default() } });
        }
        if size_of::<T>() > maximum_bytes { return Ok(Default::default()); }
        *item = owner.pop();
        self.expected_length -= 1;
        self.complete = true;
        Ok(PagedListEditStep { complete: true, moved_items: 1, progress: PagedListProgress { progressed: true, placed_bytes: size_of::<T>(), ..Default::default() } })
    }
}
