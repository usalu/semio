//! 🛠️ Reservation construction: the construction slots, claims and hand-backs of the replay reservation. See [`super`].

use super::*;

//#region 🔖️Ids
//#region 🔖️HistoryReplay
#[derive(Debug, PartialEq, Eq)]
pub(super) struct HistoryReplayReservationConstructionToken {
    pub slot: usize,
    pub generation: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct HistoryReplayReservationConstructionHandbackRejection {
    pub slot: usize,
    pub generation: u64,
}

struct HistoryReplayReservationConstructionSlot {
    pub generation: u64,
    occupied: bool,
    checked_out: bool,
    pub error: Option<DbError>,
    pub cursor: Option<HistoryReplayReservationCloseCursor>,
}

impl Default for HistoryReplayReservationConstructionSlot {
    fn default() -> Self {
        Self { generation: 0, occupied: false, checked_out: false, error: None, cursor: None }
    }
}

struct HistoryReplayReservationConstructionRegistry {
    pub slots: [HistoryReplayReservationConstructionSlot; HISTORY_REPLAY_CONSTRUCTION_SLOTS],
    pub(super) next_generation: u64,
}

pub(super) fn history_replay_reservation_construction_registry() -> &'static std::sync::Mutex<HistoryReplayReservationConstructionRegistry> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<HistoryReplayReservationConstructionRegistry>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(HistoryReplayReservationConstructionRegistry { slots: std::array::from_fn(|_| HistoryReplayReservationConstructionSlot::default()), next_generation: 1 }))
}

pub(super) fn claim_history_replay_reservation_construction() -> Result<HistoryReplayReservationConstructionToken, DbError> {
    let mut registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(slot) = registry.slots.iter().position(|slot| !slot.occupied) else {
        return Err(DbError::Unavailable("history construction registry capacity exhausted".to_string()));
    };
    let generation = registry.next_generation;
    let Some(next_generation) = generation.checked_add(1) else {
        return Err(DbError::LimitExceeded("history construction generation"));
    };
    registry.next_generation = next_generation;
    registry.slots[slot] = HistoryReplayReservationConstructionSlot {
        generation,
        occupied: true,
        checked_out: true,
        error: None,
        cursor: Some(HistoryReplayReservationCloseCursor {
            source_pages: Some(Vec::new()),
            source_page_count: 0,
            result_pages: Some(Vec::new()),
            operation_ids: Some(Vec::new()),
            entries: Some(Vec::new()),
            scratch: Some(Vec::new()),
            retained_operation_bytes: 0,
            retained_result_bytes: 0,
            started: false,
        }),
    };
    Ok(HistoryReplayReservationConstructionToken { slot, generation })
}

pub(super) fn handback_history_replay_reservation_construction(token: &HistoryReplayReservationConstructionToken) -> Result<(), HistoryReplayReservationConstructionHandbackRejection> {
    let mut registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(slot) = registry.slots.get_mut(token.slot) else {
        return Err(HistoryReplayReservationConstructionHandbackRejection { slot: token.slot, generation: token.generation });
    };
    if !slot.occupied || !slot.checked_out || slot.generation != token.generation || slot.cursor.is_none() {
        return Err(HistoryReplayReservationConstructionHandbackRejection { slot: token.slot, generation: token.generation });
    }
    slot.checked_out = false;
    Ok(())
}

fn release_history_replay_reservation_construction(token: &HistoryReplayReservationConstructionToken) -> bool {
    let mut registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(slot) = registry.slots.get_mut(token.slot) else {
        return false;
    };
    if !slot.occupied || !slot.checked_out || slot.generation != token.generation || slot.error.is_some() || slot.cursor.is_some() {
        return false;
    }
    slot.generation = 0;
    slot.occupied = false;
    slot.checked_out = false;
    true
}

#[cfg(test)]
pub(crate) fn take_history_replay_reservation_construction_fault(generation: u64) -> Option<HistoryReplayReservationConstructionFault> {
    let mut registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let slot_index = registry.slots.iter().position(|slot| slot.occupied && !slot.checked_out && slot.generation == generation)?;
    let slot = &mut registry.slots[slot_index];
    slot.checked_out = true;
    Some(HistoryReplayReservationConstructionFault { token: Some(HistoryReplayReservationConstructionToken { slot: slot_index, generation }), unregistered_error: None })
}

