use super::*;
use protocol::{Mutation, MutationDiff};
use protocol::command::DiffAlgebra;
use semio_framework_plugin::ArtifactEditor;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap()
}

#[test]
fn archive_target_resolution_yields_without_copying_entry_payloads() {
    let row = fixture();
    let mut snapshot: ZipSnapshot = dsl::os_pack::json::from_json_str(&row["snapshot"].to_string()).unwrap();
    snapshot.entries[0].data.resize(row["retainedResolution"]["preservedPayloadBytes"].as_u64().unwrap() as usize, 42);
    let owner = snapshot.entries[0].data.as_ptr();
    let mut cursor = retained::ArchiveTextCursor::default();
    let name = row["rename"]["name"].as_str().unwrap();
    let value = row["rename"]["value"].as_str().unwrap();
    let node = entry_node_id(name);
    let revision = text_revision(name);
    for index in 0..snapshot.entries.len() {
        assert!(cursor.advance(&snapshot, &node, value, &revision).unwrap().is_none());
        assert_eq!(cursor.scanned_entries() - index, row["retainedResolution"]["maximumEntriesPerStep"].as_u64().unwrap() as usize);
        assert_eq!(snapshot.entries[0].data.as_ptr(), owner);
    }
    let emit = cursor.advance(&snapshot, &node, value, &revision).unwrap().unwrap();
    assert_eq!(emit.artifact_mutations, edit_node(&snapshot, &node, value, &revision).unwrap().artifact_mutations);
    println!("[DEBUG] ZIP target resolution yields once per member and leaves the 2 MiB payload owner unchanged");
}

#[test]
fn archive_diffs_preserve_explicit_member_order_and_exact_inverse() {
    let row = fixture();
    let snapshot: ZipSnapshot = dsl::os_pack::json::from_json_str(&row["snapshot"].to_string()).unwrap();
    let mut reversed = snapshot.clone();
    reversed.entries.reverse();
    let diff = crate::schema::diff::ZipDiff::between(&snapshot, &reversed);
    assert_eq!(diff.apply(&snapshot).unwrap(), reversed);
    assert_eq!(diff.inverse(&snapshot).apply(&reversed).unwrap(), snapshot);
    let rename = crate::schema::diff::diff_rename_entry(&reversed.entries[0].name, "renamed.txt");
    let expected = rename.apply(&reversed).unwrap();
    let mut combined = diff;
    combined.absorb(rename);
    assert_eq!(combined.apply(&snapshot).unwrap(), expected);
    for order in row["entryOrdering"]["invalidOrders"].as_array().unwrap() {
        let value = serde_json::json!({"entries": {"order": order}});
        let diff: crate::schema::diff::ZipDiff = dsl::os_pack::json::from_json_str(&value.to_string()).unwrap();
        assert!(diff.apply(&snapshot).is_err());
    }
}

#[test]
fn archive_save_preserves_member_order_with_the_independent_zip_reader() {
    let row = fixture();
    let mut snapshot: ZipSnapshot = dsl::os_pack::json::from_json_str(&row["snapshot"].to_string()).unwrap();
    snapshot.entries.reverse();
    let bytes = crate::standards::v2_0::subsets::base::io::encode_zip(&snapshot).unwrap();
    let mut oracle = zip::ZipArchive::new(std::io::Cursor::new(&bytes)).unwrap();
    let names: Vec<String> = (0..oracle.len()).map(|index| oracle.by_index(index).unwrap().name().to_string()).collect();
    assert_eq!(names, snapshot.entries.iter().map(|entry| entry.name.clone()).collect::<Vec<_>>());
    assert_eq!(crate::standards::v2_0::subsets::base::io::decode_zip(&bytes).unwrap(), snapshot);
}

#[test]
fn removing_each_archive_member_undoes_to_its_exact_original_position() {
    let row = fixture();
    let snapshot: ZipSnapshot = dsl::os_pack::json::from_json_str(&row["snapshot"].to_string()).unwrap();
    for entry in &snapshot.entries {
        let mutation = ZipMutation::RemoveEntry(crate::schema::mutations::remove_entry::RemoveEntry { name: entry.name.clone() });
        let next = mutation.diff(&snapshot).diff().apply(&snapshot).unwrap();
        let inverse = mutation.inverse(&snapshot);
        assert_eq!(inverse.len(), 1);
        assert_eq!(inverse[0].diff(&next).diff().apply(&next).unwrap(), snapshot);
    }
}

