use super::*;

trait ProcedureChildOwnerOracle {
    fn expected() -> serde_json::Value;
}

struct SerdeJsonProcedureChildOwnerOracle;

impl ProcedureChildOwnerOracle for SerdeJsonProcedureChildOwnerOracle {
    fn expected() -> serde_json::Value {
        serde_json::from_str(include_str!("../../🧫️fixtures/🧫️child-owner-isolation/🔣️.json")).expect("language-neutral Imperative child-owner fixture")
    }
}

/// 🗂️ The manifest-facing `ArtifactKindSpec.schema` ("procedure.document") is deliberately NOT
/// `PROCEDURE_DOCUMENT_SCHEMA` ("procedure.document/v1") — the former names the artifact kind in
/// the OS media catalogue, the latter keys the store envelope. Pinned so a future edit can't silently
/// merge them.
#[semio_framework_async_macros::async_test]
async fn artifact_kind_keeps_the_media_schema_distinct_from_the_store_schema() {
    assert_eq!(artifact_kind().schema, "procedure.document");
    assert_eq!(PROCEDURE_DOCUMENT_SCHEMA, "procedure.document/v1");
}

#[semio_framework_async_macros::async_test]
async fn default_snapshot_is_empty_with_the_bare_schema() {
    let snapshot = ProcedureSnapshot::default();
    assert_eq!(snapshot.schema, "procedure.document");
    let scene = procedure_working_scene(&snapshot);
    assert!(scene.path.steps.is_empty());
    assert!(scene.seed.keys().next().is_none());
}

#[semio_framework_async_macros::async_test]
async fn working_content_is_owned_by_each_exact_child() {
    let flow = procedure_flow_child_with_owner(&Path::new());
    let text = procedure_text_child_with_owner(&BTreeMap::new());
    let flow_wire = dsl::os_pack::to_json_string(&flow);
    let text_wire = dsl::os_pack::to_json_string(&text);
    let reconstructed_flow: ProcedureFlowChild = dsl::os_pack::from_json_str(&flow_wire).expect("Imperative flow child wire roundtrip");
    let reconstructed_text: ProcedureTextChild = dsl::os_pack::from_json_str(&text_wire).expect("Imperative text child wire roundtrip");
    let observed = serde_json::json!({
        "ownedFlowHasPayload": flow.local_owner::<ProcedureFlowWorkingData>().is_some(),
        "ownedTextHasPayload": text.local_owner::<ProcedureTextWorkingData>().is_some(),
        "flowWireIdentityMatches": flow == reconstructed_flow,
        "textWireIdentityMatches": text == reconstructed_text,
        "flowWireHasPayload": reconstructed_flow.local_owner::<ProcedureFlowWorkingData>().is_some(),
        "textWireHasPayload": reconstructed_text.local_owner::<ProcedureTextWorkingData>().is_some(),
    });

    assert_eq!(observed, SerdeJsonProcedureChildOwnerOracle::expected());
}
