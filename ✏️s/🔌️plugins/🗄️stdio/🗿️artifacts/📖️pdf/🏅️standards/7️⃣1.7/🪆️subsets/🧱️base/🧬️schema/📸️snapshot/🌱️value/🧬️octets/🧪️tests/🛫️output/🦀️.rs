//! 🛫️ Literal all-variant COS output matches ordinary ownership and independent Serde JSON.
use super::*;
#[test]
fn sqlite_snapshot_pdf17_controlled_cos_output_preserves_every_literal_variant(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🛫️cos-output.json")).unwrap();let maximum=corpus["maximumBytes"].as_u64().unwrap()as usize;let cases=corpus["cases"].as_array().unwrap();assert_eq!(cases.len(),10);
 for case in cases{let source=PdfObject::from_value(V::from(case["wire"].clone())).unwrap();let ordinary=source.to_value();assert_eq!(serde_json::to_value(&ordinary).unwrap(),case["wire"]);
  let mut accept=|_|true;let output=source.to_value_controlled(&mut pack::value::NativeEncodeControl::new(maximum,&mut accept)).unwrap();assert_eq!(output,ordinary);assert_eq!(serde_json::to_value(&output).unwrap(),case["wire"]);
  let mut accept=|_|true;let decoded=PdfObject::from_value_controlled(&output,&mut pack::value::NativeDecodeControl::new(maximum,&mut accept)).unwrap();assert_eq!(decoded,source);PdfObject::retire_decoded(decoded);
  let mut accept=|_|true;let mut control=pack::value::NativeEncodeControl::new(1,&mut accept);assert!(source.to_value_controlled(&mut control).is_err());assert_eq!(control.owned_bytes(),0);
  let mut cancel=|_|false;let mut control=pack::value::NativeEncodeControl::new(maximum,&mut cancel);assert!(source.to_value_controlled(&mut control).is_err());assert_eq!(control.owned_bytes(),0);PdfObject::retire_decoded(source);
 }
 println!("pdf-cos-output tags={} directions=2 serde-json-parity=10 preallocation-refusals=20",cases.len());
}
