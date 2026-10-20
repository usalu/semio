//! ⌨️ Drawing editor unit laws over commands: every command round trip, wire keyword, retained route dispositions and example resolution.

use super::*;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::ArtifactEditor;

//#region 🔖️WireGuards
/// 🔖️ One `DrawingCommand` value per row, in binary-variant-ordinal order — feeds both the
/// op-text/binary equivalence loop and the "printed line starts with the row's wire keyword"
/// assertion. Permanent wire guard: appending a variant is safe, reordering breaks the format.
fn every_command() -> Vec<DrawingCommand> {
    vec![
        DrawingCommand::SetSnapshot(set_snapshot::SetSnapshot { snapshot: default_drawing_document("cmd-doc", None) }),
        DrawingCommand::CommitDocument(commit_document::CommitDocument { snapshot: default_drawing_document("cmd-doc-2", None) }),
        DrawingCommand::LoadDocumentJson(load_document_json::LoadDocumentJson { json: "{}".into() }),
        DrawingCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "demo".into() }),
        DrawingCommand::SetSelectedOpacity(set_selected_opacity::SetSelectedOpacity { value: 0.5 }),
        DrawingCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: Some("Renamed \"layer\"".into()) }),
        DrawingCommand::AddLayer(add_layer::AddLayer { kind: "shape:rect".into() }),
        DrawingCommand::DropLayerKind(drop_layer_kind::DropLayerKind { kind: "path".into(), target_row_id: "drawing-play-layers".into(), drop_position: "inside".into() }),
        DrawingCommand::MoveLayer(move_layer::MoveLayer { layer_id: "layer-1".into(), target_row_id: "drawing-play-layers".into(), drop_position: "after".into() }),
        DrawingCommand::DeleteLayer(delete_layer::DeleteLayer { layer_id: "layer-1".into() }),
        DrawingCommand::DuplicateLayer(duplicate_layer::DuplicateLayer { layer_id: "layer-1".into() }),
        DrawingCommand::ToggleLayerVisible(toggle_layer_visible::ToggleLayerVisible { layer_id: "layer-1".into() }),
        DrawingCommand::CombineBoolean(combine_boolean::CombineBoolean { operation: "union".into(), ids: vec!["a".into(), "b".into()] }),
        DrawingCommand::PatchLayer(patch_layer::PatchLayer { layer_id: "layer-1".into(), field: "opacity".into(), value: "0.4".into() }),
        DrawingCommand::PatchLayers(patch_layers::PatchLayers { index:None, layer_ids: vec!["a".into(), "b".into()], field: "blendMode".into(), value: "\"multiply\"".into() }),
        DrawingCommand::SetCamera(set_camera::SetCamera { camera: store::Viewport2d { x: 1.0, y: 2.0, zoom: 1.5 } }),
        DrawingCommand::SetCameraZoom(set_camera_zoom::SetCameraZoom { value: 2.0 }),
        DrawingCommand::EngagementInput(engagement_input::EngagementInput { value: "typing".into() }),
        DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {
            x: 1.0,
            y: 2.0,
            width: 800.0,
            height: 600.0,
            shift: true,
            ctrl: false,
            meta: false,
            generation: None,
            checkpoint_completed_work: None,
            checkpoint_pending_work: None,
            ..Default::default()
        }),
        DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { shift: false, alt: false, x: 1.0, y: 2.0, width: 800.0, height: 600.0, samples: vec![[0.5, 1.5], [1.0, 2.0]] }),
        DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { alt: false, x: 1.0, y: 2.0, width: 800.0, height: 600.0, shift: false, ctrl: true, meta: false, cancelled: false }),
        DrawingCommand::CanvasDoubleClick(canvas_double_click::CanvasDoubleClick {}),
        DrawingCommand::CanvasCommitDraft(canvas_commit_draft::CanvasCommitDraft {}),
        DrawingCommand::CanvasEscape(canvas_escape::CanvasEscape {}),
        DrawingCommand::ExportDocument(export_document::ExportDocument { format: "pdf".into(), ..Default::default() }),
        DrawingCommand::EditSelection(edit_selection::EditSelection { operation: "group".into(), ids: vec!["a".into(), "b".into()] }),
        DrawingCommand::EditPath(edit_path::EditPath { layer_id: "path".into(), edit: Box::new(semio_s_artifact_draw_drawing_geometry::editing::PathEdit::Reverse) }),
        DrawingCommand::EditFill(edit_fill::EditFill { layer_id: "a".into(), edit: Box::new(semio_s_artifact_draw_drawing_model::fill::FillEdit::Type { value: semio_s_artifact_draw_drawing_model::fill::FillType::LinearGradient }) }),
        DrawingCommand::DeleteSelection(delete_selection::DeleteSelection {}),
        DrawingCommand::NudgeSelectionLeft(nudge_selection_left::NudgeSelectionLeft {}),
        DrawingCommand::NudgeSelectionLeftFast(nudge_selection_left_fast::NudgeSelectionLeftFast {}),
        DrawingCommand::NudgeSelectionRight(nudge_selection_right::NudgeSelectionRight {}),
        DrawingCommand::NudgeSelectionRightFast(nudge_selection_right_fast::NudgeSelectionRightFast {}),
        DrawingCommand::NudgeSelectionUp(nudge_selection_up::NudgeSelectionUp {}),
        DrawingCommand::NudgeSelectionUpFast(nudge_selection_up_fast::NudgeSelectionUpFast {}),
        DrawingCommand::NudgeSelectionDown(nudge_selection_down::NudgeSelectionDown {}),
        DrawingCommand::NudgeSelectionDownFast(nudge_selection_down_fast::NudgeSelectionDownFast {}),
        DrawingCommand::ImportImage(import_image::ImportImage {payload:"data:image/png;base64,AAAA".into(),name:Some("Image.png".into()),parent_id:None,index:None}),
    ]
}

