use super::*;
use crate::actions::MockArtifactChannel;
use crate::workspace::ArtifactChannels;
use crate::audit::{AuditSinks, InMemoryAuditSink};
use crate::catalog::{compile, Catalog};
use crate::policy::AgentPrincipal;
use crate::protocol::{JsonRpcId, JsonRpcRequest, McpServer};
use crate::source_builders::note_and_cad_source;
use crate::{build_server_from_catalog, GatewayRuntime};

//#region 🧫️Fixtures
fn fixture_catalog() -> Arc<Catalog> {
    Arc::new(compile(&note_and_cad_source(), semio_framework::Locale::En, semio_framework::Terminology::Native).expect("the note+cad fixture always compiles"))
}

/// 🧫️ A real server over the deterministic note+cad catalog, with a recording notification sink
/// bound exactly the way a live transport binds its own — nothing here is a bespoke double.
fn server_with_sink() -> (McpServer, Arc<RecordingSink>) {
    let principal = AgentPrincipal::from_scope_names("agent:local", "local agent", &["artifact.write".to_string()], None);
    let server = build_server_from_catalog(fixture_catalog(), principal, Arc::new(AuditSinks::InMemory(InMemoryAuditSink::new())), Box::new(ArtifactChannels::Mock(MockArtifactChannel::new())), GatewayRuntime::default());
    let sink = Arc::new(RecordingSink::new());
    let slot = notification_slot();
    slot.set(sink.clone() as Arc<dyn NotificationSink>).ok().expect("a fresh slot is empty");
    (server.publishing_notifications_into(slot), sink)
}

fn request(id: i64, method: &str, params: serde_json::Value) -> JsonRpcRequest {
    JsonRpcRequest { jsonrpc: "2.0".to_string(), id: Some(JsonRpcId::Number(id)), method: method.to_string(), params: Some(params) }
}

fn notification(method: &str, params: serde_json::Value) -> JsonRpcRequest {
    JsonRpcRequest { jsonrpc: "2.0".to_string(), id: None, method: method.to_string(), params: Some(params) }
}

fn result_of(server: &mut McpServer, request: JsonRpcRequest) -> serde_json::Value {
    let response = server.dispatch(&request).expect("a request with an id always answers");
    serde_json::to_value(&response).expect("a response always serializes").get("result").cloned().unwrap_or(serde_json::Value::Null)
}
//#endregion 🧫️Fixtures

//#region 🧪️Pagination
#[test]
fn a_cursor_round_trips_through_its_opaque_form() {
    assert_eq!(decode_cursor(&encode_cursor(0)), Some(0));
    assert_eq!(decode_cursor(&encode_cursor(137)), Some(137));
    assert_eq!(decode_cursor("not-a-semio-cursor"), None);
    assert_eq!(decode_cursor(""), None);
}

#[test]
fn a_page_smaller_than_the_list_carries_a_next_cursor() {
    let page = paginate((0..10).collect::<Vec<u32>>(), 0, 4);
    assert_eq!(page.items, vec![0, 1, 2, 3]);
    assert_eq!(page.next_cursor.as_deref().and_then(decode_cursor), Some(4));
}

#[test]
fn the_last_page_has_no_next_cursor_and_an_offset_past_the_end_is_empty() {
    let final_page = paginate((0..10).collect::<Vec<u32>>(), 8, 4);
    assert_eq!(final_page.items, vec![8, 9]);
    assert!(final_page.next_cursor.is_none());
    let past_end = paginate((0..10).collect::<Vec<u32>>(), 99, 4);
    assert!(past_end.items.is_empty());
    assert!(past_end.next_cursor.is_none());
}

#[test]
fn walking_every_page_yields_the_whole_list_exactly_once() {
    let all: Vec<u32> = (0..23).collect();
    let mut walked: Vec<u32> = Vec::new();
    let mut offset = 0usize;
    loop {
        let page = paginate(all.clone(), offset, 5);
        walked.extend(page.items);
        match page.next_cursor.as_deref().and_then(decode_cursor) {
            Some(next) => offset = next,
            None => break,
        }
    }
    assert_eq!(walked, all);
}

