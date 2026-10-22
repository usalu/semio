//! 🪣️ Pending raster upload queues, surfaces, upload cursors and authority close.

use super::*;

//#region SceneRuntime
const RASTER_SURFACE_CAPACITY: usize = 256;
pub const RASTER_UPLOADS_PER_SURFACE_CAPACITY: usize = 16;
pub const RASTER_UPLOAD_KEY_BYTE_CAPACITY: usize = 256;
pub const RASTER_UPLOAD_BYTE_CAPACITY: usize = 1024 * 1024;
/// 🖼️ Per-image refusals one surface parks for silent retirement between two upload-cursor passes; admission keeps room for one.
const RASTER_REFUSALS_PER_SURFACE: usize = 4;

pub struct PendingRasterQueue {
    pub slots: Box<[Option<PreparedRasterProducer>; RASTER_UPLOADS_PER_SURFACE_CAPACITY]>,
    pub(crate) epochs: [u64; RASTER_UPLOADS_PER_SURFACE_CAPACITY],
    pub head: u8,
    pub len: u8,
    pub(crate) checked_out: Option<PendingRasterQueueToken>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PendingRasterQueueToken {
    pub slot: u8,
    pub epoch: u64,
}

impl Default for PendingRasterQueue {
    fn default() -> Self {
        Self { slots: semio_framework_async_slots::boxed_fixed_slots(|| None), epochs: [0; RASTER_UPLOADS_PER_SURFACE_CAPACITY], head: 0, len: 0, checked_out: None }
    }
}

impl PendingRasterQueue {
    pub fn len(&self) -> usize {
        usize::from(self.len)
    }

    fn is_full(&self) -> bool {
        self.len() == RASTER_UPLOADS_PER_SURFACE_CAPACITY
    }

    pub fn push_back(&mut self, producer: PreparedRasterProducer) -> Result<(), PreparedRasterProducer> {
        if self.is_full() {
            return Err(producer);
        }
        let index = (usize::from(self.head) + self.len()) % RASTER_UPLOADS_PER_SURFACE_CAPACITY;
        self.epochs[index] = self.epochs[index].wrapping_add(1).max(1);
        self.slots[index] = Some(producer);
        self.len += 1;
        Ok(())
    }

    pub fn checkout_front(&mut self) -> Option<PendingRasterQueueToken> {
        if self.len == 0 || self.checked_out.is_some() {
            return None;
        }
        let index = usize::from(self.head);
        self.slots[index].as_ref()?;
        let token = PendingRasterQueueToken { slot: self.head, epoch: self.epochs[index] };
        self.checked_out = Some(token);
        Some(token)
    }

    pub fn take_checked_out(&mut self, token: PendingRasterQueueToken) -> Option<PreparedRasterProducer> {
        if self.checked_out != Some(token) || self.head != token.slot || self.epochs[usize::from(token.slot)] != token.epoch {
            return None;
        }
        let producer = self.slots[usize::from(token.slot)].take()?;
        self.checked_out = None;
        let index = usize::from(self.head);
        self.head = ((index + 1) % RASTER_UPLOADS_PER_SURFACE_CAPACITY) as u8;
        self.len -= 1;
        Some(producer)
    }

    pub fn hand_back(&mut self, token: PendingRasterQueueToken) -> bool {
        if self.checked_out != Some(token) || self.head != token.slot || self.epochs[usize::from(token.slot)] != token.epoch || self.slots[usize::from(token.slot)].is_none() {
            return false;
        }
        self.checked_out = None;
        true
    }

