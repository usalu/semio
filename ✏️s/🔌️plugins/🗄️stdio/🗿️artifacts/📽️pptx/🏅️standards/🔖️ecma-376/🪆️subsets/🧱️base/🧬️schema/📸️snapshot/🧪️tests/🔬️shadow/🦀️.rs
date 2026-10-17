use semio_framework_test_fixtures::fixture_text;
use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::*;

#[semio_framework_async_macros::async_test]
async fn logical_snapshot_and_facets_have_no_shadow_state() {
    let json = format!("{:?}", PptxSnapshot::default());
    for forbidden in ["physical", "sourceBytes", "nativeArchive", "semanticBlake3"] {
        assert!(!json.contains(forbidden), "snapshot contains forbidden shadow field {forbidden}");
    }
    for facet in [
        fixture_text!("../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts").to_owned(),
        fixture_text!("../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔗️.graphql").to_owned(),
        fixture_text!("../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔣️.json").to_owned(),
        fixture_text!("../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🛰️.proto").to_owned(),
        fixture_text!("../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🟦️.ts").to_owned(),
        fixture_text!("../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔗️.graphql").to_owned(),
        fixture_text!("../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔣️.json").to_owned(),
        fixture_text!("../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🛰️.proto").to_owned(),
        fixture_text!("../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🟦️.ts").to_owned(),
        fixture_text!("../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🔗️.graphql").to_owned(),
        fixture_text!("../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🔣️.json").to_owned(),
        fixture_text!("../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🛰️.proto").to_owned(),
        fixture_text!("../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🟦️.ts").to_owned(),
        fixture_text!("../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🔗️.graphql").to_owned(),
        fixture_text!("../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🔣️.json").to_owned(),
        fixture_text!("../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🛰️.proto").to_owned(),
    ] {
        for forbidden in ["PptxPhysical", "sourceBytes", "source_bytes", "nativeArchive", "native_archive", "semanticBlake3", "semantic_blake3", "archiveBytes", "archive_bytes", "rawXml", "raw_xml"] {
            assert!(!facet.contains(forbidden), "facet contains forbidden shadow concept {forbidden}");
        }
    }
}
