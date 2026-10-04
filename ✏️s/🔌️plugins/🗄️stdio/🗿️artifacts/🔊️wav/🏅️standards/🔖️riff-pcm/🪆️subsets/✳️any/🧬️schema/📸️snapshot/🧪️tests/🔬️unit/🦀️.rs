use super::*;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_snapshot() -> WavSnapshot {
    WavSnapshot { data: WavData::Pcm16(vec![1, -1, 100, -100]), ..WavSnapshot::default() }
}

#[semio_framework_async_macros::async_test]
async fn json_pack_round_trips() {
    let snap = sample_snapshot();
    let bytes = <WavSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <WavSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn dsl_text_round_trips() {
    let snap = sample_snapshot();
    let text = <WavSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back = <WavSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn typed_snapshot_fields_preserve_samples_and_unknown_chunks() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎼️typed-chunks/🔣️.json")).unwrap();
    for vector in corpus["cases"].as_array().unwrap() {
        let oracle = &vector["snapshot"];
        let snapshot: WavSnapshot = semio_framework_pack_json::from_json_str(&serde_json::to_string(oracle).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let text = <WavSnapshot as store::ArtifactDsl>::print_dsl(&snapshot);
        assert_eq!(<WavSnapshot as store::ArtifactDsl>::parse_dsl(&text).unwrap(), snapshot, "{}", vector["id"]);
        let bytes = <WavSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        assert_eq!(<WavSnapshot as store::ArtifactPack>::decode_pack(&bytes).unwrap(), snapshot, "{}", vector["id"]);
        assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&snapshot)).unwrap(), *oracle);
    }
}

#[test]
fn typed_snapshot_fields_refuse_ambiguous_or_unknown_discriminants() {
    use semio_framework_dsl_record::DslField;
    let mut data = match WavData::Pcm16(vec![1]).to_value() {
        semio_framework_dsl_record::FieldValue::Record(record) => record,
        _ => unreachable!(),
    };
    data.fields.insert(3, semio_framework_dsl_record::FieldValue::List(vec![]));
    assert!(WavData::from_value(&semio_framework_dsl_record::FieldValue::Record(data)).is_err());
    let mut chunk = match WavChunkRef::Format.to_value() {
        semio_framework_dsl_record::FieldValue::Record(record) => record,
        _ => unreachable!(),
    };
    chunk.fields.insert(2, semio_framework_dsl_record::FieldValue::UInt(0));
    assert!(WavChunkRef::from_value(&semio_framework_dsl_record::FieldValue::Record(chunk)).is_err());
    let mut extra = match WavChunkRef::Samples.to_value() {
        semio_framework_dsl_record::FieldValue::Record(record) => record,
        _ => unreachable!(),
    };
    extra.fields.insert(3, semio_framework_dsl_record::FieldValue::UInt(0));
    assert!(WavChunkRef::from_value(&semio_framework_dsl_record::FieldValue::Record(extra)).is_err());
    let mut unknown = semio_framework_dsl_record::RecordValue::default();
    unknown.fields.insert(1, semio_framework_dsl_record::FieldValue::Enum(9));
    assert!(WavData::from_value(&semio_framework_dsl_record::FieldValue::Record(unknown.clone())).is_err());
    assert!(WavChunkRef::from_value(&semio_framework_dsl_record::FieldValue::Record(unknown)).is_err());
}
