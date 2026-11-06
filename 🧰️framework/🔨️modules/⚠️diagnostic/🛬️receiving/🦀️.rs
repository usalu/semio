//! 🛬️ Diagnostic defaults are assembled from the authenticated original fields.
use crate::{Diagnostic,ExpectedSet,FaultCode,FaultScope,Severity,TextSpan};
use semio_framework_value_native_owned::native_decoding::owned::{NativeReceiveMap,NativeMappedReceiver,tree::NativeValueSource};
use semio_framework_value_native_owned::NativeReceive;

/// 🧩️ Holds nullable original expected-set fields until their paid assembly.
#[doc(hidden)]
#[derive(semio_framework_value::NativeReceive,semio_framework_value::RetireOwned)]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct ExpectedSetFields{tokens:Option<Vec<String>>,keywords:Option<Vec<String>>,keys:Option<Vec<String>>}
impl NativeReceiveMap<ExpectedSetFields> for ExpectedSet{
 fn from_native(fields:ExpectedSetFields)->Self{Self{tokens:fields.tokens.unwrap_or_default(),keywords:fields.keywords.unwrap_or_default(),keys:fields.keys.unwrap_or_default()}}
}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for ExpectedSet{
 type Receiver=NativeMappedReceiver<I,ExpectedSetFields,Self>;
 fn receiver()->Self::Receiver{NativeMappedReceiver::new()}
}

/// 🧩️ Holds nullable diagnostic fields and their original decoded children until assembly.
#[doc(hidden)]
#[derive(semio_framework_value::NativeReceive,semio_framework_value::RetireOwned)]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct DiagnosticFields{code:FaultCode,severity:Severity,span:TextSpan,message:String,expected:Option<ExpectedSet>,scope:Option<FaultScope>}
impl NativeReceiveMap<DiagnosticFields> for Diagnostic{
 fn from_native(fields:DiagnosticFields)->Self{Self{code:fields.code,severity:fields.severity,span:fields.span,message:fields.message,expected:fields.expected,scope:fields.scope.unwrap_or_default()}}
}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for Diagnostic{
 type Receiver=NativeMappedReceiver<I,DiagnosticFields,Self>;
 fn receiver()->Self::Receiver{NativeMappedReceiver::new()}
}

#[cfg(test)]
mod tests{
 use super::*;
 use semio_framework_value::{DslValue,FromValue};
 #[test]fn null_and_absent_expected_fields_follow_the_existing_wire_owner(){
  let absent=ExpectedSet::from_value(DslValue::from(serde_json::json!({}))).unwrap();
  let null=ExpectedSet::from_value(DslValue::from(serde_json::json!({"tokens":null,"keywords":null,"keys":null}))).unwrap();
  let fields=ExpectedSetFields{tokens:None,keywords:None,keys:None};
  let received=<ExpectedSet as NativeReceiveMap<ExpectedSetFields>>::from_native(fields);
  assert_eq!(absent,null);
  assert_eq!(received,absent);
 }
 #[test]fn null_and_absent_scope_follow_the_existing_wire_owner(){
  let base=serde_json::json!({"code":"DSL1","severity":"error","span":{"line":1,"column":2,"length":3},"message":"failure"});
  let mut null=base.clone();null["scope"]=serde_json::Value::Null;null["expected"]=serde_json::Value::Null;
  let absent=Diagnostic::from_value(DslValue::from(base)).unwrap();
  let existing_null=Diagnostic::from_value(DslValue::from(null)).unwrap();
  let fields=DiagnosticFields{code:FaultCode("DSL1".into()),severity:Severity::Error,span:TextSpan{line:1,column:2,length:3},message:"failure".into(),expected:None,scope:None};
  let received=<Diagnostic as NativeReceiveMap<DiagnosticFields>>::from_native(fields);
  assert_eq!(absent,existing_null);
  assert_eq!(received,absent);
 }
}
