/// 🧵️ Cursorized route laws: the exact retained publication contracts and the static laws that keep the suggestion, transform, focus, inspector, attraction and active-example routes cursorized.

#[test]
fn retained_publication_contracts_are_an_exact_nonempty_tool_bijection() {
    let scene_snapshot: Value = parse(std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🧪️tests/🧫️fixtures/🗄️retained-jobs/🔣️.json")).expect("committed test data reads").as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("Puzzle3D retained route scene_snapshot");
    assert_eq!(scene_snapshot.get("toolIds"), Some(&Value::Array(PUZZLE3D_RETAINED_TOOL_IDS.iter().map(|id| Value::from(*id)).collect())));
    let manifest = create_puzzle3d_app();
    for tool_id in PUZZLE3D_RETAINED_TOOL_IDS {
        let actions = action_declarations(&manifest, tool_id);
        assert_eq!(actions.len(), 1, "{tool_id} requires exactly one manifest declaration");
        assert_eq!(actions[0].semantics.execution.interactive_job, semio_framework_plugin::InteractiveJobClassification::Migrated, "{tool_id}");
    }
    let exact = |contracts: &[ArtifactToolPublicationContract]| {
        let ids = contracts.iter().map(|contract| contract.tool_id).collect::<std::collections::BTreeSet<_>>();
        ids == PUZZLE3D_RETAINED_TOOL_IDS.iter().copied().collect()
            && ids.len() == contracts.len()
            && contracts.iter().all(|contract| !contract.lanes.is_empty() && (!contract.lanes.contains(&ArtifactToolPublicationLane::HostOnly) || contract.lanes.len() == 1))
    };
    let contracts = <Puzzle3dRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS;
    assert!(exact(contracts));
    assert!(!exact(&contracts[..contracts.len() - 1]));
    let mut duplicate = contracts.to_vec();
    let copied = duplicate[1];
    duplicate[0] = copied;
    assert!(!exact(&duplicate));
}

#[test]
fn retained_command_catalog_excludes_framework_owned_shared_actions() {
    assert!(!PUZZLE3D_RETAINED_TOOL_IDS.contains(&SET_ACTIVE_TOOL_ACTION_ID));
    assert!(!PUZZLE3D_RETAINED_TOOL_IDS.contains(&SET_ACTIVE_UTILITY_ACTION_ID));
}

fn suggestion_and_precompute_routes_are_cursorized(source: &str) -> bool {
    [
        r#""acceptSuggestion" => owned!(Puzzle3dAcceptSuggestionWork, |request, scope| Puzzle3dAcceptSuggestionWork::from_original_construction(scope).map(|(work, progress)| (work.bound(Puzzle3dInstanceBinding { app_instance_id: request.app_instance_id, parent_document_id: String::new(), owner: request.instance_operation_owner.clone() }), progress)))"#,
        r#"| "targetBrushSuggestions""#,
        "=> bound!(|request, instance| Puzzle3dPrecomputeCommandWork::new(tool_id).bound(instance))",
        "Puzzle3dAcceptSuggestionStage::Target",
        "Puzzle3dAcceptSuggestionStage::Candidate",
        "Puzzle3dAcceptSuggestionStage::Vortices",
        "Puzzle3dAcceptSuggestionStage::ExistingAttractions",
        "Puzzle3dAcceptSuggestionStage::PublishObject",
        "Puzzle3dAcceptSuggestionStage::PublishAttraction",
        "Puzzle3dPrecomputeCommandStage::Objects",
        "Puzzle3dPrecomputeCommandStage::Vortices",
        "Puzzle3dPrecomputeCommandStage::Attractions",
        "Puzzle3dPrecomputeCommandStage::CatalogObjects",
        "Puzzle3dPrecomputeCommandStage::CatalogVortices",
        "Puzzle3dPrecomputeCommandStage::Positions",
        "Puzzle3dPrecomputeCommandStage::Indices",
        "PUZZLE3D_MESH_PAGE_SCAN_CHARS",
        "Puzzle3dPrecomputeCommandStage::Publish",
    ]
    .into_iter()
    .all(|marker| source.contains(marker))
        && !source.contains(r#""acceptSuggestion" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
        && !source.contains(r#""cycleBrushCandidate" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
        && !source.contains(r#""fillBuildTick" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
        && !source.contains(r#""registerBrushMesh" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
        && !source.contains(r#""setFillCount" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
        && !source.contains(r#""targetBrushSuggestions" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn suggestion_and_precompute_hostile_static_law_rejects_one_grant_reducers_and_missing_boundaries() {
    let source: &str = &editor_source();
    assert!(suggestion_and_precompute_routes_are_cursorized(source));
    let direct_accept = source.replace(
        r#""acceptSuggestion" => owned!(Puzzle3dAcceptSuggestionWork, |request, scope| Puzzle3dAcceptSuggestionWork::from_original_construction(scope).map(|(work, progress)| (work.bound(Puzzle3dInstanceBinding { app_instance_id: request.app_instance_id, parent_document_id: String::new(), owner: request.instance_operation_owner.clone() }), progress)))"#,
        r#""acceptSuggestion" => work!(|request| crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#,
    );
    assert!(!suggestion_and_precompute_routes_are_cursorized(&direct_accept));
    let direct_precompute = source.replace("=> bound!(|request, instance| Puzzle3dPrecomputeCommandWork::new(tool_id).bound(instance))", "=> work!(|request| crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))");
    assert!(!suggestion_and_precompute_routes_are_cursorized(&direct_precompute));
    for marker in [
        "Puzzle3dAcceptSuggestionStage::Target",
        "Puzzle3dAcceptSuggestionStage::Vortices",
        "Puzzle3dAcceptSuggestionStage::ExistingAttractions",
        "Puzzle3dAcceptSuggestionStage::PublishObject",
        "Puzzle3dAcceptSuggestionStage::PublishAttraction",
        "Puzzle3dPrecomputeCommandStage::Objects",
        "Puzzle3dPrecomputeCommandStage::Vortices",
        "Puzzle3dPrecomputeCommandStage::Attractions",
        "Puzzle3dPrecomputeCommandStage::CatalogObjects",
        "Puzzle3dPrecomputeCommandStage::CatalogVortices",
        "Puzzle3dPrecomputeCommandStage::Positions",
        "Puzzle3dPrecomputeCommandStage::Indices",
        "PUZZLE3D_MESH_PAGE_SCAN_CHARS",
        "Puzzle3dPrecomputeCommandStage::Publish",
    ] {
        assert!(!suggestion_and_precompute_routes_are_cursorized(&source.replace(marker, "cursor-removed")), "missing retained boundary was falsely accepted: {marker}");
    }
}

fn selection_transforms_run_the_transform_tool(source: &str) -> bool {
    source.contains(r#""translateSelection" | "rotateSelection" | "scaleSelection" | "worldRelocate" | "relocateTargetVolume" => seed!(|request, seed| Puzzle3dTransformWork::new(tool_id, seed))"#)
        && source.contains("Puzzle3dTransformStage::Read")
        && source.contains("Puzzle3dTransformStage::Scan")
        && source.contains("Puzzle3dRelocateScan::begin(document")
        && source.contains("Puzzle3dTransformStage::Commit")
        && source.contains("puzzle3d_transform_tool_commit(self.tool_id")
        && !source.contains(r#""translateSelection" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
        && !source.contains(r#""worldRelocate" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

/// 🛠️ Every selection transform — the three gumball verbs, the target-volume relocate and the Relocate
/// utility — is ONE staged work that states the gesture and commits it through the transform tool: a
/// one-grant reducer, a missing stage or a bypassed tool fails closed.
#[test]
fn selection_transform_hostile_static_law_rejects_one_grant_reducers_and_bypassed_tools() {
    let source: &str = &editor_source();
    assert!(selection_transforms_run_the_transform_tool(source));
    let direct = source.replace(
        r#""translateSelection" | "rotateSelection" | "scaleSelection" | "worldRelocate" | "relocateTargetVolume" => seed!(|request, seed| Puzzle3dTransformWork::new(tool_id, seed))"#,
        r#""translateSelection" | "rotateSelection" | "scaleSelection" | "worldRelocate" | "relocateTargetVolume" => work!(|request| crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#,
    );
    assert!(!selection_transforms_run_the_transform_tool(&direct), "hostile old-reducer replacement must fail closed");
    for marker in ["Puzzle3dTransformStage::Read", "Puzzle3dTransformStage::Scan", "Puzzle3dRelocateScan::begin(document", "Puzzle3dTransformStage::Commit", "puzzle3d_transform_tool_commit(self.tool_id"] {
        assert!(!selection_transforms_run_the_transform_tool(&source.replace(marker, "stage-removed")), "missing transform stage was falsely accepted: {marker}");
    }
}

fn focus_selection_is_cursorized(source: &str) -> bool {
    source.contains(r#""focusSelection" => work!(|request| Puzzle3dFocusSelectionWork::default())"#)
        && source.contains("Puzzle3dFocusSelectionStage::Selection")
        && source.contains("Puzzle3dFocusSelectionStage::SumObjects")
        && source.contains("Puzzle3dFocusSelectionStage::DistanceObjects")
        && source.contains("Puzzle3dFocusSelectionStage::Publish")
        && !source.contains(r#""focusSelection" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn focus_selection_hostile_static_law_rejects_hidden_whole_collection_work() {
    let source: &str = &editor_source();
    assert!(focus_selection_is_cursorized(source));
    let direct = source
        .replace(r#""focusSelection" => work!(|request| Puzzle3dFocusSelectionWork::default())"#, r#""focusSelection" => work!(|request| crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#);
    assert!(!focus_selection_is_cursorized(&direct));
    for marker in ["Puzzle3dFocusSelectionStage::Selection", "Puzzle3dFocusSelectionStage::SumObjects", "Puzzle3dFocusSelectionStage::DistanceObjects", "Puzzle3dFocusSelectionStage::Publish"] {
        assert!(!focus_selection_is_cursorized(&source.replace(marker, "cursor-removed")), "missing focus cursor was falsely accepted: {marker}");
    }
}

fn patch_inspector_is_cursorized(source: &str) -> bool {
    source.contains(r#""patchInspector" => owned!(Puzzle3dPatchInspectorWork, |request, scope| Puzzle3dPatchInspectorWork::from_original_construction(scope))"#)
        && source.contains("Puzzle3dPatchInspectorStage::Selection")
        && source.contains("Puzzle3dPatchInspectorStage::Objects")
        && source.contains("Puzzle3dPatchInspectorStage::Vortices")
        && source.contains("Puzzle3dPatchInspectorStage::Attractions")
        && source.contains("Puzzle3dPatchInspectorStage::AttractionReconnect")
        && source.contains("Puzzle3dPatchInspectorStage::References")
        && source.contains("Puzzle3dPatchInspectorStage::Volumes")
        && !source.contains(r#""patchInspector" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn patch_inspector_hostile_static_law_rejects_old_reducer_and_hidden_collection_loops() {
    let source: &str = &editor_source();
    assert!(patch_inspector_is_cursorized(source));
    let direct = source
        .replace(r#""patchInspector" => owned!(Puzzle3dPatchInspectorWork, |request, scope| Puzzle3dPatchInspectorWork::from_original_construction(scope))"#, r#""patchInspector" => work!(|request| crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#);
    assert!(!patch_inspector_is_cursorized(&direct));
    for marker in [
        "Puzzle3dPatchInspectorStage::Selection",
        "Puzzle3dPatchInspectorStage::Objects",
        "Puzzle3dPatchInspectorStage::Vortices",
        "Puzzle3dPatchInspectorStage::Attractions",
        "Puzzle3dPatchInspectorStage::AttractionReconnect",
        "Puzzle3dPatchInspectorStage::References",
        "Puzzle3dPatchInspectorStage::Volumes",
    ] {
        assert!(!patch_inspector_is_cursorized(&source.replace(marker, "cursor-removed")), "missing inspector cursor was falsely accepted: {marker}");
    }
}

fn create_attraction_is_cursorized(source: &str) -> bool {
    source.contains(r#""createAttraction" => work!(|request| Puzzle3dCreateAttractionWork::default())"#)
        && source.contains("Puzzle3dCreateAttractionStage::Existing")
        && source.contains("Puzzle3dCreateAttractionStage::Attracting")
        && source.contains("Puzzle3dCreateAttractionStage::Attracted")
        && source.contains("Puzzle3dCreateAttractionStage::Compatibility")
        && source.contains("Puzzle3dCreateAttractionStage::Publish")
        && !source.contains(r#""createAttraction" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn create_attraction_hostile_static_law_rejects_nested_whole_scans() {
    let source: &str = &editor_source();
    assert!(create_attraction_is_cursorized(source));
    let direct = source.replace(
        r#""createAttraction" => work!(|request| Puzzle3dCreateAttractionWork::default())"#,
        r#""createAttraction" => work!(|request| crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#,
    );
    assert!(!create_attraction_is_cursorized(&direct));
    for marker in ["Puzzle3dCreateAttractionStage::Existing", "Puzzle3dCreateAttractionStage::Attracting", "Puzzle3dCreateAttractionStage::Attracted", "Puzzle3dCreateAttractionStage::Compatibility", "Puzzle3dCreateAttractionStage::Publish"] {
        assert!(!create_attraction_is_cursorized(&source.replace(marker, "cursor-removed")), "missing attraction cursor was falsely accepted: {marker}");
    }
}

fn set_active_example_is_cursorized(source: &str) -> bool {
    source.contains(r#""setActiveExample" => owned!(Puzzle3dSetActiveExampleWork, |request, scope| Puzzle3dSetActiveExampleWork::from_original_construction(scope))"#)
        && source.contains("Puzzle3dSetActiveExampleStage::DeleteAttractions")
        && source.contains("Puzzle3dSetActiveExampleStage::DeleteObjects")
        && source.contains("Puzzle3dSetActiveExampleStage::DeleteVolumes")
        && source.contains("Puzzle3dSetActiveExampleStage::DeleteReferences")
        && source.contains("Puzzle3dSetActiveExampleStage::DeleteCompatibility")
        && source.contains("Puzzle3dSetActiveExampleStage::CreateObjects")
        && source.contains("Puzzle3dSetActiveExampleStage::CreateAttractions")
        && source.contains("Puzzle3dSetActiveExampleStage::CreateVolumes")
        && source.contains("Puzzle3dSetActiveExampleStage::CreateReferences")
        && source.contains("Puzzle3dSetActiveExampleStage::CreateCompatibility")
        && source.contains("Puzzle3dSetActiveExampleStage::Publish")
        && !source.contains(r#""setActiveExample" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn set_active_example_hostile_static_law_rejects_whole_document_reset() {
    let source: &str = &editor_source();
    assert!(set_active_example_is_cursorized(source));
    let direct = source.replace(
        r#""setActiveExample" => owned!(Puzzle3dSetActiveExampleWork, |request, scope| Puzzle3dSetActiveExampleWork::from_original_construction(scope))"#,
        r#""setActiveExample" => work!(|request| crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#,
    );
    assert!(!set_active_example_is_cursorized(&direct));
    for marker in [
        "Puzzle3dSetActiveExampleStage::DeleteAttractions",
        "Puzzle3dSetActiveExampleStage::DeleteObjects",
        "Puzzle3dSetActiveExampleStage::DeleteVolumes",
        "Puzzle3dSetActiveExampleStage::DeleteReferences",
        "Puzzle3dSetActiveExampleStage::DeleteCompatibility",
        "Puzzle3dSetActiveExampleStage::CreateObjects",
        "Puzzle3dSetActiveExampleStage::CreateAttractions",
        "Puzzle3dSetActiveExampleStage::CreateVolumes",
        "Puzzle3dSetActiveExampleStage::CreateReferences",
        "Puzzle3dSetActiveExampleStage::CreateCompatibility",
        "Puzzle3dSetActiveExampleStage::Publish",
    ] {
        assert!(!set_active_example_is_cursorized(&source.replace(marker, "cursor-removed")), "missing example cursor was falsely accepted: {marker}");
    }
}
