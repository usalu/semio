/// 🎬️ Oracle cases and hostile sessions: the actual oracle case, the operation begins, the named grants, hostile sessions and setups, the cursor match and the rollback boundary assertion.

fn flow_oracle_actual_case(feature: &str, session: &FlowRetainedVcs, page: FlowVcsPage) -> FlowOracleCase {
    let fingerprint = session.resource_fingerprint();
    let document = semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(session.document.as_ref().expect("oracle retained document").host_snapshot()));
    FlowOracleCase {
        feature: feature.to_owned(),
        document: flow_oracle_canonical_json(&document),
        page: FlowOraclePage {
            sequence: page.sequence,
            operation: page.operation,
            session_generation: page.session_generation,
            revision: page.revision,
            parent_revision: page.parent_revision,
            document_generation: page.document_generation,
            widget_count: page.widget_count,
            synapse_count: page.synapse_count,
            layout_count: page.layout_count,
            semantic_digest: page.semantic_digest,
        },
        history: FlowOracleHistory { undo_owners: fingerprint.undo_owners, redo_owners: fingerprint.redo_owners },
        handback: FlowOracleHandback {
            credits: [fingerprint.credits.operations, fingerprint.credits.pages, fingerprint.credits.items, fingerprint.credits.bytes, fingerprint.credits.outputs, fingerprint.credits.events, fingerprint.credits.controls],
            active_operations: fingerprint.active_operations,
            leased_pages: fingerprint.leased_pages,
            undo_owners: fingerprint.undo_owners,
            redo_owners: fingerprint.redo_owners,
            retired_action_owners: fingerprint.retired_action_owners,
            retired_surface_owners: fingerprint.retired_surface_owners,
            revision: fingerprint.revision,
            parent_revision: fingerprint.parent_revision,
            document_generation: fingerprint.document_generation,
            document_digest: fingerprint.document_digest,
            document_versions: fingerprint.document_versions,
            active_document_version: fingerprint.active_document_version,
            edit_owner: fingerprint.edit_owner,
            document_retained: fingerprint.document_retained,
            closing: fingerprint.closing,
        },
    }
}

