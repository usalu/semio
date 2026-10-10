//! 🫙️ Container decoding selects the original caller-controlled visitor before generic field ownership.
use super::*;
use crate::ValueRefusalKind;

#[derive(Debug,PartialEq,crate::FromValue,serde::Deserialize)]
#[value(crate="crate",deserialize_controlled_with="read_original",deny_unknown_fields)]
#[serde(deny_unknown_fields)]
struct Original { value:i64 }

fn read_original(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Original,ValueError>{
 control.checkpoint()?;
 let DslValue::Object(fields)=value else{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original container must be an object"))};
 if fields.len()!=1||fields[0].0!="value"{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original container must have its sole field"))}
 let result=i64::from_value_controlled(&fields[0].1,control)?;
 Ok(Original{value:result})
}

#[test]
fn controlled_container_visitor_preserves_zero_heap_neutral_fields_and_cancellation(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let oracle:Original=serde_json::from_value(row.clone()).unwrap();let original=DslValue::from(row);
  let mut callback=|_|true;let mut control=NativeDecodeControl::new(0,&mut callback);
  assert_eq!(Original::from_value_controlled(&original,&mut control).unwrap(),oracle);assert_eq!(control.owned_bytes(),0);
  let mut cancel=|_|false;let mut control=NativeDecodeControl::new(0,&mut cancel);
  assert_eq!(Original::from_value_controlled(&original,&mut control).unwrap_err().kind,ValueRefusalKind::Canceled);assert_eq!(control.owned_bytes(),0);
 }
 for row in fixture["malformed"].as_array().unwrap(){let original=DslValue::from(row);let mut callback=|_|true;let mut control=NativeDecodeControl::new(0,&mut callback);assert!(serde_json::from_value::<Original>(row.clone()).is_err());assert!(Original::from_value_controlled(&original,&mut control).is_err());assert_eq!(control.owned_bytes(),0);}
 println!("[DEBUG] original controlled container visitor: three neutral rows, three malformed rows, three cancellation boundaries");
}
