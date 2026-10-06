//! 🧪️ Neutral planar regions, grant limits, fill-rule semantics and private publication.
use super::*;
use serde_json::{Value,json};
#[test]
fn boolean_translated_stroke_junctions_preserve_endpoint_incidence(){
 let rows:Vec<Value>=serde_json::from_str(include_str!("../🧫️fixtures/🔗️endpoints/🔣️.json")).unwrap();
 for row in rows {let source=input(&row);let mut previous=None;for grant in [1,7,4096]{let (segments,_)=finish(source.clone(),grant);let rings=contours(&segments);assert_eq!(rings.len(),1,"{}",row["name"]);assert!(area(&rings[0])>0.0);for ring in &rings{for at in 0..ring.len(){assert!(length(ring[at],ring[(at+1)%ring.len()])>source.epsilon);}}if let Some(before)=&previous{assert_eq!(&segments,before);}previous=Some(segments);println!("[DEBUG] Native endpoint-preserving translated stroke junction {} grant={grant} contours=1",row["name"]);}}
}
fn rows()->Vec<Value> {serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()}
fn input(row:&Value)->BooleanInput {
 let v=&row["input"];BooleanInput {operation:BooleanOperation::parse(v["operation"].as_str().unwrap()).unwrap(),operands:v["operands"].as_array().unwrap().iter().map(|operand|BooleanOperand {fill_rule:match operand["fillRule"].as_str().unwrap() {"nonzero"=>BooleanFillRule::Nonzero,"evenodd"=>BooleanFillRule::Evenodd,_=>panic!("Invalid fixture rule")},contours:operand["contours"].as_array().unwrap().iter().map(|ring|ring.as_array().unwrap().iter().map(|p|[p[0].as_f64().unwrap(),p[1].as_f64().unwrap()]).collect()).collect()}).collect(),epsilon:v["epsilon"].as_f64().unwrap(),max_edges:v["maxEdges"].as_u64().unwrap() as usize,max_parameters:v["maxParameters"].as_u64().unwrap() as usize,max_atomic_edges:v["maxAtomicEdges"].as_u64().unwrap() as usize,max_segments:v["maxSegments"].as_u64().unwrap() as usize,max_work:v["maxWork"].as_u64().unwrap()}
}
fn contours(segments:&[PathSegment])->Vec<Vec<Vec2>> {
 let mut rings=Vec::<Vec<Vec2>>::new();for segment in segments {match segment {PathSegment::Move {to}=>rings.push(vec![*to]),PathSegment::Line {to}=>rings.last_mut().unwrap().push(*to),PathSegment::Close=>{},_=>panic!("Unexpected boolean curve")}}rings
}
fn area(ring:&[Vec2])->f64 {let mut sum=0.0;for at in 1..ring.len().saturating_sub(1) {sum+=cross(ring[0],ring[at],ring[at+1]);}sum/2.0}
fn filled(rings:&[Vec<Vec2>],p:Vec2,rule:BooleanFillRule)->bool {
 let mut winding=0;for ring in rings {for at in 0..ring.len() {let (a,b)=(ring[at],ring[(at+1)%ring.len()]);let c=cross(a,b,p);if a[1]<=p[1]&&b[1]>p[1]&&c>0.0 {winding+=1;}else if a[1]>p[1]&&b[1]<=p[1]&&c<0.0 {winding-=1;}}}if rule==BooleanFillRule::Evenodd {winding%2!=0} else {winding!=0}
}
fn finish(input:BooleanInput,grant:usize)->(Vec<PathSegment>,BooleanProgress) {
 let limit=input.max_work;let mut job=BooleanJob::new(input).unwrap();assert_eq!(job.result().unwrap_err(),BooleanError::Incomplete);let mut work=0;
 loop {let p=job.advance(grant).unwrap();assert!(p.work-work<=grant as u64);assert!(p.work<=limit);work=p.work;if p.done {return (job.into_result().unwrap(),p);}assert_eq!(job.result().unwrap_err(),BooleanError::Incomplete);}
}
#[test]
fn boolean_neutral_regions_with_identical_outputs_across_grants() {
 for row in rows() {let input=input(&row);let mut previous=None;
  for grant in [1,7,4096] {let (segments,p)=finish(input.clone(),grant);let rings=contours(&segments);assert_eq!(rings.len(),row["expected"]["contours"].as_u64().unwrap() as usize,"{}",row["name"]);let actual=rings.iter().map(|ring|area(ring)).sum::<f64>();let expected=row["expected"]["area"].as_f64().unwrap();assert!((actual-expected).abs()<=1e-18f64.max(input.epsilon*input.epsilon*16.0),"{}: {actual} vs {expected}",row["name"]);
   if row["name"]!="translated mixed axis preserves geometry"&&row["name"]!="tiny separate regions" {for y in -2..17 {for x in -2..23 {let point=[x as f64+0.317,y as f64+0.173];let expected=input.operands.iter().map(|operand|filled(&operand.contours,point,operand.fill_rule)).reduce(|a,b|input.operation.apply(a,b)).unwrap();assert_eq!(filled(&rings,point,BooleanFillRule::Nonzero),expected);}}}
   if let Some(expected)=row["expected"]["segments"].as_array() {let actual=segments.iter().map(|s|match s {PathSegment::Move{to}=>json!({"kind":"move","to":to}),PathSegment::Line{to}=>json!({"kind":"line","to":to}),PathSegment::Close=>json!({"kind":"close"}),_=>panic!("Unexpected boolean curve")}).collect::<Vec<_>>();assert_eq!(actual.len(),expected.len());for (a,b) in actual.iter().zip(expected) {assert_eq!(a["kind"],b["kind"]);if a["kind"]!="close" {for axis in 0..2 {assert_eq!(a["to"][axis].as_f64(),b["to"][axis].as_f64());}}}}
   if let Some(limit)=row["expected"]["maxPairs"].as_u64() {assert!(p.pairs<=limit);assert!(p.work<=row["expected"]["maxWork"].as_u64().unwrap());println!("[DEBUG] boolean spatial index checked {} candidate pairs in {} work units",p.pairs,p.work);}
   if let Some(before)=&previous {assert_eq!(&segments,before);}previous=Some(segments);assert!(p.edges<=input.max_edges);assert!(p.parameters<=input.max_parameters);
  }println!("[DEBUG] boolean fixture {} retained filled regions and identical grant outputs",row["name"]);
 }
}
#[test]
fn boolean_resource_and_precision_refusals_remain_sticky() {
 let source=input(&rows()[0]);assert!(BooleanOperation::parse("bogus").is_err());let mut invalid=Vec::new();
 for kind in 0..9 {let mut row=source.clone();match kind {0=>row.operands.clear(),1=>row.operands=vec![row.operands[0].clone();1025],2=>row.epsilon=0.0,3=>row.epsilon=f64::INFINITY,4=>row.max_edges=0,5=>row.max_parameters=0,6=>row.max_atomic_edges=0,7=>row.max_segments=0,_=>row.max_work=0}invalid.push(row);}
 for row in invalid {assert!(BooleanJob::new(row).is_err());}
 for kind in 0..8 {let mut row=source.clone();match kind {0=>row.operands[0].contours[0][0][0]=f64::NAN,1=>row.operands[0].contours[0][0][0]=1e12+1.0,2=>row.max_edges=3,3=>row.max_parameters=2,4=>row.max_atomic_edges=1,5=>row.max_segments=3,6=>row.max_work=1,_=>{row=input(&rows()[25]);row.epsilon=1e-12;}}
  let mut job=BooleanJob::new(row).unwrap();let error=job.advance(10000000).unwrap_err();assert_eq!(job.result().unwrap_err(),error);assert_eq!(job.advance(1).unwrap_err(),error);
 }assert!(BooleanJob::new(source).unwrap().advance(0).is_err());
}
#[test]
fn boolean_cancellation_covers_every_public_phase_and_keeps_owned_results() {
 let input=input(&rows()[0]);let mut probe=BooleanJob::new(input.clone()).unwrap();let mut seen=BTreeSet::new();
 loop {let p=probe.advance(1).unwrap();if seen.insert(p.phase.as_str()) {let mut job=BooleanJob::new(input.clone()).unwrap();job.advance(p.work as usize).unwrap();job.cancel();assert_eq!(job.result().unwrap_err(),BooleanError::Cancelled);assert_eq!(job.advance(1).unwrap_err(),BooleanError::Cancelled);}if p.done {break;}}
 assert_eq!(seen,BTreeSet::from(["preparing","indexing","intersections","splitting","classifying","contours","compacting","emitting","complete"]));let mut job=BooleanJob::new(input).unwrap();job.advance(1000000).unwrap();let published=job.result().unwrap().to_vec();job.cancel();assert!(!published.is_empty());
}
#[test]
fn boolean_exact_resource_caps_and_progress_taxonomy() {
 let input=input(&rows()[0]);let (expected,p)=finish(input.clone(),1);let mut exact=input;exact.max_edges=p.vertices;exact.max_parameters=p.parameters;exact.max_atomic_edges=p.atomic_edges;exact.max_segments=p.segments;exact.max_work=p.work;assert_eq!(finish(exact,7).0,expected);
 let value=json!({"phase":p.phase.as_str(),"operands":p.operands,"vertices":p.vertices,"edges":p.edges,"parameters":p.parameters,"pairs":p.pairs,"atomicEdges":p.atomic_edges,"boundaryEdges":p.boundary_edges,"contours":p.contours,"segments":p.segments,"work":p.work,"done":p.done});assert_eq!(value.as_object().unwrap().len(),12);println!("[DEBUG] boolean exact limits completed {} segments in {} work units",p.segments,p.work);
}