fn flow_oracle_begin_operation(session: &mut FlowRetainedVcs, operation: &semio_framework_pack_json::Value) -> FlowVcsHandle {
    let feature = operation.get("feature").and_then(semio_framework_pack_json::Value::as_str).expect("oracle feature");
    let input = operation.get("input").expect("oracle operation input");
    let authority = session.authority();
    match feature {
        "addWidget" => {
            let mut source = FlowVcsSource::new(<Widget as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&input.get("widget").expect("oracle widget").clone())).expect("oracle widget input"));
            session.begin_add_widget(authority, flow_oracle_usize(input, "index"), &mut source).expect("oracle add widget")
        }
        "removeWidget" => {
            let mut source = FlowVcsSource::new(input.get("id").and_then(semio_framework_pack_json::Value::as_str).expect("oracle widget id").to_owned());
            session.begin_remove_widget(authority, &mut source).expect("oracle remove widget")
        }
        "moveWidget" => {
            let mut source = FlowVcsSource::new(input.get("id").and_then(semio_framework_pack_json::Value::as_str).expect("oracle widget id").to_owned());
            session.begin_move_widget(authority, flow_oracle_usize(input, "index"), &mut source).expect("oracle move widget")
        }
        "patchWidget" => {
            let mut id = FlowVcsSource::new(input.get("id").and_then(semio_framework_pack_json::Value::as_str).expect("oracle widget id").to_owned());
            let mut source = FlowVcsSource::new(<Widget as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&input.get("widget").expect("oracle widget").clone())).expect("oracle widget patch"));
            session.begin_patch_widget(authority, &mut id, &mut source).expect("oracle patch widget")
        }
        "addSynapse" => {
            let mut source = FlowVcsSource::new(<SynapseSpec as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&input.get("synapse").expect("oracle synapse").clone())).expect("oracle synapse input"));
            session.begin_add_synapse(authority, flow_oracle_usize(input, "index"), &mut source).expect("oracle add synapse")
        }
        "removeSynapse" => {
            let mut source = FlowVcsSource::new(input.get("id").and_then(semio_framework_pack_json::Value::as_str).expect("oracle synapse id").to_owned());
            session.begin_remove_synapse(authority, &mut source).expect("oracle remove synapse")
        }
        "moveSynapse" => {
            let mut source = FlowVcsSource::new(input.get("id").and_then(semio_framework_pack_json::Value::as_str).expect("oracle synapse id").to_owned());
            session.begin_move_synapse(authority, flow_oracle_usize(input, "index"), &mut source).expect("oracle move synapse")
        }
        "patchSynapse" => {
            let mut id = FlowVcsSource::new(input.get("id").and_then(semio_framework_pack_json::Value::as_str).expect("oracle synapse id").to_owned());
            let mut source = FlowVcsSource::new(<SynapseSpec as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&input.get("synapse").expect("oracle synapse").clone())).expect("oracle synapse patch"));
            session.begin_patch_synapse(authority, &mut id, &mut source).expect("oracle patch synapse")
        }
        "setLayout" => {
            let layout = input.get("layout").expect("oracle layout");
            let mut source = FlowVcsSource::new(FlowLayoutEntry {
                id: input.get("id").and_then(semio_framework_pack_json::Value::as_str).expect("oracle layout id").to_owned(),
                layout: Some(WidgetLayout { x: layout.get("x").and_then(semio_framework_pack_json::Value::as_f64).expect("oracle layout x"), y: layout.get("y").and_then(semio_framework_pack_json::Value::as_f64).expect("oracle layout y") }),
            });
            session.begin_set_layout(authority, &mut source).expect("oracle set layout")
        }
        "replaceDocument" => {
            let mut source = FlowVcsSource::new(<FlowHostSnapshot as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&input.get("document").expect("oracle replacement").clone())).expect("oracle replacement document"));
            session.begin_replace_document(authority, &mut source).expect("oracle replace document")
        }
        "undo" => session.begin_undo(authority).expect("oracle undo"),
        "redo" => session.begin_redo(authority).expect("oracle redo"),
        "checkpoint" => session.begin_checkpoint(authority).expect("oracle checkpoint"),
        _ => panic!("unsupported retained oracle operation {feature}"),
    }
}

fn flow_hostile_named_grant(lifecycle: &semio_framework_pack_json::Value, name: &str) -> FlowVcsGrant {
    let vector = lifecycle.get("grantVectors").and_then(semio_framework_pack_json::Value::as_array).and_then(|values| values.iter().find(|value| value.get("name").and_then(semio_framework_pack_json::Value::as_str) == Some(name))).expect("hostile named grant");
    flow_hostile_grant(vector.get("protocol").and_then(|value| value.get("call")).and_then(|value| value.get("grant")).expect("hostile named grant input"))
}

fn flow_hostile_session(lifecycle: &semio_framework_pack_json::Value, oracle: &semio_framework_pack_json::Value, protocol: &semio_framework_pack_json::Value) -> FlowRetainedVcs {
    let document_reference = protocol.get("document").and_then(semio_framework_pack_json::Value::as_str).expect("hostile protocol document");
    assert_eq!(document_reference, "oracle.initial");
    let document = <FlowHostSnapshot as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&oracle.get("initial").expect("hostile oracle initial").clone())).expect("hostile initial Flow fixture");
    let session = protocol.get("session").expect("hostile protocol session");
    let _ = lifecycle;
    FlowRetainedVcs::new(document, u32::try_from(flow_oracle_u64(session, "generation")).expect("hostile session generation"), flow_oracle_u64(session, "revision"), flow_oracle_u64(session, "parentRevision"))
}

