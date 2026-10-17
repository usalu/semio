//! 🧰️ Test-only host surface: the step context fixture and the close-driving `Drop` for `World3dState` (feature `testkit`).

use semio_framework_os_infinite_world_bridge::step_world3d_scene_bridge_close;
use semio_framework_os_infinite_world_host::World3dState;
use semio_framework_test_fixtures::fixture_text;
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};

/// 📖️ Reads one test input at run time (relative to the package manifest) so it does not count as an embedded compile input.

/// 📖️ Reads several test inputs at run time and joins them into the one text the unsplit sources formed.
fn read_joined(paths: &[&str]) -> &'static str {
    let mut joined = String::new();
    for path in paths {
        joined.push_str(fixture_text!(path));
    }
    Box::leak(joined.into_boxed_str())
}

impl Drop for World3dState {
    fn drop(&mut self) {
        for _ in 0..16_384 {
            if !with_world_step_context(1,|context|step_world3d_scene_bridge_close(&mut self.bridge,context)) { return; }
        }
        assert!(std::thread::panicking(), "test fixture retained an original scene bridge owner after its close ceiling");
    }
}

pub(crate) fn with_world_step_context<T>(fuel: u64, step: impl FnOnce(&mut semio_framework_job::StepContext<'_>) -> T) -> T {
    let mut sequence = 0;
    let grant=world_original_fixture_grant();
    let mut receipt=semio_framework_value::RetainedCloneProgress::default();
    let mut context = semio_framework_job::StepContext::new(
        semio_framework_job::OperationId(1),
        semio_framework_job::Generation(1),
        semio_framework_job::StepBudget::new(fuel, u64::MAX,grant),
        semio_framework_job::root_cancel_token(),
        semio_framework_job::default_now_us,
        &mut sequence,
        &mut receipt,
    );
    let result=step(&mut context);
    assert!(context.retained_progress().fits(grant));
    result
}

fn world_original_fixture_grant()->semio_framework_value::RetainedCloneGrant{
    let law:serde_json::Value=serde_json::from_str(fixture_text!("../../../🧪️tests/🔬️unit/🧫️retained-attributes.json")).unwrap();let axes=&law["grant"];
    semio_framework_value::RetainedCloneGrant{maximum_items:axes["items"].as_u64().unwrap()as usize,maximum_copy_bytes:axes["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:axes["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:axes["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:axes["depth"].as_u64().unwrap()as usize}
}
