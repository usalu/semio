//! 🔁️ A resolved task follow-up is queued on the mounted puzzle3d editor and delivered by the instance across funded publication units through its own typed source: it applies exactly once, is recorded as an undoable edit, answers its task once, a refused delivery answers with the fault, and a cancelled queue returns every owner.

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;

use semio_plugin::artifact_app_laws::{close_registered_fixture_app, fixture_identity, meta, new_app_with_registry, settle_history_verb, settle_registered_typed_operation, OriginalFixtureCaller, OriginalFixtureScope};
use semio_plugin::{App, ArtifactApp, EditorApp, MountedOwnerPolicyV1, PluginApp, VcsArtifactApp};
use semio_s_artifact_puzzle_3d_editor::{create_puzzle3d_app, Puzzle3dPlayApp};

type Puzzle3d = EditorApp<Puzzle3dPlayApp>;
type Puzzle3dMounted = VcsArtifactApp<Puzzle3d>;

const POLICY: MountedOwnerPolicyV1 = MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;


/// 🪟️ The puzzle3d window commands require the host's per-window view state, exactly as the browser host supplies it.
fn window_view() -> semio_plugin::ViewModel {
    semio_plugin::ViewModel { window_id: Some("puzzle3d-main".into()), window_instances: vec![semio_plugin::ViewWindowInstance { id: "puzzle3d-main".into(), window_kind_id: "puzzle3d-main".into() }], ..semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native) }
}

macro_rules! window_meta {
    ($actor:expr) => {{
        let mut window_meta = meta($actor);
        window_meta.view_state = Some(window_view());
        window_meta
    }};
}

/// 🔒️ The worker-job retirement slots are process-global: a parked retirement of ANY mounted app keeps every other app's maintenance non-terminal, so mounted laws of one binary run one at a time.
static SERIAL: std::sync::Mutex<()> = std::sync::Mutex::new(());

async fn mounted() -> Box<Puzzle3dMounted> {
    let mut app = Box::new(new_app_with_registry::<Puzzle3d>(|| App { definition: create_puzzle3d_app(), examples: Vec::new() }, semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), POLICY, &mut fixture_identity()).await);
    app.bind_instance_id(1).await;
    app
}

fn target_volumes(app: &Puzzle3dMounted) -> usize {
    semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(app.snapshot().expect("snapshot").typed())).get("targetVolumes").and_then(|volumes| volumes.as_array()).map_or(0, Vec::len)
}

async fn settle(app: &mut Puzzle3dMounted, scope: &mut OriginalFixtureScope) -> semio_plugin::artifact_app_laws::TypedOperationFixtureReceipt {
    match settle_registered_typed_operation(&mut *app, 1, POLICY, &mut fixture_identity(), scope.cx()).await {
        Ok(receipt) => receipt,
        Err(error) => {
            panic!("the registered typed operation settles: {error:?}")
        }
    }
}

async fn follow_up(x: f64) -> Vec<u8> {
    let mut args = Some(serde_json::json!({ "origin": [x, 2.0, 3.0] }).into());
    let command = <Puzzle3d as ArtifactApp>::command_from_action("addTargetVolume", &mut args, &mut None).await.expect("addTargetVolume births its typed command");
    <<Puzzle3d as ArtifactApp>::Command as protocol::OpBinary>::encode_op(&command).expect("the follow-up command encodes")
}

#[semio_framework_async_macros::async_test]
async fn a_queued_task_resume_is_delivered_across_funded_turns_applies_once_and_undoes() {
    let _serial = SERIAL.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let mut scope = OriginalFixtureScope::admit();
    let mut app = mounted().await;
    let before = target_volumes(&app);
    let task_meta = window_meta!("local");

    app.queue_task_resume_command(follow_up(1.0).await, &task_meta).expect("the follow-up queues");
    assert!(app.has_runnable_typed_operations(), "a queued follow-up keeps the actor's turn armed");
    assert_eq!(target_volumes(&app), before, "queueing applies nothing");

    let mut units = 0;
    let outcome = loop {
        assert!(units < OriginalFixtureCaller::maximum_turns(), "the follow-up never settled");
        app.advance_typed_operation_publication(&mut fixture_identity(), scope.cx()).await.expect("one funded publication unit");
        units += 1;
        if let Some(outcome) = app.take_task_resume_outcome() { break outcome; }
    };
    assert!(units > 1, "the delivery spans several funded units, got {units}");
    outcome.expect("the follow-up mounted");
    assert!(app.take_task_resume_outcome().is_none(), "the follow-up answers exactly once");

    let receipt = settle(&mut app, &mut scope).await;
    assert_eq!(target_volumes(&app), before + 1, "the follow-up applied exactly once");
    assert!(receipt.resume_outcomes.is_empty(), "settling after the answer was claimed owes no second answer");
    settle_history_verb(&mut *app, "undo", task_meta.instance_id, POLICY, &mut fixture_identity(), scope.cx()).await;
    assert_eq!(target_volumes(&app), before, "the follow-up is an ordinary undoable edit");
    close_registered_fixture_app(&mut *app, POLICY, scope.cx()).expect("original caller paid app close");
}

