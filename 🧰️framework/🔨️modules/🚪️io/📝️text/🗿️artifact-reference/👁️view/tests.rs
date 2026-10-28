//! 🧪️ Borrowed URI fragments match the portable canonical text at every boundary.
use super::*;
use crate::ArtifactDialect;
#[test]
fn original_uri_view_matches_portable_fragment_laws(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){let reference=ArtifactRef{artifact_id:row["id"].as_str().unwrap().into(),dialect:ArtifactDialect{artifact_kind:row["kind"].as_str().unwrap().into(),standard:row["standard"].as_str().unwrap().into(),subset:row["subset"].as_str().unwrap().into()}};check(&reference,row["uri"].as_str().unwrap().as_bytes(),&law);}
 let reference=ArtifactRef{artifact_id:"λ".repeat(law["wideRepeat"].as_u64().unwrap()as usize),dialect:ArtifactDialect{artifact_kind:"s.energy.model".into(),standard:"1".into(),subset:"*".into()}};let expected=format!("{}!s.energy.model@1/*",reference.artifact_id);check(&reference,expected.as_bytes(),&law);
}
fn check(reference:&ArtifactRef,expected:&[u8],law:&serde_json::Value){let view=ArtifactReferenceUriView::new(reference);assert_eq!(view.byte_len(),Some(expected.len()));assert_eq!((0..7).map(|i|view.part(i).unwrap()).collect::<String>().as_bytes(),expected);assert!(view.part(7).is_none());for width in law["widths"].as_array().unwrap(){let width=width.as_u64().unwrap()as usize;let mut bytes=vec![0;width];let mut output=Vec::new();loop{let n=view.read_bytes(output.len(),&mut bytes);if n==0{break}output.extend_from_slice(&bytes[..n]);}assert_eq!(output,expected);assert_eq!(view.read_bytes(expected.len()+1,&mut bytes),0);assert_eq!(view.read_bytes(usize::MAX,&mut bytes),0);}assert_eq!(view.read_bytes(0,&mut[]),0);}
