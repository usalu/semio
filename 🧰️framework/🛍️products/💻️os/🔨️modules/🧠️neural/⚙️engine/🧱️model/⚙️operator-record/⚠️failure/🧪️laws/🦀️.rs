//! 🧪️ Independent JSON oracle checks every original typed evaluation refusal.
use super::*;
fn json(source:&impl OriginalDomainProjection,path:&mut Vec<usize>)->serde_json::Value{match source.projection_view(path).unwrap(){
 V::IntrinsicObject(n)=>{let mut map=serde_json::Map::new();for i in 0..n{let key=source.projection_key(path,i).unwrap().to_owned();path.push(i);let value=json(source,path);path.pop();map.insert(key,value);}serde_json::Value::Object(map)},
 V::IntrinsicText(s)=>serde_json::Value::String(s.into()),V::IntrinsicNumber(Number::UInt(n))=>n.into(),_=>panic!("unexpected failure projection leaf")
}}
#[test]
fn original_operator_failure_lends_all_variants_receipt_and_exact_payload(){
 let cases=[(EvalError::UnknownKind("κ".into()),"unknownKind"),(EvalError::MissingInput("a".into()),"missingInput"),(EvalError::InvalidInput("b".into()),"invalidInput"),(EvalError::CardinalityViolation("c".into()),"cardinalityViolation"),(EvalError::HeterogeneousList("d".into()),"heterogeneousList"),(EvalError::CycleDetected,"cycleDetected"),(EvalError::PendingExtension{extension_id:"ext".into(),operator_id:"op".into(),node_hash:71},"pendingExtension"),(EvalError::Retained(ValueError::literal(ValueRefusalKind::InvalidValue,"native")),"retained")];
 let progress=RetainedCloneProgress{copied_items:3,copied_bytes:5,retained_capacity_bytes:7,released_bytes:11};
 for(primary,kind)in cases{
  let failure=OperatorWorkFailure::Evaluation{primary,retained:progress,receipt_refusal:Some(ValueError::literal(ValueRefusalKind::WorkLimit,"receipt"))};
  let value=json(&failure,&mut Vec::new());
  assert_eq!(value["primary"]["variant"],kind);
  assert_eq!(value["retained"],serde_json::json!({"copiedItems":3,"copiedBytes":5,"retainedCapacityBytes":7,"releasedBytes":11}));
  assert_eq!(value["receiptRefusal"]["message"],"receipt");
  assert_eq!(failure.domain_retained_progress(),progress);
  assert_eq!(failure.domain_code(),"neural.operator-evaluation");
  if let OperatorWorkFailure::Evaluation{primary:EvalError::UnknownKind(original),..}=&failure{let V::IntrinsicText(text)=failure.projection_view(&[0,1]).unwrap()else{panic!("text")};assert_eq!(text.as_ptr(),original.as_ptr());assert_eq!(text.len(),original.len());}
  assert!(failure.projection_view(&[9]).is_err());
 }
 let without_receipt=OperatorWorkFailure::from(EvalError::CycleDetected);
 let actual=json(&without_receipt,&mut Vec::new());assert_eq!(actual.as_object().unwrap().len(),2);assert!(actual.get("receiptRefusal").is_none());assert!(without_receipt.projection_view(&[2]).is_err());
 let original=OperatorWorkFailure::Original(ValueError::literal(ValueRefusalKind::InvalidValue,"original").into());
 let actual=json(&original,&mut Vec::new());assert_eq!(actual["primary"]["original"]["message"],"original");
}