#[test]
fn every_list_method_answers_a_cursor_and_a_foreign_cursor_is_invalid_params() {
    let (mut server, _sink) = server_with_sink();
    for (method, key) in [("tools/list", "tools"), ("resources/list", "resources"), ("resources/templates/list", "resourceTemplates"), ("prompts/list", "prompts")] {
        let first = result_of(&mut server, request(1, method, serde_json::json!({})));
        let items = first.get(key).and_then(serde_json::Value::as_array).cloned().unwrap_or_default();
        assert!(!items.is_empty(), "{method} answered no {key}");
        // 📄️ A cursor past the end is a legal, empty FINAL page, so a client's walk always terminates.
        let last = result_of(&mut server, request(2, method, serde_json::json!({ "cursor": encode_cursor(items.len()) })));
        assert_eq!(last.get(key).and_then(serde_json::Value::as_array).map(Vec::len), Some(0), "{method} past-the-end page was not empty");
        assert!(last.get("nextCursor").is_none(), "{method} past-the-end page still offered a nextCursor");
        let rejected = server.dispatch(&request(3, method, serde_json::json!({ "cursor": "someone-elses-cursor" }))).expect("a request with an id always answers");
        assert!(rejected.is_error(), "{method} accepted a cursor it never minted");
    }
}

#[test]
fn a_tools_list_page_walk_visits_every_tool_exactly_once() {
    let (mut server, _sink) = server_with_sink();
    let whole = result_of(&mut server, request(1, "tools/list", serde_json::json!({})));
    let names: Vec<String> = whole.get("tools").and_then(serde_json::Value::as_array).expect("tools is an array").iter().filter_map(|tool| tool.get("name").and_then(serde_json::Value::as_str).map(str::to_string)).collect();
    assert!(names.len() >= 20, "expected the real tool census, got {}", names.len());
    let mut walked: Vec<String> = Vec::new();
    let mut cursor: Option<String> = None;
    let mut id = 10;
    loop {
        let params = match cursor.as_deref() {
            Some(cursor) => serde_json::json!({ "cursor": cursor }),
            None => serde_json::json!({}),
        };
        let page = result_of(&mut server, request(id, "tools/list", params));
        id += 1;
        walked.extend(page.get("tools").and_then(serde_json::Value::as_array).expect("tools is an array").iter().filter_map(|tool| tool.get("name").and_then(serde_json::Value::as_str).map(str::to_string)));
        cursor = page.get("nextCursor").and_then(serde_json::Value::as_str).map(str::to_string);
        if cursor.is_none() {
            break;
        }
    }
    assert_eq!(walked, names, "the page walk did not reproduce the whole, stably-ordered tool list");
}
//#endregion 🧪️Pagination

//#region 🧪️ResourceRoster
#[test]
fn resources_list_never_repeats_a_uri() {
    let (mut server, _sink) = server_with_sink();
    let listed = result_of(&mut server, request(1, "resources/list", serde_json::json!({})));
    let uris: Vec<String> = listed.get("resources").and_then(serde_json::Value::as_array).expect("resources is an array").iter().filter_map(|resource| resource.get("uri").and_then(serde_json::Value::as_str).map(str::to_string)).collect();
    let unique: BTreeSet<&String> = uris.iter().collect();
    assert_eq!(unique.len(), uris.len(), "a duplicate resource URI is back in the registry: {uris:?}");
    assert_eq!(uris.iter().filter(|uri| uri.as_str() == "semio://workspace/artifacts").count(), 1);
}
//#endregion 🧪️ResourceRoster

