/// 🧨️ Hostile fingerprints and scalars: the expected and actual hostile fingerprints and states, the grants, fault names, fixture digest, scalar paths and the signed scalar mutation law helper.

fn flow_hostile_expected_fingerprint(lifecycle: &semio_framework_pack_json::Value, name: &str) -> FlowOracleHandback {
    let value = lifecycle.get("fingerprints").and_then(|values| values.get(name)).expect("hostile fingerprint reference");
    let credits = value.get("credits").expect("hostile fingerprint credits");
    FlowOracleHandback {
        credits: [
            flow_oracle_usize(credits, "operations"),
            flow_oracle_usize(credits, "pages"),
            flow_oracle_usize(credits, "items"),
            flow_oracle_usize(credits, "bytes"),
            flow_oracle_usize(credits, "outputs"),
            flow_oracle_usize(credits, "events"),
            flow_oracle_usize(credits, "controls"),
        ],
        active_operations: flow_oracle_usize(value, "activeOperations"),
        leased_pages: flow_oracle_usize(value, "leasedPages"),
        undo_owners: flow_oracle_usize(value, "undoOwners"),
        redo_owners: flow_oracle_usize(value, "redoOwners"),
        retired_action_owners: flow_oracle_usize(value, "retiredActionOwners"),
        retired_surface_owners: flow_oracle_usize(value, "retiredSurfaceOwners"),
        revision: flow_oracle_u64(value, "revision"),
        parent_revision: flow_oracle_u64(value, "parentRevision"),
        document_generation: flow_oracle_u64(value, "documentGeneration"),
        document_digest: flow_oracle_u64(value, "documentDigest"),
        document_versions: flow_oracle_usize(value, "documentVersions"),
        active_document_version: flow_oracle_usize(value, "activeDocumentVersion"),
        edit_owner: value.get("editOwner").and_then(semio_framework_pack_json::Value::as_u64),
        document_retained: value.get("documentRetained").and_then(semio_framework_pack_json::Value::as_bool).expect("hostile document retained"),
        closing: value.get("closing").and_then(semio_framework_pack_json::Value::as_bool).expect("hostile closing"),
    }
}

fn flow_hostile_resolve_document<'a>(lifecycle: &'a semio_framework_pack_json::Value, oracle: &'a semio_framework_pack_json::Value, reference: &semio_framework_pack_json::Value) -> &'a semio_framework_pack_json::Value {
    let fixture = reference.get("fixture").and_then(semio_framework_pack_json::Value::as_str).expect("hostile document fixture");
    let path = reference.get("path").and_then(semio_framework_pack_json::Value::as_str).expect("hostile document path");
    match (fixture, path) {
        ("oracle", "initial") => oracle.get("initial").expect("oracle initial document"),
        ("lifecycle", "protocolDocuments.replacementBoundary") => lifecycle.get("protocolDocuments").and_then(|value| value.get("replacementBoundary")).expect("replacement boundary document"),
        ("lifecycle", "protocolDocuments.publishedLayoutBoundary") => lifecycle.get("protocolDocuments").and_then(|value| value.get("publishedLayoutBoundary")).expect("published layout boundary document"),
        _ => panic!("unsupported hostile document reference {fixture}:{path}"),
    }
}

fn flow_hostile_expected_state(lifecycle: &semio_framework_pack_json::Value, oracle: &semio_framework_pack_json::Value, name: &str) -> FlowHostileState {
    let state = lifecycle.get("expectedStates").and_then(|states| states.get(name)).expect("hostile expected state reference");
    let document = flow_hostile_resolve_document(lifecycle, oracle, state.get("document").expect("hostile expected document"));
    assert!(state.get("page").is_some_and(semio_framework_pack_json::Value::is_null), "hostile state page must be explicitly null");
    let history = flow_oracle_expected_history(state.get("history").expect("hostile expected history"));
    let fingerprint_name = state.get("handback").and_then(|value| value.get("fingerprint")).and_then(semio_framework_pack_json::Value::as_str).expect("hostile fingerprint name");
    FlowHostileState { document: flow_oracle_canonical_json(document), page: None, history, handback: flow_hostile_expected_fingerprint(lifecycle, fingerprint_name) }
}

fn flow_hostile_actual_fingerprint(fingerprint: FlowVcsResourceFingerprint) -> FlowOracleHandback {
    FlowOracleHandback {
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
    }
}

fn flow_hostile_actual_state(session: &FlowRetainedVcs) -> FlowHostileState {
    let fingerprint = session.resource_fingerprint();
    let document = semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(session.document.as_ref().expect("hostile retained document").host_snapshot()));
    let page = session.operations[0]
        .as_ref()
        .and_then(|operation| operation.page)
        .or_else(|| session.operations[1].as_ref().and_then(|operation| operation.page))
        .or_else(|| session.operations[2].as_ref().and_then(|operation| operation.page))
        .or_else(|| session.operations[3].as_ref().and_then(|operation| operation.page))
        .map(|page| FlowOraclePage {
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
        });
    FlowHostileState { document: flow_oracle_canonical_json(&document), page, history: FlowOracleHistory { undo_owners: fingerprint.undo_owners, redo_owners: fingerprint.redo_owners }, handback: flow_hostile_actual_fingerprint(fingerprint) }
}