    fn pop_front_for_close(&mut self) -> Option<PreparedRasterProducer> {
        if self.checked_out.is_some() {
            return None;
        }
        let token = self.checkout_front()?;
        self.take_checked_out(token)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn close_all(&mut self) {
        while let Some(mut producer) = self.pop_front_for_close() {
            producer.begin_close();
            while !producer.close_step() {}
        }
    }
}

#[derive(Default)]
pub struct PendingRasterSurface {
    pub queue: PendingRasterQueue,
    pub admission: Option<PreparedRasterReservation>,
    pub rejected: Option<PreparedRasterRejected>,
    #[cfg(any(test, feature = "testkit"))]
    #[doc(hidden)]
    pub retiring: Option<PreparedRasterRejected>,
    #[cfg(not(any(test, feature = "testkit")))]
    pub(crate) retiring: Option<PreparedRasterRejected>,
    pub closing: Option<PreparedRasterProducer>,
    /// 🖼️ Per-image refusals awaiting silent bounded retirement ([`PendingRasterSurface::park_refusal`]).
    #[cfg(any(test, feature = "testkit"))]
    #[doc(hidden)]
    pub refused: [Option<PreparedRasterRejected>; RASTER_REFUSALS_PER_SURFACE],
    #[cfg(not(any(test, feature = "testkit")))]
    pub(crate) refused: [Option<PreparedRasterRejected>; RASTER_REFUSALS_PER_SURFACE],
}

impl PendingRasterSurface {
    /// 🚦️ Whether one more image may be admitted: a free FIFO slot, no admission in flight, no authority owner retained or
    /// retiring, and room in the refusal ring for the refusal the admission could end in.
    pub fn admits_upload(&self) -> bool {
        !self.queue.is_full() && self.admission.is_none() && self.rejected.is_none() && self.retiring.is_none() && self.closing.is_none() && self.refused.iter().any(Option::is_none)
    }

    /// 🖼️ Parks one refused owner for silent retirement and answers the typed outcome; admission reserved the ring slot (a full
    /// ring falls back to the authority slot).
    pub fn park_refusal(&mut self, refusal: PreparedRasterRejected) -> RasterUploadRefusal {
        let outcome = if refusal.is_content_refusal() { RasterUploadRefusal::Invalid(refusal.fault()) } else { RasterUploadRefusal::Busy };
        match self.refused.iter_mut().find(|slot| slot.is_none()) {
            Some(slot) => *slot = Some(refusal),
            None => self.rejected = Some(refusal),
        }
        outcome
    }

