//! 🧪️ Full scene and typed raster failure projections borrow existing paid payloads.
use super::super::{DocumentSceneError,DocumentSceneMessage};
use semio_framework_job::{OriginalDomainProjection,OriginalDomainView as V};
use semio_s_artifact_draw_drawing_geometry::raster::{PathRasterError,PathRasterRetirementCause};
use semio_framework_2d::{flatten::PathFlattenError,stroke::StrokeOutlineError};
use semio_framework_pixels::editing::PixelEditError;
use semio_framework_pixels::affine_sampling::AffineImageError;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
use serde_json::{Value,json};
#[path="../../../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🛤️path/🎟️work/🧪️tests/🦀️.rs"]
mod wallet;
const MESSAGE:&str="Owned 源 \"message\"\n";
fn text(source:&str,bank:&mut wallet::Wallet)->String{let mut value=String::new();for _ in 0..2{if bank.turn_effect(1,|work,effect|work.reserve_string(&mut value,source.len(),effect.actual()).unwrap()){break}}assert!(value.capacity()>=source.len());assert!(bank.turn(1,|work|work.string_push(&mut value,source).unwrap()));value}
fn fixture(source:&str,bank:&mut wallet::Wallet)->DocumentSceneError{
 match source{
  "affineInvalid"=>AffineImageError::Invalid(MESSAGE).into(),"affinePixels"=>AffineImageError::Pixels(PixelEditError::Invalid(MESSAGE)).into(),"affineIncomplete"=>AffineImageError::Incomplete.into(),"affineCancelled"=>AffineImageError::Cancelled.into(),
  "literal"=>DocumentSceneError::literal(MESSAGE),
  "owned"=>{let value=text(MESSAGE,bank);DocumentSceneError::owned(bank.turn_effect(1,|work,effect|work.arc_new(value,effect.actual()).unwrap()))},
  "flatten"=>PathRasterError::Flatten(PathFlattenError::Invalid(MESSAGE)).into(),
  "stroke"=>PathRasterError::Stroke(StrokeOutlineError::Incomplete).into(),
  "pixels"=>PathRasterError::Pixels(PixelEditError::Cancelled).into(),
  "retirement"=>PathRasterError::Retirement{kind:"ownershipLimit",message:MESSAGE,posting_refusal:Some("original retained refusal")}.into(),
  "ownedRetirement"=>{let message=text(MESSAGE,bank);let cause=PathRasterRetirementCause{kind:"ownershipLimit",message,posting_refusal:Some("original retained refusal")};PathRasterError::OwnedRetirement(bank.turn_effect(1,|work,effect|work.arc_new(cause,effect.actual()).unwrap())).into()},
  "incomplete"=>DocumentSceneError::Incomplete,"cancelled"=>DocumentSceneError::Cancelled,
  "rasterInvalid"=>PathRasterError::Invalid(MESSAGE).into(),"rasterIncomplete"=>PathRasterError::Incomplete.into(),"rasterCancelled"=>PathRasterError::Cancelled.into(),_=>panic!("unknown neutral source")
 }
}
fn reference(source:&dyn OriginalDomainProjection,path:&mut Vec<usize>)->Value{match source.projection_view(path).unwrap(){V::IntrinsicObject(len)=>{let mut object=serde_json::Map::new();for index in 0..len{let key=source.projection_key(path,index).unwrap().to_owned();path.push(index);let child=reference(source,path);path.pop();object.insert(key,child);}Value::Object(object)},V::IntrinsicText(text)=>json!(text),_=>panic!("unexpected error projection")}}
#[test]
fn scene_error_borrows_complete_original_typed_causes_without_heap_or_payload_copy(){
 let law:Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let profile=serde_json::to_string(&law["wallet"]).unwrap();
 for row in law["cases"].as_array().unwrap(){assert_eq!(row["message"],MESSAGE);let mut bank=wallet::Wallet::new(&profile);let error=fixture(row["source"].as_str().unwrap(),&mut bank);let pointer=error.message().as_ptr();let scope=match &error{DocumentSceneError::Invalid(DocumentSceneMessage::Owned(value))=>Some(std::ptr::from_ref(&**value)as usize),DocumentSceneError::PathRaster(PathRasterError::OwnedRetirement(value))=>Some(std::ptr::from_ref(&**value)as usize),_=>None};
  let(cloned,heap)=observe(||error.clone());assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cloned.message().as_ptr(),pointer);let other=match &cloned{DocumentSceneError::Invalid(DocumentSceneMessage::Owned(value))=>Some(std::ptr::from_ref(&**value)as usize),DocumentSceneError::PathRaster(PathRasterError::OwnedRetirement(value))=>Some(std::ptr::from_ref(&**value)as usize),_=>None};assert_eq!(scope,other);
  assert_eq!(reference(&error,&mut Vec::new()),row["expected"]);assert_eq!(reference(&cloned,&mut Vec::new()),row["expected"]);let(_,heap)=observe(||{assert_eq!(error.message(),row["expected"]["message"].as_str().unwrap());assert!(matches!(error.projection_view(&[]).unwrap(),V::IntrinsicObject(_)));assert_eq!(error.projection_key(&[],0).unwrap(),"kind");});assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));bank.close_owned((error,cloned));
 }
}

#[test]
fn boolean_refusal_preserves_framework_failure_projection() {
 let original = semio_framework_value::ValueError::literal(
  semio_framework_value::ValueRefusalKind::OwnershipLimit, MESSAGE,
 );
 let error = DocumentSceneError::from(semio_framework_2d::booleans::BooleanError::Refused(original.clone()));
 assert_eq!(error.message(), MESSAGE);
 assert_eq!(error.projection_key(&[], 2).unwrap(), "original");
 let V::IntrinsicObject(fields) = semio_framework_job::project_original_value_error(&original, &[]).unwrap() else { panic!("Expected a typed refusal"); };
 assert!(matches!(error.projection_view(&[2]).unwrap(), V::IntrinsicObject(len) if len == fields));
 for index in 0..fields {
  assert_eq!(error.projection_key(&[2], index).unwrap(), semio_framework_job::key_original_value_error(&original, &[], index).unwrap());
 }
}
