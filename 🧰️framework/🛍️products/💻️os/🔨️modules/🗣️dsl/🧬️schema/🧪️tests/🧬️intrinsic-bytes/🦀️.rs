//! 🧬️ Owned intrinsic octets retain one value node across native text, Pack and JSON boundaries.
use super::*;
use std::{io::Write,process::{Command,Stdio}};
#[test]
fn owned_intrinsic_bytes_native_literals_and_pack_match_independent_base64(){
    let fixture=include_str!("../../../../../../../🔨️modules/🌱️value/🧬️bytes/🧪️tests/🧬️base64/🧫️fixtures/🔣️.json");let schema=include_str!("../../../../../../../🔨️modules/🌱️value/🧬️bytes/🧪️tests/🧬️base64/🧬️schema/🔣️.json");
    let input:serde_json::Value=serde_json::from_str(fixture).unwrap();
    let script="import Ajv from 'ajv/dist/2020.js';import {Buffer} from 'node:buffer';const {fixture,schema}=JSON.parse(await Bun.stdin.text());const check=new Ajv({strict:true}).compile(schema);if(!check(fixture))throw Error(JSON.stringify(check.errors));for(const c of fixture.cases){if(Buffer.from(c.octets).toString('base64')!==c.base64||JSON.stringify([...Buffer.from(c.base64,'base64')])!==JSON.stringify(c.octets))throw Error(c.name);}await Bun.write(Bun.stdout,'ok');";
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(serde_json::to_string(&serde_json::json!({"fixture":input,"schema":serde_json::from_str::<serde_json::Value>(schema).unwrap()})).unwrap().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));assert_eq!(output.stdout,b"ok");
    let spec=RecordSpec::new(None,RecordLayout::Lines,vec![FieldSpec::new(1,"data",Shape::Value)]);
    for case in input["cases"].as_array().unwrap(){let bytes=case["octets"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as u8).collect::<Vec<_>>();let value=DslValue::Bytes(bytes.clone());let source=format!("data=bytes64(\"{}\")",case["base64"].as_str().unwrap());let record=parse(&source,&spec,&ParseOptions::default()).unwrap();assert_eq!(record.get(1),Some(&FieldValue::Value(value.clone())));assert_eq!(print(&record,&spec,JoinMode::Inline),source);let wire=crate::store::pack_rt::encode_wire_value(&value);assert_eq!(crate::store::pack_rt::decode_wire_value(&wire).unwrap(),value);assert_eq!(serde_json::Value::from(&value),case["octets"]);assert_eq!(protocol::bytes::from_value(value).unwrap(),bytes);assert_eq!(protocol::bytes::from_value(DslValue::from(&case["octets"])).unwrap(),bytes);}
    for encoded in input["invalid"].as_array().unwrap(){assert!(parse(&format!("data=bytes64(\"{}\")",encoded.as_str().unwrap()),&spec,&ParseOptions::default()).is_err());}
    for invalid in [serde_json::json!([-1]),serde_json::json!([256]),serde_json::json!([1.5]),serde_json::json!([true]),serde_json::json!("AA==")]{assert!(protocol::bytes::from_value(DslValue::from(&invalid)).is_err());}
    assert_eq!(protocol::bytes::optional::from_value(DslValue::Null).unwrap(),None);assert_eq!(protocol::bytes::optional::from_value(DslValue::Bytes(Vec::new())).unwrap(),Some(Vec::new()));
    let data=DslValue::Bytes(vec![0xab;131_072]);let record=RecordValue{fields:[(1,FieldValue::Value(data.clone()))].into_iter().collect()};let wire=crate::store::pack_rt::encode_wire_value(&data);let options=crate::store::PackDecodeOptions{limits:crate::store::PackLimits{max_total_alloc:64,..Default::default()},..Default::default()};assert!(crate::store::pack_rt::decode_wire_value_with_options(&wire,&options).is_err());
    let text=print(&record,&spec,JoinMode::Inline);let options=ParseOptions{limits:Limits{max_bytes:256*1024,max_tokens:8,max_nodes:1,..Limits::default()},..ParseOptions::default()};assert_eq!(parse(&text,&spec,&options).unwrap().get(1),Some(&FieldValue::Value(data)));assert!(parse(&text,&spec,&ParseOptions{limits:Limits{max_bytes:64,..Limits::default()},..ParseOptions::default()}).is_err());
}
#[test]
fn owned_intrinsic_bytes_chunk_budget_rejects_before_payload_integrity_work(){
    let spec=RecordSpec::new(None,RecordLayout::Lines,vec![FieldSpec::new(1,"data",Shape::Value)]);
    let value=DslValue::Bytes(vec![0xab;131072]);let record=RecordValue{fields:[(1,FieldValue::Value(value))].into_iter().collect()};
    let options=crate::store::PackEncodeOptions{codec:crate::codec::CodecId(0),chunk_threshold:1,chunk_size:65536,..Default::default()};
    let mut bytes=crate::store::pack_rt::encode_document(&spec,&record,&options).unwrap();
    let file=::semio_framework_async::poll::resolve_ready(crate::os_pack::format::PackFile::open_manifest(bytes.as_slice(),&crate::store::PackLimits::default(),crate::os_pack::format::VerificationLevel::Standard)).unwrap();
    let offset=file.chunk_range(crate::codec::ids::ChunkId(0)).unwrap().offset as usize;drop(file);bytes[offset]^=1;
    let options=crate::store::PackDecodeOptions{limits:crate::store::PackLimits{max_total_alloc:4096,..Default::default()},..Default::default()};
    assert!(matches!(crate::store::pack_rt::decode_document(&bytes,&spec,&options),Err(crate::store::PackError::Refusal(pack::PackRefusal::LimitExceeded { kind: semio_framework_value::ValueRefusalKind::OwnershipLimit, .. }))));
}

