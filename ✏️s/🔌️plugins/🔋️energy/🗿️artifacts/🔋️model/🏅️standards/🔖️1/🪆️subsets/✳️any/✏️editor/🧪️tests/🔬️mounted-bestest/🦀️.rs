//! 🪟️ Mounted editor pane laws over the bundled BESTEST case 600: it loads into the native owner, one action applies natively and undoes and redoes through history in en and de.
#![cfg(test)]

#[global_allocator]
static MOUNTED_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

use semio_os_kernel::{os_store, ActorId, LOCAL_ACTOR_ID};
use semio_plugin::artifact_app_laws as laws;
use semio_plugin::{App, DslValue, EditorApp, MountedOwnerPolicyV1, PluginApp as _, VcsArtifactApp, ViewModel};
use semio_ui_locale::{Locale, Terminology};
use semio_s_artifact_energy_model_editor::{create_energy_model_editor, examples, EnergyModelEditor};
use semio_s_artifact_energy_model_editor_commands::EnergyModelEditorCommand;
use semio_s_artifact_energy_model_mutations::EnergyModelMutation;
use semio_s_artifact_energy_model_snapshot::{EnergyModelSnapshot, ENERGY_MODEL_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_semio::SemioMembers;

type Mounted = VcsArtifactApp<EditorApp<EnergyModelEditor>, SemioMembers>;

fn manifest() -> App {
    App { definition: create_energy_model_editor(), examples: examples().into_iter().map(Into::into).collect() }
}

struct Pane(Mounted, laws::OriginalFixtureScope);

/// 📒️ A registry-backed editor mounted on the live instance the fixture actor addresses, holding the bundled case 600.
async fn bestest_pane() -> Pane {
    let scope = laws::OriginalFixtureScope::admit();
    let mut app = laws::new_app_with_registry_and_members::<EditorApp<EnergyModelEditor>, SemioMembers>(manifest, ActorId(LOCAL_ACTOR_ID.into()), laws::fixture_mounted_policy(), &mut laws::fixture_identity()).await;
    app.bind_instance_id(laws::meta("local").instance_id).await;
    let mut pane = Pane(app, scope);
    let receiver = laws::meta("local").instance_id;
    let args = DslValue::Object(vec![("exampleId".to_string(), DslValue::String("bestest-600".to_string()))]);
    laws::dispatch_fixture_action(&mut pane.0, "setActiveExample", Some(args), &laws::meta("local"), &mut laws::fixture_identity(), pane.1.cx(), MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.expect("dispatch the load-example action");
    let published = laws::settle_registered_typed_operation(&mut pane.0, receiver, laws::fixture_mounted_policy(), &mut laws::fixture_identity(), pane.1.cx()).await.expect("load-example publication");
    let (pack, spr) = published.effects.iter().find_map(|effect| match effect {
        semio_plugin::plugin_app_close_prelude::Effect::LoadDocument { pack, spr } => Some((pack.clone(), spr.clone())),
        _ => None,
    }).expect("the load-example command publishes a LoadDocument effect");
    laws::load_document(&mut pane.0, &os_store::serialization::ArtifactPackFiles { pack, spr, ops: String::new() }, &mut laws::fixture_identity(), pane.1.cx()).await.expect("case 600 crosses the archive door");
    pane
}

fn model_name(pane: &Pane) -> String {
    pane.0.snapshot().expect("projection").model.read().name.clone()
}

async fn render(pane: &mut Pane, body_key: &str) -> String {
    laws::render_registered_fixture_app(&mut pane.0, body_key, &ViewModel::new(Locale::En, Terminology::Native), pane.1.cx()).await.unwrap_or_else(|fault| panic!("render {body_key}: {fault:?}"))
}

/// 🧪️ LAW: the bundled case 600 loads into the native owner and every pane body renders from it without a projection copy.
#[semio_framework_async_macros::async_test]
async fn the_bundled_bestest_case_loads_and_every_pane_body_renders() {
    let mut pane = bestest_pane().await;
    let expected = semio_s_plugin_energy_engine_bestest::model("600").expect("case 600 is registered").name;
    assert_eq!(model_name(&pane), expected);
    for body in [
        "energy.model.artifact",
        "energy.model.inspection",
    ] {
        assert!(!render(&mut pane, body).await.is_empty(), "{body} renders");
    }
    laws::close_registered_fixture_app(&mut pane.0, laws::fixture_mounted_policy(), pane.1.cx()).expect("original caller paid app close");
}

/// ⚖️ LAW: one action edits the native document exactly once and history undoes, redoes and undoes it again natively, in en and de, with the artifact tree carrying that language's catalog strings.
#[semio_framework_async_macros::async_test]
async fn one_action_applies_natively_and_undoes_and_redoes() {
    for locale in laws::PANE_SMOKE_LOCALES {
        let mut scope = laws::OriginalFixtureScope::admit();
        let mut app = laws::new_app_with_registry_and_members::<EditorApp<EnergyModelEditor>, SemioMembers>(manifest, ActorId(LOCAL_ACTOR_ID.into()), laws::fixture_mounted_policy(), &mut laws::fixture_identity()).await;
        app.bind_instance_id(laws::meta("local").instance_id).await;
        let language = laws::PaneSmokeLanguage { body_key: "energy.model.artifact", catalog: vec![semio_s_artifact_energy_model_editor_panels::artifact::site_label(locale).into()], view: ViewModel::new(locale, Terminology::Native) };
        let example = EnergyModelEditorCommand::SetActiveExample { example_id: "bestest-600".into() };
        let command = EnergyModelEditorCommand::SetStructureField { field: "name".into(), value: "Mounted rename".into() };
        let (loaded, applied) = laws::assert_pane_smoke_by_example(&mut app, &language, example, command, |app| app.snapshot().expect("projection").model.read().name.clone(), laws::fixture_mounted_policy(), &mut laws::fixture_identity(), scope.cx()).await;
        assert_eq!(loaded, semio_s_plugin_energy_engine_bestest::model("600").expect("case 600 is registered").name);
        assert_eq!(applied, "Mounted rename");
    }
}
