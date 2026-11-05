use crate::standards::v_r12::subsets::any::schema::mutations::DxfMutation;
use crate::standards::v_r12::subsets::any::schema::snapshot::DxfSnapshot;
use semio_plugin::ArtifactEditor;
use semio_plugin::Editor;
use semio_s_artifact_stdio_contract::editing;
use crate::apply_mutation;
use crate::editor::dxf::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_dxf_any_editor();
    assert_eq!(def.role, semio_plugin::AppRole::Editor);
    assert_eq!(def.dialect, DXF_ANY_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<DxfAnyEditor as ArtifactEditor>::DIALECT, DXF_ANY_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    semio_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<DxfAnyEditor, crate::viewer::dxf::DxfAnyViewer>().await;
}

semio_plugin::history_edit_acceptance_law!("stdio", crate::editor::dxf::DxfAnyEditor, || semio_plugin::App { definition: crate::editor::dxf::create_dxf_any_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️r12/🪆️subsets/📰️header");

#[semio_framework_async_macros::async_test]
async fn details_edits_resolve_to_the_kind_of_the_addressed_row() {
    use crate::schema::snapshot::{DxfEntity, DxfLine, DxfHeaderVar, DxfLayer, DxfValue};
    use crate::standards::v_r12::subsets::any::schema::mutations::{DxfMutation};
    let mut base = DxfSnapshot::default();
    base.tables.layers.push(DxfLayer { name: "0".into(), color: 7, linetype: "CONTINUOUS".into(), flags: 0, unknown_group_codes: vec![] });
    base.entities.push(DxfEntity::Line(DxfLine { start: [0.0, 0.0, 0.0], end: [1.0, 1.0, 0.0], layer: "0".into(), unknown_group_codes: vec![] }));
    base.header_vars.push(DxfHeaderVar { name: "$ACADVER".into(), group_code: 1, value: DxfValue::Str { value: "AC1009".into() }, extra_group_codes: vec![] });
    let emit = |event: editing::SnapshotEditEvent| <DxfAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &base);
    let recolor = emit(editing::SnapshotEditEvent::SetValue { path: "/tables/layers/0/color".into(), value: semio_framework_value::DslValue::uint(3) }).expect("a layer field edit resolves");
    let [mutation @ DxfMutation::SetLayer(_)] = recolor.artifact_mutations.as_slice() else { panic!("a layer field edit raises the layer kind") };
    let mut state = base.clone();
    apply_mutation(&mut state, mutation);
    assert_eq!(state.tables.layers[0].color, 3);
    let removal = emit(editing::SnapshotEditEvent::RemoveValue { path: "/entities/0".into() }).expect("an entity removal resolves");
    assert!(matches!(removal.artifact_mutations.as_slice(), [DxfMutation::RemoveEntity(_)]));
    let dropped = emit(editing::SnapshotEditEvent::RemoveValue { path: "/headerVars/0".into() }).expect("a header variable removal resolves");
    assert!(matches!(dropped.artifact_mutations.as_slice(), [DxfMutation::RemoveHeaderVar(_)]));
    let unnamed = emit(editing::SnapshotEditEvent::SetValue { path: "/schema".into(), value: semio_framework_value::DslValue::String("other".into()) }).err().expect("the schema stamp has no kind");
    assert_eq!(unnamed.code.0, "snapshot-edit.unsupported-path");
}

#[semio_framework_async_macros::async_test]
async fn natural_dxf_file_round_trips_a_geometry_edit_and_keeps_unedited_entities() {
    use crate::schema::snapshot::{DxfCircle, DxfEntity, DxfLayer, DxfLine};
    use crate::standards::v_r12::subsets::any::schema::mutations::DxfMutation;
    let codec = <DxfAnyEditor as ArtifactEditor>::natural_file_codec().expect("the DXF editor declares a natural file");
    assert_eq!((codec.format_kind, codec.extension, codec.media_type, codec.binary), ("s.stdio.dxf@r12", ".dxf", "image/vnd.dxf", false));
    let mut base = DxfSnapshot::default();
    base.tables.layers.push(DxfLayer { name: "WALLS".into(), color: 1, linetype: "CONTINUOUS".into(), flags: 0, unknown_group_codes: vec![] });
    base.entities.push(DxfEntity::Line(DxfLine { start: [0.0, 0.0, 0.0], end: [4.0, 0.0, 0.0], layer: "WALLS".into(), unknown_group_codes: vec![] }));
    base.entities.push(DxfEntity::Circle(DxfCircle { center: [2.0, 2.0, 0.0], radius: 1.5, layer: "WALLS".into(), unknown_group_codes: vec![] }));
    let opened = <DxfAnyEditor as ArtifactEditor>::decode_natural_file(&<DxfAnyEditor as ArtifactEditor>::encode_natural_file(&base).unwrap()).expect("saved DXF reopens");
    assert_eq!(opened.entities, base.entities);
    assert_eq!(opened.tables.layers, base.tables.layers);
    let moved = <DxfAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&editing::SnapshotEditEvent::SetValue { path: "/entities/0/end/1".into(), value: semio_framework_value::DslValue::float(3.0) }, &opened).expect("a coordinate edit resolves");
    let [mutation @ DxfMutation::SetEntity(_)] = moved.artifact_mutations.as_slice() else { panic!("a coordinate edit raises the entity kind") };
    let mut edited = opened.clone();
    apply_mutation(&mut edited, mutation);
    let reopened = <DxfAnyEditor as ArtifactEditor>::decode_natural_file(&<DxfAnyEditor as ArtifactEditor>::encode_natural_file(&edited).unwrap()).unwrap();
    let DxfEntity::Line(line) = &reopened.entities[0] else { panic!("the first entity stays a line") };
    assert_eq!(line.end, [4.0, 3.0, 0.0]);
    let independent = semio_s_artifact_stdio_dxf_test_oracle::standards::v_r12::subsets::header::oracle_project_entities(&<DxfAnyEditor as ArtifactEditor>::encode_natural_file(&edited).unwrap()).expect("the dxf reference parser reads the saved file");
    assert_eq!(independent, vec![("line".to_string(), vec![0.0, 0.0, 0.0, 4.0, 3.0, 0.0]), ("circle".to_string(), vec![2.0, 2.0, 0.0, 1.5])]);
    assert_eq!(reopened.entities[1], base.entities[1]);
}

