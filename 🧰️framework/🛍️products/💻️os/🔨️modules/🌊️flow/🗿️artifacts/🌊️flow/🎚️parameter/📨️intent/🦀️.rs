//! 🎚️ Typed parameter intent; scene lookup, ownership copying, and Store publication belong to retained work.

use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔣️Payload
/// 🎚️ One finite numeric intent addressed to a domain widget; surface_id is transport metadata only.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
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

}
//#endregion 🔣️Payload

//#region 🔏️CanonicalIntent
impl crate::os_store::ArtifactCanonicalJson for SetGraphParameter {
    fn canonical_json_node(&self, path: &[usize]) -> Result<crate::os_store::ArtifactCanonicalJsonNode<'_>, semio_framework_value::ValueError> {
        use crate::os_store::ArtifactCanonicalJsonNode as Node;
        self.validate().map_err(|reason| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, reason))?;
        match path {
            [] => Ok(Node::Object(2 + usize::from(self.surface_id.is_some()))),
            [0] => Ok(Node::String(&self.widget_id)),
            [1] => Ok(Node::F64(self.value)),
            [2] if self.surface_id.is_some() => Ok(Node::String(self.surface_id.as_deref().unwrap())),
            _ => Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "flow-parameter-canonical-ordinal-invalid")),
        }
    }
    fn canonical_json_key(&self, path: &[usize], ordinal: usize) -> Result<crate::os_store::ArtifactCanonicalJsonText<'_>, semio_framework_value::ValueError> {
        if !path.is_empty() { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "flow-parameter-canonical-key-parent-invalid")); }
        match ordinal {
            0 => Ok("widgetId".into()),
            1 => Ok("value".into()),
            2 if self.surface_id.is_some() => Ok("surfaceId".into()),
            _ => Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "flow-parameter-canonical-key-ordinal-invalid")),
        }
    }
}
//#endregion 🔏️CanonicalIntent

//#region 🧪️Contract
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Contract