#[semio_framework_async_macros::async_test]
async fn drawing_command_op_text_round_trips_every_variant() {
    for command in every_command() {
        store::os_store::test_support::assert_op_line_round_trip(&command);
    }
    store::os_store::test_support::assert_op_line_round_trip(&DrawingCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: None }));
}

#[semio_framework_async_macros::async_test]
async fn drawing_command_op_binary_round_trips_every_variant() {
    for command in every_command() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// 🔖️ Pins the exact pre-migration hex for the two rows whose `Option` fields make `None`/`Some`
/// distinct wire cases — copied verbatim from the `wire-baseline-before.txt` capture taken from
/// the OLD `drawing_protocol` crate before this migration. A byte-for-byte diff, not just a
/// round-trip law, since round-trip alone would happily pass on a changed-but-consistent format.
#[semio_framework_async_macros::async_test]
async fn optional_field_rows_keep_their_pre_migration_bytes() {
    use protocol::OpBinary;
    let engagement_submit_some = DrawingCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: Some("Renamed \"layer\"".into()) });
    assert_eq!(engagement_submit_some.encode_op().expect("encode"), hex_bytes("0105010f52656e616d656420226c617965722201000600"));
    let engagement_submit_none = DrawingCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: None });
    assert_eq!(engagement_submit_none.encode_op().expect("encode"), hex_bytes("01050000"));
}

fn hex_bytes(hex: &str) -> Vec<u8> {
    (0..hex.len()).step_by(2).map(|i| u8::from_str_radix(&hex[i..i + 2], 16).expect("valid hex")).collect()
}

#[semio_framework_async_macros::async_test]
async fn every_command_row_prints_starting_with_its_wire_keyword() {
    use protocol::OpText;
    let expected_keywords = [
        "set-snapshot",
        "commit-document",
        "document-json",
        "active-example",
        "selected-opacity",
        "engagement-submit",
        "add-layer",
        "drop-layer-kind",
        "move-layer",
        "delete-layer",
        "duplicate-layer",
        "toggle-layer-visible",
        "combine-boolean",
        "patch-layer",
        "patch-layers",
        "camera",
        "camera-zoom",
        "engagement-input",
        "canvas-pointer-down",
        "canvas-pointer-move",
        "canvas-pointer-up",
        "canvas-double-click",
        "canvas-commit-draft",
        "canvas-escape",
        "export-document",
        "edit-selection",
        "edit-path",
        "edit-fill",
    ];
    for (command, keyword) in every_command().into_iter().zip(expected_keywords) {
        let printed = command.print_op();
        assert!(printed.starts_with(keyword), "expected '{printed}' to start with '{keyword}'");
    }
}
//#endregion 🔖️WireGuards