#[semio_framework_async_macros::async_test]
async fn invalid_natural_dxf_files_are_refused_with_a_payload_error() {
    assert!(<DxfAnyEditor as ArtifactEditor>::decode_natural_file(&[0xff, 0xfe, 0x00]).is_err());
    assert!(<DxfAnyEditor as ArtifactEditor>::decode_natural_file(b"not a dxf at all\nfoo").is_err());
}

#[semio_framework_async_macros::async_test]
async fn demo_dxf_is_a_real_drawing_that_the_reference_parser_reads_identically() {
    use crate::schema::snapshot::DxfEntity;
    let bytes = include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🧪️example/🖊️.dxf");
    let opened = <DxfAnyEditor as ArtifactEditor>::decode_natural_file(bytes).expect("the demo is a real DXF R12 document");
    assert_eq!(opened.tables.layers.iter().map(|layer| layer.name.as_str()).collect::<Vec<_>>(), ["0", "WALLS"]);
    let ours: Vec<(String, Vec<f64>)> = opened
        .entities
        .iter()
        .filter_map(|entity| match entity {
            DxfEntity::Line(line) => Some(("line".to_string(), line.start.iter().chain(line.end.iter()).copied().collect())),
            DxfEntity::Circle(circle) => Some(("circle".to_string(), circle.center.iter().copied().chain([circle.radius]).collect())),
            _ => None,
        })
        .collect();
    let independent = semio_s_artifact_stdio_dxf_test_oracle::standards::v_r12::subsets::header::oracle_project_entities(bytes).expect("the dxf reference parser reads the demo");
    assert_eq!(ours.len(), 3);
    assert_eq!(ours, independent);
}