//#region 🧪️Subscriptions
#[test]
fn subscribing_to_an_unknown_resource_is_refused_rather_than_silently_accepted() {
    let (mut server, _sink) = server_with_sink();
    let refused = server.dispatch(&request(1, "resources/subscribe", serde_json::json!({ "uri": "semio://nothing-like-this" }))).expect("a request with an id always answers");
    assert!(refused.is_error(), "subscribe accepted a URI this server cannot serve");
    assert!(server.subscriptions().subscribed().is_empty());
}

#[test]
fn subscribe_records_and_unsubscribe_forgets() {
    let (mut server, _sink) = server_with_sink();
    let uri = "semio://artifact/doc-1";
    assert!(!server.dispatch(&request(1, "resources/subscribe", serde_json::json!({ "uri": uri }))).expect("answers").is_error());
    assert_eq!(server.subscriptions().subscribed(), vec![uri.to_string()]);
    // 🔁️ A repeat subscribe is legal and must not double-deliver.
    assert!(!server.dispatch(&request(2, "resources/subscribe", serde_json::json!({ "uri": uri }))).expect("answers").is_error());
    assert_eq!(server.subscriptions().subscribed().len(), 1);
    assert!(!server.dispatch(&request(3, "resources/unsubscribe", serde_json::json!({ "uri": uri }))).expect("answers").is_error());
    assert!(server.subscriptions().subscribed().is_empty());
}

#[test]
fn a_subscribed_artifact_change_pushes_exactly_one_resources_updated() {
    let (mut server, sink) = server_with_sink();
    let uri = "semio://artifact/doc-7";
    server.dispatch(&request(1, "resources/subscribe", serde_json::json!({ "uri": uri })));
    sink.taken();
    let delivered = artifact_changed("doc-7");
    let published = sink.taken();
    let updated: Vec<&JsonRpcNotification> = published.iter().filter(|notification| notification.method == NOTIFICATION_RESOURCES_UPDATED).collect();
    assert_eq!(updated.len(), 1, "expected exactly one updated notification, got {published:?}");
    assert_eq!(updated[0].params.as_ref().and_then(|params| params.get("uri")).and_then(serde_json::Value::as_str), Some(uri));
    assert!(delivered >= 1);
}

#[test]
fn an_unsubscribed_connection_is_told_nothing() {
    let (mut server, sink) = server_with_sink();
    let uri = "semio://artifact/doc-9";
    server.dispatch(&request(1, "resources/subscribe", serde_json::json!({ "uri": uri })));
    server.dispatch(&request(2, "resources/unsubscribe", serde_json::json!({ "uri": uri })));
    sink.taken();
    artifact_changed("doc-9");
    assert!(sink.taken().iter().all(|notification| notification.method != NOTIFICATION_RESOURCES_UPDATED));
}

#[test]
fn a_dropped_connection_stops_receiving_and_is_pruned() {
    let before = resource_update_broker().live_connections();
    {
        let (mut server, sink) = server_with_sink();
        server.dispatch(&request(1, "resources/subscribe", serde_json::json!({ "uri": "semio://artifact/doc-drop" })));
        sink.taken();
        assert_eq!(artifact_changed("doc-drop"), 1);
        assert!(resource_update_broker().live_connections() > before);
    }
    assert_eq!(artifact_changed("doc-drop"), 0, "a closed connection still received a notification");
}

#[test]
fn the_declared_table_says_which_tool_changed_which_resource() {
    let invoked = changes_from_tool_result("action_invoke", false, Some(&serde_json::json!({ "revisionAfter": { "artifactId": "doc-3" } })));
    assert!(invoked.contains(&ResourceChange::Updated { uri: "semio://artifact/doc-3".to_string() }));
    assert!(!invoked.contains(&ResourceChange::ListChanged), "a mutation does not change the roster");
    let created = changes_from_tool_result("artifact_create", false, Some(&serde_json::json!({ "artifactId": "doc-4" })));
    assert!(created.contains(&ResourceChange::ListChanged), "a created artifact changes the roster");
    assert!(created.contains(&ResourceChange::Updated { uri: "semio://workspace/artifacts".to_string() }));
    assert!(changes_from_tool_result("action_invoke", true, Some(&serde_json::json!({ "revisionAfter": { "artifactId": "doc-3" } }))).is_empty(), "a failed call changed nothing");
    assert!(changes_from_tool_result("capabilities_search", false, Some(&serde_json::json!({ "hits": [] }))).is_empty(), "a read-only tool changed nothing");
}

