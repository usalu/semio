//! 🧪️ The xlsx model laws, as an integration target over the artifact's public API.

extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_s_artifact_stdio_xlsx_base_mutations as mutations;

#[path = "../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/💡️inferences/🧪️tests/🔬️unit/🦀️.rs"]
mod inferences_tests;
#[path = "../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/💡️inferences/🧾outline/🧪️tests/🔬️unit/🦀️.rs"]
mod outline_tests;
#[path = "../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs"]
mod handcrafted_diff_codec_tests;
#[path = "../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🧪️tests/🔬️result-apply/🦀️.rs"]
mod result_apply_tests;
#[path = "../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧪️tests/🔬️unit/🦀️.rs"]
mod schema_tests;

#[test]
fn xlsx_guest_pack_metadata_matches_the_authored_native_factory() {
    use pack::snapshot::ArtifactPackMetadata;
    use semio_s_artifact_stdio_xlsx::{definition, native_codecs};
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSnapshot;

    definition().expect("native codec binding must match the artifact definition");
    let factories = native_codecs();
    assert_eq!(factories.len(), 1);
    assert_eq!(XlsxSnapshot::native_pack_schema_hash(), Some((factories[0].codec)().pack_schema_hash));
}
