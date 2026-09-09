
use super::*;
use crate::os_spr::{ArtifactId, Edit, Identified, Mutation, MutationDiff, SchemaId};
use crate::os_store::{create_document_envelope, ArtifactCommand, ArtifactStore, SnapshotRetirementFactory, SnapshotRetirementStep};
use neural::{Atom, Dictionary, Tree, Value as NeuralValue};
use std::sync::Arc;

#[derive(Debug, PartialEq)]
struct FlowOraclePage {
    sequence: u64,
    operation: u64,
    session_generation: u32,
    revision: u64,
    parent_revision: u64,
    document_generation: u64,
    widget_count: u32,
    synapse_count: u32,
    layout_count: u32,
    semantic_digest: u64,
}

#[derive(Debug, PartialEq)]
struct FlowOracleHistory {
    undo_owners: usize,
    redo_owners: usize,
}

#[derive(Debug, PartialEq)]
struct FlowOracleHandback {
    credits: [usize; 7],
    active_operations: usize,
    leased_pages: usize,
    undo_owners: usize,
    redo_owners: usize,
    retired_action_owners: usize,
    retired_surface_owners: usize,
    revision: u64,
    parent_revision: u64,
    document_generation: u64,
    document_digest: u64,
    document_versions: usize,
    active_document_version: usize,
    edit_owner: Option<u64>,
    document_retained: bool,
    closing: bool,
}

#[derive(Debug, PartialEq)]
struct FlowOracleCase {
    feature: String,
    document: String,
    page: FlowOraclePage,
    history: FlowOracleHistory,
    handback: FlowOracleHandback,
}

#[derive(Debug, PartialEq)]
struct FlowHostileState {
    document: String,
    page: Option<FlowOraclePage>,
    history: FlowOracleHistory,
    handback: FlowOracleHandback,
}

trait FlowSemanticOracle {
    fn evaluate_operations(&self, source: &str) -> Vec<FlowOracleCase>;
    fn expected_operations(&self, source: &str) -> Vec<FlowOracleCase>;
}

struct SerdeJsonFlowOracle;

impl FlowSemanticOracle for SerdeJsonFlowOracle {
    fn evaluate_operations(&self, source: &str) -> Vec<FlowOracleCase> {
        let root: crate::os_pack::json::Value = crate::os_pack::json::parse(source).expect("test-only Flow oracle fixture");
        let mut document = root.get("initial").expect("oracle initial document").clone();
        let mut undo: Vec<(crate::os_pack::json::Value, usize)> = Vec::new();
        let mut redo: Vec<(crate::os_pack::json::Value, usize)> = Vec::new();
        let mut versions = 1usize;
        let mut active = 0usize;
        let mut revision = 1u64;
        let mut document_generation = 1u64;
        let mut semantic_digest = flow_oracle_scalar_digest(&document);
        let mut results = Vec::new();
        let operations = root.get("operations").and_then(crate::os_pack::json::Value::as_array).expect("oracle operations");
        for (index, operation) in operations.iter().enumerate() {
            let feature = operation.get("feature").and_then(crate::os_pack::json::Value::as_str).expect("oracle feature");
            let input = operation.get("input").expect("oracle operation input");
            match feature {
                "undo" => {
                    let previous = undo.pop().expect("oracle undo owner");
                    redo.push((document.clone(), active));
                    document = previous.0;
                    active = previous.1;
                }
                "redo" => {
                    let next = redo.pop().expect("oracle redo owner");
                    undo.push((document.clone(), active));
                    document = next.0;
                    active = next.1;
                }
                "checkpoint" => {}
                _ => {
                    undo.push((document.clone(), active));
                    redo.clear();
                    flow_oracle_apply_operation(feature, input, &mut document);
                    if feature == "replaceDocument" {
                        versions += 1;
                        active = versions - 1;
                    }
                }
            }
            let parent_revision = revision;
            revision += 1;
            document_generation += 1;
            let widget_count = flow_oracle_collection_len(&document, "widgets");
            let synapse_count = flow_oracle_collection_len(&document, "synapses");
            let layout_count = flow_oracle_object_len(&document, "layout");
            semantic_digest = semantic_digest.rotate_left(13)
                ^ revision
                ^ u64::try_from(widget_count).expect("oracle widget count").rotate_left(7)
                ^ u64::try_from(synapse_count).expect("oracle synapse count").rotate_left(17)
                ^ u64::try_from(layout_count).expect("oracle layout count").rotate_left(29)
                ^ u64::try_from(active).expect("oracle active version");
            let page = FlowOraclePage {
                sequence: u64::try_from(index + 1).expect("oracle page sequence"),
                operation: u64::try_from(index + 1).expect("oracle operation id"),
                session_generation: 77,
                revision,
                parent_revision,
                document_generation,
                widget_count: u32::try_from(widget_count).expect("oracle widget count"),
                synapse_count: u32::try_from(synapse_count).expect("oracle synapse count"),
                layout_count: u32::try_from(layout_count).expect("oracle layout count"),
                semantic_digest,
            };
            let history = FlowOracleHistory { undo_owners: undo.len(), redo_owners: redo.len() };
            let fingerprint_name = operation.get("expected").and_then(|value| value.get("handback")).and_then(|value| value.get("fingerprint")).and_then(crate::os_pack::json::Value::as_str).expect("oracle fingerprint reference");
            let fingerprint = root.get("terminalFingerprints").and_then(|value| value.get(fingerprint_name)).expect("oracle terminal fingerprint");
            results.push(FlowOracleCase { feature: feature.to_owned(), document: flow_oracle_canonical_json(&document), handback: flow_oracle_expected_handback(fingerprint, &page, &history, versions, active), page, history });
        }
        results
    }

    fn expected_operations(&self, source: &str) -> Vec<FlowOracleCase> {
        let root: crate::os_pack::json::Value = crate::os_pack::json::parse(source).expect("test-only Flow oracle fixture");
        let documents = root.get("documents").and_then(crate::os_pack::json::Value::as_object).expect("oracle document ledger");
        root.get("operations")
            .and_then(crate::os_pack::json::Value::as_array)
            .expect("oracle operations")
            .iter()
            .map(|operation| {
                let feature = operation.get("feature").and_then(crate::os_pack::json::Value::as_str).expect("oracle feature").to_owned();
                let expected = operation.get("expected").expect("oracle expected result");
                let document_name = expected.get("document").and_then(crate::os_pack::json::Value::as_str).expect("oracle expected document");
                let page = flow_oracle_expected_page(expected.get("page").expect("oracle expected page"));
                let history = flow_oracle_expected_history(expected.get("history").expect("oracle expected history"));
                let handback = expected.get("handback").expect("oracle expected handback");
                let versions = flow_oracle_usize(handback, "documentVersions");
                let active = flow_oracle_usize(handback, "activeDocumentVersion");
                let fingerprint_name = handback.get("fingerprint").and_then(crate::os_pack::json::Value::as_str).expect("oracle fingerprint reference");
                let fingerprint = root.get("terminalFingerprints").and_then(|value| value.get(fingerprint_name)).expect("oracle terminal fingerprint");
                FlowOracleCase { feature, document: flow_oracle_canonical_json(documents.get(document_name).expect("oracle document reference")), handback: flow_oracle_expected_handback(fingerprint, &page, &history, versions, active), page, history }
            })
            .collect()
    }
}

fn flow_oracle_collection_len(document: &crate::os_pack::json::Value, key: &str) -> usize {
    document.get(key).and_then(crate::os_pack::json::Value::as_array).expect("oracle collection").len()
}

fn flow_oracle_object_len(document: &crate::os_pack::json::Value, key: &str) -> usize {
    document.get(key).and_then(crate::os_pack::json::Value::as_object).expect("oracle object").len()
}

fn flow_oracle_id_position(values: &[crate::os_pack::json::Value], id: &str) -> usize {
    values.iter().position(|value| value.get("id").and_then(crate::os_pack::json::Value::as_str) == Some(id)).expect("oracle retained id")
}

fn flow_oracle_apply_operation(feature: &str, input: &crate::os_pack::json::Value, document: &mut crate::os_pack::json::Value) {
    match feature {
        "addWidget" => {
            let index = flow_oracle_usize(input, "index");
            document.get_mut("widgets").and_then(crate::os_pack::json::Value::as_array_mut).expect("oracle widgets").insert(index, input.get("widget").expect("oracle widget input").clone());
        }
        "removeWidget" => {
            let id = input.get("id").and_then(crate::os_pack::json::Value::as_str).expect("oracle widget id");
            let widgets = document.get_mut("widgets").and_then(crate::os_pack::json::Value::as_array_mut).expect("oracle widgets");
            let index = flow_oracle_id_position(widgets, id);
            widgets.remove(index);
        }
        "moveWidget" => {
            let id = input.get("id").and_then(crate::os_pack::json::Value::as_str).expect("oracle widget id");
            let target = flow_oracle_usize(input, "index");
            let widgets = document.get_mut("widgets").and_then(crate::os_pack::json::Value::as_array_mut).expect("oracle widgets");
            let index = flow_oracle_id_position(widgets, id);
            let widget = widgets.remove(index);
            widgets.insert(target, widget);
        }
        "patchWidget" => {
            let id = input.get("id").and_then(crate::os_pack::json::Value::as_str).expect("oracle widget id");
            let widgets = document.get_mut("widgets").and_then(crate::os_pack::json::Value::as_array_mut).expect("oracle widgets");
            let index = flow_oracle_id_position(widgets, id);
            widgets[index] = input.get("widget").expect("oracle widget patch").clone();
        }
        "addSynapse" => {
            let index = flow_oracle_usize(input, "index");
            document.get_mut("synapses").and_then(crate::os_pack::json::Value::as_array_mut).expect("oracle synapses").insert(index, input.get("synapse").expect("oracle synapse input").clone());
        }
        "removeSynapse" => {
            let id = input.get("id").and_then(crate::os_pack::json::Value::as_str).expect("oracle synapse id");
            let synapses = document.get_mut("synapses").and_then(crate::os_pack::json::Value::as_array_mut).expect("oracle synapses");
            let index = flow_oracle_id_position(synapses, id);
            synapses.remove(index);
        }
        "moveSynapse" => {
            let id = input.get("id").and_then(crate::os_pack::json::Value::as_str).expect("oracle synapse id");
            let target = flow_oracle_usize(input, "index");
            let synapses = document.get_mut("synapses").and_then(crate::os_pack::json::Value::as_array_mut).expect("oracle synapses");
            let index = flow_oracle_id_position(synapses, id);
            let synapse = synapses.remove(index);
            synapses.insert(target, synapse);
        }
        "patchSynapse" => {
            let id = input.get("id").and_then(crate::os_pack::json::Value::as_str).expect("oracle synapse id");
            let synapses = document.get_mut("synapses").and_then(crate::os_pack::json::Value::as_array_mut).expect("oracle synapses");
            let index = flow_oracle_id_position(synapses, id);
            synapses[index] = input.get("synapse").expect("oracle synapse patch").clone();
        }
        "setLayout" => {
            let id = input.get("id").and_then(crate::os_pack::json::Value::as_str).expect("oracle layout id").to_owned();
            let layout = input.get("layout").expect("oracle layout input").clone();
            document.get_mut("layout").and_then(crate::os_pack::json::Value::as_object_mut).expect("oracle layout").insert(id, layout);
        }
        "replaceDocument" => *document = input.get("document").expect("oracle replacement").clone(),
        _ => panic!("unsupported oracle operation {feature}"),
    }
}

fn flow_oracle_scalar_digest(document: &crate::os_pack::json::Value) -> u64 {
    let schema = document.get("schema").and_then(crate::os_pack::json::Value::as_str).expect("oracle schema");
    let camera = document.get("camera").expect("oracle camera");
    14_695_981_039_346_656_037
        ^ u64::try_from(schema.len()).expect("oracle schema bytes").rotate_left(3)
        ^ u64::try_from(flow_oracle_collection_len(document, "widgets")).expect("oracle widget count").rotate_left(11)
        ^ u64::try_from(flow_oracle_collection_len(document, "synapses")).expect("oracle synapse count").rotate_left(23)
        ^ u64::try_from(flow_oracle_object_len(document, "layout")).expect("oracle layout count").rotate_left(37)
        ^ camera.get("x").and_then(crate::os_pack::json::Value::as_f64).expect("oracle camera x").to_bits()
        ^ camera.get("y").and_then(crate::os_pack::json::Value::as_f64).expect("oracle camera y").to_bits().rotate_left(17)
        ^ camera.get("zoom").and_then(crate::os_pack::json::Value::as_f64).expect("oracle camera zoom").to_bits().rotate_left(31)
}

