//! 🧪️ Native IDML snapshot, semantic mutation, sparse diff and representation laws.
use crate::{IdmlSnapshot, IdmlDiff, IdmlMutation};
use semio_framework_value::FromValue;
use semio_framework_os_kernel::{ColdMutation, ColdDiffAlgebra, OpText, OpBinary};
use semio_framework_pack::snapshot::{ArtifactDsl, ArtifactPack};

fn decode<T: semio_framework_value::FromValue>(value: &serde_json::Value) -> T {
    semio_framework_pack_json::from_json_str(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()
}

#[test]
fn native_idml_fixture_mutations_are_atomic_and_invertible() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("mutations.json")).unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let base: IdmlSnapshot = decode(&case["before"]);
        let expected: IdmlSnapshot = decode(&case["after"]);
        let mutation: IdmlMutation = decode(&case["mutation"]);
        let saved = base.clone();
        let outcome = mutation.diff(&base);
        let after = semio_framework_os_kernel::apply_diff_cold(outcome.diff(), &base).unwrap();
        assert_eq!(after, expected, "{name}: RFC 6902 witness");
        assert_eq!(base, saved, "{name}: immutable base");
        assert_eq!(semio_framework_os_kernel::apply_diff_cold(&outcome.diff().inverse(&base), &after).unwrap(), base, "{name}: diff inverse");
        let mut restored = after.clone();
        for inverse in mutation.inverse(&base).unwrap().into_iter().rev() { restored = semio_framework_os_kernel::apply_diff_cold(inverse.diff(&restored).diff(), &restored).unwrap(); }
        assert_eq!(restored, base, "{name}: semantic inverse");
        if !case["accepted"].as_bool().unwrap() { assert!(outcome.messages().iter().any(|message| message.code == "mutation.invariant"), "{name}: rejection diagnostic"); }
        assert_eq!(IdmlMutation::parse_op(&mutation.print_op()).unwrap(), mutation, "{name}: op text");
        assert_eq!(IdmlMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation, "{name}: op binary");
        assert_eq!(IdmlSnapshot::parse_dsl(&after.print_dsl()).unwrap(), after, "{name}: snapshot text");
        assert_eq!(IdmlSnapshot::decode_pack(&after.encode_pack()).unwrap(), after, "{name}: snapshot pack");
        let value = semio_framework_value::ToValue::to_value(outcome.diff());
        assert_eq!(IdmlDiff::from_value(value).unwrap(), *outcome.diff(), "{name}: sparse diff wire");
    }
}

#[test]
fn hostile_diff_does_not_modify_an_unpublished_snapshot() {
    use semio_framework_value::FromValue;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("intrinsic.json")).unwrap();
    let mut base: IdmlSnapshot = decode(&fixture["valid"]);
    let saved = base.clone();
    let diff = IdmlDiff::from_value(semio_framework_value::DslValue::from(&serde_json::json!({"steps":[{"parts":[{"path":"designmap.xml","bytes":null}]}]}))).unwrap();
    assert!(semio_framework_os_kernel::apply_diff_cold_into(&diff, &mut base).is_err());
    assert_eq!(base, saved);
}

#[test]
fn native_pack_schema_hash_matches_independent_declaration_identity() {
    let hex = crate::schema_hash().iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    assert_eq!(hex, "32afdfb0a01bab498e3b3e63cc557cb8e1b9e5619be657d6bfdbd0993997757e");
}

