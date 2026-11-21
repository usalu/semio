//! 📤️ Turns over the occupied instance slots once even when every file-picker outbox is empty.

use std::sync::{Arc, Mutex};

use semio_framework::kernel::Effect;
use semio_plugin_instance_app::VcsArtifactApp;
use semio_plugin_plugin_program::AppInstance;
use semio_plugin_reactor_app::{PluginApp, PluginAppDelegate};
use semio_plugin_reactor_lifecycle::life_app_cell::RuntimeAppCell;
use semio_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime;
use semio_plugin_runtime_ops::ops_raw_media::continue_original_picker_effects;
use semio_plugin_testing_caller::OriginalFixtureCaller;
use semio_plugin_testing_laws::artifact_app_laws;
use semio_plugin_tests_05::plugin_runtime_plugin_builder_contract_support_1::{contract_app_raw, TestApp};
use serde_json::Value;

struct IdlePicker {
    app: VcsArtifactApp<TestApp>,
    instance: u32,
    visits: Arc<Mutex<Vec<u32>>>,
    maximum_visits: usize,
}

impl PluginAppDelegate for IdlePicker {
    type Target = VcsArtifactApp<TestApp>;
    fn delegate_ref(&self) -> &Self::Target { &self.app }
    fn delegate_mut(&mut self) -> &mut Self::Target { &mut self.app }

    fn original_picker_effect(&mut self) -> Option<&mut Effect> {
        let mut visits = self.visits.lock().unwrap();
        assert!(visits.len() < self.maximum_visits, "picker scan revisited occupied slots");
        visits.push(self.instance);
        None
    }
}

#[semio_framework_async_macros::async_test]
async fn idle_picker_turn_scans_each_occupied_slot_once() {
    let fixture: Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let policy = artifact_app_laws::fixture_mounted_policy();
    let mut identity = artifact_app_laws::fixture_identity();
    let mut caller = OriginalFixtureCaller::admit().unwrap();
    let mut context = caller.context();
    for scan in fixture["idlePickerScans"].as_array().unwrap() {
        let instances: Vec<u32> = serde_json::from_value(scan["instances"].clone()).unwrap();
        let expected: Vec<u32> = serde_json::from_value(scan["visits"].clone()).unwrap();
        let runtime = PluginRuntime::new(policy).unwrap();
        let visits = Arc::new(Mutex::new(Vec::new()));
        for &id in &instances {
            let mut inner = contract_app_raw(policy, &mut identity).await;
            inner.bind_instance_id(id).await;
            let app = IdlePicker {
                app: inner,
                instance: id,
                visits: Arc::clone(&visits),
                maximum_visits: expected.len(),
            };
            let cell = RuntimeAppCell::new(AppInstance { id, app, surface_contexts: Default::default() }, policy);
            runtime.instances.borrow_mut().insert_admitted(id, Arc::new(cell));
        }
        assert!(!continue_original_picker_effects(&runtime, &mut context).unwrap());
        assert_eq!(*visits.lock().unwrap(), expected);
        for id in instances {
            let cell = runtime.instances.borrow_mut().take(id).unwrap();
            let mut instance = cell.host_instance().unwrap();
            artifact_app_laws::close_registered_fixture_app(&mut instance.app.app, policy, &mut context).unwrap();
        }
    }
    let fuel = context.fuel_remaining();
    drop(context);
    caller.retire(fuel).unwrap();
}
