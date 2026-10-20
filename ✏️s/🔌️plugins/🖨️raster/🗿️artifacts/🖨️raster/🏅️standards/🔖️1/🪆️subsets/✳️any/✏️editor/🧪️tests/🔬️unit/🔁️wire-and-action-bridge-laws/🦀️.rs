/// 🔁️ Command wire, retained route and host action bridge laws.

/// 🧾️ One representative value per row, in declaration (= binary ordinal) order — TEMPLATE.md §7's
/// permanent wire guard, feeding the round-trip/keyword-uniqueness/leading-token laws below.
fn every_command() -> Vec<RasterCommand> {
    vec![
        RasterCommand::AddLayer(semio_s_artifact_raster_raster_editor::commands::add_layer::AddLayer { kind: "pixel".into() }),
        RasterCommand::DropLayerKind(semio_s_artifact_raster_raster_editor::commands::drop_layer_kind::DropLayerKind { kind: "group".into() }),
        RasterCommand::SetLayerVisible(semio_s_artifact_raster_raster_editor::commands::set_layer_visible::SetLayerVisible { layer_id: "l1".into(), visible: Some(true) }),
        RasterCommand::ToggleLayerVisible(semio_s_artifact_raster_raster_editor::commands::toggle_layer_visible::ToggleLayerVisible { layer_id: "l1".into() }),
        RasterCommand::DeleteLayer(semio_s_artifact_raster_raster_editor::commands::delete_layer::DeleteLayer { layer_id: "l1".into() }),
        RasterCommand::DuplicateLayer(semio_s_artifact_raster_raster_editor::commands::duplicate_layer::DuplicateLayer { layer_id: "l1".into() }),
        RasterCommand::PatchLayer(semio_s_artifact_raster_raster_editor::commands::patch_layer::PatchLayer { layer_id: "l1".into(), field: "opacity".into(), value: "0.4".into() }),
        RasterCommand::PatchLayers(semio_s_artifact_raster_raster_editor::commands::patch_layers::PatchLayers { layer_ids: vec!["a".into(), "b".into()], field: "name".into(), value: "Renamed".into() }),
        RasterCommand::MoveLayer(semio_s_artifact_raster_raster_editor::commands::move_layer::MoveLayer { layer_id: "l1".into(), target_row_id: "raster-play-layers".into(), drop_position: "after".into() }),
        RasterCommand::SetBrushSize(semio_s_artifact_raster_raster_editor::commands::set_brush_size::SetBrushSize { value: 40.0 }),
        RasterCommand::SetBrushOpacity(semio_s_artifact_raster_raster_editor::commands::set_brush_opacity::SetBrushOpacity { value: 0.5 }),
        RasterCommand::SetBrushColor(semio_s_artifact_raster_raster_editor::commands::set_brush_color::SetBrushColor { value: "#e07020".into() }),
        RasterCommand::SetBrushHardness(semio_s_artifact_raster_raster_editor::commands::set_brush_hardness::SetBrushHardness { value: 0.25 }),
        RasterCommand::SetPaintTarget(semio_s_artifact_raster_raster_editor::commands::set_paint_target::SetPaintTarget{value:"mask".into()}),
        RasterCommand::SetMaskValue(semio_s_artifact_raster_raster_editor::commands::set_mask_value::SetMaskValue{value:96}),
        RasterCommand::SetPixelSelection(semio_s_artifact_raster_raster_editor::commands::set_pixel_selection::SetPixelSelection{selection:Some(semio_s_artifact_raster_raster_editor::selection::RasterPixelSelection{layer_id:"l1".into(),target:"pixels".into(),width:2,height:2,spans:vec![]}),expected_image_key:None}),
        RasterCommand::SetCompositeViewport(semio_s_artifact_raster_raster_editor::commands::set_composite_viewport::SetCompositeViewport { width: 640.0, height: 480.0 }),
        RasterCommand::SetCamera(semio_s_artifact_raster_raster_editor::commands::camera::set_camera::SetCamera { camera: semio_s_artifact_raster_raster_model::RasterCamera { x: 1.0, y: 2.0, zoom: 1.5 } }),
        RasterCommand::SetCameraZoom(semio_s_artifact_raster_raster_editor::commands::camera::set_camera_zoom::SetCameraZoom { zoom: 2.0 }),
        RasterCommand::SetActiveExample(semio_s_artifact_raster_raster_editor::commands::set_active_example::SetActiveExample { example_id: semio_s_artifact_raster_raster_examples::art_raster_demo::ID.into() }),
        RasterCommand::ExportPng(semio_s_artifact_raster_raster_editor::commands::export_png::ExportPng {}),
        RasterCommand::FlattenLayers(semio_s_artifact_raster_raster_editor::commands::flatten_layers::FlattenLayers {name:"Flattened Image".into()}),
        RasterCommand::MergeDown(semio_s_artifact_raster_raster_editor::commands::merge_down::MergeDown {layer_id:"l1".into()}),
        RasterCommand::MaskFromSelection(semio_s_artifact_raster_raster_editor::commands::mask_from_selection::MaskFromSelection {layer_id:"l1".into(),expected_image_key:None,selection:vec![semio_s_artifact_raster_raster_mutations_pixels::paint_stroke::RasterSelectionSpan { start: 0, length: 1, coverage: 255 }]}),
        RasterCommand::PaintStroke(semio_s_artifact_raster_raster_editor::commands::paint_stroke::PaintStroke { layer_id: "l1".into(), tool: "eraser".into(), xs: vec![0.5, 3.25], ys: vec![1.5, 2.0], phase: None, reason: None, gesture: None }),
        RasterCommand::FillRegion(semio_s_artifact_raster_raster_editor::commands::fill_region::FillRegion { layer_id: "l1".into(), x: 1.5, y: 0.25 }),
        RasterCommand::SetFillTolerance(semio_s_artifact_raster_raster_editor::commands::set_fill_tolerance::SetFillTolerance { value: 40 }),
        RasterCommand::ApplyFilter(semio_s_artifact_raster_raster_editor::commands::apply_filter::ApplyFilter { layer_id: "l1".into(), filter: "posterize".into(), amount: 4.0 }),
        RasterCommand::TransformImage(semio_s_artifact_raster_raster_editor::commands::transform_image::TransformImage { layer_id: "l1".into(), operation: "crop".into(), x: 1, y: 2, width: 3, height: 4, bilinear: false }),
        RasterCommand::FillSelection(semio_s_artifact_raster_raster_editor::commands::fill_selection::FillSelection { layer_id: "l1".into() }),
    ]
}

