//! 🗃️ Ledgers and port of the ABI: the generation-checked handle table, the reply ledger and the port poll, rejection and trait. See [`super`].

use super::*;

//#region 🗃️Ledgers
pub(super) struct HandleSlot<T> {
    pub(super) generation: u32,
    pub(super) value: Option<T>,
}

/// 🏓️ Fixed-capacity owned handle table with generation-preserving slot reuse.
pub struct AbiHandleTable<T> {
    pub(super) slots: Vec<HandleSlot<T>>,
    free: Vec<usize>,
    quarantined: usize,
}

impl<T> Default for AbiHandleTable<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> AbiHandleTable<T> {
    pub const fn new() -> Self {
        Self { slots: Vec::new(), free: Vec::new(), quarantined: 0 }
    }

    pub fn open(&mut self, value: T) -> Result<AbiHandle, (AbiErrorCode, T)> {
        if let Some(index) = self.free.pop() {
            let slot = &mut self.slots[index];
            let Some(generation) = slot.generation.checked_add(1) else {
                self.quarantined += 1;
                return Err((AbiErrorCode::GenerationExhausted, value));
            };
            slot.generation = generation;
            slot.value = Some(value);
            return Ok(AbiHandle { slot: index as u32 + 1, generation: slot.generation });
        }
        if self.slots.len() == ABI_MAX_IN_FLIGHT_HANDLES {
            let code = if self.quarantined == 0 { AbiErrorCode::LimitExceeded } else { AbiErrorCode::GenerationExhausted };
            return Err((code, value));
        }
        self.slots.push(HandleSlot { generation: 1, value: Some(value) });
        Ok(AbiHandle { slot: self.slots.len() as u32, generation: 1 })
    }

    pub fn get(&self, handle: AbiHandle) -> Result<&T, AbiErrorCode> {
        let slot = self.slot(handle)?;
        slot.value.as_ref().ok_or(AbiErrorCode::UnknownHandle)
    }

    pub fn get_mut(&mut self, handle: AbiHandle) -> Result<&mut T, AbiErrorCode> {
        let slot = self.slot_mut(handle)?;
        slot.value.as_mut().ok_or(AbiErrorCode::UnknownHandle)
    }

    pub fn close(&mut self, handle: AbiHandle) -> Result<T, AbiErrorCode> {
        let index = handle.slot.checked_sub(1).ok_or(AbiErrorCode::UnknownHandle)? as usize;
        let slot = self.slots.get_mut(index).ok_or(AbiErrorCode::UnknownHandle)?;
        compare_generation(slot.generation, handle.generation)?;
        let value = slot.value.take().ok_or(AbiErrorCode::UnknownHandle)?;
        if slot.generation == u32::MAX {
            self.quarantined += 1;
        } else {
            self.free.push(index);
        }
        Ok(value)
    }

    pub fn lose(&mut self, handle: AbiHandle) -> Result<T, AbiErrorCode> {
        self.close(handle)
    }

    pub(super) fn slot(&self, handle: AbiHandle) -> Result<&HandleSlot<T>, AbiErrorCode> {
        let slot = handle.slot.checked_sub(1).and_then(|index| self.slots.get(index as usize)).ok_or(AbiErrorCode::UnknownHandle)?;
        compare_generation(slot.generation, handle.generation)?;
        Ok(slot)
    }

    fn slot_mut(&mut self, handle: AbiHandle) -> Result<&mut HandleSlot<T>, AbiErrorCode> {
        let slot = handle.slot.checked_sub(1).and_then(|index| self.slots.get_mut(index as usize)).ok_or(AbiErrorCode::UnknownHandle)?;
        compare_generation(slot.generation, handle.generation)?;
        Ok(slot)
    }
}

fn compare_generation(current: u32, actual: u32) -> Result<(), AbiErrorCode> {
    if actual < current {
        Err(AbiErrorCode::AbaHandle)
    } else if actual > current {
        Err(AbiErrorCode::StaleGeneration)
    } else {
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct ReplySlot {
    pub(super) request_id: AbiRequestId,
    pub(super) generation: u32,
    state: ReplyState,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ReplyState {
    Active,
    Completed,
    Lost,
}

/// 📬 Fixed-capacity correlation ledger rejecting duplicate and late completions.
pub struct AbiReplyLedger {
    pub(super) slots: [Option<ReplySlot>; ABI_MAX_IN_FLIGHT_REQUESTS],
}

impl Default for AbiReplyLedger {
    fn default() -> Self {
        Self::new()
    }
}

impl AbiReplyLedger {
    pub const fn new() -> Self {
        Self { slots: [None; ABI_MAX_IN_FLIGHT_REQUESTS] }
    }

    pub fn admit(&mut self, request_id: AbiRequestId, generation: u32) -> Result<(), AbiErrorCode> {
        let index = reply_slot_index(request_id);
        if let Some(slot) = self.slots[index].as_ref() {
            if slot.request_id != request_id && slot.state == ReplyState::Active {
                return Err(AbiErrorCode::Busy);
            }
            if slot.request_id == request_id && generation <= slot.generation {
                return Err(if generation == slot.generation && slot.state == ReplyState::Active { AbiErrorCode::Busy } else { AbiErrorCode::AbaHandle });
            }
        }
        self.slots[index] = Some(ReplySlot { request_id, generation, state: ReplyState::Active });
        Ok(())
    }

    pub fn accept(&mut self, reply: &AbiReply) -> Result<(), AbiErrorCode> {
        let slot = self.slots[reply_slot_index(reply.request_id)].as_mut().filter(|slot| slot.request_id == reply.request_id).ok_or(AbiErrorCode::LateReply)?;
        if slot.generation != reply.generation || slot.state == ReplyState::Lost {
            return Err(AbiErrorCode::LateReply);
        }
        if slot.state == ReplyState::Completed {
            return Err(AbiErrorCode::DuplicateReply);
        }
        slot.state = ReplyState::Completed;
        Ok(())
    }

    pub fn lose(&mut self, request_id: AbiRequestId, generation: u32) -> Result<(), AbiErrorCode> {
        let slot = self.slots[reply_slot_index(request_id)].as_mut().filter(|slot| slot.request_id == request_id && slot.generation == generation).ok_or(AbiErrorCode::UnknownHandle)?;
        slot.state = ReplyState::Lost;
        Ok(())
    }
}

fn reply_slot_index(request_id: AbiRequestId) -> usize {
    (request_id.0 % ABI_MAX_IN_FLIGHT_REQUESTS as u64) as usize
}
//#endregion 🗃️Ledgers

//#region 🔌️Port
/// 💤 Result of polling the future generated host shim through an owned port.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AbiPortPoll {
    Pending,
    Message(AbiMessage),
    Closed,
}

/// ↩️ Port admission failure returning the exact owned envelope.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AbiPortRejection {
    pub code: AbiErrorCode,
    pub message: AbiMessage,
}

/// 🔌 Primitive-only seam implemented later by native or generated JS host shims.
pub trait AbiPort {
    fn try_send(&mut self, message: AbiMessage, budget: AbiWorkBudget) -> Result<(), AbiPortRejection>;
    fn poll(&mut self, budget: AbiWorkBudget) -> Result<AbiPortPoll, AbiErrorCode>;
}
//#endregion 🔌️Port
