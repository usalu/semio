//! 🧱️ Plugin-owned native records agree with the portable JSON corpus and independent Serde.
use semio_s_plugin_block::*;
fn roundtrip<T: serde::de::DeserializeOwned + serde::Serialize + semio_framework_value::ToValue + semio_framework_value::FromValue>(input:serde_json::Value)->serde_json::Value{
 let value:T=serde_json::from_value(input.clone()).expect("native shared record");
 let mut encode_calls=0usize;let mut encode_progress=|event:semio_framework_value::native_encoding::NativeEncodeProgress|{encode_calls+=1;event.owned_bytes<=65536&&(event.total==0||event.completed<=event.total)};
 let mut encode=semio_framework_value::NativeEncodeControl::new(65536,&mut encode_progress);
 let owned=value.to_value_controlled(&mut encode).expect("controlled shared record projection");drop(encode);assert!(encode_calls>0);assert_eq!(serde_json::Value::from(&owned),input);
 let mut decode_calls=0usize;let mut decode_progress=|event:semio_framework_value::native_decoding::NativeDecodeProgress|{decode_calls+=1;event.owned_bytes<=65536&&(event.total==0||event.completed<=event.total)};
 let mut recipient=semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();let mut decode=semio_framework_value::NativeDecodeControl::new(65536,&mut decode_progress);decode.install_retirement_recipient(&mut recipient).expect("explicit shared record retirement recipient");
 let actual=T::from_value_controlled(&owned,&mut decode).expect("controlled shared record construction");drop(decode);assert!(decode_calls>0);assert!(semio_framework_value::ErasedSnapshotRetirement::terminal_is_empty(&recipient));
 serde_json::to_value(actual).expect("independent shared record output")
}
#[test]
fn shared_plugin_records_match_the_portable_schema_vectors(){let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).expect("shared corpus");assert_eq!(fixture["schemaVersion"],1);let rows=fixture["cases"].as_array().expect("eight records");assert_eq!(rows.len(),8);for row in rows{let input=row["input"].clone();let actual=match row["type"].as_str().expect("type"){"BlockKindIdentity"=>roundtrip::<BlockKindIdentity>(input.clone()),"BlockAttribute"=>roundtrip::<BlockAttribute>(input.clone()),"BlockAuthor"=>roundtrip::<BlockAuthor>(input.clone()),"BlockCompatibilityRule"=>roundtrip::<BlockCompatibilityRule>(input.clone()),"BlockRepresentation"=>roundtrip::<BlockRepresentation>(input.clone()),"BlockCamera2d"=>roundtrip::<BlockCamera2d>(input.clone()),"BlockCamera3d"=>roundtrip::<BlockCamera3d>(input.clone()),"BlockMeta"=>roundtrip::<BlockMeta>(input.clone()),_=>panic!("unknown shared type")};assert_eq!(actual,input,"{}",row["type"]);eprintln!("[DEBUG] Native Block plugin shared schema {}",row["type"]);}}
#[test]
fn shared_retirement_is_owned_by_the_plugin(){fn owner<T:semio_framework_value::retirement::RetireOwned>(){} owner::<BlockKindIdentity>();owner::<BlockAttribute>();owner::<BlockAuthor>();owner::<BlockCompatibilityRule>();owner::<BlockRepresentation>();owner::<BlockCamera2d>();owner::<BlockCamera3d>();owner::<BlockMeta>();}