    /// 🖼️ One bounded retirement step of the first parked refusal; `false` when the ring is empty.
    pub fn retire_refusal_step(&mut self) -> bool {
        let Some(slot) = self.refused.iter_mut().find(|slot| slot.is_some()) else { return false };
        if slot.as_mut().is_some_and(PreparedRasterRejected::close_step) {
            debug_assert!(slot.as_ref().is_some_and(PreparedRasterRejected::terminal_is_empty), "a retired refusal is terminal-empty");
            *slot = None;
        }
        true
    }
}

pub enum PendingRasterUploadStep {
    Pending,
    Upload(PendingRasterCheckedOut),
    Complete,
    Fault(&'static str),
}

pub struct PendingRasterCheckedOut {
    pub surface: AdmittedSurfaceToken,
    pub queue: PendingRasterQueueToken,
    pub(crate) active: bool,
}

impl PendingRasterCheckedOut {
    pub fn take(mut self) -> Result<PreparedRasterProducer, Self> {
        let producer = PENDING_RASTER_STATE.with(|cell| cell.borrow_mut().get_token_mut(self.surface).and_then(|surface| surface.queue.take_checked_out(self.queue)));
        let Some(producer) = producer else { return Err(self) };
        self.active = false;
        Ok(producer)
    }
}

impl Drop for PendingRasterCheckedOut {
    fn drop(&mut self) {
        if !self.active {
            return;
        }
        let returned = PENDING_RASTER_STATE.with(|cell| cell.borrow_mut().get_token_mut(self.surface).is_some_and(|surface| surface.queue.hand_back(self.queue)));
        debug_assert!(returned || std::thread::panicking(), "checked-out raster producer must return to its exact FIFO slot");
    }
}

#[derive(Default)]
pub struct PendingRasterUploadCursor {
    surface_index: usize,
    pub closing: Option<PreparedRasterProducer>,
}

impl PendingRasterUploadCursor {
    pub fn step(&mut self) -> PendingRasterUploadStep {
        if let Some(producer) = self.closing.as_mut() {
            if !producer.close_step() {
                return PendingRasterUploadStep::Pending;
            }
            self.closing = None;
            return PendingRasterUploadStep::Fault("raster producer handoff was rejected");
        }
        PENDING_RASTER_STATE.with(|cell| {
            let mut states = cell.borrow_mut();
            if states.len() > RASTER_SURFACE_CAPACITY {
                return PendingRasterUploadStep::Fault("raster surface credits exceeded");
            }
            if let Some(fault) = states.take_fault() {
                return PendingRasterUploadStep::Fault(fault);
            }
            let Some(surface_id) = states.id_at(self.surface_index).map(str::to_owned) else { return PendingRasterUploadStep::Complete };
            let Some(surface_token) = states.token(&surface_id) else { return PendingRasterUploadStep::Fault("raster surface token lost ownership") };
            let Some(state) = states.get_mut(&surface_id) else { return PendingRasterUploadStep::Fault("raster surface order lost ownership") };
            if let Some(reservation) = state.admission.take() {
                state.rejected = Some(reservation.reject("raster reservation was abandoned before publication", Vec::new()));
                return PendingRasterUploadStep::Pending;
            }
            if state.retire_refusal_step() {
                return PendingRasterUploadStep::Pending;
            }
            if let Some(retiring) = state.retiring.as_mut() {
                if !retiring.close_step() {
                    return PendingRasterUploadStep::Pending;
                }
                assert!(retiring.terminal_is_empty(), "retired raster reservation must be terminal-empty");
                state.retiring = None;
                return PendingRasterUploadStep::Pending;
            }
            if let Some(rejected) = state.rejected.as_mut() {
                let fault = rejected.fault();
                if !rejected.close_step() {
                    return PendingRasterUploadStep::Pending;
                }
                assert!(rejected.terminal_is_empty(), "rejected raster reservation must be terminal-empty");
                state.rejected = None;
                return PendingRasterUploadStep::Fault(fault);
            }
            if let Some(producer) = state.closing.as_mut() {
                if !producer.close_step() {
                    return PendingRasterUploadStep::Pending;
                }
                state.closing = None;
                return PendingRasterUploadStep::Fault("raster producer admission changed before FIFO publication");
            }
            if let Some(queue) = state.queue.checkout_front() {
                return PendingRasterUploadStep::Upload(PendingRasterCheckedOut { surface: surface_token, queue, active: true });
            }
            self.surface_index += 1;
            PendingRasterUploadStep::Pending
        })
    }

    pub fn retain_rejected(&mut self, mut producer: PreparedRasterProducer) -> Result<(), PreparedRasterProducer> {
        if self.closing.is_some() {
            return Err(producer);
        }
        producer.begin_close();
        self.closing = Some(producer);
        Ok(())
    }

    pub fn close_step(&mut self) -> bool {
        let Some(producer) = self.closing.as_mut() else { return true };
        if !producer.close_step() {
            return false;
        }
        self.closing = None;
        true
    }
}

pub(crate) struct PendingRasterSurfaceRetirement {
    pub(crate) id: String,
    pub(crate) surface: PendingRasterSurface,
    producer: Option<PreparedRasterProducer>,
    id_released: bool,
    scalars_released: bool,
}

impl PendingRasterSurfaceRetirement {
    pub(crate) fn new(owner: AdmittedSurfaceCloseOwner<PendingRasterSurface>) -> Self {
        Self { id: owner.id, surface: owner.value, producer: None, id_released: false, scalars_released: false }
    }

