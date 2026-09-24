//! 💼️ Replays `🧫️fixtures/💼️inference-service-law.json`: service selection and execution site,
//! proposal digests, the guest job lifecycle folded from a real `JobRegistry` journal, and the hub
//! page projection — every produced page is also held against `$defs/InferenceJobPageV1`.

use super::*;
use crate::ui::JobRegistry;

fn law() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/💼️inference-service-law.json")).expect("inference service law fixture")
}

/// 🧬️ `name` rooted over exactly the component `$defs` it reaches, so a sibling definition's
/// cross-document reference never has to resolve here.
fn schema_def(name: &str) -> serde_json::Value {
    let component: serde_json::Value = serde_json::from_str(include_str!("../../🧬️schema/🔣️.json")).expect("inference component schema");
    let mut reached = std::collections::BTreeMap::new();
    let mut pending = vec![name.to_string()];
    while let Some(next) = pending.pop() {
        if reached.contains_key(&next) {
            continue;
        }
        let def = component["$defs"][&next].clone();
        let text = def.to_string();
        pending.extend(text.split("\"#/$defs/").skip(1).filter_map(|rest| rest.split('"').next()).map(str::to_string));
        reached.insert(next, def);
    }
    serde_json::json!({ "$schema": "http://json-schema.org/draft-07/schema#", "$ref": format!("#/$defs/{name}"), "$defs": reached })
}

fn assert_valid(def: &str, value: &serde_json::Value) {
    let validator = crate::schema::compile_validator(&schema_def(def)).unwrap_or_else(|error| panic!("{def} compiles: {error}"));
    crate::schema::validate(&validator, value).unwrap_or_else(|error| panic!("{def} rejects {value}: {error}"));
}

fn text<'a>(value: &'a serde_json::Value, key: &str) -> Option<&'a str> {
    value.get(key).and_then(serde_json::Value::as_str)
}

fn code(error: &GatewayError) -> String {
    serde_json::to_value(error.code).ok().and_then(|value| value.as_str().map(str::to_string)).unwrap_or_default()
}

#[test]
fn the_law_fixture_is_its_own_schema() {
    assert_valid("InferenceServiceLawV1", &law());
}

#[test]
fn selection_resolves_one_service_and_the_site_that_executes_it() {
    let law = law();
    let declared: Vec<DeclaredInference> = serde_json::from_value(law["declared"].clone()).expect("declared roster");
    for case in law["selection"].as_array().expect("selection cases") {
        let name = text(case, "name").unwrap_or_default();
        let hub: Vec<HubInferenceServiceV1> = serde_json::from_value(case["hubServices"].clone()).expect("hub services");
        let outcome = select_inference_service(&declared, &hub, text(case, "artifactKind").unwrap_or_default(), text(case, "inferenceSchema"), text(case, "pluginId"));
        match (&outcome, case.get("expect"), case.get("refusal")) {
            (Ok(service), Some(expect), None) => {
                assert_eq!(service.service_id(), text(expect, "serviceId").unwrap_or_default(), "{name}");
                assert_eq!(service.plugin_id(), text(expect, "pluginId").unwrap_or_default(), "{name}");
                assert_eq!(serde_json::to_value(service.site).unwrap(), expect["site"], "{name}");
                assert_eq!(service.route.as_deref(), text(expect, "route"), "{name}");
                assert_eq!(service.commit_action.as_deref(), text(expect, "commitAction"), "{name}");
            }
            (Err(error), None, Some(refusal)) => {
                assert_eq!(code(error), text(refusal, "code").unwrap_or_default(), "{name}");
                if let Some(field) = text(refusal, "field") {
                    assert_eq!(error.details.get("field").and_then(serde_json::Value::as_str), Some(field), "{name}");
                }
            }
            (other, _, _) => panic!("{name}: unexpected outcome {other:?}"),
        }
    }
}

#[test]
fn a_proposal_carries_exactly_the_declared_arguments_under_a_stable_digest() {
    for case in law()["proposals"].as_array().expect("proposal cases") {
        let name = text(case, "name").unwrap_or_default();
        let args: Vec<String> = serde_json::from_value(case["declaredArgs"].clone()).expect("declared args");
        let outcome = guest_proposal(text(case, "serviceId").unwrap(), text(case, "documentId").unwrap(), text(case, "action").unwrap(), text(case, "capabilityId").unwrap(), &args, &case["result"]);
        match (outcome, case.get("expect"), case.get("refusal")) {
            (Ok(proposal), Some(expect), None) => {
                assert_eq!(proposal.input.as_ref(), Some(&expect["input"]), "{name}");
                assert_eq!(proposal.hash, text(expect, "hash").unwrap_or_default(), "{name}");
                assert_eq!(proposal.preview.as_ref(), Some(&case["result"]), "{name}");
            }
            (Err(error), None, Some(refusal)) => assert_eq!(code(&error), text(refusal, "code").unwrap_or_default(), "{name}"),
            (other, _, _) => panic!("{name}: unexpected outcome {other:?}"),
        }
    }
}

fn law_service(declared: &[DeclaredInference], proposal: bool) -> InferenceService {
    let mut service = select_inference_service(declared, &[], "s.wfc.bitmap", None, None).expect("the wfc service");
    service.commit_action = proposal.then(|| "pin-solution".to_string());
    service
}

