use semio_framework_os_mcp::inference::{
    decode_inference_reply, hub_inference_approval_undo_path, read_hub_inference_job_events, undo_hub_inference_approval,
    GisMapInferencePreviewV1, HubInferenceApprovalRequestV1, HubInferenceEventPageV1, InferenceHubRequestV1,
    InferenceHubResponseV1, InferenceHubTransport, InferenceHubTransportErrorV1, InferenceRouteErrorV1,
    GIS_MAP_INFERENCE_PREVIEW_RING_POINTS, HUB_INFERENCE_APPROVAL_UNDO_RECEIPT_SCHEMA, HUB_INFERENCE_EVENTS_SCHEMA,
    INFERENCE_EVENT_PAGE_MAX_ITEMS, INFERENCE_JOB_MAX_LIFETIME_MS, INFERENCE_PROGRESS_MAX_CURSOR, INFERENCE_REQUEST_MAX_BYTES,
    INFERENCE_ROUTE_ERRORS,
};
use semio_framework_os_mcp::schema::hub_inference_approval_request_schema;
use semio_framework_async::{CancelToken, OperationContext, TraceId};
use directory::os_directory::DocumentScope;
use semio_framework_schema::OwnedJsonSchemaValidator;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../🧫️fixtures/🗳️gis-map-proposal-approval-v1/🔣️.json")).expect("GIS proposal fixture")
}

fn sample_job_id() -> String {
    fixture()["sampleJobId"].as_str().unwrap().to_string()
}

fn sample_proposal_hash() -> String {
    fixture()["proposalHash"].as_str().unwrap().to_string()
}

fn context(cancel: &CancelToken) -> OperationContext {
    OperationContext { actor: 1, generation: 0, trace: TraceId(1), lane: 1, deadline_ms: Some(u64::MAX), cancel: cancel.child_now(), capability: None }
}

struct ScriptedTransport {
    response: InferenceHubResponseV1,
    seen: std::sync::Mutex<Vec<InferenceHubRequestV1>>,
}

impl ScriptedTransport {
    fn ok(status: u16, body: serde_json::Value) -> Self {
        Self { response: InferenceHubResponseV1 { status, body: serde_json::to_vec(&body).unwrap() }, seen: std::sync::Mutex::new(Vec::new()) }
    }

    fn requests(&self) -> Vec<InferenceHubRequestV1> {
        self.seen.lock().unwrap().clone()
    }
}

impl InferenceHubTransport for ScriptedTransport {
    async fn request(&self, _context: &OperationContext, request: &InferenceHubRequestV1) -> Result<InferenceHubResponseV1, InferenceHubTransportErrorV1> {
        self.seen.lock().unwrap().push(request.clone());
        Ok(self.response.clone())
    }
}

fn block_on<F: std::future::Future>(future: F) -> F::Output {
    tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(future)
}

fn checked_page(page: HubInferenceEventPageV1, job_id: &str) -> Result<HubInferenceEventPageV1, InferenceRouteErrorV1> {
    let transport = ScriptedTransport::ok(200, serde_json::to_value(page).unwrap());
    let cancel = CancelToken::root_now();
    block_on(read_hub_inference_job_events(&transport, &context(&cancel), "https://hub.invalid", &DocumentScope::new("space:alpha", "doc:tokyo"), "inference/gis-map", job_id, 0))
}

