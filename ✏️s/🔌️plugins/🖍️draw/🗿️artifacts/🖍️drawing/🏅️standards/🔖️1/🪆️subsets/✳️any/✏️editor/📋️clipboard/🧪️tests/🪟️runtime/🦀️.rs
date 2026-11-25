//! 🪟️ Registered clipboard verbs publish mapped selection and one reversible history edit.
use super::*;
use semio_framework_test_fixtures::fixture_text;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
async fn reserved_clipboard(app:&mut DrawingAppFixture,verb:&str,args:Option<semio_framework_value::DslValue>,meta:&semio_plugin::ActionMeta)->(semio_plugin::InvocationResult,artifact_laws::TypedOperationFixtureReceipt){
 let admitted=semio_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, verb, args.clone(), meta, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.unwrap();
 let result=semio_plugin::app::settle_framework_reserved_admission(&mut app.0, admitted, app.1.cx()).await.unwrap();
 let receipt=artifact_laws::settle_registered_typed_operation(&mut app.0, meta.instance_id, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.unwrap();(result,receipt)
}
#[semio_framework_async_macros::async_test]
async fn registered_clipboard_copy_paste_cut_preserve_atomic_inverse_and_live_selection(){
 use semio_framework_value::ToValue;
 let fixtures:Vec<serde_json::Value>=serde_json::from_str(fixture_text!("../../../../../🧪️tests/🧫️fixtures/🎬️scene/📋️prepare/🔣️.json")).unwrap();
 let authored=fixtures.iter().find(|row|row["name"]=="rectangle").unwrap()["document"].clone();let before:DrawingSnapshot=serde_json::from_value(authored).unwrap();let original_id=semio_s_artifact_draw_drawing_model::layer_base(&before.layers[0]).id.to_string_owner();
 let(mut app,meta)=inline_selection_app().await;load_drawing_fixture(&mut app,&before).await;
 let targets=serde_json::to_string(&vec![serde_json::json!({"granularity":DRAWING_INTERACTION_GRANULARITY,"id":original_id})]).unwrap();
 let selection=semio_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, semio_framework::INTERACTION_SELECT_ACTION_ID, Some(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({"domainId":DRAWING_INTERACTION_DOMAIN,"targets":targets,"merge":"replace","method":"pick"}))), &meta, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.unwrap();
 semio_plugin::app::settle_framework_reserved_admission(&mut app.0, selection, app.1.cx()).await.unwrap();artifact_laws::settle_registered_typed_operation(&mut app.0, meta.instance_id, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.unwrap();
 let initial_edits=app.history_snapshot().await.unwrap().edit_count;let(copied,copy_receipt)=reserved_clipboard(&mut app,"copy",None,&meta).await;
 let effects=copied.requested_effects.iter().chain(copy_receipt.effects.iter()).filter_map(|effect|if let Effect::ClipboardWrite{fragment}=effect{Some(fragment)}else{None}).collect::<Vec<_>>();assert_eq!(effects.len(),1);let fragment=effects[0].clone();assert_eq!(app.snapshot().unwrap(),before);assert_eq!(app.history_snapshot().await.unwrap().edit_count,initial_edits);assert_eq!(selected_strokes(&app).await,vec![original_id.clone()]);
 let args=semio_framework_value::DslValue::Object(vec![("fragment".into(),fragment.to_value()),("anchor".into(),semio_framework_value::DslValue::String("original".into()))]);
 reserved_clipboard(&mut app,"paste",Some(args),&meta).await;let pasted=app.snapshot().unwrap();let selected=selected_strokes(&app).await;assert_eq!(pasted.layers.len(),before.layers.len()+1);assert_eq!(selected.len(),1);assert_ne!(selected[0],original_id);assert!(semio_s_artifact_draw_drawing_model::find_drawing_layer(&pasted,&selected[0]).is_some());assert_eq!(app.history_snapshot().await.unwrap().edit_count,initial_edits+1);
 artifact_laws::settle_history_verb(&mut app.0, "undo", meta.instance_id, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;assert_eq!(app.snapshot().unwrap(),before);assert!(selected_strokes(&app).await.iter().all(|id|semio_s_artifact_draw_drawing_model::find_drawing_layer(&before,id).is_some()));
 artifact_laws::settle_history_verb(&mut app.0, "redo", meta.instance_id, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;assert_eq!(app.snapshot().unwrap(),pasted);
 let targets=serde_json::to_string(&vec![serde_json::json!({"granularity":DRAWING_INTERACTION_GRANULARITY,"id":selected[0]})]).unwrap();let selection=semio_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, semio_framework::INTERACTION_SELECT_ACTION_ID, Some(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({"domainId":DRAWING_INTERACTION_DOMAIN,"targets":targets,"merge":"replace","method":"pick"}))), &meta, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.unwrap();semio_plugin::app::settle_framework_reserved_admission(&mut app.0, selection, app.1.cx()).await.unwrap();artifact_laws::settle_registered_typed_operation(&mut app.0, meta.instance_id, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.unwrap();
 let edits=app.history_snapshot().await.unwrap().edit_count;reserved_clipboard(&mut app,"cut",None,&meta).await;assert_eq!(app.snapshot().unwrap(),before);assert!(selected_strokes(&app).await.is_empty());assert_eq!(app.history_snapshot().await.unwrap().edit_count,edits+1);artifact_laws::settle_history_verb(&mut app.0, "undo", meta.instance_id, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;assert_eq!(app.snapshot().unwrap(),pasted);
 artifact_laws::close_registered_fixture_app(&mut app.0, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");assert!(app.close_terminal_is_empty());
}
/// 🌐️ The per-turn fuel and one interactive grant the browser shard issues, and the turns a browser command may take before the shell calls it stuck.
const BROWSER_TURN_FUEL:u64=50_000_000;
const BROWSER_TURN_LIMIT:usize=4096;
async fn browser_clipboard_settled(app:&mut DrawingAppFixture,verb:&str,args:Option<semio_framework_value::DslValue>,meta:&semio_plugin::ActionMeta)->(semio_plugin::InvocationResult,Result<(semio_plugin::artifact_app_laws::TypedOperationFixtureReceipt,usize),semio_plugin::PluginLifecycleFailure>){
 let policy=semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
 let _fuel=semio_plugin::artifact_app_laws::arm_host_turn_fuel(BROWSER_TURN_FUEL);let _grant=semio_plugin::artifact_app_laws::arm_host_turn_grant(policy.preparation);let _lifetime=semio_plugin::artifact_app_laws::arm_host_lifetime(app.1.cx());
 let admitted=semio_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, verb, args, meta, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), policy.close).await.unwrap();
 let result=semio_plugin::app::settle_framework_reserved_admission(&mut app.0, admitted, app.1.cx()).await.unwrap();
 (result,semio_plugin::artifact_app_laws::settle_registered_typed_operation_within(&mut app.0, meta.instance_id, policy, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), BROWSER_TURN_LIMIT).await)
}
async fn browser_clipboard(app:&mut DrawingAppFixture,verb:&str,args:Option<semio_framework_value::DslValue>,meta:&semio_plugin::ActionMeta)->(semio_plugin::InvocationResult,semio_plugin::artifact_app_laws::TypedOperationFixtureReceipt,usize){
 let(result,settled)=browser_clipboard_settled(app,verb,args,meta).await;let(receipt,turns)=settled.unwrap_or_else(|error|panic!("{verb} did not settle within {BROWSER_TURN_LIMIT} browser turns: {error:?}"));(result,receipt,turns)
}
#[semio_framework_async_macros::async_test]
async fn browser_grants_copy_emits_clipboard_write_and_paste_inserts_the_layer(){
 use semio_framework_value::ToValue;
 let fixtures:Vec<serde_json::Value>=serde_json::from_str(fixture_text!("../../../../../🧪️tests/🧫️fixtures/🎬️scene/📋️prepare/🔣️.json")).unwrap();
 let before:DrawingSnapshot=serde_json::from_value(fixtures.iter().find(|row|row["name"]=="rectangle").unwrap()["document"].clone()).unwrap();let original_id=semio_s_artifact_draw_drawing_model::layer_base(&before.layers[0]).id.to_string_owner();
 let(mut app,meta)=inline_selection_app().await;load_drawing_fixture(&mut app,&before).await;
 let policy=semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
 let targets=serde_json::to_string(&vec![serde_json::json!({"granularity":DRAWING_INTERACTION_GRANULARITY,"id":original_id})]).unwrap();
 let selection=semio_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, semio_framework::INTERACTION_SELECT_ACTION_ID, Some(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({"domainId":DRAWING_INTERACTION_DOMAIN,"targets":targets,"merge":"replace","method":"pick"}))), &meta, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), policy.close).await.unwrap();
 semio_plugin::app::settle_framework_reserved_admission(&mut app.0, selection, app.1.cx()).await.unwrap();artifact_laws::settle_registered_typed_operation(&mut app.0, meta.instance_id, policy, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.unwrap();
 let(copied,copy_receipt,_)=browser_clipboard(&mut app,"copy",None,&meta).await;
 let fragments=copied.requested_effects.iter().chain(copy_receipt.effects.iter()).filter_map(|effect|if let Effect::ClipboardWrite{fragment}=effect{Some(fragment.clone())}else{None}).collect::<Vec<_>>();assert_eq!(fragments.len(),1,"copy must emit exactly one clipboard-write effect");
 let args=semio_framework_value::DslValue::Object(vec![("fragment".into(),fragments[0].to_value()),("anchor".into(),semio_framework_value::DslValue::String("original".into()))]);
 browser_clipboard(&mut app,"paste",Some(args),&meta).await;
 let pasted=app.snapshot().unwrap();assert_eq!(pasted.layers.len(),before.layers.len()+1);let selected=selected_strokes(&app).await;assert_eq!(selected.len(),1);assert_ne!(selected[0],original_id);assert!(semio_s_artifact_draw_drawing_model::find_drawing_layer(&pasted,&selected[0]).is_some());
 artifact_laws::close_registered_fixture_app(&mut app.0, policy, app.1.cx()).expect("original caller paid app close");assert!(app.close_terminal_is_empty());
}
#[semio_framework_async_macros::async_test]
async fn browser_grants_copy_without_selection_is_refused_within_the_turn_limit(){
 let fixtures:Vec<serde_json::Value>=serde_json::from_str(fixture_text!("../../../../../🧪️tests/🧫️fixtures/🎬️scene/📋️prepare/🔣️.json")).unwrap();
 let before:DrawingSnapshot=serde_json::from_value(fixtures.iter().find(|row|row["name"]=="rectangle").unwrap()["document"].clone()).unwrap();
 let(mut app,meta)=inline_selection_app().await;load_drawing_fixture(&mut app,&before).await;
 let(_,settled)=browser_clipboard_settled(&mut app,"copy",None,&meta).await;
 let Err(refused)=settled else{panic!("copy without a selection must be refused, not settled")};assert!(!format!("{refused:?}").contains("original fixture settlement stopped"),"copy without a selection stalled instead of being refused: {refused:?}");
 assert_eq!(app.snapshot().unwrap(),before);
 artifact_laws::close_registered_fixture_app(&mut app.0, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");assert!(app.close_terminal_is_empty());
}
/// 📏️ The clipboard job is allocated, moved and retired as one indivisible body (twice at close), so its size must stay a small share of the interactive grant's copy budget; the message lists every inline component so one failing run names the offender.
#[test]
fn clipboard_job_body_fits_a_small_share_of_the_interactive_grant(){
 use semio_s_artifact_draw_drawing_model::DrawingLayerNode;
 let sizes=[("DrawingClipboardJob",size_of::<semio_s_artifact_draw_drawing_editor::clipboard_job::DrawingClipboardJob>()),("DrawingScenePlacementJob",size_of::<semio_s_artifact_draw_drawing_scene_paint::placement::DrawingScenePlacementJob>()),("DrawingLayerLocator",size_of::<semio_s_artifact_draw_drawing_host_arena::DrawingLayerLocator>()),("DrawingLayerCloneWorkAuthority",size_of::<semio_s_artifact_draw_drawing_host_arena::DrawingLayerCloneWorkAuthority>()),("DrawingNativeCloneAuthority<DrawingLayerNode>",size_of::<semio_s_artifact_draw_drawing_host_arena::DrawingNativeCloneAuthority<DrawingLayerNode>>()),("DrawingLayerNode",size_of::<DrawingLayerNode>())];
 let budget=semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.preparation.maximum_copy_bytes/8;
 assert!(sizes[0].1<=budget,"the clipboard job body must stay within {budget} bytes (an eighth of the interactive copy grant; close moves it twice), measured {sizes:?}");
}
