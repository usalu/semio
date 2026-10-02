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

/// 🪪️ `artifact_kind().schema` IS `PROCEDURE_DOCUMENT_SCHEMA`: a document kind has ONE schema identity — the hub's codec rows, document-open targets and genesis, the MCP workspace
/// store and host-media contributions all key on it (ticket 26/09/23 W4: a distinct "media schema" left the package without a
/// codec owner, so the trusted catalog refused it). The former media string stays declared as `source_format`.
#[semio_framework_async_macros::async_test]
async fn artifact_kind_names_the_store_schema() {
    assert_eq!(artifact_kind().schema, PROCEDURE_DOCUMENT_SCHEMA);
    assert_eq!(artifact_kind().source_format, "procedure.document");
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

/// 🧬️ The projection names exactly the snapshot's declared child slots — what the live envelope load checks
/// before a decoded document may replace the store.
#[test]
fn the_child_restore_projection_names_every_declared_child_slot() {
    let snapshot = crate::schema::default_snapshot();
    let projection = crate::procedure_child_restore_projection(&snapshot).expect("the loaded-parent child projection");
    assert_eq!(projection.len(), <crate::ProcedureSnapshot as semio_framework_schema_composition::ArtifactCompositionFields>::child_slots().len());
}