/// ⚖️ Every command has one exact factory proof and publication lane; PNG export is resumable and host-only.
#[semio_framework_async_macros::async_test]
async fn retained_route_dispositions_are_exact_and_exhaustive() {
    use semio_framework::{ToolCancellationPolicy, ToolExecutionShape, ToolJobFactory};
    use std::collections::BTreeSet;
    assert_eq!(RASTER_RETAINED_TOOL_IDS.len(), 29);
    assert_eq!(<RasterPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), 30);
    assert_eq!(RasterRetainedCommandJobFactory::PUBLICATION_CONTRACTS.len(), 29);
    assert_eq!(raster_retained_contract().shape, ToolExecutionShape::BoundedFirstStep);
    assert_eq!(raster_retained_contract().cancellation, ToolCancellationPolicy::PerOperation);

    let retained: BTreeSet<&str> = RASTER_RETAINED_TOOL_IDS.iter().copied().chain(["exportPng"]).collect();
    assert_eq!(semio_s_artifact_raster_raster_editor::media_export::RasterDownloadJobFactory::new(RASTER_PLAY_CONTROLLER_ID).execution_contract().shape,ToolExecutionShape::Resumable);
    assert_eq!(semio_s_artifact_raster_raster_editor::media_export::RasterDownloadJobFactory::PUBLICATION_CONTRACTS[0].lanes,&[ArtifactToolPublicationLane::HostOnly]);
    assert_eq!(retained, every_command().iter().map(RasterCommand::command_id).collect::<BTreeSet<_>>(), "every RasterCommand row must be a retained route");
    assert_eq!(retained, RasterCommand::TOOL_JOB_IDS.iter().copied().collect::<BTreeSet<_>>(), "the retained table must equal the generated tool-job id set");

    let artifact_lane: BTreeSet<&str> = ["addLayer", "dropLayerKind", "setLayerVisible", "toggleLayerVisible", "deleteLayer", "duplicateLayer", "patchLayer", "patchLayers", "moveLayer", "setActiveExample", "paintStroke", "fillRegion", "applyFilter", "transformImage", "fillSelection", "maskFromSelection", "flattenLayers", "mergeDown"].into_iter().collect();
    for tool_id in RASTER_RETAINED_TOOL_IDS {
        let contract = RasterRetainedCommandJobFactory::PUBLICATION_CONTRACTS.iter().find(|contract| contract.tool_id == *tool_id).unwrap_or_else(|| panic!("publication contract for {tool_id}"));
        let expected: &[ArtifactToolPublicationLane] = match *tool_id {
            "paintStroke" => &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowTransient],
            id if artifact_lane.contains(id) => &[ArtifactToolPublicationLane::Artifact],
            _ => &[ArtifactToolPublicationLane::Config],
        };
        assert_eq!(contract.lanes, expected, "{tool_id} publishes into exactly its lanes");
    }
    assert!(<RasterPlayApp as ArtifactEditor>::build_artifact_store_one_item_preparation_factory().is_some(), "the Artifact lane is rejected outright without a document one-item preparation factory");
    assert!(<RasterPlayApp as ArtifactEditor>::build_config_store_one_item_preparation_factory().is_some(), "the Config lane is rejected outright without a config one-item preparation factory");

    let definition = create_raster_app();
    for tool_id in RASTER_RETAINED_TOOL_IDS {
        let action = definition
            .actions
            .iter()
            .chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter()))
            .find(|action| action.id == *tool_id)
            .unwrap_or_else(|| panic!("action {tool_id} declared"));
        assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "{tool_id} must be UI-dispatchable");
    }
    for action in definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter())) {
        assert_ne!(action.semantics.execution.interactive_job, InteractiveJobClassification::Unclassified, "action {} is unclassified", action.id);
    }
    for command in definition.commands.iter() {
        assert_ne!(command.semantics.execution.interactive_job, InteractiveJobClassification::Unclassified, "command {} is unclassified", command.id);
    }

    let migrated_ids: BTreeSet<&str> = definition
        .actions
        .iter()
        .chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter()))
        .filter(|action| action.semantics.execution.interactive_job == InteractiveJobClassification::Migrated)
        .map(|action| action.id.as_str())
        .collect();
    assert_eq!(RasterCommand::TOOL_JOB_IDS.iter().copied().filter(|tool_id| migrated_ids.contains(tool_id)).collect::<BTreeSet<_>>(), retained, "every registered proof must be Migrated, and every Migrated tool-job row must be proven");
}

/// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_through_text_and_binary() {
    for command in every_command() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// 🎫️ Every `app_commands!` row's wire keyword must be distinct — the cross-cutting invariant the
/// macro exists to hold.
#[semio_framework_async_macros::async_test]
async fn command_wire_keywords_are_unique_across_every_row() {
    let commands = every_command();
    assert_eq!(commands.len(), 30, "every RasterCommand row must be covered by every_command()");
    let mut keywords: Vec<String> = commands.iter().map(|command| protocol::OpText::print_op(command).split(' ').next().unwrap_or_default().to_string()).collect();
    keywords.sort();
    keywords.dedup();
    assert_eq!(keywords.len(), commands.len(), "every row's wire keyword must be distinct");
}

/// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — what a
/// missing `#[dsl(keyword = ..)]` on a payload struct silently breaks (the record prints with no
/// keyword at all and no longer parses).
#[semio_framework_async_macros::async_test]
async fn every_printed_op_line_starts_with_the_rows_declared_wire_keyword() {
    let expectations: Vec<(&str, RasterCommand)> = every_command()
        .into_iter()
        .map(|command| {
            let keyword: &'static str = match &command {
                RasterCommand::ExportPng(_) => "export-png",
                RasterCommand::PaintStroke(_) => "paint-stroke",
                RasterCommand::FillRegion(_) => "fill-region",
                RasterCommand::ApplyFilter(_) => "apply-filter",
                RasterCommand::TransformImage(_) => "transform-image",
                RasterCommand::FillSelection(_) => "fill-selection",
                RasterCommand::AddLayer(_) => "add-layer",
                RasterCommand::DropLayerKind(_) => "drop-layer-kind",
                RasterCommand::SetLayerVisible(_) => "set-layer-visible",
                RasterCommand::ToggleLayerVisible(_) => "toggle-layer-visible",
                RasterCommand::DeleteLayer(_) => "delete-layer",
                RasterCommand::DuplicateLayer(_) => "duplicate-layer",
                RasterCommand::PatchLayer(_) => "patch-layer",
                RasterCommand::PatchLayers(_) => "patch-layers",
                RasterCommand::MoveLayer(_) => "move-layer",
                RasterCommand::SetBrushSize(_) => "brush-size",
                RasterCommand::SetBrushOpacity(_) => "brush-opacity",
                RasterCommand::SetBrushColor(_) => "brush-color",
                RasterCommand::SetBrushHardness(_) => "brush-hardness",
                RasterCommand::SetPaintTarget(_)=>"paint-target",
                RasterCommand::SetMaskValue(_)=>"mask-value",
                RasterCommand::SetFillTolerance(_) => "fill-tolerance",
                RasterCommand::SetPixelSelection(_)=>"pixel-selection",
                RasterCommand::SetCompositeViewport(_) => "composite-viewport",
                RasterCommand::SetCamera(_) => "camera",
                RasterCommand::SetCameraZoom(_) => "camera-zoom",
                RasterCommand::SetActiveExample(_) => "set-active-example",
                RasterCommand::FlattenLayers(_) => "flatten-layers",
                RasterCommand::MergeDown(_) => "merge-down",
                RasterCommand::MaskFromSelection(_) => "mask-from-selection",
            };
            (keyword, command)
        })
        .collect();
    for (expected_keyword, command) in expectations {
        let printed = protocol::OpText::print_op(&command);
        assert_eq!(printed.split(' ').next().unwrap_or_default(), expected_keyword, "wire keyword drifted for {command:?}: {printed:?}");
    }
}

