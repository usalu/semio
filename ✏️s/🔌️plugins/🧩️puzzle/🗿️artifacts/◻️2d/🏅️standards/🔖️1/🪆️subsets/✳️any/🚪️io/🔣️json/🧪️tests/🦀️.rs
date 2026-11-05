//! 🧪️ Independent wire cases exercise actual typed import and unrelated semantic objects.
use super::*;
use crate::Puzzle2dSnapshot;
use crate::standards::v1::subsets::any::io::import::deserializers::artifacts::json::v_rfc8259::any as input;
use store::os_store::serialization::ArtifactSqliteSnapshotHost as _;
use serde_json::Value;

fn laws() -> Value {
    serde_json::from_str(include_str!("../../🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json")).expect("neutral wire cases")
}

fn expected_wire() -> Value {
    let laws = laws();
    let mut wire = laws["snapshot"].clone();
    for path in laws["declaredJson"]["floatPointers"].as_array().unwrap() {
        let value = wire.pointer_mut(path.as_str().unwrap()).unwrap();
        let bits = value.as_f64().unwrap().to_bits();
        *value = serde_json::json!({"bits": format!("{bits:016x}")});
    }
    wire
}

struct SnapshotOwner(Option<Puzzle2dSnapshot>);
impl Drop for SnapshotOwner {
    fn drop(&mut self) {
        if let Some(snapshot) = self.0.take() { snapshot.retire_sqlite_snapshot(); }
    }
}

fn decode(bytes: &[u8]) -> Result<SnapshotOwner, semio_framework_diagnostic::TextError> {
    input::deserialize_bytes(bytes).map(|snapshot| SnapshotOwner(Some(snapshot)))
}

#[test]
fn declared_json_word_and_presence_cases_follow_typed_owner() {
    let laws = laws();
    for case in laws["declaredJson"]["wordInputs"].as_array().unwrap() {
        let mut wire = expected_wire();
        *wire.pointer_mut("/camera/x").unwrap() = Value::String("__puzzle2d_word__".into());
        let text = serde_json::to_string(&wire).unwrap();
        let candidate = text.replacen("\"__puzzle2d_word__\"", case["json"].as_str().unwrap(), 1);
        assert_ne!(candidate, text, "nonvacuous authored word input");
        let result = decode(candidate.as_bytes());
        assert_eq!(result.is_ok(), case["accepted"].as_bool().unwrap(), "{}", case["name"]);
    }
    for case in laws["declaredJson"]["presenceCases"].as_array().unwrap() {
        let mut wire = expected_wire();
        let path = case["path"].as_str().unwrap();
        if case["operation"] == "remove" {
            let (parent, key) = path.rsplit_once('/').unwrap();
            assert!(wire.pointer_mut(parent).unwrap().as_object_mut().unwrap().remove(key).is_some());
        } else {
            *wire.pointer_mut(path).unwrap() = case["value"].clone();
        }
        let result = decode(&serde_json::to_vec(&wire).unwrap());
        assert_eq!(result.is_ok(), case["accepted"].as_bool().unwrap(), "{}", case["name"]);
        if let Ok(snapshot) = result {
            let raw = semio_framework_value::ToValue::to_value(snapshot.0.as_ref().unwrap());
            let native: Value = serde_json::from_str(&semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&raw))).unwrap();
            if case["nativeAbsent"] == true {
                assert!(native.pointer(path).is_none(), "{}", case["name"]);
            } else if let Some(expected) = case.get("native") {
                assert_eq!(native.pointer(path), Some(expected), "{}", case["name"]);
            }
        }
    }
}

#[test]
fn declared_json_only_visits_typed_binary64_roles() {
    for value in laws()["declaredJson"]["untouchedObjects"].as_array().unwrap() {
        let raw: DslValue = semio_framework_pack_json::from_json_str(&serde_json::to_string(value).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(convert(raw.clone(), false).unwrap(), raw);
        assert_eq!(convert(raw.clone(), true).unwrap(), raw);
    }
}