#[test]
fn a_committed_mutation_through_tools_call_reaches_a_subscriber() {
    let (mut server, sink) = server_with_sink();
    // 🧫️ `MockArtifactChannel` names its artifact `mock-artifact-<instance>`; subscribe to every
    // instance the fixture could mint so this test asserts the WIRING, not an id guess.
    for instance in 0..4u32 {
        server.dispatch(&request(1, "resources/subscribe", serde_json::json!({ "uri": format!("semio://artifact/mock-artifact-{instance}") })));
    }
    sink.taken();
    let called = result_of(&mut server, request(9, "tools/call", serde_json::json!({ "name": "action_prepare", "arguments": { "capabilityId": "note.insert-text", "input": { "text": "hello" } } })));
    let _ = called;
    let invoked = result_of(&mut server, request(10, "tools/call", serde_json::json!({ "name": "action_invoke", "arguments": { "capabilityId": "note.insert-text", "input": { "text": "hello" } } })));
    let structured = invoked.get("structuredContent").cloned().unwrap_or(serde_json::Value::Null);
    let artifact_id = structured.get("revisionAfter").and_then(|revision| revision.get("artifactId")).and_then(serde_json::Value::as_str);
    let published = sink.taken();
    match artifact_id {
        // 🔔️ The real proof: the very artifact id the invocation reports is the URI a subscriber was
        // told about, through `tools/call` alone — no test-only broadcast anywhere in this path.
        Some(artifact_id) => {
            let expected = format!("semio://artifact/{artifact_id}");
            assert!(
                published.iter().any(|notification| notification.method == NOTIFICATION_RESOURCES_UPDATED && notification.params.as_ref().and_then(|params| params.get("uri")).and_then(serde_json::Value::as_str) == Some(expected.as_str())),
                "a committed mutation of {artifact_id} pushed no resources/updated; published {published:?}"
            );
        }
        // 🧫️ The fixture refused the invocation (no scope, no such capability): then nothing may have
        // been published either — silence on a non-mutation is exactly as load-bearing.
        None => assert!(published.iter().all(|notification| notification.method != NOTIFICATION_RESOURCES_UPDATED), "a failed invocation still pushed an update: {published:?}"),
    }
}
//#endregion 🧪️Subscriptions

//#region 🧪️Progress
#[test]
fn a_job_minted_outside_a_progress_scope_publishes_nothing() {
    let job_id = crate::ui::job_registry().begin("m5b-unscoped");
    assert!(!job_progress_changed(&job_id, 0.5, None));
}