#[test]
fn the_published_error_vocabulary_is_exactly_the_neutral_fixtures_and_status_alone_is_ambiguous() {
    let fixture = fixture();
    let rows = fixture["errors"].as_array().expect("error vocabulary");
    assert_eq!(rows.len(), INFERENCE_ROUTE_ERRORS.len(), "the closed vocabulary drifted from the neutral corpus");
    for row in rows {
        let code = row["code"].as_str().expect("code");
        let status = row["status"].as_u64().expect("status");
        let published = InferenceRouteErrorV1::from_code(code).unwrap_or_else(|| panic!("{code} is not a published inference route code"));
        assert_eq!(u64::from(published.status()), status, "{code}");
    }
    for published in INFERENCE_ROUTE_ERRORS {
        assert!(rows.iter().any(|row| row["code"] == published.code()), "{} is published but not pinned by the corpus", published.code());
    }
    assert_eq!(InferenceRouteErrorV1::from_status(503), InferenceRouteErrorV1::Unavailable, "503 is shared by three codes, so the widest honest member is the only safe fallback");
    assert_eq!(InferenceRouteErrorV1::from_status(409), InferenceRouteErrorV1::Conflict);
    assert_eq!(InferenceRouteErrorV1::from_status(418), InferenceRouteErrorV1::Unavailable);
}

#[test]
fn the_client_mirrors_the_neutral_fixtures_exact_fixed_limits() {
    let fixture = fixture();
    let limits = &fixture["limits"];
    assert_eq!(limits["requestMaxBytes"], INFERENCE_REQUEST_MAX_BYTES as u64);
    assert_eq!(limits["jobMaxLifetimeMs"], INFERENCE_JOB_MAX_LIFETIME_MS);
    assert_eq!(limits["progressMaxCursor"], INFERENCE_PROGRESS_MAX_CURSOR);
    assert_eq!(limits["eventPageMaxItems"], INFERENCE_EVENT_PAGE_MAX_ITEMS as u64);
    assert_eq!(fixture["binding"]["serviceId"], "s.gis.gismap.inference");
    assert_eq!(fixture["binding"]["artifactSchema"], "gis.map");
    assert_eq!(fixture["binding"]["artifactKind"], "s.gis.gismap");
}

