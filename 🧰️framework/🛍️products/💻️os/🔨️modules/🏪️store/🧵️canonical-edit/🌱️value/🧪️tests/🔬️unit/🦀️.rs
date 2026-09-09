use super::super::super::{ArtifactCanonicalJsonCursor, ARTIFACT_CANONICAL_JSON_CHUNK_BYTES};
use super::{ArtifactCanonicalValueAdmission, ArtifactCanonicalValueCloseStep, ArtifactCanonicalValueGrant, ArtifactCanonicalValueLimits, ArtifactCanonicalValueStep};
use crate::DslValue;
use std::sync::Arc;

fn limits() -> ArtifactCanonicalValueLimits {
    ArtifactCanonicalValueLimits { maximum_depth: 64, maximum_retained_bytes: 1_048_576, maximum_work_items: 100_000 }
}

fn grant(bytes: usize) -> ArtifactCanonicalValueGrant {
    ArtifactCanonicalValueGrant { maximum_items: 1, maximum_bytes: bytes }
}

fn close_and_return(cursor: &mut ArtifactCanonicalValueAdmission<Arc<DslValue>>) -> Arc<DslValue> {
    for _ in 0..100_000 {
        match cursor.close_step(ArtifactCanonicalValueGrant { maximum_items: 1, maximum_bytes: 0 }) {
            ArtifactCanonicalValueCloseStep::Progress { structural_items, .. } => assert_eq!(structural_items, 1),
            ArtifactCanonicalValueCloseStep::Returned { source, structural_items, .. } => {
                assert_eq!(structural_items, 1);
                assert!(cursor.terminal_is_empty());
                assert!(matches!(cursor.close_step(grant(1)), ArtifactCanonicalValueCloseStep::Complete(_)));
                return source;
            }
            _ => panic!("positive structural grant must progress or return the exact source"),
        }
    }
    panic!("canonical close exceeded the neutral work bound")
}

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap()
}

fn entries(value: &serde_json::Value) -> DslValue {
    DslValue::Object(value.as_array().unwrap().iter().map(|row| (row[0].as_str().unwrap().into(), DslValue::from(&row[1]))).collect())
}

#[test]
fn shared_value_canonical_json_admission_rejects_duplicate_keys_before_exposing_a_source() {
    let vectors = fixture();
    let duplicate = entries(&vectors["duplicateEntries"]);
    let key = vectors["largeKey"]["unit"].as_str().unwrap().repeat(vectors["largeKey"]["repeat"].as_u64().unwrap() as usize);
    let long_duplicate = DslValue::Object(vec![(key.clone(), DslValue::Null), (key, DslValue::Bool(true))]);
    assert_eq!(crate::os_pack::json::to_json_string(&duplicate), vectors["duplicatePackJson"].as_str().unwrap());
    for bytes in [1, 7, 256] {
        for value in [duplicate.clone(), DslValue::Array(vec![duplicate.clone()]), long_duplicate.clone()] {
            let source = Arc::new(value);
            let identity = Arc::as_ptr(&source);
            let mut admission = ArtifactCanonicalValueAdmission::new(source, limits()).unwrap_or_else(|(_, error)| panic!("{error}"));
            let mut failure = None;
            for _ in 0..100_000 {
                let before = admission.checkpoint();
                let step = admission.advance(grant(bytes));
                let after = admission.checkpoint();
                assert!(after.processed_bytes - before.processed_bytes <= bytes.min(256));
                match step {
                    Err(error) => {
                        failure = Some(error);
                        break;
                    }
                    Ok(ArtifactCanonicalValueStep::Complete(_)) => panic!("duplicate object became a canonical source"),
                    Ok(_) => {}
                }
            }
            assert_eq!(failure, vectors["duplicateFault"].as_str());
            assert!(admission.take_value().is_none());
            let returned = close_and_return(&mut admission);
            assert_eq!(identity, Arc::as_ptr(&returned));
            assert_eq!(Arc::strong_count(&returned), 1);
            assert!(admission.terminal_is_empty());
        }
    }
    println!("[DEBUG] canonical value duplicate keys reject at 1, 7, and 256 bytes and return their exact immutable source");
}

