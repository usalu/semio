use super::*;
#[test]
fn framework_reserved_emit_publication_contracts_match_neutral_routes(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let declarations=[("copy",FrameworkCopyJobFactory::<TestApp>::PUBLICATION_CONTRACTS),("cut",FrameworkCutJobFactory::<TestApp>::PUBLICATION_CONTRACTS),("paste",FrameworkPasteJobFactory::<TestApp>::PUBLICATION_CONTRACTS),("import-media",FrameworkImportMediaJobFactory::<TestApp>::PUBLICATION_CONTRACTS),("undo",FrameworkUndoJobFactory::<TestApp>::PUBLICATION_CONTRACTS),("redo",FrameworkRedoJobFactory::<TestApp>::PUBLICATION_CONTRACTS)];
 for(row,(id,contracts))in law["routes"].as_array().unwrap().iter().zip(declarations){assert_eq!(row["id"].as_str().unwrap(),id);assert_eq!(contracts.len(),1);assert_eq!(contracts[0].tool_id,id);let actual=contracts[0].lanes.iter().map(|lane|format!("{lane:?}")).collect::<Vec<_>>();let expected=row["lanes"].as_array().unwrap().iter().map(|lane|lane.as_str().unwrap().to_string()).collect::<Vec<_>>();assert_eq!(actual,expected,"reserved route {id} declares its exact original durable emission authority");println!("[DEBUG] reserved route {id} exact lanes={actual:?}");}
}

#[semio_framework_async_macros::async_test]
async fn framework_reserved_emit_admission_preserves_saturated_foreign_slots_and_original_lease(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){
  let occupied=row["occupied"].as_u64().unwrap()as usize;
  let mut app=contract_composed_app_raw(crate::app::artifact_app_laws::fixture_mounted_policy(), &mut crate::app::artifact_app_laws::fixture_identity()).await;
  let admitted=app.test_reserved_emit_admission_case(occupied,&meta());
  drain_and_close_composed_fixture(&mut app, crate::app::artifact_app_laws::fixture_mounted_policy());
  assert_eq!(admitted,row["admitted"].as_bool().unwrap());
  println!("[DEBUG] reserved producer occupied={occupied} exact original reservation+keyed lease, no producer handoff at saturation, foreign identity unchanged");
 }
}

#[semio_framework_async_macros::async_test]
async fn framework_reserved_emit_cancelled_worker_keeps_original_permit_until_completion_handoff(){
 let mut app=contract_composed_app_raw(crate::app::artifact_app_laws::fixture_mounted_policy(), &mut crate::app::artifact_app_laws::fixture_identity()).await;
 let retained=app.test_reserved_emit_cancelled_worker_handoff(&meta()).await;
 drain_and_close_composed_fixture(&mut app, crate::app::artifact_app_laws::fixture_mounted_policy());
 assert!(retained,"cancelled producer must return its original permit so completed original output can enter bounded mounted cancellation");
 println!("[DEBUG] cancelled reserved worker retains original operation+keyed lease through completion handoff, then exact finish");
}

#[semio_framework_async_macros::async_test]
async fn framework_reserved_emit_busy_completion_keeps_original_fixed_owner(){
 let mut app=contract_composed_app_raw(crate::app::artifact_app_laws::fixture_mounted_policy(), &mut crate::app::artifact_app_laws::fixture_identity()).await;
 let retained=app.test_reserved_emit_busy_completion_owner(&meta()).await;
 drain_and_close_composed_fixture(&mut app, crate::app::artifact_app_laws::fixture_mounted_policy());
 assert!(retained,"busy original completion remains in the same pre-admitted operation until bounded close");
 println!("[DEBUG] busy completion retains same operation, original cell/payload pointer and cancelled keyed lease");
}
