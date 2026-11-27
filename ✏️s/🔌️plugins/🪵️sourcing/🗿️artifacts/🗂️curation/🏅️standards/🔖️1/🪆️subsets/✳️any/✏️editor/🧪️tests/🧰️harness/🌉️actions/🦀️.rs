//! 🌉️ The action-bridge law of the sourcing editor tests: every declared UI action reaches its command, instantiated here once.
use semio_plugin::EditorApp;
use semio_s_artifact_sourcing_curation::editor::sourcing::SourcingCurationApp;
use semio_s_artifact_sourcing_curation_harness_mount::sourcing_manifest_for_tests;

/// 🌉️ Every declared action of the sourcing curation editor bridges to a command.
pub async fn assert_declared_actions_bridge_to_commands() {
    semio_plugin::artifact_app_laws::assert_declared_actions_bridge_to_commands::<EditorApp<SourcingCurationApp>>(sourcing_manifest_for_tests).await;
}
