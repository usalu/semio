// 🧵️ HUB WP7: mount hub of WP7 instance tier 2 and 3, builder, testing: instance-commands, instance-typed-ops, instance-render, instance-ladders, time-travel-turns, tool-run-turns, instance-app, builder, testing-laws, testing-acceptance and the root test fixtures.
// This file is spliced into the plugin crate root by `include!` from the root, so every item below stays a crate-root item;
// its `#[path]` literals resolve against this folder (`🪝️mounts/<hub>/`), hence the `../../` prefix of the plugin folder.
// Carving a crate removes its unit here and puts `extern crate <crate> as <old module name>;` in its place (PLAN-PLUGIN-CRATES.md, section 12.5).

//#region 🏛️OwnershipFacet
#[cfg(any(test, feature = "artifact-app-testing"))]
pub use semio_plugin_testing_acceptance::ui_action_laws as ui_action_laws;
/// 🧬️ Declares an exact bounded-first-step proof catalog in the owning plugin source.
pub use semio_plugin_decl_app::bounded_first_step_tool_proofs;
#[cfg(any(test, feature = "artifact-app-testing"))]
pub use semio_plugin_testing_acceptance::{composed_child_history_law, composed_reload_law, history_edit_acceptance_law};
//#endregion 🏛️OwnershipFacet
//#region 📄️DeclarationFixtureMutationMount

//#endregion 📄️DeclarationFixtureMutationMount
//#region 🏛️OwnershipFacet
#[cfg(test)]
pub(crate) use semio_plugin_testing_fixtures_publication::publication_fixture as publication_fixture;
#[cfg(test)]
pub(crate) use semio_plugin_testing_fixtures::test_app_mutation_fixture as test_app_mutation_fixture;
#[cfg(test)]
pub(crate) use semio_plugin_testing_fixtures::contributed_mutation_wire as contributed_mutation_wire;
//#endregion 🏛️OwnershipFacet
//#region 🧵️InstanceFacet
pub(crate) extern crate semio_plugin_instance_app as artifact_instance;
pub(crate) extern crate semio_plugin_builder as builder;
//#endregion 🧵️InstanceFacet
//#region ⚖️AppLaws
#[cfg(any(test, feature = "artifact-app-testing"))]
pub(crate) mod app_testing {
    pub use semio_plugin_testing_laws::*;
}
#[cfg(any(test, feature = "artifact-app-testing"))]
/// 🪟️ Drives a persisted window envelope through its registered mounted owner on the fixture's original turns.
pub async fn restore_fixture_window_config<P: crate::PluginApp>(
    app: &mut P,
    pack: semio_plugin_window_config::WindowConfigPack,
    cx: &mut semio_framework_job::StepContext<'_>,
) -> Result<(), semio_framework_job::OriginalTurnFailure> {
    use semio_plugin_testing_laws::artifact_app_laws::{fixture_renew_turn, OriginalFixtureCaller};
    use semio_plugin_window_config::WindowConfigPackSource;
    let envelope_pack = match pack.source {
        WindowConfigPackSource::Pair(files) => semio_os_kernel::os_store::serialization::encode_document_pack_bytes(&files.pack, &files.spr).await,
        WindowConfigPackSource::Framed(bytes) => bytes,
    };
    let mut entry = semio_os_kernel::WindowConfigPackEntry { window_id: pack.window_id, window_kind_id: pack.window_kind_id, envelope_pack };
    let mut load = Default::default();
    let grant = cx.retained_grant();
    for _ in 0..OriginalFixtureCaller::maximum_turns() {
        fixture_renew_turn(cx, OriginalFixtureCaller::turn_fuel(), grant)?;
        if app.advance_window_config_load(&mut load, &mut entry, cx)? { return Ok(()); }
    }
    Err(semio_framework_value::ValueError::invariant("the window config restore exceeded its fixture turn bound").into())
}
//#endregion ⚖️AppLaws