fn flow_hostile_apply_setup(session: &mut FlowRetainedVcs, setup: &semio_framework_pack_json::Value) {
    let undo_owners = setup.get("undoOwners").and_then(semio_framework_pack_json::Value::as_u64).unwrap_or(0);
    let redo_owners = setup.get("redoOwners").and_then(semio_framework_pack_json::Value::as_u64).unwrap_or(0);
    for _ in 0..undo_owners {
        session.undo.push(FlowVcsAction::Checkpoint).expect("hostile undo setup");
    }
    for _ in 0..redo_owners {
        session.redo.push(FlowVcsAction::Checkpoint).expect("hostile redo setup");
    }
    if let Some(surface) = setup.get("surface") {
        session.bind_surface(flow_oracle_u64(surface, "surface"), flow_oracle_u64(surface, "host"), flow_oracle_u64(surface, "generation")).expect("hostile surface setup");
    }
}

fn flow_hostile_authority(session: &FlowRetainedVcs, operation: &semio_framework_pack_json::Value) -> FlowVcsAuthority {
    operation.get("authority").map_or_else(
        || session.authority(),
        |value| FlowVcsAuthority {
            session_generation: u32::try_from(flow_oracle_u64(value, "sessionGeneration")).expect("hostile authority session"),
            base_revision: flow_oracle_u64(value, "baseRevision"),
            parent_revision: flow_oracle_u64(value, "parentRevision"),
        },
    )
}

fn flow_hostile_begin_operation(session: &mut FlowRetainedVcs, lifecycle: &semio_framework_pack_json::Value, oracle: &semio_framework_pack_json::Value, operation: &semio_framework_pack_json::Value) -> FlowVcsHandle {
    let feature = operation.get("feature").and_then(semio_framework_pack_json::Value::as_str).expect("hostile feature");
    let authority = flow_hostile_authority(session, operation);
    match feature {
        "checkpoint" => session.begin_checkpoint(authority).expect("hostile checkpoint"),
        "undo" => session.begin_undo(authority).expect("hostile undo"),
        "setLayout" => {
            let input = operation.get("input").expect("hostile layout input");
            let layout = input.get("layout").expect("hostile layout value");
            let mut source = FlowVcsSource::new(FlowLayoutEntry {
                id: input.get("id").and_then(semio_framework_pack_json::Value::as_str).expect("hostile layout id").to_owned(),
                layout: Some(WidgetLayout { x: layout.get("x").and_then(semio_framework_pack_json::Value::as_f64).expect("hostile layout x"), y: layout.get("y").and_then(semio_framework_pack_json::Value::as_f64).expect("hostile layout y") }),
            });
            session.begin_set_layout(authority, &mut source).expect("hostile set layout")
        }
        "addWidget" => {
            let input = operation.get("input").expect("hostile widget input");
            let mut source = FlowVcsSource::new(<Widget as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&input.get("widget").expect("hostile widget").clone())).expect("hostile widget input"));
            session.begin_add_widget(authority, flow_oracle_usize(input, "index"), &mut source).expect("hostile add widget")
        }
        "removeWidget" => {
            let input = operation.get("input").expect("hostile remove input");
            let mut source = FlowVcsSource::new(input.get("id").and_then(semio_framework_pack_json::Value::as_str).expect("hostile remove id").to_owned());
            session.begin_remove_widget(authority, &mut source).expect("hostile remove widget")
        }
        "moveWidget" => {
            let input = operation.get("input").expect("hostile move input");
            let mut source = FlowVcsSource::new(input.get("id").and_then(semio_framework_pack_json::Value::as_str).expect("hostile move id").to_owned());
            session.begin_move_widget(authority, flow_oracle_usize(input, "index"), &mut source).expect("hostile move widget")
        }
        "replaceDocument" => {
            let reference = operation.get("input").and_then(|value| value.get("document")).expect("hostile replacement reference");
            let document = flow_hostile_resolve_document(lifecycle, oracle, reference);
            let mut source = FlowVcsSource::new(<FlowHostSnapshot as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&document.clone())).expect("hostile replacement document"));
            session.begin_replace_document(authority, &mut source).expect("hostile replace document")
        }
        _ => panic!("unsupported hostile operation {feature}"),
    }
}

