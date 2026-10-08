//! 🧪️ Every committed CC2 intent preserves native codecs, diff and exact inverse.

use super::StepCc2Mutation;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};
use semio_framework_value::{FromValue, ToValue};

#[test]
fn class_native_committed_intents_have_canonical_codecs_and_exact_semantics() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️ap214/🪆️subsets/2️⃣cc2/🧫️fixtures/🎛️history-inputs");
    let mut directories: Vec<_> = std::fs::read_dir(root).unwrap().map(|entry| entry.unwrap().path()).collect();
    directories.sort();
    assert_eq!(directories.len(), 5);
    for directory in directories {
        let wire = std::fs::read_to_string(directory.join("🦠️mutation/🔣️.json")).unwrap();
        let parsed = semio_framework_pack_json::parse(&wire, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let fixture = semio_framework_pack_json::to_dsl_value(&parsed);
        let before = crate::StepSnapshot::from_value(fixture.get("before").unwrap().clone()).unwrap();
        let after = crate::StepSnapshot::from_value(fixture.get("after").unwrap().clone()).unwrap();
        let mutation = StepCc2Mutation::from_value(fixture.get("mutation").unwrap().clone()).unwrap();
        assert_eq!(StepCc2Mutation::parse_op(&mutation.print_op()).unwrap(), mutation);
        let binary = mutation.encode_op().unwrap();
        assert_eq!(StepCc2Mutation::decode_op(&binary).unwrap(), mutation);
        let result = mutation.diff(&before);
        assert!(result.messages().is_empty(), "{:?}", result.messages());
        let applied = result.diff().apply(&before).unwrap();
        assert_eq!(applied.to_value(), after.to_value());
        let mut restored = applied;
        for inverse in mutation.inverse(&before).unwrap() { restored = inverse.diff(&restored).diff().apply(&restored).unwrap(); }
        assert_eq!(restored.to_value(), before.to_value());
        eprintln!("[DEBUG] STEP CC2 {} retained real class wire, typed codec, full semantic change and exact inverse", mutation.kind());
    }
}