#[test]
fn shared_value_canonical_json_admission_honors_grants_and_cancellation() {
    let vectors = fixture();
    let key = vectors["largeKey"]["unit"].as_str().unwrap().repeat(vectors["largeKey"]["repeat"].as_u64().unwrap() as usize);
    let value = DslValue::Object(vec![(key.clone(), entries(&vectors["validEntries"])), (format!("{key}x"), DslValue::Null)]);
    for bytes in [1, 7, 256] {
        for stop in vectors["cancelSteps"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize).chain([100_000]) {
            let source = Arc::new(value.clone());
            let identity = Arc::as_ptr(&source);
            let mut admission = ArtifactCanonicalValueAdmission::new(source, limits()).unwrap_or_else(|(_, error)| panic!("{error}"));
            for _ in 0..stop {
                let before = admission.checkpoint();
                assert_eq!(admission.advance(grant(0)).unwrap(), ArtifactCanonicalValueStep::Blocked);
                assert_eq!(admission.advance(ArtifactCanonicalValueGrant { maximum_items: 0, maximum_bytes: bytes }).unwrap(), ArtifactCanonicalValueStep::Blocked);
                assert_eq!(admission.checkpoint(), before);
                assert!(matches!(admission.close_step(ArtifactCanonicalValueGrant { maximum_items: 0, maximum_bytes: bytes }), ArtifactCanonicalValueCloseStep::Blocked(checkpoint) if checkpoint == before));
                let step = admission.advance(grant(bytes)).unwrap();
                let after = admission.checkpoint();
                assert!(after.processed_items - before.processed_items <= 1);
                assert!(after.processed_bytes - before.processed_bytes <= bytes.min(256));
                assert!(after.retained_bytes <= limits().maximum_retained_bytes);
                if matches!(step, ArtifactCanonicalValueStep::Complete(_)) {
                    break;
                }
            }
            if stop == 100_000 {
                let accepted = admission.take_value().unwrap();
                let returned = accepted.into_source();
                assert_eq!(identity, Arc::as_ptr(&returned));
                assert_eq!(Arc::strong_count(&returned), 1);
            } else {
                admission.cancel();
                let returned = close_and_return(&mut admission);
                assert_eq!(identity, Arc::as_ptr(&returned));
                assert_eq!(Arc::strong_count(&returned), 1);
            }
            assert!(admission.terminal_is_empty());
        }
    }
    println!("[DEBUG] canonical value admission obeys tiny grants and cancellation at every neutral checkpoint");
}

#[test]
fn shared_value_canonical_json_admission_returns_owners_after_limits_and_late_cancel() {
    let vectors = fixture();
    for row in vectors["limitsCases"].as_array().unwrap() {
        let mut bounds = limits();
        let value = match row["kind"].as_str().unwrap() {
            "capacity" => {
                bounds.maximum_retained_bytes = row["maximumRetainedBytes"].as_u64().unwrap() as usize;
                entries(&vectors["validEntries"])
            }
            "work" => {
                bounds.maximum_work_items = row["maximumWorkItems"].as_u64().unwrap() as usize;
                DslValue::Array(vec![DslValue::Null])
            }
            "depth" => {
                bounds.maximum_depth = row["maximumDepth"].as_u64().unwrap() as usize;
                DslValue::Array(vec![DslValue::Null])
            }
            "nonFinite" => DslValue::float(f64::NAN),
            _ => unreachable!(),
        };
        let source = Arc::new(value);
        let identity = Arc::as_ptr(&source);
        let mut admission = ArtifactCanonicalValueAdmission::new(source, bounds).unwrap_or_else(|(_, error)| panic!("{error}"));
        let mut failure = None;
        for _ in 0..100_000 {
            if let Err(error) = admission.advance(grant(7)) {
                failure = Some(error);
                break;
            }
        }
        assert_eq!(failure, row["fault"].as_str());
        assert!(admission.take_value().is_none());
        assert_eq!(Arc::as_ptr(&close_and_return(&mut admission)), identity);
        assert!(admission.terminal_is_empty());
    }
    let source = Arc::new(entries(&vectors["validEntries"]));
    let identity = Arc::as_ptr(&source);
    let mut admission = ArtifactCanonicalValueAdmission::new(source, limits()).unwrap_or_else(|(_, error)| panic!("{error}"));
    for _ in 0..100_000 {
        if matches!(admission.advance(grant(7)).unwrap(), ArtifactCanonicalValueStep::Complete(_)) {
            break;
        }
    }
    admission.cancel();
    assert!(admission.take_value().is_none());
    assert_eq!(Arc::as_ptr(&close_and_return(&mut admission)), identity);
    assert!(admission.terminal_is_empty());
    println!("[DEBUG] canonical value capacity, work, depth, non-finite, and completed cancellation preserve exact source ownership");
}

