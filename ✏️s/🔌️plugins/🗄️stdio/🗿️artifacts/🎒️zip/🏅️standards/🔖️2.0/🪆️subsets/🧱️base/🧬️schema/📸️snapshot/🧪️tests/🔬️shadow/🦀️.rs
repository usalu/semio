use super::*;

#[semio_framework_async_macros::async_test]
async fn logical_snapshot_and_facets_have_no_shadow_state() {
    let json = format!("{:?}", ZipSnapshot::default());
    for forbidden in ["localExtra", "centralExtra", "physical", "sourceBytes", "nativeArchive", "method", "dosDate", "flags", "versionMadeBy", "internalAttrs", "externalAttrs"] {
        assert!(!json.contains(forbidden), "snapshot contains forbidden shadow field {forbidden}");
    }
    for facet in [
        include_str!("../../🟦️.ts"),
        include_str!("../../🔗️.graphql"),
        include_str!("../../🔣️.json"),
        include_str!("../../🛰️.proto"),
        include_str!("../../../🔺️diff/🟦️.ts"),
        include_str!("../../../🔺️diff/🔗️.graphql"),
        include_str!("../../../🔺️diff/🔣️.json"),
        include_str!("../../../🔺️diff/🛰️.proto"),
        include_str!("../../../🧬️mutations/🟦️.ts"),
        include_str!("../../../🧬️mutations/🔗️.graphql"),
        include_str!("../../../🧬️mutations/🔣️.json"),
        include_str!("../../../🧬️mutations/🛰️.proto"),
    ] {
        for forbidden in [
            "ZipExtraField",
            "localExtra",
            "centralExtra",
            "local_extra",
            "central_extra",
            "ZipCompressionMethod",
            "SetEntryMethod",
            "SetEntryFlags",
            "setEntryMethod",
            "setEntryFlags",
            "set_entry_method",
            "set_entry_flags",
            "dosDate",
            "dos_date",
            "unixMtime",
            "unix_mtime",
            "versionMadeBy",
            "version_made_by",
            "internalAttrs",
            "internal_attrs",
            "externalAttrs",
            "external_attrs",
            "nativeArchive",
            "sourceBytes",
        ] {
            assert!(!facet.contains(forbidden), "facet contains forbidden shadow concept {forbidden}");
        }
    }
}
