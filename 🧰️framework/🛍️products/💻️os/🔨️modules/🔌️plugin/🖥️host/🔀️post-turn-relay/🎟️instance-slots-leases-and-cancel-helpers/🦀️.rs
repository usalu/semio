//! 🎟️ Instance slots, leases and cancel helpers: the guest instance slot, its lease with the drop, the quarantine, cleanup pending and cancel helpers. See [`super`].

use super::*;

//#region 🔀️PostTurnRelay
pub(super) enum GuestInstanceSlot {
    Available(GuestInstance),
    Leased,
    CleanupPending { instance: Option<GuestInstance>, detail: Vec<u8> },
    Quarantined { instance: GuestInstance, detail: Vec<u8> },
}

impl GuestInstanceSlot {
    #[cfg(any(test, feature = "testkit"))]
    pub(super) fn is_available(&self) -> bool {
        matches!(self, Self::Available(_))
    }

    #[cfg(any(test, feature = "testkit"))]
    pub(super) fn is_quarantined(&self) -> bool {
        matches!(self, Self::Quarantined { .. })
    }

    #[cfg(any(test, feature = "testkit"))]
    pub(super) fn is_cleanup_pending(&self) -> bool {
        matches!(self, Self::CleanupPending { .. })
    }

    pub(super) fn admission_fault(&self) -> Option<Vec<u8>> {
        match self {
            Self::CleanupPending { detail, .. } | Self::Quarantined { detail, .. } => Some(detail.clone()),
            Self::Leased => Some(b"plugin instance is busy retiring a prior relay".to_vec()),
            Self::Available(_) => None,
        }
    }
}

enum GuestInstanceLeaseOrigin {
    Mounted,
    Cleanup,
}

enum GuestInstanceLeaseDisposition {
    Available,
    CleanupResolved,
    CleanupPending(Vec<u8>),
    Quarantined(Vec<u8>),
}

pub(super) struct GuestInstanceLease {
    pub(super) slot: Arc<Mutex<GuestInstanceSlot>>,
    pub(super) guest: Option<GuestInstance>,
    origin: GuestInstanceLeaseOrigin,
    disposition: GuestInstanceLeaseDisposition,
}

impl GuestInstanceLease {
    pub(super) fn acquire(slot: Arc<Mutex<GuestInstanceSlot>>, unwind_detail: Vec<u8>) -> Result<Self, Vec<u8>> {
        let state = {
            let mut state = slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            std::mem::replace(&mut *state, GuestInstanceSlot::Leased)
        };
        match state {
            GuestInstanceSlot::Available(guest) => Ok(Self { slot, guest: Some(guest), origin: GuestInstanceLeaseOrigin::Mounted, disposition: GuestInstanceLeaseDisposition::CleanupPending(unwind_detail) }),
            GuestInstanceSlot::Leased => {
                *slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = GuestInstanceSlot::Leased;
                Err(b"plugin instance missing behind an acquired relay permit".to_vec())
            }
            pending @ GuestInstanceSlot::CleanupPending { .. } => {
                let error = pending.admission_fault().expect("cleanup-pending slot owns its detail");
                *slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = pending;
                Err(error)
            }
            GuestInstanceSlot::Quarantined { instance, detail } => {
                let error = detail.clone();
                *slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = GuestInstanceSlot::Quarantined { instance, detail };
                Err(error)
            }
        }
    }

    pub(super) fn acquire_cleanup(slot: Arc<Mutex<GuestInstanceSlot>>) -> Result<Self, Vec<u8>> {
        let state = {
            let mut state = slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            std::mem::replace(&mut *state, GuestInstanceSlot::Leased)
        };
        match state {
            GuestInstanceSlot::CleanupPending { instance: Some(guest), detail } => {
                *slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = GuestInstanceSlot::CleanupPending { instance: None, detail };
                Ok(Self { slot, guest: Some(guest), origin: GuestInstanceLeaseOrigin::Cleanup, disposition: GuestInstanceLeaseDisposition::Quarantined(b"plugin instance quarantined after cleanup cancel-job panic".to_vec()) })
            }
            state => {
                let error = state.admission_fault().unwrap_or_else(|| b"plugin instance was not cleanup-pending behind an acquired cleanup permit".to_vec());
                *slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = state;
                Err(error)
            }
        }
    }

    pub(crate) fn guest(&mut self) -> &mut GuestInstance {
        self.guest.as_mut().expect("guest instance lease owns its instance")
    }

    pub(crate) fn quarantine(&mut self, detail: Vec<u8>) {
        self.disposition = GuestInstanceLeaseDisposition::Quarantined(detail);
    }

    pub(super) fn cleanup_pending(&mut self, detail: Vec<u8>) {
        self.disposition = GuestInstanceLeaseDisposition::CleanupPending(detail);
    }

    pub(crate) fn available(&mut self) {
        self.disposition = GuestInstanceLeaseDisposition::Available;
    }

    fn cleanup_resolved(&mut self) {
        self.disposition = GuestInstanceLeaseDisposition::CleanupResolved;
    }
}

