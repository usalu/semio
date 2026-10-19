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
fn canceled(operation:impl FnOnce(&mut NativeEncodeControl<'_>)->Result<(),ValueError>){let mut hit=false;let mut cancel=|p:semio_framework_value::native_encoding::NativeEncodeProgress|{if p.total>=texts()[0].len()&&p.completed>=65536{hit=true;false}else{true}};let mut control=NativeEncodeControl::new(0,&mut cancel);let((kind,diagnostic),requests,releases)=semio_framework_trace_memory_testing::observe_backing(||{let error=operation(&mut control).expect_err("actual metadata interior cancellation");let result=(error.kind,match &error.message { std::borrow::Cow::Borrowed(_) => 0, std::borrow::Cow::Owned(message) => message.capacity() });drop(error);result});assert_eq!(kind,K::Canceled);assert_eq!(requests,releases);assert_eq!(requests,diagnostic);assert_eq!(control.owned_bytes(),0);drop(control);assert!(hit);}
fn integer(out:&mut Vec<u8>,mut n:usize){while n>=128{out.push((n as u8&127)|128);n>>=7;}out.push(n as u8);}
fn text(out:&mut Vec<u8>,s:&str){integer(out,s.len());out.extend_from_slice(s.as_bytes());}
#[test]
fn record_borrowed_preflight_long_schema_metadata_hash_parity_and_paged_ordering(){
 let f=fixture();let mut bytes=vec![1,1,0];text(&mut bytes,"label");bytes.extend_from_slice(&[0,7,2]);for index in [1,0]{integer(&mut bytes,0);text(&mut bytes,&texts()[index]);}let expected=*blake3::hash(&bytes).as_bytes();let hex:String=expected.iter().map(|byte|format!("{byte:02x}")).collect();assert_eq!(hex,f["longSchemaMetadata"]["schemaHash"]);
 let mut allow=|_|true;let mut control=NativeEncodeControl::new(0,&mut allow);let(actual,requests,releases)=semio_framework_trace_memory_testing::observe_backing(||hash(enum_spec(),&mut control));assert_eq!(actual.unwrap(),expected);assert_eq!((requests,releases,control.owned_bytes()),(0,0,0));canceled(|control|hash(enum_spec(),control).map(|_|()));
}
#[test]
fn record_borrowed_preflight_long_schema_metadata_equivalence_is_cancellable(){
 let graph=graph();let f=fixture();for case in f["longSchemaMetadata"]["comparisonCases"].as_array().unwrap(){let case=case.as_str().unwrap();let operation=|control:&mut NativeEncodeControl<'_>|match case{"field"=>equal_record(&graph,0,1,None,control),"enum"=>equal_shape(&graph,H::Enum(labels(false)),H::Enum(labels(true)),None,0,control),"statement"=>equal_shape(&graph,H::Statements(variants(false)),H::Statements(variants(true)),None,0,control),"reference"=>equal_shape(&graph,H::Ref(&texts()[0]),H::Ref(&texts()[2]),None,0,control),_=>panic!("closed metadata comparison")};let mut allow=|_|true;let mut control=NativeEncodeControl::new(0,&mut allow);assert!(operation(&mut control).unwrap());canceled(|control|operation(control).map(|_|()));}
}

fn cyclic()->R{static FIELDS:OnceLock<[F;2]>=OnceLock::new();R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS.get_or_init(||[F::new(7,"next",H::Record(cyclic)),F::new(2,"value",H::Int)])}}
fn wide()->R{static FIELDS:OnceLock<[F;300]>=OnceLock::new();R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS.get_or_init(||std::array::from_fn(|index|F::new((299-index)as u16,"wide",H::Int)))}}
fn statement_spec()->R{static FIELDS:OnceLock<[F;1]>=OnceLock::new();R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS.get_or_init(||[F::new(0,"statement",H::Statements(variants(false)))])}}
fn mapped_record()->H{H::Map(||H::Record(empty))}
fn wrapper_spec()->R{static FIELDS:[F;2]=[F::new(0,"nested",H::List(||H::Block(||H::Tuple(mapped_record,Some(3))))),F::new(1,"table",H::Table(empty))];R{keyword:None,layout:RecordLayout::Inline,fields:&FIELDS}}
fn shared()->R{static FIELDS:OnceLock<[F;2]>=OnceLock::new();R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS.get_or_init(||[F::new(1,"left",H::Record(cyclic)),F::new(2,"right",H::Record(cyclic))])}}
#[test]
fn record_borrowed_schema_hash_retains_exact_paged_work_and_cancellation(){
 use semio_framework_value::RetainedCloneGrant;
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/⏳️cursor/🔣️.json")).unwrap();
 let mut canonical=vec![1];integer(&mut canonical,300);for id in 0..300{integer(&mut canonical,id);text(&mut canonical,"wide");canonical.extend_from_slice(&[0,2]);}
 assert_eq!(corpus["cases"][2]["fieldCount"],300);
 for(case,spec)in [enum_spec(),cyclic(),shared(),wide(),statement_spec(),wrapper_spec()].into_iter().enumerate(){
  let bytes=match case{
   0=>{let mut bytes=vec![1,1,0];text(&mut bytes,"label");bytes.extend_from_slice(&[0,7,2]);for index in [1,0]{integer(&mut bytes,0);text(&mut bytes,&texts()[index]);}bytes},
   3=>canonical.clone(),
   4=>{let mut bytes=vec![2,1,0];text(&mut bytes,"statement");bytes.extend_from_slice(&[0,12,2]);for index in [1,0]{text(&mut bytes,&texts()[index]);integer(&mut bytes,1);}bytes.push(0);bytes},
   5=>{let mut bytes=vec![2,2,0];text(&mut bytes,"nested");bytes.extend_from_slice(&[0,9,11,8,13,10,1,1,3,1]);text(&mut bytes,"table");bytes.extend_from_slice(&[0,15,1,0]);bytes},
   _=>corpus["cases"][case-1]["canonicalBytes"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as u8).collect()
  };let expected=*blake3::hash(&bytes).as_bytes();
  let mut cursor=super::retained::BorrowedSchemaHashCursor::default();let mut allocation=0;let mut release=0;let mut turns=0;
  loop{
   let capacity=cursor.next_capacity_byte_demand().unwrap();let copy=cursor.next_copy_byte_demand().unwrap();let depth=cursor.next_depth_demand().unwrap();
   let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:capacity,maximum_copy_bytes:copy,maximum_depth:depth,maximum_release_bytes:0};
   for axis in 0..4{if axis==1&&capacity==0||axis==2&&copy==0||axis==3&&depth==0{continue}let mut denied=grant;match axis{0=>denied.maximum_items=0,1=>denied.maximum_capacity_bytes-=1,2=>denied.maximum_copy_bytes-=1,_=>denied.maximum_depth-=1};let(step,requested,freed)=crate::test_allocation::observe_backing(||cursor.advance(spec,denied).unwrap());assert_eq!(step.progress,Default::default());assert_eq!((requested,freed),(0,0));assert_eq!(cursor.next_capacity_byte_demand().unwrap(),capacity);assert_eq!(cursor.next_copy_byte_demand().unwrap(),copy);assert_eq!(cursor.next_depth_demand().unwrap(),depth);}
   let(step,requested,freed)=crate::test_allocation::observe_backing(||cursor.advance(spec,grant).unwrap());assert_eq!(requested,step.progress.retained_capacity_bytes);assert_eq!(freed,0);assert!(capacity<=4096);assert!(step.progress.copied_bytes<=copy);allocation+=requested;turns+=1;if step.complete{break}assert!(turns<2000000);
  }
  assert_eq!(cursor.digest(),Some(expected));assert!(turns>10);
  while !cursor.terminal_is_empty(){let demand=cursor.retirement_demands().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:0,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};for axis in 0..4{if axis==1&&demand.copy_bytes==0||axis==2&&demand.release_bytes==0||axis==3&&demand.depth==0{continue}let mut denied=grant;match axis{0=>denied.maximum_items=0,1=>denied.maximum_copy_bytes-=1,2=>denied.maximum_release_bytes-=1,_=>denied.maximum_depth-=1};let(step,requested,freed)=crate::test_allocation::observe_backing(||cursor.close(denied).unwrap());assert_eq!(step.progress,Default::default());assert_eq!((requested,freed),(0,0));assert_eq!(cursor.retirement_demands().unwrap(),demand);}let(step,requested,freed)=crate::test_allocation::observe_backing(||cursor.close(grant).unwrap());assert_eq!(requested,0);assert_eq!(freed,step.progress.released_bytes);release+=freed;assert!(freed<=4096);}
  assert_eq!(allocation,release);
 }
 for stop in corpus["cancelTurns"].as_array().unwrap(){let mut cursor=super::retained::BorrowedSchemaHashCursor::default();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:8192,maximum_capacity_bytes:4096,maximum_release_bytes:4096,maximum_depth:128};for _ in 0..stop.as_u64().unwrap(){cursor.advance(enum_spec(),grant).unwrap();}while !cursor.terminal_is_empty(){cursor.close(grant).unwrap();}assert!(cursor.digest().is_none());}
 for stage in 1..=10{let mut cursor=super::retained::BorrowedSchemaHashCursor::default();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:8192,maximum_capacity_bytes:4096,maximum_release_bytes:4096,maximum_depth:128};let((),requested,released)=crate::test_allocation::observe_backing(||{let mut turns=0;while cursor.stage_for_test()!=stage{cursor.advance(shared(),grant).unwrap();turns+=1;assert!(turns<10000);}while !cursor.terminal_is_empty(){cursor.close(grant).unwrap();}});assert_eq!(requested,released);}
 assert!(std::mem::size_of::<super::retained::BorrowedSchemaHashCursor>()<=4096);
 println!("[DEBUG] retained schema discovery, cyclic/shared quotient, long immutable metadata, exact four-axis denial and every original paged allocation retire progressively");
}
