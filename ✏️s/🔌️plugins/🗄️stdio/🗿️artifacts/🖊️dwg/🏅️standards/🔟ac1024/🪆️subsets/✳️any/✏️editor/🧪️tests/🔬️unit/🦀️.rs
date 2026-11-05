use semio_s_artifact_stdio_contract::apply_mutation;
use super::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_dwg_ac1024_editor();
    assert_eq!(def.role, semio_plugin::AppRole::Editor);
    assert_eq!(def.dialect, DWG_AC1024_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<DwgAc1024Editor as ArtifactEditor>::DIALECT, DWG_AC1024_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    semio_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<DwgAc1024Editor, crate::viewer::dwg_ac1024::DwgAc1024Viewer>().await;
}

semio_plugin::history_edit_acceptance_law!("stdio", super::DwgAc1024Editor, || semio_plugin::App { definition: super::create_dwg_ac1024_editor(), examples: Vec::new() }, "../../..");

#[semio_framework_async_macros::async_test]
async fn details_edits_resolve_to_the_kind_of_the_addressed_block() {
    
    let base = semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::schema::snapshot::demo_dwg_snapshot();
    let title = editing::SnapshotEditEvent::SetValue { path: "/summary/title".into(), value: semio_framework_value::DslValue::String("renamed".into()) };
    let emit = <DwgAc1024Editor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&title, &base).expect("a summary edit resolves");
    let [mutation @ DwgMutation::SetSummary(_)] = emit.artifact_mutations.as_slice() else { panic!("a summary edit raises the summary kind") };
    let mut state = base.clone();
    apply_mutation(&mut state, mutation);
    assert_eq!(state.summary.title, "renamed");
    let codepage = editing::SnapshotEditEvent::SetValue { path: "/codepage".into(), value: semio_framework_value::DslValue::uint(u64::from(base.codepage) + 1) };
    let emit = <DwgAc1024Editor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&codepage, &base).expect("a preamble edit resolves");
    let [mutation @ DwgMutation::SetVersionInfo(_)] = emit.artifact_mutations.as_slice() else { panic!("a preamble edit raises the version-info kind") };
    apply_mutation(&mut state, mutation);
    assert_eq!(state.codepage, base.codepage + 1);
    let stamp = editing::SnapshotEditEvent::SetValue { path: "/schema".into(), value: semio_framework_value::DslValue::String("other".into()) };
    assert_eq!(<DwgAc1024Editor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&stamp, &base).err().expect("the schema stamp has no kind").code.0, "snapshot-edit.unsupported-path");
}

#[semio_framework_async_macros::async_test]
async fn natural_dwg_file_round_trips_a_summary_edit_and_keeps_the_drawing() {
    let codec = <DwgAc1024Editor as ArtifactEditor>::natural_file_codec().expect("the DWG editor declares a natural file");
    assert_eq!((codec.format_kind, codec.extension, codec.media_type, codec.binary), ("s.stdio.dwg@ac1024", ".dwg", "image/vnd.dwg", true));
    let input = include_bytes!("../../../🖼️assets/🏛️architectural/🏛️architectural.dwg");
    let opened = <DwgAc1024Editor as ArtifactEditor>::decode_natural_file(input).expect("a real AC1024 file opens");
    let title = editing::SnapshotEditEvent::SetValue { path: "/summary/title".into(), value: semio_framework_value::DslValue::String("renamed".into()) };
    let emit = <DwgAc1024Editor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&title, &opened).expect("a summary edit resolves");
    let [mutation @ DwgMutation::SetSummary(_)] = emit.artifact_mutations.as_slice() else { panic!("a summary edit raises the summary kind") };
    let mut edited = opened.clone();
    apply_mutation(&mut edited, mutation);
    let saved = <DwgAc1024Editor as ArtifactEditor>::encode_natural_file(&edited).expect("the edited drawing saves");
    let reopened = <DwgAc1024Editor as ArtifactEditor>::decode_natural_file(&saved).expect("the saved file reopens");
    assert_eq!(reopened.summary.title, "renamed");
    assert_eq!(reopened.drawing, opened.drawing);
    assert_eq!(reopened.header, opened.header);
}

