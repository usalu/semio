//! 🔗️ Neutral artifact identity projection and independent JSON admission.
use crate::{ArtifactRef,ArtifactDialect};
use semio_framework_value::{ToValue,FromValue};

#[test]
fn semantic_artifact_reference_preserves_the_exact_four_owned_fields() {
    let reference=ArtifactRef{artifact_id:"".into(),dialect:ArtifactDialect{artifact_kind:"s.stdio.svg".into(),standard:"1.1".into(),subset:"*".into()}};
    let value=reference.to_value();
    assert_eq!(ArtifactRef::from_value(value).expect("owned reference"),reference);
    eprintln!("[DEBUG] Semantic artifact reference owns its four exact identity fields");
}

#[test]
fn neutral_reference_admission_and_json_projection_match_the_independent_oracle() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧬️schema/🗿️artifact-reference/🧫️fixtures/🪪️identity/🔣️.json")).expect("neutral identities");
    for expected in fixture["valid"].as_array().unwrap() {
        let input:semio_framework_value::DslValue=serde_json::from_value(expected.clone()).expect("independent JSON input");
        let reference=ArtifactRef::from_value(input).expect("owned semantic admission");
        let actual=serde_json::to_value(reference.to_value()).expect("independent JSON output");
        assert_eq!(actual,*expected);
    }
    for invalid in fixture["invalid"].as_array().unwrap() {
        let input:semio_framework_value::DslValue=serde_json::from_value(invalid.clone()).expect("independent JSON input");
        assert!(ArtifactRef::from_value(input).is_err());
    }
    eprintln!("[DEBUG] Two valid and seven invalid neutral references match first-party admission and serde_json output");
}

#[test]
fn native_reference_text_extensions_preserve_literal_identity_and_control() {
    use crate::io::text::artifact_reference::{ArtifactReferenceText,DialectCoordinateText};
    let expected=ArtifactRef{artifact_id:"shape-1".into(),dialect:ArtifactDialect{artifact_kind:"s.stdio.svg".into(),standard:"1.1".into(),subset:"*".into()}};
    assert_eq!(expected.to_uri(),"shape-1!s.stdio.svg@1.1/*");
    assert_eq!(ArtifactRef::parse_uri(&expected.to_uri()).unwrap(),expected);
    assert_eq!(ArtifactDialect::parse_coordinate(&expected.dialect.to_coordinate()).unwrap(),expected.dialect);
    let mut allow=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(4096,&mut allow);
    assert_eq!(ArtifactRef::parse_uri_controlled(&expected.to_uri(),&mut control).unwrap(),expected);
    let mut deny=|_|false;let mut control=semio_framework_value::NativeDecodeControl::new(4096,&mut deny);
    assert!(ArtifactRef::parse_uri_controlled(&expected.to_uri(),&mut control).is_err());
    eprintln!("[DEBUG] Explicit native reference text extension retains bounded admission and cancellation");
}
