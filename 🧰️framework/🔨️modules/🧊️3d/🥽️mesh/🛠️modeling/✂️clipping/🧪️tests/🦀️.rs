//! ✂️ Clipping: 2 tests covering edge classification matches shared vectors; clipping provenance is complete and transferred once.
use super::*;
#[test]
fn edge_classification_matches_shared_vectors(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for row in cases.as_array().unwrap(){
  let vector=|name:&str|{let a=row[name].as_array().unwrap();Vec3(std::array::from_fn(|i|a[i].as_f64().unwrap() as f32))};
  let epsilon=row["epsilon"].as_f64().unwrap() as f32;
  let(da,_,intersection)=mesh_clip_edge(vector("a"),vector("b"),vector("normal"),row["distance"].as_f64().unwrap() as f32,epsilon);
  assert_eq!(da<=epsilon,row["expected"]["retained"].as_bool().unwrap());
  match intersection{Some((point,t))=>{assert_eq!(t as f64,row["expected"]["parameter"].as_f64().unwrap());let position=row["expected"]["position"].as_array().unwrap();for i in 0..3{assert_eq!(point.0[i] as f64,position[i].as_f64().unwrap());}},None=>assert!(row["expected"]["position"].is_null())}
 }
}

#[test]
fn clipping_provenance_is_complete_and_transferred_once(){
 let mut job=MeshClipJob::new(Vec3([1.0,0.0,0.0]),0.0,0.0001);
 assert!(job.take_source_faces().is_err());assert!(job.vertex_source(0).is_err());
 let mut soup=Soup {positions:vec![[-1.0,-1.0,-1.0],[1.0,-1.0,-1.0],[1.0,1.0,-1.0],[-1.0,1.0,-1.0],[-1.0,-1.0,1.0],[1.0,-1.0,1.0],[1.0,1.0,1.0],[-1.0,1.0,1.0]],faces:vec![vec![0,3,2,1],vec![4,5,6,7],vec![0,1,5,4],vec![1,2,6,5],vec![2,3,7,6],vec![3,0,4,7]],normals:Vec::new()};
 let mut done=false;for _ in 0..4096{if job.advance(&mut soup).unwrap(){done=true;break;}}
 assert!(done);assert!(soup.positions.iter().all(|point|point.iter().all(|value|value.is_finite())));
 assert!(!job.take_source_faces().unwrap().is_empty());assert!(job.take_source_faces().is_err());
 assert!(!job.take_corner_faces().unwrap().is_empty());assert!(job.take_corner_faces().is_err());
 assert!(!job.take_face_weights().unwrap().is_empty());assert!(job.take_face_weights().is_err());
 assert!(job.vertex_source(0).is_ok());assert!(job.advance(&mut soup).is_err());
}
