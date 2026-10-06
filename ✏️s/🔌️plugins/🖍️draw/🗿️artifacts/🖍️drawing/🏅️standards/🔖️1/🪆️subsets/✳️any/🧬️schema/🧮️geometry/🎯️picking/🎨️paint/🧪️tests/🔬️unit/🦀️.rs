use super::*;
fn query(row:&serde_json::Value)->PaintedPathQuery{
 let q=&row["query"];
 PaintedPathQuery{point:serde_json::from_value(q["point"].clone()).unwrap(),transform:serde_json::from_value(q["transform"].clone()).unwrap(),tolerance:q["tolerance"].as_f64().unwrap(),flatness:q["flatness"].as_f64().unwrap(),fill:q["fill"].as_bool().unwrap(),fill_rule:serde_json::from_value(q["fillRule"].clone()).unwrap(),stroke:(!q["stroke"].is_null()).then(||PaintedPathStroke{width:q["stroke"]["width"].as_f64().unwrap(),cap:serde_json::from_value(q["stroke"]["cap"].clone()).unwrap(),join:serde_json::from_value(q["stroke"]["join"].clone()).unwrap(),dash:serde_json::from_value(q["stroke"]["dash"].clone()).unwrap()})}
}
#[test]
fn painted_query_neutral_regions_are_granted_and_borrowed(){
 let rows:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
 for row in rows.as_array().unwrap(){for grant in [1,7,4096]{
  let source:Vec<crate::PathSegment>=serde_json::from_value(row["segments"].clone()).unwrap();let before=serde_json::to_value(&source).unwrap();let mut job=PaintedPathHitJob::new(query(row)).unwrap();let mut work=0;let mut reads=0;let mut done=false;
  assert!(job.result().is_err());
  for _ in 0..2000000{let p=job.advance(grant,|index|{reads+=1;source.get(index).cloned()}).unwrap();assert!(p.work>work&&p.work-work<=grant as u64);work=p.work;if p.done{done=true;break;}}
  assert!(done);assert_eq!(reads,source.len()+1);assert_eq!(job.result().unwrap().contains,row["expected"].as_bool().unwrap(),"{}",row["name"]);if !row["bounds"].is_null(){assert_eq!(job.result().unwrap().bounds,Some(serde_json::from_value(row["bounds"].clone()).unwrap()));}assert_eq!(serde_json::to_value(&source).unwrap(),before);
  let output=job.result().unwrap();let(mut retired,moved)=job.into_retirement();assert_eq!(moved,Some(output));while !retired.advance(grant).unwrap().done{}assert!(retired.terminal_is_empty());
  eprintln!("[DEBUG] Native painted query {} grant={grant} work={work} contains={} terminal_empty=true",row["name"],output.contains);
 }}
}
#[test]
fn painted_query_cancellation_preserves_actual_owners_until_retired(){
 let rows:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();let row=&rows[6];let source:Vec<crate::PathSegment>=serde_json::from_value(row["segments"].clone()).unwrap();
 let phases:Vec<String>=serde_json::from_str(include_str!("../../🧫️fixtures/🧹️retirement/🔣️.json")).unwrap();
 for phase in phases{for grant in [1,7,4096]{
  let mut job=PaintedPathHitJob::new(query(row)).unwrap();let mut reached=false;
  for _ in 0..2000000{if job.phase==phase{reached=true;break;}job.advance(1,|index|source.get(index).cloned()).unwrap();}
  assert!(reached,"{phase}");let flat=job.flat.len();let polygons=job.polygons.len();let child=job.flatten.is_some()||job.outline.is_some();job.cancel();assert_eq!(job.flat.len(),flat);assert_eq!(job.polygons.len(),polygons);assert_eq!(job.flatten.is_some()||job.outline.is_some(),child);assert!(job.result().is_err());assert!(job.advance(1,|_|None).is_err());
  let(mut retired,output)=job.into_retirement();assert!(output.is_none());assert!(retired.advance(0).is_err());let mut work=0;let mut done=false;
  for _ in 0..2000000{let p=retired.advance(grant).unwrap();assert!(p.work>work&&p.work-work<=grant as u64);work=p.work;if p.done{done=true;break;}}
  assert!(done&&retired.terminal_is_empty());assert!(retired.advance(1).unwrap().done);
  eprintln!("[DEBUG] Native painted query cancelled phase={phase} grant={grant} actual_retirement_work={work} terminal_empty=true");
 }}
}
#[test]
fn painted_query_refuses_unsafe_grants_before_borrowed_admission(){
 let rows:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();let mut job=PaintedPathHitJob::new(query(&rows[0])).unwrap();let mut reads=0;
 for grant in [0,9_007_199_254_740_992]{assert!(job.advance(grant,|_|{reads+=1;None}).is_err());}assert_eq!(reads,0);let(mut retired,output)=job.into_retirement();assert!(output.is_none());while !retired.advance(1).unwrap().done{}assert!(retired.terminal_is_empty());
}
#[test]
fn failed_painted_query_retires_its_actual_flatten_child(){
 let rows:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();let mut job=PaintedPathHitJob::new(query(&rows[0])).unwrap();let source=[PathSegment::Move{to:[1e10,0.0]},PathSegment::Line{to:[10.0,10.0]}];
 assert!(job.advance(4096,|index|source.get(index).cloned()).is_err());assert!(job.flatten.is_some());assert!(job.result().is_err());assert!(job.advance(1,|_|None).is_err());job.cancel();assert!(job.flatten.is_some());let(mut retired,output)=job.into_retirement();assert!(output.is_none());let mut work=0;
 while !retired.terminal_is_empty(){let p=retired.advance(1).unwrap();assert_eq!(p.work-work,1);work=p.work;}eprintln!("[DEBUG] Native failed painted query retired its actual flatten child in {work} structural units");
}
