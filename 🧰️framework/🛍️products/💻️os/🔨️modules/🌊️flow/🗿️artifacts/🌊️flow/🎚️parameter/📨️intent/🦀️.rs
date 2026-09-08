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
mod tests {
    use super::*;
    use crate::os_store::{ErasedSnapshotRetirement, SnapshotRetirementStep};

    #[test]
    fn graph_parameter_intent_matches_strict_typed_schema() {
        let fixture = crate::os_pack::json::parse(include_str!("🧪️fixture/🔣️.json")).unwrap();
        for row in fixture.get("cases").and_then(crate::os_pack::json::Value::as_array).unwrap() {
            let payload: SetGraphParameter = crate::os_dsl::FromValue::from_value(crate::os_pack::json::to_dsl_value(row)).unwrap();
            payload.validate().unwrap();
            assert_eq!(crate::os_pack::json::from_dsl_value(&crate::os_dsl::ToValue::to_value(&payload)), *row);
        }
        for row in fixture.get("rejected").and_then(crate::os_pack::json::Value::as_array).unwrap() {
            assert!(<SetGraphParameter as crate::os_dsl::FromValue>::from_value(crate::os_pack::json::to_dsl_value(row)).map_or(true, |value| value.validate().is_err()));
        }
        let row = fixture.get("longWidgetId").unwrap();
        let widget_id = row.get("unit").and_then(crate::os_pack::json::Value::as_str).unwrap().repeat(row.get("repetitions").and_then(crate::os_pack::json::Value::as_u64).unwrap() as usize);
        assert_eq!(widget_id.len(), row.get("expectedBytes").and_then(crate::os_pack::json::Value::as_u64).unwrap() as usize);
        let payload = SetGraphParameter { widget_id, value: row.get("value").and_then(crate::os_pack::json::Value::as_f64).unwrap(), surface_id: None };
        payload.validate().unwrap();
        let round_tripped: SetGraphParameter = crate::os_pack::json::from_json_str(&crate::os_pack::json::to_json_string(&payload)).unwrap();
        assert_eq!(round_tripped, payload);
    }

    #[test]
    fn graph_parameter_intent_rejects_non_finite_before_retained_admission() {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(SetGraphParameter { widget_id: "slider".into(), value, surface_id: None }.validate().is_err());
        }
    }

    #[test]
    fn graph_parameter_intent_retirement_preserves_exact_bytes_and_worker_transfer() {
        let fixture = crate::os_pack::json::parse(include_str!("🧪️fixture/🔣️.json")).unwrap();
        let text = fixture.get("longWidgetId").unwrap();
        let law = fixture.get("retirement").unwrap();
        for maximum in [1, 4096] {
            for pause in law.get("cancelAt").and_then(crate::os_pack::json::Value::as_array).unwrap() {
                let payload = SetGraphParameter {
                    widget_id: text.get("unit").and_then(crate::os_pack::json::Value::as_str).unwrap().repeat(text.get("repetitions").and_then(crate::os_pack::json::Value::as_u64).unwrap() as usize), value: 4.0,
                    surface_id: Some(law.get("surfaceUnit").and_then(crate::os_pack::json::Value::as_str).unwrap().repeat(law.get("surfaceRepetitions").and_then(crate::os_pack::json::Value::as_u64).unwrap() as usize)),
                };
                let expected = payload.widget_id.len() + payload.surface_id.as_ref().unwrap().len();
                let mut owner = payload.into_retirement();
                let mut released = 0;
                for _ in 0..pause.as_u64().unwrap() {
                    if let SnapshotRetirementStep::Pending { released_bytes, .. } = owner.close_step(1, maximum).unwrap() { released += released_bytes; }
                }
                assert_eq!(owner.close_step(0, maximum).unwrap(), SnapshotRetirementStep::Blocked);
                assert_eq!(owner.close_step(1, 0).unwrap(), SnapshotRetirementStep::Blocked);
                let (owner, released) = std::thread::spawn(move || {
                    for _ in 0..100_000 {
                        match owner.close_step(1, maximum).unwrap() {
                            SnapshotRetirementStep::Pending { released_items, released_bytes } => { assert!(released_items <= 1 && released_bytes <= maximum); released += released_bytes; }
                            SnapshotRetirementStep::Complete => break,
                            SnapshotRetirementStep::Blocked => panic!("positive intent retirement grant blocked"),
                        }
                    }
                    (owner, released)
                }).join().unwrap();
                assert!(owner.terminal_is_empty());
                assert_eq!(released, expected);
            }
        }
        let owner = SetGraphParameter { widget_id: "guard".into(), value: 1.0, surface_id: None }.into_retirement();
        assert!(std::panic::catch_unwind(|| drop(owner)).is_err());
    }
}
//#endregion 🧪️Contract