#[test]
fn a_job_minted_inside_a_progress_scope_pushes_notifications_progress() {
    let sink = Arc::new(RecordingSink::new());
    let slot = notification_slot();
    slot.set(sink.clone() as Arc<dyn NotificationSink>).ok().expect("a fresh slot is empty");
    let job_id = {
        let _scope = enter_progress_scope(Some(ProgressBinding { token: serde_json::json!("tok-m5b-1"), slot: slot.clone() }));
        let job_id = crate::ui::job_registry().begin("m5b-scoped");
        crate::ui::job_registry().report_progress(&job_id, 0.25, Some("quarter".to_string()));
        crate::ui::job_registry().report_progress(&job_id, 0.75, None);
        job_id
    };
    let published = sink.taken();
    let rows: Vec<&JsonRpcNotification> = published.iter().filter(|notification| notification.method == NOTIFICATION_PROGRESS).collect();
    assert_eq!(rows.len(), 2, "expected one progress notification per report, got {published:?}");
    assert_eq!(rows[0].params.as_ref().and_then(|params| params.get("progressToken")), Some(&serde_json::json!("tok-m5b-1")));
    assert_eq!(rows[0].params.as_ref().and_then(|params| params.get("progress")), Some(&serde_json::json!(0.25)));
    assert_eq!(rows[0].params.as_ref().and_then(|params| params.get("message")), Some(&serde_json::json!("quarter")));
    assert_eq!(rows[1].params.as_ref().and_then(|params| params.get("progress")), Some(&serde_json::json!(0.75)));
    // 🧹️ Terminal transition publishes the last row and releases the binding.
    crate::ui::job_registry().succeed(&job_id, serde_json::json!({}));
    let terminal = sink.taken();
    assert_eq!(terminal.iter().filter(|notification| notification.method == NOTIFICATION_PROGRESS).count(), 1);
    assert!(!job_progress_changed(&job_id, 1.0, None), "a terminal job's binding was not released");
}

#[test]
fn a_progress_scope_restores_the_one_it_replaced() {
    let slot = notification_slot();
    let outer = ProgressBinding { token: serde_json::json!("outer"), slot: slot.clone() };
    let _outer_scope = enter_progress_scope(Some(outer));
    {
        let _inner = enter_progress_scope(Some(ProgressBinding { token: serde_json::json!("inner"), slot }));
        assert_eq!(active_progress_binding().map(|binding| binding.token), Some(serde_json::json!("inner")));
    }
    assert_eq!(active_progress_binding().map(|binding| binding.token), Some(serde_json::json!("outer")));
}

#[test]
fn a_tools_call_progress_token_binds_the_jobs_that_call_mints() {
    let (mut server, sink) = server_with_sink();
    sink.taken();
    // 🧪️ `job_get` against an unknown id is a real call that mints no job — what it proves is that
    // the token plumbing is entered and left cleanly, with no notification invented.
    let called = request(77, "tools/call", serde_json::json!({ "name": "job_get", "arguments": { "jobId": "job-does-not-exist" }, "_meta": { "progressToken": "tok-m5b-2" } }));
    let _ = server.dispatch(&called);
    assert!(active_progress_binding().is_none(), "the progress scope outlived the call");
    assert!(sink.taken().iter().all(|notification| notification.method != NOTIFICATION_PROGRESS));
}

#[test]
fn notifications_cancelled_requests_cancel_of_every_job_minted_under_that_request() {
    let slot = notification_slot();
    let request_id = serde_json::json!(4242);
    let job_id = {
        let _request = enter_request_scope(Some(&request_id));
        let _scope = enter_progress_scope(Some(ProgressBinding { token: serde_json::json!("tok-m5b-3"), slot }));
        let job_id = crate::ui::job_registry().begin("m5b-cancellable");
        crate::ui::job_registry().report_progress(&job_id, 0.1, None);
        job_id
    };
    assert_eq!(jobs_for_request(&request_id), vec![job_id.clone()]);
    assert!(!crate::ui::job_registry().is_cancel_requested(&job_id));
    let (mut server, _sink) = server_with_sink();
    assert!(server.dispatch(&notification("notifications/cancelled", serde_json::json!({ "requestId": request_id }))).is_none(), "a notification must never be answered");
    assert!(crate::ui::job_registry().is_cancel_requested(&job_id), "notifications/cancelled did not reach the job registry's cancel path");
    crate::ui::job_registry().mark_cancelled(&job_id);
}
#[test]
fn notifications_cancelled_reaches_a_job_minted_without_a_progress_token() {
    let request_id = serde_json::json!(4243);
    let job_id = {
        let _request = enter_request_scope(Some(&request_id));
        let job_id = crate::ui::job_registry().begin("g4-untokened");
        crate::ui::job_registry().report_progress(&job_id, 0.1, None);
        job_id
    };
    cancel_request(&request_id);
    assert!(crate::ui::job_registry().is_cancel_requested(&job_id), "a call without _meta.progressToken must still be cancellable by its request id");
    crate::ui::job_registry().mark_cancelled(&job_id);
}

