/// 🧾️ Oracle records and the serde JSON oracle: the oracle page, history, handback, case and hostile state records, the semantic oracle trait and its serde JSON implementation.

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
        let root: semio_framework_pack_json::Value = semio_framework_pack_json::parse(source, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("test-only Flow oracle fixture");
        let mut document = root.get("initial").expect("oracle initial document").clone();
        let mut undo: Vec<(semio_framework_pack_json::Value, usize)> = Vec::new();
        let mut redo: Vec<(semio_framework_pack_json::Value, usize)> = Vec::new();
        let mut versions = 1usize;
        let mut active = 0usize;
        let mut revision = 1u64;
        let mut document_generation = 1u64;
        let mut semantic_digest = flow_oracle_scalar_digest(&document);
        let mut results = Vec::new();
        let operations = root.get("operations").and_then(semio_framework_pack_json::Value::as_array).expect("oracle operations");
        for (index, operation) in operations.iter().enumerate() {
            let feature = operation.get("feature").and_then(semio_framework_pack_json::Value::as_str).expect("oracle feature");
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
                "replaceDocument" => {
                    assert!(undo.is_empty() && redo.is_empty(), "oracle document load needs a pristine history");
                    flow_oracle_apply_operation(feature, input, &mut document);
                    versions += 1;
                    active = versions - 1;
                }
                _ => {
                    undo.push((document.clone(), active));
                    redo.clear();
                    flow_oracle_apply_operation(feature, input, &mut document);
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
            let fingerprint_name = operation.get("expected").and_then(|value| value.get("handback")).and_then(|value| value.get("fingerprint")).and_then(semio_framework_pack_json::Value::as_str).expect("oracle fingerprint reference");
            let fingerprint = root.get("terminalFingerprints").and_then(|value| value.get(fingerprint_name)).expect("oracle terminal fingerprint");
            results.push(FlowOracleCase { feature: feature.to_owned(), document: flow_oracle_canonical_json(&document), handback: flow_oracle_expected_handback(fingerprint, &page, &history, versions, active), page, history });
        }
        results
    }

    fn expected_operations(&self, source: &str) -> Vec<FlowOracleCase> {
        let root: semio_framework_pack_json::Value = semio_framework_pack_json::parse(source, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("test-only Flow oracle fixture");
        let documents = root.get("documents").and_then(semio_framework_pack_json::Value::as_object).expect("oracle document ledger");
        root.get("operations")
            .and_then(semio_framework_pack_json::Value::as_array)
            .expect("oracle operations")
            .iter()
            .map(|operation| {
                let feature = operation.get("feature").and_then(semio_framework_pack_json::Value::as_str).expect("oracle feature").to_owned();
                let expected = operation.get("expected").expect("oracle expected result");
                let document_name = expected.get("document").and_then(semio_framework_pack_json::Value::as_str).expect("oracle expected document");
                let page = flow_oracle_expected_page(expected.get("page").expect("oracle expected page"));
                let history = flow_oracle_expected_history(expected.get("history").expect("oracle expected history"));
                let handback = expected.get("handback").expect("oracle expected handback");
                let versions = flow_oracle_usize(handback, "documentVersions");
                let active = flow_oracle_usize(handback, "activeDocumentVersion");
                let fingerprint_name = handback.get("fingerprint").and_then(semio_framework_pack_json::Value::as_str).expect("oracle fingerprint reference");
                let fingerprint = root.get("terminalFingerprints").and_then(|value| value.get(fingerprint_name)).expect("oracle terminal fingerprint");
                FlowOracleCase { feature, document: flow_oracle_canonical_json(documents.get(document_name).expect("oracle document reference")), handback: flow_oracle_expected_handback(fingerprint, &page, &history, versions, active), page, history }
            })
            .collect()
    }
}
