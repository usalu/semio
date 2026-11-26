//! 🔁️ The live cleanup law: a mounted app inside the production runtime never exhausts the zero-progress credit of its live maintenance pump. See [`super`].

use super::*;
use semio_plugin_plugin_program::AppInstance;
use semio_plugin_reactor_lifecycle::life_app_cell::RuntimeAppCell;
use semio_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime;
use semio_plugin_runtime_instances::inst_cleanup_turns::{plugin_destroy_app, plugin_step_close_cleanup, plugin_step_live_cleanup};
use std::sync::atomic::Ordering;

/// 📊️ What one pumped stretch of live maintenance observed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LiveCleanupReport {
    pub turns: usize,
    pub longest_stall: u32,
    pub final_stall: u32,
}

/// 🏠️ One mounted app admitted to the production runtime, so its live maintenance is pumped by the very `plugin_step_live_cleanup` a host turn runs.
pub struct LiveCleanupFixture<PA: PluginApp + 'static> {
    runtime: PluginRuntime<PA>,
    cell: Option<std::sync::Arc<RuntimeAppCell<PA>>>,
    id: u32,
}

impl<PA: PluginApp + 'static> LiveCleanupFixture<PA> {
    /// 🚪️ Admits `app`, already bound to `id`, under the fixture mounted policy.
    pub fn admit(app: PA, id: u32) -> Self {
        let policy = fixture_mounted_policy();
        let runtime = PluginRuntime::new(policy).expect("explicit fixture mounted policy");
        let cell = std::sync::Arc::new(RuntimeAppCell::new(AppInstance { id, app, surface_contexts: Default::default() }, policy));
        runtime.instances.borrow_mut().insert_admitted(id, cell.clone());
        Self { runtime, cell: Some(cell), id }
    }

    /// 🔓️ The app under test; drop the guard before the next pump.
    pub fn app(&self) -> std::sync::MutexGuard<'_, AppInstance<PA>> {
        self.cell.as_ref().expect("live fixture").instance.lock().expect("fixture app lock")
    }

    /// ⚖️ Pumps `turns` live cleanup turns and refuses the first fault, naming the pump state that stalled.
    pub fn pump(&self, turns: usize, cx: &mut semio_framework_job::StepContext<'_>) -> Result<LiveCleanupReport, String> {
        let cell = self.cell.as_ref().expect("live fixture");
        let (grant, _) = cx.original_turn_ceiling();
        let fuel = OriginalFixtureCaller::turn_fuel();
        let mut report = LiveCleanupReport::default();
        for turn in 0..turns {
            fixture_renew_turn(cx, fuel, grant).map_err(|error| format!("turn {turn} could not be renewed: {error:?}"))?;
            if let Err(failure) = plugin_step_live_cleanup(&self.runtime, cx) {
                let pump = cell.maintenance_pump.lock().expect("failed maintenance pump");
                return Err(format!("{failure:?}: turn={turn} stalled={} session={} outcome={} rejected={} terminal={} pending={:?} faulted={}", cell.maintenance_stalled_steps.load(Ordering::SeqCst), pump.session.is_some(), pump.outcome_pending, pump.rejected.is_some(), pump.terminal, pump.pending_status, pump.faulted));
            }
            let stalled = cell.maintenance_stalled_steps.load(Ordering::SeqCst);
            report.longest_stall = report.longest_stall.max(stalled);
            report.final_stall = stalled;
            report.turns += 1;
        }
        Ok(report)
    }

    /// ♻️ Destroys the instance and pays its close cleanup through the production close pump.
    pub async fn retire(mut self, cx: &mut semio_framework_job::StepContext<'_>) {
        drop(self.cell.take());
        plugin_destroy_app(&self.runtime, self.id).await.expect("destroy the live fixture instance");
        let (grant, _) = cx.original_turn_ceiling();
        let fuel = OriginalFixtureCaller::turn_fuel();
        for _ in 0..OriginalFixtureCaller::maximum_turns() {
            fixture_renew_turn(cx, fuel, grant).expect("renew the close turn");
            plugin_step_close_cleanup(&self.runtime, cx).expect("the close cleanup of the live fixture settles");
            if self.runtime.close_quarantine.borrow().get(self.id).is_none() { return }
        }
        panic!("the live fixture instance did not retire within its original caller bound")
    }
}
