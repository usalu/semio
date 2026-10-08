//! 🎚️ Typed parameter intent; scene lookup, ownership copying, and Store publication belong to retained work.

use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔣️Payload
/// 🎚️ One finite numeric intent addressed to a domain widget; surface_id is transport metadata only.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
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
    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        if grant.maximum_depth < 1 { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "intent retirement exceeds admitted depth")); }
        let released_bytes = self.next_release_byte_demand()?;
        if released_bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        drop(self.bytes[self.index].take());
        self.index += 1;
        let progress = RetainedCloneProgress { copied_items: 1, released_bytes, ..empty };
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }
    fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
    fn next_capacity_byte_demand(&self, _copy: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
    fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.bytes.get(self.index).and_then(Option::as_ref).map_or(0, Vec::capacity)) }
    fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(usize::from(!self.terminal_is_empty())) }
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
