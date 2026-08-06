//! 🖥️ OS `EngineHost` surface for brep: plugins hold handles and call through here — never a private kernel registry.

use std::sync::Mutex;

use semio_framework_os_kernel::{Engine, EngineCache, EngineFault, EngineHandle, EngineHost};

use crate::brep::kernel::Brep;

/// 🔑 Registered brep compute engine id (document ops and replay derive land here).
pub const BREP_ENGINE_ID: &str = "s.3d.brep";

//#region 🔖️DocumentOpEngine
/// 📜️ Placeholder for content-addressed brep document operations (input pack → output pack).
pub struct BrepDocumentOpEngine;

impl Engine for BrepDocumentOpEngine {
    const ENGINE_ID: &'static str = BREP_ENGINE_ID;

    fn compute(&self, input: &[u8]) -> Result<Vec<u8>, EngineFault> {
        if input.is_empty() {
            return Err(EngineFault::InvalidInput("empty brep op".into()));
        }
        Err(EngineFault::Compute(format!(
            "brep document op dispatch not implemented ({} bytes)",
            input.len()
        )))
    }
}
//#endregion 🔖️DocumentOpEngine

//#region 🔖️Host
/// 🧠 Host-owned brep session: LRU engine cache plus one compute-scoped `Brep` registry.
pub struct BrepEngineHost {
    cache: Mutex<EngineCache>,
    kernel: Mutex<Brep>,
}

impl BrepEngineHost {
    /// 🏗️ New host with the given byte budget for cached engine outputs.
    pub fn new(cache_budget_bytes: usize) -> Self {
        let mut cache = EngineCache::new(cache_budget_bytes);
        cache.register(BrepDocumentOpEngine);
        Self {
            cache: Mutex::new(cache),
            kernel: Mutex::new(Brep::new()),
        }
    }

    /// 🔩 Synchronous `BrepKernel` session mutex (host-owned, not a process-global kernel).
    pub fn kernel(&self) -> &Mutex<Brep> {
        &self.kernel
    }

    /// 🔩 Run a closure against the brep kernel.
    pub fn with_kernel<R>(&self, f: impl FnOnce(&mut Brep) -> R) -> Result<R, EngineFault> {
        let mut guard = self.kernel.lock().map_err(|_| EngineFault::Compute("brep kernel lock poisoned".into()))?;
        Ok(f(&mut guard))
    }
}

impl EngineHost for BrepEngineHost {
    fn derive(&self, engine_id: &str, input: &[u8]) -> Result<EngineHandle, EngineFault> {
        let mut guard = self.cache.lock().map_err(|_| EngineFault::Compute("brep cache lock poisoned".into()))?;
        guard.derive(engine_id, input)
    }

    fn read(&self, handle: &EngineHandle) -> Result<Vec<u8>, EngineFault> {
        let guard = self.cache.lock().map_err(|_| EngineFault::Compute("brep cache lock poisoned".into()))?;
        guard.read(handle)
    }
}
//#endregion 🔖️Host

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_derive_registers_brep_engine() {
        let host = BrepEngineHost::new(4096);
        let err = host.derive(BREP_ENGINE_ID, b"\x01");
        assert!(matches!(err, Err(EngineFault::Compute(_))));
    }

    #[test]
    fn kernel_lock_runs_box_prim() {
        use crate::brep::engine::{block_on, BrepKernel};
        let host = BrepEngineHost::new(4096);
        let mut kernel = host.kernel().lock().expect("lock");
        let handle = block_on(kernel.box_prim(1.0, 1.0, 1.0));
        assert!(handle.is_ok());
        let handle = handle.unwrap();
        assert_eq!(handle.as_str().len(), 64);
        assert!(handle.as_str().chars().all(|ch| ch.is_ascii_hexdigit()));
    }
}