#[test]
fn archive_insertions_use_the_neutral_following_member_anchor() {
    let row = fixture();
    let snapshot: ZipSnapshot = dsl::os_pack::json::from_json_str(&row["snapshot"].to_string()).unwrap();
    let entry: crate::schema::snapshot::ZipEntry = dsl::os_pack::json::from_json_str(&row["insertion"]["entry"].to_string()).unwrap();
    let mutation = ZipMutation::AddEntry(crate::schema::mutations::add_entry::AddEntry { entry, before: Some(row["insertion"]["before"].as_str().unwrap().into()) });
    let next = mutation.diff(&snapshot).diff().apply(&snapshot).unwrap();
    let mut expected = row["snapshot"].clone();
    expected["entries"].as_array_mut().unwrap().insert(row["insertion"]["index"].as_u64().unwrap() as usize, row["insertion"]["entry"].clone());
    let actual: serde_json::Value = serde_json::from_str(&dsl::os_pack::json::to_json_string(&next)).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn archive_parser_supplies_the_neutral_empty_defaults() {
    let row = fixture();
    let snapshot: ZipSnapshot = dsl::os_pack::json::from_json_str(&row["omittedDefaults"].to_string()).unwrap();
    let actual: serde_json::Value = serde_json::from_str(&dsl::os_pack::json::to_json_string(&snapshot)).unwrap();
    assert_eq!(actual, row["defaultSnapshot"]);
}

#[test]
fn archive_text_byte_limits_match_the_neutral_unicode_boundaries() {
    let snapshot = ZipSnapshot::default();
    for row in fixture()["textByteBoundaries"].as_array().unwrap() {
        let value = row["text"].as_str().unwrap().repeat(row["repeat"].as_u64().unwrap() as usize);
        assert_eq!(edit_node(&snapshot, COMMENT_NODE_ID, &value, &text_revision(&snapshot.comment)).is_ok(), row["valid"].as_bool().unwrap());
        let arguments = dsl::DslValue::object([
            ("nodeId".into(), dsl::DslValue::String(COMMENT_NODE_ID.into())),
            ("value".into(), dsl::DslValue::String(value)),
            ("revision".into(), dsl::DslValue::String(text_revision(&snapshot.comment))),
        ]);
        assert_eq!(edit_arguments(Some(&arguments)).is_ok(), row["valid"].as_bool().unwrap());
    }
}

#[test]
fn archive_edit_arguments_are_required_in_both_dialects() {
    let rows = fixture();
    for row in rows["rejectedArguments"].as_array().unwrap() {
        let args: dsl::DslValue = dsl::os_pack::json::from_json_str(&row.to_string()).unwrap();
        assert!(crate::editor::zip::base::ZipAnyEditor::command_from_action("set-node", Some(&args)).is_err(), "{row}");
        assert!(crate::editor::zip::iso21320::ZipIso21320Editor::command_from_action("set-node", Some(&args)).is_err(), "{row}");
    }
}

#[test]
fn archive_rename_survives_reordering_and_matches_independent_json_oracle() {
    let row = fixture();
    let mut input = row["snapshot"].clone();
    let original_name = row["rename"]["name"].as_str().unwrap();
    let replacement = row["rename"]["value"].as_str().unwrap();
    let node = entry_node_id(original_name);
    input["entries"].as_array_mut().unwrap().reverse();
    let snapshot: ZipSnapshot = dsl::os_pack::json::from_json_str(&input.to_string()).unwrap();
    let emit = edit_node(&snapshot, &node, replacement, &text_revision(original_name)).unwrap();
    assert_eq!(emit.artifact_mutations.len(), 1);
    let mutation = &emit.artifact_mutations[0];
    let next = MutationDiff::apply(mutation.diff(&snapshot).diff(), &snapshot).unwrap();
    let mut expected = input.clone();
    let entry = expected["entries"].as_array_mut().unwrap().iter_mut().find(|entry| entry["name"] == original_name).unwrap();
    entry["name"] = replacement.into();
    let actual: serde_json::Value = serde_json::from_str(&dsl::os_pack::json::to_json_string(&dsl::ToValue::to_value(&next))).unwrap();
    assert_eq!(actual, expected);
    let inverse = mutation.inverse(&snapshot);
    assert_eq!(inverse.len(), 1);
    assert_eq!(MutationDiff::apply(inverse[0].diff(&next).diff(), &next).unwrap(), snapshot);
    assert!(edit_node(&next, &node, "stale.txt", &text_revision(original_name)).is_err());
    println!("[DEBUG] ZIP rename preserves entry identity across reordering and exactly undoes the native mutation");
}

#[test]
fn archive_edits_refuse_ambiguous_stale_and_colliding_names() {
    let row = fixture();
    let snapshot: ZipSnapshot = dsl::os_pack::json::from_json_str(&row["snapshot"].to_string()).unwrap();
    for target in row["rejectedTargets"].as_array().unwrap() {
        assert!(edit_node(&snapshot, target.as_str().unwrap(), "changed", "stale").is_err());
    }
    let node = entry_node_id(&snapshot.entries[0].name);
    assert!(edit_node(&snapshot, &node, &snapshot.entries[1].name, &text_revision(&snapshot.entries[0].name)).is_err());
    assert!(edit_node(&snapshot, &node, "", &text_revision(&snapshot.entries[0].name)).is_err());
    assert!(edit_node(&snapshot, &node, &"ä".repeat(MAXIMUM_TEXT_BYTES / 2 + 1), &text_revision(&snapshot.entries[0].name)).is_err());
    assert!(edit_node(&snapshot, &node, &snapshot.entries[0].name, &text_revision(&snapshot.entries[0].name)).unwrap().artifact_mutations.is_empty());
    let mut duplicate = snapshot.clone();
    duplicate.entries.push(snapshot.entries[0].clone());
    assert!(edit_node(&duplicate, &node, "unique.txt", &text_revision(&snapshot.entries[0].name)).is_err());
    let comment = edit_node(&snapshot, COMMENT_NODE_ID, row["emptyComment"].as_str().unwrap(), &text_revision(&snapshot.comment)).unwrap();
    let next = MutationDiff::apply(comment.artifact_mutations[0].diff(&snapshot).diff(), &snapshot).unwrap();
    assert!(next.comment.is_empty());
    assert!(edit_node(&next, COMMENT_NODE_ID, "stale comment", &text_revision(&snapshot.comment)).is_err());
    assert_eq!(next.entries, snapshot.entries);
}

#[test]
fn archive_primary_window_materializes_only_the_requested_slice() {
    use semio_framework_plugin::{TreeWindowRequest, ViewModel};
    let snapshot = ZipSnapshot { entries: (0..300).map(|index| crate::schema::snapshot::ZipEntry { name: format!("file-{index}.txt"), data: vec![] }).collect(), ..Default::default() };
    let mounts = [
        (crate::editor::zip::base::modes::edit::windows::main::BODY_KEY, crate::editor::zip::base::modes::edit::windows::main::render as fn(&ZipSnapshot, &TreeWindows<'_>, Locale) -> UiAssemblyResult<BuiltNode>),
        (crate::editor::zip::iso21320::modes::edit::windows::main::BODY_KEY, crate::editor::zip::iso21320::modes::edit::windows::main::render),
    ];
    for (body_key, render) in mounts {
        let view = ViewModel {
            tree_windows: vec![TreeWindowRequest { body_key: body_key.into(), node_key: "archive-fields".into(), open: Some(true), offset: 100, rows: 4 }],
            tree_viewport_rows: Some(4),
            ..Default::default()
        };
        let node = render(&snapshot, &TreeWindows::for_body(&view, body_key), Locale::En).unwrap();
        let projection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).unwrap();
        assert!(projection.contains("\"total\":301"));
        assert!(projection.contains("file-99.txt"));
        assert!(projection.contains("file-102.txt"));
        assert!(!projection.contains("file-98.txt"));
        assert!(!projection.contains("file-103.txt"));
    }
}

async fn retained_archive_law<E: ArtifactEditor<Snapshot = ZipSnapshot, Mutation = ZipMutation>>(definition: semio_framework_plugin::AppDefinition) {
    use semio_framework_plugin::{artifact_app_laws, EditorApp, PluginApp};
    let row = fixture();
    let original: ZipSnapshot = dsl::os_pack::json::from_json_str(&row["snapshot"].to_string()).unwrap();
    let mut app = artifact_app_laws::new_registered_app::<EditorApp<E>, _>(async { semio_framework_plugin::App { definition, examples: Vec::new() } }).await;
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(&original, crate::STDIO_ZIP_DOCUMENT_SCHEMA) else { panic!("archive fixture produces a document load") };
    app.load_document_pack(&store::ArtifactPackFiles { pack, spr, ops: String::new() }).await.unwrap();
    let name = row["rename"]["name"].as_str().unwrap();
    let value = row["rename"]["value"].as_str().unwrap();
    let arguments = dsl::DslValue::object([
        ("nodeId".into(), dsl::DslValue::String(entry_node_id(name))),
        ("value".into(), dsl::DslValue::String(value.into())),
        ("revision".into(), dsl::DslValue::String(text_revision(name))),
    ]);
    let meta = artifact_app_laws::meta("local");
    app.handle_action("set-node", Some(&arguments), &meta).await.unwrap();
    artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.unwrap();
    let mut expected = original.clone();
    expected.entries[0].name = value.into();
    assert_eq!(app.snapshot().unwrap(), expected);
    artifact_app_laws::settle_history_verb(&mut app, "undo", meta.instance_id).await;
    assert_eq!(app.snapshot().unwrap(), original);
    artifact_app_laws::settle_history_verb(&mut app, "redo", meta.instance_id).await;
    assert_eq!(app.snapshot().unwrap(), expected);
    let saved = <ZipSnapshot as store::ArtifactPack>::encode_pack(&expected);
    assert_eq!(<ZipSnapshot as store::ArtifactPack>::decode_pack(&saved).unwrap(), expected);
    println!("[DEBUG] ZIP primary draft publishes through its exact retained factory, undoes, redoes, and reopens without changing entry bytes");
    artifact_app_laws::close_registered_fixture_app(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn base_archive_draft_uses_the_registered_retained_factory() {
    retained_archive_law::<crate::editor::zip::base::ZipAnyEditor>(crate::editor::zip::base::create_zip_any_editor()).await;
}

#[semio_framework_async_macros::async_test]
async fn iso_archive_draft_uses_the_registered_retained_factory() {
    retained_archive_law::<crate::editor::zip::iso21320::ZipIso21320Editor>(crate::editor::zip::iso21320::create_zip_iso21320_editor()).await;
}
