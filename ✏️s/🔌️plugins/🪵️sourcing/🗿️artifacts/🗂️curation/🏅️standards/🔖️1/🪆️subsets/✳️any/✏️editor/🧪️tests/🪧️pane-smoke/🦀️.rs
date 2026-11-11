//! 🪧️ Aussuchen pane smoke: the bundled curation demo loads, one `CurationAdd` applies, undo, redo and a second undo restore it and the app closes, in en and de, through its exact retained close.


use semio_plugin::artifact_app_laws::{self as laws, OriginalFixtureScope};
use semio_plugin::{App, EditorApp, MountedOwnerPolicyV1, PluginApp as _, VcsArtifactApp};
use semio_s_artifact_sourcing_curation::editor::sourcing::modes::edit::windows::pool;
use semio_s_artifact_sourcing_curation::editor::sourcing::terminology::sourcing_curation_labels;
use semio_s_artifact_sourcing_curation::editor::sourcing::commands::{curation_add, set_active_example};
use semio_s_artifact_sourcing_curation::editor::sourcing::{create_sourcing_curation_app, SourcingCurationApp, SourcingCurationCommand};
use semio_s_artifact_stdio_semio::SemioMembers;

type Mounted = VcsArtifactApp<EditorApp<SourcingCurationApp>, SemioMembers>;

fn manifest() -> App {
    App { definition: create_sourcing_curation_app(), examples: Vec::new() }
}

/// 🪵️ LAW: the Aussuchen pane loads its bundled demo, curates one object, undoes, redoes and closes in en and de with the store drop witness satisfied.
#[semio_framework_async_macros::async_test]
async fn aussuchen_pane_loads_applies_undoes_and_closes() {
    for locale in laws::PANE_SMOKE_LOCALES {
        let mut scope = OriginalFixtureScope::admit();
        let mut app: Mounted = laws::new_app_with_registry_and_members::<EditorApp<SourcingCurationApp>, SemioMembers>(manifest, semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut laws::fixture_identity()).await;
        app.bind_instance_id(laws::meta("local").instance_id).await;
        let boot = app.snapshot().expect("boot snapshot");
        let object_id = semio_s_artifact_sourcing_curation::stock_of(&boot)[0].id.clone();
        assert!(matches!(semio_s_artifact_sourcing_curation::schema::curation_decision_for_delta(&boot, &object_id, 1), semio_s_artifact_sourcing_curation::schema::CurationDecision::Create(_)), "the first stock row must be curatable in the boot document, else the handler emits nothing and the apply is a no-op by construction");
        let view = semio_plugin::ViewModel::new(locale, semio_ui_locale::Terminology::Native);
        let labels = sourcing_curation_labels(&view);
        let language = laws::PaneSmokeLanguage { body_key: pool::SOURCING_CURATION_BODY_POOL, catalog: vec![labels.restock.as_str().into(), labels.col_name.as_str().into()], view };
        let command = SourcingCurationCommand::CurationAdd(curation_add::CurationAdd { object_id: object_id.clone() });
        let example = SourcingCurationCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: semio_s_artifact_sourcing_curation::editor::sourcing::editor_vocabulary::DEMO_STOCK_EXAMPLE_ID.into() });
        let (loaded, applied) = laws::assert_pane_smoke_by_example(&mut app, &language, example, command, |app| app.snapshot().expect("snapshot"), MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut laws::fixture_identity(), scope.cx()).await;
        assert!(!loaded.stock_extra.is_empty(), "the loaded demo carries its stock");
        assert_eq!(semio_s_artifact_sourcing_curation::schema::curated_count(&loaded, &object_id), 0);
        assert_eq!(semio_s_artifact_sourcing_curation::schema::curated_count(&applied, &object_id), 1);
    }
}
