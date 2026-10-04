//! 🧪️ Lossless typed refusal identity across the actual Pack error boundary.
use super::*;
use semio_framework_value::ValueRefusalKind;

fn kind(text:&str)->ValueRefusalKind {
 match text {
  "InvalidValue"=>ValueRefusalKind::InvalidValue, "Canceled"=>ValueRefusalKind::Canceled,
  "OwnershipLimit"=>ValueRefusalKind::OwnershipLimit, "AllocationFailed"=>ValueRefusalKind::AllocationFailed,
  "WorkLimit"=>ValueRefusalKind::WorkLimit, "DepthLimit"=>ValueRefusalKind::DepthLimit,
  "UnsupportedOwner"=>ValueRefusalKind::UnsupportedOwner, "InvariantViolated"=>ValueRefusalKind::InvariantViolated,
  _=>panic!("unknown authored kind")
 }
}

#[test]
fn canonical_pack_refusal_preserves_kind_path_and_owned_error_identity(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/⚠️refusal/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let expected_kind=kind(row["kind"].as_str().unwrap());
  let mut error=ValueError::new(expected_kind,row["message"].as_str().unwrap());
  for part in row["path"].as_array().unwrap(){error=error.under(part.as_str().unwrap());}
  let allocation=error.message.as_ptr();
  let wrapped=PackError::from(error);
  let PackError::Refusal(PackRefusal::ValueRefusal(refusal))=&wrapped else {panic!("typed refusal variant");};
  assert_eq!(refusal.kind,expected_kind);
  assert_eq!(refusal.message.as_ptr(),allocation);
  assert_eq!(wrapped.to_string(),row["display"].as_str().unwrap());
  let source=std::error::Error::source(&wrapped).unwrap().downcast_ref::<ValueError>().unwrap();
  assert!(std::ptr::eq(source,refusal));
  let reference=serde_json::json!({"kind":row["kind"],"message":row["display"].as_str().unwrap().strip_prefix("schema error: ").unwrap()});
  assert_eq!(serde_json::json!({"kind":format!("{:?}",refusal.kind),"message":refusal.message}),reference);
 }
}
