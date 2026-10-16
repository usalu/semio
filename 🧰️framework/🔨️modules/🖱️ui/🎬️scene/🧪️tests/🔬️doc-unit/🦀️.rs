//! 🔬️ Unit proofs of the scene doc contract: the lane hash against published FNV-1a vectors and the fnv crate, the lane reference codecs and the value primitives.

use super::*;
use protocol::value::{DslValue, FromValue, ToValue, ValueError};
use std::hash::Hasher;

#[test]
fn scene_lane_hash_matches_the_published_fnv1a_64_vectors_and_the_fnv_crate() {
    for (payload, expected) in [("", "cbf29ce484222325"), ("a", "af63dc4c8601ec8c"), ("foobar", "85944171f73967e8")] {
        assert_eq!(scene_lane_hash(payload), expected, "published vector {payload:?}");
    }
    for payload in ["", "x", r#"{"lane":"annotations"}"#, "grüße — 世界 🌍", &"0123456789abcdef".repeat(4096)] {
        let mut oracle = fnv::FnvHasher::default();
        oracle.write(payload.as_bytes());
        assert_eq!(scene_lane_hash(payload), format!("{:016x}", oracle.finish()), "fnv crate oracle");
    }
}

#[test]
fn scene_lane_ref_round_trips_through_the_value_codec_and_serde() {
    let reference = SceneLaneRef { lane: "annotations".into(), bytes: 42, hash: scene_lane_hash("payload") };
    assert_eq!(SceneLaneRef::from_value(reference.to_value()), Ok(reference.clone()));
    let json = serde_json::to_value(&reference).unwrap();
    assert_eq!(json, serde_json::json!({"lane": "annotations", "bytes": 42, "hash": reference.hash}));
    assert_eq!(serde_json::from_value::<SceneLaneRef>(json).unwrap(), reference);
}

#[test]
fn value_primitives_follow_serde_optionality() {
    let entries = vec![("present".to_string(), DslValue::String("yes".into())), ("nothing".to_string(), DslValue::Null)];
    assert_eq!(value_decode::<String>(&entries, "present"), Ok("yes".to_string()));
    assert_eq!(value_decode_option::<String>(&entries, "absent"), Ok(None));
    assert_eq!(value_decode_option::<String>(&entries, "nothing"), Ok(None));
    assert_eq!(value_decode_default(&entries, "absent", || 7u32), Ok(7));
    assert_eq!(value_decode::<String>(&entries, "absent"), Err(ValueError::new(protocol::value::ValueRefusalKind::InvalidValue, "missing field `absent`")));
    let mut out = Vec::new();
    value_push(&mut out, "k", &"v".to_string());
    value_push_option(&mut out, "none", &Option::<String>::None);
    value_push_if_nonempty::<String>(&mut out, "empty", &[]);
    assert_eq!(out, vec![("k".to_string(), DslValue::String("v".into()))]);
}