#[test]
fn controlled_snapshot_admission_refuses_unchecked_xml_and_obeys_cancellation() {
    use semio_framework_value::{DslValue, NativeDecodeControl};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("intrinsic.json")).unwrap();
    let value = DslValue::from(&fixture["valid"]);
    let mut proceed = |_| true;
    let mut control = NativeDecodeControl::new(64 << 20, &mut proceed);
    assert!(IdmlSnapshot::from_value_controlled(&value, &mut control).is_ok());
    let mut cancel = |_| false;
    let mut control = NativeDecodeControl::new(64 << 20, &mut cancel);
    assert!(IdmlSnapshot::from_value_controlled(&value, &mut control).is_err());
    for case in fixture["invalid"].as_array().unwrap() {
        let mut value = fixture["valid"].clone();
        match case["operation"].as_str().unwrap() {
            "setVersion" => value["domVersion"] = case["value"].clone(),
            "setPath" => value["parts"][1]["path"] = case["value"].clone(),
            "duplicate" => { let part = value["parts"][1].clone(); value["parts"].as_array_mut().unwrap().push(part); }
            "setDesignmap" => value["parts"][1]["bytes"] = serde_json::json!(case["value"].as_str().unwrap().as_bytes()),
            _ => unreachable!(),
        }
        let mut control = NativeDecodeControl::new(64 << 20, &mut proceed);
        assert!(IdmlSnapshot::from_value_controlled(&DslValue::from(&value), &mut control).is_err(), "{}", case["name"]);
    }
}

#[test]
fn sequential_sparse_diff_composition_preserves_results_and_refusals() {
    use semio_framework_os_kernel::ColdMutationDiff;
    let corpus: serde_json::Value = serde_json::from_str(include_str!("mutations.json")).unwrap();
    let first = &corpus["cases"][0];
    let last = corpus["cases"].as_array().unwrap().last().unwrap();
    let base: IdmlSnapshot = decode(&first["before"]);
    let middle: IdmlSnapshot = decode(&first["after"]);
    let target: IdmlSnapshot = decode(&last["after"]);
    let mut combined = IdmlDiff::changing(&base, &middle);
    combined.absorb(IdmlDiff::changing(&middle, &target));
    assert_eq!(semio_framework_os_kernel::apply_diff_cold(&combined, &base).unwrap(), target);
    assert_eq!(semio_framework_os_kernel::apply_diff_cold(&combined.inverse(&base), &target).unwrap(), base);
    let mut invalid: IdmlDiff = decode(&serde_json::json!({"steps":[{"parts":[{"path":"designmap.xml","bytes":null}]}]}));
    invalid.absorb(IdmlDiff::changing(&base, &middle));
    assert!(semio_framework_os_kernel::apply_diff_cold(&invalid, &base).is_err());
}

#[test]
fn native_declaration_formats_factory_and_receipt_share_one_identity() {
    let definition = crate::definition().unwrap();
    assert_eq!(definition.identity().as_str(), crate::ARTIFACT_KIND);
    definition.validate_stdio().unwrap();
    let assembly = crate::assembly().unwrap();
    assert_eq!(assembly.definition().identity().as_str(), crate::ARTIFACT_KIND);
    assert!(matches!(assembly, semio_s_artifact_stdio_contract::ArtifactAssembly::Runtime(_)));
    let formats = crate::formats().unwrap();
    assert_eq!(formats.len(), 1);
    assert_eq!(formats[0].short_id, "idml");
    assert_eq!(formats[0].extensions, [".idml"]);
    assert_eq!(formats[0].mimes, [crate::PACKAGE_MIME]);
    assert!(formats[0].is_binary);
    let factories = crate::native_codecs();
    assert_eq!(factories.len(), 1);
    let codec = (factories[0].codec)();
    assert_eq!(codec.schema, crate::DOCUMENT_SCHEMA);
    assert_eq!(codec.extension, "idml");
    assert_eq!(codec.pack_schema_hash, crate::schema_hash());
    let contribution = crate::contribution();
    let receipts = semio_s_artifact_stdio_contract::native_codec_factory_receipts(&contribution, "s.stdio", "s.stdio.idml", "0.1.0").unwrap();
    assert_eq!(receipts.len(), 1);
    assert_eq!(receipts[0].artifact_kind, crate::ARTIFACT_KIND);
    assert_eq!(receipts[0].instantiate().unwrap().pack_schema_hash, crate::schema_hash());
    assert!(semio_s_artifact_stdio_contract::registry::ContributionRegistry::new(vec![contribution]).is_err());
    for descriptor in <IdmlMutation as semio_framework_os_kernel::Mutation<IdmlSnapshot>>::DESCRIPTORS { descriptor.validate().unwrap(); }
}
