//! 🧪️ Persistent table scans and DSL projections preserve actual original row owners independently of SQLite output.
use semio_framework_test_fixtures::fixture_text;
use super::super::{PersistentList,PERSISTENT_PAGE_FANOUT};
use std::mem::size_of;
use crate::{ToValue,FromValue,ValueShape,retained_clone::{RetainedCloneGrant,RetainedCloneSource},retirement::RetireOwned,shared_issuance::{SealedShared,SharedIssuer}};
#[derive(crate::RetireOwned,crate::ToValue)]#[value(crate="crate")]
struct Row{rank:i64,label:String}
impl Clone for Row{fn clone(&self)->Self{panic!("persistent table traversal cloned an original row")}}
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:4096}}
fn make(case:&serde_json::Value,law:&serde_json::Value)->PersistentList<Row>{
 let mut rank=0;let mut pages=Vec::new();
 for len in std::iter::repeat_n(PERSISTENT_PAGE_FANOUT,case["fullLeaves"].as_u64().unwrap()as usize).chain(case["tailLeaves"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as usize)){
  let rows=std::array::from_fn(|slot|if slot<len{let row=Row{rank:rank*law["stride"].as_i64().unwrap(),label:format!("{}{rank}",law["prefix"].as_str().unwrap())};rank+=1;Some(SealedShared::admit(row,SharedIssuer::owned(),grant()).unwrap_or_else(|_|panic!("table row issuance")).0)}else{None});
  pages.push(PersistentList::admit_leaf(rows,len,grant()).unwrap_or_else(|_|panic!("table leaf issuance")).0);
 }
 while pages.len()>1{let mut originals=pages.into_iter();let mut parents=Vec::new();loop{let children=std::array::from_fn(|_|originals.next());if children[0].is_none(){break;}parents.push(PersistentList::admit_branch(children,grant()).unwrap_or_else(|_|panic!("table branch issuance")).0);}pages=parents;}
 pages.pop().unwrap_or_default()
}
fn close<T:RetireOwned+Send+Sync+'static>(source:&mut RetainedCloneSource<T>){
 let mut idle=0;for _ in 0..1000000{
  if source.terminal_is_empty(){return;}let mut d=source.next_close_demand(4096).unwrap();let copy=4096.max(d.copy_bytes);if copy!=4096{d=source.next_close_demand(copy).unwrap();}
  let g=RetainedCloneGrant{maximum_copy_bytes:copy,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth,..grant()};let p=source.close_step(g).unwrap().progress();assert!(p.fits(g));if p==Default::default(){idle+=1;assert!(idle<8,"table original source repeats an inert quote: {g:?}");}else{idle=0;}
 }
 panic!("table original source did not close");
}
fn law()->serde_json::Value{serde_json::from_str(fixture_text!("../../🔎️traversal/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()}
fn oracle(law:&serde_json::Value)->serde_json::Value{
 let schema:serde_json::Value=serde_json::from_str(fixture_text!("../../🔎️traversal/🧬️schema/🔣️.json")).unwrap();
 let mut child=std::process::Command::new("bun").args(["-e",r#"import Ajv from "ajv/dist/2020";import{Database}from"bun:sqlite";const{law,schema}=JSON.parse(await Bun.stdin.text());const v=new Ajv({strict:true}).compile(schema);if(!v(law)||v({...law,stride:0}))throw Error(JSON.stringify(v.errors));const db=new Database(":memory:");db.exec("create table rows(position integer primary key,rank integer,label text)");const output=[];for(const c of law.cases){db.exec("delete from rows");const count=c.fullLeaves*16+c.tailLeaves.reduce((a,b)=>a+b,0);if(count!==c.rowCount)throw Error("neutral cardinality");db.transaction(()=>{for(let i=0;i<count;i++)db.query("insert into rows values(?,?,?)").run(i,i*law.stride,law.prefix+i)})();output.push(db.query("select rank,label from rows order by position").all());}console.log(JSON.stringify(output));db.close();"#]).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn().unwrap();
 use std::io::Write;child.stdin.take().unwrap().write_all(serde_json::json!({"law":law,"schema":schema}).to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success());serde_json::from_slice(&output.stdout).unwrap()
}
#[test]
fn persistent_table_traversal_preserves_original_rows_and_linear_zero_heap_scan(){
 let law=law();let independent=oracle(&law);
 for(case,rows)in law["cases"].as_array().unwrap().iter().zip(independent.as_array().unwrap()){
  let list=make(case,&law);let mut source=RetainedCloneSource::admit_owned(list,(),grant()).unwrap_or_else(|_|panic!("table source")).0;let original=source.borrow();let list=original.get();let count=list.len();assert_eq!(count,case["rowCount"].as_u64().unwrap()as usize);
  assert!(size_of::<super::PersistentListIter<'_,Row>>()<=law["expected"]["maximumIteratorBytes"].as_u64().unwrap()as usize);
  let((seen,visits),heap)=crate::value::observe_retirement_allocations(||{let mut cursor=list.iter();assert_eq!(cursor.size_hint(),(count,Some(count)));let mut seen=0;while let Some(row)=cursor.next(){assert!(std::ptr::eq(row,list.get(seen).unwrap()));assert_eq!(row.rank,rows[seen]["rank"].as_i64().unwrap());assert_eq!(row.label,rows[seen]["label"].as_str().unwrap());seen+=1;assert_eq!(cursor.len(),count-seen);}assert_eq!(cursor.size_hint(),(0,Some(0)));assert!(cursor.next().is_none());assert!(cursor.nth(usize::MAX).is_none());assert!(cursor.next().is_none());(seen,cursor.visited_pages)});
  assert_eq!(heap,(0,0));assert_eq!(seen,count);assert_eq!(visits,case["pageCount"].as_u64().unwrap()as usize);
  let(_,heap)=crate::value::observe_retirement_allocations(||{let mut cursor=list.iter();let mut position=0usize;for skip in law["queries"].as_array().unwrap(){let skip=skip.as_u64().unwrap()as usize;let expected=position.checked_add(skip).filter(|n|*n<count);let actual=cursor.nth(skip);assert_eq!(actual.map(|row|row.rank),expected.map(|index|rows[index]["rank"].as_i64().unwrap()));position=expected.map_or(count,|index|index+1);assert_eq!(cursor.len(),count-position);}assert!(cursor.next().is_none());});
  assert_eq!(heap,(0,0));
  if count>0{let mut cursor=list.iter();let(_,heap)=crate::value::observe_retirement_allocations(||{let row=cursor.nth(count-1).unwrap();assert!(std::ptr::eq(row,list.get(count-1).unwrap()));});assert_eq!(heap,(0,0));assert!(cursor.visited_pages<=usize::from(list.height)+1);assert_eq!(cursor.len(),0);}
  close(&mut source);assert_eq!(crate::value::observe_retirement_allocations(||drop(source)).1,(0,0));
 }
}
#[test]
fn persistent_table_traversal_projects_borrowed_records_and_controlled_wire_without_payload_clone(){
 let law=law();let independent=oracle(&law);
 let sealed=SealedShared::admit(Row{rank:law["stride"].as_i64().unwrap(),label:format!("{}1",law["prefix"].as_str().unwrap())},SharedIssuer::owned(),grant()).unwrap_or_else(|_|panic!("sealed record issuance")).0;
 let mut sealed_source=RetainedCloneSource::admit_owned(sealed,(),grant()).unwrap_or_else(|_|panic!("sealed record source")).0;
 let mut accepted=|_|true;let mut control=semio_framework_value_native_encoding::native_encoding::NativeEncodeControl::new(65536,&mut accepted);let borrowed=sealed_source.borrow();let row=borrowed.get();
 let(shape,heap)=crate::value::observe_retirement_allocations(||row.value_shape_at_path(&["label"]).unwrap());assert_eq!(shape,ValueShape::String);assert_eq!(heap,(0,0));
 let(output,heap)=crate::value::observe_retirement_allocations(||row.to_value_controlled(&mut control).unwrap());assert_eq!(heap,(control.owned_bytes(),0));assert_eq!(serde_json::Value::from(&output),independent[3][1]);<crate::DslValue as FromValue>::retire_decoded(output);close(&mut sealed_source);assert_eq!(crate::value::observe_retirement_allocations(||drop(sealed_source)).1,(0,0));
 for(case,rows)in law["cases"].as_array().unwrap().iter().zip(independent.as_array().unwrap()){
  let mut source=RetainedCloneSource::admit_owned(make(case,&law),(),grant()).unwrap_or_else(|_|panic!("wire table source")).0;
  let list=source.borrow();let list=list.get();
  let(shape,heap)=crate::value::observe_retirement_allocations(||list.value_shape_at_path(&[]).unwrap());assert_eq!(shape,ValueShape::Array{len:list.len()});assert_eq!(heap,(0,0));
  if !list.is_empty(){let(shape,heap)=crate::value::observe_retirement_allocations(||list.value_shape_at_path(&["0","label"]).unwrap());assert_eq!(shape,ValueShape::String);assert_eq!(heap,(0,0));assert_eq!(list.value_key_at_path(&["0"],1).unwrap(),"label");for bad in["","00","-","-1","184467440737095516160"]{assert!(list.value_shape_at_path(&[bad]).is_err());}}
  let mut callback=|_|true;let mut control=semio_framework_value_native_encoding::native_encoding::NativeEncodeControl::new(1<<24,&mut callback);let(output,heap)=crate::value::observe_retirement_allocations(||list.to_value_controlled(&mut control).unwrap());assert_eq!(heap.0,control.owned_bytes());assert_eq!(heap.1,0);assert_eq!(serde_json::Value::from(&output),*rows);<crate::DslValue as FromValue>::retire_decoded(output);
  let mut canceled=|_|false;let mut control=semio_framework_value_native_encoding::native_encoding::NativeEncodeControl::new(1<<24,&mut canceled);let(result,heap)=crate::value::observe_retirement_allocations(||list.to_value_controlled(&mut control));assert!(result.is_err());assert_eq!(heap,(0,0));
  if !list.is_empty(){let mut accepted=|_|true;let mut control=semio_framework_value_native_encoding::native_encoding::NativeEncodeControl::new(0,&mut accepted);let(result,heap)=crate::value::observe_retirement_allocations(||list.to_value_controlled(&mut control));assert!(result.is_err());assert_eq!(heap,(0,0));}
  close(&mut source);assert_eq!(crate::value::observe_retirement_allocations(||drop(source)).1,(0,0));
 }
}