use super::*;
use protocol::command::DiffAlgebra;
use protocol::{Mutation, MutationDiff};
use semio_framework_plugin::ArtifactEditor;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap()
}

#[test]
fn archive_cursor_checkpoint_matches_the_neutral_wire_and_resumes() {
    let row = fixture();
    let snapshot: ZipSnapshot = dsl::os_pack::json::from_json_str(&row["snapshot"].to_string()).unwrap();
    let name = row["rename"]["name"].as_str().unwrap();
    let value = row["rename"]["value"].as_str().unwrap();
    let node = entry_node_id(name);
    let revision = text_revision(name);
    let mut cursor = retained::ArchiveTextCursor::default();
    let resume_after = row["retainedResolution"]["resumeAfterEntries"].as_u64().unwrap() as usize;
    for _ in 0..resume_after {
        assert!(cursor.advance(&snapshot, &node, value, &revision).unwrap().is_none());
    }
    let mut bytes = [0; retained::CHECKPOINT_BYTES];
    assert_eq!(cursor.checkpoint(&mut bytes).unwrap(), row["retainedResolution"]["checkpointBytes"].as_u64().unwrap() as usize);
    assert_eq!(&bytes[..4], row["retainedResolution"]["checkpointMagic"].as_str().unwrap().as_bytes());
    assert_eq!(bytes[4], row["retainedResolution"]["checkpointVersion"].as_u64().unwrap() as u8);
    assert_eq!(bytes[5], 9);
    assert_eq!(u64::from_le_bytes(bytes[8..16].try_into().unwrap()), resume_after as u64);
    assert_eq!(u64::from_le_bytes(bytes[24..32].try_into().unwrap()), snapshot.entries.len() as u64);
    assert_eq!(&bytes[32..], text_revision(&format!("{node}\0{revision}\0{value}")).as_bytes());
    assert!(cursor.advance(&snapshot, &node, "changed while scanning", &revision).is_err());
    let mut unchanged = [0; retained::CHECKPOINT_BYTES];
    cursor.checkpoint(&mut unchanged).unwrap();
    assert_eq!(unchanged, bytes);
    let mut resumed = retained::ArchiveTextCursor::default();
    resumed.restore(&bytes).unwrap();
    assert_eq!(resumed.scanned_entries(), resume_after);
    for _ in resume_after..snapshot.entries.len() {
        assert!(resumed.advance(&snapshot, &node, value, &revision).unwrap().is_none());
    }
    assert_eq!(resumed.advance(&snapshot, &node, value, &revision).unwrap().unwrap().artifact_mutations, edit_node(&snapshot, &node, value, &revision).unwrap().artifact_mutations);
    resumed.restore(&bytes).unwrap();
    assert!(resumed.advance(&snapshot, &node, "different.txt", &revision).is_err());
    for (index, value) in [(0, 0), (4, 2), (5, 255), (6, 1), (8, 255), (16, 255), (32, 255)] {
        let mut invalid = bytes;
        invalid[index] = value;
        assert!(retained::ArchiveTextCursor::default().restore(&invalid).is_err());
    }
    assert!(retained::ArchiveTextCursor::default().restore(&bytes[1..]).is_err());
    println!("[DEBUG] ZIP resumes a serialized cursor after one scanned member and refuses changed command or malformed checkpoint");
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
    use protocol::{OpBinary, OpText};
    let row = fixture();
    let snapshot: ZipSnapshot = dsl::os_pack::json::from_json_str(&row["snapshot"].to_string()).unwrap();
    let entry: crate::schema::snapshot::ZipEntry = dsl::os_pack::json::from_json_str(&row["insertion"]["entry"].to_string()).unwrap();
    let mutation = ZipMutation::AddEntry(crate::schema::mutations::add_entry::AddEntry { entry, before: Some(row["insertion"]["before"].as_str().unwrap().into()) });
    assert_eq!(ZipMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
    assert_eq!(ZipMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
    let next = mutation.diff(&snapshot).diff().apply(&snapshot).unwrap();
    let mut expected = row["snapshot"].clone();
    expected["entries"].as_array_mut().unwrap().insert(row["insertion"]["index"].as_u64().unwrap() as usize, row["insertion"]["entry"].clone());
    let actual: serde_json::Value = serde_json::from_str(&dsl::os_pack::json::to_json_string(&next)).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn archive_snapshot_and_diff_refuse_the_neutral_invalid_bytes() {
    let row = fixture();
    for byte in row["invalidPayloadBytes"].as_array().unwrap() {
        let snapshot = serde_json::json!({ "schema": "stdio.zip", "entries": [{ "name": "invalid.bin", "data": [byte] }] });
        let diff = serde_json::json!({ "entries": { "modified": [{ "name": "invalid.bin", "diff": { "data": [byte] } }] } });
        assert!(dsl::os_pack::json::from_json_str::<ZipSnapshot>(&snapshot.to_string()).is_err());
        assert!(dsl::os_pack::json::from_json_str::<crate::schema::diff::ZipDiff>(&diff.to_string()).is_err());
    }
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
        let arguments = dsl::DslValue::object([("nodeId".into(), dsl::DslValue::String(COMMENT_NODE_ID.into())), ("value".into(), dsl::DslValue::String(value)), ("revision".into(), dsl::DslValue::String(text_revision(&snapshot.comment)))]);
        assert_eq!(edit_arguments(Some(&arguments)).is_ok(), row["valid"].as_bool().unwrap());
    }
}

#[test]
fn cp437_comment_edit_promotes_saves_reopens_and_undoes_exactly() {
    let source = ZipSnapshot { comment: "é".into(), comment_utf8: false, ..Default::default() };
    let emit = edit_node(&source, COMMENT_NODE_ID, "edited 🎒", &text_revision(&source.comment)).unwrap();
    let mutation = emit.artifact_mutations.into_iter().next().expect("comment mutation");
    assert_eq!(
        mutation,
        ZipMutation::SetArchiveComment(set_archive_comment::SetArchiveComment { comment: "edited 🎒".into(), comment_utf8: true })
    );
    let next = mutation.diff(&source).diff().apply(&source).unwrap();
    let encoded = crate::standards::v2_0::subsets::base::io::encode_zip(&next).unwrap();
    let oracle = zip::ZipArchive::new(std::io::Cursor::new(&encoded)).unwrap();
    assert_eq!(oracle.comment(), "edited 🎒".as_bytes());
    let reopened = crate::standards::v2_0::subsets::base::io::decode_zip(&encoded).unwrap();
    assert_eq!(reopened, next);

    let inverse = mutation.inverse(&source).into_iter().next().expect("comment inverse");
    let restored = inverse.diff(&reopened).diff().apply(&reopened).unwrap();
    let restored_bytes = crate::standards::v2_0::subsets::base::io::encode_zip(&restored).unwrap();
    let restored_oracle = zip::ZipArchive::new(std::io::Cursor::new(&restored_bytes)).unwrap();
    assert_eq!(restored_oracle.comment(), &[0x82]);
    assert_eq!(crate::standards::v2_0::subsets::base::io::decode_zip(&restored_bytes).unwrap(), source);
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
    let snapshot = ZipSnapshot {
        entries: (0..300).map(|index| crate::schema::snapshot::ZipEntry { name: format!("file-{index}.txt"), data: vec![], ..Default::default() }).collect(),
        ..Default::default()
    };
    let mounts = [
        (crate::editor::zip::base::modes::edit::windows::main::BODY_KEY, crate::editor::zip::base::modes::edit::windows::main::render as fn(&ZipSnapshot, &TreeWindows<'_>, Locale) -> UiAssemblyResult<BuiltNode>),
        (crate::editor::zip::iso21320::modes::edit::windows::main::BODY_KEY, crate::editor::zip::iso21320::modes::edit::windows::main::render),
    ];
    for (body_key, render) in mounts {
        let view = ViewModel { tree_windows: vec![TreeWindowRequest { body_key: body_key.into(), node_key: "archive-fields".into(), open: Some(true), offset: 100, rows: 4 }], tree_viewport_rows: Some(4), ..Default::default() };
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
    let arguments = dsl::DslValue::object([("nodeId".into(), dsl::DslValue::String(entry_node_id(name))), ("value".into(), dsl::DslValue::String(value.into())), ("revision".into(), dsl::DslValue::String(text_revision(name)))]);
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

#[test]
fn archive_registered_factory_resumes_the_exact_cursor_and_refuses_another_revision() {
    use protocol::OpBinary;
    use semio_framework_job::{Generation, InteractiveJob, Operation, OperationId, RevisionId, StepBudget, StepContext, StepOutcome};
    use semio_framework_plugin::action_bus::{ActionBus, ToolOperationSpec, ToolWirePage, TOOL_WIRE_PAGE_BYTES};
    use semio_framework_plugin::app::{AppOperationContext, ArtifactOwnedToolJobContext, ArtifactOwnedToolJobSnapshots, ArtifactToolCompletion, ChildContentView, InteractionHoverState};
    use semio_framework_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandPayload};
    use semio_framework_plugin::{EditorApp, HistoryView};
    use semio_s_artifact_stdio_contract::editing::{BoundedNativeEditToolJobFactory, BoundedNativeEditingEditor};
    use std::sync::Arc;
    type Editor = crate::editor::zip::base::ZipAnyEditor;
    type App = EditorApp<Editor>;
    let row = fixture();
    let snapshot: Arc<ZipSnapshot> = Arc::new(dsl::os_pack::json::from_json_str(&row["snapshot"].to_string()).unwrap());
    let arguments = dsl::DslValue::object([
        ("nodeId".into(), dsl::DslValue::String(entry_node_id(row["rename"]["name"].as_str().unwrap()))),
        ("value".into(), dsl::DslValue::String(row["rename"]["value"].as_str().unwrap().into())),
        ("revision".into(), dsl::DslValue::String(text_revision(row["rename"]["name"].as_str().unwrap()))),
    ]);
    let bus = ActionBus::new();
    bus.register(BoundedNativeEditToolJobFactory::<Editor>::new("zip-replay")).unwrap();
    let operation = Operation::new(OperationId(41), RevisionId(2), Generation(3), 5);
    let wire = Editor::command_from_action("set-node", Some(&arguments)).unwrap().encode_op().unwrap();
    let input = |bytes: &[u8]| {
        let (admission, mut input) = bus.begin_exact_wire("zip-replay", "set-node", Editor::NATIVE_PAYLOAD_SCHEMA, bytes.len()).unwrap();
        for page in bytes.chunks(TOOL_WIRE_PAGE_BYTES) {
            input.admit_page(ToolWirePage::try_copy_from(page).unwrap()).map_err(|(fault, _)| fault).unwrap();
        }
        input.seal().unwrap();
        (admission, input)
    };
    let payload = |completion, revision| {
        ArtifactRetainedCommandPayload::<App>::try_new(
            ArtifactRetainedCommandInputs {
                command: Editor::command_from_action("set-node", Some(&arguments)).unwrap(),
                snapshot: snapshot.clone(),
                config: Arc::new(Default::default()),
                history: Arc::new(HistoryView::empty()),
                interaction_state: Arc::new(Default::default()),
                interaction_hover: Arc::new(InteractionHoverState::new()),
                context: Some(Arc::new(ArtifactOwnedToolJobContext::new(
                    7,
                    None,
                    revision,
                    0,
                    0,
                    ArtifactOwnedToolJobSnapshots { children: Arc::new(ChildContentView::EMPTY), draft: Arc::new(Default::default()), transient: Arc::new(Default::default()), window_config: None, window_transient: None },
                ))),
                operation: AppOperationContext { app_instance_id: 7, parent_document_id: "zip-replay".into(), operation_id: 41, generation: 3, canonical_base_revision: revision },
                completion,
            },
            Editor::command_id,
            Editor::NATIVE_MAXIMUM_RAW_BYTES,
            Editor::NATIVE_MAXIMUM_WORK_ITEMS,
            Editor::native_edit_work("set-node"),
        )
        .unwrap()
    };
    let retire = |payload: &mut semio_framework_job::RetainedJobPayload| {
        while !payload.terminal_is_empty() {
            let _ = payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        }
    };
    let completion = ArtifactToolCompletion::<App>::test_new();
    let consumer = completion.clone();
    let spec = ToolOperationSpec::new("zip-replay", "set-node", Editor::NATIVE_PAYLOAD_SCHEMA, payload(completion, [5; 32]), operation);
    let (admission, original_input) = input(&wire);
    let mut original = bus.dispatch_wire_retained_with_spec(&admission, original_input, None, spec).unwrap_or_else(|_| panic!("initial ZIP dispatch"));
    let mut sequence = 0;
    let mut saved = None;
    for _ in 0..64 {
        let mut cx = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        match original.job.step(&mut cx) {
            StepOutcome::PreviewReady(mut value) => retire(&mut value),
            StepOutcome::CheckpointReady(mut value) => {
                let bytes = value.state.single_page().unwrap();
                if bytes.get(5) == Some(&1) && bytes.get(56) == Some(&1) {
                    saved = Some(bytes.to_vec());
                }
                retire(&mut value.state);
                if saved.is_some() {
                    break;
                }
            }
            StepOutcome::Yield => {}
            _ => panic!("ZIP resolver must yield a checkpoint before completion"),
        }
    }
    let saved = saved.expect("checkpoint after exactly one member");
    original.job.begin_close();
    for _ in 0..64 {
        if original.job.terminal_is_empty() {
            break;
        }
        let _ = original.job.close_step(1, usize::MAX);
    }
    assert!(original.job.terminal_is_empty());
    assert!(consumer.test_take_emit().unwrap().is_none());
    for (revision, should_complete) in [([5; 32], true), ([6; 32], false)] {
        let completion = ArtifactToolCompletion::<App>::test_new();
        let consumer = completion.clone();
        let spec = ToolOperationSpec::new("zip-replay", "set-node", Editor::NATIVE_PAYLOAD_SCHEMA, payload(completion, revision), operation);
        let (admission, wire) = input(&wire);
        let (_, checkpoint) = input(&saved);
        let mut resumed = bus.dispatch_wire_retained_with_spec(&admission, wire, Some(checkpoint), spec).unwrap_or_else(|_| panic!("ZIP factory declares checkpoint resume"));
        let mut completed = false;
        let mut faulted = false;
        for _ in 0..96 {
            let mut cx = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
            match resumed.job.step(&mut cx) {
                StepOutcome::PreviewReady(mut value) => retire(&mut value),
                StepOutcome::CheckpointReady(mut value) => {
                    let bytes = value.state.single_page().unwrap();
                    if bytes.get(5) == Some(&1) {
                        assert_ne!(bytes.get(56), Some(&1), "resumed work must not rescan the first member");
                    }
                    retire(&mut value.state);
                }
                StepOutcome::Complete(mut value) => {
                    retire(&mut value.state);
                    retire(&mut value.output);
                    completed = true;
                    break;
                }
                StepOutcome::Fault(mut value) => {
                    retire(&mut value.detail);
                    faulted = true;
                    break;
                }
                StepOutcome::Yield => {}
                StepOutcome::Cancelled => panic!("uncancelled ZIP resume"),
            }
        }
        assert_eq!(completed, should_complete);
        assert_eq!(faulted, !should_complete);
        if should_complete {
            let (emit, _) = consumer.test_take_emit().unwrap().unwrap();
            let expected = edit_node(&snapshot, &entry_node_id(row["rename"]["name"].as_str().unwrap()), row["rename"]["value"].as_str().unwrap(), &text_revision(row["rename"]["name"].as_str().unwrap())).unwrap();
            assert_eq!(emit.unwrap().artifact_mutations, expected.artifact_mutations);
        }
        resumed.job.begin_close();
        for _ in 0..64 {
            if resumed.job.terminal_is_empty() {
                break;
            }
            let _ = resumed.job.close_step(1, usize::MAX);
        }
        assert!(resumed.job.terminal_is_empty());
    }
    assert!(crate::editor::zip::iso21320::ZipIso21320Editor::NATIVE_CHECKPOINT_RESUME);
    println!("[DEBUG] Registered ZIP factory resumes after one scanned member, preserves exact output, refuses another canonical revision, and closes retained owners");
}
