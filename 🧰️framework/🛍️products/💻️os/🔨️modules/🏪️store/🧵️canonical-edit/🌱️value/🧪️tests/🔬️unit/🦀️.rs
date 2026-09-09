use super::super::super::{ArtifactCanonicalJsonCursor, ARTIFACT_CANONICAL_JSON_CHUNK_BYTES};
use crate::DslValue;

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
                for value in values { output.serialize_element(&OrderedOracle(value))?; }
                output.end()
            }
            DslValue::Object(values) => {
                let mut output = serializer.serialize_map(Some(values.len()))?;
                for (key, value) in values { output.serialize_entry(key, &OrderedOracle(value))?; }
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
            let mut cursor = ArtifactCanonicalJsonCursor::default();
            let mut actual = Vec::new();
            for _ in 0..100_000 {
                let mut chunk = [0; ARTIFACT_CANONICAL_JSON_CHUNK_BYTES];
                let written = cursor.encode_chunk(&source, &mut chunk[..chunk_size]).unwrap();
                assert!(written <= chunk_size);
                actual.extend_from_slice(&chunk[..written]);
                if cursor.is_complete() { break; }
            }
            assert!(cursor.is_complete());
            assert_eq!(actual, expected);
        }
    }
    println!("[DEBUG] shared value canonical JSON: 11 neutral/adversarial values match serde_json at 1, 7, and 256 bytes per chunk");
}
