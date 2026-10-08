use super::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_svg_basic_editor();
    assert_eq!(def.role, semio_framework::AppRole::Editor);
    assert_eq!(def.dialect, SVG_BASIC_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<SvgBasicEditor as ArtifactEditor>::DIALECT, SVG_BASIC_DIALECT);
}

#[test]
fn natural_file_route_enforces_basic_and_publishes_the_decoded_snapshot() {
    let codec = <SvgBasicEditor as ArtifactEditor>::natural_file_codec().expect("SVG Basic natural codec");
    assert_eq!((codec.format_kind, codec.extension, codec.media_type, codec.binary), ("s.stdio.svg@1.1", ".svg", "image/svg+xml", false));
    let source = include_bytes!("../../../🧫️fixtures/✍️set-text-applied/➡️after.svg");
    let imported = <SvgBasicEditor as ArtifactEditor>::decode_natural_file(source).expect("SVG Basic natural import");
    let exported = <SvgBasicEditor as ArtifactEditor>::encode_natural_file(&imported).expect("SVG Basic natural export");
    let independent = semio_s_artifact_stdio_svg_test_oracle::standards::v1_1::subsets::basic::oracle_round_trip(&exported).expect("quick-xml reopens SVG Basic export");
    SvgSnapshot::import_utf8(&independent).expect("independent SVG Basic output reopens");

    let outside = br#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1" baseProfile="basic"><filter><feTurbulence/></filter></svg>"#;
    assert!(<SvgBasicEditor as ArtifactEditor>::decode_natural_file(outside).is_err());
    let outside = SvgSnapshot::import_utf8(outside).unwrap();
    assert!(<SvgBasicEditor as ArtifactEditor>::encode_natural_file(&outside).is_err());
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", super::SvgBasicEditor, || semio_framework_plugin::App { definition: super::create_svg_basic_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️1.1/🪆️subsets/🔰️basic");
