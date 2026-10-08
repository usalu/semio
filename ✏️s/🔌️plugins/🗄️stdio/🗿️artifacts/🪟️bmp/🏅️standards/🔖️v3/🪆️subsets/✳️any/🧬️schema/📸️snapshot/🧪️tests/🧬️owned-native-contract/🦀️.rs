//! 🧪️ Neutral precise sample paint, metadata retention and inverse laws.
use crate::schema::{diff::BmpDiff,operations::{bmp_revision,paint_direct_region_controlled,paint_indexed_region_controlled},snapshot::{BmpSnapshot,BmpPixels,BmpColor,BmpRegion}};
use protocol::{DiffAlgebra,MutationDiff};
use semio_framework_value::FromValue;

#[test]
fn neutral_native_sample_paints_preserve_metadata_and_inverse() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🧬️owned-native-samples/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let base = BmpSnapshot::from_value(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse_bytes(row["snapshot"].to_string().as_bytes(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap())).unwrap();
        base.validate().unwrap();
        let paint = &row["paint"];
        let region = BmpRegion { x:paint["x"].as_u64().unwrap() as u32,y:paint["y"].as_u64().unwrap() as u32,width:paint["width"].as_u64().unwrap() as u32,height:paint["height"].as_u64().unwrap() as u32 };
        let revision = bmp_revision(&base);
        let mut progress = Vec::new();
        let next = if let Some(index) = paint["paletteIndex"].as_u64() { paint_indexed_region_controlled(&base,&revision,region,index as u8,&mut |completed,total| { progress.push((completed,total)); true }).unwrap() } else { let color = BmpColor { red:paint["red"].as_u64().unwrap() as u8,green:paint["green"].as_u64().unwrap() as u8,blue:paint["blue"].as_u64().unwrap() as u8,alpha:paint["alpha"].as_u64().unwrap() as u8 }; paint_direct_region_controlled(&base,&revision,region,color,&mut |completed,total| { progress.push((completed,total)); true }).unwrap() };
        let retained_paint=if let Some(index)=paint["paletteIndex"].as_u64() {crate::schema::operations::BmpRetainedPaint::Indexed(index as u8)} else {crate::schema::operations::BmpRetainedPaint::Direct(BmpColor {red:paint["red"].as_u64().unwrap() as u8,green:paint["green"].as_u64().unwrap() as u8,blue:paint["blue"].as_u64().unwrap() as u8,alpha:paint["alpha"].as_u64().unwrap() as u8})};
        let mut reader=std::sync::Arc::new(base.clone());
        let mut operation=crate::schema::operations::BmpPaintWorkOperation::try_new_retained(std::sync::Arc::clone(&reader),region,retained_paint,512*1024*1024).unwrap();
        assert!(std::sync::Arc::get_mut(&mut reader).is_none(),"retained immutable source cannot be edited in place");
        let mut changed_reader=std::sync::Arc::clone(&reader);std::sync::Arc::make_mut(&mut changed_reader).image.reserved_1^=1;
        assert!(!operation.retained_reader_matches(&changed_reader));
        let mut yields=0;
        loop {let mut sequence=0;let mut context=semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(),semio_framework_job::Generation(1),semio_framework_job::StepBudget::new(1,u64::MAX),semio_framework_job::root_cancel_token(),||Some(0),&mut sequence);match operation.advance(&mut context).unwrap() {crate::schema::operations::BmpPaintWorkStep::Yield {..}=>yields+=1,crate::schema::operations::BmpPaintWorkStep::Complete=>break,crate::schema::operations::BmpPaintWorkStep::Cancelled=>panic!("uncancelled owned operation")}}
        assert!(yields>0);assert_eq!(operation.take_result().unwrap(),next);
        operation.begin_close();while !operation.terminal_is_empty() {operation.close_step(1,usize::MAX);}
        let cancel=semio_framework_job::root_cancel_token();cancel.cancel_now();
        let mut operation=crate::schema::operations::BmpPaintWorkOperation::try_new(&base,region,retained_paint,512*1024*1024).unwrap();let mut sequence=0;let mut context=semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(),semio_framework_job::Generation(1),semio_framework_job::StepBudget::new(1,u64::MAX),cancel,||Some(0),&mut sequence);
        assert_eq!(operation.advance(&mut context).unwrap(),crate::schema::operations::BmpPaintWorkStep::Cancelled);assert!(operation.take_result().is_err());operation.begin_close();assert!(matches!(operation.close_step(1,0),semio_framework_job::InteractiveJobCloseStep::Pending {released_items:0,released_bytes:0}));while !operation.terminal_is_empty() {operation.close_step(1,usize::MAX);}
        let mut metadata = next.image.clone(); metadata.pixels = base.image.pixels.clone(); assert_eq!(metadata,base.image);
        match &next.image.pixels { BmpPixels::Indexed { indices } => assert_eq!(serde_json::json!(indices),row["expectedIndices"]),BmpPixels::Direct { samples } => assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(samples)).unwrap(),row["expectedSamples"]) }
        assert_eq!(progress.first(),Some(&(0,region.height as usize))); assert_eq!(progress.last(),Some(&(region.height as usize,region.height as usize)));
        let error = paint_indexed_region_controlled(&base,"stale",region,0,&mut |_,_|true).unwrap_err(); assert!(error.contains("stale"));
        eprintln!("[DEBUG] bmp owned native case={} samples={} inverse=exact",row["name"],base.image.width * base.image.height);
    }
}
