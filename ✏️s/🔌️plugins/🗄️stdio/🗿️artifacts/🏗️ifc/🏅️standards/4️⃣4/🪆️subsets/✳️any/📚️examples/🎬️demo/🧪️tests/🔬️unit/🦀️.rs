use super::*;

#[test]
fn authored_ifc4_example_has_one_native_and_one_exchange_authority() {
    let expected = crate::standards::v4::engine::demo_ifc_snapshot();
    let native = <crate::IfcSnapshot as store::ArtifactDsl>::parse_dsl(PRIMARY_TEXT).expect("authored IFC4 native text");
    assert_eq!(native, expected);
    assert_eq!(store::ArtifactDsl::print_dsl(&native), PRIMARY_TEXT);
    assert_eq!(<crate::IfcSnapshot as store::ArtifactPack>::decode_pack(PRIMARY_PACK).expect("authored IFC4 native Pack"), expected);
    assert_eq!(store::ArtifactPack::encode_pack(&native), PRIMARY_PACK);
    let exchange = semio_s_artifact_stdio_contract::part21::parse_part21(EXCHANGE_TEXT).expect("authored IFC4 external Part21");
    assert_eq!(crate::schema::snapshot::from_part21_document(crate::STDIO_IFC_DOCUMENT_SCHEMA, &exchange), expected);
    let input = semio_s_artifact_stdio_binary::BinarySnapshot { schema: semio_s_artifact_stdio_binary::STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes: EXCHANGE_TEXT.as_bytes().to_vec() };
    let import = crate::standards::v4::subsets::any::io::import::deserializers::artifacts::binary::v_raw::any::deserialize;
    assert_eq!(import(&input).expect("explicit IFC4 exchange importer"), expected);
    let invalid = semio_s_artifact_stdio_binary::BinarySnapshot { bytes: PRIMARY_TEXT.as_bytes().to_vec(), ..input };
    match import(&invalid) {
        Err(store::PackError::Refusal(store::PackRefusal::ValueRefusal(error))) => assert_eq!(error.kind, semio_framework_value::ValueRefusalKind::InvalidValue),
        other => panic!("IFC4 exchange importer accepted another format: {other:?}"),
    }
    let _ = source();
}
