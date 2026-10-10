//! 🌳️ Original Drawing typed prefixes survive malformed input and cancellation until physically funded close.
use semio_framework_os_kernel as store;
use semio_framework_value::{DslValue,FromValue,ToValue,ErasedSnapshotRetirement,RetirementDemand,ValueRefusalKind};
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::native_decoding::{NativeDecodeControl,NativeDecodeRetirementRecipient};
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneStep,RetainedCloneProgress};
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode,DrawStyle,DrawCanvas,DrawLayer,SemioDrawingSnapshot};
#[global_allocator]
static DRAW_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
fn fixture()->serde_json::Value{serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()}
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:65536,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:4096}}
fn demand(owner:&dyn ErasedSnapshotRetirement)->RetirementDemand{let copy=owner.next_copy_byte_demand().unwrap();let release=owner.next_release_byte_demand().unwrap();owner.next_demand(if copy==0{release}else{copy}).unwrap()}
fn close(owner:&mut dyn ErasedSnapshotRetirement,denials:&mut Vec<serde_json::Value>)->(usize,usize){
 let policy=grant();let(mut born,mut freed)=(0,0);
 for turn in 0..100000{
  if owner.terminal_is_empty(){return (born,freed)}
  let before=demand(owner);
  for axis in ["items","copy","capacity","release","depth"]{
   let mut denied=policy;let required=match axis{"items"=>{denied.maximum_items=0;1},"copy"=>{denied.maximum_copy_bytes=before.copy_bytes.saturating_sub(1);before.copy_bytes},"capacity"=>{denied.maximum_capacity_bytes=before.capacity_bytes.saturating_sub(1);before.capacity_bytes},"release"=>{denied.maximum_release_bytes=before.release_bytes.saturating_sub(1);before.release_bytes},_=>{denied.maximum_depth=before.depth.saturating_sub(1);before.depth}};
   if required==0{continue}
   let(result,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(denied));
   assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0),"denied {axis} at turn {turn}");if let Ok(result)=result{assert_eq!(result.progress(),RetainedCloneProgress::default())}assert_eq!(demand(owner),before);
   denials.push(serde_json::json!({"axis":axis,"requested":physical.requested_bytes,"released":physical.released_bytes}));
  }
  let(step,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(policy).unwrap());let progress=step.progress();assert!(progress.fits(policy));assert_eq!(physical.requested_bytes,progress.retained_capacity_bytes);assert_eq!(physical.released_bytes,progress.released_bytes);born+=physical.requested_bytes;freed+=physical.released_bytes;
  if matches!(step,RetainedCloneStep::Complete(_)){assert!(owner.terminal_is_empty());return (born,freed)}
 }
 panic!("Drawing original close did not converge")
}
fn close_owned<T:RetireOwned>(value:T,denials:&mut Vec<serde_json::Value>)->(usize,usize){
 let mut pending=Some(value);let mut active=None;let(step,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||store::artifact_retirement_admit_owned(&mut pending,&mut active,grant()).unwrap());assert_eq!(physical.requested_bytes,step.progress().retained_capacity_bytes);assert_eq!(physical.released_bytes,step.progress().released_bytes);let(mut born,mut freed)=(physical.requested_bytes,physical.released_bytes);
 while let Some(owner)=active.as_mut(){let(b,f)=close(owner.as_mut(),denials);born+=b;freed+=f;let(step,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||store::artifact_retirement_box_close_step(&mut active,grant()).unwrap());assert_eq!(physical.requested_bytes,step.progress().retained_capacity_bytes);assert_eq!(physical.released_bytes,step.progress().released_bytes);born+=physical.requested_bytes;freed+=physical.released_bytes;}
 assert!(pending.is_none());(born,freed)
}
#[test]
fn drawing_original_controlled_prefix_matches_neutral_nodes_and_five_denied_currencies(){
 let corpus=fixture();let mut denials=Vec::new();let mut cases=Vec::new();let mut cancelled=0;
 for row in corpus["cases"].as_array().unwrap(){let input=DslValue::from(&row["node"]);let expected=DrawNode::from_value(input.clone()).unwrap();let mut recipient=NativeDecodeRetirementRecipient::new();let mut callback=|_|true;let mut control=NativeDecodeControl::new(1048576,&mut callback);control.install_retirement_recipient(&mut recipient).unwrap();
  let(result,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||DrawNode::from_value_controlled(&input,&mut control));drop(control);assert_eq!(physical.released_bytes,0);let node=result.unwrap();assert_eq!(node,expected);cases.push(serde_json::json!({"name":row["name"],"node":serde_json::Value::from(node.to_value())}));let(b1,f1)=close(&mut recipient,&mut denials);let(b2,f2)=close_owned(node,&mut denials);assert_eq!(physical.requested_bytes+b1+b2,f1+f2);let(_,terminal)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(recipient));assert_eq!((terminal.requested_bytes,terminal.released_bytes),(0,0));
 }
 for row in corpus["malformed"].as_array().unwrap(){let input=DslValue::from(row);let mut recipient=NativeDecodeRetirementRecipient::new();let mut callback=|_|true;let mut control=NativeDecodeControl::new(1048576,&mut callback);control.install_retirement_recipient(&mut recipient).unwrap();let(result,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||DrawNode::from_value_controlled(&input,&mut control));drop(control);assert!(result.is_err());assert_eq!(physical.released_bytes,0);let(b,f)=close(&mut recipient,&mut denials);assert_eq!(physical.requested_bytes+b,f);}
 for row in corpus["cases"].as_array().unwrap(){let input=DslValue::from(&row["node"]);for stop in corpus["cancelAfter"].as_array().unwrap(){let limit=stop.as_u64().unwrap()as usize;let mut calls=0;let mut callback=|_|{let run=calls<limit;calls+=1;run};let mut recipient=NativeDecodeRetirementRecipient::new();let mut control=NativeDecodeControl::new(1048576,&mut callback);control.install_retirement_recipient(&mut recipient).unwrap();let(result,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||DrawNode::from_value_controlled(&input,&mut control));drop(control);assert_eq!(physical.released_bytes,0);let(mut b,mut f)=close(&mut recipient,&mut denials);match result{Ok(node)=>{let(nb,nf)=close_owned(node,&mut denials);b+=nb;f+=nf},Err(error)=>{assert_eq!(error.kind,ValueRefusalKind::Canceled);cancelled+=1}}assert_eq!(physical.requested_bytes+b,f);}}
 for axis in ["items","copy","capacity","release","depth"]{assert!(denials.iter().any(|row|row["axis"]==axis));}
 if let Ok(directory)=std::env::var("SEMIO_TEST_ARTIFACT_DIR"){std::fs::create_dir_all(&directory).unwrap();std::fs::write(std::path::Path::new(&directory).join("original-drawing.json"),serde_json::to_vec(&serde_json::json!({"cases":cases,"malformed":corpus["malformed"].as_array().unwrap().len(),"cancelled":cancelled,"denials":denials,"terminal":{"requested":0,"released":0}})).unwrap()).unwrap();}
 println!("[DEBUG] Drawing four original typed nodes/four malformed prefixes/forty cancellation positions, five denied currencies and exact physical allocation conservation");
}
/// 🧬️ Cold and caller-controlled entry points enforce the same exact schema variants.
#[test]
fn drawing_original_exact_variants_reject_foreign_fields_and_missing_payload(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧬️shape.json")).unwrap();
 for row in fixture["malformed"].as_array().unwrap(){assert!(DrawNode::from_value(DslValue::from(row)).is_err());}
 println!("[DEBUG] Drawing six original exact variant shape refusals match independent portable AJV");
}
/// 🪶️ Relational reconstruction keeps rows, ordering scratch and original typed layers in the caller return scope.
#[test]
fn drawing_original_sqlite_reconstruction_keeps_all_partial_original_storage(){
 use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteSnapshotControl,SqliteDatabaseLimits,SqliteSnapshotPhase}};
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🌱️defaults.json")).unwrap();let snapshot=SemioDrawingSnapshot::from_value(DslValue::from(&corpus["defaults"][2]["input"])).unwrap();
 let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let mut denials=Vec::new();let mut interrupted=0;
 for stop in [0,1,3,7,13,21,34,55,89,144,233,377,usize::MAX]{
  let mut calls=0;let mut callback=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{if event.phase!=SqliteSnapshotPhase::ReconstructSnapshot{return true}let run=calls<stop;calls+=1;run};
  let mut recipient=NativeDecodeRetirementRecipient::new();let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());control.install_native_retirement_recipient(&mut recipient).unwrap();
  let(result,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||SemioDrawingSnapshot::from_sqlite_database(&database,&mut control));drop(control);assert_eq!(physical.released_bytes,0,"unreturned relational prefix at stop {stop}");
  let(mut born,mut freed)=close(&mut recipient,&mut denials);match result{Ok(output)=>{assert_eq!(output,snapshot);let(b,f)=close_owned(output,&mut denials);born+=b;freed+=f},Err(error)=>{assert_eq!(error.kind,ValueRefusalKind::Canceled);interrupted+=1}}
  assert_eq!(physical.requested_bytes+born,freed);let(_,terminal)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(recipient));assert_eq!((terminal.requested_bytes,terminal.released_bytes),(0,0));
 }
 assert!(interrupted>0);println!("[DEBUG] Drawing relational thirteen complete/cancel original row/layer journeys preserve every backing allocation until funded close");
}
/// 🗂️ Whole original document fields and nested layers retain physical custody across cancellation.
#[test]
fn drawing_original_document_controlled_prefix_preserves_layers_and_partial_fields(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🌱️defaults.json")).unwrap();let mut denials=Vec::new();let mut interrupted=0;
 for row in corpus["defaults"].as_array().unwrap().iter().filter(|row|row["input"].get("kind").is_none()){
  let input=DslValue::from(&row["input"]);let expected=SemioDrawingSnapshot::from_value(input.clone()).unwrap();
  for stop in [0,1,3,7,13,21,34,55,usize::MAX]{
   let mut calls=0;let mut callback=|_|{let run=calls<stop;calls+=1;run};let mut recipient=NativeDecodeRetirementRecipient::new();let mut control=NativeDecodeControl::new(1048576,&mut callback);control.install_retirement_recipient(&mut recipient).unwrap();
   let(result,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||SemioDrawingSnapshot::from_value_controlled(&input,&mut control));drop(control);assert_eq!(physical.released_bytes,0);
   let(mut born,mut freed)=close(&mut recipient,&mut denials);match result{Ok(output)=>{assert_eq!(output,expected);let(b,f)=close_owned(output,&mut denials);born+=b;freed+=f},Err(error)=>{assert_eq!(error.kind,ValueRefusalKind::Canceled);interrupted+=1}}
   assert_eq!(physical.requested_bytes+born,freed);let(_,terminal)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(recipient));assert_eq!((terminal.requested_bytes,terminal.released_bytes),(0,0));
  }
 }
 let mut long=corpus["defaults"][2]["input"].clone();let text=corpus["longText"]["text"].as_str().unwrap().repeat(corpus["longText"]["repeat"].as_u64().unwrap()as usize);let total=text.len();long["layers"][1]["root"]["children"][1]["value"]=serde_json::Value::String(text);let input=DslValue::from(&long);
 let mut interior=false;let mut callback=|event:semio_framework_value::NativeDecodeProgress|{let cancel=event.total==total&&event.completed>=corpus["longText"]["minimumCheckpointBytes"].as_u64().unwrap()as usize&&event.completed<event.total;interior|=cancel;!cancel};
 let mut recipient=NativeDecodeRetirementRecipient::new();let mut control=NativeDecodeControl::new(1048576,&mut callback);control.install_retirement_recipient(&mut recipient).unwrap();let(result,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||SemioDrawingSnapshot::from_value_controlled(&input,&mut control));drop(control);assert!(interior);assert_eq!(result.unwrap_err().kind,ValueRefusalKind::Canceled);assert_eq!(physical.released_bytes,0);let(born,freed)=close(&mut recipient,&mut denials);assert_eq!(physical.requested_bytes+born,freed);let(_,terminal)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(recipient));assert_eq!((terminal.requested_bytes,terminal.released_bytes),(0,0));
 assert!(interrupted>0);println!("[DEBUG] Drawing original document defaults/authored layers/eighteen completion-cancel journeys/one 100000-byte UTF8 interior cancellation preserve physical owned fields and terminal zero");
}
/// 🌱️ Native defaults and exact record fields share the independent portable schema corpus.
#[test]
fn drawing_original_container_defaults_and_foreign_fields_match_shared_schema(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🌱️defaults.json")).unwrap();
 for row in corpus["defaults"].as_array().unwrap(){
  let input=DslValue::from(&row["input"]);
  let output=if row["input"].get("kind").is_some(){DrawNode::from_value(input).unwrap().to_value()}else{SemioDrawingSnapshot::from_value(input).unwrap().to_value()};
  assert_eq!(serde_json::Value::from(output),row["expected"]);
 }
 for row in corpus["malformed"].as_array().unwrap(){
  let input=DslValue::from(&row["input"]);let refused=match row["kind"].as_str().unwrap(){
   "style"=>DrawStyle::from_value(input).is_err(),"canvas"=>DrawCanvas::from_value(input).is_err(),"layer"=>DrawLayer::from_value(input).is_err(),"snapshot"=>SemioDrawingSnapshot::from_value(input).is_err(),_=>panic!("unknown shared Drawing container"),
  };assert!(refused,"native Drawing {} accepted foreign fields",row["kind"]);
 }
 println!("[DEBUG] Drawing native two canonical defaults/authored layers/four exact container refusals match shared independent AJV corpus");
}
#[test]
fn drawing_original_controlled_prefix_refuses_absent_recipient_without_heap(){
 let corpus=fixture();let input=DslValue::from(&corpus["cases"][0]["node"]);let mut callback=|_|true;let mut control=NativeDecodeControl::new(1048576,&mut callback);let(result,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||DrawNode::from_value_controlled(&input,&mut control));assert_eq!(result.unwrap_err().kind,ValueRefusalKind::OwnershipLimit);assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));assert_eq!(control.owned_bytes(),0);println!("[DEBUG] Drawing absent explicit original recipient refused before any heap ownership");
}