fn flow_oracle_canonical_json(value: &crate::os_pack::json::Value) -> String {
    fn append(value: &crate::os_pack::json::Value, output: &mut String) {
        match value {
            crate::os_pack::json::Value::Null => output.push_str("null"),
            crate::os_pack::json::Value::Bool(value) => output.push_str(if *value { "true" } else { "false" }),
            crate::os_pack::json::Value::Number(value) => output.push_str(&format!("f64:{:016x}", value.as_f64().to_bits())),
            crate::os_pack::json::Value::String(value) => output.push_str(&crate::os_pack::json::to_string(&crate::os_pack::json::Value::String(value.clone()))),
            crate::os_pack::json::Value::Array(values) => {
                output.push('[');
                for value in values {
                    append(value, output);
                    output.push(',');
                }
                output.push(']');
            }
            crate::os_pack::json::Value::Object(values) => {
                let mut keys: Vec<&str> = values.iter().map(|(key, _)| key).collect();
                keys.sort();
                output.push('{');
                for key in keys {
                    output.push_str(&crate::os_pack::json::to_string(&crate::os_pack::json::Value::String(key.to_string())));
                    output.push(':');
                    append(values.get(key).expect("oracle value"), output);
                    output.push(',');
                }
                output.push('}');
            }
        }
    }
    let mut output = String::new();
    append(value, &mut output);
    output
}

fn flow_oracle_u64(value: &crate::os_pack::json::Value, key: &str) -> u64 {
    let value = value.get(key).expect("oracle numeric field");
    value.as_u64().or_else(|| value.as_str().and_then(|text| text.parse().ok())).expect("oracle u64 field")
}

fn flow_oracle_usize(value: &crate::os_pack::json::Value, key: &str) -> usize {
    usize::try_from(flow_oracle_u64(value, key)).expect("oracle usize field")
}

fn flow_oracle_expected_page(value: &crate::os_pack::json::Value) -> FlowOraclePage {
    FlowOraclePage {
        sequence: flow_oracle_u64(value, "sequence"),
        operation: flow_oracle_u64(value, "operation"),
        session_generation: u32::try_from(flow_oracle_u64(value, "sessionGeneration")).expect("oracle session generation"),
        revision: flow_oracle_u64(value, "revision"),
        parent_revision: flow_oracle_u64(value, "parentRevision"),
        document_generation: flow_oracle_u64(value, "documentGeneration"),
        widget_count: u32::try_from(flow_oracle_u64(value, "widgetCount")).expect("oracle widget count"),
        synapse_count: u32::try_from(flow_oracle_u64(value, "synapseCount")).expect("oracle synapse count"),
        layout_count: u32::try_from(flow_oracle_u64(value, "layoutCount")).expect("oracle layout count"),
        semantic_digest: flow_oracle_u64(value, "semanticDigest"),
    }
}

fn flow_oracle_expected_history(value: &crate::os_pack::json::Value) -> FlowOracleHistory {
    FlowOracleHistory { undo_owners: flow_oracle_usize(value, "undoOwners"), redo_owners: flow_oracle_usize(value, "redoOwners") }
}

fn flow_oracle_expected_handback(template: &crate::os_pack::json::Value, page: &FlowOraclePage, history: &FlowOracleHistory, document_versions: usize, active_document_version: usize) -> FlowOracleHandback {
    let credits = template.get("credits").expect("oracle terminal credits");
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
        active_operations: flow_oracle_usize(template, "activeOperations"),
        leased_pages: flow_oracle_usize(template, "leasedPages"),
        undo_owners: history.undo_owners,
        redo_owners: history.redo_owners,
        retired_action_owners: flow_oracle_usize(template, "retiredActionOwners"),
        retired_surface_owners: flow_oracle_usize(template, "retiredSurfaceOwners"),
        revision: page.revision,
        parent_revision: page.parent_revision,
        document_generation: page.document_generation,
        document_digest: page.semantic_digest,
        document_versions,
        active_document_version,
        edit_owner: template.get("editOwner").and_then(crate::os_pack::json::Value::as_u64),
        document_retained: template.get("documentRetained").and_then(crate::os_pack::json::Value::as_bool).expect("oracle document retained"),
        closing: template.get("closing").and_then(crate::os_pack::json::Value::as_bool).expect("oracle closing"),
    }
}

fn flow_hostile_expected_fingerprint(lifecycle: &crate::os_pack::json::Value, name: &str) -> FlowOracleHandback {
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
        edit_owner: value.get("editOwner").and_then(crate::os_pack::json::Value::as_u64),
        document_retained: value.get("documentRetained").and_then(crate::os_pack::json::Value::as_bool).expect("hostile document retained"),
        closing: value.get("closing").and_then(crate::os_pack::json::Value::as_bool).expect("hostile closing"),
    }
}

fn flow_hostile_resolve_document<'a>(lifecycle: &'a crate::os_pack::json::Value, oracle: &'a crate::os_pack::json::Value, reference: &crate::os_pack::json::Value) -> &'a crate::os_pack::json::Value {
    let fixture = reference.get("fixture").and_then(crate::os_pack::json::Value::as_str).expect("hostile document fixture");
    let path = reference.get("path").and_then(crate::os_pack::json::Value::as_str).expect("hostile document path");
    match (fixture, path) {
        ("oracle", "initial") => oracle.get("initial").expect("oracle initial document"),
        ("lifecycle", "protocolDocuments.replacementBoundary") => lifecycle.get("protocolDocuments").and_then(|value| value.get("replacementBoundary")).expect("replacement boundary document"),
        ("lifecycle", "protocolDocuments.publishedLayoutBoundary") => lifecycle.get("protocolDocuments").and_then(|value| value.get("publishedLayoutBoundary")).expect("published layout boundary document"),
        _ => panic!("unsupported hostile document reference {fixture}:{path}"),
    }
}

