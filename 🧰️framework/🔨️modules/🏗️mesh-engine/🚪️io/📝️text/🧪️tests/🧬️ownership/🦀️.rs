//! 🧪️ Neutral native mesh admission and physical closure grants.
use super::*;
use pack::value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress};
#[test]
fn neutral_polygon_io_preserves_original_parser_on_short_grants(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🔣️.json")).unwrap();
 let text=serde_json::to_string(&law["source"]).unwrap();let mut cursor=PolygonSourcePreparation::new();
 let empty=RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:0};
 let work=PolygonSourceGrant{maximum_units:4096,maximum_projection_bytes:4096,retirement:empty};
 assert!(cursor.step(&text,work).unwrap().source.is_none());assert_eq!(cursor.stage,1);
 let original=cursor.parser.as_ref().unwrap() as *const json::JsonParseCursor;
 let demand=pack::value::retirement::owned_retirement_birth_bytes::<json::JsonParseCursor>();
 for retirement in [empty,RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:demand-1,maximum_depth:1,..empty}]{let result=cursor.step(&text,PolygonSourceGrant{retirement,..work}).unwrap();assert_eq!(result.retirement,RetainedCloneProgress::default());assert_eq!(cursor.parser.as_ref().unwrap() as *const json::JsonParseCursor,original);assert!(cursor.retirement.is_none());}
 assert!(cursor.step(&text,PolygonSourceGrant{retirement:RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:demand,..empty},..work}).is_err());assert_eq!(cursor.parser.as_ref().unwrap() as *const json::JsonParseCursor,original);
 let mut turns=0;let mut allocated=0;let mut released=0;let source=loop{let grant=cursor.cold_grant(1,8).unwrap();let result=cursor.step(&text,grant).unwrap();assert!(result.retirement.fits(grant.retirement));allocated+=result.retirement.retained_capacity_bytes;released+=result.retirement.released_bytes;turns+=1;assert!(turns<100000);if let Some(source)=result.source{break source;}};
 let encoded:serde_json::Value=serde_json::from_str(&encode_polygon_mesh_source(&source)).unwrap();assert_eq!(encoded["faces"],law["source"]["faces"]);for (actual,expected) in encoded["vertices"].as_array().unwrap().iter().zip(law["source"]["vertices"].as_array().unwrap()){for (actual,expected) in actual.as_array().unwrap().iter().zip(expected.as_array().unwrap()){assert_eq!(actual.as_f64(),expected.as_f64());}}
 assert!(cursor.retirement.is_none());assert!(cursor.parser.is_none());assert!(cursor.projection.is_none());assert!(allocated>0);assert!(released>0);
 eprintln!("[DEBUG] Mesh neutral IO: vertices={} faces={} turns={turns} admittedFrameBytes={allocated} releasedBytes={released}; independent=Serde",source.vertices.len(),source.faces.len());
}
