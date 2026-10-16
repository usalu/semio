//! 🛡️ The process-wide resident-ledger guard that serialises the laws admitting into the shared ledgers and drains their pending returns before and after each law. Exposed under the `testkit` feature so every reconcile crate guards its suites the same way.

use std::sync::{Mutex, MutexGuard, OnceLock};

pub struct RegistryGuard {
    _lock: MutexGuard<'static, ()>,
}

pub fn registry_guard() -> RegistryGuard {
    static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    let lock = GUARD.get_or_init(|| Mutex::new(())).lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    drain_resident_registries_until_idle();
    RegistryGuard { _lock: lock }
}

impl Drop for RegistryGuard {
    fn drop(&mut self) {
        drain_resident_registries_until_idle();
    }
}

fn drain_resident_registries_until_idle() {
    for _ in 0..(64 * 4 + ui_contract::UI_RESIDENT_SLOTS * 6 * 8) {
        let resident_pending = ui_contract::UiResidentPermit::has_pending_returns();
        if resident_pending {
            let _ = ui_contract::UiResidentPermit::drain_one();
        }
        let value_idle = ui_contract::close_ui_value_page_one();
        let built_idle = ui_contract::close_built_node_page_one();
        if !resident_pending && value_idle && built_idle {
            break;
        }
    }
}
