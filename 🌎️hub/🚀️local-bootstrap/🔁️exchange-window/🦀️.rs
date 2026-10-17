//! 🔁️ The bounded replay window and the outstanding-request slots of exchange ids.

use super::*;

/// ⏳️ Exchange ids admitted inside their own validity window. An id whose `expiresAt` is past can
/// never validate again — every frame carrying it is refused by its own window — so it is evicted
/// instead of pinning a slot for the rest of the run.
pub(super) struct ExchangeWindow<const N: usize> {
    values: [Option<(String, i64)>; N],
}

impl<const N: usize> ExchangeWindow<N> {
    pub(super) fn new() -> Self {
        Self { values: std::array::from_fn(|_| None) }
    }

    pub(super) fn admit(&mut self, value: &str, expires_at: i64, now: i64) -> Result<(), LocalBootstrapRejectCode> {
        for slot in &mut self.values {
            if slot.as_ref().is_some_and(|(_, expiry)| *expiry < now) {
                *slot = None;
            }
        }
        if self.values.iter().flatten().any(|(existing, _)| existing == value) {
            return Err(LocalBootstrapRejectCode::Denied);
        }
        let slot = self.values.iter_mut().find(|slot| slot.is_none()).ok_or(LocalBootstrapRejectCode::ResourceLimit)?;
        *slot = Some((value.to_owned(), expires_at));
        Ok(())
    }
}

pub(super) struct ExchangeSlots<const N: usize> {
    values: [Option<String>; N],
}

impl<const N: usize> ExchangeSlots<N> {
    pub(super) fn new() -> Self {
        Self { values: std::array::from_fn(|_| None) }
    }

    pub(super) fn contains(&self, value: &str) -> bool {
        self.values.iter().flatten().any(|existing| existing == value)
    }

    pub(super) fn insert(&mut self, value: &str) -> bool {
        if self.contains(value) {
            return false;
        }
        let Some(slot) = self.values.iter_mut().find(|slot| slot.is_none()) else { return false };
        *slot = Some(value.to_string());
        true
    }

    pub(super) fn remove(&mut self, value: &str) -> bool {
        let Some(slot) = self.values.iter_mut().find(|slot| slot.as_deref() == Some(value)) else { return false };
        *slot = None;
        true
    }
}