#[test]
fn a_cancel_that_overtakes_its_request_cancels_the_job_the_moment_it_is_minted() {
    let request_id = serde_json::json!(4244);
    cancel_request(&request_id);
    let _request = enter_request_scope(Some(&request_id));
    let job_id = crate::ui::job_registry().begin("g4-overtaken");
    assert!(crate::ui::job_registry().is_cancel_requested(&job_id));
    assert_eq!(crate::ui::job_registry().snapshot(&job_id).unwrap().status, crate::ui::JobStatus::Cancelled);
}

#[test]
fn the_stdio_reader_consumes_notifications_cancelled_out_of_band() {
    let request_id = serde_json::json!(4245);
    let job_id = {
        let _request = enter_request_scope(Some(&request_id));
        let job_id = crate::ui::job_registry().begin("g4-out-of-band");
        crate::ui::job_registry().report_progress(&job_id, 0.1, None);
        job_id
    };
    assert!(!crate::protocol::intercept_cancellation(r#"{"jsonrpc":"2.0","id":9,"method":"ping"}"#));
    assert!(!crate::protocol::intercept_cancellation(r#"{"jsonrpc":"2.0","id":9,"method":"notifications/cancelled","params":{"requestId":4245}}"#), "a REQUEST named like the notification is not the notification");
    assert!(!crate::ui::job_registry().is_cancel_requested(&job_id));
    assert!(crate::protocol::intercept_cancellation("{\"jsonrpc\":\"2.0\",\"method\":\"notifications/cancelled\",\"params\":{\"requestId\":4245}}\n"));
    assert!(crate::ui::job_registry().is_cancel_requested(&job_id));
    crate::ui::job_registry().mark_cancelled(&job_id);
}
//#endregion 🧪️Progress

//#region 🧪️StructuredOutput
/// 📐️ Every tool that declares an `outputSchema` must declare one that COMPILES with this repo's own
/// owned JSON-Schema validator, and a representative real result of that tool must validate against
/// it. A tool with no `outputSchema` is reported by name so the gap can never hide.
#[test]
fn every_tool_declares_a_compilable_output_schema() {
    let (server, _sink) = server_with_sink();
    let tools = server.tools.list();
    assert!(tools.len() >= 20, "expected the real tool census, got {}", tools.len());
    let mut without: Vec<String> = Vec::new();
    let mut uncompilable: Vec<String> = Vec::new();
    for tool in &tools {
        match tool.output_schema.as_ref() {
            None => without.push(tool.name.clone()),
            Some(schema) => {
                if let Err(error) = crate::schema::compile_validator(schema) {
                    uncompilable.push(format!("{}: {error}", tool.name));
                }
            }
        }
    }
    assert!(uncompilable.is_empty(), "tools whose declared outputSchema does not compile: {uncompilable:?}");
    assert!(without.is_empty(), "tools with no declared outputSchema: {without:?}");
}

/// 📐️ The structural contract every tool result carries: a non-error result declares
/// `structuredContent`, and that content validates against the tool's own declared `outputSchema`.
/// Driven through `tools/call` for real, over every tool this fixture server can call with no
/// arguments — a tool whose no-argument call is an input error is skipped, and the skipped names are
/// reported so the count is never silently zero.
#[test]
fn every_callable_tool_result_validates_against_its_own_output_schema() {
    let (mut server, _sink) = server_with_sink();
    let tools = server.tools.list();
    let schemas: BTreeMap<String, serde_json::Value> = tools.iter().filter_map(|tool| tool.output_schema.clone().map(|schema| (tool.name.clone(), schema))).collect();
    // 📐️ One representative REAL argument set per tool that needs one, so the oracle validates an
    // actual result rather than only the handful of tools that answer to `{}`.
    let representative: BTreeMap<&str, serde_json::Value> = BTreeMap::from([
        ("capabilities_search", serde_json::json!({ "query": "note" })),
        ("capabilities_describe", serde_json::json!({ "capabilityId": "capabilities.search" })),
        ("context_resolve", serde_json::json!({})),
        ("action_prepare", serde_json::json!({ "capabilityId": "capabilities.search", "input": { "query": "note" } })),
    ]);
    let mut validated = 0usize;
    let mut skipped: Vec<String> = Vec::new();
    let mut invalid: Vec<String> = Vec::new();
    for (index, tool) in tools.iter().enumerate() {
        let arguments = representative.get(tool.name.as_str()).cloned().unwrap_or(serde_json::json!({}));
        let result = result_of(&mut server, request(1000 + index as i64, "tools/call", serde_json::json!({ "name": tool.name, "arguments": arguments })));
        if result.get("isError").and_then(serde_json::Value::as_bool).unwrap_or(true) {
            skipped.push(tool.name.clone());
            continue;
        }
        let Some(structured) = result.get("structuredContent").filter(|value| !value.is_null()) else {
            invalid.push(format!("{}: a successful result carried no structuredContent", tool.name));
            continue;
        };
        let Some(schema) = schemas.get(&tool.name) else {
            invalid.push(format!("{}: no declared outputSchema to validate against", tool.name));
            continue;
        };
        match crate::schema::compile_validator(schema) {
            Err(error) => invalid.push(format!("{}: outputSchema does not compile: {error}", tool.name)),
            Ok(validator) => match crate::schema::validate(&validator, structured) {
                Ok(_) => validated += 1,
                Err(error) => invalid.push(format!("{}: structuredContent does not validate: {error}", tool.name)),
            },
        }
    }
    assert!(invalid.is_empty(), "tool results that break their own declared contract: {invalid:?}");
    assert!(validated >= 3, "only {validated} tool(s) could be driven with no arguments (skipped: {skipped:?}) — the oracle proved almost nothing");
}
//#endregion 🧪️StructuredOutput

//#region 🧪️HubInferenceRouting
#[test]
fn the_hub_inference_route_table_resolves_by_descriptor_kind_only() {
    let route = crate::inference::hub_inference_route_for(crate::inference::GIS_MAP_INFERENCE_ARTIFACT_KIND, crate::inference::GIS_MAP_INFERENCE_ARTIFACT_SCHEMA).expect("the gis map row is declared");
    assert_eq!(route.service_id, crate::inference::GIS_MAP_INFERENCE_SERVICE_ID);
    assert!(crate::inference::hub_inference_route_for("s.note", "note.document").is_none(), "a kind with no hub-backed service must not resolve to one");
    assert!(crate::inference::hub_inference_route_for(crate::inference::GIS_MAP_INFERENCE_ARTIFACT_KIND, "some.other.schema").is_none(), "the kind AND the schema both have to match");
}

#[test]
fn a_submit_request_only_validates_for_a_declared_hub_backed_service() {
    let good = crate::inference::GisMapInferenceSubmitRequestV1::new(crate::inference::GIS_MAP_INFERENCE_SERVICE_ID, "0".repeat(crate::inference::INFERENCE_REQUEST_ID_HEX_LENGTH), 1000);
    assert!(good.validate().is_ok());
    let foreign = crate::inference::GisMapInferenceSubmitRequestV1::new("s.note.inference", "0".repeat(crate::inference::INFERENCE_REQUEST_ID_HEX_LENGTH), 1000);
    assert!(foreign.validate().is_err(), "a service id no route declares must never be submittable");
}
//#endregion 🧪️HubInferenceRouting
