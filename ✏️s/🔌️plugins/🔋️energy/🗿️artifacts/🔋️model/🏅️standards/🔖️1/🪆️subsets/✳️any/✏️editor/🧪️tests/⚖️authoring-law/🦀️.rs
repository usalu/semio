//! ⚖️ Real-store authoring: an energy config edit authors, applies, undoes and redoes through the energy editor's own config catalog and the runtime's canonical operation wire.
use semio_framework_plugin::{authoring_law::author_through_config_store, EditorApp};
use semio_framework_value::ToValue;
use semio_s_artifact_energy_model_editor_config::{ChangeResultField, EnergyModelConfig, EnergyModelConfigMutation};
use semio_s_artifact_energy_model_editor::EnergyModelEditor;

fn assert_row_is_the_serde_document(row: &[u8], mutation: &EnergyModelConfigMutation) {
    assert!(row.starts_with(b"op"), "the canonical wire carries its operation header");
    let wire: serde_json::Value = serde_json::from_slice(&row[2..]).expect("the wire body is JSON an independent parser accepts");
    assert_eq!(wire, serde_json::to_value(mutation.to_value()).expect("independent serde_json document"));
}

/// 🎚️ Changing the result field publishes one forward row equal to the independent serde_json document, and its inverse restores the displaced field.
#[semio_framework_async_macros::async_test]
async fn an_energy_config_edit_authors_applies_undoes_and_redoes_through_its_real_store() {
    let base = EnergyModelConfig::default();
    let mutation = EnergyModelConfigMutation::ChangeResultField(ChangeResultField { field: "solarAbsorbed".into() });
    let law = author_through_config_store::<EditorApp<EnergyModelEditor>>(mutation.clone()).await;
    assert_eq!(law.applied.result_field, "solarAbsorbed");
    assert_eq!(law.undone.result_field, base.result_field, "undo restores the displaced field");
    assert_eq!(law.redone.result_field, "solarAbsorbed");
    assert_eq!((law.forward_rows.len(), law.inverse_rows.len()), (1, 1));
    assert_row_is_the_serde_document(&law.forward_rows[0], &mutation);
    assert_row_is_the_serde_document(&law.inverse_rows[0], &EnergyModelConfigMutation::ChangeResultField(ChangeResultField { field: base.result_field }));
}