#[test]
fn boolean_retirement_hands_off_owned_sources_and_drains_private_arrangements(){
 let cases:Value=serde_json::from_str(include_str!("../🧫️fixtures/🧹️retirement/🔣️.json")).unwrap();
 for row in cases["cases"].as_array().unwrap(){for grant in [1,7,4096]{
  let fixture=rows().into_iter().find(|r|r["name"]==row["source"]).unwrap();let mut source=input(&fixture);if let Some(n)=row["maxParameters"].as_u64(){source.max_parameters=n as usize;}let source_ptr=source.operands.as_ptr();let source_points=source.operands.iter().map(|o|o.contours.clone()).collect::<Vec<_>>();let mut job=BooleanJob::new(source).unwrap();let phase=row["phase"].as_str().unwrap();
  if phase=="failure"{assert!(job.advance(4096).is_err());}else if phase!="fresh"{if phase!="cancelled"{let mut reached=false;for _ in 0..100000{if job.advance(1).unwrap().phase.as_str()==phase{reached=true;break;}}assert!(reached);}for _ in 0..row["offset"].as_u64().unwrap(){job.advance(1).unwrap();}}
  if phase=="cancelled"{job.cancel();}let published=if phase=="complete"{Some(job.result().unwrap().as_ptr())}else{None};
  let inventory=json!({"edges":job.source.iter().map(|e|e.parameters.len()).collect::<Vec<_>>(),"grid":job.grid.len(),"atomic":job.atomic_ids.len(),"outgoing":job.outgoing.len(),"raw":job.raw.len(),"positions":job.positions.len(),"rings":job.rings.len()});
  let expected=cases["flatSlots"].as_u64().unwrap()+inventory["edges"].as_array().unwrap().len()as u64*cases["sourceEdgeSteps"].as_u64().unwrap()+inventory["edges"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()).sum::<u64>()+["grid","atomic","outgoing","raw","positions","rings"].iter().map(|k|inventory[k].as_u64().unwrap()).sum::<u64>();
  assert_eq!(expected,row["work"].as_u64().unwrap());let(mut close,operands,output)=job.into_retirement();if phase=="cancelled"{assert!(operands.is_empty());}else{assert_eq!(operands.as_ptr(),source_ptr);assert_eq!(operands.iter().map(|o|o.contours.clone()).collect::<Vec<_>>(),source_points);}
  if let Some(ptr)=published{assert_eq!(output.as_ref().unwrap().as_ptr(),ptr);let actual=contours(output.as_ref().unwrap()).iter().map(|r|area(r)).sum::<f64>();assert_eq!(actual,fixture["expected"]["area"].as_f64().unwrap());}else{assert!(output.is_none());}
  assert!(close.advance(0).is_err());let mut work=0;loop{let before=close.job.as_ref().map(|j|(j.source.len(),j.raw.len(),j.rings.len())).unwrap_or_default();let p=close.advance(grant).unwrap();let after=close.job.as_ref().map(|j|(j.source.len(),j.raw.len(),j.rings.len())).unwrap_or_default();assert!(before.0-after.0<=grant&&before.1-after.1<=grant&&before.2-after.2<=grant);assert!(p.work>work&&p.work-work<=grant as u64);work=p.work;if p.done{break;}}assert_eq!(work,expected);assert!(close.terminal_is_empty());assert!(close.job.is_none());assert!(close.edge.is_none());let p=close.advance(1).unwrap();assert_eq!((p.phase,p.work,p.done),("complete",work,true));
  println!("[DEBUG] Native Boolean retirement {} {}: {} structural steps at grant {}",row["source"],phase,expected,grant);
 }}
}