/// ⚖️ The rows whose `Option` fields make `None`/`Some` distinct wire cases, pinned to exact
/// bytes so an ACCIDENTAL row reorder is caught. Baseline rebased once, deliberately, by the
/// `26/08/12/SEMANTIC-MUTATIONS-OVERHAUL` ticket: dropping the two leading `setSnapshot`/
/// `setActiveExample` rows (whole-document replace is no longer expressible as a mutation)
/// shifted every later row's binary ordinal down by two — `set-layer-visible` 4→2 (`0104`→`0102`).
/// `setActiveExample` returned in `26/09/05/RASTER-PLUGIN-END-TO-END` as an ordered mutation batch
/// rather than a snapshot swap, APPENDED as the last row, so no earlier ordinal moved.
/// Rebased again by `26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM`: the `set-selection`/
/// `set-hover`/`select-all` rows (the only other `Option`-carrying case, `set-hover`) are deleted
/// outright — layer selection/hover is the framework-owned `"layers"` interaction domain now.
/// `set-layer-visible`'s ordinal is unaffected (it sits before the deleted rows). Greenfield repo,
/// no persisted wire data to migrate. Any FURTHER drift here is a real format break, not a
/// fixture mismatch.
#[semio_framework_async_macros::async_test]
async fn optional_field_rows_keep_their_declared_wire_bytes() {
    let cases: [(RasterCommand, &str, &str); 1] = [(RasterCommand::SetLayerVisible(semio_s_artifact_raster_raster_editor::commands::set_layer_visible::SetLayerVisible { layer_id: "l1".into(), visible: None }), "set-layer-visible set-layer-visible layer-id=l1", "010201026c3101000600")];
    for (command, text, hex) in cases {
        assert_eq!(protocol::OpText::print_op(&command), text, "printed text drifted for {command:?}");
        let bytes = protocol::OpBinary::encode_op(&command).expect("encode");
        assert_eq!(bytes.iter().map(|b| format!("{b:02x}")).collect::<String>(), hex, "binary bytes drifted for {command:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn command_ids_are_unique_across_every_row() {
    let mut seen = std::collections::HashSet::new();
    for command in every_command() {
        assert!(seen.insert(command.command_id().to_string()), "duplicate command_id {}", command.command_id());
    }
}

//#region 🔖️ActionBridge
/// 🌉️ Every command row the shells reach by action id must decode through `command_from_action`
/// (camelCase host keys → the payloads' own snake_case `FromValue` names) and its `command_id` must
/// round-trip — the boundary that was missing before the 2026-09-16 boot wave of ticket
/// 26/09/05/RASTER-PLUGIN-END-TO-END (every shell action was refused as "not framework-reserved").
#[test]
fn command_from_action_round_trips_every_command_id() {
    for command in every_command() {
        let id = command.command_id();
        let args = semio_framework_value::ToValue::to_value(&command);
        let payload = match &args {
            semio_framework_value::DslValue::Object(entries) if entries.len() == 1 => entries[0].1.clone(),
            other => other.clone(),
        };
        let camel = camel_case_keys(&payload);
        let bridged = <RasterPlayApp as semio_framework_plugin::ArtifactEditor>::command_from_action(id, Some((camel).clone()), None).unwrap_or_else(|error| panic!("action {id} failed to bridge: {}", error.message));
        assert_eq!(bridged.command_id(), id, "command_id mismatch for action {id}");
        assert_eq!(bridged, command, "payload mismatch for action {id}");
    }
    assert!(<RasterPlayApp as semio_framework_plugin::ArtifactEditor>::command_from_action("nonsense", None, None).is_err());
}

/// 🐫️ The shell's spelling of the payload keys.
fn camel_case_keys(value: &semio_framework_value::DslValue) -> semio_framework_value::DslValue {
    match value {
        semio_framework_value::DslValue::Object(entries) => semio_framework_value::DslValue::Object(
            entries
                .iter()
                .map(|(key, value)| {
                    let mut camel = String::new();
                    let mut upper = false;
                    for ch in key.chars() {
                        if ch == '_' { upper = true; } else if upper { camel.push(ch.to_ascii_uppercase()); upper = false; } else { camel.push(ch); }
                    }
                    (camel, value.clone())
                })
                .collect(),
        ),
        other => other.clone(),
    }
}

/// 🧱️ The tree/host verbs' own spellings (bare `id`, `exampleId`, the Paint2dHost's `{camera:{…}}`)
/// reach the same rows.
#[test]
fn command_from_action_bridges_host_spellings() {
    let args = |json: &str| semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("fixture JSON"));
    let bridge = |action: &str, json: &str| <RasterPlayApp as semio_framework_plugin::ArtifactEditor>::command_from_action(action, Some((args(json)).clone()), None).expect(action);
    assert_eq!(bridge("setActiveExample", r#"{"exampleId":"demo"}"#), RasterCommand::SetActiveExample(semio_s_artifact_raster_raster_editor::commands::set_active_example::SetActiveExample { example_id: "demo".into() }));
    assert_eq!(bridge("deleteLayer", r#"{"id":"layer-1"}"#), RasterCommand::DeleteLayer(semio_s_artifact_raster_raster_editor::commands::delete_layer::DeleteLayer { layer_id: "layer-1".into() }));
    assert_eq!(bridge("setCamera", r#"{"camera":{"x":1,"y":2,"zoom":3}}"#), RasterCommand::SetCamera(semio_s_artifact_raster_raster_editor::commands::camera::set_camera::SetCamera { camera: semio_s_artifact_raster_raster_model::RasterCamera { x: 1.0, y: 2.0, zoom: 3.0 } }));
    assert_eq!(bridge("setCompositeViewport", r#"{"width":640,"height":480}"#), RasterCommand::SetCompositeViewport(semio_s_artifact_raster_raster_editor::commands::set_composite_viewport::SetCompositeViewport { width: 640.0, height: 480.0 }));
    assert_eq!(bridge("addLayer", r#"{"kind":"pixel"}"#), RasterCommand::AddLayer(semio_s_artifact_raster_raster_editor::commands::add_layer::AddLayer { kind: "pixel".into() }));
}
//#endregion 🔖️ActionBridge
