use super::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_svg_tiny_editor();
    assert_eq!(def.role, semio_framework::AppRole::Editor);
    assert_eq!(def.dialect, SVG_TINY_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<SvgTinyEditor as ArtifactEditor>::DIALECT, SVG_TINY_DIALECT);
}

#[test]
fn natural_file_route_enforces_tiny_and_publishes_one_fresh_snapshot_event() {
    let codec = <SvgTinyEditor as ArtifactEditor>::natural_file_codec().expect("SVG Tiny natural codec");
    assert_eq!((codec.format_kind, codec.extension, codec.media_type, codec.binary), ("s.stdio.svg@1.1", ".svg", "image/svg+xml", false));
    let source = include_bytes!("../../../🧫️fixtures/📸️set-snapshot-applied/➡️after.svg");
    let imported = <SvgTinyEditor as ArtifactEditor>::decode_natural_file(source).expect("SVG Tiny natural import");
    let exported = <SvgTinyEditor as ArtifactEditor>::encode_natural_file(&imported).expect("SVG Tiny natural export");
    let independent = semio_s_artifact_stdio_svg_test_oracle::standards::v1_1::subsets::tiny::oracle_round_trip(&exported).expect("quick-xml reopens SVG Tiny export");
    SvgSnapshot::import_utf8(&independent).expect("independent SVG Tiny output reopens");
    let Some(SvgTinyMutation::SetSnapshot(set)) = <SvgTinyEditor as ArtifactEditor>::whole_document_operation(imported.clone()) else { panic!("natural SVG Tiny opens through one event-sourced snapshot mutation") };
    assert_eq!(set.snapshot, imported);

    let outside = br#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1" baseProfile="tiny"><filter/></svg>"#;
    assert!(<SvgTinyEditor as ArtifactEditor>::decode_natural_file(outside).is_err());
    let outside = SvgSnapshot::import_utf8(outside).unwrap();
    assert!(<SvgTinyEditor as ArtifactEditor>::encode_natural_file(&outside).is_err());
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", super::SvgTinyEditor, || semio_framework_plugin::App { definition: super::create_svg_tiny_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️1.1/🪆️subsets/🔬️tiny");
