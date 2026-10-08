//! 🪟️ Persisted-local configuration schema for one concrete Jack graph window.

use crate::Camera;

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[artifact(extension = "trinity.jackgraphwindowcfg")]
#[dsl(layout = "lines")]
pub struct JackGraphWindowConfig {
    #[dsl(block)]
    pub camera: Option<Camera>,
    pub lod_mode: String,
}

impl Default for JackGraphWindowConfig {
    fn default() -> Self {
        Self { camera: None, lod_mode: "automatic".into() }
    }
}

/// 🕳️ Tri-state decode of every `Option<Option<T>>` diff slot: a missing key is the unchanged slot (`None`) and a PRESENT
/// `null` is the clear `Some(None)`, never the unchanged slot the blanket `Option<T>` decode would fold it into.
fn deserialize_double_option<T: semio_framework_value::FromValue>(value: semio_framework_value::DslValue) -> Result<Option<Option<T>>, semio_framework_value::ValueError> {
    <Option<T> as semio_framework_value::FromValue>::from_value(value).map(Some)
}

/// 🔺️ Sparse typed delta of one Jack graph window's persisted-local configuration: names only the fields a mutation changes.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct JackGraphWindowConfigDiff {
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub camera: Option<Option<Camera>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub lod_mode: Option<String>,
}

impl protocol::MutationDiff<JackGraphWindowConfig> for JackGraphWindowConfigDiff {
    fn apply(&self, base: &JackGraphWindowConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<JackGraphWindowConfig> {
        let mut next = base.clone();
        if let Some(camera) = &self.camera {
            next.camera.clone_from(camera);
        }
        if let Some(lod_mode) = &self.lod_mode {
            next.lod_mode.clone_from(lod_mode);
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.camera.is_some() {
            self.camera = other.camera;
        }
        if other.lod_mode.is_some() {
            self.lod_mode = other.lod_mode;
        }
    }
}

impl protocol::DiffAlgebra<JackGraphWindowConfig> for JackGraphWindowConfigDiff {
    fn inverse(&self, base: &JackGraphWindowConfig) -> Self {
        Self { camera: self.camera.as_ref().map(|_| base.camera.clone()), lod_mode: self.lod_mode.as_ref().map(|_| base.lod_mode.clone()) }
    }
    fn is_empty(&self) -> bool {
        self.camera.is_none() && self.lod_mode.is_none()
    }
}
