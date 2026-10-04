//! 📚️ Every example the puzzle 3d subset registers builds its document. The descriptor probe and the example picker run
//! each deferred body's producer (`ExampleSource::document_json`); a fixture the DSL grammar no longer reads panics
//! there and traps the guest (ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, S3-PUZZLE). Twins:
//! `◻️2d/…/🧪️tests/🧪️every-example`, `🖐️5d/…/🧪️tests/🧪️every-example`.

use super::examples;
use crate::standards::v1::subsets::any::schema::snapshot::text::parse_dsl;
use crate::Puzzle3dSnapshot;
use semio_framework_plugin::ExampleSourceBody;

/// 🧱️ Each registered example's authored DSL parses, its produced document decodes to that same snapshot, and the
/// snapshot round-trips through the DSL and pack codecs.
#[test]
fn every_registered_example_builds_its_document() {
    assert!(!examples().is_empty(), "puzzle 3d registers examples");
    for source in examples() {
        let ExampleSourceBody::Deferred { source: authored, .. } = source.body() else { panic!("{} is a deferred DSL example", source.id()) };
        let text = std::str::from_utf8(authored).unwrap_or_else(|error| panic!("{} is UTF-8: {error}", source.id()));
        let parsed = parse_dsl(text).unwrap_or_else(|error| panic!("{} example DSL parses: {error}", source.id()));
        let built: Puzzle3dSnapshot = semio_framework_pack_json::from_json_str(&source.document_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("{} document decodes: {error:?}", source.id()));
        assert!(built == parsed, "{} document equals its authored DSL", source.id());
        semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&built);
        semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&built);
    }
}