fn flow_hostile_grant(value: &semio_framework_pack_json::Value) -> FlowVcsGrant {
    FlowVcsGrant {
        retained: RetainedCloneGrant {maximum_items:flow_oracle_usize(value.get("retained").unwrap(),"maximumItems"),maximum_copy_bytes:flow_oracle_usize(value.get("retained").unwrap(),"maximumCopyBytes"),maximum_capacity_bytes:flow_oracle_usize(value.get("retained").unwrap(),"maximumCapacityBytes"),maximum_release_bytes:flow_oracle_usize(value.get("retained").unwrap(),"maximumReleaseBytes"),maximum_depth:flow_oracle_usize(value.get("retained").unwrap(),"maximumDepth")},
        items: flow_oracle_usize(value, "items"),
        bytes: flow_oracle_usize(value, "bytes"),
        outputs: flow_oracle_usize(value, "outputs"),
        events: flow_oracle_usize(value, "events"),
        controls: flow_oracle_usize(value, "controls"),
        fuel: u32::try_from(flow_oracle_u64(value, "fuel")).expect("hostile grant fuel"),
        now_milliseconds: flow_oracle_u64(value, "nowMilliseconds"),
        deadline_milliseconds: flow_oracle_u64(value, "deadlineMilliseconds"),
        interrupted: value.get("interrupted").and_then(semio_framework_pack_json::Value::as_bool).expect("hostile grant interruption"),
    }
}

fn flow_hostile_fault_name(fault: FlowVcsFault) -> &'static str {
    match fault {
        FlowVcsFault::Limit => "limit",
        FlowVcsFault::SourceExhausted => "sourceExhausted",
        FlowVcsFault::WrongHandle => "wrongHandle",
        FlowVcsFault::StaleHandle => "staleHandle",
        FlowVcsFault::StaleAuthority => "staleAuthority",
        FlowVcsFault::DuplicateControl => "duplicateControl",
        FlowVcsFault::InsufficientGrant => "insufficientGrant",
        FlowVcsFault::InvalidMutation => "invalidMutation",
        _ => "unexpectedFault",
    }
}

#[derive(Clone)]
enum FlowHostilePath {
    Key(String),
    Index(usize),
}

fn flow_hostile_fixture_digest(value: &semio_framework_pack_json::Value) -> u64 {
    let mut digest = 14_695_981_039_346_656_037u64;
    for byte in flow_oracle_canonical_json(value).as_bytes() {
        digest ^= u64::from(*byte);
        digest = digest.wrapping_mul(1_099_511_628_211);
    }
    digest
}

fn flow_hostile_scalar_paths(value: &semio_framework_pack_json::Value, path: &mut Vec<FlowHostilePath>, output: &mut Vec<Vec<FlowHostilePath>>) {
    match value {
        semio_framework_pack_json::Value::Array(values) => {
            for (index, value) in values.iter().enumerate() {
                path.push(FlowHostilePath::Index(index));
                flow_hostile_scalar_paths(value, path, output);
                path.pop();
            }
        }
        semio_framework_pack_json::Value::Object(values) => {
            for (key, value) in values.iter() {
                path.push(FlowHostilePath::Key(key.to_string()));
                flow_hostile_scalar_paths(value, path, output);
                path.pop();
            }
        }
        _ => output.push(path.clone()),
    }
}

fn flow_hostile_mutate_scalar(value: &mut semio_framework_pack_json::Value, path: &[FlowHostilePath]) {
    let mut target = value;
    for component in path {
        target = match component {
            FlowHostilePath::Key(key) => target.get_mut(key).expect("hostile mutation key"),
            FlowHostilePath::Index(index) => target.as_array_mut().and_then(|array| array.get_mut(*index)).expect("hostile mutation index"),
        };
    }
    *target = match target {
        semio_framework_pack_json::Value::Null => semio_framework_pack_json::Value::Bool(true),
        semio_framework_pack_json::Value::Bool(value) => semio_framework_pack_json::Value::Bool(!*value),
        semio_framework_pack_json::Value::Number(value) => semio_framework_pack_json::Value::Number((value.as_f64() + 1.0).into()),
        semio_framework_pack_json::Value::String(value) => semio_framework_pack_json::Value::String(format!("{value}!")),
        _ => unreachable!("hostile mutation targets scalars"),
    };
}

fn flow_hostile_assert_every_scalar_is_signed(value: &semio_framework_pack_json::Value, expected: u64) {
    assert_eq!(flow_hostile_fixture_digest(value), expected);
    let mut paths = Vec::new();
    flow_hostile_scalar_paths(value, &mut Vec::new(), &mut paths);
    assert!(!paths.is_empty());
    for path in paths {
        let mut mutation = value.clone();
        flow_hostile_mutate_scalar(&mut mutation, &path);
        assert_ne!(flow_hostile_fixture_digest(&mutation), expected, "every hostile vector scalar must affect its fixture signature");
    }
}
