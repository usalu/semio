//! 🪟️ Registered clipboard verbs publish mapped selection and one reversible history edit.
use super::*;
async fn reserved_clipboard(app:&mut DrawingApp,verb:&str,args:Option<semio_framework_value::DslValue>,meta:&semio_framework_plugin::ActionMeta)->(semio_framework_plugin::InvocationResult,artifact_laws::TypedOperationFixtureReceipt){
 let admitted=app.handle_action(verb,args.as_ref(),meta).await.unwrap();
 let result=semio_framework_plugin::app::settle_framework_reserved_admission(app,admitted).await.unwrap();
 let receipt=artifact_laws::settle_registered_typed_operation(app,meta.instance_id).await.unwrap();(result,receipt)
}
#[semio_framework_async_macros::async_test]
async fn registered_clipboard_copy_paste_cut_preserve_atomic_inverse_and_live_selection(){
 use semio_framework_value::ToValue;
 let fixtures:Vec<serde_json::Value>=serde_json::from_str(include_str!("../../../../🧬️schema/🎬️scene/📋️prepare/🧫️fixtures/🔣️.json")).unwrap();
 let authored=fixtures.iter().find(|row|row["name"]=="rectangle").unwrap()["document"].clone();let before:DrawingSnapshot=serde_json::from_value(authored).unwrap();let original_id=crate::schema::layer_base(&before.layers[0]).id.to_string_owner();
 let(mut app,meta)=inline_selection_app().await;load_drawing_fixture(&mut app,&before);
 let targets=serde_json::to_string(&vec![serde_json::json!({"granularity":DRAWING_INTERACTION_GRANULARITY,"id":original_id})]).unwrap();
 let selection=app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID,Some(&semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({"domainId":DRAWING_INTERACTION_DOMAIN,"targets":targets,"merge":"replace","method":"pick"}))),&meta).await.unwrap();
 semio_framework_plugin::app::settle_framework_reserved_admission(&mut *app,selection).await.unwrap();artifact_laws::settle_registered_typed_operation(&mut *app,meta.instance_id).await.unwrap();
 let initial_edits=app.history_snapshot().await.unwrap().edit_count;let(copied,copy_receipt)=reserved_clipboard(&mut app,"copy",None,&meta).await;
 let effects=copied.requested_effects.iter().chain(copy_receipt.effects.iter()).filter_map(|effect|if let Effect::ClipboardWrite{fragment}=effect{Some(fragment)}else{None}).collect::<Vec<_>>();assert_eq!(effects.len(),1);let fragment=effects[0].clone();assert_eq!(app.snapshot().unwrap(),before);assert_eq!(app.history_snapshot().await.unwrap().edit_count,initial_edits);assert_eq!(selected_strokes(&app).await,vec![original_id.clone()]);
 let args=semio_framework_value::DslValue::Object(vec![("fragment".into(),fragment.to_value()),("anchor".into(),"original".into())]);
 reserved_clipboard(&mut app,"paste",Some(args),&meta).await;let pasted=app.snapshot().unwrap();let selected=selected_strokes(&app).await;assert_eq!(pasted.layers.len(),before.layers.len()+1);assert_eq!(selected.len(),1);assert_ne!(selected[0],original_id);assert!(crate::schema::find_drawing_layer(&pasted,&selected[0]).is_some());assert_eq!(app.history_snapshot().await.unwrap().edit_count,initial_edits+1);
 artifact_laws::settle_history_verb(&mut *app,"undo",meta.instance_id).await;assert_eq!(app.snapshot().unwrap(),before);assert!(selected_strokes(&app).await.iter().all(|id|crate::schema::find_drawing_layer(&before,id).is_some()));
 artifact_laws::settle_history_verb(&mut *app,"redo",meta.instance_id).await;assert_eq!(app.snapshot().unwrap(),pasted);
 let targets=serde_json::to_string(&vec![serde_json::json!({"granularity":DRAWING_INTERACTION_GRANULARITY,"id":selected[0]})]).unwrap();let selection=app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID,Some(&semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({"domainId":DRAWING_INTERACTION_DOMAIN,"targets":targets,"merge":"replace","method":"pick"}))),&meta).await.unwrap();semio_framework_plugin::app::settle_framework_reserved_admission(&mut *app,selection).await.unwrap();artifact_laws::settle_registered_typed_operation(&mut *app,meta.instance_id).await.unwrap();
 let edits=app.history_snapshot().await.unwrap().edit_count;reserved_clipboard(&mut app,"cut",None,&meta).await;assert_eq!(app.snapshot().unwrap(),before);assert!(selected_strokes(&app).await.is_empty());assert_eq!(app.history_snapshot().await.unwrap().edit_count,edits+1);artifact_laws::settle_history_verb(&mut *app,"undo",meta.instance_id).await;assert_eq!(app.snapshot().unwrap(),pasted);
 artifact_laws::close_registered_fixture_app(&mut *app);assert!(app.close_terminal_is_empty());eprintln!("[DEBUG] Native registered copy preserves history, paste/cut publish mapped selection and one inverse history entry, undo keeps live IDs and actual app owners close");
}
