//! 🤖️ The agent lane, the composed history laws and the foreign carrier archive of the editor laws. See [`super`].

use super::*;

//#region 🤖️AgentLaneLaws
/// 🤖️ LAW: an agent — no window, no selection — names the nodes `patchNodes` renames in its declared `nodeIds`
/// argument, published as `entityId`s of the `ast/node` granularity. Through the agent lane (the MCP gateway's
/// `action_prepare`, `artifact_app_laws::probe_agent_lane`) the named node is renamed exactly as the shell renames it;
/// without `nodeIds` the agent is refused by name with `app.command.targets-required`, as the shell is with nothing
/// selected. The rail keeps its text field: empty still means the selection.
#[semio_framework_async_macros::async_test]
async fn an_agent_names_the_nodes_patch_nodes_renames_and_is_refused_by_name_without_them() {
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    use semio_framework_plugin::artifact_app_laws::{declared_verb_agent_divergences, declared_verb_probe, declared_verb_staged_wrote_document, declared_verb_wrote_document, probe_agent_lane, DeclaredVerbOutcome};
    let definition = create_trinity_jack_app();
    let patch = definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter())).find(|action| action.id == "patchNodes").expect("declared");
    let node_ids = patch.args.iter().find(|arg| arg.id == "nodeIds").expect("nodeIds");
    assert!(!node_ids.required, "the rail may leave nodeIds empty to act on the selection");
    let schema = node_ids.json_schema(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En);
    let items = schema.get("items").expect("nodeIds is a list");
    assert_eq!(
        (schema.get("type").and_then(semio_framework_plugin::DslValue::as_str), items.get("x-semio-format").and_then(semio_framework_plugin::DslValue::as_str), items.get("x-semio-entity-kind").and_then(semio_framework_plugin::DslValue::as_str)),
        (Some("array"), Some("entityId"), Some("ast/node"))
    );
    let first = jack::standards::v1::subsets::any::schema::snapshot::jack_working_scene(&SemioJackComposition::initial_snapshot()).expect("curated example scene").nodes[0].id.clone();
    let named = probe_agent_lane::<EditorApp<TrinityJackPlayApp<SemioJackComposition>>, semio_s_artifact_stdio_semio::SemioMembers>(
        create_trinity_jack_app,
        Some(&format!(r#"{{"verbs":{{"patchNodes":{{"nodeIds":["{first}"],"field":"name","value":"Agent Named"}}}}}}"#)),
        &["patchNodes"],semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()
    )
    .await;
    let probe = declared_verb_probe(&named, "patchNodes");
    assert!(declared_verb_wrote_document(probe.agent.as_ref().expect("patchNodes is agent-facing")), "the agent's preview renames the named node: {:?}", probe.agent);
    assert!(declared_verb_staged_wrote_document(probe), "the shell renames the named node too");
    let unnamed = probe_agent_lane::<EditorApp<TrinityJackPlayApp<SemioJackComposition>>, semio_s_artifact_stdio_semio::SemioMembers>(create_trinity_jack_app, Some(r#"{"verbs":{"patchNodes":{"field":"name","value":"Agent Unnamed"}}}"#), &["patchNodes"], semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
    match declared_verb_probe(&unnamed, "patchNodes").agent.as_ref() {
        Some(DeclaredVerbOutcome::Refused { code, .. }) => assert_eq!(code, "app.command.targets-required"),
        other => panic!("an agent naming no node is refused by name: {other:?}"),
    }
    assert_eq!((declared_verb_agent_divergences(&named), declared_verb_agent_divergences(&unnamed)), (Vec::<String>::new(), Vec::<String>::new()), "agent-lane divergences");

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}
//#endregion 🤖️AgentLaneLaws

semio_framework_plugin::history_edit_acceptance_law!("trinity", super::super::TrinityJackPlayApp, || semio_framework_plugin::App { definition: super::super::create_trinity_jack_app(), examples: Vec::new() }, "../../🏅️standards/🔖️1/🪆️subsets/✳️any");

semio_framework_plugin::composed_reload_law!("trinity", super::super::TrinityJackPlayApp, || semio_framework_plugin::App { definition: super::super::create_trinity_jack_app(), examples: Vec::new() }, "../../🏅️standards/🔖️1/🪆️subsets/✳️any");

semio_framework_plugin::composed_child_history_law!("trinity", super::super::TrinityJackPlayApp, || semio_framework_plugin::App { definition: super::super::create_trinity_jack_app(), examples: Vec::new() }, [("patchNodes", r#"{"nodeIds":["7dc5b737-3b6b-4068-b315-b7bacc91c2e1"],"field":"name","value":"Renamed core"}"#)]);

/// 📦️ A real edited registered Jack document restores its exact parent, histories and rich member bytes.
#[semio_framework_async_macros::async_test]
async fn foreign_carriers_restore_the_edited_registered_rich_document(){
 use semio_framework_plugin::PluginApp;
 let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().unwrap();let mut original_context=original_caller.context();let cx=&mut original_context;
 let mut app=new_app().await;let first=node_id_at(&app,0).await;
 dispatch_rail(&mut app,"patchNodes",&[("nodeIds",&first),("field","name"),("value","Original rich archive")],cx).await.unwrap();
 semio_framework_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app, &TrinityJackCommand::TextEdit{text:"MATCH (n) RETURN n".into()}, &meta("local"), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut artifact_app_laws::fixture_identity(), cx).await.unwrap();settle(&mut app,cx).await;
 let original=PluginApp::document_archive(&app.app).await.unwrap();assert!(!original.members.is_empty());assert!(!original.parent_spr.is_empty());
 let original_bytes=store::channel::encode_document_archive_bytes(&original).unwrap();
 for format in 0..2{
  let decoded=super::super::archive_laws::round_trip(&original_bytes,format);assert_eq!(decoded,original_bytes);
  let received=super::super::archive_laws::with_control(|control|store::channel::decode_document_archive_bytes_controlled(&decoded,&mut control.snapshot_decode().unwrap())).unwrap();assert_eq!(received,original);
  dispatch_rail(&mut app,"patchNodes",&[("nodeIds",&first),("field","name"),("value","Replaced after export")],cx).await.unwrap();
  let operation=0x0000_0000_4a41_434b+format as u64;PluginApp::begin_document_archive_load(&mut app.app,operation,received).unwrap();
  let mut ready=false;for _ in 0..4096{assert!(!cx.should_yield(),"restore remains under its original fixture caller");cx.consume_fuel(1);let status=PluginApp::poll_document_archive_load(&mut app.app,operation,&mut artifact_app_laws::fixture_identity(),cx).await.unwrap();match status.state{protocol::DocumentArchiveLoadState::Ready=>{ready=true;break},protocol::DocumentArchiveLoadState::Fault=>panic!("original archive restore refused: {:?}",status),protocol::DocumentArchiveLoadState::Cancelled=>panic!("original archive restore cancelled"),_=>{}}}
  assert!(ready,"original archive reached terminal restore");PluginApp::acknowledge_document_archive_load(&mut app.app,operation).unwrap();assert_eq!(node_name(&app,&first).await,"Original rich archive");
  let restored=PluginApp::document_archive(&app.app).await.unwrap();assert_eq!(restored,original);super::super::archive_laws::retire(restored);super::super::archive_laws::retire(decoded);
 }
 super::super::archive_laws::retire(original_bytes);super::super::archive_laws::retire(original);app.close(cx);let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).unwrap();
}
