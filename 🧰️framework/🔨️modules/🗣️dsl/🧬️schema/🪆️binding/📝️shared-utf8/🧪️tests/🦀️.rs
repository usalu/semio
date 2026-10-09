//! 🧪️ Original shared actor projections compared with independent Serde text.
use super::*;
#[test]
fn original_shared_utf8_field_borrows_original_source_and_refuses_absent_authority(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 let owner=semio_framework_value::SharedUtf8::from(law["source"].as_str().unwrap());let pointer=owner.as_ptr();
 let native_encoding::FieldProjectionView::Text(text)=owner.projection_view(&[]).unwrap()else{panic!("original shared text")};assert_eq!(text.as_ptr(),pointer);assert_eq!(serde_json::to_string(text).unwrap(),serde_json::to_string(&owner).unwrap());
 assert_eq!(owner.to_value(),FieldValue::Text(text.to_owned()));
 let mut callback=|_|true;let mut control=NativeDecodeControl::new_retained(&mut callback);
 let error=<semio_framework_value::SharedUtf8 as DslField>::from_value_controlled(&FieldValue::Text(text.to_owned()),&mut control).unwrap_err();assert_eq!(error.kind,ValueRefusalKind::UnsupportedOwner);assert_eq!(error.retained_progress(),Default::default());assert_eq!(owner.as_ptr(),pointer);
 eprintln!("[DEBUG] original SharedUtf8 DslField text projection borrows same source pointer, matches Serde, refuses missing five-axis authority before birth");
}
