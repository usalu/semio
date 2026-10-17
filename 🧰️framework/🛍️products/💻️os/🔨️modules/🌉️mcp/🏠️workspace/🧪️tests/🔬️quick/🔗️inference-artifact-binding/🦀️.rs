/// 🔗️ Inference artifact binding: the bound inference rows and commands and the binding field, encoding and contract laws.

//#region 🔗️InferenceArtifactBinding

/// 🧾️ One declared inference row carrying a published contract, for the binding laws below. The
/// shape is exactly what a re-described `🀄️wfc` commits under
/// `contributions.inferenceServices[].payload`.
fn bound_inference_row(required: bool, encoding: &str) -> semio_framework::ContributedInferenceMetadata {
    semio_framework::ContributedInferenceMetadata {
        owner: "wfc".into(),
        artifact_kind: "s.wfc.bitmap".into(),
        artifact_schema: "s.wfc.bitmap".into(),
        artifact_schema_version: 1,
        inference_schema: "s.wfc.bitmap.solve".into(),
        inference_schema_version: 1,
        algorithm_version: 1,
        policy_version: 1,
        contributor: "wfc".into(),
        depends_on: Vec::new(),
        payload: Some(semio_framework::InferencePayloadContract {
            payload_schema_id: "s.wfc.bitmap.inference.request.v1".into(),
            input_schema: "{\"type\":\"object\"}".into(),
            output_schema: "{\"type\":\"object\"}".into(),
            progress_unit: "cells".into(),
            artifact_binding: Some(semio_framework::InferenceArtifactBinding { field: "document".into(), encoding: encoding.into(), required }),
            commit: None,
        }),
    }
}

fn bound_inference_command(document: Option<ArtifactDocumentBinding>, payload: &[u8]) -> InferCommand {
    InferCommand { plugin_id: "wfc".into(), artifact_kind: "s.wfc.bitmap".into(), inference_schema: "s.wfc.bitmap.solve".into(), turn:semio_framework_actor::RetainedTurnInput{operation:71,generation:3,epoch:19,work:semio_framework_actor::OriginalActorTurnWork{clock_binding:1671,absolute_deadline_us:1000000,fuel:64},grant:semio_framework_value::RetainedCloneGrant{maximum_items:7,maximum_copy_bytes:3,maximum_capacity_bytes:129,maximum_release_bytes:4096,maximum_depth:2}},retained_policy:semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:3,maximum_capacity_bytes:129,maximum_release_bytes:4096,maximum_depth:2},revision:7,cancellation_id:"original binding cancellation".into(),work_units:11,maximum_elapsed_milliseconds:13, canonical_payload: payload.to_vec(),artifact_id:String::new(), artifact_document: document,cancel:InferenceCancel::default() }
}

/// 🔗️ The artifact the caller named lands under the field the PLUGIN declared, base64, and nowhere
/// else — the fix for the gap `📓️pz2-puzzle-describe-under-budget.md` §5.2 named (`InferCommand`
/// carried no artifact binding at all, so the guest had no document to read a snapshot from).
///
/// 🔡️ …and what the host wrote is what a guest decodes back, byte for byte — the two halves of
///    `artifact-pack-base64` meet here and nowhere else.
#[test]
fn a_named_artifact_is_bound_under_the_field_the_contract_declares() {
    let declared = bound_inference_row(true, semio_framework::INFERENCE_ARTIFACT_PACK_BASE64);
    let command = bound_inference_command(Some(ArtifactDocumentBinding { pack: b"PACK-BYTES".to_vec(), spr: b"SPR".to_vec() }), b"{}");
    let bound: serde_json::Value = serde_json::from_slice(&bind_inference_document(&declared, &command).expect("a named artifact binds")).expect("the bound body is JSON");
    assert_eq!(bound["document"]["pack"], serde_json::Value::String(encode_base64(b"PACK-BYTES")));
    assert_eq!(bound["document"]["spr"], serde_json::Value::String(encode_base64(b"SPR")));
    assert_eq!(decode_base64(bound["document"]["pack"].as_str().expect("a base64 string")), Some(b"PACK-BYTES".to_vec()));
}

/// ✍️ A caller who authored the bound field keeps it: the binding fills a GAP, it never overwrites a
/// body the caller wrote.
#[test]
fn a_caller_authored_binding_field_is_never_overwritten() {
    let declared = bound_inference_row(true, semio_framework::INFERENCE_ARTIFACT_PACK_BASE64);
    let command = bound_inference_command(Some(ArtifactDocumentBinding { pack: b"HOST".to_vec(), spr: b"HOST".to_vec() }), br#"{"document":{"pack":"mine","spr":"mine"}}"#);
    let bound: serde_json::Value = serde_json::from_slice(&bind_inference_document(&declared, &command).expect("the caller's body survives")).expect("JSON");
    assert_eq!(bound["document"]["pack"], serde_json::Value::String("mine".to_string()));
}

/// 🚧️ A REQUIRED binding with no artifact named never reaches a guest: the refusal says which tool
/// argument fixes it, in milliseconds, instead of a 240 s dispatch that ends in a decode fault.
#[test]
fn a_required_binding_without_an_artifact_refuses_by_name() {
    let declared = bound_inference_row(true, semio_framework::INFERENCE_ARTIFACT_PACK_BASE64);
    let fault = bind_inference_document(&declared, &bound_inference_command(None, b"{}")).expect_err("a required binding with no artifact is refused");
    assert!(fault.message.contains("artifactId") && fault.message.contains("payload.document"), "{}", fault.message);
}

/// 🚧️ An encoding this gateway has no writer for is named, never silently written in the one shape
/// this host happens to know.
#[test]
fn an_unknown_binding_encoding_is_refused_rather_than_guessed() {
    let declared = bound_inference_row(true, "artifact-dsl-text");
    let fault = bind_inference_document(&declared, &bound_inference_command(Some(ArtifactDocumentBinding::default()), b"{}")).expect_err("an unwritable encoding is refused");
    assert!(fault.message.contains("artifact-dsl-text") && fault.message.contains(semio_framework::INFERENCE_ARTIFACT_PACK_BASE64), "{}", fault.message);
}

/// 🫙 An inference that publishes NO contract keeps the host-opaque behaviour it always had: the
/// caller's own body travels verbatim.
#[test]
fn an_inference_without_a_published_contract_passes_its_payload_through() {
    let mut declared = bound_inference_row(true, semio_framework::INFERENCE_ARTIFACT_PACK_BASE64);
    declared.payload = None;
    let bound = bind_inference_document(&declared, &bound_inference_command(None, br#"{"seed":7}"#)).expect("no contract, no binding");
    assert_eq!(bound, br#"{"seed":7}"#.to_vec());
}

//#endregion 🔗️InferenceArtifactBinding