pub(super) struct HistoryReplayReservationConstructionBuilder {
    pub token: Option<HistoryReplayReservationConstructionToken>,
}

impl HistoryReplayReservationConstructionBuilder {
    pub fn new() -> Result<Self, HistoryReplayReservationConstructionFault> {
        let token = match claim_history_replay_reservation_construction() {
            Ok(token) => token,
            Err(error) => return Err(HistoryReplayReservationConstructionFault::unregistered(error)),
        };
        Ok(Self { token: Some(token) })
    }

    pub(super) fn edit_cursor<T>(&mut self, edit: impl FnOnce(&mut HistoryReplayReservationCloseCursor) -> T) -> Option<T> {
        let token = self.token.as_ref()?;
        let mut registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = registry.slots.get_mut(token.slot)?;
        if !slot.occupied || !slot.checked_out || slot.generation != token.generation || slot.error.is_some() {
            return None;
        }
        slot.cursor.as_mut().map(edit)
    }

    pub(super) fn read_cursor<T>(&self, read: impl FnOnce(&HistoryReplayReservationCloseCursor) -> T) -> Option<T> {
        let token = self.token.as_ref()?;
        let registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = registry.slots.get(token.slot)?;
        if !slot.occupied || !slot.checked_out || slot.generation != token.generation || slot.error.is_some() {
            return None;
        }
        slot.cursor.as_ref().map(read)
    }

    pub fn fault(mut self, error: impl FnOnce() -> DbError) -> HistoryReplayReservationConstructionFault {
        let Some(token) = self.token.take() else {
            return HistoryReplayReservationConstructionFault::unregistered(DbError::Unavailable("history construction token was unavailable".to_string()));
        };
        {
            let mut registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(slot) = registry.slots.get_mut(token.slot) else {
                return HistoryReplayReservationConstructionFault { token: Some(token), unregistered_error: None };
            };
            if slot.occupied && slot.checked_out && slot.generation == token.generation && slot.error.is_none() && slot.cursor.is_some() {
                slot.error = Some(error());
            }
        }
        HistoryReplayReservationConstructionFault { token: Some(token), unregistered_error: None }
    }

    pub fn finish(mut self, retained_operation_bytes: u64, retained_result_bytes: u64) -> Result<HistoryReplayReservation, HistoryReplayReservationConstructionFault> {
        let Some(token) = self.token.take() else {
            return Err(HistoryReplayReservationConstructionFault::unregistered(DbError::Unavailable("history construction token was unavailable".to_string())));
        };
        let reservation = {
            let mut registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(slot) = registry.slots.get_mut(token.slot) else {
                return Err(HistoryReplayReservationConstructionFault { token: Some(token), unregistered_error: None });
            };
            if !slot.occupied || !slot.checked_out || slot.generation != token.generation || slot.error.is_some() {
                return Err(HistoryReplayReservationConstructionFault { token: Some(token), unregistered_error: None });
            }
            let Some(mut cursor) = slot.cursor.take() else {
                return Err(HistoryReplayReservationConstructionFault { token: Some(token), unregistered_error: None });
            };
            cursor.retained_operation_bytes = retained_operation_bytes;
            cursor.retained_result_bytes = retained_result_bytes;
            let Some(reservation) = cursor.resume() else {
                slot.cursor = Some(cursor);
                return Err(HistoryReplayReservationConstructionFault { token: Some(token), unregistered_error: None });
            };
            slot.generation = 0;
            slot.occupied = false;
            slot.checked_out = false;
            reservation
        };
        Ok(reservation)
    }
}

impl Drop for HistoryReplayReservationConstructionBuilder {
    fn drop(&mut self) {
        if let Some(token) = self.token.as_ref() {
            let mut registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(slot) = registry.slots.get_mut(token.slot).filter(|slot| slot.occupied && slot.checked_out && slot.generation == token.generation && slot.error.is_none() && slot.cursor.is_some()) {
                slot.error = Some(DbError::Unavailable("history reservation construction unwound".to_string()));
            }
        }
        if let Some(token) = self.token.take() {
            let _ = handback_history_replay_reservation_construction(&token);
        }
    }
}