//#region 🔖️CommandSurface
/// ⚖️ The dispatch law: the two owned factories partition the command enum exactly, every route
/// carries a publication contract, and every proof row has an owner. A regression in any of the
/// four lists below is exactly the class of defect that made twenty of draw's twenty-six commands
/// unreachable from the client.
#[semio_framework_async_macros::async_test]
async fn retained_route_dispositions_are_exact_and_exhaustive() {
    use semio_framework_plugin::ArtifactOwnedToolJobFactory as _;

    assert_eq!(DRAWING_GESTURE_TOOL_IDS.len(), 6);
    assert_eq!(DRAWING_BOUNDED_TOOL_IDS.len(), 30);
    assert_eq!(DRAWING_PATH_TOOL_IDS.len(), 1);
    assert_eq!(DRAWING_IMAGE_TOOL_IDS.len(), 1);
    let mut routes = DRAWING_GESTURE_TOOL_IDS.iter().chain(DRAWING_BOUNDED_TOOL_IDS).chain(DRAWING_PATH_TOOL_IDS).chain(DRAWING_IMAGE_TOOL_IDS).copied().collect::<Vec<_>>();
    routes.sort_unstable();
    let mut declared = every_command().into_iter().map(|command| command.command_id()).collect::<Vec<_>>();
    declared.sort_unstable();
    assert_eq!(routes, declared, "every declared command owns exactly one retained route, and no route names a command that does not exist");
    assert_eq!(routes.windows(2).filter(|pair| pair[0] == pair[1]).count(), 0, "a tool id may be owned by exactly one factory");

    assert_eq!(DrawingGestureOperationJobFactory::PUBLICATION_CONTRACTS.len(), DRAWING_GESTURE_TOOL_IDS.len());
    assert_eq!(DrawingBoundedCommandJobFactory::PUBLICATION_CONTRACTS.len(), DRAWING_BOUNDED_TOOL_IDS.len());
    for contract in DrawingGestureOperationJobFactory::PUBLICATION_CONTRACTS.iter().chain(DrawingBoundedCommandJobFactory::PUBLICATION_CONTRACTS).chain(DrawingPathCommandJobFactory::PUBLICATION_CONTRACTS).chain(DrawingImageCommandJobFactory::PUBLICATION_CONTRACTS) {
        assert!(routes.contains(&contract.tool_id), "{} publishes without owning a route", contract.tool_id);
        assert!(!contract.lanes.is_empty(), "{} declares no publication lane", contract.tool_id);
        assert!(!contract.lanes.contains(&semio_framework_plugin::ArtifactToolPublicationLane::HostOnly) || contract.lanes.len() == 1, "{} pairs HostOnly with another lane, which the registry rejects", contract.tool_id);
    }

    assert_eq!(<DrawingPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), routes.len());
    assert!(<DrawingPlayApp as ArtifactEditor>::build_artifact_store_one_item_preparation_factory().is_some(), "the artifact lane needs its one-item preparation authority");
    assert!(<DrawingPlayApp as ArtifactEditor>::build_config_store_one_item_preparation_factory().is_none(), "NoConfig owns no document-config publication lane");
    let mut window_config_owners = semio_framework_plugin::WindowConfigOwnerRegistry::new(protocol::ActorId("drawing-contract-owner".into()));
    <DrawingPlayApp as ArtifactEditor>::register_window_config_owners(&mut window_config_owners).expect("register canvas window config owner");
    assert!(!window_config_owners.is_empty(), "the canvas window config owns its own retained publication authority");

    let definition = create_drawing_app();
    let classified = definition
        .actions
        .iter()
        .map(|action| (action.id.as_str(), action.semantics.execution.interactive_job))
        .chain(
            definition
        .window_kinds
        .iter()
        .flat_map(|window| window.actions.iter())
        .map(|action| (action.id.as_str(), action.semantics.execution.interactive_job)),
        )
        .chain(definition.commands.iter().map(|command| (command.id.as_str(), command.semantics.execution.interactive_job)))
        .collect::<Vec<_>>();
    for route in &routes {
        assert!(classified.iter().any(|(id, classification)| id == route && *classification == semio_framework_plugin::InteractiveJobClassification::Migrated), "{route} owns a retained route but the manifest does not classify it Migrated");
    }
}

/// 🖼️ `setActiveExample` resolves against the registered catalogue rather than a hardcoded id, so
/// the switcher's ids load a document and an unregistered id faults instead of silently doing
/// nothing.
#[semio_framework_async_macros::async_test]
async fn set_active_example_resolves_the_registered_catalogue() {
    let examples = semio_s_artifact_draw_drawing_examples::examples();
    assert!(examples.iter().any(|source| source.id() == "demo"), "the subset registers its demo example");
    let doc = default_drawing_document("example-probe", None);
    let history = semio_framework_plugin::HistoryView::empty();
    let config = NoConfig::default();
    let view = ArtifactView::with_operation(&doc, &history,semio_s_artifact_draw_drawing_editor::identity_test::operation());
    let cfg = ConfigView { snapshot: &config, window: None };
    let mut session = DrawingSession::default();
    for source in examples {
        let emit = set_active_example::handle(&set_active_example::SetActiveExample { example_id: source.id().into() }, &view, &cfg, &mut session).expect("a registered example id loads its document");
        assert_eq!(emit.effects.len(), 1, "{} must load exactly one document", source.id());
    }
    let reset = set_active_example::handle(&set_active_example::SetActiveExample { example_id: String::new() }, &view, &cfg, &mut session).expect("the empty id resets to a blank drawing");
    assert_eq!(reset.effects.len(), 1);
    assert!(set_active_example::handle(&set_active_example::SetActiveExample { example_id: "semio".into() }, &view, &cfg, &mut session).is_err(), "an unregistered id faults instead of silently doing nothing");
}
//#endregion 🔖️CommandSurface
