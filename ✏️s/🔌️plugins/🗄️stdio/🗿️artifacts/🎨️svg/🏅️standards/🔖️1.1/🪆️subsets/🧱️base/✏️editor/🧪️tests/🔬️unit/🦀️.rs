use super::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_svg_any_editor();
    assert_eq!(def.role, semio_framework::AppRole::Editor);
    assert_eq!(def.dialect, SVG_ANY_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<SvgAnyEditor as ArtifactEditor>::DIALECT, SVG_ANY_DIALECT);
}

#[test]
fn natural_file_route_round_trips_svg_with_an_independent_xml_oracle_and_fresh_snapshot_event() {
    let codec = <SvgAnyEditor as ArtifactEditor>::natural_file_codec().expect("SVG natural codec");
    assert_eq!((codec.format_kind, codec.extension, codec.media_type, codec.binary), ("s.stdio.svg@1.1", ".svg", "image/svg+xml", false));
    let source = include_bytes!("../../../🧫️fixtures/🎨️native-third-party-shape.svg");
    let current = <SvgAnyEditor as ArtifactEditor>::initial_snapshot();
    let imported = <SvgAnyEditor as ArtifactEditor>::decode_natural_file(source).expect("SVG natural import");
    let exported = <SvgAnyEditor as ArtifactEditor>::encode_natural_file(&imported).expect("SVG natural export");
    let independent = semio_s_artifact_stdio_svg_test_oracle::standards::v1_1::subsets::base::oracle_round_trip(&exported).expect("quick-xml reopens SVG export");
    let independent = SvgSnapshot::import_utf8(&independent).expect("independent SVG output reopens");
    assert_eq!(independent.doc.root, imported.doc.root);
    let Some(SvgMutation::SetSnapshot(set)) = <SvgAnyEditor as ArtifactEditor>::whole_document_operation(imported.clone()) else { panic!("natural SVG opens through one event-sourced snapshot mutation") };
    assert_eq!(set.snapshot, imported);
    assert_eq!(current, <SvgAnyEditor as ArtifactEditor>::initial_snapshot(), "opening does not replace the selected owner before publication");
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", super::SvgAnyEditor, || semio_framework_plugin::App { definition: super::create_svg_any_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️1.1/🪆️subsets/🧱️base");
