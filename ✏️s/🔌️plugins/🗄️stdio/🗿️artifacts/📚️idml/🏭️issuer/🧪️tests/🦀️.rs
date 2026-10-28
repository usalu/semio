use super::*;
use protocol::ArtifactCodecOwnerSource;
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
#[test]
fn declared_idml_source_admits_one_shared_retained_issuer(){
 let demand=IdmlCodecOwners::demand().unwrap();assert!(demand.copy_bytes>0);assert!(demand.capacity_bytes>0);
 for short in 0..4 {
  let mut supplied=grant(demand);match short{0=>supplied.maximum_items=0,1=>supplied.maximum_copy_bytes=demand.copy_bytes-1,2=>supplied.maximum_capacity_bytes=demand.capacity_bytes-1,_=>supplied.maximum_depth=demand.depth-1}
  match IdmlCodecOwners::admit(supplied){Err(error)=>{assert!(error.owners.is_none());assert_eq!(error.progress,RetainedCloneProgress::default())},Ok((owners,_))=>{owners::close_unscheduled_catalog(owners);panic!("IDML source accepted short original authority")}}
 }
 let owners=protocol::codec_document_owners::<IdmlSnapshot,IdmlMutation,IdmlCodecOwners>().unwrap();
 assert!(owners.constructor_is_complete());assert!(owners.original_semantic_issuer_is_shared());assert!(owners.one_item_preparation.is_some());assert!(owners.replay_preparation.is_some());owners::close_unscheduled_catalog(owners);
 let codec=(crate::artifact_declaration::native_codecs()[0].codec)();assert_eq!(codec.schema,crate::DOCUMENT_SCHEMA);assert_eq!(codec.extension,"idml");
}