#[semio_framework_async_macros::async_test]
async fn a_refused_task_resume_answers_with_a_fault_and_the_next_one_still_delivers() {
    let _serial = SERIAL.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let mut scope = OriginalFixtureScope::admit();
    let mut app = mounted().await;
    let before = target_volumes(&app);
    let mut foreign = window_meta!("local");
    foreign.instance_id = 99;

    app.queue_task_resume_command(follow_up(1.0).await, &foreign).expect("the follow-up queues");
    app.queue_task_resume_command(follow_up(2.0).await, &window_meta!("local")).expect("the follow-up queues");
    let mut answers = Vec::new();
    for _ in 0..OriginalFixtureCaller::maximum_turns() {
        if !app.has_pending_typed_operations() { break; }
        app.advance_typed_operation_publication(&mut fixture_identity(), scope.cx()).await.expect("a refused delivery is an answer, not a failed turn");
        if let Some(outcome) = app.take_task_resume_outcome() { answers.push(outcome); }
        settle(&mut app, &mut scope).await;
    }
    assert_eq!(answers.len(), 2, "every queued follow-up is answered, in order");
    let fault = answers[0].as_ref().expect_err("a follow-up for a foreign instance is refused");
    assert!(fault.message.contains("original mounted app instance"), "{}", fault.message);
    assert!(answers[1].is_ok(), "the lane keeps serving the follow-ups behind a refused one");
    assert_eq!(target_volumes(&app), before + 1, "only the delivered follow-up applied");
    close_registered_fixture_app(&mut *app, POLICY, scope.cx()).expect("original caller paid app close");
}

#[semio_framework_async_macros::async_test]
async fn a_cancelled_task_resume_queue_returns_every_owner_after_any_number_of_turns() {
    let _serial = SERIAL.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    for cut in [0, 1, 2, 4, 6] {
        let mut scope = OriginalFixtureScope::admit();
        let mut app = mounted().await;
        let before = target_volumes(&app);
        for x in [1.0, 2.0] {
            app.queue_task_resume_command(follow_up(x).await, &window_meta!("local")).expect("the follow-up queues");
        }
        for _ in 0..cut {
            app.advance_typed_operation_publication(&mut fixture_identity(), scope.cx()).await.expect("one funded publication unit");
            drop(app.take_task_resume_outcome());
        }
        scope.cx().original_cancel_token().cancel_now();
        for _ in 0..OriginalFixtureCaller::maximum_turns() {
            if app.take_task_resume_outcome().is_none() && !app.has_pending_typed_operations() { break; }
            app.advance_typed_operation_publication(&mut fixture_identity(), scope.cx()).await.expect("a cancelled unit retires, it does not fail");
        }
        assert!(app.host_events.resumes.is_empty() && !app.original_typed_command.pending(), "cancel after {cut} units left the queue or the typed source owned");
        assert!(app.take_task_resume_outcome().is_none(), "cancel after {cut} units answered a cancelled follow-up");
        std::mem::forget(app);
    }
}

/// 🧹️ An idle mounted editor owes no maintenance: the ladder's terminal witness is `true` before any work, so a settle law's `while pending || !maintenance_terminal` loop ends (it used to spin forever on the private child group registry's allocation, which only the close ladder returns).
#[semio_framework_async_macros::async_test]
async fn a_freshly_mounted_editor_owes_no_maintenance() {
    let _serial = SERIAL.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let mut scope = OriginalFixtureScope::admit();
    let mut app = mounted().await;
    assert!(app.maintenance_terminal_is_empty(), "a fresh mounted editor is maintenance-terminal");
    assert!(!app.has_pending_typed_operations() && !app.has_runnable_typed_operations());
    close_registered_fixture_app(&mut *app, POLICY, scope.cx()).expect("original caller paid app close");
}
