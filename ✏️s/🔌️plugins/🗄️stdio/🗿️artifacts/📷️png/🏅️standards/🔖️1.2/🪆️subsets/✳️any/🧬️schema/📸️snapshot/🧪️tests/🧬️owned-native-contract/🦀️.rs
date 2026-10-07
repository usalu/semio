//! 🧪️ Neutral precise PNG sample paints and semantic diff laws.
use crate::schema::{snapshot::{PngSnapshot,PngRegion,PngNativePaint},operations::{png_revision,paint_native_region_controlled,validate_completed_native_paint},diff::PngDiff};
use protocol::{DiffAlgebra,MutationDiff};
#[test]
fn neutral_native_samples_preserve_metadata_and_inverse_laws() {
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧫️fixtures/🧬️owned-native-samples/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap() {
  let base:PngSnapshot=semio_framework_pack_json::from_json_str(&row["snapshot"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();base.validate().unwrap();let input=&row["paint"];let region=PngRegion {x:input["x"].as_u64().unwrap()as u32,y:input["y"].as_u64().unwrap()as u32,width:input["width"].as_u64().unwrap()as u32,height:input["height"].as_u64().unwrap()as u32};
  let paint:PngNativePaint=semio_framework_pack_json::from_json_str(&serde_json::json!({"profile":input["profile"],"first":input["first"],"second":input["second"],"third":input["third"],"fourth":input["fourth"]}).to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
  let revision=png_revision(&base);let mut progress=Vec::new();let next=paint_native_region_controlled(&base,&revision,region,paint,&mut|completed,total|{progress.push((completed,total));true}).unwrap();
  let mut reader=std::sync::Arc::new(base.clone());let mut work=crate::schema::operations::PngNativePaintWorkOperation::try_new_retained(std::sync::Arc::clone(&reader),region,paint,512*1024*1024).unwrap();
  assert!(std::sync::Arc::get_mut(&mut reader).is_none(),"immutable read capability prevents in-place sample edits");let mut changed_reader=std::sync::Arc::clone(&reader);std::sync::Arc::make_mut(&mut changed_reader).image.samples[0]^=1;assert!(!work.retained_reader_matches(&changed_reader));
  let mut yields=0;loop {let mut sequence=0;let mut context=semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(),semio_framework_job::Generation(1),semio_framework_job::StepBudget::new(1,u64::MAX),semio_framework_job::root_cancel_token(),||Some(0),&mut sequence);match work.advance(&mut context).unwrap() {crate::schema::operations::PngNativePaintWorkStep::Yield(_)=>yields+=1,crate::schema::operations::PngNativePaintWorkStep::Complete=>break,crate::schema::operations::PngNativePaintWorkStep::Cancelled=>panic!("uncancelled precise operation")}}assert!(yields>0);assert_eq!(work.take_result().unwrap(),next);work.begin_close();while !work.terminal_is_empty() {work.close_step(1,usize::MAX);}
  assert_eq!(serde_json::json!(next.image.samples),row["expectedSamples"]);assert!(base.image.same_metadata(&next.image));validate_completed_native_paint(&base,&next,region,paint).unwrap();
  let diff=PngDiff::between(&base,&next);assert_eq!(diff.apply(&base).unwrap(),next);assert_eq!(diff.inverse(&base).apply(&next).unwrap(),base);let mut composite=PngDiff::default();composite.absorb(diff);assert_eq!(composite.apply(&base).unwrap(),next);
  assert_eq!(progress.first(),Some(&(0,region.height as usize)));assert_eq!(progress.last(),Some(&(region.height as usize,region.height as usize)));assert!(paint_native_region_controlled(&base,"stale",region,paint,&mut|_,_|true).unwrap_err().contains("stale"));
  eprintln!("[DEBUG] png owned native case={} metadata=exact inverse=exact",row["name"]);
 }
}
