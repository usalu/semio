use semio_framework_composition_admission::{ChildRestoreProjection,ChildRestoreProjectionError};
use semio_framework_schema_composition::{ArtifactCompositionFields,ChildRefFields,ChildRefVisitor,ChildSlotSelection,ChildSlotSpec};
use serde_json::Value;

struct Parent(Vec<Value>);
impl ArtifactCompositionFields for Parent {
 fn child_slots()->&'static[ChildSlotSpec]{&[ChildSlotSpec{name:"members",selection:ChildSlotSelection::ArtifactKind("s.test.member"),many:true}]}
 fn visit_child_refs<'a,V:ChildRefVisitor<'a>>(&'a self,visitor:&mut V)->Result<(),V::Error>{for row in &self.0{visitor.step()?;visitor.child(if row["slot"]=="members"{"members"}else{"other"},ChildRefFields{child_id:row["childId"].as_str().unwrap(),artifact_id:row["artifactId"].as_str().unwrap(),artifact_kind:row["kind"].as_str().unwrap(),standard:row["standard"].as_str().unwrap(),subset:row["subset"].as_str().unwrap()})?;}Ok(())}
}

#[test]
fn original_projection_neutral_corpus_matches_serde_identity_sets(){
 let fixture:Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){let children=row["children"].as_array().unwrap().clone();let ids:std::collections::BTreeSet<_>=children.iter().map(|child|child["childId"].as_str().unwrap()).collect();let artifacts:std::collections::BTreeSet<_>=children.iter().map(|child|child["artifactId"].as_str().unwrap()).collect();let oracle=ids.len()==children.len()&&artifacts.len()==children.len()&&children.iter().all(|child|child["slot"]=="members"&&child["kind"]=="s.test.member"&&["childId","artifactId","standard","subset"].iter().all(|key|child[*key].as_str().is_some_and(|value|!value.is_empty()&&value.len()<=256)));assert_eq!(oracle,row["accepted"].as_bool().unwrap());let parent=Parent(children);let projection=ChildRestoreProjection::from_snapshot(&parent);assert_eq!(projection.is_ok(),oracle,"{}",row["id"]);if let Ok(projection)=projection{assert_eq!(projection.len(),parent.0.len());let rows=(0..projection.len()).map(|index|projection.get(index).unwrap()).collect::<Vec<_>>();for(slot,fields)in &rows{let expected=semio_framework_artifact_reference_borrow::BorrowedArtifactRef{artifact_id:fields.artifact_id,artifact_kind:fields.artifact_kind,standard:fields.standard,subset:fields.subset};assert!(projection.admits_member(slot,expected));assert!(!projection.admits_member("absent",expected));for other in [semio_framework_artifact_reference_borrow::BorrowedArtifactRef{artifact_id:"absent",..expected},semio_framework_artifact_reference_borrow::BorrowedArtifactRef{artifact_kind:"s.other.member",..expected},semio_framework_artifact_reference_borrow::BorrowedArtifactRef{standard:"absent",..expected},semio_framework_artifact_reference_borrow::BorrowedArtifactRef{subset:"absent",..expected}]{assert!(!projection.admits_member(slot,other));}}assert!(projection.admit_complete(rows.iter().rev().copied()).is_ok());if !rows.is_empty(){assert_eq!(projection.admit_complete(rows.iter().skip(1).copied()),Err(ChildRestoreProjectionError::IncompleteSet));assert_eq!(projection.admit_complete([rows[0],rows[0]]),Err(ChildRestoreProjectionError::DuplicateChild));}}}
}

#[test]
fn original_projection_keeps_reference_field_and_walk_limits(){
 let child=|index:usize|serde_json::json!({"slot":"members","childId":index.to_string(),"artifactId":index.to_string(),"kind":"s.test.member","standard":"v1","subset":"all"});let mut parent=Parent((0..64).map(child).collect());assert_eq!(ChildRestoreProjection::from_snapshot(&parent).unwrap().len(),64);parent.0.push(child(64));assert!(matches!(ChildRestoreProjection::from_snapshot(&parent),Err(ChildRestoreProjectionError::ReferenceLimit)));parent.0=vec![child(0)];parent.0[0]["childId"]=Value::String("ä".repeat(128));assert!(ChildRestoreProjection::from_snapshot(&parent).is_ok());parent.0[0]["childId"]=Value::String(format!("{}x","ä".repeat(128)));assert!(matches!(ChildRestoreProjection::from_snapshot(&parent),Err(ChildRestoreProjectionError::InvalidReference)));
}

struct SparseParent(usize);
impl ArtifactCompositionFields for SparseParent {
 fn visit_child_refs<'a,V:ChildRefVisitor<'a>>(&'a self,visitor:&mut V)->Result<(),V::Error>{for _ in 0..self.0{visitor.step()?;}Ok(())}
}

#[test]
fn original_projection_charges_empty_walk_steps_before_identity_visits(){
 assert!(ChildRestoreProjection::from_snapshot(&SparseParent(256)).unwrap().is_empty());assert!(matches!(ChildRestoreProjection::from_snapshot(&SparseParent(257)),Err(ChildRestoreProjectionError::TraversalLimit)));
}
