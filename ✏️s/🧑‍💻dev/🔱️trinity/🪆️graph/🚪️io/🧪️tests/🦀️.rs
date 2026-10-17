//! 📦️ Native IO executes the same byte witnesses under explicitly authored original controls.
use crate::store;
use semio_framework_value::{RetireOwned,RetainedCloneGrant};
use store::io::io_mechanism::IoRunControl;
use store::io_schema::{IoPayload,IoResult};
fn fixture()->serde_json::Value{serde_json::from_str(include_str!("🧫️fixtures/📦️archive.json")).unwrap()}
fn grant()->RetainedCloneGrant{serde_json::from_value(fixture()["control"]["snapshotGrant"].clone()).unwrap()}
pub(crate) fn with_control<T>(operation:impl FnOnce(&mut IoRunControl<'_,'_>)->T)->T{
 let maximum=fixture()["control"]["nativeBytes"].as_u64().unwrap()as usize;let mut receive=|_|true;let mut publish=|_|true;
 let mut decode=semio_framework_value::NativeDecodeControl::new(maximum,&mut receive);let mut encode=semio_framework_value::NativeEncodeControl::new(maximum,&mut publish);
 let mut decode_recipient=semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();let mut encode_recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
 decode.install_retirement_recipient(&mut decode_recipient).unwrap();encode.install_retirement_recipient(&mut encode_recipient).unwrap();
 let result={let mut original=IoRunControl::new(&mut decode,&mut encode,grant());operation(&mut original)};
 while decode.has_retirement_owner(){decode.close_retirement_recipient(grant()).unwrap();}while encode.has_retirement_owner(){encode.close_retirement_recipient(grant()).unwrap();}result
}
pub(crate) fn retire<T:RetireOwned>(value:T){let mut original=semio_framework_value::retirement::controlled::ControlledRetirement::new(Some(value)).ok().unwrap();while !original.terminal_is_empty(){original.step(grant()).unwrap();}}
pub(crate) fn round_trip(bytes:&[u8],format:usize)->Vec<u8>{
 let input=IoPayload::Binary(bytes.to_vec());let entries=crate::io::entries();
 let encoded=with_control(|original|(entries[format*2].run)(&input,original)).unwrap().value;
 let decoded=with_control(|original|(entries[format*2+1].run)(&encoded,original)).unwrap().value;retire(encoded);retire(input);
 let IoPayload::Binary(bytes)=decoded else{panic!("original native binary output required")};bytes
}
#[test]
fn archive_carriers_preserve_original_octets_against_serde_json(){
 for row in fixture()["cases"].as_array().unwrap(){let bytes:Vec<u8>=serde_json::from_value(row["bytes"].clone()).unwrap();for (format,key)in[(0,"json"),(1,"txt")]{let input=IoPayload::Binary(bytes.clone());let entries=crate::io::entries();let encoded=with_control(|control|(entries[format*2].run)(&input,control)).unwrap().value;let IoPayload::Text(text)=&encoded else{panic!("text output required")};assert_eq!(text,row[key].as_str().unwrap());if format==0{let independent:serde_json::Value=serde_json::from_str(text).unwrap();assert_eq!(independent["schema"],fixture()["schema"]);assert_eq!(independent["archive"].as_str().unwrap().len(),bytes.len()*2);}let decoded=with_control(|control|(entries[format*2+1].run)(&encoded,control)).unwrap().value;assert_eq!(decoded,IoPayload::Binary(bytes.clone()));retire(decoded);retire(encoded);retire(input);}retire(bytes);}
}
#[test]
fn archive_carriers_refuse_closed_schema_and_incomplete_octets(){for row in fixture()["refusals"].as_array().unwrap(){let input=IoPayload::Text(row["text"].as_str().unwrap().into());let entry=if row["format"]=="json"{1}else{3};let outcome:IoResult<IoPayload>=with_control(|control|(crate::io::entries()[entry].run)(&input,control));assert!(outcome.is_err());retire(outcome.err().unwrap().cause);retire(input);}}
