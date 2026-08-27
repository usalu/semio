//#region 🚪️RuntimeInstanceCloseAuthority
use super::*;

/// 🪪️ Captures the exact app allocation; a reused numeric ID cannot acquire this close authority.
pub struct PluginInstanceCloseLease<PA: PluginApp> {
    instance_id: u32,
    cell: std::sync::Weak<RuntimeAppCell<PA>>,
    admitted: Option<std::sync::Arc<RuntimeCloseWorkerState<PA>>>,
}

impl<PA: PluginApp + 'static> PluginInstanceCloseLease<PA> {
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
    pub fn is_retired(&self) -> Result<bool, Fault> {
        let Some(state) = self.admitted.as_ref() else { return Ok(false) };
        match state.status.load(Ordering::SeqCst) {
            RUNTIME_CLOSE_FAULT => return Err(plugin_internal_fault("captured app close faulted before terminal ownership")),
            RUNTIME_CLOSE_COMPLETE => {},
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
        Ok(true)
    }
}

/// 📸️ Borrows only the exact immutable allocation identity; no snapshot or app value is cloned.
pub fn plugin_capture_instance_close<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32) -> Result<PluginInstanceCloseLease<PA>, Fault> {
    let instances = runtime.instances.try_borrow().map_err(|_| plugin_internal_fault("runtime instance authority is busy"))?;
    let cell = instances.get(instance_id).ok_or_else(|| plugin_internal_fault("cannot capture an absent app lifetime"))?;
    Ok(PluginInstanceCloseLease { instance_id, cell: std::sync::Arc::downgrade(cell), admitted: None })
}
//#endregion 🚪️RuntimeInstanceCloseAuthority
