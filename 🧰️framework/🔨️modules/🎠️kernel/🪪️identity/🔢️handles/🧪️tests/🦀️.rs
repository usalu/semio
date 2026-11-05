//! 🆔️ Serde, the value codec and bounded native scalar receiving agree at the exact full-width boundary.
use super::*;
use semio_framework_value::{DslValue,FromValue,ToValue};
#[test]
fn canonical_u128_handles_preserve_full_string_identity_without_numeric_fallback(){
 let c:serde_json::Value=serde_json::from_str(include_str!("../🔣️.json")).unwrap();
 for row in c["cases"].as_array().unwrap(){let wire=row["wire"].as_str().unwrap();let accepted=row["accepted"].as_bool().unwrap();let value:DslValue=DslValue::from(serde_json::from_str::<serde_json::Value>(wire).unwrap());
  macro_rules! check{($ty:ty)=>{{let oracle=serde_json::from_str::<$ty>(wire);let native=<$ty as NativeScalarValue<DslValue>>::read(&value);let cold=<$ty as FromValue>::from_value(value.clone());assert_eq!(oracle.is_ok(),accepted);assert_eq!(native.is_ok(),accepted);assert_eq!(cold.is_ok(),accepted);if let Ok(original)=oracle{assert_eq!(native.unwrap(),original);assert_eq!(cold.unwrap(),original);assert_eq!(serde_json::to_string(&original).unwrap(),wire);assert_eq!(original.to_value(),value);assert_eq!(<$ty as NativeScalarValue<DslValue>>::read_copy_bytes(&value).unwrap(),128);}}};}
  check!(ArtifactHandle);check!(WindowHandle);
 }
}
