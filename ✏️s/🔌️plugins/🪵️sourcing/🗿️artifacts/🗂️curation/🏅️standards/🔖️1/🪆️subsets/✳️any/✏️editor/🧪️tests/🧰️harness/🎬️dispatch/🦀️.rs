//! 🎬️ The dispatch of the sourcing editor tests: commands, wire actions, settlement, archive loads and media export through the mounted app, instantiated here once.
use semio_s_artifact_sourcing_curation::editor::sourcing::SourcingCurationCommand;
use semio_plugin::artifact_app_laws::{meta, OriginalFixtureCaller, settle_registered_typed_operation, dispatch_original_fixture_command, dispatch_fixture_action, admit_document_archive_load, settle_document_archive_load, observe_registered_fixture_app, render_registered_fixture_app, fixture_identity};
use semio_plugin::{ActionMeta, InvocationResult, MountedOwnerPolicyV1, ViewModel};
use semio_s_artifact_sourcing_curation_harness_mount::{SourcingTestApp, SourcingApp};
use semio_os_kernel as protocol;
use semio_plugin::PluginApp;

/// 🎬️ Dispatches one typed command and drives its retained operation to publication, the way the
/// host does — a migrated command's document and config edits land only once it settles.
pub async fn dispatch(app: &mut SourcingTestApp, command: SourcingCurationCommand) -> InvocationResult {
    let result = match &command {
        SourcingCurationCommand::SetContributions(payload) => {
            let address = semio_framework::manifest::CommandAddress { owner: semio_framework::manifest::CommandOwnerAddress::App { plugin_id: "sourcing".into(), app_id: "s.sourcing.curation@1/*#editor".into() }, command_id: "setContributions".into() };
            let arguments = Some(semio_framework_value::DslValue::object([("json".to_owned(), semio_framework_value::DslValue::String(payload.json.clone()))]));
            semio_plugin::artifact_app_laws::dispatch_fixture_command(&mut app.0, &address, arguments, None, &meta("local"), &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance).await.expect("dispatch")
        }
        _ => semio_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app.0, &command, &meta("local"), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("dispatch"),
    };
    settle(app).await;
    result
}

/// 🌐️ Dispatches one UI action through the action endpoint with the JSON arguments the browser host delivers, then settles it.
pub async fn dispatch_wire(app: &mut SourcingTestApp, action: &str, arguments: &str) -> Result<InvocationResult, semio_framework_job::OriginalTurnFailure> {
    let result = dispatch_wire_unsettled(app, action, arguments).await;
    settle(app).await;
    result
}

/// 🌐️ Dispatches one UI action like [`dispatch_wire`] but leaves its typed operation unsettled, as the browser does: the dispatch answers its operation id at once and the next wire message can arrive while the operation still publishes.
pub async fn dispatch_wire_unsettled(app: &mut SourcingTestApp, action: &str, arguments: &str) -> Result<InvocationResult, semio_framework_job::OriginalTurnFailure> {
    let args = Some(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(arguments, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("wire json")));
    semio_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, action, args, &meta("local"), &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance).await
}

/// 🏁️ Drains every pending typed operation for instance 1, returning the document effects it published.
pub async fn settle(app: &mut SourcingTestApp) -> Vec<semio_framework::kernel::Effect> {
    let receipt = semio_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app.0, 1, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("complete original typed operation settlement");
    receipt.effects
}

pub async fn drain_auxiliary_outboxes(app: &mut SourcingApp) {
    while app.take_typed_operation_ui_progress().is_some() {}
    while app.take_typed_operation_composed_result().is_some() {}
    while app.take_typed_operation_completion().await.expect("typed operation completion").is_some() {
    }
    while let Some(reply) = app.take_local_interaction_query_reply() {
        if let protocol::LocalInteractionQueryReply::Page { page } = reply {
            let token = protocol::LocalInteractionQueryToken { request_id: page.request_id, query_generation: page.query_generation, identity: page.identity.clone(), ordinal: page.ordinal };
            assert!(app.acknowledge_local_interaction_query(&token), "exact local interaction query acknowledgement");
        }
    }
}

/// 📤️ One media port exported through the app's original caller, its pending turns driven until the port answers.
pub async fn export_media(app: &mut SourcingTestApp, port: &str) -> semio_plugin::Media {
    for _ in 0..semio_plugin::artifact_app_laws::OriginalFixtureCaller::maximum_turns() {
        if let Some(media) = app.0.export_media(port, app.1.cx()).await.expect("media export turn") {
            return media;
        }
    }
    panic!("media export of {port} stayed pending under its original caller");
}

/// 🎬️ Dispatches one typed command under the given action meta and leaves its typed operation unsettled.
pub async fn dispatch_meta(app: &mut SourcingTestApp, command: &SourcingCurationCommand, meta: &ActionMeta) -> InvocationResult {
    dispatch_original_fixture_command(&mut app.0, command, meta, MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut fixture_identity(), app.1.cx()).await.expect("dispatch")
}

/// 🏁️ Drains the typed operation of one instance and returns its receipt.
pub async fn settle_instance(app: &mut SourcingTestApp, instance_id: u32) -> semio_plugin::artifact_app_laws::TypedOperationFixtureReceipt {
    settle_registered_typed_operation(&mut app.0, instance_id, MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut fixture_identity(), app.1.cx()).await.expect("complete original typed operation settlement")
}

/// 🗄️ Admits one document archive as a load and settles it to ready.
pub async fn load_archive(app: &mut SourcingTestApp, operation: u64, archive: protocol::DocumentArchivePack) {
    admit_document_archive_load(&mut app.0, operation, archive, app.1.cx()).expect("archive admission");
    settle_document_archive_load(&mut app.0, operation, &mut fixture_identity(), app.1.cx()).await.expect("archive load settles Ready");
}

/// ↩️ Dispatches the framework undo action and settles its reserved job.
pub async fn undo(app: &mut SourcingTestApp) {
    let admitted = dispatch_fixture_action(&mut app.0, "undo", None, &meta("local"), &mut fixture_identity(), app.1.cx(), MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.expect("undo");
    semio_plugin::app::settle_framework_reserved_admission(&mut app.0, admitted, app.1.cx()).await.expect("undo settles its reserved job");
}
