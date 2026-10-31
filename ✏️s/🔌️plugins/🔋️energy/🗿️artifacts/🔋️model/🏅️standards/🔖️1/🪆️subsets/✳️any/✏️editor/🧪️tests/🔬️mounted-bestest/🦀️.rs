//! 🪟️ Mounted editor pane laws over the bundled BESTEST case 600: it loads into the native owner, one action applies natively and undoes and redoes through history.
#![cfg(test)]

use semio_framework_os_kernel::{os_store, ActorId, LOCAL_ACTOR_ID};
use semio_framework_plugin::artifact_app_laws as laws;
use semio_framework_plugin::{App, DslValue, EditorApp, MountedOwnerPolicyV1, PluginApp as _, VcsArtifactApp, ViewModel};
use semio_framework_ui_locale::{Locale, Terminology};
use semio_s_artifact_energy_model_editor::{create_energy_model_editor, examples, EnergyModelEditor};
use semio_s_artifact_energy_model_editor_commands::SET_NODE_ACTION_ID;
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
    let model = semio_s_plugin_energy_engine_bestest::model("600").expect("case 600 is registered");
    let snapshot = EnergyModelSnapshot::from_model(&model).expect("case 600 is an admissible native document");
    let pack = <EnergyModelSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&snapshot);
    let envelope = os_store::document_materialize::create_document_envelope::<EnergyModelSnapshot, EnergyModelMutation>(ENERGY_MODEL_DOCUMENT_SCHEMA, "model", snapshot, None, &semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).into_owners();
    let spr = os_store::text_format::print_document_spr(&envelope).await.expect("case 600 spr");
    laws::load_document(&mut pane.0, &os_store::serialization::ArtifactPackFiles { pack, spr, ops: String::new() }, &mut laws::fixture_identity(), pane.1.cx()).await.expect("case 600 loads");
    pane
}

fn model_name(pane: &Pane) -> String {
    pane.0.snapshot().expect("projection").model.read().name.clone()
}

async fn rename(pane: &mut Pane, name: &str) {
    let args = DslValue::Object(vec![("id".to_string(), DslValue::String("name".to_string())), ("value".to_string(), DslValue::String(name.to_string()))]);
    laws::dispatch_fixture_action(&mut pane.0, SET_NODE_ACTION_ID, Some(args), &laws::meta("local"), &mut laws::fixture_identity(), pane.1.cx(), MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.expect("the rename is admitted");
    laws::settle_registered_typed_operation(&mut pane.0, laws::meta("local").instance_id, laws::fixture_mounted_policy(), &mut laws::fixture_identity(), pane.1.cx()).await.expect("the rename settles");
}

async fn history(pane: &mut Pane, verb: &str) {
    laws::settle_history_verb(&mut pane.0, verb, laws::meta("local").instance_id, laws::fixture_mounted_policy(), &mut laws::fixture_identity(), pane.1.cx()).await;
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

/// ⚖️ LAW: one action edits the native document exactly once and history undoes and redoes it natively.
#[semio_framework_async_macros::async_test]
async fn one_action_applies_natively_and_undoes_and_redoes() {
    let mut pane = bestest_pane().await;
    let original = model_name(&pane);
    rename(&mut pane, "Mounted rename").await;
    assert_eq!(model_name(&pane), "Mounted rename");
    history(&mut pane, "undo").await;
    assert_eq!(model_name(&pane), original);
    history(&mut pane, "redo").await;
    assert_eq!(model_name(&pane), "Mounted rename");
    laws::close_registered_fixture_app(&mut pane.0, laws::fixture_mounted_policy(), pane.1.cx()).expect("original caller paid app close");
}