fn flow_hostile_cursor_matches(session: &FlowRetainedVcs, handle: FlowVcsHandle, target: &semio_framework_pack_json::Value) -> bool {
    let operation = session.operations[usize::from(handle.slot)].as_ref().expect("hostile operation slot");
    let cursor = &operation.cursor;
    if let Some(phase) = target.get("phase").and_then(semio_framework_pack_json::Value::as_str) {
        let actual = match cursor.phase {
            FlowVcsCursorPhase::LoadHistory => "LoadHistory",
            FlowVcsCursorPhase::Scan => "Scan",
            FlowVcsCursorPhase::Mutate => "Mutate",
            FlowVcsCursorPhase::Shift => "Shift",
            FlowVcsCursorPhase::ReserveReplacement => "ReserveReplacement",
            FlowVcsCursorPhase::ReplaceSchema => "ReplaceSchema",
            FlowVcsCursorPhase::ReplaceCameraX => "ReplaceCameraX",
            FlowVcsCursorPhase::ReplaceCameraY => "ReplaceCameraY",
            FlowVcsCursorPhase::ReplaceCameraZoom => "ReplaceCameraZoom",
            FlowVcsCursorPhase::ReplaceWidgets => "ReplaceWidgets",
            FlowVcsCursorPhase::ReverseWidgets => "ReverseWidgets",
            FlowVcsCursorPhase::ReplaceSynapses => "ReplaceSynapses",
            FlowVcsCursorPhase::ReverseSynapses => "ReverseSynapses",
            FlowVcsCursorPhase::ReplaceLayout => "ReplaceLayout",
            FlowVcsCursorPhase::RetireRedo => "RetireRedo",
            FlowVcsCursorPhase::TransferHistory => "TransferHistory",
            FlowVcsCursorPhase::TransferSurface => "TransferSurface",
            FlowVcsCursorPhase::PublishVisibility => "PublishVisibility",
            FlowVcsCursorPhase::PublishPage => "PublishPage",
            FlowVcsCursorPhase::Rollback => "Rollback",
        };
        if actual != phase {
            return false;
        }
    }
    if let Some(kind) = target.get("kind").and_then(semio_framework_pack_json::Value::as_str) {
        let actual = match cursor.kind {
            FlowVcsCursorKind::None => "None",
            FlowVcsCursorKind::InsertWidget => "InsertWidget",
            FlowVcsCursorKind::RemoveWidget => "RemoveWidget",
            FlowVcsCursorKind::MoveWidget => "MoveWidget",
            FlowVcsCursorKind::PatchWidget => "PatchWidget",
            FlowVcsCursorKind::InsertSynapse => "InsertSynapse",
            FlowVcsCursorKind::RemoveSynapse => "RemoveSynapse",
            FlowVcsCursorKind::MoveSynapse => "MoveSynapse",
            FlowVcsCursorKind::PatchSynapse => "PatchSynapse",
            FlowVcsCursorKind::Layout => "Layout",
            FlowVcsCursorKind::ReplaceDocument => "ReplaceDocument",
        };
        if actual != kind {
            return false;
        }
    }
    if target.get("scan").is_some_and(|value| value.as_u64() != u64::try_from(cursor.scan).ok())
        || target.get("current").is_some_and(|value| value.as_u64() != u64::try_from(cursor.current).ok())
        || target.get("redoRetired").is_some_and(|value| value.as_u64() != u64::try_from(cursor.redo_retired).ok())
        || target.get("historyLoaded").is_some_and(|value| value.as_bool() != Some(cursor.history_loaded))
        || target.get("historyTransferred").is_some_and(|value| value.as_bool() != Some(cursor.history_transferred))
        || target.get("surfaceTransferred").is_some_and(|value| value.as_bool() != Some(cursor.surface_transferred))
        || target.get("visibilityPublished").is_some_and(|value| value.as_bool() != Some(cursor.visibility_published))
        || target.get("ownsEdit").is_some_and(|value| value.as_bool() != Some(cursor.owns_edit))
        || target.get("mutated").is_some_and(|value| value.as_bool() != Some(cursor.mutated))
    {
        return false;
    }
    let candidate = session.document.as_ref().and_then(|document| document.versions.get(cursor.target));
    if target.get("candidateWidgets").is_some_and(|value| value.as_u64() != candidate.and_then(|document| u64::try_from(document.widgets.len()).ok()))
        || target.get("candidateSynapses").is_some_and(|value| value.as_u64() != candidate.and_then(|document| u64::try_from(document.synapses.len()).ok()))
        || target.get("candidateLayout").is_some_and(|value| value.as_u64() != candidate.and_then(|document| u64::try_from(document.layout.len()).ok()))
    {
        return false;
    }
    true
}