    pub(crate) fn close_step(&mut self) -> bool {
        if let Some(producer) = self.producer.as_mut() {
            if !producer.close_step() {
                return false;
            }
            assert!(producer.terminal_is_empty(), "realm-retired raster producer must be terminal-empty");
            self.producer = None;
            return false;
        }
        if let Some(retiring) = self.surface.retiring.as_mut() {
            if !retiring.close_step() {
                return false;
            }
            assert!(retiring.terminal_is_empty(), "realm-retired raster reservation must be terminal-empty");
            self.surface.retiring = None;
            return false;
        }
        if let Some(reservation) = self.surface.admission.take() {
            self.surface.retiring = Some(reservation.reject("realm closed pending raster reservation", Vec::new()));
            return false;
        }
        if self.surface.retire_refusal_step() {
            return false;
        }
        if let Some(rejected) = self.surface.rejected.as_mut() {
            if !rejected.close_step() {
                return false;
            }
            assert!(rejected.terminal_is_empty(), "realm-rejected raster reservation must be terminal-empty");
            self.surface.rejected = None;
            return false;
        }
        if let Some(mut producer) = self.surface.closing.take() {
            producer.begin_close();
            self.producer = Some(producer);
            return false;
        }
        if self.surface.queue.checked_out.is_some() {
            return false;
        }
        if let Some(mut producer) = self.surface.queue.pop_front_for_close() {
            producer.begin_close();
            self.producer = Some(producer);
            return false;
        }
        if self.id.pop().is_some() {
            return false;
        }
        if !self.id_released {
            self.id = String::new();
            self.id_released = true;
            return false;
        }
        if !self.scalars_released {
            self.surface.queue.head = 0;
            self.surface.queue.len = 0;
            self.surface.queue.checked_out = None;
            self.surface.queue.epochs = [0; RASTER_UPLOADS_PER_SURFACE_CAPACITY];
            self.scalars_released = true;
            return false;
        }
        true
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.id.is_empty()
            && self.id.capacity() == 0
            && self.id_released
            && self.producer.is_none()
            && self.surface.retiring.is_none()
            && self.surface.admission.is_none()
            && self.surface.rejected.is_none()
            && self.surface.refused.iter().all(Option::is_none)
            && self.surface.closing.is_none()
            && self.surface.queue.len == 0
            && self.surface.queue.checked_out.is_none()
            && self.surface.queue.slots.iter().all(Option::is_none)
            && self.scalars_released
    }
}

pub struct PendingRasterAuthorityClose {
    complete: bool,
}

pub fn begin_pending_raster_authority_close() -> PendingRasterAuthorityClose {
    PENDING_RASTER_STATE.with(|cell| cell.borrow_mut().begin_close());
    PendingRasterAuthorityClose { complete: false }
}

impl PendingRasterAuthorityClose {
    pub fn close_step(&mut self) -> bool {
        if self.complete {
            return true;
        }
        let active = PENDING_RASTER_CLOSE_OWNER.with(|cell| {
            let mut owner = cell.borrow_mut();
            let Some(surface) = owner.as_mut() else { return false };
            if !surface.close_step() {
                return true;
            }
            assert!(surface.terminal_is_empty(), "pending raster surface must be terminal-empty before realm release");
            *owner = None;
            true
        });
        if active {
            return false;
        }
        let owner = PENDING_RASTER_STATE.with(|cell| cell.borrow_mut().close_step());
        if let Some(owner) = owner {
            PENDING_RASTER_CLOSE_OWNER.with(|cell| {
                let mut retained = cell.borrow_mut();
                assert!(retained.is_none(), "one pending raster close owner is admitted at a time");
                *retained = Some(PendingRasterSurfaceRetirement::new(owner));
            });
            return false;
        }
        self.complete = PENDING_RASTER_STATE.with(|cell| cell.borrow().terminal_is_empty()) && PENDING_RASTER_CLOSE_OWNER.with(|cell| cell.borrow().is_none());
        self.complete
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.complete && PENDING_RASTER_STATE.with(|cell| cell.borrow().terminal_is_empty()) && PENDING_RASTER_CLOSE_OWNER.with(|cell| cell.borrow().is_none())
    }
}
//#endregion SceneRuntime