#[test]
fn an_offered_page_carries_the_corpus_preview_and_a_forged_or_open_ring_is_refused() {
    let fixture = fixture();
    let job = sample_job_id();
    let preview: GisMapInferencePreviewV1 = serde_json::from_value(fixture["preview"].clone()).expect("the corpus preview decodes closed");
    assert_eq!(preview.validate(&job), Ok(()));
    assert_eq!(preview.job_id, job);
    assert_eq!(preview.region_id, format!("inference-{job}"));
    assert_eq!(preview.proposal_hash, sample_proposal_hash());
    assert_eq!(preview.ring[0], preview.ring[GIS_MAP_INFERENCE_PREVIEW_RING_POINTS - 1], "the published ring is closed");
    let bounds = &fixture["base"]["expectedInference"]["bounds"];
    let (lon_min, lat_min) = (bounds["lonMin"].as_f64().expect("lonMin"), bounds["latMin"].as_f64().expect("latMin"));
    let (lon_max, lat_max) = (bounds["lonMax"].as_f64().expect("lonMax"), bounds["latMax"].as_f64().expect("latMax"));
    assert_eq!(preview.ring, [[lon_min, lat_min], [lon_max, lat_min], [lon_max, lat_max], [lon_min, lat_max], [lon_min, lat_min]], "the preview does not fold to the corpus's own bounds");

    let mut foreign_job = preview.clone();
    foreign_job.job_id = "2".repeat(32);
    assert_eq!(foreign_job.validate(&job), Err(InferenceRouteErrorV1::Invalid));
    let mut forged_region = preview.clone();
    forged_region.region_id = "inference-forged".into();
    assert_eq!(forged_region.validate(&job), Err(InferenceRouteErrorV1::Invalid));
    let mut wrong_schema = preview.clone();
    wrong_schema.schema = HUB_INFERENCE_EVENTS_SCHEMA.into();
    assert_eq!(wrong_schema.validate(&job), Err(InferenceRouteErrorV1::Invalid));
    let mut open_ring = preview.clone();
    open_ring.ring[4] = [lon_max, lat_max];
    assert_eq!(open_ring.validate(&job), Err(InferenceRouteErrorV1::Conflict), "an unclosed ring is never rendered");
    let mut inverted = preview.clone();
    inverted.ring = [[lon_max, lat_max], [lon_min, lat_max], [lon_min, lat_min], [lon_max, lat_min], [lon_max, lat_max]];
    assert_eq!(inverted.validate(&job), Err(InferenceRouteErrorV1::Conflict));

    let page = serde_json::json!({
        "schema": HUB_INFERENCE_EVENTS_SCHEMA,
        "jobId": job,
        "state": "succeeded",
        "proposalState": "offered",
        "cancelRequested": false,
        "stale": false,
        "proposalHash": sample_proposal_hash(),
        "preview": fixture["preview"],
        "events": [],
        "progress": [],
        "nextCursor": 0,
    });
    let decoded: HubInferenceEventPageV1 = decode_inference_reply(&InferenceHubResponseV1 { status: 200, body: serde_json::to_vec(&page).expect("body") }).expect("an offered page decodes");
    assert_eq!(checked_page(decoded, &job).expect("the checked page keeps its verified preview").preview, Some(preview));

    let mut mismatched = page.clone();
    mismatched["proposalHash"] = serde_json::json!("0".repeat(64));
    let decoded: HubInferenceEventPageV1 = decode_inference_reply(&InferenceHubResponseV1 { status: 200, body: serde_json::to_vec(&mismatched).expect("body") }).expect("it still decodes");
    assert_eq!(checked_page(decoded, &job).unwrap_err(), InferenceRouteErrorV1::Conflict, "a preview whose digest disagrees with the page is never handed on");

    let mut foreign_page = page.clone();
    foreign_page["jobId"] = serde_json::json!("3".repeat(32));
    let decoded: HubInferenceEventPageV1 = decode_inference_reply(&InferenceHubResponseV1 { status: 200, body: serde_json::to_vec(&foreign_page).expect("body") }).expect("it still decodes");
    assert_eq!(checked_page(decoded, &job).unwrap_err(), InferenceRouteErrorV1::Conflict, "a page for another job is never accepted");

    let cancelled = serde_json::json!({
        "schema": HUB_INFERENCE_EVENTS_SCHEMA,
        "jobId": job,
        "state": "cancelled",
        "proposalState": "cancelled",
        "cancelRequested": true,
        "stale": false,
        "proposalHash": serde_json::Value::Null,
        "events": [],
        "progress": [],
        "nextCursor": 0,
    });
    let decoded: HubInferenceEventPageV1 = decode_inference_reply(&InferenceHubResponseV1 { status: 200, body: serde_json::to_vec(&cancelled).expect("body") }).expect("an omitted preview is absent, not an error");
    assert_eq!(decoded.preview, None);
}

#[test]
fn the_neutral_lifecycles_decode_into_the_closed_event_page_in_order() {
    let fixture = fixture();
    for (name, trace) in [("lifecycle", &fixture["lifecycle"]), ("cancelLifecycle", &fixture["cancelLifecycle"])] {
        let rows = trace.as_array().expect("trace");
        assert!(rows.len() <= INFERENCE_EVENT_PAGE_MAX_ITEMS, "{name} exceeds one bounded page");
        let events: Vec<serde_json::Value> = rows.iter().map(|row| serde_json::json!({ "ordinal": row["ordinal"], "kind": row["kind"], "atMs": 1_000 })).collect();
        let body = serde_json::json!({
            "schema": HUB_INFERENCE_EVENTS_SCHEMA,
            "jobId": sample_job_id(),
            "state": if name == "lifecycle" { "succeeded" } else { "cancelled" },
            "proposalState": if name == "lifecycle" { "approved" } else { "cancelled" },
            "cancelRequested": name != "lifecycle",
            "stale": false,
            "proposalHash": sample_proposal_hash(),
            "events": events,
            "progress": [],
            "nextCursor": 0,
        });
        let page: HubInferenceEventPageV1 = decode_inference_reply(&InferenceHubResponseV1 { status: 200, body: serde_json::to_vec(&body).expect("body") }).unwrap_or_else(|error| panic!("{name} did not decode: {error:?}"));
        assert_eq!(page.events.iter().map(|event| event.ordinal).collect::<Vec<_>>(), (1..=rows.len() as u64).collect::<Vec<_>>());
        assert_eq!(page.events.iter().map(|event| event.kind.clone()).collect::<Vec<_>>(), rows.iter().map(|row| row["kind"].as_str().expect("kind").to_string()).collect::<Vec<_>>());
    }
}