#[test]
fn a_journal_folds_into_the_same_page_state_at_every_step() {
    let law = law();
    let declared: Vec<DeclaredInference> = serde_json::from_value(law["declared"].clone()).expect("declared roster");
    for case in law["lifecycles"].as_array().expect("lifecycle cases") {
        let name = text(case, "name").unwrap_or_default();
        let registry = JobRegistry::new();
        let job_id = registry.begin("inference.submit");
        let proposal = guest_proposal("s.wfc.bitmap.solve", "doc-bitmap", "pin-solution", "wfc.s.wfc.bitmap@1/*#editor.pin-solution", &["pixels".to_string()], &serde_json::json!({ "pixels": "AAEC" })).expect("law proposal");
        let mut job = GuestInferenceJob { service: law_service(&declared, case["proposal"] == true), document_id: "doc-bitmap".into(), proposal: None, committed: None, commit_refused: false };
        for step in case["steps"].as_array().expect("steps") {
            let op = text(step, "op").unwrap_or_default();
            let accepted = match op {
                "progress" => registry.report_progress(&job_id, step["fraction"].as_f64().unwrap_or(0.0), None),
                "offer" => {
                    job.proposal = Some(proposal.clone());
                    registry.await_approval(&job_id, serde_json::to_value(&proposal).unwrap())
                }
                "approve" => registry.resume_approved(&job_id),
                "commit" => {
                    job.committed = Some(serde_json::json!({ "invocationId": "inv-law" }));
                    registry.record_event(&job_id, "committed", None) && registry.succeed(&job_id, serde_json::json!({}))
                }
                "refuse" => {
                    job.commit_refused = true;
                    registry.fail(&job_id, GatewayError::new(crate::errors::GatewayErrorCode::SideEffectRejected, "law refusal"))
                }
                "cancel" => registry.request_cancel(&job_id).is_ok(),
                "cancelled" => registry.mark_cancelled(&job_id),
                "succeed" => registry.succeed(&job_id, serde_json::json!({ "pixels": "AAEC" })),
                other => panic!("{name}: unknown op {other}"),
            };
            if let Some(expected) = step.get("accepted") {
                assert_eq!(serde_json::Value::Bool(accepted), *expected, "{name}: {op} acceptance");
            }
            let journal = registry.events(&job_id, 0, INFERENCE_JOB_EVENT_PAGE_MAX_ITEMS).expect("journal");
            let page = guest_page(&job_id, &registry.snapshot(&job_id).expect("snapshot"), &journal.events, journal.next_cursor, &job);
            let expect = &step["expect"];
            assert_eq!(serde_json::to_value(page.state).unwrap(), expect["state"], "{name}: {op} state");
            assert_eq!(serde_json::to_value(page.proposal_state).unwrap(), expect["proposalState"], "{name}: {op} proposal state");
            assert_eq!(page.cancel_requested, expect.get("cancelRequested") == Some(&serde_json::Value::Bool(true)), "{name}: {op} cancel flag");
            assert_eq!(page.result.is_some(), expect.get("result") == Some(&serde_json::Value::Bool(true)), "{name}: {op} result");
            assert_valid("InferenceJobPageV1", &serde_json::to_value(&page).unwrap());
        }
        let kinds: Vec<String> = registry.events(&job_id, 0, 256).expect("journal").events.into_iter().map(|event| event.kind).collect();
        let expected: Vec<String> = serde_json::from_value(case["journal"].clone()).expect("journal kinds");
        assert_eq!(kinds, expected, "{name}: journal");
        let tail = registry.events(&job_id, 2, 256).expect("tail");
        assert!(tail.events.iter().all(|event| event.ordinal > 2), "{name}: a cursor read returns only later rows");
    }
}

#[test]
fn a_hub_page_projects_onto_the_same_page_shape() {
    let law = law();
    let declared: Vec<DeclaredInference> = serde_json::from_value(law["declared"].clone()).expect("declared roster");
    let hub: Vec<HubInferenceServiceV1> = serde_json::from_value(serde_json::json!([{ "serviceId": "s.gis.gismap.inference", "route": "inference/gis-map" }])).unwrap();
    let service = select_inference_service(&declared, &hub, "s.gis.gismap", None, None).expect("the gis service");
    for case in law["hubPages"].as_array().expect("hub page cases") {
        let name = text(case, "name").unwrap_or_default();
        let page: HubInferenceEventPageV1 = serde_json::from_value(case["page"].clone()).expect("hub page");
        let projected = hub_events_page(&service, "doc-map", &page);
        let expect = &case["expect"];
        assert_eq!(serde_json::to_value(projected.state).unwrap(), expect["state"], "{name}");
        assert_eq!(serde_json::to_value(projected.proposal_state).unwrap(), expect["proposalState"], "{name}");
        assert_eq!(projected.proposal.as_ref().map(|proposal| proposal.hash.as_str()), text(expect, "proposalHash"), "{name}");
        let fractions: Vec<f64> = projected.progress.iter().map(|row| row.fraction).collect();
        assert_eq!(serde_json::to_value(fractions).unwrap(), expect["fractions"], "{name}");
        let events: Vec<&str> = projected.events.iter().map(|event| event.kind.as_str()).collect();
        assert_eq!(serde_json::to_value(events).unwrap(), expect["events"], "{name}");
        assert_eq!(projected.next_cursor, expect["nextCursor"].as_u64().unwrap(), "{name}");
        assert_eq!(projected.site, InferenceExecutionSiteV1::Hub);
        assert_valid("InferenceJobPageV1", &serde_json::to_value(&projected).unwrap());
    }
}

#[test]
fn canonical_json_orders_keys_at_every_depth() {
    let value = serde_json::json!({ "b": { "z": 1, "a": [ { "y": true, "x": null } ] }, "a": "é" });
    assert_eq!(canonical_json(&value), r#"{"a":"é","b":{"a":[{"x":null,"y":true}],"z":1}}"#);
}
