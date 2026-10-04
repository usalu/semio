//! 🧹️ Public HTML cold retirement and late SQL refusal share literal neutral ownership.
use super::*;
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabaseLimits,SqliteSnapshotControl,SqliteSnapshotPhase}};
std::thread_local!{
 static TRACK:std::cell::Cell<bool>=const{std::cell::Cell::new(false)};
 static REFUSE:std::cell::Cell<bool>=const{std::cell::Cell::new(false)};
 static ATTEMPTS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};
 static LIVE:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};
 static VISITS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};
 static SQL_LATE:std::cell::Cell<bool>=const{std::cell::Cell::new(false)};
}
pub(super) fn retirement_visit(){VISITS.with(|count|count.set(count.get()+1));}
pub(super) struct SqlLateCopyScope(bool);
impl SqlLateCopyScope{pub(super) fn enter()->Self{Self(SQL_LATE.with(|active|active.replace(true)))}}
impl Drop for SqlLateCopyScope{fn drop(&mut self){SQL_LATE.with(|active|active.set(self.0));}}
struct ObservedAllocator;
fn refuse()->bool{if REFUSE.try_with(std::cell::Cell::get).unwrap_or(false){let _=ATTEMPTS.try_with(|count|count.set(count.get()+1));true}else{false}}
fn allocated(bytes:usize){if TRACK.try_with(std::cell::Cell::get).unwrap_or(false){let _=LIVE.try_with(|count|count.set(count.get()+bytes));}}
fn released(bytes:usize){if TRACK.try_with(std::cell::Cell::get).unwrap_or(false){let _=LIVE.try_with(|count|count.set(count.get()-bytes));}}
unsafe impl std::alloc::GlobalAlloc for ObservedAllocator{
 unsafe fn alloc(&self,layout:std::alloc::Layout)->*mut u8{if refuse(){return std::ptr::null_mut()}let result=unsafe{std::alloc::GlobalAlloc::alloc(&std::alloc::System,layout)};if !result.is_null(){allocated(layout.size())}result}
 unsafe fn alloc_zeroed(&self,layout:std::alloc::Layout)->*mut u8{if refuse(){return std::ptr::null_mut()}let result=unsafe{std::alloc::GlobalAlloc::alloc_zeroed(&std::alloc::System,layout)};if !result.is_null(){allocated(layout.size())}result}
 unsafe fn realloc(&self,pointer:*mut u8,layout:std::alloc::Layout,size:usize)->*mut u8{if refuse(){return std::ptr::null_mut()}let result=unsafe{std::alloc::GlobalAlloc::realloc(&std::alloc::System,pointer,layout,size)};if !result.is_null(){released(layout.size());allocated(size)}result}
 unsafe fn dealloc(&self,pointer:*mut u8,layout:std::alloc::Layout){released(layout.size());unsafe{std::alloc::GlobalAlloc::dealloc(&std::alloc::System,pointer,layout)}}
}
#[global_allocator]
static OBSERVED_ALLOCATOR:ObservedAllocator=ObservedAllocator;
fn corpus()->serde_json::Value{serde_json::from_str(include_str!("../../../🧫️fixtures/🪶️sqlite/🧹️lifecycle/🔣️.json")).unwrap()}
fn leaf(index:usize,cases:&serde_json::Value)->HtmlNode{let value=&cases["leaves"][index%4];let text=value["text"].as_str().unwrap().to_owned();match value["kind"].as_str().unwrap(){"text"=>HtmlNode::Text{text},"comment"=>HtmlNode::Comment{text},"rawText"=>HtmlNode::RawText{parent_kind:match value["parentKind"].as_str().unwrap(){"script"=>RawTextKind::Script,"style"=>RawTextKind::Style,_=>panic!("literal raw kind")},text},_=>panic!("literal leaf kind")}}
fn attributes(cases:&serde_json::Value)->Vec<HtmlAttr>{cases["attributes"].as_array().unwrap().iter().map(|value|HtmlAttr{name:value["name"].as_str().unwrap().to_owned(),value:value.get("value").map(|value|value.as_str().unwrap().to_owned())}).collect()}
fn tree(case:usize,cases:&serde_json::Value)->(HtmlSnapshot,usize){
 let depth=cases["retirementDepth"].as_u64().unwrap()as usize;let mut root=leaf(0,cases);let nodes=match case{1=>{let width=cases["wide"].as_u64().unwrap()as usize;root=HtmlNode::Element{name:cases["elementName"].as_str().unwrap().to_owned(),attributes:attributes(cases),children:(0..width).map(|index|leaf(index,cases)).collect()};width+1},_=>{for index in 0..depth{let mut children=vec![root];if case==2{children.push(leaf(index,cases))}root=HtmlNode::Element{name:cases["elementName"].as_str().unwrap().to_owned(),attributes:attributes(cases),children};}depth*(1+usize::from(case==2))+1}};
 (HtmlSnapshot{schema:cases["schema"].as_str().unwrap().to_owned(),doctype:Some(cases["doctype"].as_str().unwrap().to_owned()),root},nodes)
}
fn isolated(flag:&str,name:&str)->bool{if std::env::var_os(flag).is_some(){return false}let module=module_path!().split_once("::").expect("actual crate/module path").1;let selector=format!("{module}::{name}");let output=std::process::Command::new(std::env::current_exe().unwrap()).args([selector.as_str(),"--exact","--test-threads=1","--nocapture"]).env(flag,"1").output().unwrap();let stdout=String::from_utf8_lossy(&output.stdout);let stderr=String::from_utf8_lossy(&output.stderr);assert!(output.status.success()&&stdout.contains("test result: ok. 1 passed"),"isolated owning law must execute exactly once and pass: {stdout}{stderr}");true}
#[test]
fn sqlite_snapshot_html_public_retirement_releases_deep_wide_fields_when_all_allocations_refuse(){
 if isolated("SEMIO_HTML_ALLOCATION_FREE_RETIRE_CHILD","sqlite_snapshot_html_public_retirement_releases_deep_wide_fields_when_all_allocations_refuse"){return}
 let cases=corpus();for case in 0..3{let cases=cases.clone();let stack=cases["stackBytes"].as_u64().unwrap()as usize;let(owned,remaining,attempts,visits,nodes)=std::thread::Builder::new().stack_size(stack).spawn(move||{
  TRACK.with(|active|active.set(true));let(snapshot,nodes)=tree(case,&cases);let owned=LIVE.with(std::cell::Cell::get);ATTEMPTS.with(|count|count.set(0));VISITS.with(|count|count.set(0));REFUSE.with(|active|active.set(true));snapshot.retire_sqlite_snapshot();REFUSE.with(|active|active.set(false));let remaining=LIVE.with(std::cell::Cell::get);TRACK.with(|active|active.set(false));(owned,remaining,ATTEMPTS.with(std::cell::Cell::get),VISITS.with(std::cell::Cell::get),nodes)
 }).unwrap().join().unwrap();assert!(owned>0);assert_eq!(remaining,0,"all owned fields must release in case {case}");assert_eq!(attempts,0,"cold retirement must request no allocation in case {case}");assert!(visits>=nodes&&visits<=3*nodes,"case {case}: {visits} visits for {nodes} nodes must stay linear");}
}
#[test]
fn sqlite_snapshot_html_late_sql_metadata_cancellation_retires_complete_deep_tree(){
 if isolated("SEMIO_HTML_LATE_SQL_CANCEL_CHILD","sqlite_snapshot_html_late_sql_metadata_cancellation_retires_complete_deep_tree"){return}
 let cases=corpus();for field in ["schema","doctype"]{let cases=cases.clone();let stack=cases["stackBytes"].as_u64().unwrap()as usize;std::thread::Builder::new().stack_size(stack).spawn(move||{
  let depth=cases["lateSqlDepth"].as_u64().unwrap()as usize;let boundary=cases["cancelUtf8ByteBoundary"].as_u64().unwrap()as usize;let long=cases["lateTextUnit"].as_str().unwrap().repeat(cases["lateTextRepeat"].as_u64().unwrap()as usize);let bytes=long.len();assert!(boundary<bytes&&long.is_char_boundary(boundary));let mut root=leaf(0,&cases);for _ in 0..depth{root=HtmlNode::Element{name:"literal ancestor".into(),attributes:Vec::new(),children:vec![root]};}let snapshot=HtmlSnapshot{schema:if field=="schema"{long.clone()}else{"literal schema".into()},doctype:if field=="doctype"{Some(long)}else{None},root};let limits=SqliteDatabaseLimits::default();let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();VISITS.with(|count|count.set(0));let mut reached=false;let result=HtmlSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |event|{if SQL_LATE.with(std::cell::Cell::get)&&event.phase==SqliteSnapshotPhase::ReconstructSnapshot&&event.total==bytes&&event.completed==boundary{reached=true;false}else{true}},limits));let visits=VISITS.with(std::cell::Cell::get);let refused=result.is_err();if let Ok(candidate)=result{candidate.retire_sqlite_snapshot();}snapshot.retire_sqlite_snapshot();assert!(reached,"only actual late {field} copy after full SQL tree construction may refuse");assert!(refused,"late metadata cancellation must refuse publication");assert!(visits>=depth+1,"completed SQL tree must use cold retirement: {visits} visits for at least {}",depth+1);
 }).unwrap().join().unwrap();}
}