#[semio_framework_async_macros::async_test]
async fn invalid_natural_dwg_files_are_refused_with_a_payload_error() {
    assert!(<DwgAc1024Editor as ArtifactEditor>::decode_natural_file(b"not a dwg").is_err());
    assert!(<DwgAc1024Editor as ArtifactEditor>::decode_natural_file(&[]).is_err());
}

fn layer_records(snapshot: &DwgSnapshot) -> Vec<(u64, String)> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgTableRecordBody;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    snapshot
        .drawing
        .objects
        .iter()
        .filter_map(|object| match object.body.as_ref() {
            Some(DwgLogicalObjectBody::TableRecord(DwgTableRecordBody::Layer(record))) => Some((object.handle, record.common.name.clone())),
            _ => None,
        })
        .collect()
}

fn layer_control_entries(snapshot: &DwgSnapshot) -> Vec<u64> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgTableControlBody;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    snapshot
        .drawing
        .objects
        .iter()
        .find_map(|object| match object.body.as_ref() {
            Some(DwgLogicalObjectBody::TableControl(DwgTableControlBody::Layer(entries))) => Some(entries.entry_handles.iter().filter_map(|entry| entry.handle).collect()),
            _ => None,
        })
        .unwrap_or_default()
}

fn new_layer(name: &str, color: u8) -> semio_s_artifact_stdio_dwg_snapshot_drawing::standards::v_ac1024::subsets::any::schema::snapshot::drawing::geometry::DwgLogicalLayer {
    semio_s_artifact_stdio_dwg_snapshot_drawing::standards::v_ac1024::subsets::any::schema::snapshot::drawing::geometry::DwgLogicalLayer { name: name.into(), color }
}

fn used_layer_name(snapshot: &DwgSnapshot) -> String {
    let zero = snapshot.drawing.layers.iter().position(|layer| layer.name == "0");
    let entity = snapshot.drawing.entities().into_iter().find(|entity| Some(entity.layer) != zero).expect("the fixture draws on a layer other than 0");
    snapshot.drawing.layers[entity.layer].name.clone()
}

fn pointer_to_line_end(value: &semio_framework_value::DslValue, prefix: String) -> Option<String> {
    if let Some(entries) = value.as_object() {
        if entries.iter().any(|(key, _)| key == "start") && entries.iter().any(|(key, _)| key == "end") {
            return Some(format!("{prefix}/end/1"));
        }
        return entries.iter().find_map(|(key, child)| pointer_to_line_end(child, format!("{prefix}/{key}")));
    }
    value.as_array()?.iter().enumerate().find_map(|(index, child)| pointer_to_line_end(child, format!("{prefix}/{index}")))
}

#[semio_framework_async_macros::async_test]
async fn moving_a_line_endpoint_edits_the_drawing_object_and_round_trips_through_a_dwg_file() {
    use semio_framework_value::ToValue;
    let input = include_bytes!("../../../🖼️assets/🏛️architectural/🏛️architectural.dwg");
    let opened = <DwgAc1024Editor as ArtifactEditor>::decode_natural_file(input).expect("a real AC1024 file opens");
    let path = pointer_to_line_end(&opened.to_value(), String::new()).expect("the fixture holds a typed line");
    assert!(path.starts_with("/drawing/objects/"), "{path}");
    let emit = <DwgAc1024Editor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&editing::SnapshotEditEvent::SetValue { path: path.clone(), value: semio_framework_value::DslValue::float(1234.5) }, &opened).expect("an entity coordinate edit resolves");
    let [mutation @ DwgMutation::SetDrawing(_)] = emit.artifact_mutations.as_slice() else { panic!("an entity edit raises the drawing kind") };
    let mut edited = opened.clone();
    apply_mutation(&mut edited, mutation);
    assert_ne!(edited.drawing, opened.drawing);
    let saved = <DwgAc1024Editor as ArtifactEditor>::encode_natural_file(&edited).expect("the edited drawing saves; refusal here is the DWG object writer gap");
    let reopened = <DwgAc1024Editor as ArtifactEditor>::decode_natural_file(&saved).expect("the saved file reopens");
    assert_eq!(reopened.drawing, edited.drawing);
}