#[test]
fn owned_intrinsic_bytes_controlled_base64_bounds_and_cancellation(){
    use protocol::{Base64Control as Control,Base64ControlError as Error,Base64Phase as Phase,base64_standard_encode_controlled as encode,base64_standard_decode_controlled as decode};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../🔨️modules/🌱️value/🧬️bytes/🧪️tests/🧬️base64/🧫️fixtures/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap(){let bytes=case["octets"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as u8).collect::<Vec<_>>();let text=case["base64"].as_str().unwrap();assert_eq!(encode(&bytes,&mut Control{maximum_output_bytes:text.len(),progress:&mut |_|true}).unwrap(),text);assert_eq!(decode(text.as_bytes(),&mut Control{maximum_output_bytes:bytes.len(),progress:&mut |_|true}).unwrap(),bytes);}
    let bytes=vec![0xab;131072];let text=protocol::base64_standard_encode(&bytes);
    assert_eq!(encode(&bytes,&mut Control{maximum_output_bytes:text.len()-1,progress:&mut |_|true}).unwrap_err(),Error::OutputLimit);
    assert_eq!(decode(text.as_bytes(),&mut Control{maximum_output_bytes:bytes.len()-1,progress:&mut |_|true}).unwrap_err(),Error::OutputLimit);
    assert_eq!(encode(&bytes,&mut Control{maximum_output_bytes:usize::MAX,progress:&mut |_|false}).unwrap_err(),Error::Cancelled);
    assert_eq!(decode(text.as_bytes(),&mut Control{maximum_output_bytes:usize::MAX,progress:&mut |_|false}).unwrap_err(),Error::Cancelled);
    for decode_pass in [false,true]{let mut events=Vec::new();let mut callback=|event:protocol::Base64Progress|{events.push(event);event.completed<8192};let mut control=Control{maximum_output_bytes:usize::MAX,progress:&mut callback};let error=if decode_pass{decode(text.as_bytes(),&mut control).unwrap_err()}else{encode(&bytes,&mut control).unwrap_err()};assert_eq!(error,Error::Cancelled);assert_eq!(events[0].completed,0);assert!(events.last().unwrap().completed>=8192);for pair in events.windows(2){if pair[0].phase==pair[1].phase{assert!(pair[1].completed-pair[0].completed<=4096);}}if decode_pass{assert!(events.iter().all(|event|event.phase==Phase::Validate));}}
    for text in fixture["invalid"].as_array().unwrap(){assert!(matches!(decode(text.as_str().unwrap().as_bytes(),&mut Control{maximum_output_bytes:usize::MAX,progress:&mut |_|true}),Err(Error::Codec(_))));}
}