impl Drop for GuestInstanceLease {
    fn drop(&mut self) {
        let Some(guest) = self.guest.take() else {
            return;
        };
        let mut slot = self.slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let external_cleanup_detail = match (&self.origin, &*slot) {
            (GuestInstanceLeaseOrigin::Mounted, GuestInstanceSlot::CleanupPending { instance: None, detail }) => Some(detail.clone()),
            _ => None,
        };
        let expected = match self.origin {
            GuestInstanceLeaseOrigin::Mounted => matches!(&*slot, GuestInstanceSlot::Leased | GuestInstanceSlot::CleanupPending { instance: None, .. }),
            GuestInstanceLeaseOrigin::Cleanup => matches!(&*slot, GuestInstanceSlot::CleanupPending { instance: None, .. }),
        };
        if expected {
            *slot = match std::mem::replace(&mut self.disposition, GuestInstanceLeaseDisposition::Quarantined(b"plugin instance lease lost its disposition".to_vec())) {
                GuestInstanceLeaseDisposition::Available => match external_cleanup_detail {
                    Some(detail) => GuestInstanceSlot::CleanupPending { instance: Some(guest), detail },
                    None => GuestInstanceSlot::Available(guest),
                },
                GuestInstanceLeaseDisposition::CleanupResolved => GuestInstanceSlot::Available(guest),
                GuestInstanceLeaseDisposition::CleanupPending(detail) => GuestInstanceSlot::CleanupPending { instance: Some(guest), detail },
                GuestInstanceLeaseDisposition::Quarantined(detail) => GuestInstanceSlot::Quarantined { instance: guest, detail },
            };
            return;
        }
        *slot = match std::mem::replace(&mut *slot, GuestInstanceSlot::Leased) {
            GuestInstanceSlot::Available(instance) => GuestInstanceSlot::Quarantined { instance, detail: b"plugin instance ownership conflict during relay recovery".to_vec() },
            GuestInstanceSlot::CleanupPending { instance: Some(instance), .. } => GuestInstanceSlot::Quarantined { instance, detail: b"plugin instance ownership conflict during cleanup recovery".to_vec() },
            GuestInstanceSlot::CleanupPending { instance: None, .. } | GuestInstanceSlot::Leased => GuestInstanceSlot::Quarantined { instance: guest, detail: b"plugin instance ownership conflict during relay recovery".to_vec() },
            quarantine @ GuestInstanceSlot::Quarantined { .. } => quarantine,
        };
    }
}

pub(super) fn quarantine_guest_instance(slot: &Arc<Mutex<GuestInstanceSlot>>, detail: Vec<u8>) {
    let mut slot = slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    *slot = match std::mem::replace(&mut *slot, GuestInstanceSlot::Leased) {
        GuestInstanceSlot::Available(instance) => GuestInstanceSlot::Quarantined { instance, detail },
        GuestInstanceSlot::CleanupPending { instance: Some(instance), .. } => GuestInstanceSlot::Quarantined { instance, detail },
        pending @ GuestInstanceSlot::CleanupPending { instance: None, .. } => pending,
        GuestInstanceSlot::Leased => GuestInstanceSlot::Leased,
        quarantine @ GuestInstanceSlot::Quarantined { .. } => quarantine,
    };
}

pub(super) fn mark_guest_cleanup_pending(slot: &Arc<Mutex<GuestInstanceSlot>>, detail: Vec<u8>) {
    let mut slot = slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    *slot = match std::mem::replace(&mut *slot, GuestInstanceSlot::Leased) {
        GuestInstanceSlot::Available(instance) => GuestInstanceSlot::CleanupPending { instance: Some(instance), detail },
        GuestInstanceSlot::Leased => GuestInstanceSlot::CleanupPending { instance: None, detail },
        pending @ GuestInstanceSlot::CleanupPending { .. } => pending,
        quarantine @ GuestInstanceSlot::Quarantined { .. } => quarantine,
    };
}

pub(super) async fn cancel_guest_job_once(runtime: &GuestRuntimes, instance: &mut GuestInstance, job: u64, cancel_admitted: &AtomicBool) -> Result<bool, TurnFault> {
    if cancel_admitted.compare_exchange(false, true, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok() {
        runtime.cancel_job(instance, job).await?;
        return Ok(true);
    }
    Ok(false)
}

fn cancel_guest_job_failure_detail(error: &TurnFault) -> Vec<u8> {
    format!("plugin instance quarantined after cancel-job failure: {error}").into_bytes()
}

pub(super) fn guest_relay_cleanup_pending_detail(stage: &str, error: &TurnFault) -> Vec<u8> {
    format!("plugin instance cleanup pending after {stage} failure: {error}").into_bytes()
}

pub(super) fn foreground_cancel_completion(result: Result<bool, TurnFault>, guest: &mut GuestInstanceLease) -> GuestRelayCompletion {
    match result {
        Ok(true) => {
            guest.cleanup_resolved();
            GuestRelayCompletion::Cancelled
        }
        Ok(false) => {
            let detail = b"plugin instance quarantined because cancel-job admission was already consumed".to_vec();
            guest.quarantine(detail.clone());
            GuestRelayCompletion::TerminalFault(GuestRelayOwnedBytes::new(detail))
        }
        Err(error) => {
            let detail = cancel_guest_job_failure_detail(&error);
            guest.quarantine(detail.clone());
            GuestRelayCompletion::TerminalFault(GuestRelayOwnedBytes::new(detail))
        }
    }
}
//#endregion 🔀️PostTurnRelay
