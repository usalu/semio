//! 🛬️ Schema and independent serializer checks for retained diagnostic leaves.
use crate::{Severity,TextSpan};
use semio_framework_value::{DslValue,ToValue};

#[test]
fn severity_spellings_and_span_follow_the_schema(){
 let schema:serde_json::Value=serde_json::from_str(include_str!("../../🧬️schema/🎛️controlled/🔣️.json")).unwrap();
 let names=schema["$defs"]["Severity"]["enum"].as_array().unwrap();
 let variants=[Severity::Info,Severity::Warning,Severity::Error,Severity::Fatal];
 assert_eq!(names.len(),variants.len());
 for (value,name) in variants.into_iter().zip(names){assert_eq!(value.to_value(),DslValue::String(name.as_str().unwrap().to_owned()));}
 let span=TextSpan{line:4,column:7,length:2};
 let reference=serde_json::to_value(span).unwrap();
 assert_eq!(reference,serde_json::json!({"line":4,"column":7,"length":2}));
 assert_eq!(span.to_value(),DslValue::Object(vec![("line".into(),DslValue::Number(semio_framework_value::Number::UInt(4))),("column".into(),DslValue::Number(semio_framework_value::Number::UInt(7))),("length".into(),DslValue::Number(semio_framework_value::Number::UInt(2)))]));
 assert_eq!(schema["$defs"]["TextSpan"]["required"],serde_json::json!(["line","column","length"]));
}