fn flow_hostile_expected_state(lifecycle: &crate::os_pack::json::Value, oracle: &crate::os_pack::json::Value, name: &str) -> FlowHostileState {
    let state = lifecycle.get("expectedStates").and_then(|states| states.get(name)).expect("hostile expected state reference");
    let document = flow_hostile_resolve_document(lifecycle, oracle, state.get("document").expect("hostile expected document"));
    assert!(state.get("page").is_some_and(crate::os_pack::json::Value::is_null), "hostile state page must be explicitly null");
    let history = flow_oracle_expected_history(state.get("history").expect("hostile expected history"));
    let fingerprint_name = state.get("handback").and_then(|value| value.get("fingerprint")).and_then(crate::os_pack::json::Value::as_str).expect("hostile fingerprint name");
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
    let document = crate::os_pack::json::from_dsl_value(&crate::os_dsl::ToValue::to_value(session.document.as_ref().expect("hostile retained document").fixture()));
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

fn flow_hostile_grant(value: &crate::os_pack::json::Value) -> FlowVcsGrant {
    FlowVcsGrant {
        items: flow_oracle_usize(value, "items"),
        bytes: flow_oracle_usize(value, "bytes"),
        outputs: flow_oracle_usize(value, "outputs"),
        events: flow_oracle_usize(value, "events"),
        controls: flow_oracle_usize(value, "controls"),
        fuel: u32::try_from(flow_oracle_u64(value, "fuel")).expect("hostile grant fuel"),
        now_milliseconds: flow_oracle_u64(value, "nowMilliseconds"),
        deadline_milliseconds: flow_oracle_u64(value, "deadlineMilliseconds"),
        interrupted: value.get("interrupted").and_then(crate::os_pack::json::Value::as_bool).expect("hostile grant interruption"),
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

fn flow_hostile_fixture_digest(value: &crate::os_pack::json::Value) -> u64 {
    let mut digest = 14_695_981_039_346_656_037u64;
    for byte in flow_oracle_canonical_json(value).as_bytes() {
        digest ^= u64::from(*byte);
        digest = digest.wrapping_mul(1_099_511_628_211);
    }
    digest
}

fn flow_hostile_scalar_paths(value: &crate::os_pack::json::Value, path: &mut Vec<FlowHostilePath>, output: &mut Vec<Vec<FlowHostilePath>>) {
    match value {
        crate::os_pack::json::Value::Array(values) => {
            for (index, value) in values.iter().enumerate() {
                path.push(FlowHostilePath::Index(index));
                flow_hostile_scalar_paths(value, path, output);
                path.pop();
            }
        }
        crate::os_pack::json::Value::Object(values) => {
            for (key, value) in values.iter() {
                path.push(FlowHostilePath::Key(key.to_string()));
                flow_hostile_scalar_paths(value, path, output);
                path.pop();
            }
        }
        _ => output.push(path.clone()),
    }
}

fn flow_hostile_mutate_scalar(value: &mut crate::os_pack::json::Value, path: &[FlowHostilePath]) {
    let mut target = value;
    for component in path {
        target = match component {
            FlowHostilePath::Key(key) => target.get_mut(key).expect("hostile mutation key"),
            FlowHostilePath::Index(index) => target.as_array_mut().and_then(|array| array.get_mut(*index)).expect("hostile mutation index"),
        };
    }
    *target = match target {
        crate::os_pack::json::Value::Null => crate::os_pack::json::Value::Bool(true),
        crate::os_pack::json::Value::Bool(value) => crate::os_pack::json::Value::Bool(!*value),
        crate::os_pack::json::Value::Number(value) => crate::os_pack::json::Value::Number((value.as_f64() + 1.0).into()),
        crate::os_pack::json::Value::String(value) => crate::os_pack::json::Value::String(format!("{value}!")),
        _ => unreachable!("hostile mutation targets scalars"),
    };
}

fn flow_hostile_assert_every_scalar_is_signed(value: &crate::os_pack::json::Value, expected: u64) {
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

fn flow_oracle_actual_case(feature: &str, session: &FlowRetainedVcs, page: FlowVcsPage) -> FlowOracleCase {
    let fingerprint = session.resource_fingerprint();
    let document = crate::os_pack::json::from_dsl_value(&crate::os_dsl::ToValue::to_value(session.document.as_ref().expect("oracle retained document").fixture()));
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

fn flow_oracle_begin_operation(session: &mut FlowRetainedVcs, operation: &crate::os_pack::json::Value) -> FlowVcsHandle {
    let feature = operation.get("feature").and_then(crate::os_pack::json::Value::as_str).expect("oracle feature");
    let input = operation.get("input").expect("oracle operation input");
    let authority = session.authority();
    match feature {
        "addWidget" => {
            let mut source = FlowVcsSource::new(<Widget as crate::os_dsl::FromValue>::from_value(crate::os_pack::json::to_dsl_value(&input.get("widget").expect("oracle widget").clone())).expect("oracle widget input"));
            session.begin_add_widget(authority, flow_oracle_usize(input, "index"), &mut source).expect("oracle add widget")
        }
        "removeWidget" => {
            let mut source = FlowVcsSource::new(input.get("id").and_then(crate::os_pack::json::Value::as_str).expect("oracle widget id").to_owned());
            session.begin_remove_widget(authority, &mut source).expect("oracle remove widget")
        }
        "moveWidget" => {
            let mut source = FlowVcsSource::new(input.get("id").and_then(crate::os_pack::json::Value::as_str).expect("oracle widget id").to_owned());
            session.begin_move_widget(authority, flow_oracle_usize(input, "index"), &mut source).expect("oracle move widget")
        }
        "patchWidget" => {
            let mut id = FlowVcsSource::new(input.get("id").and_then(crate::os_pack::json::Value::as_str).expect("oracle widget id").to_owned());
            let mut source = FlowVcsSource::new(<Widget as crate::os_dsl::FromValue>::from_value(crate::os_pack::json::to_dsl_value(&input.get("widget").expect("oracle widget").clone())).expect("oracle widget patch"));
            session.begin_patch_widget(authority, &mut id, &mut source).expect("oracle patch widget")
        }
        "addSynapse" => {
            let mut source = FlowVcsSource::new(<SynapseSpec as crate::os_dsl::FromValue>::from_value(crate::os_pack::json::to_dsl_value(&input.get("synapse").expect("oracle synapse").clone())).expect("oracle synapse input"));
            session.begin_add_synapse(authority, flow_oracle_usize(input, "index"), &mut source).expect("oracle add synapse")
        }
        "removeSynapse" => {
            let mut source = FlowVcsSource::new(input.get("id").and_then(crate::os_pack::json::Value::as_str).expect("oracle synapse id").to_owned());
            session.begin_remove_synapse(authority, &mut source).expect("oracle remove synapse")
        }
        "moveSynapse" => {
            let mut source = FlowVcsSource::new(input.get("id").and_then(crate::os_pack::json::Value::as_str).expect("oracle synapse id").to_owned());
            session.begin_move_synapse(authority, flow_oracle_usize(input, "index"), &mut source).expect("oracle move synapse")
        }
        "patchSynapse" => {
            let mut id = FlowVcsSource::new(input.get("id").and_then(crate::os_pack::json::Value::as_str).expect("oracle synapse id").to_owned());
            let mut source = FlowVcsSource::new(<SynapseSpec as crate::os_dsl::FromValue>::from_value(crate::os_pack::json::to_dsl_value(&input.get("synapse").expect("oracle synapse").clone())).expect("oracle synapse patch"));
            session.begin_patch_synapse(authority, &mut id, &mut source).expect("oracle patch synapse")
        }
        "setLayout" => {
            let layout = input.get("layout").expect("oracle layout");
            let mut source = FlowVcsSource::new(FlowLayoutEntry {
                id: input.get("id").and_then(crate::os_pack::json::Value::as_str).expect("oracle layout id").to_owned(),
                layout: Some(WidgetLayout { x: layout.get("x").and_then(crate::os_pack::json::Value::as_f64).expect("oracle layout x"), y: layout.get("y").and_then(crate::os_pack::json::Value::as_f64).expect("oracle layout y") }),
            });
            session.begin_set_layout(authority, &mut source).expect("oracle set layout")
        }
        "replaceDocument" => {
            let mut source = FlowVcsSource::new(<FlowFixture as crate::os_dsl::FromValue>::from_value(crate::os_pack::json::to_dsl_value(&input.get("document").expect("oracle replacement").clone())).expect("oracle replacement document"));
            session.begin_replace_document(authority, &mut source).expect("oracle replace document")
        }
        "undo" => session.begin_undo(authority).expect("oracle undo"),
        "redo" => session.begin_redo(authority).expect("oracle redo"),
        "checkpoint" => session.begin_checkpoint(authority).expect("oracle checkpoint"),
        _ => panic!("unsupported retained oracle operation {feature}"),
    }
}

fn flow_hostile_named_grant(lifecycle: &crate::os_pack::json::Value, name: &str) -> FlowVcsGrant {
    let vector = lifecycle.get("grantVectors").and_then(crate::os_pack::json::Value::as_array).and_then(|values| values.iter().find(|value| value.get("name").and_then(crate::os_pack::json::Value::as_str) == Some(name))).expect("hostile named grant");
    flow_hostile_grant(vector.get("protocol").and_then(|value| value.get("call")).and_then(|value| value.get("grant")).expect("hostile named grant input"))
}

fn flow_hostile_session(lifecycle: &crate::os_pack::json::Value, oracle: &crate::os_pack::json::Value, protocol: &crate::os_pack::json::Value) -> FlowRetainedVcs {
    let document_reference = protocol.get("document").and_then(crate::os_pack::json::Value::as_str).expect("hostile protocol document");
    assert_eq!(document_reference, "oracle.initial");
    let document = <FlowFixture as crate::os_dsl::FromValue>::from_value(crate::os_pack::json::to_dsl_value(&oracle.get("initial").expect("hostile oracle initial").clone())).expect("hostile initial Flow fixture");
    let session = protocol.get("session").expect("hostile protocol session");
    let _ = lifecycle;
    FlowRetainedVcs::new(document, u32::try_from(flow_oracle_u64(session, "generation")).expect("hostile session generation"), flow_oracle_u64(session, "revision"), flow_oracle_u64(session, "parentRevision"))
}

fn flow_hostile_apply_setup(session: &mut FlowRetainedVcs, setup: &crate::os_pack::json::Value) {
    let undo_owners = setup.get("undoOwners").and_then(crate::os_pack::json::Value::as_u64).unwrap_or(0);
    let redo_owners = setup.get("redoOwners").and_then(crate::os_pack::json::Value::as_u64).unwrap_or(0);
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

fn flow_hostile_authority(session: &FlowRetainedVcs, operation: &crate::os_pack::json::Value) -> FlowVcsAuthority {
    operation.get("authority").map_or_else(
        || session.authority(),
        |value| FlowVcsAuthority {
            session_generation: u32::try_from(flow_oracle_u64(value, "sessionGeneration")).expect("hostile authority session"),
            base_revision: flow_oracle_u64(value, "baseRevision"),
            parent_revision: flow_oracle_u64(value, "parentRevision"),
        },
    )
}

fn flow_hostile_begin_operation(session: &mut FlowRetainedVcs, lifecycle: &crate::os_pack::json::Value, oracle: &crate::os_pack::json::Value, operation: &crate::os_pack::json::Value) -> FlowVcsHandle {
    let feature = operation.get("feature").and_then(crate::os_pack::json::Value::as_str).expect("hostile feature");
    let authority = flow_hostile_authority(session, operation);
    match feature {
        "checkpoint" => session.begin_checkpoint(authority).expect("hostile checkpoint"),
        "undo" => session.begin_undo(authority).expect("hostile undo"),
        "setLayout" => {
            let input = operation.get("input").expect("hostile layout input");
            let layout = input.get("layout").expect("hostile layout value");
            let mut source = FlowVcsSource::new(FlowLayoutEntry {
                id: input.get("id").and_then(crate::os_pack::json::Value::as_str).expect("hostile layout id").to_owned(),
                layout: Some(WidgetLayout { x: layout.get("x").and_then(crate::os_pack::json::Value::as_f64).expect("hostile layout x"), y: layout.get("y").and_then(crate::os_pack::json::Value::as_f64).expect("hostile layout y") }),
            });
            session.begin_set_layout(authority, &mut source).expect("hostile set layout")
        }
        "addWidget" => {
            let input = operation.get("input").expect("hostile widget input");
            let mut source = FlowVcsSource::new(<Widget as crate::os_dsl::FromValue>::from_value(crate::os_pack::json::to_dsl_value(&input.get("widget").expect("hostile widget").clone())).expect("hostile widget input"));
            session.begin_add_widget(authority, flow_oracle_usize(input, "index"), &mut source).expect("hostile add widget")
        }
        "removeWidget" => {
            let input = operation.get("input").expect("hostile remove input");
            let mut source = FlowVcsSource::new(input.get("id").and_then(crate::os_pack::json::Value::as_str).expect("hostile remove id").to_owned());
            session.begin_remove_widget(authority, &mut source).expect("hostile remove widget")
        }
        "moveWidget" => {
            let input = operation.get("input").expect("hostile move input");
            let mut source = FlowVcsSource::new(input.get("id").and_then(crate::os_pack::json::Value::as_str).expect("hostile move id").to_owned());
            session.begin_move_widget(authority, flow_oracle_usize(input, "index"), &mut source).expect("hostile move widget")
        }
        "replaceDocument" => {
            let reference = operation.get("input").and_then(|value| value.get("document")).expect("hostile replacement reference");
            let document = flow_hostile_resolve_document(lifecycle, oracle, reference);
            let mut source = FlowVcsSource::new(<FlowFixture as crate::os_dsl::FromValue>::from_value(crate::os_pack::json::to_dsl_value(&document.clone())).expect("hostile replacement document"));
            session.begin_replace_document(authority, &mut source).expect("hostile replace document")
        }
        _ => panic!("unsupported hostile operation {feature}"),
    }
}

fn flow_hostile_cursor_matches(session: &FlowRetainedVcs, handle: FlowVcsHandle, target: &crate::os_pack::json::Value) -> bool {
    let operation = session.operations[usize::from(handle.slot)].as_ref().expect("hostile operation slot");
    let cursor = &operation.cursor;
    if let Some(phase) = target.get("phase").and_then(crate::os_pack::json::Value::as_str) {
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
    if let Some(kind) = target.get("kind").and_then(crate::os_pack::json::Value::as_str) {
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
    while !session.close_operation_step(handle, grant).expect("hostile operation close") {}
    while session.resource_fingerprint().retired_action_owners > 0 || session.resource_fingerprint().retired_surface_owners > 0 {
        session.close_retired_step(grant).expect("hostile retirement close");
    }
}

fn flow_hostile_expected_handle(value: &crate::os_pack::json::Value) -> FlowVcsHandle {
    FlowVcsHandle {
        operation: flow_oracle_u64(value, "operation"),
        slot: u8::try_from(flow_oracle_u64(value, "slot")).expect("hostile handle slot"),
        generation: u32::try_from(flow_oracle_u64(value, "generation")).expect("hostile handle generation"),
    }
}

fn flow_hostile_surface_owner(value: &crate::os_pack::json::Value) -> FlowSurfaceOwner {
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

fn flow_hostile_assert_rollback_boundary(session: &FlowRetainedVcs, handle: FlowVcsHandle, operation_fixture: &crate::os_pack::json::Value, target: &crate::os_pack::json::Value, expected: &crate::os_pack::json::Value) {
    let operation = session.operations[usize::from(handle.slot)].as_ref().expect("rollback operation");
    let stage = match operation.stage {
        FlowVcsStage::Cancelled => "Cancelled",
        FlowVcsStage::Faulted => "Faulted",
        _ => "Unexpected",
    };
    assert_eq!(stage, expected.get("stage").and_then(crate::os_pack::json::Value::as_str).expect("rollback stage"));
    assert_eq!(operation.authority, flow_hostile_authority(session, expected));
    assert_eq!(operation.authority, flow_hostile_authority(session, operation_fixture));
    let surface = target.get("surfaceOwner").expect("rollback surface owner");
    let owner = flow_hostile_surface_owner(surface);
    match surface.get("location").and_then(crate::os_pack::json::Value::as_str).expect("rollback surface location") {
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

fn retained_grant() -> FlowVcsGrant {
    FlowVcsGrant { items: 1, bytes: 256, outputs: 1, events: 1, controls: 1, fuel: 1, now_milliseconds: 1, deadline_milliseconds: 8, interrupted: false }
}

fn rejected_control_grants() -> [FlowVcsGrant; 4] {
    let mut zero_fuel = retained_grant();
    zero_fuel.fuel = 0;
    let mut interrupted = retained_grant();
    interrupted.interrupted = true;
    let mut expired = retained_grant();
    expired.deadline_milliseconds = expired.now_milliseconds;
    let mut over_window = retained_grant();
    over_window.deadline_milliseconds = over_window.now_milliseconds + FLOW_VCS_DEADLINE_MILLISECONDS + 1;
    [zero_fuel, interrupted, expired, over_window]
}

fn retained_fixture() -> FlowFixture {
    let mut fixture = FlowFixture::default();
    fixture.widgets.push(Widget::InputNote { id: "source".into(), text: "retained".into() });
    fixture.widgets.push(Widget::OutputPreview { id: "preview".into(), preview: Dictionary::new(), expanded: crate::OrderedSet::from(["value".into()]) });
    fixture.synapses.push(SynapseSpec { id: "source-preview".into(), from: "source".into(), to: "preview".into(), from_port: "text".into(), to_port: String::new() });
    fixture.layout.insert("source".into(), WidgetLayout { x: 1.0, y: 2.0 });
    fixture.layout.insert("preview".into(), WidgetLayout { x: 4.0, y: 5.0 });
    fixture
}

fn drive_to_preview(session: &mut FlowRetainedVcs, handle: FlowVcsHandle) -> FlowVcsPoll {
    for _ in 0..(FLOW_VCS_MAX_ITEMS * 4 + 32) {
        let event = session.poll(handle, retained_grant()).expect("retained cursor step");
        if matches!(event, FlowVcsPoll::Preview { .. }) {
            return event;
        }
    }
    panic!("retained cursor exceeded its fixed semantic bound")
}

fn publish_and_close(session: &mut FlowRetainedVcs, handle: FlowVcsHandle) -> FlowVcsPage {
    drive_to_preview(session, handle);
    let page = session.take_page(handle).expect("published page");
    session.acknowledge_page(handle, page.sequence).expect("page acknowledgement");
    while !session.close_operation_step(handle, retained_grant()).expect("published operation close") {}
    page
}

//#region 📍️OrderedLayoutLaws
#[test]
fn retained_vcs_shared_snapshot_readers_retire_without_waiting_on_each_other() {
    let fixture: crate::os_pack::json::Value = crate::os_pack::json::parse(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let snapshot = Arc::new(<FlowFixture as crate::os_dsl::FromValue>::from_value(crate::os_pack::json::to_dsl_value(&fixture["initial"].clone())).unwrap());
    let mut readers = [std::mem::ManuallyDrop::new(FlowSnapshotRetirementFactory.retire(Arc::clone(&snapshot))), std::mem::ManuallyDrop::new(FlowSnapshotRetirementFactory.retire(snapshot))];
    for reader in &mut readers {
        assert!(matches!(reader.close_step(0, 256).unwrap(), SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
        assert!(!reader.terminal_is_empty());
    }
    for _ in 0..4096 {
        for reader in &mut readers {
            if !reader.terminal_is_empty() {
                reader.close_step(1, 256).unwrap();
            }
        }
        if readers.iter().all(|reader| reader.terminal_is_empty()) {
            for reader in &mut readers {
                unsafe {
                    std::mem::ManuallyDrop::drop(reader);
                }
            }
            return;
        }
    }
    panic!("shared snapshot readers retained each other's final-owner claim");
}

fn close_layout_session(session: &mut FlowRetainedVcs) {
    session.begin_close();
    for _ in 0..4096 {
        if session.close_retired_step(retained_grant()).expect("layout session retirement") {
            assert!(session.terminal_is_empty());
            return;
        }
    }
    panic!("layout session did not retire within its fixture bound");
}

#[test]
fn retained_vcs_ordered_layout_edits_undo_redo_match_json_oracle() {
    let fixture: crate::os_pack::json::Value = crate::os_pack::json::parse(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let mut expected = fixture["initial"]["layout"].clone();
    let mut session = FlowRetainedVcs::new(crate::os_dsl::FromValue::from_value(crate::os_pack::json::to_dsl_value(&fixture["initial"].clone())).unwrap(), 1, 0, 0);
    for edit in fixture["edits"].as_array().unwrap() {
        let previous = expected.clone();
        let key = edit["id"].as_str().unwrap();
        if edit["layout"].is_null() {
            expected.as_object_mut().unwrap().remove(key);
        } else {
            expected.as_object_mut().unwrap().insert(key.to_owned(), edit["layout"].clone());
        }
        let mut source = FlowVcsSource::new(<FlowLayoutEntry as crate::os_dsl::FromValue>::from_value(crate::os_pack::json::to_dsl_value(&edit.clone())).unwrap());
        let handle = session.begin_set_layout(session.authority(), &mut source).unwrap();
        publish_and_close(&mut session, handle);
        assert_eq!(crate::os_pack::json::from_dsl_value(&crate::os_dsl::ToValue::to_value(&session.document.as_ref().unwrap().fixture().layout)), expected);
        let undo = session.begin_undo(session.authority()).unwrap();
        publish_and_close(&mut session, undo);
        assert_eq!(crate::os_pack::json::from_dsl_value(&crate::os_dsl::ToValue::to_value(&session.document.as_ref().unwrap().fixture().layout)), previous);
        let redo = session.begin_redo(session.authority()).unwrap();
        publish_and_close(&mut session, redo);
        assert_eq!(crate::os_pack::json::from_dsl_value(&crate::os_dsl::ToValue::to_value(&session.document.as_ref().unwrap().fixture().layout)), expected);
        while !session.close_retired_step(retained_grant()).unwrap() {}
    }
    close_layout_session(&mut session);
}

#[test]
fn retained_vcs_ordered_layout_cancel_at_each_unpublished_boundary_retires_exactly() {
    let fixture: crate::os_pack::json::Value = crate::os_pack::json::parse(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for edit in fixture["edits"].as_array().unwrap() {
        for boundary in 0..64 {
            let mut session = FlowRetainedVcs::new(crate::os_dsl::FromValue::from_value(crate::os_pack::json::to_dsl_value(&fixture["initial"].clone())).unwrap(), 1, 0, 0);
            let mut source = FlowVcsSource::new(<FlowLayoutEntry as crate::os_dsl::FromValue>::from_value(crate::os_pack::json::to_dsl_value(&edit.clone())).unwrap());
            let handle = session.begin_set_layout(session.authority(), &mut source).unwrap();
            let mut published = false;
            for _ in 0..boundary {
                if matches!(session.poll(handle, retained_grant()).unwrap(), FlowVcsPoll::Preview { .. }) {
                    published = true;
                    break;
                }
            }
            if published {
                let page = session.take_page(handle).unwrap();
                session.acknowledge_page(handle, page.sequence).unwrap();
            } else {
                session.cancel(handle, retained_grant()).unwrap();
            }
            while !session.close_operation_step(handle, retained_grant()).unwrap() {}
            if !published {
                assert_eq!(crate::os_pack::json::from_dsl_value(&crate::os_dsl::ToValue::to_value(&session.document.as_ref().unwrap().fixture().layout)), fixture["initial"]["layout"], "cancel boundary {boundary}");
                assert_eq!(session.credits(), FlowVcsCredits::default());
            }
            close_layout_session(&mut session);
            if published {
                break;
            }
        }
    }
}
//#endregion 📍️OrderedLayoutLaws

#[test]
fn retained_vcs_repeated_rejection_preserves_source_and_credits_then_valid_control_progresses() {
    let mut session = FlowRetainedVcs::new(retained_fixture(), 7, 10, 9);
    let before = session.credits();
    let mut too_large = FlowVcsSource::new(Widget::InputNote { id: "large".into(), text: "x".repeat(FLOW_VCS_MAX_BYTES + 1) });
    assert_eq!(session.begin_add_widget(session.authority(), 2, &mut too_large), Err(FlowVcsFault::Limit));
    assert_eq!(session.begin_add_widget(session.authority(), 2, &mut too_large), Err(FlowVcsFault::Limit));
    assert!(too_large.retained());
    assert_eq!(session.credits(), before);

    let mut valid = FlowVcsSource::new(Widget::InputNote { id: "valid".into(), text: "next".into() });
    let handle = session.begin_add_widget(session.authority(), 2, &mut valid).expect("valid request follows repeated rejection");
    assert!(!valid.retained());
    assert!(matches!(drive_to_preview(&mut session, handle), FlowVcsPoll::Preview { .. }));
}

#[test]
fn retained_vcs_stale_aba_cancel_ack_and_incremental_close_are_fail_closed() {
    let mut session = FlowRetainedVcs::new(retained_fixture(), 3, 2, 1);
    session.bind_surface(41, 73, 5).expect("surface owner");
    let before_digest = flow_vcs_fixture_scalar_digest(session.document.as_ref().expect("document").fixture());
    let stale = FlowVcsAuthority { base_revision: 1, ..session.authority() };
    let mut source = FlowVcsSource::new("source".to_owned());
    let handle = session.begin_remove_widget(stale, &mut source).expect("stale work may be admitted but not published");
    session.poll(handle, retained_grant()).expect("progress");
    session.poll(handle, retained_grant()).expect("checkpoint");
    let credits = session.credits();
    assert_eq!(session.poll(handle, retained_grant()), Err(FlowVcsFault::StaleAuthority));
    assert_eq!(session.poll(handle, retained_grant()), Err(FlowVcsFault::StaleAuthority));
    assert_eq!(session.credits(), credits);
    assert_eq!(flow_vcs_fixture_scalar_digest(session.document.as_ref().expect("document").fixture()), before_digest);
    session.cancel(handle, retained_grant()).expect("valid cancel follows rejection");
    assert_eq!(session.cancel(handle, retained_grant()), Err(FlowVcsFault::DuplicateControl));
    while !session.close_operation_step(handle, retained_grant()).expect("incremental close") {}
    while !session.close_retired_step(retained_grant()).expect("retired source close") {}
    assert_eq!(session.credits(), FlowVcsCredits::default());
    assert_eq!(session.rediscover(handle.operation, handle.generation), Err(FlowVcsFault::StaleHandle));
}

#[test]
fn retained_vcs_all_thirteen_fixture_operations_match_independent_third_party_oracle_after_ack_close() {
    let source = include_str!("../../🧫️fixtures/🔮️oracle/🔣️.json");
    let expected = SerdeJsonFlowOracle.expected_operations(source);
    let independently_evaluated = SerdeJsonFlowOracle.evaluate_operations(source);
    assert_eq!(independently_evaluated, expected);
    assert_eq!(expected.len(), FLOW_VCS_FEATURES.len());

    let root: crate::os_pack::json::Value = crate::os_pack::json::parse(source).expect("retained oracle fixture");
    let initial = <FlowFixture as crate::os_dsl::FromValue>::from_value(crate::os_pack::json::to_dsl_value(&root.get("initial").expect("oracle initial document").clone())).expect("oracle initial Flow fixture");
    let operations = root.get("operations").and_then(crate::os_pack::json::Value::as_array).expect("oracle operation ledger");
    let mut session = FlowRetainedVcs::new(initial, 77, 1, 0);
    let mut actual = Vec::new();
    for (operation, expected_feature) in operations.iter().zip(FLOW_VCS_FEATURES) {
        let feature = operation.get("feature").and_then(crate::os_pack::json::Value::as_str).expect("oracle feature");
        assert_eq!(feature, expected_feature);
        let handle = flow_oracle_begin_operation(&mut session, operation);
        let page = publish_and_close(&mut session, handle);
        actual.push(flow_oracle_actual_case(feature, &session, page));
    }
    assert_eq!(actual, independently_evaluated);
    close_layout_session(&mut session);
}

#[test]
fn retained_vcs_language_neutral_vector_signatures_detect_every_field_and_value_mutation() {
    let oracle: crate::os_pack::json::Value = crate::os_pack::json::parse(include_str!("../../🧫️fixtures/🔮️oracle/🔣️.json")).expect("oracle fixture");
    let lifecycle: crate::os_pack::json::Value = crate::os_pack::json::parse(include_str!("../../🧫️fixtures/🔄️lifecycle/🔣️.json")).expect("lifecycle fixture");
    let owners: crate::os_pack::json::Value = crate::os_pack::json::parse(include_str!("../../🧫️fixtures/🫴️owners/🔣️.json")).expect("owner fixture");
    let operations = oracle.get("operations").and_then(crate::os_pack::json::Value::as_array).expect("operation ledger");
    assert_eq!(operations.len(), FLOW_VCS_FEATURES.len());
    for (operation, feature) in operations.iter().zip(FLOW_VCS_FEATURES) {
        assert_eq!(operation.get("feature").and_then(crate::os_pack::json::Value::as_str), Some(feature));
        let expected = operation.get("expected").expect("expected operation ledger");
        assert!(operation.get("input").is_some());
        assert!(expected.get("document").is_some());
        assert!(expected.get("page").is_some());
        assert!(expected.get("history").is_some());
        assert!(expected.get("handback").is_some());
    }
    let signatures = lifecycle.get("hostileVectorDigests").expect("hostile vector signatures");
    for name in ["byteVectors", "authorityVectors", "malformedVectors", "grantVectors", "transferControlLedger"] {
        let values = lifecycle.get(name).and_then(crate::os_pack::json::Value::as_array).expect("hostile vector collection");
        let expected = signatures.get(name).and_then(crate::os_pack::json::Value::as_array).expect("hostile vector digest collection");
        assert_eq!(values.len(), expected.len());
        for (value, digest) in values.iter().zip(expected) {
            flow_hostile_assert_every_scalar_is_signed(value, digest.as_str().and_then(|value| value.parse().ok()).expect("hostile vector digest"));
        }
    }
    for name in ["fingerprints", "expectedStates", "protocolDocuments"] {
        let values = lifecycle.get(name).and_then(crate::os_pack::json::Value::as_object).expect("hostile vector map");
        let expected = signatures.get(name).and_then(crate::os_pack::json::Value::as_object).expect("hostile vector digest map");
        assert_eq!(values.len(), expected.len());
        for (key, value) in values {
            flow_hostile_assert_every_scalar_is_signed(value, expected.get(key).and_then(crate::os_pack::json::Value::as_str).and_then(|value| value.parse().ok()).expect("hostile map digest"));
        }
    }
    assert_eq!(lifecycle.get("byteVectors").and_then(crate::os_pack::json::Value::as_array).expect("byte vectors").len(), 3);
    assert_eq!(lifecycle.get("authorityVectors").and_then(crate::os_pack::json::Value::as_array).expect("authority vectors").len(), 4);
    assert_eq!(lifecycle.get("malformedVectors").and_then(crate::os_pack::json::Value::as_array).expect("malformed vectors").len(), 3);
    assert_eq!(lifecycle.get("grantVectors").and_then(crate::os_pack::json::Value::as_array).expect("grant vectors").len(), 5);
    let transfers = lifecycle.get("transferControlLedger").and_then(crate::os_pack::json::Value::as_array).expect("transfer ledgers");
    assert_eq!(transfers.len(), 24);
    assert!(transfers.iter().all(|value| value.get("controls").and_then(crate::os_pack::json::Value::as_array).is_some_and(|controls| controls.len() == 2)));
    let rollback = transfers.iter().filter(|value| value.get("protocol").and_then(|protocol| protocol.get("target")).and_then(|target| target.get("rollbackSteps")).is_some()).collect::<Vec<_>>();
    assert_eq!(rollback.len(), 5);
    for value in rollback {
        let controls = value.get("controls").and_then(crate::os_pack::json::Value::as_array).expect("rollback controls");
        assert_eq!(controls[0].get("control").and_then(crate::os_pack::json::Value::as_str), Some("cancel"));
        assert_eq!(controls[1].get("control").and_then(crate::os_pack::json::Value::as_str), Some("fault"));
        assert!(controls.iter().all(|control| control.get("expected").and_then(|expected| expected.get("result")).and_then(crate::os_pack::json::Value::as_str) == Some("ok")));
        assert_eq!(controls[0].get("expected").and_then(|expected| expected.get("atBoundary")).and_then(|boundary| boundary.get("stage")).and_then(crate::os_pack::json::Value::as_str), Some("Cancelled"));
        assert_eq!(controls[1].get("expected").and_then(|expected| expected.get("atBoundary")).and_then(|boundary| boundary.get("stage")).and_then(crate::os_pack::json::Value::as_str), Some("Faulted"));
    }
    assert_eq!(owners.get("fixtureLedgers").and_then(|value| value.get("hostileOmissionLaws")).and_then(crate::os_pack::json::Value::as_array).expect("hostile omission laws").len(), 17);

    let source = include_str!("../../🦀️.rs");
    for required in ["evaluate_operations", "flow_oracle_apply_operation", "flow_oracle_actual_case(feature, &session, page)", "flow_hostile_expected_state", "flow_hostile_actual_state", "flow_hostile_assert_every_scalar_is_signed"] {
        assert!(source.contains(required), "oracle extraction source law lacks {required}");
        assert!(!source.replace(required, "").contains(required), "hostile omission must fail the extraction gate for {required}");
    }
    let forbidden_literals = [["semantic: \"widget", "Count+1\""].concat(), ["semantic: \"inverse", "Published\""].concat(), ["feature_", "cases("].concat()];
    for forbidden in forbidden_literals {
        assert!(!source.contains(&forbidden), "literal oracle label remains reachable: {forbidden}");
    }
}

#[test]
fn retained_vcs_fixture_byte_vectors_execute_exact_multibyte_max_and_max_plus_one_results() {
    let oracle: crate::os_pack::json::Value = crate::os_pack::json::parse(include_str!("../../🧫️fixtures/🔮️oracle/🔣️.json")).expect("oracle fixture");
    let lifecycle: crate::os_pack::json::Value = crate::os_pack::json::parse(include_str!("../../🧫️fixtures/🔄️lifecycle/🔣️.json")).expect("lifecycle fixture");
    for vector in lifecycle.get("byteVectors").and_then(crate::os_pack::json::Value::as_array).expect("byte vectors") {
        let protocol = vector.get("protocol").expect("byte protocol");
        let input = protocol.get("operation").expect("byte operation");
        assert_eq!(input.get("feature").and_then(crate::os_pack::json::Value::as_str), Some("removeWidget"));
        let value = match input.get("encoding").and_then(crate::os_pack::json::Value::as_str).expect("byte encoding") {
            "literal" => input.get("value").and_then(crate::os_pack::json::Value::as_str).expect("byte literal").to_owned(),
            "repeatUtf8" => input.get("unit").and_then(crate::os_pack::json::Value::as_str).expect("byte unit").repeat(flow_oracle_usize(input, "repetitions")),
            encoding => panic!("unsupported byte encoding {encoding}"),
        };
        assert_eq!(value.chars().count(), flow_oracle_usize(input, "characterCount"));
        assert_eq!(value.len(), flow_oracle_usize(input, "byteLength"));
        let mut session = flow_hostile_session(&lifecycle, &oracle, protocol);
        let mut source = FlowVcsSource::new(value);
        let authority = flow_hostile_authority(&session, input);
        let result = session.begin_remove_widget(authority, &mut source);
        let expected = vector.get("expected").expect("byte expected result");
        let expected_result = expected.get("result").and_then(crate::os_pack::json::Value::as_str).expect("byte result");
        match result {
            Ok(handle) => {
                assert_eq!(expected_result, "accepted");
                assert_eq!(handle, flow_hostile_expected_handle(expected.get("expectedHandle").expect("byte expected handle")));
                assert_eq!(source.retained(), expected.get("sourceRetained").and_then(crate::os_pack::json::Value::as_bool).expect("byte retained result"));
                assert_eq!(flow_hostile_actual_state(&session), flow_hostile_expected_state(&lifecycle, &oracle, expected.get("admissionState").and_then(crate::os_pack::json::Value::as_str).expect("byte admission state")));
                let grant_name = protocol.get("cleanup").and_then(|value| value.get("grant")).and_then(crate::os_pack::json::Value::as_str).expect("byte cleanup grant");
                let grant = flow_hostile_named_grant(&lifecycle, grant_name);
                let cleanup = protocol.get("cleanup").and_then(|value| value.get("control")).and_then(crate::os_pack::json::Value::as_str).expect("byte cleanup control");
                assert_eq!(cleanup, "cancel");
                session.cancel(handle, grant).expect("byte vector cleanup cancel");
                flow_hostile_close_and_drain(&mut session, handle, grant);
            }
            Err(fault) => {
                assert_eq!(flow_hostile_fault_name(fault), expected_result);
                assert!(expected.get("expectedHandle").is_some_and(crate::os_pack::json::Value::is_null));
                assert_eq!(source.retained(), expected.get("sourceRetained").and_then(crate::os_pack::json::Value::as_bool).expect("byte retained result"));
            }
        }
        assert_eq!(flow_hostile_actual_state(&session), flow_hostile_expected_state(&lifecycle, &oracle, expected.get("afterCloseState").and_then(crate::os_pack::json::Value::as_str).expect("byte final state")));
    }
}

#[test]
fn retained_vcs_fixture_authority_malformed_and_grant_vectors_execute_exact_results() {
    let oracle: crate::os_pack::json::Value = crate::os_pack::json::parse(include_str!("../../🧫️fixtures/🔮️oracle/🔣️.json")).expect("oracle fixture");
    let lifecycle: crate::os_pack::json::Value = crate::os_pack::json::parse(include_str!("../../🧫️fixtures/🔄️lifecycle/🔣️.json")).expect("lifecycle fixture");
    let valid_grant = flow_hostile_named_grant(&lifecycle, "valid");

    for vector in lifecycle.get("authorityVectors").and_then(crate::os_pack::json::Value::as_array).expect("authority vectors") {
        let protocol = vector.get("protocol").expect("authority protocol");
        let mut session = flow_hostile_session(&lifecycle, &oracle, protocol);
        let operation = protocol.get("operation").expect("authority operation");
        assert_eq!(operation.get("feature").and_then(crate::os_pack::json::Value::as_str), Some("checkpoint"));
        let authority = operation.get("authority").map_or_else(
            || session.authority(),
            |value| FlowVcsAuthority {
                session_generation: u32::try_from(flow_oracle_u64(value, "sessionGeneration")).expect("authority session"),
                base_revision: flow_oracle_u64(value, "baseRevision"),
                parent_revision: flow_oracle_u64(value, "parentRevision"),
            },
        );
        let handle = session.begin_checkpoint(authority).expect("authority checkpoint admission");
        assert_eq!(handle, flow_hostile_expected_handle(protocol.get("expectedAdmittedHandle").expect("authority admitted handle")));
        let call = protocol.get("call").expect("authority call");
        assert_eq!(call.get("method").and_then(crate::os_pack::json::Value::as_str), Some("poll"));
        let grant = flow_hostile_named_grant(&lifecycle, call.get("grant").and_then(crate::os_pack::json::Value::as_str).expect("authority grant"));
        let result = if let Some(polls) = call.get("polls").and_then(crate::os_pack::json::Value::as_u64) {
            for _ in 1..polls {
                session.poll(handle, grant).expect("authority setup poll");
            }
            session.poll(handle, grant)
        } else {
            let forged = call.get("handle").expect("forged handle");
            if let Some(prior) = call.get("priorGeneration").and_then(crate::os_pack::json::Value::as_u64) {
                assert_eq!(prior, u64::from(handle.generation));
            }
            session.poll(
                FlowVcsHandle {
                    operation: flow_oracle_u64(forged, "operation"),
                    slot: u8::try_from(flow_oracle_u64(forged, "slot")).expect("forged slot"),
                    generation: u32::try_from(flow_oracle_u64(forged, "generation")).expect("forged generation"),
                },
                grant,
            )
        };
        let expected = vector.get("expected").expect("authority expected");
        assert_eq!(flow_hostile_fault_name(result.expect_err("authority rejection")), expected.get("result").and_then(crate::os_pack::json::Value::as_str).expect("authority result"));
        assert_eq!(flow_hostile_actual_state(&session), flow_hostile_expected_state(&lifecycle, &oracle, expected.get("atResultState").and_then(crate::os_pack::json::Value::as_str).expect("authority result state")));
        session.cancel(handle, valid_grant).expect("authority cleanup cancel");
        flow_hostile_close_and_drain(&mut session, handle, valid_grant);
        assert_eq!(flow_hostile_actual_state(&session), flow_hostile_expected_state(&lifecycle, &oracle, expected.get("afterCloseState").and_then(crate::os_pack::json::Value::as_str).expect("authority final state")));
    }

    for vector in lifecycle.get("malformedVectors").and_then(crate::os_pack::json::Value::as_array).expect("malformed vectors") {
        let protocol = vector.get("protocol").expect("malformed protocol");
        let mut session = flow_hostile_session(&lifecycle, &oracle, protocol);
        let operation = protocol.get("operation").expect("malformed operation");
        assert_eq!(operation.get("feature").and_then(crate::os_pack::json::Value::as_str), Some("patchWidget"));
        let id = operation.get("id").expect("malformed id");
        let widget = operation.get("widget").expect("malformed widget");
        let mut id_source =
            FlowVcsSource { value: id.get("present").and_then(crate::os_pack::json::Value::as_bool).filter(|present| *present).map(|_| id.get("value").and_then(crate::os_pack::json::Value::as_str).expect("malformed id value").to_owned()) };
        let mut widget_source = FlowVcsSource {
            value: widget
                .get("present")
                .and_then(crate::os_pack::json::Value::as_bool)
                .filter(|present| *present)
                .map(|_| <Widget as crate::os_dsl::FromValue>::from_value(crate::os_pack::json::to_dsl_value(&widget.get("value").expect("malformed widget value").clone())).expect("malformed widget")),
        };
        let authority = flow_hostile_authority(&session, operation);
        let result = session.begin_patch_widget(authority, &mut id_source, &mut widget_source);
        let expected = vector.get("expected").expect("malformed expected");
        assert_eq!(flow_hostile_fault_name(result.expect_err("malformed rejection")), expected.get("result").and_then(crate::os_pack::json::Value::as_str).expect("malformed result"));
        assert!(expected.get("expectedHandle").is_some_and(crate::os_pack::json::Value::is_null));
        let sources = expected.get("sources").expect("malformed source results");
        assert_eq!(id_source.retained(), sources.get("idRetained").and_then(crate::os_pack::json::Value::as_bool).expect("malformed id retained"));
        assert_eq!(widget_source.retained(), sources.get("widgetRetained").and_then(crate::os_pack::json::Value::as_bool).expect("malformed widget retained"));
        assert_eq!(flow_hostile_actual_state(&session), flow_hostile_expected_state(&lifecycle, &oracle, expected.get("atResultState").and_then(crate::os_pack::json::Value::as_str).expect("malformed result state")));
    }

    for vector in lifecycle.get("grantVectors").and_then(crate::os_pack::json::Value::as_array).expect("grant vectors") {
        let protocol = vector.get("protocol").expect("grant protocol");
        let mut session = flow_hostile_session(&lifecycle, &oracle, protocol);
        assert_eq!(protocol.get("operation").and_then(|value| value.get("feature")).and_then(crate::os_pack::json::Value::as_str), Some("checkpoint"));
        let handle = flow_hostile_begin_operation(&mut session, &lifecycle, &oracle, protocol.get("operation").expect("grant operation"));
        assert_eq!(handle, flow_hostile_expected_handle(protocol.get("expectedHandle").expect("grant expected handle")));
        let call = protocol.get("call").expect("grant call");
        assert_eq!(call.get("method").and_then(crate::os_pack::json::Value::as_str), Some("poll"));
        let result = session.poll(handle, flow_hostile_grant(call.get("grant").expect("grant input")));
        let actual_result = match result {
            Ok(FlowVcsPoll::Progress { .. }) => "progress",
            Ok(_) => "unexpectedPoll",
            Err(fault) => flow_hostile_fault_name(fault),
        };
        let expected = vector.get("expected").expect("grant expected");
        assert_eq!(actual_result, expected.get("result").and_then(crate::os_pack::json::Value::as_str).expect("grant result"));
        assert_eq!(flow_hostile_actual_state(&session), flow_hostile_expected_state(&lifecycle, &oracle, expected.get("atResultState").and_then(crate::os_pack::json::Value::as_str).expect("grant result state")));
        session.cancel(handle, valid_grant).expect("grant cleanup cancel");
        flow_hostile_close_and_drain(&mut session, handle, valid_grant);
        assert_eq!(flow_hostile_actual_state(&session), flow_hostile_expected_state(&lifecycle, &oracle, expected.get("afterCloseState").and_then(crate::os_pack::json::Value::as_str).expect("grant final state")));
    }
}

#[test]
fn retained_vcs_fixture_cancel_and_fault_execute_all_twenty_four_exact_transfer_states() {
    let oracle: crate::os_pack::json::Value = crate::os_pack::json::parse(include_str!("../../🧫️fixtures/🔮️oracle/🔣️.json")).expect("oracle fixture");
    let lifecycle: crate::os_pack::json::Value = crate::os_pack::json::parse(include_str!("../../🧫️fixtures/🔄️lifecycle/🔣️.json")).expect("lifecycle fixture");
    for boundary in lifecycle.get("transferControlLedger").and_then(crate::os_pack::json::Value::as_array).expect("transfer control ledger") {
        let protocol = boundary.get("protocol").expect("transfer protocol");
        let target = protocol.get("target").expect("transfer target");
        let grant = flow_hostile_named_grant(&lifecycle, protocol.get("grant").and_then(crate::os_pack::json::Value::as_str).expect("transfer grant"));
        for control in boundary.get("controls").and_then(crate::os_pack::json::Value::as_array).expect("transfer controls") {
            let mut session = flow_hostile_session(&lifecycle, &oracle, protocol);
            flow_hostile_apply_setup(&mut session, protocol.get("setup").expect("transfer setup"));
            let handle = flow_hostile_begin_operation(&mut session, &lifecycle, &oracle, protocol.get("operation").expect("transfer operation"));
            assert_eq!(handle, flow_hostile_expected_handle(protocol.get("expectedHandle").expect("transfer expected handle")));
            session.poll(handle, grant).expect("transfer admission progress");
            session.poll(handle, grant).expect("transfer admission checkpoint");

            let rollback_steps = target.get("rollbackSteps").and_then(crate::os_pack::json::Value::as_u64);
            if rollback_steps.is_some() {
                for _ in 0..2048 {
                    let operation = session.operations[usize::from(handle.slot)].as_ref().expect("transfer operation");
                    if operation.cursor.phase == FlowVcsCursorPhase::PublishPage && operation.cursor.visibility_published {
                        break;
                    }
                    session.poll(handle, grant).expect("reach rollback publication boundary");
                }
            } else {
                for _ in 0..2048 {
                    if flow_hostile_cursor_matches(&session, handle, target) {
                        break;
                    }
                    session.poll(handle, grant).expect("reach transfer cursor target");
                }
                assert!(flow_hostile_cursor_matches(&session, handle, target));
            }

            let before_control = session.resource_fingerprint();
            let control_name = control.get("control").and_then(crate::os_pack::json::Value::as_str).expect("transfer control");
            let result = match control_name {
                "cancel" => session.cancel(handle, grant),
                "fault" => session.fault(handle, grant),
                value => panic!("unsupported transfer control {value}"),
            };
            let expected = control.get("expected").expect("transfer expected");
            assert_eq!(result.map(|_| "ok").unwrap_or_else(flow_hostile_fault_name), expected.get("result").and_then(crate::os_pack::json::Value::as_str).expect("transfer control result"));
            if let Some(steps) = rollback_steps {
                for _ in 0..steps {
                    assert!(!session.close_operation_step(handle, grant).expect("rollback boundary step"));
                }
                assert!(flow_hostile_cursor_matches(&session, handle, target));
                let at_boundary = expected.get("atBoundary").expect("rollback expected boundary");
                flow_hostile_assert_rollback_boundary(&session, handle, protocol.get("operation").expect("rollback operation fixture"), target, at_boundary);
                assert_eq!(
                    flow_hostile_actual_state(&session),
                    flow_hostile_expected_state(&lifecycle, &oracle, at_boundary.get("state").and_then(crate::os_pack::json::Value::as_str).expect("rollback boundary state")),
                    "fixture rollback boundary mismatch at {} via {}",
                    boundary.get("boundary").and_then(crate::os_pack::json::Value::as_str).expect("rollback boundary"),
                    control_name
                );
                let before_repeat = session.resource_fingerprint();
                let repeat = match control_name {
                    "cancel" => session.cancel(handle, grant),
                    "fault" => session.fault(handle, grant),
                    value => panic!("unsupported repeated transfer control {value}"),
                };
                assert_eq!(repeat.map(|_| "ok").unwrap_or_else(flow_hostile_fault_name), expected.get("repeatResult").and_then(crate::os_pack::json::Value::as_str).expect("rollback repeat result"));
                assert_eq!(session.resource_fingerprint(), before_repeat);
            } else if expected.get("result").and_then(crate::os_pack::json::Value::as_str) == Some("duplicateControl") {
                assert_eq!(session.resource_fingerprint(), before_control);
            }
            flow_hostile_close_and_drain(&mut session, handle, grant);
            assert_eq!(
                flow_hostile_actual_state(&session),
                flow_hostile_expected_state(&lifecycle, &oracle, expected.get("finalState").and_then(crate::os_pack::json::Value::as_str).expect("transfer final state")),
                "fixture transfer result mismatch at {} via {}",
                boundary.get("boundary").and_then(crate::os_pack::json::Value::as_str).expect("transfer boundary"),
                control_name
            );
        }
    }
}

#[test]
fn retained_vcs_zero_fuel_deadline_and_interrupted_close_preserve_every_credit() {
    let mut session = FlowRetainedVcs::new(retained_fixture(), 13, 1, 0);
    let handle = session.begin_checkpoint(session.authority()).expect("checkpoint");
    let before = session.credits();
    let mut rejected = retained_grant();
    rejected.fuel = 0;
    assert_eq!(session.poll(handle, rejected), Err(FlowVcsFault::InsufficientGrant));
    rejected = retained_grant();
    rejected.interrupted = true;
    assert_eq!(session.poll(handle, rejected), Err(FlowVcsFault::InsufficientGrant));
    rejected = retained_grant();
    rejected.deadline_milliseconds = rejected.now_milliseconds;
    assert_eq!(session.poll(handle, rejected), Err(FlowVcsFault::InsufficientGrant));
    assert_eq!(session.credits(), before);
    session.cancel(handle, retained_grant()).expect("cancel checkpoint");
    rejected = retained_grant();
    rejected.items = 0;
    assert_eq!(session.close_operation_step(handle, rejected), Err(FlowVcsFault::InsufficientGrant));
    assert_eq!(session.credits(), before);
}

#[test]
fn retained_vcs_every_mutating_control_rejects_partial_grants_without_state_change() {
    let mut session = FlowRetainedVcs::new(retained_fixture(), 15, 1, 0);
    let handle = session.begin_checkpoint(session.authority()).expect("checkpoint");
    for grant in rejected_control_grants() {
        let before = session.resource_fingerprint();
        assert_eq!(session.cancel(handle, grant), Err(FlowVcsFault::InsufficientGrant));
        assert_eq!(session.fault(handle, grant), Err(FlowVcsFault::InsufficientGrant));
        assert_eq!(session.panic_fault(handle, grant), Err(FlowVcsFault::InsufficientGrant));
        assert_eq!(session.resource_fingerprint(), before);
    }
    session.cancel(handle, retained_grant()).expect("valid cancel");
    for grant in rejected_control_grants() {
        let before = session.resource_fingerprint();
        assert_eq!(session.close_operation_step(handle, grant), Err(FlowVcsFault::InsufficientGrant));
        assert_eq!(session.resource_fingerprint(), before);
    }
    while !session.close_operation_step(handle, retained_grant()).expect("valid operation close") {}
    for grant in rejected_control_grants() {
        let before = session.resource_fingerprint();
        assert_eq!(session.close_retired_step(grant), Err(FlowVcsFault::InsufficientGrant));
        assert_eq!(session.resource_fingerprint(), before);
    }
}

#[test]
fn retained_vcs_256_plus_one_and_terminal_empty_laws_hold() {
    let mut fixed = FlowFixedOwners::<FlowVcsAction, FLOW_VCS_MAX_ITEMS>::new();
    for _ in 0..FLOW_VCS_MAX_ITEMS {
        assert!(fixed.push(FlowVcsAction::Checkpoint).is_ok());
    }
    assert!(matches!(fixed.push(FlowVcsAction::Checkpoint), Err(FlowVcsAction::Checkpoint)));
    for remaining in (0..FLOW_VCS_MAX_ITEMS).rev() {
        assert!(matches!(fixed.pop(), Some(FlowVcsAction::Checkpoint)));
        assert_eq!(fixed.len(), remaining);
    }
    assert!(fixed.is_empty());

    let mut session = FlowRetainedVcs::new(retained_fixture(), 17, 1, 0);
    assert_eq!(session.preflight(FlowVcsCensus { items: FLOW_VCS_MAX_ITEMS + 1, bytes: 0, depth: 1 }), Err(FlowVcsFault::Limit));
    assert!(session.preflight(FlowVcsCensus { items: FLOW_VCS_MAX_ITEMS, bytes: 0, depth: 1 }).is_ok());
    assert_eq!(session.credits(), FlowVcsCredits::default());

    let handle = session.begin_checkpoint(session.authority()).expect("checkpoint");
    let rediscovered = session.rediscover(handle.operation, handle.generation).expect("lost handle rediscovery");
    assert_eq!(rediscovered, handle);
    drive_to_preview(&mut session, rediscovered);
    let page = session.take_page(rediscovered).expect("page");
    session.acknowledge_page(rediscovered, page.sequence).expect("ACK");
    while !session.close_operation_step(rediscovered, retained_grant()).expect("operation close") {}
    session.begin_close();
    while !session.close_retired_step(retained_grant()).expect("session close") {}
    assert!(session.terminal_is_empty());
    let terminal = session.resource_fingerprint();
    assert!(session.close_retired_step(retained_grant()).expect("idempotent close"));
    assert!(session.close_retired_step(retained_grant()).expect("repeated idempotent close"));
    assert_eq!(session.resource_fingerprint(), terminal);
}

#[test]
fn retained_vcs_malformed_sources_fail_before_transfer_with_exact_fingerprint() {
    let mut session = FlowRetainedVcs::new(retained_fixture(), 19, 1, 0);
    let before = session.resource_fingerprint();
    let mut wrong_id = FlowVcsSource::new("other".to_owned());
    let mut patch = FlowVcsSource::new(Widget::InputNote { id: "source".into(), text: "patched".into() });
    assert_eq!(session.begin_patch_widget(session.authority(), &mut wrong_id, &mut patch), Err(FlowVcsFault::InvalidMutation));
    assert!(wrong_id.retained() && patch.retained());
    assert_eq!(session.resource_fingerprint(), before);
    assert_eq!(session.begin_patch_widget(session.authority(), &mut wrong_id, &mut patch), Err(FlowVcsFault::InvalidMutation));
    assert_eq!(session.resource_fingerprint(), before);

    let mut valid_id = FlowVcsSource::new("source".to_owned());
    let mut valid_patch = FlowVcsSource::new(Widget::InputNote { id: "source".into(), text: "valid".into() });
    assert!(session.begin_patch_widget(session.authority(), &mut valid_id, &mut valid_patch).is_ok());
}

#[test]
fn retained_vcs_panic_fault_preserves_exact_resources_and_next_close_progresses() {
    let mut session = FlowRetainedVcs::new(retained_fixture(), 23, 1, 0);
    let handle = session.begin_checkpoint(session.authority()).expect("checkpoint");
    session.poll(handle, retained_grant()).expect("progress");
    let before = session.resource_fingerprint();
    session.panic_fault(handle, retained_grant()).expect("panic fault");
    assert_eq!(session.resource_fingerprint(), before);
    assert_eq!(session.panic_fault(handle, retained_grant()), Err(FlowVcsFault::DuplicateControl));
    assert_eq!(session.resource_fingerprint(), before);
    while !session.close_operation_step(handle, retained_grant()).expect("close after panic") {}
}

#[test]
fn retained_vcs_cancel_around_every_transfer_has_exact_resource_fingerprints() {
    for completed_polls in 0..3 {
        let mut session = FlowRetainedVcs::new(retained_fixture(), 29 + completed_polls, 1, 0);
        let handle = session.begin_checkpoint(session.authority()).expect("checkpoint");
        for _ in 0..completed_polls {
            session.poll(handle, retained_grant()).expect("pre-cancel transfer");
        }
        let before = session.resource_fingerprint();
        session.cancel(handle, retained_grant()).expect("cancel before publication");
        assert_eq!(session.resource_fingerprint(), before);
        while !session.close_operation_step(handle, retained_grant()).expect("cancel close") {}
    }

    let mut session = FlowRetainedVcs::new(retained_fixture(), 37, 1, 0);
    let handle = session.begin_checkpoint(session.authority()).expect("checkpoint");
    drive_to_preview(&mut session, handle);
    let before_page = session.resource_fingerprint();
    assert_eq!(session.cancel(handle, retained_grant()), Err(FlowVcsFault::Published));
    assert_eq!(session.resource_fingerprint(), before_page);

    let page = session.take_page(handle).expect("take");
    let after_take = session.resource_fingerprint();
    assert_eq!(session.cancel(handle, retained_grant()), Err(FlowVcsFault::Published));
    assert_eq!(session.resource_fingerprint(), after_take);
    session.resume_page(handle, page.sequence).expect("resume");
    let after_resume = session.resource_fingerprint();
    assert_eq!(session.cancel(handle, retained_grant()), Err(FlowVcsFault::Published));
    assert_eq!(session.resource_fingerprint(), after_resume);
    session.retry_page(handle, page.sequence).expect("retry");
    let after_retry = session.resource_fingerprint();
    assert_eq!(session.cancel(handle, retained_grant()), Err(FlowVcsFault::Published));
    assert_eq!(session.resource_fingerprint(), after_retry);
    session.acknowledge_page(handle, page.sequence).expect("ack");
    let after_ack = session.resource_fingerprint();
    assert_eq!(session.cancel(handle, retained_grant()), Err(FlowVcsFault::Published));
    assert_eq!(session.resource_fingerprint(), after_ack);
}

#[test]
fn retained_vcs_scan_and_shift_advance_only_one_semantic_unit_per_grant() {
    let mut session = FlowRetainedVcs::new(retained_fixture(), 41, 1, 0);
    let mut source = FlowVcsSource::new(Widget::InputNote { id: "cursor-item".into(), text: "bounded".into() });
    let handle = session.begin_add_widget(session.authority(), 0, &mut source).expect("cursor admission");
    session.poll(handle, retained_grant()).expect("progress");
    session.poll(handle, retained_grant()).expect("checkpoint");
    let slot = usize::from(handle.slot);
    assert_eq!(session.operations[slot].as_ref().expect("operation").cursor.scan, 0);
    session.poll(handle, retained_grant()).expect("one scan");
    assert_eq!(session.operations[slot].as_ref().expect("operation").cursor.scan, 1);
    session.poll(handle, retained_grant()).expect("second scan");
    assert_eq!(session.operations[slot].as_ref().expect("operation").cursor.scan, 2);
    drive_to_preview(&mut session, handle);
}

#[test]
fn retained_vcs_cancel_during_adjacent_transfer_rolls_back_exact_document() {
    let mut session = FlowRetainedVcs::new(retained_fixture(), 43, 1, 0);
    let before = session.resource_fingerprint();
    let before_digest = flow_vcs_fixture_scalar_digest(session.document.as_ref().expect("document").fixture());
    let before_ids: Vec<String> = session.document.as_ref().expect("document").fixture().widgets.iter().map(|widget| widget_id_for(widget).to_owned()).collect();
    let mut source = FlowVcsSource::new(Widget::InputNote { id: "rollback-item".into(), text: "owned".into() });
    let handle = session.begin_add_widget(session.authority(), 0, &mut source).expect("cursor admission");
    session.poll(handle, retained_grant()).expect("progress");
    session.poll(handle, retained_grant()).expect("checkpoint");
    while !session.operations[usize::from(handle.slot)].as_ref().expect("operation").cursor.mutated {
        session.poll(handle, retained_grant()).expect("reach first transfer");
    }
    session.poll(handle, retained_grant()).expect("one adjacent swap");
    session.cancel(handle, retained_grant()).expect("cancel between transfers");
    while !session.close_operation_step(handle, retained_grant()).expect("incremental rollback close") {}
    while !session.close_retired_step(retained_grant()).expect("retire cancelled source") {}
    assert_eq!(flow_vcs_fixture_scalar_digest(session.document.as_ref().expect("document").fixture()), before_digest);
    let after_ids: Vec<String> = session.document.as_ref().expect("document").fixture().widgets.iter().map(|widget| widget_id_for(widget).to_owned()).collect();
    assert_eq!(after_ids, before_ids);
    let after = session.resource_fingerprint();
    assert_eq!(after, before);
}

#[test]
fn retained_vcs_replace_document_uses_persistent_owner_transfer_phases() {
    let mut replacement = retained_fixture();
    replacement.widgets.push(Widget::InputNote { id: "replacement-tail".into(), text: "tail".into() });
    let expected_widgets = replacement.widgets.len();
    let mut session = FlowRetainedVcs::new(retained_fixture(), 47, 1, 0);
    let mut source = FlowVcsSource::new(replacement);
    let handle = session.begin_replace_document(session.authority(), &mut source).expect("replace admission");
    session.poll(handle, retained_grant()).expect("progress");
    session.poll(handle, retained_grant()).expect("checkpoint");
    let slot = usize::from(handle.slot);
    session.poll(handle, retained_grant()).expect("reserve empty version");
    assert_eq!(session.document.as_ref().expect("document").versions.len(), 2);
    assert_eq!(session.document.as_ref().expect("document").versions.get(1).expect("candidate").widgets.len(), 0);
    session.poll(handle, retained_grant()).expect("transfer schema only");
    assert_eq!(session.document.as_ref().expect("document").versions.get(1).expect("candidate").widgets.len(), 0);
    drive_to_preview(&mut session, handle);
    assert_eq!(session.document.as_ref().expect("document").fixture().widgets.len(), expected_widgets);
    assert_eq!(session.operations[slot].as_ref().expect("operation").stage, FlowVcsStage::PageReady);
}

#[test]
fn retained_vcs_cancel_restores_every_partially_retired_redo_owner() {
    let mut session = FlowRetainedVcs::new(retained_fixture(), 49, 1, 0);
    for _ in 0..4 {
        session.redo.push(FlowVcsAction::Checkpoint).expect("fixed redo owner");
    }
    let before = session.resource_fingerprint();
    let mut source = FlowVcsSource::new(FlowLayoutEntry { id: "source".into(), layout: Some(WidgetLayout { x: 22.0, y: 23.0 }) });
    let handle = session.begin_set_layout(session.authority(), &mut source).expect("new branch");
    let slot = usize::from(handle.slot);
    while session.operations[slot].as_ref().expect("operation").cursor.phase != FlowVcsCursorPhase::RetireRedo {
        session.poll(handle, retained_grant()).expect("reach redo retirement");
    }
    for expected in 1..=3 {
        session.poll(handle, retained_grant()).expect("retire one redo owner");
        assert_eq!(session.operations[slot].as_ref().expect("operation").cursor.redo_retired, expected);
    }
    session.cancel(handle, retained_grant()).expect("cancel after redo transfer");
    while !session.close_operation_step(handle, retained_grant()).expect("restore redo and semantic owner") {}
    while !session.close_retired_step(retained_grant()).expect("retire cancelled request") {}
    assert_eq!(session.resource_fingerprint(), before);
    assert_eq!(session.document.as_ref().expect("document").fixture().layout.get("source"), Some(&WidgetLayout { x: 1.0, y: 2.0 }));
}

#[test]
fn retained_vcs_cancel_restores_each_split_publication_boundary() {
    for target in [FlowVcsCursorPhase::TransferSurface, FlowVcsCursorPhase::PublishVisibility, FlowVcsCursorPhase::PublishPage] {
        let mut session = FlowRetainedVcs::new(retained_fixture(), 51, 1, 0);
        session.bind_surface(101, 202, 303).expect("surface owner");
        let before = session.resource_fingerprint();
        let mut source = FlowVcsSource::new(FlowLayoutEntry { id: "source".into(), layout: Some(WidgetLayout { x: 31.0, y: 32.0 }) });
        let handle = session.begin_set_layout(session.authority(), &mut source).expect("publication cursor");
        let slot = usize::from(handle.slot);
        while session.operations[slot].as_ref().expect("operation").cursor.phase != target {
            session.poll(handle, retained_grant()).expect("reach publication boundary");
        }
        if target == FlowVcsCursorPhase::PublishVisibility {
            session.fault(handle, retained_grant()).expect("fault at surface boundary");
        } else {
            session.cancel(handle, retained_grant()).expect("cancel at publication boundary");
        }
        loop {
            let rejected = session.resource_fingerprint();
            assert_eq!(session.cancel(handle, retained_grant()), Err(FlowVcsFault::DuplicateControl));
            assert_eq!(session.resource_fingerprint(), rejected);
            if session.close_operation_step(handle, retained_grant()).expect("publication rollback") {
                break;
            }
        }
        while !session.close_retired_step(retained_grant()).expect("retire cancelled request") {}
        assert_eq!(session.resource_fingerprint(), before);
        assert_eq!(session.document.as_ref().expect("document").fixture().layout.get("source"), Some(&WidgetLayout { x: 1.0, y: 2.0 }));
    }
}

#[test]
fn retained_vcs_cancel_restores_each_document_replacement_boundary() {
    let targets = [
        FlowVcsCursorPhase::ReserveReplacement,
        FlowVcsCursorPhase::ReplaceSchema,
        FlowVcsCursorPhase::ReplaceCameraX,
        FlowVcsCursorPhase::ReplaceCameraY,
        FlowVcsCursorPhase::ReplaceCameraZoom,
        FlowVcsCursorPhase::ReplaceWidgets,
        FlowVcsCursorPhase::ReverseWidgets,
        FlowVcsCursorPhase::ReplaceSynapses,
        FlowVcsCursorPhase::ReverseSynapses,
        FlowVcsCursorPhase::ReplaceLayout,
        FlowVcsCursorPhase::TransferHistory,
    ];
    for (generation, target) in targets.into_iter().enumerate() {
        let mut session = FlowRetainedVcs::new(retained_fixture(), u32::try_from(61 + generation).expect("generation"), 1, 0);
        let before = session.resource_fingerprint();
        let mut replacement = retained_fixture();
        replacement.widgets.push(Widget::InputNote { id: "replacement-boundary".into(), text: "owner".into() });
        let mut source = FlowVcsSource::new(replacement);
        let handle = session.begin_replace_document(session.authority(), &mut source).expect("replacement cursor");
        let slot = usize::from(handle.slot);
        while session.operations[slot].as_ref().expect("operation").cursor.phase != target {
            session.poll(handle, retained_grant()).expect("reach replacement boundary");
        }
        session.cancel(handle, retained_grant()).expect("cancel replacement boundary");
        loop {
            let rejected = session.resource_fingerprint();
            assert_eq!(session.fault(handle, retained_grant()), Err(FlowVcsFault::DuplicateControl));
            assert_eq!(session.resource_fingerprint(), rejected);
            if session.close_operation_step(handle, retained_grant()).expect("replacement rollback") {
                break;
            }
        }
        while !session.close_retired_step(retained_grant()).expect("replacement retirement") {}
        assert_eq!(session.resource_fingerprint(), before);
    }
}

#[test]
fn retained_vcs_complete_route_rejects_hidden_scans_whole_apply_combined_publish_and_partial_grants() {
    let source = include_str!("../../🦀️.rs");
    let start = source.find("//#region 🌊️RetainedVcs").expect("retained route start");
    let end = source.find("//#endregion 🌊️RetainedVcs").expect("retained route end");
    let cursor = &source[start..end];
    let forbidden = [
        "apply_action",
        "flow_vcs_apply_action",
        "flow_vcs_fixture_digest",
        "flow_vcs_dictionary_census",
        "flow_vcs_tree_census",
        "flow_vcs_flow_ui_census",
        "fn publish_cursor",
        ".widgets.insert(",
        ".widgets.remove(",
        ".synapses.insert(",
        ".synapses.remove(",
        "mem::replace",
        ".iter()",
        ".position(",
        ".find(",
        ".filter(",
        ".fold(",
        ".sum(",
        ".clone()",
        "from_fn",
        "for ",
        "while ",
        "serde_json",
    ];
    for token in forbidden {
        assert!(!cursor.contains(token), "retained cursor admits forbidden whole-action mutation: {token}");
        let mutation = format!("{cursor}\n{token}");
        assert!(mutation.contains(token), "hostile mutation must be observable");
    }
    for required in [
        "flow_vcs_fixture_scalar_digest",
        "flow_vcs_fixture_census",
        "transfer_history_cursor",
        "transfer_surface_cursor",
        "publish_visibility_cursor",
        "publish_page_cursor",
        "history_transferred",
        "surface_transferred",
        "visibility_published",
        "redo_retired > 0",
    ] {
        assert!(cursor.contains(required), "complete retained route lacks bounded contract: {required}");
    }
    assert!(cursor.matches("!grant.permits_work()").count() >= 4, "every mutating control and close must validate the complete grant");
}

fn sample_widget(id: &str) -> Widget {
    Widget::InputNote { id: id.into(), text: format!("note {id}") }
}

fn round_trip(fixture: &FlowFixture, operation: &FlowMutation) -> FlowFixture {
    let forward = operation.diff(fixture).diff().apply(fixture).expect("valid flow diff");
    let inverse = operation.inverse(fixture);
    let mut restored = forward.clone();
    for back in inverse.iter().rev() {
        let next = back.diff(&restored).diff().apply(&restored).expect("valid inverse flow diff");
        restored.retire_cold();
        restored = next;
    }
    assert_eq!(&restored, fixture, "inverse() must exactly restore the pre-operation fixture");
    restored.retire_cold();
    forward
}

#[test]
fn widget_add_patch_remove_round_trip() {
    let fixture = FlowFixture { widgets: Vec::new(), synapses: Vec::new(), ..FlowFixture::default() };
    let add = FlowMutation::AddWidget(AddWidget { index: 0, widget: sample_widget("w1") });
    let with_widget = round_trip(&fixture, &add);
    assert_eq!(with_widget.widgets.len(), 1);

    let patch = FlowMutation::ChangeWidget(ChangeWidget { id: "w1".into(), widget: Widget::InputNote { id: "w1".into(), text: "renamed".into() } });
    let patched = round_trip(&with_widget, &patch);
    assert!(matches!(&patched.widgets[0], Widget::InputNote { text, .. } if text == "renamed"));

    let remove = FlowMutation::RemoveWidget(RemoveWidget { id: "w1".into() });
    let removed = round_trip(&patched, &remove);
    assert!(removed.widgets.is_empty());
}

#[test]
fn set_layout_round_trip() {
    let fixture = FlowFixture::default();
    let operation = FlowMutation::ChangeLayout(ChangeLayout { entries: vec![FlowLayoutEntry { id: "slider".into(), layout: Some(WidgetLayout { x: 12.0, y: 34.0 }) }] });
    let next = round_trip(&fixture, &operation);
    assert_eq!(next.layout.get("slider"), Some(&WidgetLayout { x: 12.0, y: 34.0 }));
}

#[test]
fn flow_fixture_ops_diffs_widgets_synapses_layout() {
    let before = FlowFixture { widgets: vec![sample_widget("a"), sample_widget("b")], synapses: Vec::new(), ..FlowFixture::default() };
    let mut after = before.clone();
    after.widgets.retain(|widget| Identified::id(widget) != "a");
    after.widgets.push(sample_widget("c"));
    after.layout.insert("c".into(), WidgetLayout { x: 1.0, y: 2.0 });
    let operations = flow_fixture_operations(&before, &after).expect("wire-representable flow fixture");
    let materialized = operations.iter().fold(before.clone(), |acc, operation| {
        let next = operation.diff(&acc).diff().apply(&acc).expect("valid flow replay diff");
        acc.retire_cold();
        next
    });
    assert_eq!(materialized.widgets.len(), 2);
    assert!(materialized.widgets.iter().any(|widget| Identified::id(widget) == "c"));
    assert!(materialized.widgets.iter().all(|widget| Identified::id(widget) != "a"));
    assert_eq!(materialized.layout.get("c"), Some(&WidgetLayout { x: 1.0, y: 2.0 }));
}

#[semio_framework_async_macros::async_test]
async fn coalesced_layout_drag_produces_one_edit() {
    let mut store = FlowStore::new(create_document_envelope(FLOW_DOCUMENT_SCHEMA, "flow", empty_flow_snapshot(), None)).await.expect("valid flow store fixture");
    for y in [10.0, 20.0, 30.0] {
        store
            .dispatch(ArtifactCommand::AmendLast {
                mutations: vec![FlowMutation::ChangeLayout(ChangeLayout { entries: vec![FlowLayoutEntry { id: "slider".into(), layout: Some(WidgetLayout { x: 0.0, y }) }] })],
                coalesce_key: Some("move-slider".into()),
            })
            .await
            .expect("drag tick");
    }
    assert_eq!(store.envelope().vcs.edits.len(), 1, "coalesced drag must produce exactly one edit");
    let snapshot = store.snapshot().expect("projection");
    assert_eq!(snapshot.layout.get("slider"), Some(&WidgetLayout { x: 0.0, y: 30.0 }));
    snapshot.retire_cold();
}

/// 📜️ Exercises every `Widget` variant (including `Cluster`'s nested `Tree`/`flow` payload,
/// `Dictionary`-bearing `params`/`preview`, and `BTreeSet` `expanded`) through the `crate::os_dsl::` derive
/// layer — the ground-truth proof for the `🔖️Dsl` region built on top of `FlowFixture`.
#[test]
fn flow_fixture_dsl_round_trips_including_cluster_widget() {
    let mut fixture = FlowFixture::default();
    fixture.widgets.push(Widget::Cluster {
        id: "cluster-1".into(),
        name: "Cluster One".into(),
        tree: Tree {
            neurons: vec![
                Neuron { id: "inner-in".into(), kind: "core.number".into(), params: Dictionary::new().insert("value", NeuralValue::Atom(Atom::Decimal(1.0))), tree: None },
                Neuron {
                    id: "inner-add".into(),
                    kind: "math.add".into(),
                    params: Dictionary::new().insert("count", NeuralValue::Atom(Atom::Integer(2))),
                    tree: Some(Box::new(Tree { neurons: vec![Neuron::with_kind("nested", "core.text", Dictionary::new().insert("value", NeuralValue::Atom(Atom::String("deep".into()))))], synapses: vec![] })),
                },
            ],
            synapses: vec![Synapse { id: "inner-s1".into(), from: "inner-in".into(), to: "inner-add".into(), from_port: "number".into(), to_port: "a".into() }],
        },
        flow: FlowGui { camera: CameraJson { x: 1.0, y: 2.0, zoom: 1.5 }, nodes: crate::OrderedMap::new(), previews: Vec::new() },
    });
    fixture.widgets.push(Widget::OutputPreview { id: "preview2".into(), preview: Dictionary::new().insert("value", NeuralValue::Atom(Atom::Decimal(3.5))), expanded: crate::OrderedSet::from(["a".to_string(), "b".to_string()]) });
    crate::os_store::test_support::assert_dsl_round_trip(&fixture);
    crate::os_store::test_support::assert_dsl_pack_equivalence(&fixture);
}

/// 📜️ Exercises `crate::os_store::OpText` for every `FlowMutation` variant — the ground-truth proof for the
/// mounted transparent direct-leaf aggregate.
#[test]
fn flow_operation_op_text_round_trips_every_variant() {
    crate::os_store::test_support::assert_op_line_round_trip(&FlowMutation::AddWidget(AddWidget { index: 0, widget: sample_widget("w1") }));
    crate::os_store::test_support::assert_op_line_round_trip(&FlowMutation::RemoveWidget(RemoveWidget { id: "w1".into() }));
    crate::os_store::test_support::assert_op_line_round_trip(&FlowMutation::MoveWidget(MoveWidget { id: "w1".into(), to_index: 2 }));
    crate::os_store::test_support::assert_op_line_round_trip(&FlowMutation::ChangeWidget(ChangeWidget { id: "w1".into(), widget: sample_widget("w1") }));
    let synapse = SynapseSpec { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: "x".into(), to_port: "y".into() };
    crate::os_store::test_support::assert_op_line_round_trip(&FlowMutation::AddSynapse(AddSynapse { index: 0, synapse: synapse.clone() }));
    crate::os_store::test_support::assert_op_line_round_trip(&FlowMutation::RemoveSynapse(RemoveSynapse { id: "s1".into() }));
    crate::os_store::test_support::assert_op_line_round_trip(&FlowMutation::MoveSynapse(MoveSynapse { id: "s1".into(), to_index: 1 }));
    crate::os_store::test_support::assert_op_line_round_trip(&FlowMutation::ChangeSynapse(ChangeSynapse { id: "s1".into(), synapse }));
    crate::os_store::test_support::assert_op_line_round_trip(&FlowMutation::ChangeLayout(ChangeLayout { entries: vec![FlowLayoutEntry { id: "w1".into(), layout: Some(WidgetLayout { x: 1.0, y: 2.0 }) }] }));
    crate::os_store::test_support::assert_op_line_round_trip(&FlowMutation::ChangeLayout(ChangeLayout { entries: vec![FlowLayoutEntry { id: "w1".into(), layout: None }] }));
    crate::os_store::test_support::assert_op_line_round_trip(&FlowMutation::ReplaceFlowFixture(ReplaceFlowFixture { fixture: FlowFixture::default() }));
}

/// 📜️ `crate::os_store::test_support::assert_store_roundtrip` over a real `ArtifactStore<FlowFixture,
/// FlowMutation>` — proves the `Mutation`/`MutationDiff` (`🔖️Mutations`) and `OpText`
/// (`🔖️OpText`) layers semio_compose_rs correctly end to end, matching every other converted crate's test.
#[test]
fn flow_fixture_satisfies_vcs_test_support_store_roundtrip() {
    let document = FlowFixture::default();
    let operation = FlowMutation::AddWidget(AddWidget { index: 0, widget: sample_widget("w1") });
    crate::os_store::test_support::assert_store_roundtrip(document, operation);
}

/// 🪪️ The framework fixture owns the canonical `flow.flow` Semio envelope and its default
/// binary pack must round-trip before any Flow-backed app can open its initial store.
#[test]
fn flow_fixture_default_pack_uses_canonical_envelope_and_round_trips() {
    let fixture = FlowFixture::default();
    assert_eq!(<FlowFixture as crate::os_store::ArtifactDsl>::envelope_id(), "flow.flow");
    let encoded = <FlowFixture as crate::os_store::ArtifactPack>::encode_pack_with(&fixture, &crate::os_store::PackEncodeOptions::default()).expect("default flow fixture pack");
    let decoded = <FlowFixture as crate::os_store::ArtifactPack>::decode_pack_with(&encoded, &crate::os_store::PackDecodeOptions::default()).expect("default flow fixture unpack");
    assert_eq!(decoded, fixture);
}

/// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): `FlowMutation`
/// implements `crate::os_spr::OpBinary` through the transparent direct-leaf aggregate and
/// its generic variant codec, so this covers the command envelope without adding another
/// codec.
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    let envelope = create_document_envelope("test/v1", "test", FlowFixture::default(), None);
    let mut store = ArtifactStore::new(envelope).await.expect("valid artifact store fixture");
    let operation = FlowMutation::AddWidget(AddWidget { index: 0, widget: sample_widget("w1") });
    store.dispatch(ArtifactCommand::Apply { mutations: vec![operation], description: None }).await.expect("apply");
    let envelope = store.envelope();
    let edit: &Edit<FlowMutation> = envelope.vcs.edits.last().expect("dispatch must have recorded an edit");
    crate::os_store::test_support::assert_command_envelope_round_trip::<FlowFixture, FlowMutation>(edit, &ArtifactId(envelope.id.clone()), &SchemaId(envelope.schema.clone()));
}

/// 📜️ The handcrafted default Flow DSL preserves typed slider content, both synapses, and canonical pack parity.
#[test]
fn default_flow_example_dsl_round_trips() {
    let text = include_str!("../../../📚️examples/🗣️.dsl.semio");
    let fixture = <FlowFixture as crate::os_store::ArtifactDsl>::parse_dsl(text).expect("🌊️default.flow must parse");
    crate::os_store::test_support::assert_dsl_round_trip(&fixture);
    crate::os_store::test_support::assert_dsl_pack_equivalence(&fixture);
    fixture.retire_cold();
}
