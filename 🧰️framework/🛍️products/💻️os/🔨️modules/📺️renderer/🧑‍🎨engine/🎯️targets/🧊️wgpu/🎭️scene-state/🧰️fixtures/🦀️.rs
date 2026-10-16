//! 🧰️ Test fixtures of the scene state: the original camera authority every scene law installs, and the camera dispatch sweep.
//!
//! 🧭️ Behind `testkit`: the monolith test unit held these functions in the Scenes standalone law file; laws in the test bins name them through this crate.

use super::*;

/// 🧪️ Independent schema fixture authority is explicit at every original camera test boundary.
pub fn camera_fixture_grant() -> semio_framework_job::RetainedCloneGrant {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧱️elements/🎞️Scenes/⏱️camera/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    serde_json::from_value(fixture["rootGrant"].clone()).unwrap()
}

pub struct CameraFixtureLease {
    grant: semio_framework_job::RetainedCloneGrant,
}

pub fn camera_fixture_lease(grant: semio_framework_job::RetainedCloneGrant) -> CameraFixtureLease {
    install_scene_camera_authority(grant).unwrap();
    CameraFixtureLease { grant }
}

pub fn clear_camera_fixture(grant: semio_framework_job::RetainedCloneGrant) {
    install_scene_camera_authority(grant).unwrap();
    for _ in 0..2048 {
        let step = close_scene_camera_storage(grant).unwrap();
        assert!(step.progress().fits(grant));
        if scene_camera_storage_is_empty() {
            return;
        }
    }
    panic!("original camera fixture storage did not settle");
}

impl Drop for CameraFixtureLease {
    fn drop(&mut self) {
        clear_camera_fixture(self.grant);
    }
}

pub fn sweep_expired_scene_camera_dispatches(now_ms: f64) -> Vec<ActionDescriptor> {
    let grant = camera_fixture_grant();
    let (mut cursor, admission) = SceneCameraDispatchCursor::begin(now_ms, grant).unwrap();
    assert!(admission.fits(grant));
    let mut actions = Vec::new();
    loop {
        match cursor.step(grant).unwrap().0 {
            SceneCameraDispatchStep::Pending => {}
            SceneCameraDispatchStep::Action(action) => actions.push(action),
            SceneCameraDispatchStep::Complete => return actions,
            SceneCameraDispatchStep::Fault(_) => {
                while !matches!(cursor.close_step(grant).unwrap(), semio_framework_job::InteractiveJobCloseStep::Complete { .. }) {}
                return actions;
            }
        }
    }
}
