//! 🧫️ Literal protocol boundaries, known physical work and bounded iterative scopes.
use crate::{parse_protocol,print_protocol,walk_protocol,walk_literal_protocol_controlled,NativeDecodeControl};
fn fixture()->serde_json::Value{serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()}
#[test]
fn protocol_literal_bit_fields_have_neutral_buffer_witness(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧮️bits/🔣️.json")).unwrap();
 for case in fixture["cases"].as_array().unwrap(){
  let source=fixture["source"].as_str().unwrap().replace("MASK",case["mask"].as_str().unwrap());
  let spec=parse_protocol(&source).unwrap();assert_eq!(parse_protocol(&print_protocol(&spec)).unwrap(),spec);
  let word: u64=case["word"].as_str().unwrap().parse().unwrap();let mut bytes=word.to_le_bytes().to_vec();if case["present"].as_bool().unwrap(){bytes.push(fixture["payload"].as_u64().unwrap()as u8);}
  assert_eq!(walk_protocol(&spec,&bytes).unwrap().consumed,bytes.len());
  if case["present"].as_bool().unwrap(){assert!(walk_protocol(&spec,&bytes[..8]).is_err());}else{bytes.push(91);assert!(walk_protocol(&spec,&bytes).is_err());}
 }
 for mask in fixture["invalidMasks"].as_array().unwrap(){assert!(parse_protocol(&fixture["source"].as_str().unwrap().replace("MASK",mask.as_str().unwrap())).is_err());}
 eprintln!("[DEBUG] native bit-field conditions admit eight neutral unsigned64 Buffer witnesses");
}
#[test]
fn sqlite_snapshot_literal_protocol_lexical_octets_and_full_unsigned_domain(){
 let fixture=fixture();let grammar=crate::parse_grammar(fixture["lexical"]["grammar"].as_str().unwrap()).unwrap();let recognizer=crate::Recognizer::compile(&grammar, &crate::FragmentRegistry::new(), Vec::new()).expect("selected fragments");
 for case in fixture["lexical"]["cases"].as_array().unwrap(){assert_eq!(recognizer.recognize(case["source"].as_str().unwrap()).unwrap_or(false),case["valid"].as_bool().unwrap(),"{}",case["source"]);}
 let grammar=crate::parse_grammar(fixture["scalars"]["grammar"].as_str().unwrap()).unwrap();let recognizer=crate::Recognizer::compile(&grammar, &crate::FragmentRegistry::new(), Vec::new()).expect("selected fragments");for case in fixture["scalars"]["cases"].as_array().unwrap(){assert_eq!(recognizer.recognize(case["source"].as_str().unwrap()).unwrap_or(false),case["valid"].as_bool().unwrap(),"{}",case["source"]);}
}
fn varint(mut value:u64,bytes:&mut Vec<u8>){loop{let byte=(value&127)as u8;value>>=7;bytes.push(byte|if value==0{0}else{128});if value==0{break}}}
#[test]
fn sqlite_snapshot_literal_protocol_named_records_reject_truncated_and_trailing_fields(){
 let fixture=fixture();let spec=parse_protocol(fixture["source"].as_str().unwrap()).unwrap();assert_eq!(parse_protocol(&print_protocol(&spec)).unwrap(),spec);let bytes=fixture["binary"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect::<Vec<_>>();assert_eq!(walk_protocol(&spec,&bytes).unwrap().consumed,bytes.len());for end in 0..bytes.len(){assert!(walk_protocol(&spec,&bytes[..end]).is_err(),"{end}");}let mut trailing=bytes.clone();trailing.push(0);assert!(walk_protocol(&spec,&trailing).is_err());
}
#[test]
fn sqlite_snapshot_literal_protocol_deep_scopes_and_large_primitives_are_controlled(){
 let fixture=fixture();std::thread::Builder::new().stack_size(256*1024).spawn(move||{let spec=parse_protocol(fixture["source"].as_str().unwrap()).unwrap();let depth=fixture["depth"].as_u64().unwrap();let mut deep=vec![1,1];for _ in 0..depth{deep.extend_from_slice(&[1,0]);}deep.extend_from_slice(&[0,0,7]);let budget=fixture["fullOwnershipBudget"].as_u64().unwrap()as usize;assert_eq!(walk_literal_protocol_controlled(&spec,&deep,&mut NativeDecodeControl::new(budget,&mut |_|true)).unwrap().consumed,deep.len());let mut callback=|_|true;let mut tiny=NativeDecodeControl::new(1,&mut callback);assert!(walk_literal_protocol_controlled(&spec,&deep,&mut tiny).is_err());assert_eq!(tiny.owned_bytes(),0);
 let count=fixture["textBytes"].as_u64().unwrap();let mut bytes=vec![1,1,0];varint(count,&mut bytes);bytes.resize(bytes.len()+count as usize,b'x');bytes.push(7);let mut reached=false;let mut callback=|event:semio_framework_value::native_decoding::NativeDecodeProgress|{if event.completed>=256&&event.completed<event.total{reached=true;false}else{true}};assert!(walk_literal_protocol_controlled(&spec,&bytes,&mut NativeDecodeControl::new(budget,&mut callback)).is_err());assert!(reached);}).unwrap().join().unwrap();
}
