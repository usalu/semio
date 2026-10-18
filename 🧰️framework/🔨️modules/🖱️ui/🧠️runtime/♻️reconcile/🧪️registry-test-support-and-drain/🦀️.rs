//! 🧪️ Registry test support and drain: the registry test guard, the isolation, the orphaned slot reclaim and the drain until idle. See [`super`].

use super::*;

//#region ⏭️ResumableReconcile
//#region 🎟️RetainedAuthority
/// 🔒️ Serializes laws that admit into the process-wide surface output, resident, and handback registries.
pub fn surface_reconcile_registry_test_guard() -> SurfaceReconcileRegistryTestGuard {
    SurfaceReconcileRegistryTestGuard::acquire()
}

pub struct SurfaceReconcileRegistryTestGuard {
    _lock: MutexGuard<'static, ()>,
}

impl SurfaceReconcileRegistryTestGuard {
    fn acquire() -> Self {
        static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
        let lock = GUARD.get_or_init(|| Mutex::new(())).lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        isolate_surface_reconcile_registries();
        Self { _lock: lock }
    }
}

impl Drop for SurfaceReconcileRegistryTestGuard {
    fn drop(&mut self) {
        isolate_surface_reconcile_registries();
    }
}

fn isolate_surface_reconcile_registries() {
    drain_surface_reconcile_registry_until_idle();
    reclaim_orphaned_handback_slots();
    drain_surface_reconcile_registry_until_idle();
}

/// ♻️ Rebuilds the handback free list from the slots that are genuinely unowned, and returns every
/// retained state it reclaimed so the drop happens outside the registry lock.
///
/// A RESERVED slot is never reclaimed. The registry is process-wide while
/// [`SurfaceReconcileRegistryTestGuard`]'s mutex only serialises the laws that take the guard, so a
/// blanket reset frees slots that a law running beside this one still holds: that owner's own release
/// then finds the free list already full and reports `surface handback free list exhausted` — an
/// accounting overflow, reported as if the pool had run dry, in whichever law happened to release
/// next. Reclaiming by state instead of by position keeps one owner's isolation from corrupting
/// another's, and the retirement ring is rebuilt from the entries whose slot survived rather than
/// cleared, so a live owner's queued retirement is not silently dropped either.
fn reclaim_orphaned_handback_slots() {
    let mut orphaned = Vec::new();
    {
        let mut registry = SURFACE_RECONCILE_HANDBACKS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut reclaimed = [false; SURFACE_RECONCILE_HANDBACK_SLOTS];
        let mut free_len = 0;
        for index in (0..SURFACE_RECONCILE_HANDBACK_SLOTS).rev() {
            if registry.slots[index].reserved {
                continue;
            }
            if let Some(state) = registry.slots[index].state.take() {
                orphaned.push(state);
            }
            let epoch = registry.slots[index].epoch;
            registry.slots[index] = SurfaceReconcileHandbackSlot { epoch, ..SurfaceReconcileHandbackSlot::default() };
            registry.free[free_len] = index;
            reclaimed[index] = true;
            free_len += 1;
        }
        registry.free_len = free_len;
        let mut retirement = [usize::MAX; SURFACE_RECONCILE_HANDBACK_SLOTS];
        let mut retirement_len = 0;
        for offset in 0..registry.retirement_len {
            let slot = registry.retirement[(registry.retirement_head + offset) % SURFACE_RECONCILE_HANDBACK_SLOTS];
            if slot >= SURFACE_RECONCILE_HANDBACK_SLOTS || reclaimed[slot] {
                continue;
            }
            retirement[retirement_len] = slot;
            retirement_len += 1;
        }
        registry.retirement = retirement;
        registry.retirement_head = 0;
        registry.retirement_len = retirement_len;
    }
    drop(orphaned);
}

pub(super) fn drain_surface_reconcile_registry_until_idle() {
    output::recover_output_registry_poison();
    drop(SURFACE_RECONCILE_HANDBACKS.lock().unwrap_or_else(std::sync::PoisonError::into_inner));
    for _ in 0..(64 * 4 + SURFACE_RECONCILE_HANDBACK_SLOTS * 8) {
        let output_pending = output::output_registry_has_pending_returns();
        if output_pending {
            let _ = SurfaceReconcileOutputs::drain_one(1, SURFACE_RECONCILE_PAGE_BYTES);
        }
        let resident_pending = ui_contract::UiResidentPermit::has_pending_returns();
        if resident_pending {
            let _ = ui_contract::UiResidentPermit::drain_one();
        }
        let handback_idle = close_surface_reconcile_handback_one().unwrap_or(false);
        let value_idle = ui_contract::close_ui_value_page_one();
        let built_idle = ui_contract::close_built_node_page_one();
        if !output_pending && !resident_pending && handback_idle && value_idle && built_idle {
            break;
        }
    }
}
//#endregion 🎟️RetainedAuthority
//#endregion ⏭️ResumableReconcile
