use super::*;

#[semio_framework_async_macros::async_test]
async fn complete_header_state_is_explicit_without_a_raw_archive_shadow_cache() {
    let json = format!("{:?}", ZipSnapshot { entries: vec![ZipEntry::default()], ..ZipSnapshot::default() });
    assert!(json.contains("metadata") && json.contains("compression_method") && json.contains("comment_utf8"));
    for forbidden in ["physical", "sourceBytes", "nativeArchive", "rawMetadata"] {
        assert!(!json.contains(forbidden), "snapshot contains forbidden shadow field {forbidden}");
    }
    for facet in [
        include_str!("../../🟦️.ts"),
        include_str!("../../🔗️.graphql"),
        include_str!("../../🔣️.json"),
        include_str!("../../🛰️.proto"),
    ] {
        for (camel, snake) in [("ZipEntryMetadata", "ZipEntryMetadata"), ("compressionMethod", "compression_method"), ("extraFields", "extra_fields"), ("commentUtf8", "comment_utf8")] {
            assert!(facet.contains(camel) || facet.contains(snake), "snapshot facet omits explicit header state {camel}");
        }
    }
    for facet in [
        include_str!("../../../🔺️diff/🟦️.ts"),
        include_str!("../../../🔺️diff/🔗️.graphql"),
        include_str!("../../../🔺️diff/🔣️.json"),
        include_str!("../../../🔺️diff/🛰️.proto"),
    ] {
        assert!(facet.contains("metadata"));
        assert!(facet.contains("commentUtf8") || facet.contains("comment_utf8"));
    }
    for facet in [include_str!("../../🟦️.ts"), include_str!("../../🔗️.graphql"), include_str!("../../🔣️.json"), include_str!("../../🛰️.proto")] {
        for forbidden in ["nativeArchive", "sourceBytes", "rawMetadata", "raw_metadata"] {
            assert!(!facet.contains(forbidden), "facet contains forbidden shadow concept {forbidden}");
        }
    }
}
