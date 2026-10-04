//! 🧪️ Independent Serde JSON observations of explicitly owned record fields.
use semio_framework_dsl_record::{FieldValue,RecordValue};
pub fn fields(record:&RecordValue)->serde_json::Value{
 fn value(field:&FieldValue)->serde_json::Value{match field{FieldValue::Text(value)=>serde_json::json!(value),FieldValue::Float(value)=>serde_json::json!(value),FieldValue::Tuple(values)=>serde_json::Value::Array(values.iter().map(value).collect()),_=>panic!("closed binding corpus field")}}
 serde_json::Value::Array(record.fields.iter().map(|(id,field)|serde_json::json!([id,value(field)])).collect())
}

pub fn schema(spec:&semio_framework_dsl_record::RecordSpec)->serde_json::Value{serde_json::json!({"keyword":spec.keyword,"layout":format!("{:?}",spec.layout),"fields":spec.fields.iter().map(|field|serde_json::json!({"id":field.id,"key":field.key,"position":field.position,"shape":format!("{:?}",field.shape),"optional":field.optional,"flatten":field.flatten,"defines":field.defines,"callName":field.is_call_name})).collect::<Vec<_>>()})}
