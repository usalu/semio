//! 🎚️ Typed parameter intent; scene lookup, ownership copying, and Store publication belong to retained work.

use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔣️Payload
/// 🎚️ One finite numeric intent addressed to a domain widget; surface_id is transport metadata only.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetGraphParameter {
    pub widget_id: String,
    pub value: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub surface_id: Option<String>,
}

impl SetGraphParameter {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.widget_id.is_empty() { return Err("flow-parameter-widget-id-empty"); }
        if self.surface_id.as_ref().is_some_and(String::is_empty) { return Err("flow-parameter-surface-id-empty"); }
        if !self.value.is_finite() { return Err("flow-parameter-value-non-finite"); }
        Ok(())
    }

    pub fn into_retirement(self) -> SetGraphParameterRetirement {
        SetGraphParameterRetirement { bytes: std::mem::ManuallyDrop::new([Some(self.widget_id.into_bytes()), self.surface_id.map(String::into_bytes)]), index: 0 }
    }
}
//#endregion 🔣️Payload

//#region 🔏️CanonicalIntent
impl crate::os_store::ArtifactCanonicalJson for SetGraphParameter {
    fn canonical_json_borrowed_root(&self) -> Result<Option<crate::os_store::ArtifactCanonicalJsonValue<'_>>, String> {
        use crate::os_store::{ArtifactCanonicalJsonNode as Json, ArtifactCanonicalJsonObject as Object, ArtifactCanonicalJsonValue as Value};
        self.validate().map_err(str::to_owned)?;
        let required = [("widgetId", Value::Scalar(Json::String(&self.widget_id))), ("value", Value::Scalar(Json::F64(self.value)))];
        let optional = self.surface_id.iter().map(|value| ("surfaceId", Value::Scalar(Json::String(value.as_str()))));
        Ok(Some(Value::Object(Object::new(required.into_iter().chain(optional)))))
    }
}
//#endregion 🔏️CanonicalIntent

//#region 🧹️IntentRetirement
#[must_use = "graph parameter strings require terminal retained retirement"]
pub struct SetGraphParameterRetirement {
    bytes: std::mem::ManuallyDrop<[Option<Vec<u8>>; 2]>,
    index: usize,
}

impl crate::os_store::ErasedSnapshotRetirement for SetGraphParameterRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<crate::os_store::SnapshotRetirementStep, String> {
        use crate::os_store::SnapshotRetirementStep as Step;
        if maximum_items == 0 || maximum_bytes == 0 { return Ok(Step::Blocked); }
        if self.index == self.bytes.len() { return Ok(Step::Complete); }
        if let Some(value) = self.bytes[self.index].as_mut() {
            let released_bytes = maximum_bytes.min(value.len());
            value.truncate(value.len() - released_bytes);
            if value.is_empty() { self.bytes[self.index] = None; }
            return Ok(Step::Pending { released_items: 0, released_bytes });
        }
        self.index += 1;
        Ok(Step::Pending { released_items: 1, released_bytes: 0 })
    }

    fn terminal_is_empty(&self) -> bool { self.index == self.bytes.len() && self.bytes.iter().all(Option::is_none) }
}

impl Drop for SetGraphParameterRetirement {
    fn drop(&mut self) {
        if !crate::os_store::ErasedSnapshotRetirement::terminal_is_empty(self) {
            if !std::thread::panicking() { panic!("graph parameter intent dropped before bounded retirement"); }
            return;
        }
        unsafe { std::mem::ManuallyDrop::drop(&mut self.bytes); }
    }
}
//#endregion 🧹️IntentRetirement

//#region 🧪️Contract
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Contract
