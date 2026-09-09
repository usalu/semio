use super::*;

#[semio_framework_async_macros::async_test]
async fn logical_snapshot_and_facets_have_no_shadow_state() {
    let json = format!("{:?}", PptxSnapshot::default());
    for forbidden in ["physical", "sourceBytes", "nativeArchive", "semanticBlake3"] {
        assert!(!json.contains(forbidden), "snapshot contains forbidden shadow field {forbidden}");
    }
    for facet in [
        include_str!("../../🟦️.ts"),
        include_str!("../../🔗️.graphql"),
        include_str!("../../🔣️.json"),
        include_str!("../../🛰️.proto"),
        include_str!("../../../🟦️.ts"),
        include_str!("../../../🔗️.graphql"),
        include_str!("../../../🔣️.json"),
        include_str!("../../../🛰️.proto"),
        include_str!("../../../🔺️diff/🟦️.ts"),
        include_str!("../../../🔺️diff/🔗️.graphql"),
        include_str!("../../../🔺️diff/🔣️.json"),
        include_str!("../../../🔺️diff/🛰️.proto"),
        include_str!("../../../🧬️mutations/🟦️.ts"),
        include_str!("../../../🧬️mutations/🔗️.graphql"),
        include_str!("../../../🧬️mutations/🔣️.json"),
        include_str!("../../../🧬️mutations/🛰️.proto"),
    ] {
        for forbidden in ["PptxPhysical", "sourceBytes", "source_bytes", "nativeArchive", "native_archive", "semanticBlake3", "semantic_blake3", "archiveBytes", "archive_bytes", "rawXml", "raw_xml"] {
            assert!(!facet.contains(forbidden), "facet contains forbidden shadow concept {forbidden}");
        }
    }
}