#[semio_framework_async_macros::async_test]
async fn renaming_a_layer_is_written_to_its_layer_table_record_and_survives_reopen() {
    let input = include_bytes!("../../../🖼️assets/🏛️architectural/🏛️architectural.dwg");
    let opened = <DwgAc1024Editor as ArtifactEditor>::decode_natural_file(input).expect("a real AC1024 file opens");
    let old_name = opened.drawing.layers.first().expect("the fixture has layers").name.clone();
    let rename = editing::SnapshotEditEvent::SetValue { path: "/drawing/layers/0/name".into(), value: semio_framework_value::DslValue::String("RENAMED-LAYER".into()) };
    let emit = <DwgAc1024Editor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&rename, &opened).expect("a layer edit resolves");
    let [mutation @ DwgMutation::SetDrawing(_)] = emit.artifact_mutations.as_slice() else { panic!("a layer edit raises the drawing kind") };
    let mut edited = opened.clone();
    apply_mutation(&mut edited, mutation);
    let saved = <DwgAc1024Editor as ArtifactEditor>::encode_natural_file(&edited).expect("the renamed layer saves");
    let reopened = <DwgAc1024Editor as ArtifactEditor>::decode_natural_file(&saved).expect("the saved file reopens");
    let names: Vec<&str> = reopened.drawing.layers.iter().map(|layer| layer.name.as_str()).collect();
    assert!(names.contains(&"RENAMED-LAYER"), "{names:?}");
    assert!(old_name == "RENAMED-LAYER" || !names.contains(&old_name.as_str()), "the old layer name must be gone: {names:?}");
    assert_eq!(reopened.drawing.layers.len(), opened.drawing.layers.len());
    let before = layer_records(&opened).into_iter().find(|(_, name)| *name == old_name).expect("the old record exists").0;
    let after = layer_records(&reopened).into_iter().find(|(_, name)| name == "RENAMED-LAYER").expect("the renamed record exists").0;
    assert_eq!(before, after, "a rename keeps the record handle");
}

#[semio_framework_async_macros::async_test]
async fn adding_and_removing_a_layer_writes_and_deletes_its_table_record() {
    let input = include_bytes!("../../../🖼️assets/🏛️architectural/🏛️architectural.dwg");
    let opened = <DwgAc1024Editor as ArtifactEditor>::decode_natural_file(input).expect("a real AC1024 file opens");
    let mut grown = opened.clone();
    grown.drawing.layers.push(semio_s_artifact_stdio_dwg_snapshot_drawing::standards::v_ac1024::subsets::any::schema::snapshot::drawing::geometry::DwgLogicalLayer { name: "NEW-LAYER".into(), color: 3 });
    let saved = <DwgAc1024Editor as ArtifactEditor>::encode_natural_file(&grown).expect("the added layer saves");
    let reopened = <DwgAc1024Editor as ArtifactEditor>::decode_natural_file(&saved).expect("the saved file reopens");
    let added = reopened.drawing.layers.iter().find(|layer| layer.name == "NEW-LAYER").expect("the new layer survives reopen");
    assert_eq!(added.color, 3);
    assert_eq!(reopened.drawing.layers.len(), opened.drawing.layers.len() + 1);
    let new_handle = layer_records(&reopened).into_iter().find(|(_, name)| name == "NEW-LAYER").expect("the new record exists").0;
    assert!(!layer_records(&opened).iter().any(|(handle, _)| *handle == new_handle), "the new record takes a fresh handle");
    assert!(layer_control_entries(&reopened).contains(&new_handle), "the layer control lists the new record");
    let mut shrunk = reopened.clone();
    shrunk.drawing.layers.retain(|layer| layer.name != "NEW-LAYER");
    let saved = <DwgAc1024Editor as ArtifactEditor>::encode_natural_file(&shrunk).expect("the removal saves");
    let again = <DwgAc1024Editor as ArtifactEditor>::decode_natural_file(&saved).expect("the saved file reopens");
    assert_eq!(again.drawing.layers.len(), opened.drawing.layers.len());
    assert!(again.drawing.layers.iter().all(|layer| layer.name != "NEW-LAYER"));
    assert!(!layer_control_entries(&again).contains(&new_handle), "the layer control no longer lists the removed record");
    let mut without_zero = opened.clone();
    without_zero.drawing.layers.retain(|layer| layer.name != "0");
    let refusal = <DwgAc1024Editor as ArtifactEditor>::encode_natural_file(&without_zero).err().expect("layer 0 stays");
    assert!(refusal.to_string().contains("layer 0"), "{refusal}");
}

