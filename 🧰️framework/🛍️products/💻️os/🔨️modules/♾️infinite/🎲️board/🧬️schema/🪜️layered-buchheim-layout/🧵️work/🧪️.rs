//! 🧪️ Numeric continuation agrees with neutral coordinates and the original string layout.
use super::*;
use super::super::super::LayoutControl;
#[test]
fn numeric_layered_continuation_matches_original_buchheim_geometry(){
 let corpus=semio_framework_pack_json::parse(include_str!("🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 let options=DagLayoutOptions{layer_spacing:100.0,sibling_gap:40.0,..Default::default()};
 for row in corpus.get("cases").unwrap().as_array().unwrap(){
  let id=row.get("id").unwrap().as_str().unwrap();let ids:Vec<String>=row.get("nodes").unwrap().as_array().unwrap().iter().map(|v|v.as_str().unwrap().to_owned()).collect();
  let edges:Vec<_>=row.get("edges").unwrap().as_array().unwrap().iter().map(|pair|{let p=pair.as_array().unwrap();LayeredLayoutEdge{source:p[0].as_f64().unwrap()as usize,target:p[1].as_f64().unwrap()as usize}}).collect();
  let mut nodes=vec![LayeredLayoutNode::new();ids.len()+1];let mut cursor=LayeredLayoutCursor::new();let mut complete=false;
  for _ in 0..10000{if cursor.step(&mut nodes,&edges,&options).unwrap(){complete=true;break}}
  assert!(complete,"{id} must finish in finite scalar opportunities");
  let mut progress=|_|true;let mut control=LayoutControl::new(u64::MAX,&mut progress);
  let original=super::super::layered_positions(&ids.iter().cloned().collect(),&edges.iter().map(|e|(ids[e.source].clone(),ids[e.target].clone())).collect::<Vec<_>>(),&options,&mut control).unwrap();
  for(index,point)in row.get("positions").unwrap().as_array().unwrap().iter().enumerate(){let expected=point.as_array().unwrap();let expected=(expected[0].as_f64().unwrap(),expected[1].as_f64().unwrap());let actual=nodes[index].position();assert_eq!(actual,expected,"neutral {id} node {index}");assert_eq!(actual,original[&ids[index]],"genuine {id} node {index}");}
 }
}