#[test]
fn a_durable_approval_undo_posts_only_the_hub_target_frontier_and_retry_identity() {
    let undo_fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/↩️gis-map-approval-undo-v1/🔣️.json")).expect("undo fixture");
    let request: directory::os_directory::GisMapApprovalUndoRequestV1 = serde_json::from_value(undo_fixture["request"].clone()).expect("closed request");
    let scope = DocumentScope::new("space-a", "map-a");
    let response = serde_json::json!({
        "schema": HUB_INFERENCE_APPROVAL_UNDO_RECEIPT_SCHEMA,
        "targetId": request.target_id.clone(),
        "originalJobId": undo_fixture["target"]["originalJobId"],
        "mutationId": "aa".repeat(16),
        "commandHash": "bb".repeat(32),
        "applied": true,
        "replayed": false,
        "frontier": {
            "documentId": "map-a",
            "headEditOrdinal": 5,
            "headEditId": "aa".repeat(16),
            "lastCommitSeq": 5,
            "chainSha256": "cc".repeat(32),
        },
    });
    let transport = ScriptedTransport::ok(200, response);
    let cancel = CancelToken::root_now();
    let receipt = block_on(undo_hub_inference_approval(&transport, &context(&cancel), "https://hub.invalid", &scope, "inference/gis-map", &request)).expect("typed durable receipt");
    assert_eq!(receipt.target_id, request.target_id);
    let seen = transport.requests();
    assert_eq!(seen.len(), 1);
    assert_eq!(seen[0].path, hub_inference_approval_undo_path(&scope, "inference/gis-map"));
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&seen[0].body).expect("request json"), undo_fixture["request"]);
    assert!(!String::from_utf8_lossy(&seen[0].body).contains("inverse"));
}

#[test]
fn the_hub_approval_request_consumes_the_framework_contract() {
    let hub: serde_json::Value = serde_json::from_str(include_str!("../../🧬️schema/🔣️.json")).expect("hub module schema parses");
    let authority = hub["$defs"].get("InferenceApprovalRequestV1").expect("hub publishes InferenceApprovalRequestV1");
    let authority = inline_local_refs(authority, &hub);
    let mirror = hub_inference_approval_request_schema();
    for key in ["type", "additionalProperties", "required", "properties"] {
        assert_eq!(&mirror[key], &authority[key], "the os.mcp approval mirror drifted from hub on `{key}`");
    }
    let approval = HubInferenceApprovalRequestV1::new("00112233445566778899aabbccddeeff", &"ab".repeat(32));
    let owned = OwnedJsonSchemaValidator::compile(&mirror.to_string()).expect("the contract compiles");
    owned.validate_json(&serde_json::to_string(&approval).expect("approval serializes")).expect("the Rust type consumes the approval contract");
}

/// 🔗️ Replaces every `{"$ref": "#/$defs/X"}` with the document's own `X`, so a mirror that inlines
/// a pattern can be compared with an authority that names it.
fn inline_local_refs(value: &serde_json::Value, document: &serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(map) => match map.get("$ref").and_then(serde_json::Value::as_str).and_then(|reference| reference.strip_prefix("#/$defs/")) {
            Some(name) => inline_local_refs(&document["$defs"][name], document),
            None => serde_json::Value::Object(map.iter().map(|(key, entry)| (key.clone(), inline_local_refs(entry, document))).collect()),
        },
        serde_json::Value::Array(items) => serde_json::Value::Array(items.iter().map(|item| inline_local_refs(item, document)).collect()),
        other => other.clone(),
    }
}
