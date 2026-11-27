//! 🪧️ Aussuchen pane smoke: the bundled curation demo loads, curates, undoes, redoes and closes in en and de; the law is instantiated in its own library, this target only runs it.

#[semio_framework_async_macros::async_test]
async fn aussuchen_pane_loads_applies_undoes_and_closes() {
    semio_s_artifact_sourcing_curation_pane_smoke::aussuchen_pane_loads_applies_undoes_and_closes().await;
}
