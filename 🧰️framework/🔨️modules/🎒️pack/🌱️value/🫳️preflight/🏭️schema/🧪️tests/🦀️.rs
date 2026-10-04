//! 🔬️ Long immutable metadata compares and orders through actual borrowed schema work.
use super::*;
use semio_framework_dsl_record::{BorrowedFieldSpec as F,RecordLayout};
use std::sync::OnceLock;
fn fixture()->serde_json::Value{serde_json::from_str(include_str!("../../../🧪️tests/🫳️preflight/🧫️fixtures/🔣️.json")).unwrap()}
fn texts()->&'static[String;4]{static TEXTS:OnceLock<[String;4]>=OnceLock::new();TEXTS.get_or_init(||{let f=fixture();let m=&f["longSchemaMetadata"];let prefix=m["prefix"].as_str().unwrap().repeat(m["repeat"].as_u64().unwrap()as usize);let a=prefix.clone()+m["suffixes"][0].as_str().unwrap();let b=prefix+m["suffixes"][1].as_str().unwrap();[a.clone(),b.clone(),a,b]})}
fn labels(right:bool)->&'static[(&'static str,u32)]{static LEFT:OnceLock<[(&str,u32);2]>=OnceLock::new();static RIGHT:OnceLock<[(&str,u32);2]>=OnceLock::new();let start=if right{2}else{0};if right{&RIGHT}else{&LEFT}.get_or_init(||[(&texts()[start],0),(&texts()[start+1],0)])}
fn empty()->R{R{keyword:None,layout:RecordLayout::Inline,fields:&[]}}
fn variants(right:bool)->&'static[(&'static str,fn()->R)]{static LEFT:OnceLock<[(&str,fn()->R);2]>=OnceLock::new();static RIGHT:OnceLock<[(&str,fn()->R);2]>=OnceLock::new();let start=if right{2}else{0};if right{&RIGHT}else{&LEFT}.get_or_init(||[(&texts()[start],empty),(&texts()[start+1],empty)])}
fn enum_spec()->R{static FIELDS:OnceLock<[F;1]>=OnceLock::new();R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS.get_or_init(||[F::new(0,"label",H::Enum(labels(false)))])}}
fn field_spec(right:bool)->R{static LEFT:OnceLock<[F;1]>=OnceLock::new();static RIGHT:OnceLock<[F;1]>=OnceLock::new();R{keyword:None,layout:RecordLayout::Inline,fields:if right{&RIGHT}else{&LEFT}.get_or_init(||[F::new(0,&texts()[if right{2}else{0}],H::Int)])}}
fn graph()->Graph{let mut records=[field_spec(false);N];records[1]=field_spec(true);Graph{records,addresses:[usize::MAX;N],slots:[None;128],length:2}}
fn canceled(operation:impl FnOnce(&mut NativeEncodeControl<'_>)->Result<(),ValueError>){let mut hit=false;let mut cancel=|p:semio_framework_value::native_encoding::NativeEncodeProgress|{if p.total>=texts()[0].len()&&p.completed>=65536{hit=true;false}else{true}};let mut control=NativeEncodeControl::new(0,&mut cancel);let((kind,diagnostic),requests,releases)=crate::test_allocation::observe_backing(||{let error=operation(&mut control).expect_err("actual metadata interior cancellation");let result=(error.kind,error.message.capacity());drop(error);result});assert_eq!(kind,K::Canceled);assert_eq!(requests,releases);assert_eq!(requests,diagnostic);assert_eq!(control.owned_bytes(),0);drop(control);assert!(hit);}
fn integer(out:&mut Vec<u8>,mut n:usize){while n>=128{out.push((n as u8&127)|128);n>>=7;}out.push(n as u8);}
fn text(out:&mut Vec<u8>,s:&str){integer(out,s.len());out.extend_from_slice(s.as_bytes());}
#[test]
fn record_borrowed_preflight_long_schema_metadata_hash_parity_and_paged_ordering(){
 let f=fixture();let mut bytes=vec![1,1,0];text(&mut bytes,"label");bytes.extend_from_slice(&[0,7,2]);for index in [1,0]{integer(&mut bytes,0);text(&mut bytes,&texts()[index]);}let expected=*blake3::hash(&bytes).as_bytes();let hex:String=expected.iter().map(|byte|format!("{byte:02x}")).collect();assert_eq!(hex,f["longSchemaMetadata"]["schemaHash"]);
 let mut allow=|_|true;let mut control=NativeEncodeControl::new(0,&mut allow);let(actual,requests,releases)=crate::test_allocation::observe_backing(||hash(enum_spec(),&mut control));assert_eq!(actual.unwrap(),expected);assert_eq!((requests,releases,control.owned_bytes()),(0,0,0));canceled(|control|hash(enum_spec(),control).map(|_|()));
 println!("[DEBUG] actual borrowed long enum hash matches independent BLAKE3, zero heap scratch and caller interior cancellation");
}
#[test]
fn record_borrowed_preflight_long_schema_metadata_equivalence_is_cancellable(){
 let graph=graph();let f=fixture();for case in f["longSchemaMetadata"]["comparisonCases"].as_array().unwrap(){let case=case.as_str().unwrap();let operation=|control:&mut NativeEncodeControl<'_>|match case{"field"=>equal_record(&graph,0,1,None,control),"enum"=>equal_shape(&graph,H::Enum(labels(false)),H::Enum(labels(true)),None,0,control),"statement"=>equal_shape(&graph,H::Statements(variants(false)),H::Statements(variants(true)),None,0,control),"reference"=>equal_shape(&graph,H::Ref(&texts()[0]),H::Ref(&texts()[2]),None,0,control),_=>panic!("closed metadata comparison")};let mut allow=|_|true;let mut control=NativeEncodeControl::new(0,&mut allow);assert!(operation(&mut control).unwrap());canceled(|control|operation(control).map(|_|()));}
 println!("[DEBUG] actual borrowed field, enum, statement and reference equivalence consults same caller in long byte comparisons");
}