fn flow_hostile_close_and_drain(session: &mut FlowRetainedVcs, handle: FlowVcsHandle, grant: FlowVcsGrant) {
    while !matches!(session.close_operation_step(handle, grant).expect("hostile operation close"),RetainedCloneStep::Complete(_)) {}
    while session.resource_fingerprint().retired_action_owners > 0 || session.resource_fingerprint().retired_surface_owners > 0 {
        matches!(session.close_retired_step(grant).expect("hostile retirement close"),RetainedCloneStep::Complete(_));
    }
}

fn flow_hostile_expected_handle(value: &semio_framework_pack_json::Value) -> FlowVcsHandle {
    FlowVcsHandle {
        operation: flow_oracle_u64(value, "operation"),
        slot: u8::try_from(flow_oracle_u64(value, "slot")).expect("hostile handle slot"),
        generation: u32::try_from(flow_oracle_u64(value, "generation")).expect("hostile handle generation"),
    }
}

fn flow_hostile_surface_owner(value: &semio_framework_pack_json::Value) -> FlowSurfaceOwner {
    FlowSurfaceOwner {
        surface: flow_oracle_u64(value, "surface"),
        host: flow_oracle_u64(value, "host"),
        generation: flow_oracle_u64(value, "generation"),
        document: flow_oracle_usize(value, "document"),
        widgets: flow_oracle_usize(value, "widgets"),
        synapses: flow_oracle_usize(value, "synapses"),
        previews: flow_oracle_usize(value, "previews"),
        expanded: flow_oracle_usize(value, "expanded"),
        layout: flow_oracle_usize(value, "layout"),
        history: flow_oracle_usize(value, "history"),
        edit: flow_oracle_usize(value, "edit"),
        conflict: flow_oracle_usize(value, "conflict"),
        control: flow_oracle_usize(value, "control"),
        output: flow_oracle_usize(value, "output"),
    }
}

fn flow_hostile_assert_rollback_boundary(session: &FlowRetainedVcs, handle: FlowVcsHandle, operation_fixture: &semio_framework_pack_json::Value, target: &semio_framework_pack_json::Value, expected: &semio_framework_pack_json::Value) {
    let operation = session.operations[usize::from(handle.slot)].as_ref().expect("rollback operation");
    let stage = match operation.stage {
        FlowVcsStage::Cancelled => "Cancelled",
        FlowVcsStage::Faulted => "Faulted",
        _ => "Unexpected",
    };
    assert_eq!(stage, expected.get("stage").and_then(semio_framework_pack_json::Value::as_str).expect("rollback stage"));
    assert_eq!(operation.authority, flow_hostile_authority(session, expected));
    assert_eq!(operation.authority, flow_hostile_authority(session, operation_fixture));
    let surface = target.get("surfaceOwner").expect("rollback surface owner");
    let owner = flow_hostile_surface_owner(surface);
    match surface.get("location").and_then(semio_framework_pack_json::Value::as_str).expect("rollback surface location") {
        "retired" => {
            assert!(session.document.as_ref().expect("rollback document").surface.is_none());
            assert_eq!(session.retired_surfaces.len(), 1);
            assert_eq!(session.retired_surfaces.get(0), Some(&owner));
        }
        "document" => {
            assert_eq!(session.document.as_ref().expect("rollback document").surface.as_ref(), Some(&owner));
            assert_eq!(session.retired_surfaces.len(), 0);
        }
        location => panic!("unsupported rollback surface location {location}"),
    }
}
