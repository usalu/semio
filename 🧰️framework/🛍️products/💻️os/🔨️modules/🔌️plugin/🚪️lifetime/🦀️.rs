//#region 🚪️RuntimeInstanceCloseAuthority
use super::*;

/// 🪪️ Captures the exact app allocation; a reused numeric ID cannot acquire this close authority.
pub struct PluginInstanceCloseLease<PA: PluginApp> {
    instance_id: u32,
    cell: std::sync::Weak<RuntimeAppCell<PA>>,
    admitted: Option<std::sync::Arc<RuntimeCloseWorkerState<PA>>>,
    terminal: Cell<bool>,
}

impl<PA: PluginApp + 'static> PluginInstanceCloseLease<PA> {
    pub(crate) fn allocation_identity(&self) -> (u32, usize) { (self.instance_id, self.cell.as_ptr().cast::<()>() as usize) }

    pub(super) fn from_cell(instance_id: u32, cell: &std::sync::Arc<RuntimeAppCell<PA>>) -> Self {
        Self { instance_id, cell: std::sync::Arc::downgrade(cell), admitted: None, terminal: Cell::new(false) }
    }

    pub(crate) fn preflight_close(&self, runtime: &PluginRuntime<PA>) -> Result<(), Fault> {
        if self.admitted.is_some() { return Ok(()); }
        let instances = runtime.instances.try_borrow().map_err(|_| plugin_internal_fault("runtime instance authority is busy"))?;
        let current = instances.get(self.instance_id).ok_or_else(|| plugin_internal_fault("captured app lifetime is absent"))?;
        if !std::sync::Weak::ptr_eq(&self.cell, &std::sync::Arc::downgrade(current)) { return Err(plugin_internal_fault("captured app lifetime changed")); }
        if !runtime.close_quarantine.try_borrow().map_err(|_| plugin_internal_fault("runtime close quarantine is busy"))?.can_insert(self.instance_id) { return Err(plugin_internal_fault("runtime close quarantine collided")); }
        let _actors = runtime.instance_actors.try_borrow_mut().map_err(|_| plugin_internal_fault("runtime actor authority is busy"))?;
        checked_runtime_close_generation(runtime.close_generation.get())?;
        Ok(())
    }

    /// 🚪️ Admits at most one close for the captured allocation without retaining an app payload alias.
    pub fn begin_close(&mut self, runtime: &PluginRuntime<PA>) -> Result<(), Fault> {
        if self.admitted.is_some() { return Ok(()); }
        self.admitted = Some(plugin_begin_instance_close(runtime, self.instance_id, Some(&self.cell))?);
        Ok(())
    }

    /// 🔢️ Returns the checked runtime close generation only after ownership was admitted.
    pub fn close_generation(&self) -> Option<u64> {
        self.admitted.as_ref().map(|state| state.generation.0)
    }

    /// 🧾️ Verifies app and worker-session emptiness, not quarantine absence or a generic idle turn.
    ///
    /// The terminal witness LATCHES: emptiness is a fact about an allocation that has already been
    /// handed over, so once observed it can never be retracted by a later poll. The close ladder
    /// re-reads it on every release rung, and both non-terminal answers below are transient —
    /// `TryLockError::WouldBlock` on the close cell or its worker pump means "another thread holds
    /// it right now", not "the app is still live". Without the latch a single contended read after
    /// the receipt was minted retracted the witness and killed the close
    /// (ticket 26/09/02 wave B17; surfaced in the browser as `plugin.reactor-close-authority`).
    pub fn is_retired(&self) -> Result<bool, Fault> {
        if self.terminal.get() {
            return Ok(true);
        }
        let Some(state) = self.admitted.as_ref() else { return Ok(false) };
        match RuntimeCloseStatus::from_repr(state.status.load(Ordering::SeqCst)) {
            RuntimeCloseStatus::Fault(cause) => {
                let fault = runtime_cleanup_fault("close", cause, self.instance_id, state.last_callback_elapsed_us.load(Ordering::SeqCst));
                #[cfg(test)]
                eprintln!("[DEBUG] exact native close fault={fault:?} origin={} phases={:?} detail={:?}", state.last_fault_origin.load(Ordering::SeqCst), state.callback_phase_us.iter().map(|phase| phase.load(Ordering::SeqCst)).collect::<Vec<_>>(), *state.last_fault.lock().unwrap());
                return Err(fault);
            },
            RuntimeCloseStatus::Complete => {},
            _ => return Ok(false),
        }
        let cell = match state.cell.try_lock() {
            Ok(cell) => cell,
            Err(std::sync::TryLockError::WouldBlock) => return Ok(false),
            Err(std::sync::TryLockError::Poisoned(_)) => return Err(plugin_internal_fault("captured app close cell is poisoned")),
        };
        let pump = match state.pump.try_lock() {
            Ok(pump) => pump,
            Err(std::sync::TryLockError::WouldBlock) => return Ok(false),
            Err(std::sync::TryLockError::Poisoned(_)) => return Err(plugin_internal_fault("captured app close worker is poisoned")),
        };
        if cell.is_some() || pump.session.is_some() || pump.rejected.is_some() || pump.outcome.is_some() || pump.terminal || !pump.complete {
            return Err(plugin_internal_fault("captured app close reported terminal while retaining an owner"));
        }
        self.terminal.set(true);
        Ok(true)
    }
}

/// 📸️ Borrows only the exact immutable allocation identity; no snapshot or app value is cloned.
pub fn plugin_capture_instance_close<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32) -> Result<PluginInstanceCloseLease<PA>, Fault> {
    let instances = runtime.instances.try_borrow().map_err(|_| plugin_internal_fault("runtime instance authority is busy"))?;
    let cell = instances.get(instance_id).ok_or_else(|| plugin_internal_fault("cannot capture an absent app lifetime"))?;
    Ok(PluginInstanceCloseLease { instance_id, cell: std::sync::Arc::downgrade(cell), admitted: None, terminal: Cell::new(false) })
}
//#endregion 🚪️RuntimeInstanceCloseAuthority
