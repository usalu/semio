use super::*;
use crate::os_directory::io::binary::artifact_hash::hex_lower;
use semio_framework_value_derive::FromValue;

#[derive(FromValue)]
#[value(rename_all = "camelCase")]
struct ArtifactAuthorityFixture {
    descriptor: DocumentDescriptor,
    descriptor_encoding_hex: String,
    descriptor_digest_v1: ArtifactHash,
}

#[semio_framework_async_macros::async_test]
async fn document_descriptor_digest_v1_matches_the_language_neutral_binary_vector() {
    let fixture: ArtifactAuthorityFixture = semio_framework_pack_json::from_json_str(include_str!("../../../../../../🧫️fixtures/📇️directory/🛡️artifact-authority.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("artifact authority fixture decodes");
    println!("[DEBUG] descriptor canonical bytes and native authority digest admitted in explicit IO");
    assert_eq!(hex_lower(&descriptor_digest_encoding_v1(&fixture.descriptor).expect("descriptor encodes")), fixture.descriptor_encoding_hex);
    assert_eq!(descriptor_digest_v1(&fixture.descriptor).expect("descriptor hashes"), fixture.descriptor_digest_v1);
}

#[test]
fn descriptor_digest_native_owner_is_explicit_io() {
    assert!(module_path!().contains("os_directory::io::binary::descriptor_digest::tests"));
}

#[semio_framework_async_macros::async_test]
async fn descriptor_metadata_refusals_precede_native_encoding() {
    let fixture: ArtifactAuthorityFixture = semio_framework_pack_json::from_json_str(include_str!("../../../../../../🧫️fixtures/📇️directory/🛡️artifact-authority.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let mut empty = fixture.descriptor.clone(); empty.space_id.clear();
    let mut zero_hash = fixture.descriptor.clone(); zero_hash.pack_schema_hash = "0".repeat(64);
    let mut uppercase = fixture.descriptor.clone(); uppercase.pack_schema_hash = "AA".repeat(32);
    let mut zero_version = fixture.descriptor.clone(); zero_version.bootstrap_version = 0;
    let mut frontier = fixture.descriptor.clone(); frontier.bootstrap_frontier.commit_seq = frontier.bootstrap_frontier.head_seq + 1;
    for invalid in [empty, zero_hash, uppercase, zero_version, frontier] {
        assert_eq!(descriptor_digest_encoding_v1(&invalid).map(|_| ()), validate_document_descriptor_v1(&invalid));
        assert!(validate_document_descriptor_v1(&invalid).is_err());
    }
    println!("[DEBUG] Five descriptor metadata refusals agree at pure and native boundaries");
}
