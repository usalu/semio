use super::*;
use serde_json::{json,Value};
fn limits(v:&Value)->DocumentSceneLimits{DocumentSceneLimits{max_nodes:v["maxNodes"].as_u64().unwrap()as usize,max_depth:v["maxDepth"].as_u64().unwrap()as usize,max_segments:v["maxSegments"].as_u64().unwrap()as usize,max_references:v["maxReferences"].as_u64().unwrap()as usize,max_source_bytes:v["maxSourceBytes"].as_u64().unwrap()as usize}}
fn resolution_limits()->DocumentAlgorithmLimits{DocumentAlgorithmLimits{max_work:10000000,trace:DocumentTraceLimits{max_pixels:100000,max_admitted_pixels:100000,max_source_bytes:100000,max_edges:65536,max_segments:65536,max_retained_segments:65536,max_work:10000000},booleans:DocumentBooleanLimits{tolerance:0.005,epsilon:1e-8,max_depth:32,max_references:256,max_edges:65536,max_parameters:262144,max_atomic_edges:65536,max_segments:65536,max_retained_segments:262144,max_work:10000000}}}
fn content(v:&DocumentSceneContent)->Value{match v{
 DocumentSceneContent::Path{segments,fill_rule,fill,stroke}=>json!({"kind":"path","segments":segments,"fillRule":fill_rule,"fill":fill,"stroke":stroke}),
 DocumentSceneContent::Image{asset,width,height}=>json!({"kind":"image","asset":asset,"width":width,"height":height}),
 DocumentSceneContent::Group{children,isolation}=>json!({"kind":"group","children":children,"isolation":isolation}),
 DocumentSceneContent::Text{content,x,y,size,fill_rule,fill,stroke}=>json!({"kind":"text","content":content,"x":x,"y":y,"size":size,"fillRule":fill_rule,"fill":fill,"stroke":stroke}),
 DocumentSceneContent::Boolean{operation,children,reference_transform,fill_rule,fill,stroke}=>json!({"kind":"boolean","operation":operation,"children":children,"referenceTransform":reference_transform,"fillRule":fill_rule,"fill":fill,"stroke":stroke}),
 DocumentSceneContent::Trace{source,threshold,simplify_epsilon,fill_rule,fill,stroke}=>json!({"kind":"trace","source":source,"threshold":threshold,"simplifyEpsilon":simplify_epsilon,"fillRule":fill_rule,"fill":fill,"stroke":stroke}),
}}
fn output(v:&DocumentScenePlan)->Value{json!({"assets":v.assets.iter().map(|a|json!({"id":a.id,"image":{"width":a.image.width,"height":a.image.height,"pixels":a.image.pixels}})).collect::<Vec<_>>(),"nodes":v.nodes.iter().map(|n|json!({"sourcePath":n.source_path.iter().map(|index|f64::from(*index)).collect::<Vec<_>>(),"lockedAncestors":f64::from(n.locked_ancestors),"id":n.id,"groups":n.groups.iter().map(|g|json!({"id":g.id,"opacity":g.opacity,"blendMode":g.blend_mode})).collect::<Vec<_>>(),"transform":n.transform,"opacity":n.opacity,"blendMode":n.blend_mode,"visible":n.visible,"content":content(&n.content)})).collect::<Vec<_>>()})}
fn finish(job:&mut DocumentSceneJob<'_>,grant:usize)->DocumentScenePlan{let mut work=0;for _ in 0..100000{let p=job.advance(grant).unwrap();assert!(p.work-work<=grant as u64);work=p.work;if p.done{return job.result().unwrap();}}panic!("Preparation did not terminate")}
#[test]
fn painted_scenes_document_preparation_preserves_every_neutral_layer(){
 let cases:Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
 for row in cases.as_array().unwrap(){let doc:DrawingSnapshot=serde_json::from_value(row["document"].clone()).unwrap();for grant in [1,7,4096]{let mut job=DocumentSceneJob::new(&doc,limits(&row["limits"])).unwrap();assert!(matches!(job.result(),Err(DocumentSceneError::Incomplete)));let plan=finish(&mut job,grant);assert_eq!(output(&plan),neutral_value(&row["expected"]),"{} grant {grant}",row["name"]);}}
 eprintln!("[DEBUG] {} native document plans preserved all layer work under grants 1, 7 and 4096",cases.as_array().unwrap().len());
}
#[test]
fn painted_scenes_document_preparation_refuses_invalid_shared_cases(){
 let cases:Value=serde_json::from_str(include_str!("../../🧫️fixtures/⚠️invalid/🔣️.json")).unwrap();
 for row in cases.as_array().unwrap(){let doc=match serde_json::from_value::<DrawingSnapshot>(row["document"].clone()){Ok(doc)=>doc,Err(_)=>{assert!(semio_framework_pack_json::from_json_str::<DrawingSnapshot>(&row["document"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).is_err(),"{}",row["name"]);continue;}};if let Ok(mut job)=DocumentSceneJob::new(&doc,limits(&row["limits"])){let mut failed=false;for _ in 0..100000{match job.advance(1){Err(_)=>{failed=true;break},Ok(p)=>if p.done{break}}}assert!(failed,"{}",row["name"]);assert!(job.result().is_err());assert!(job.advance(1).is_err());}}
 eprintln!("[DEBUG] {} native malformed document plans refused before publication",cases.as_array().unwrap().len());
}
#[test]
fn mounted_vector_source_ancestry_preserves_ordinary_groups_and_resolved_algorithms(){
 let samples:Vec<Value>=serde_json::from_str(include_str!("../../🧫️fixtures/🧭️ancestry/🔣️.json")).unwrap();let rows:Vec<Value>=serde_json::from_str(include_str!("../../🧫️fixtures/🎬️vector/🔣️.json")).unwrap();
 fn locks(layers:&mut semio_framework_value::list::PagedList<DrawingLayerNode,{usize::MAX}>,ids:&[Value]){for layer in layers{let base=crate::schema::layer_base_mut(layer);base.locked=ids.iter().any(|id|id.as_str().is_some_and(|id|base.id.eq_str(id)));if let DrawingLayerNode::Group(group)=layer{locks(&mut group.children,ids);}}}
 for sample in &samples{let row=rows.iter().find(|row|row["name"]==sample["source"]).unwrap();let mut document:DrawingSnapshot=serde_json::from_value(row["document"].clone()).unwrap();locks(&mut document.layers,sample["locks"].as_array().unwrap());for grant in [1,7,4096]{let before=document.clone();let mut job=DocumentVectorJob::new(&document,limits(&row["limits"]),resolution_limits()).unwrap();while !job.advance(grant).unwrap().done{}let plan=job.result().unwrap();let actual:Vec<Value>=plan.nodes.iter().map(|node|json!({"id":node.id,"sourcePath":node.source_path,"lockedAncestors":node.locked_ancestors})).collect();assert_eq!(actual,sample["expected"].as_array().unwrap().clone(),"{} grant {grant}",sample["name"]);assert_eq!(document,before);eprintln!("[DEBUG] Mounted vector ancestry {} grant {grant}: exact authored addresses and unsigned ancestor locks survive real resolution",sample["name"]);}}
}
#[test]
fn mounted_vector_source_addresses_admit_unsigned_deep_locks_and_refuse_invalid_ancestry(){
 let rows:Vec<Value>=serde_json::from_str(include_str!("../../🧫️fixtures/🧭️ancestry/🛂️addresses/🔣️.json")).unwrap();
 for row in rows{let path=serde_json::from_value::<Vec<u16>>(row["sourcePath"].clone());let locks=serde_json::from_value::<u32>(row["lockedAncestors"].clone());let admitted=match(path,locks){(Ok(path),Ok(locks))=>validate_scene_source_address(&path,locks).is_ok(),_=>false};assert_eq!(admitted,row["valid"].as_bool().unwrap(),"{}",row["name"]);}
 eprintln!("[DEBUG] Mounted vector addresses preserve unsigned bit 31, refuse every malformed address, and own at most 64 u16 payload bytes per source path; node header size {} bytes excludes dynamic payloads",std::mem::size_of::<DocumentSceneNode>());
}
#[test]
fn painted_scenes_document_preparation_cancels_private_phases(){
 let cases:Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
 for index in [1,7,11,13]{let row=&cases[index];let doc:DrawingSnapshot=serde_json::from_value(row["document"].clone()).unwrap();for steps in [0,1,5,15]{let mut job=DocumentSceneJob::new(&doc,limits(&row["limits"])).unwrap();for _ in 0..steps{job.advance(1).unwrap();}job.cancel();assert!(matches!(job.result(),Err(DocumentSceneError::Cancelled)));assert!(matches!(job.advance(1),Err(DocumentSceneError::Cancelled)));}}
}
#[test]
fn painted_scenes_document_preparation_hands_real_paths_to_raster_and_refuses_unresolved(){
 let cases:Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
 let viewport=||DocumentSceneViewport{width:2,height:1,origin:[0.0,0.0],tolerance:0.01,max_pixels:100,max_source_bytes:10000};
 let doc:DrawingSnapshot=serde_json::from_value(cases[0]["document"].clone()).unwrap();let mut preparation=DocumentSceneJob::new(&doc,limits(&cases[0]["limits"])).unwrap();let input=resolved_scene_input(finish(&mut preparation,7),viewport()).unwrap();let mut raster=crate::schema::scene_raster::RasterSceneJob::new(input).unwrap();while !raster.advance(4096).unwrap().done{}assert_eq!(raster.result().unwrap().pixels,vec![255,0,0,255,255,0,0,255]);
 for index in [8,9,10]{let row=&cases[index];let doc:DrawingSnapshot=serde_json::from_value(row["document"].clone()).unwrap();let mut job=DocumentSceneJob::new(&doc,limits(&row["limits"])).unwrap();assert!(resolved_scene_input(finish(&mut job,1),viewport()).is_err());}
 eprintln!("[DEBUG] Native actual document geometry reached scene raster; unresolved text/boolean/trace was explicit");
}
#[test]
fn painted_scenes_document_raster_renders_shared_assets_privately_and_cancels_every_phase(){
 let cases:Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
 let viewport=||DocumentSceneViewport{width:2,height:1,origin:[0.0,0.0],tolerance:0.01,max_pixels:100,max_source_bytes:10000};
 for index in [0,7]{let row=&cases[index];let doc:DrawingSnapshot=serde_json::from_value(row["document"].clone()).unwrap();for grant in [1,7,4096]{let mut job=DocumentRasterJob::new(&doc,limits(&row["limits"]),viewport(),resolution_limits()).unwrap();let mut work=0;let mut done=false;let mut phases=BTreeSet::new();for _ in 0..100000{let p=job.advance(grant).unwrap();assert!(p.work-work<=grant as u64);work=p.work;phases.insert(p.phase);if p.done{done=true;break;}assert!(matches!(job.result(),Err(DocumentSceneError::Incomplete)));}assert!(done);assert_eq!(job.result().unwrap().pixels,vec![255,0,0,255,255,0,0,255]);if grant==1{assert_eq!(phases,BTreeSet::from(["preparing","tracing","algorithms","resolving","raster","complete"]));}}}
 let row=&cases[7];let doc:DrawingSnapshot=serde_json::from_value(row["document"].clone()).unwrap();for phase in ["preparing","tracing","algorithms","resolving","raster","complete"]{let mut job=DocumentRasterJob::new(&doc,limits(&row["limits"]),viewport(),resolution_limits()).unwrap();let mut seen=false;for _ in 0..100000{if job.advance(1).unwrap().phase==phase{seen=true;break;}}assert!(seen);let pixels=if phase=="complete"{Some(job.result().unwrap().pixels)}else{None};job.cancel();assert!(matches!(job.result(),Err(DocumentSceneError::Cancelled)));assert!(matches!(job.advance(1),Err(DocumentSceneError::Cancelled)));if let Some(pixels)=pixels{assert_eq!(pixels,vec![255,0,0,255,255,0,0,255]);}}
 for index in [8]{let row=&cases[index];let doc:DrawingSnapshot=serde_json::from_value(row["document"].clone()).unwrap();let mut job=DocumentRasterJob::new(&doc,limits(&row["limits"]),viewport(),resolution_limits()).unwrap();assert!(job.advance(100000).is_err());assert!(job.advance(1).is_err());assert!(job.result().is_err());}
 eprintln!("[DEBUG] Actual native image documents render through every grant; cancellation preserves published pixels");
}
#[test]
fn painted_scenes_vector_plans_preserve_neutral_text_image_and_group_records(){
 let rows:Vec<Value>=serde_json::from_str(include_str!("../../🧫️fixtures/🎬️vector/🔣️.json")).unwrap();
 for row in &rows{let document:DrawingSnapshot=serde_json::from_value(row["document"].clone()).unwrap();let mut previous=None;for grant in [1,7,4096]{let mut job=DocumentVectorJob::new(&document,limits(&row["limits"]),resolution_limits()).unwrap();assert!(matches!(job.result(),Err(DocumentSceneError::Incomplete)));let mut work=0;loop{let p=job.advance(grant).unwrap();assert!(p.work-work<=grant as u64);work=p.work;if p.done{break;}assert!(matches!(job.result(),Err(DocumentSceneError::Incomplete)));}let plan=job.result().unwrap();assert!(plan.nodes.iter().all(|n|!matches!(n.content,DocumentSceneContent::Boolean{..}|DocumentSceneContent::Trace{..})));if let Some(expected)=row.get("expectedGeometry"){let node=plan.nodes.iter().find(|n|n.id==expected["id"].as_str().unwrap()).unwrap();let DocumentSceneContent::Path{segments,..}=&node.content else{panic!("Unresolved neutral vector result")};assert_eq!(segments.iter().filter(|s|matches!(s,PathSegment::Move{..})).count(),expected["contours"].as_u64().unwrap()as usize);assert!((vector_area(segments)-expected["area"].as_f64().unwrap()).abs()<1e-8);}
 let actual=output(&plan);if let Some(expected)=row.get("expected"){assert_eq!(actual,neutral_value(expected),"{}",row["name"]);}if let Some(before)=previous{assert_eq!(actual,before);}previous=Some(actual);}}
 eprintln!("[DEBUG] Native vector preparation preserves semantic text, authored image/group/paint metadata and resolved geometry across neutral work grants");
}
#[test]
fn painted_scenes_vector_plans_cancel_every_phase_and_bound_total_work(){
 let rows:Vec<Value>=serde_json::from_str(include_str!("../../🧫️fixtures/🎬️vector/🔣️.json")).unwrap();let row=rows.last().unwrap();let document:DrawingSnapshot=serde_json::from_value(row["document"].clone()).unwrap();let mut probe=DocumentVectorJob::new(&document,limits(&row["limits"]),resolution_limits()).unwrap();let work=loop{let p=probe.advance(1).unwrap();if p.done{break p.work;}};let expected=output(&probe.result().unwrap());
 let mut exact=resolution_limits();exact.max_work=work;let mut job=DocumentVectorJob::new(&document,limits(&row["limits"]),exact).unwrap();while !job.advance(7).unwrap().done{}assert_eq!(output(&job.result().unwrap()),expected);
 for cap in [1,work-1]{let mut admitted=resolution_limits();admitted.max_work=cap;let mut job=DocumentVectorJob::new(&document,limits(&row["limits"]),admitted).unwrap();let error=job.advance(10000000).unwrap_err();assert_eq!(job.advance(1).unwrap_err(),error);assert_eq!(job.result().unwrap_err(),error);}
 for phase in ["preparing","tracing","algorithms","complete"]{let mut job=DocumentVectorJob::new(&document,limits(&row["limits"]),resolution_limits()).unwrap();let mut seen=false;for _ in 0..100000{if job.advance(1).unwrap().phase==phase{seen=true;break;}}assert!(seen);let published=if phase=="complete"{Some(job.result().unwrap())}else{None};job.cancel();assert!(matches!(job.result(),Err(DocumentSceneError::Cancelled)));assert!(matches!(job.advance(1),Err(DocumentSceneError::Cancelled)));if let Some(plan)=published{assert_eq!(output(&plan),expected);}}
}

fn vector_area(segments:&[PathSegment])->f64{let(mut origin,mut previous,mut sum)=([0.0;2],[0.0;2],0.0);for s in segments{match s{PathSegment::Move{to}=>{origin=*to;previous=*to},PathSegment::Line{to}=>{sum+=((previous[0]-origin[0])*(to[1]-origin[1])-(previous[1]-origin[1])*(to[0]-origin[0]))/2.0;previous=*to},PathSegment::Close=>{},_=>panic!("Unexpected neutral curve")}}sum.abs()}

/// 🔢️ Compares schema numbers as binary64 values while retaining every object field and array entry.
fn neutral_value(value:&Value)->Value{match value{Value::Number(n)=>json!(n.as_f64().unwrap()),Value::Array(items)=>Value::Array(items.iter().map(neutral_value).collect()),Value::Object(fields)=>Value::Object(fields.iter().map(|(key,value)|(key.clone(),neutral_value(value))).collect()),_=>value.clone()}}
#[semio_framework_async_macros::async_test]
async fn painted_scenes_vector_source_owns_and_returns_the_real_store_snapshot_read(){
 let rows:Vec<Value>=serde_json::from_str(include_str!("../../🧫️fixtures/🎬️vector/🔣️.json")).unwrap();let row=rows.last().unwrap();let document:DrawingSnapshot=serde_json::from_value(row["document"].clone()).unwrap();
 let mut store=store::ArtifactStore::new(store::create_document_envelope::<DrawingSnapshot,crate::op::DrawingMutation>("drawing.drawing","vector-owned-snapshot",document,None), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.unwrap();
 store.install_document_store_owners_exact(crate::spr::drawing_document_store_owners());
 let generation=store.generation();let revision=store.content_revision();let read=store.snapshot_read().unwrap();let pointer=read.get()as*const DrawingSnapshot;
 let mut job=DocumentVectorJob::from_snapshot_read(read,limits(&row["limits"]),resolution_limits()).unwrap();
 assert_eq!(job.source.as_ref().unwrap().document()as*const DrawingSnapshot,pointer,"the producer holds the genuine leased root without cloning it");
 assert!(job.snapshot_authority_matches(generation,revision));assert!(!job.snapshot_authority_matches(generation.wrapping_add(1),revision));let mut other_revision=revision;other_revision[0]^=1;assert!(!job.snapshot_authority_matches(generation,other_revision));
 assert!(job.take_snapshot_read().is_none(),"an active cursor retains the source authority");
 loop{let p=job.advance(1).unwrap();if p.done{break;}assert!(matches!(job.result(),Err(DocumentSceneError::Incomplete)));}
 let plan=job.result().unwrap();let node=plan.nodes.iter().find(|n|n.id=="result").unwrap();let DocumentSceneContent::Path{segments,..}=&node.content else{panic!("Unresolved owned vector result")};assert!((vector_area(&segments)-5.0).abs()<1e-8);
 let returned=job.take_snapshot_read().unwrap().return_to_registry_witness().unwrap();assert!(!job.snapshot_authority_matches(generation,revision));drop(job);drop(plan);
 for _ in 0..100000{if store::SpaceMember::close_owned_terminal_is_empty(&store){break;}store::SpaceMember::close_owned_step(&mut store,1,4096).unwrap();}
 assert!(store::SpaceMember::close_owned_terminal_is_empty(&store));assert!(returned.terminal_is_empty());eprintln!("[DEBUG] Real Draw snapshot-read ownership drives the complete vector producer and returns its exact store lease without document cloning");
}

#[semio_framework_async_macros::async_test]
async fn painted_scenes_vector_source_returns_the_real_read_after_cancellation_and_failure(){
 let rows:Vec<Value>=serde_json::from_str(include_str!("../../🧫️fixtures/🎬️vector/🔣️.json")).unwrap();let row=rows.last().unwrap();
 for phase in ["preparing","tracing","algorithms","complete","failure"]{
  let document:DrawingSnapshot=serde_json::from_value(row["document"].clone()).unwrap();let mut store=store::ArtifactStore::new(store::create_document_envelope::<DrawingSnapshot,crate::op::DrawingMutation>("drawing.drawing","vector-cancelled-snapshot",document,None), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.unwrap();store.install_document_store_owners_exact(crate::spr::drawing_document_store_owners());
  let read=store.snapshot_read().unwrap();let mut algorithms=resolution_limits();if phase=="failure"{algorithms.max_work=1;}let mut job=DocumentVectorJob::from_snapshot_read(read,limits(&row["limits"]),algorithms).unwrap();
  if phase=="failure"{let error=job.advance(2).unwrap_err();assert_eq!(job.result().unwrap_err(),error);assert_eq!(job.advance(1).unwrap_err(),error);}else{let mut reached=false;for _ in 0..100000{if job.advance(1).unwrap().phase==phase{reached=true;break;}}assert!(reached,"phase {phase} must be exercised");job.cancel();assert!(matches!(job.result(),Err(DocumentSceneError::Cancelled)));assert!(matches!(job.advance(1),Err(DocumentSceneError::Cancelled)));}
  let(mut close,output)=job.into_retirement();assert!(output.is_none());assert!(close.take_snapshot_read().is_none());while !close.advance(1).unwrap().done{}assert!(!close.terminal_is_empty());let returned=close.take_snapshot_read().unwrap().return_to_registry_witness().unwrap();assert!(close.take_snapshot_read().is_none());assert!(close.terminal_is_empty());drop(close);
  for _ in 0..100000{if store::SpaceMember::close_owned_terminal_is_empty(&store){break;}store::SpaceMember::close_owned_step(&mut store,1,4096).unwrap();}
  assert!(store::SpaceMember::close_owned_terminal_is_empty(&store));assert!(returned.terminal_is_empty());eprintln!("[DEBUG] Draw vector source returns its real read and reaches store terminal after {phase}");
 }
}

#[test]
fn preparation_retirement_drains_partial_nodes_frames_and_source_indexes(){
 let fixtures:Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧹️retirement/🔣️.json")).unwrap();let rows:Vec<Value>=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
 for sample in fixtures["preparation"].as_array().unwrap(){for grant in [1,7,4096]{let row=rows.iter().find(|r|r["name"]==sample["source"]).unwrap();let document:DrawingSnapshot=serde_json::from_value(row["document"].clone()).unwrap();let before=serde_json::to_value(&document).unwrap();let mut caps=limits(&row["limits"]);if let Some(n)=sample["maxDepth"].as_u64(){caps.max_depth=n as usize;}if let Some(n)=sample["maxSegments"].as_u64(){caps.max_segments=n as usize;}let mut job=DocumentSceneJob::new(&document,caps).unwrap();let phase=sample["phase"].as_str().unwrap();
  if phase=="failure"{assert!(job.advance(100000).is_err());assert!(!job.cursor.ids.is_empty());}else if phase!="fresh"{if phase!="cancelled"{let mut reached=false;for _ in 0..100000{if job.advance(1).unwrap().phase==phase{reached=true;break;}}assert!(reached);}for _ in 0..sample["offset"].as_u64().unwrap(){job.advance(1).unwrap();}}
  if phase=="cancelled"{job.cancel();}let pointer=if phase=="complete"{Some(job.cursor.plan.nodes.as_ptr())}else{None};let(mut close,plan)=job.into_retirement();if let Some(pointer)=pointer{assert_eq!(plan.as_ref().unwrap().nodes.as_ptr(),pointer);assert_eq!(neutral_value(&output(plan.as_ref().unwrap())),neutral_value(&row["expected"]));}else{assert!(plan.is_none());}assert!(close.advance(0).is_err());let mut work=0;loop{let count=close.cursor.as_ref().map_or(0,|c|c.frames.len());let p=close.advance(grant).unwrap();let after=close.cursor.as_ref().map_or(0,|c|c.frames.len());assert!(count-after<=grant);assert!(p.work>work&&p.work-work<=grant as u64);work=p.work;if p.done{break;}}assert!(work>=fixtures["minimumPreparationWork"].as_u64().unwrap());assert!(close.terminal_is_empty());assert!(close.cursor.is_none());let p=close.advance(1).unwrap();assert_eq!((p.phase,p.work,p.done),("complete",work,true));if let Some(plan)=plan{let mut output_close=ScenePlanCloseJob::new(plan);while !output_close.advance(grant).unwrap().done{}}assert_eq!(serde_json::to_value(&document).unwrap(),before);println!("[DEBUG] Native preparation {} {phase} retired actual owners in {work} units at grant {grant}",sample["source"]);
 }}
}

#[semio_framework_async_macros::async_test]
async fn vector_retirement_retains_genuine_read_until_all_child_owners_are_empty(){
 let fixtures:Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧹️retirement/🔣️.json")).unwrap();let rows:Vec<Value>=serde_json::from_str(include_str!("../../🧫️fixtures/🎬️vector/🔣️.json")).unwrap();
 for sample in fixtures["vector"].as_array().unwrap(){for grant in [1,7,4096]{let row=rows.iter().find(|r|r["name"]==sample["source"]).unwrap();let document:DrawingSnapshot=serde_json::from_value(row["document"].clone()).unwrap();let before=serde_json::to_value(&document).unwrap();let mut store=store::ArtifactStore::new(store::create_document_envelope::<DrawingSnapshot,crate::op::DrawingMutation>("drawing.drawing","vector-retirement-snapshot",document,None), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.unwrap();store.install_document_store_owners_exact(crate::spr::drawing_document_store_owners());let read=store.snapshot_read().unwrap();let pointer=read.get()as*const DrawingSnapshot;let mut algorithms=resolution_limits();if let Some(n)=sample["maxWork"].as_u64(){algorithms.max_work=n;}if let Some(n)=sample["maxEdges"].as_u64(){algorithms.booleans.max_edges=n as usize;}let mut job=DocumentVectorJob::from_snapshot_read(read,limits(&row["limits"]),algorithms).unwrap();let phase=sample["phase"].as_str().unwrap();
  if phase=="failure"{assert!(job.advance(100000).is_err());assert!(job.preparation.is_some()||job.traces.is_some()||job.algorithms.is_some());}else if phase!="fresh"{if phase!="cancelled"{let mut reached=false;for _ in 0..100000{if job.advance(1).unwrap().phase==phase&&(sample.get("checkpoint").is_none()||job.closing.is_some()){reached=true;break;}}assert!(reached);}for _ in 0..sample["offset"].as_u64().unwrap(){job.advance(1).unwrap();}}
  if phase=="cancelled"{job.cancel();}let published=if phase=="complete"{Some(job.output.as_ref().unwrap().nodes.as_ptr())}else{None};let(mut close,plan)=job.into_retirement();assert_eq!(close.source.as_ref().unwrap().document()as*const DrawingSnapshot,pointer);if let Some(published)=published{assert_eq!(plan.as_ref().unwrap().nodes.as_ptr(),published);}else{assert!(plan.is_none());}assert!(close.take_snapshot_read().is_none());assert!(close.advance(0).is_err());let mut work=0;loop{let p=close.advance(grant).unwrap();assert!(p.work>work&&p.work-work<=grant as u64);work=p.work;if p.done{break;}assert!(close.take_snapshot_read().is_none());assert!(!close.terminal_is_empty());}assert!(work>=fixtures["minimumVectorWork"].as_u64().unwrap());assert!(close.private_is_empty());assert!(!close.terminal_is_empty());assert!(close.child.is_none()&&close.plans.is_empty());let p=close.advance(1).unwrap();assert_eq!((p.phase,p.work,p.done),("complete",work,true));let read=close.take_snapshot_read().unwrap();assert_eq!(read.get()as*const DrawingSnapshot,pointer);assert_eq!(serde_json::to_value(read.get()).unwrap(),before);let returned=read.return_to_registry_witness().unwrap();assert!(close.terminal_is_empty());assert!(close.take_snapshot_read().is_none());if let Some(plan)=plan{let mut output_close=ScenePlanCloseJob::new(plan);while !output_close.advance(grant).unwrap().done{}}for _ in 0..100000{if store::SpaceMember::close_owned_terminal_is_empty(&store){break;}store::SpaceMember::close_owned_step(&mut store,1,4096).unwrap();}assert!(store::SpaceMember::close_owned_terminal_is_empty(&store));assert!(returned.terminal_is_empty());println!("[DEBUG] Native vector {phase} retained its exact source read until {work} private cleanup units completed at grant {grant}");
 }}
}

#[test]
fn mounted_vector_selection_relations_preserve_group_handles_without_unlocking_children(){
 let rows:Vec<Value>=serde_json::from_str(include_str!("../../🧭️selection/🧫️fixtures/🔣️.json")).unwrap();
 for row in rows{let path:Vec<u16>=serde_json::from_value(row["sourcePath"].clone()).unwrap();let selected:Vec<Vec<u16>>=serde_json::from_value(row["selectedPaths"].clone()).unwrap();let locks=serde_json::from_value(row["lockedAncestors"].clone()).unwrap();let result=scene_selection_relation(&path,locks,&selected).unwrap();assert_eq!(json!({"pickable":result.pickable,"boundsSelection":result.bounds_selection}),row["expected"],"{}",row["name"]);println!("[DEBUG] Prepared selection {} resolves pickable={} boundsSelection={:?} from real bounded unsigned ancestry",row["name"],result.pickable,result.bounds_selection);}
}

#[test]
fn mounted_vector_selection_relations_refuse_invalid_unused_addresses_and_capacity(){
 let rows:Vec<Value>=serde_json::from_str(include_str!("../../🧭️selection/🧫️fixtures/⚠️invalid/🔣️.json")).unwrap();
 for row in rows{let path=serde_json::from_value::<Vec<u16>>(row["sourcePath"].clone());let selected=serde_json::from_value::<Vec<Vec<u16>>>(row["selectedPaths"].clone());let locks=serde_json::from_value::<u32>(row["lockedAncestors"].clone());let admitted=match(path,locks,selected){(Ok(path),Ok(locks),Ok(selected))=>scene_selection_relation(&path,locks,&selected).is_ok(),_=>false};assert!(!admitted,"{}",row["name"]);println!("[DEBUG] Prepared selection refused {} through actual unsigned native inputs or bounded ancestry policy",row["name"]);}
}
