//! 📦️ Native IO executes the same byte witnesses under explicitly authored original controls.
use crate::store;
use semio_framework_value::{RetireOwned,RetainedCloneGrant,ValueError};
use semio_framework_test_original_snapshot_owners::{with_decoding_granted, with_encoding};
use store::io::io_mechanism::IoRunControl;
use store::io_schema::{IoPayload,IoResult};
fn fixture()->serde_json::Value{serde_json::from_str(include_str!("🧫️fixtures/📦️archive.json")).unwrap()}
fn grant()->RetainedCloneGrant{serde_json::from_value(fixture()["control"]["snapshotGrant"].clone()).unwrap()}
fn native_maximum()->usize { fixture()["control"]["nativeBytes"].as_u64().unwrap() as usize }
pub(crate) fn with_control<T>(operation:impl FnOnce(&mut IoRunControl<'_,'_>)->T)->T{
 let grant = grant();
 with_decoding_granted(grant, &mut |_| true, |owner| {
  owner.native().scoped_maximum(native_maximum(), |native| Ok::<T, ValueError>(operation(&mut IoRunControl::decoding(native, grant))))
 }).expect("authored archive decode ceiling")
}
fn with_export_control<T>(operation:impl FnOnce(&mut IoRunControl<'_,'_>)->T)->T{
 let grant = grant();
 with_encoding(|owner| {
  owner.native().scoped_maximum(native_maximum(), |native| Ok::<T, ValueError>(operation(&mut IoRunControl::encoding(native, grant))))
 }).expect("authored archive encode ceiling")
}
pub(crate) fn retire<T:RetireOwned>(value:T){crate::jack::standards::v1::subsets::any::schema::content::retire_owned_to_terminal(value).expect("archive fixture retires");}
pub(crate) fn round_trip(bytes:&[u8],format:usize)->Vec<u8>{
 let input=IoPayload::Binary(bytes.to_vec());let entries=crate::io::entries();
 let encoded=with_export_control(|original|(entries[format*2].run)(&input,original)).unwrap().value;
 let decoded=with_control(|original|(entries[format*2+1].run)(&encoded,original)).unwrap().value;retire(encoded);retire(input);
 let IoPayload::Binary(bytes)=decoded else{panic!("original native binary output required")};bytes
}
#[test]
fn archive_carriers_preserve_original_octets_against_serde_json(){
 for row in fixture()["cases"].as_array().unwrap(){let bytes:Vec<u8>=serde_json::from_value(row["bytes"].clone()).unwrap();for (format,key)in[(0,"json"),(1,"txt")]{let input=IoPayload::Binary(bytes.clone());let entries=crate::io::entries();let encoded=with_export_control(|control|(entries[format*2].run)(&input,control)).unwrap().value;let IoPayload::Text(text)=&encoded else{panic!("text output required")};assert_eq!(text,row[key].as_str().unwrap());if format==0{let independent:serde_json::Value=serde_json::from_str(text).unwrap();assert_eq!(independent["schema"],fixture()["schema"]);assert_eq!(independent["archive"].as_str().unwrap().len(),bytes.len()*2);}let decoded=with_control(|control|(entries[format*2+1].run)(&encoded,control)).unwrap().value;assert_eq!(decoded,IoPayload::Binary(bytes.clone()));retire(decoded);retire(encoded);retire(input);}retire(bytes);}
}
#[test]
fn archive_carriers_refuse_closed_schema_and_incomplete_octets(){for row in fixture()["refusals"].as_array().unwrap(){let input=IoPayload::Text(row["text"].as_str().unwrap().into());let entry=if row["format"]=="json"{1}else{3};let outcome:IoResult<IoPayload>=with_control(|control|(crate::io::entries()[entry].run)(&input,control));assert!(outcome.is_err());retire(outcome.err().unwrap().cause);retire(input);}}