struct OrderedOracle<'a>(&'a DslValue);

impl serde::Serialize for OrderedOracle<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use protocol::value::Number;
        use serde::ser::{SerializeMap, SerializeSeq};
        match self.0 {
            DslValue::Null => serializer.serialize_unit(),
            DslValue::Bool(value) => serializer.serialize_bool(*value),
            DslValue::Number(Number::UInt(value)) => serializer.serialize_u64(*value),
            DslValue::Number(Number::Int(value)) => serializer.serialize_i64(*value),
            DslValue::Number(Number::Float(value)) => serializer.serialize_f64(*value),
            DslValue::String(value) => serializer.serialize_str(value),
            DslValue::Array(values) => {
                let mut output = serializer.serialize_seq(Some(values.len()))?;
                for value in values {
                    output.serialize_element(&OrderedOracle(value))?;
                }
                output.end()
            }
            DslValue::Object(values) => {
                let mut output = serializer.serialize_map(Some(values.len()))?;
                for (key, value) in values {
                    output.serialize_entry(key, &OrderedOracle(value))?;
                }
                output.end()
            }
        }
    }
}

#[test]
fn shared_value_canonical_json_matches_neutral_vectors_and_serde_json_at_each_chunk_size() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🌱️value/🧬️clone/🧫️fixtures/🔣️.json")).unwrap();
    let mut values: Vec<DslValue> = vectors["values"].as_array().unwrap().iter().map(DslValue::from).collect();
    let unit = vectors["largeString"]["unit"].as_str().unwrap();
    let repeat = vectors["largeString"]["repeat"].as_u64().unwrap() as usize;
    values.push(DslValue::Object(vec![(unit.repeat(repeat), DslValue::String(unit.repeat(repeat)))]));
    values.push(DslValue::Object(vec![("z".into(), DslValue::uint(u64::MAX)), ("a".into(), DslValue::float(1.0)), ("m".into(), DslValue::int(i64::MIN))]));
    for source in values {
        let expected = serde_json::to_vec(&OrderedOracle(&source)).unwrap();
        assert_eq!(expected, crate::os_pack::json::to_json_string(&source).as_bytes());
        for chunk_size in [1, 7, ARTIFACT_CANONICAL_JSON_CHUNK_BYTES] {
            let mut admission = ArtifactCanonicalValueAdmission::new(Arc::new(source.clone()), limits()).unwrap_or_else(|(_, error)| panic!("{error}"));
            for _ in 0..100_000 {
                if matches!(admission.advance(grant(chunk_size)).unwrap(), ArtifactCanonicalValueStep::Complete(_)) {
                    break;
                }
            }
            let accepted = admission.take_value().unwrap();
            assert!(admission.terminal_is_empty());
            let mut cursor = ArtifactCanonicalJsonCursor::default();
            let mut actual = Vec::new();
            for _ in 0..100_000 {
                let mut chunk = [0; ARTIFACT_CANONICAL_JSON_CHUNK_BYTES];
                let written = cursor.encode_chunk(&accepted, &mut chunk[..chunk_size]).unwrap();
                assert!(written <= chunk_size);
                actual.extend_from_slice(&chunk[..written]);
                if cursor.is_complete() {
                    break;
                }
            }
            assert!(cursor.is_complete());
            assert_eq!(actual, expected);
        }
    }
    println!("[DEBUG] shared value canonical JSON: 11 neutral/adversarial values match serde_json at 1, 7, and 256 bytes per chunk");
}
