use super::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_step_any_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, STEP_ANY_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<StepAnyEditor as ArtifactEditor>::DIALECT, STEP_ANY_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    semio_framework_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<StepAnyEditor, crate::viewer::step_any::StepAnyViewer>().await;
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", super::StepAnyEditor, || semio_framework_plugin::App { definition: super::create_step_any_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️ap214/🪆️subsets/🧱️base");

#[test]
fn natural_file_codec_declares_the_exact_kind_extension_and_media_type() {
    let codec = <StepAnyEditor as ArtifactEditor>::natural_file_codec().expect("natural codec");
    assert_eq!((codec.format_kind, codec.extension, codec.media_type, codec.binary), ("s.stdio.step@ap214", ".step", "model/step", false));
}

#[test]
fn natural_step_round_trips_edits_agrees_with_an_independent_text_reader_and_refuses_invalid_files() {
    let source = include_bytes!("../🧫️fixtures/📂️natural/sample.step");
    let snapshot = <StepAnyEditor as ArtifactEditor>::decode_natural_file(source).expect("step opens");
    assert_eq!(snapshot.entities.len(), 2);
    let event = editing::SnapshotEditEvent::SetValue { path: editing::TRANSFORM_PATH.into(), value: semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(r#"{"translate":[10,0,0]}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()) };
    let emit = <StepAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).expect("transform resolves");
    let mut edited = snapshot.clone();
    emit.artifact_mutations.iter().for_each(|mutation| assert!(semio_s_artifact_stdio_step_mutations::standards::v_ap214::subsets::base::schema::mutations::apply_step_mutation(&mut edited, mutation).messages().is_empty()));
    let saved = <StepAnyEditor as ArtifactEditor>::encode_natural_file(&edited).expect("step saves");
    let text = String::from_utf8(saved.clone()).expect("part 21 is text");
    let point = text.lines().find(|line| line.contains("CARTESIAN_POINT")).expect("the point survives");
    let inner = &point[point.rfind("((").map_or(point.rfind('(').unwrap(), |at| at + 1) + 1..point.find("))").unwrap()];
    let coordinates: Vec<f64> = inner.split(',').map(|token| token.trim().parse().expect("a decimal coordinate")).collect();
    assert_eq!(coordinates, vec![11.0, 2.0, 3.0]);
    assert!(text.starts_with("ISO-10303-21;") && text.trim_end().ends_with("END-ISO-10303-21;"));
    let reopened = <StepAnyEditor as ArtifactEditor>::decode_natural_file(&saved).expect("the saved file reopens");
    assert_eq!(reopened.entities[0], edited.entities[0]);
    assert_eq!(reopened.entities[1], snapshot.entities[1]);
    assert_eq!(reopened.header, snapshot.header);
    assert!(<StepAnyEditor as ArtifactEditor>::decode_natural_file(include_bytes!("../🧫️fixtures/📂️natural/invalid.step")).is_err());
}

fn reference() -> semio_framework_value::DslValue {
    semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(include_str!("../🧫️fixtures/📂️natural/expected.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the reference fixture parses"))
}

fn numbers(value: &semio_framework_value::DslValue) -> Vec<f64> {
    value.as_array().expect("an array").iter().map(|item| item.as_f64().expect("a number")).collect()
}

fn rows(value: &semio_framework_value::DslValue) -> Vec<Vec<f64>> {
    value.as_array().expect("an array").iter().map(numbers).collect()
}

#[test]
fn natural_step_decodes_to_what_the_independent_reference_parser_read() {
    use semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::schema::snapshot::StepValue;
    let expected = reference();
    let snapshot = <StepAnyEditor as ArtifactEditor>::decode_natural_file(include_bytes!("../🧫️fixtures/📂️natural/sample.step")).expect("step opens");
    let aggregate = |name: &str| snapshot.entities.iter().filter(|entity| entity.name == name).map(|entity| match &entity.args[1] { StepValue::Aggregate(items) => items.iter().map(|item| match item { StepValue::Real(real) => *real, other => panic!("{other:?}") }).collect::<Vec<f64>>(), other => panic!("{other:?}") }).collect::<Vec<_>>();
    assert_eq!(aggregate("CARTESIAN_POINT"), rows(expected.get("cartesianPoints").unwrap()));
    assert_eq!(aggregate("DIRECTION"), rows(expected.get("directions").unwrap()));
}

#[test]
fn the_scene_draws_the_files_cartesian_points() {
    let snapshot = <StepAnyEditor as ArtifactEditor>::decode_natural_file(include_bytes!("../🧫️fixtures/📂️natural/sample.step")).expect("step opens");
    assert_eq!(snapshot.scene_points(), vec![[1.0, 2.0, 3.0]]);
    assert!(main::render(&snapshot).is_ok());
}

#[test]
fn the_independent_part_21_reader_accepts_the_saved_edit_and_returns_the_edited_points() {
    use semio_s_artifact_stdio_step_test_oracle::standards::v_ap214::subsets::base::oracle_round_trip;
    let snapshot = <StepAnyEditor as ArtifactEditor>::decode_natural_file(include_bytes!("../🧫️fixtures/📂️natural/sample.step")).expect("step opens");
    let event = editing::SnapshotEditEvent::SetValue { path: editing::TRANSFORM_PATH.into(), value: semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(r#"{"translate":[10,0,0]}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()) };
    let emit = <StepAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).expect("transform resolves");
    let mut edited = snapshot.clone();
    emit.artifact_mutations.iter().for_each(|mutation| assert!(semio_s_artifact_stdio_step_mutations::standards::v_ap214::subsets::base::schema::mutations::apply_step_mutation(&mut edited, mutation).messages().is_empty()));
    let saved = <StepAnyEditor as ArtifactEditor>::encode_natural_file(&edited).expect("step saves");
    let by_ruststep = oracle_round_trip(&saved).expect("ruststep parses the saved file");
    let reopened = <StepAnyEditor as ArtifactEditor>::decode_natural_file(&by_ruststep).expect("the ruststep output reopens");
    assert_eq!(reopened.scene_points(), vec![[11.0, 2.0, 3.0]]);
    assert_eq!(reopened.entities.len(), edited.entities.len());
    assert_eq!(reopened.header.file_name.name, edited.header.file_name.name);
}