pub struct HistoryReplayReservationConstructionFault {
    pub token: Option<HistoryReplayReservationConstructionToken>,
    pub(super) unregistered_error: Option<DbError>,
}

impl HistoryReplayReservationConstructionFault {
    fn unregistered(error: DbError) -> Self {
        Self { token: None, unregistered_error: Some(error) }
    }

    pub fn generation(&self) -> Option<u64> {
        self.token.as_ref().map(|token| token.generation)
    }

    pub fn take_error(&mut self) -> Option<DbError> {
        if self.unregistered_error.is_some() {
            return self.unregistered_error.take();
        }
        let token = self.token.as_ref()?;
        let mut registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = registry.slots.get_mut(token.slot)?;
        if !slot.occupied || !slot.checked_out || slot.generation != token.generation {
            return None;
        }
        slot.error.take()
    }

    pub fn close_step(&mut self) -> bool {
        if self.unregistered_error.take().is_some() {
            return true;
        }
        let Some(token) = self.token.as_ref() else {
            return false;
        };
        {
            let mut registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(slot) = registry.slots.get_mut(token.slot) else {
                return false;
            };
            if !slot.occupied || !slot.checked_out || slot.generation != token.generation {
                return false;
            }
            if let Some(cursor) = slot.cursor.as_mut() {
                if cursor.close_step() {
                    return true;
                }
                if cursor.terminal_is_empty() {
                    slot.cursor = None;
                    return true;
                }
            }
            if slot.error.take().is_some() {
                return true;
            }
        }
        if release_history_replay_reservation_construction(token) {
            self.token = None;
            return true;
        }
        false
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.token.is_none() && self.unregistered_error.is_none()
    }

    #[cfg(test)]
    pub(super) fn retained_result_page_count(&self) -> usize {
        let Some(token) = self.token.as_ref() else {
            return 0;
        };
        let registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        registry.slots.get(token.slot).filter(|slot| slot.occupied && slot.generation == token.generation).and_then(|slot| slot.cursor.as_ref()).and_then(|cursor| cursor.result_pages.as_ref()).map_or(0, Vec::len)
    }

    #[cfg(test)]
    pub(super) fn retained_result_page_pointer(&self, index: usize) -> Option<*const u8> {
        let token = self.token.as_ref()?;
        let registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        registry
            .slots
            .get(token.slot)
            .filter(|slot| slot.occupied && slot.generation == token.generation)
            .and_then(|slot| slot.cursor.as_ref())
            .and_then(|cursor| cursor.result_pages.as_ref())
            .and_then(|owners| owners.get(index))
            .and_then(Option::as_ref)
            .map(Vec::as_ptr)
    }

    #[cfg(test)]
    pub(super) fn retained_error_pointer(&self) -> Option<*const u8> {
        if let Some(DbError::Unavailable(message)) = self.unregistered_error.as_ref() {
            return Some(message.as_ptr());
        }
        let token = self.token.as_ref()?;
        let registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        match registry.slots.get(token.slot).filter(|slot| slot.occupied && slot.generation == token.generation)?.error.as_ref()? {
            DbError::Unavailable(message) => Some(message.as_ptr()),
            _ => None,
        }
    }
}

impl std::fmt::Debug for HistoryReplayReservationConstructionFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("HistoryReplayReservationConstructionFault").field("generation", &self.generation()).field("terminal_is_empty", &self.terminal_is_empty()).finish()
    }
}

impl Drop for HistoryReplayReservationConstructionFault {
    fn drop(&mut self) {
        if let Some(token) = self.token.take() {
            let _ = handback_history_replay_reservation_construction(&token);
        }
    }
}

impl Drop for HistoryReplayReservationCloseCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "history replay reservation reached Drop before retained retirement or exact resume");
    }
}
//#endregion 🔖️HistoryReplay
//#endregion 🔖️Ids