#[semio_framework_async_macros::async_test]
async fn adding_one_layer_while_deleting_a_used_one_is_refused_not_turned_into_a_rename() {
    let input = include_bytes!("../../../🖼️assets/🏛️architectural/🏛️architectural.dwg");
    let opened = <DwgAc1024Editor as ArtifactEditor>::decode_natural_file(input).expect("a real AC1024 file opens");
    let used = used_layer_name(&opened);
    let mut edited = opened.clone();
    edited.drawing.layers.retain(|layer| layer.name != used);
    edited.drawing.layers.push(new_layer("REPLACEMENT", 5));
    let refusal = <DwgAc1024Editor as ArtifactEditor>::encode_natural_file(&edited).err().expect("the used layer cannot be deleted");
    assert!(refusal.to_string().contains("still used"), "{refusal}");
}

#[semio_framework_async_macros::async_test]
async fn layer_zero_cannot_be_renamed() {
    let input = include_bytes!("../../../🖼️assets/🏛️architectural/🏛️architectural.dwg");
    let mut edited = <DwgAc1024Editor as ArtifactEditor>::decode_natural_file(input).expect("a real AC1024 file opens");
    let layer = edited.drawing.layers.iter_mut().find(|layer| layer.name == "0").expect("layer 0 exists");
    layer.name = "NOT-ZERO".into();
    let refusal = <DwgAc1024Editor as ArtifactEditor>::encode_natural_file(&edited).err().expect("layer 0 keeps its name");
    assert!(refusal.to_string().contains("layer 0"), "{refusal}");
}

#[semio_framework_async_macros::async_test]
async fn a_removed_maximum_handle_is_not_reused_by_the_next_record() {
    let input = include_bytes!("../../../🖼️assets/🏛️architectural/🏛️architectural.dwg");
    let opened = <DwgAc1024Editor as ArtifactEditor>::decode_natural_file(input).expect("a real AC1024 file opens");
    let highest = opened.drawing.objects.iter().map(|object| object.handle).max().unwrap();
    let mut grown = opened.clone();
    grown.drawing.layers.push(new_layer("FIRST", 2));
    let reopened = <DwgAc1024Editor as ArtifactEditor>::decode_natural_file(&<DwgAc1024Editor as ArtifactEditor>::encode_natural_file(&grown).unwrap()).unwrap();
    let first = layer_records(&reopened).into_iter().find(|(_, name)| name == "FIRST").unwrap().0;
    assert!(first > highest);
    let mut removed = reopened.clone();
    removed.drawing.layers.retain(|layer| layer.name != "FIRST");
    let shrunk = <DwgAc1024Editor as ArtifactEditor>::decode_natural_file(&<DwgAc1024Editor as ArtifactEditor>::encode_natural_file(&removed).unwrap()).unwrap();
    assert!(layer_records(&shrunk).iter().all(|(_, name)| name != "FIRST"));
    let mut swapped = shrunk.clone();
    swapped.drawing.layers.push(new_layer("SECOND", 4));
    let again = <DwgAc1024Editor as ArtifactEditor>::decode_natural_file(&<DwgAc1024Editor as ArtifactEditor>::encode_natural_file(&swapped).unwrap()).unwrap();
    let second = layer_records(&again).into_iter().find(|(_, name)| name == "SECOND").unwrap().0;
    assert!(second > first, "the removed record's handle {first:#x} is not reused (got {second:#x})");
}
