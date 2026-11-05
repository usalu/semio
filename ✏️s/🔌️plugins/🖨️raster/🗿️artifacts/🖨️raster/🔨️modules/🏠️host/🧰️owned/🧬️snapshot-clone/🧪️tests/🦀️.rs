//! 🧪️ Clone physical receipts bind every payload page to the actual caller grant.
use super::*;

fn turn(clone:&mut RasterDslValueCloneAuthority,source:&semio_framework_value::DslValue,totals:&mut RasterOwnerTotals,grant:RetainedCloneGrant)->RetainedCloneProgress{
 let demand=clone.demands(source,grant.maximum_copy_bytes).unwrap();
 let mut progress=RetainedCloneProgress::default();let mut sequence=0;
 let mut context=semio_framework_job::StepContext::new(semio_framework_job::OperationId(719),semio_framework_job::Generation(1),semio_framework_job::StepBudget::new(1,u64::MAX,grant),semio_framework_job::root_cancel_token(),semio_framework_job::default_now_us,&mut sequence,&mut progress);
 clone.step(source,totals,&mut context).unwrap();drop(context);assert!(progress.fits(grant));assert!(progress.fits(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}));progress
}

#[test]
fn byte_clone_denial_preserves_owner_and_payload_copy_is_paged(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("receipts.json")).unwrap();let length=fixture["byteLength"].as_u64().unwrap()as usize;
 let source=semio_framework_value::DslValue::Bytes(vec![37;length]);let mut clone=RasterDslValueCloneAuthority::new(&source);let mut totals=RasterOwnerTotals::new();
 let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:fixture["copyPageBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:length,maximum_release_bytes:32768,maximum_depth:64};
 let mut capacity=0;let mut payload_copy=0;let mut denied=false;
 for _ in 0..100{
  if clone.terminal{break;}
  let before=match clone.value.as_ref(){Some(semio_framework_value::DslValue::Bytes(bytes))=>bytes.len(),_=>0};
  if !denied&&clone.root_capacity_observed{let receipt=turn(&mut clone,&source,&mut totals,RetainedCloneGrant{maximum_capacity_bytes:0,..grant});assert_eq!(receipt,RetainedCloneProgress::default());let Some(semio_framework_value::DslValue::Bytes(bytes))=clone.value.as_ref()else{panic!("exact byte owner")};assert_eq!(bytes.len(),fixture["deniedPayloadBytes"].as_u64().unwrap()as usize);assert_eq!(bytes.capacity(),fixture["deniedPayloadCapacity"].as_u64().unwrap()as usize);denied=true;}
  let receipt=turn(&mut clone,&source,&mut totals,grant);capacity+=receipt.retained_capacity_bytes;
  let after=match clone.value.as_ref(){Some(semio_framework_value::DslValue::Bytes(bytes))=>bytes.len(),_=>0};let copied=after-before;payload_copy+=copied;if copied!=0{assert_eq!(receipt.copied_bytes,copied+size_of::<Vec<u8>>());}
 }
 assert!(denied&&clone.terminal);assert_eq!(capacity,fixture["expectedPayloadCapacity"].as_u64().unwrap()as usize);assert_eq!(payload_copy,fixture["expectedPayloadBytes"].as_u64().unwrap()as usize);let result=clone.take().unwrap();assert_eq!(result,source);retire_raster_value_cold(result);retire_raster_value_cold(source);
}

#[test]
fn empty_nested_clone_controller_requires_its_own_release_turn(){
 let source=RasterLayerNode::Pixel{id:String::new(),name:String::new(),visible:true,locked:false,opacity:1.0,blend_mode:String::new(),transform:semio_s_artifact_raster_raster_model::RasterTransform::default(),mask:None,width:None,height:None,image_key:None};
 let mut snapshot=RasterSnapshotCloneAuthority::new();*snapshot.layer=Some(Box::new(RasterLayerCloneAuthority::new(&source)));let bytes=size_of::<RasterLayerCloneAuthority>();let demand=snapshot.close_demands(128).unwrap();assert_eq!(demand.release_bytes,bytes);
 let denied=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:32768,maximum_release_bytes:bytes-1,maximum_depth:64};let denied_turn=snapshot.close_step(denied);assert_eq!(denied_turn.work,0);assert_eq!(denied_turn.result.unwrap().progress(),RetainedCloneProgress::default());assert!(snapshot.layer.is_some());let turn=snapshot.close_step(RetainedCloneGrant{maximum_release_bytes:bytes,..denied});assert_eq!(turn.work,1);let receipt=turn.result.unwrap().progress();assert_eq!(receipt.released_bytes,bytes);assert!(snapshot.layer.is_none());let empty=snapshot.value.take().unwrap();retire_raster_value_cold(empty);retire_raster_value_cold(source);assert!(snapshot.terminal_is_empty());
}

#[test]
fn nested_snapshot_clone_finishes_with_only_each_quoted_effect(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("receipts.json")).unwrap();let bytes=fixture["byteLength"].as_u64().unwrap()as usize;
 let mut params=RasterOwnedMap::new();params.insert("amount".into(),semio_framework_value::DslValue::Object(vec![("payload".into(),semio_framework_value::DslValue::Bytes(vec![9;bytes]))])).unwrap();
 let adjustment=RasterLayerNode::Adjustment{id:"adjustment".into(),name:"Adjust".into(),visible:true,locked:false,opacity:1.0,blend_mode:"normal".into(),transform:semio_s_artifact_raster_raster_model::RasterTransform::default(),adjustment_kind:"brightness".into(),params};
 let mut source=semio_s_artifact_raster_raster_model::io::text::snapshot::snapshot_wire_codec::empty_raster_snapshot();source.layers.push(RasterLayerNode::Group{id:"group".into(),name:"Group".into(),visible:true,locked:false,opacity:1.0,blend_mode:"normal".into(),transform:semio_s_artifact_raster_raster_model::RasterTransform::default(),mask:None,children:vec![adjustment]});
 let mut clone=RasterSnapshotCloneAuthority::new();
 for _ in 0..20000{
  if clone.terminal{break}let demand=clone.demands(&source,128).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let mut progress=RetainedCloneProgress::default();let mut sequence=0;
  let mut context=semio_framework_job::StepContext::new(semio_framework_job::OperationId(720),semio_framework_job::Generation(1),semio_framework_job::StepBudget::new(1,u64::MAX,grant),semio_framework_job::root_cancel_token(),semio_framework_job::default_now_us,&mut sequence,&mut progress);clone.step(&source,&mut context).unwrap();drop(context);assert!(progress.fits(grant));
 }
 assert!(clone.terminal);let result=clone.take_value().unwrap();assert_eq!(result,source);retire_raster_value_cold(result);retire_raster_value_cold(source);
}
