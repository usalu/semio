//! 🌞️ World-3d sun state and scene constructor shared down to the base window kits (the mesh kit renders through them).
use semio_framework_ui_scene::World3dScene;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};
use serde_json::json;
/** 🌞️ Plugin-owned directional-light state for a `world-3d` scene; off by default so meshes render flat until a dev opts in via the window-options Sun toggle. */
#[derive(Clone, Debug, PartialEq, Serialize, ToValue, Deserialize, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(rename_all = "camelCase", default)]
#[value(rename_all = "camelCase", default)]
pub struct WorldSunConfig {
    pub enabled: bool,
    pub azimuth: f64,
    pub elevation: f64,
    pub intensity: f64,
    pub color: String,
}
impl Default for WorldSunConfig {
    fn default() -> Self {
        Self { enabled: false, azimuth: 45.0, elevation: 35.0, intensity: 0.85, color: "#ffffff".into() }
    }
}
/** 🌞️ Builds the `environment_json` payload consumed by `world-3d-host.tsx`'s `WorldEnvironmentRecord.sun`. */
pub fn world3d_environment_json(sun: &WorldSunConfig) -> String {
    json!({ "sun": sun }).to_string()
}
pub fn world3d_scene(camera_json: String, meshes_json: String, instances_json: String, selection_json: String, sun: &WorldSunConfig) -> World3dScene {
    let mut scene = World3dScene::base(camera_json, meshes_json, instances_json, selection_json);
    scene.environment_json = Some(world3d_environment_json(sun));
    scene
}
