//! 🪧️ Aussuchen pane smoke: the bundled curation demo loads, one `CurationAdd` applies, undo restores the loaded snapshot and the app closes through its exact retained close.

use semio_framework_plugin::artifact_app_laws::{self as laws, OriginalFixtureScope};
use semio_framework_plugin::{App, EditorApp, MountedOwnerPolicyV1, PluginApp as _, VcsArtifactApp};
use semio_s_artifact_sourcing_curation::editor::sourcing::commands::curation_add;
use semio_s_artifact_sourcing_curation::editor::sourcing::{create_sourcing_curation_app, SourcingCurationApp, SourcingCurationCommand};
use semio_s_artifact_stdio_semio::SemioMembers;

type Mounted = VcsArtifactApp<EditorApp<SourcingCurationApp>, SemioMembers>;

fn manifest() -> App {
    App { definition: create_sourcing_curation_app(), examples: Vec::new() }
}

/// 🪵️ LAW: the Aussuchen pane loads its bundled demo, curates one object, undoes it and closes with the store drop witness satisfied.
#[semio_framework_async_macros::async_test]
async fn aussuchen_pane_loads_applies_undoes_and_closes() {
    let mut scope = OriginalFixtureScope::admit();
    let mut app: Mounted = laws::new_app_with_registry_and_members::<EditorApp<SourcingCurationApp>, SemioMembers>(manifest, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut laws::fixture_identity()).await;
    app.bind_instance_id(laws::meta("local").instance_id).await;
    let object_id = semio_s_artifact_sourcing_curation::stock_of(&app.snapshot().expect("boot snapshot"))[0].id.clone();
    let command = SourcingCurationCommand::CurationAdd(curation_add::CurationAdd { object_id: object_id.clone() });
    let (loaded, applied) = laws::assert_pane_smoke(&mut app, semio_s_artifact_sourcing_curation::standards::v1::subsets::any::io::text::snapshot::DEMO_STOCK_TEXT, command, |app| app.snapshot().expect("snapshot"), MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut laws::fixture_identity(), scope.cx()).await;
    assert!(!loaded.stock_extra.is_empty(), "the loaded demo carries its stock");
    assert_eq!(semio_s_artifact_sourcing_curation_domain::curated_count(&loaded, &object_id), 0);
    assert_eq!(semio_s_artifact_sourcing_curation_domain::curated_count(&applied, &object_id), 1);
}
