//! 🧵️ Original neutral DAG owners conserve physical custody under independently admitted currencies.
use super::*;
use semio_framework_value::{retained_clone::{RetainedCloneGrant as Grant,RetainedCloneProgress as Progress}};
use crate::sqlite_snapshot_tests::property_allocation::heap;

fn close(owner:Box<dyn ErasedSnapshotRetirement>,copy:usize,born:usize,original:usize){
 let mut slot=Some(owner);let mut allocated=born;let mut released=0;
 for turn in 0..1_000_000{
  let owner=slot.as_ref().unwrap();let work=copy.max(owner.next_copy_byte_demand().unwrap());
  let(demand,a,b)=heap(||semio_framework_value::factory_ticket_demands(owner,work).unwrap());assert_eq!((a,b),(0,0));
  let grant=Grant{maximum_items:1,maximum_copy_bytes:work,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
  if owner.terminal_is_empty()&&demand.release_bytes>0{let(step,a,b)=heap(||semio_framework_value::close_factory_ticket(&mut slot,Grant{maximum_release_bytes:demand.release_bytes-1,..grant}).unwrap());assert_eq!(step.progress(),Progress::default());assert_eq!((a,b),(0,0));assert!(slot.is_some());}
  let(step,a,b)=heap(||semio_framework_value::close_factory_ticket(&mut slot,grant).unwrap());let receipt=step.progress();assert!(receipt.fits(grant));assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(a,b));allocated+=a;released+=b;
  if slot.is_none(){assert_eq!(released,original+allocated);let(step,a,b)=heap(||semio_framework_value::close_factory_ticket(&mut slot,Grant::default()).unwrap());assert!(matches!(step,semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)));assert_eq!((a,b),(0,0));println!("[DEBUG] Original DAG physical custody copy={copy} turns={turn} original={original} born={allocated} release={released}");return;}
 }
 panic!("DAG retirement never reached actual terminal emptiness");
}

#[test]
fn neutral_fixture_retires_exact_mutation_shared_snapshot_and_final_snapshot_owners(){
 let source=include_str!("../../../🌿️vcs/🧫️fixtures/🔣️mutations.json");let oracle:serde_json::Value=serde_json::from_str(source).unwrap();let parsed=semio_framework_pack_json::parse(source,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&parsed)).unwrap(),oracle);
 let laws:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
 for copy in laws["retirement"]["copyBytes"].as_array().unwrap(){let copy=copy.as_u64().unwrap()as usize;
  let(snapshot,a,b)=heap(||crate::sqlite_snapshot_tests::full(0x8000000000000000));let original=a-b;let pointer=snapshot.nodes.as_ptr();let factory=DagOwnedSnapshotRetirementFactory;let bytes=factory.retirement_birth_bytes(&snapshot);let mut value=snapshot;
  for grant in [Grant{maximum_items:0,maximum_capacity_bytes:bytes,maximum_depth:1,..Default::default()},Grant{maximum_items:1,maximum_capacity_bytes:bytes-1,maximum_depth:1,..Default::default()},Grant{maximum_items:1,maximum_capacity_bytes:bytes,maximum_depth:0,..Default::default()}]{let(result,a,b)=heap(||factory.retire_owned(value,grant));let(_,returned)=result.err().unwrap();assert_eq!((a,b),(0,0));assert_eq!(returned.nodes.as_ptr(),pointer);value=returned;}
  let(result,a,b)=heap(||factory.retire_owned(value,Grant{maximum_items:1,maximum_capacity_bytes:bytes,maximum_depth:1,..Default::default()}));let(owner,receipt)=result.unwrap();assert_eq!((a,b),(bytes,0));assert_eq!(receipt.retained_capacity_bytes,bytes);close(owner,copy,a,original);
  let(shared,a,b)=heap(||Arc::new(crate::sqlite_snapshot_tests::full(0x8000000000000000)));let original=a-b;let factory=DagSnapshotRetirementFactory;let bytes=factory.retirement_birth_bytes(&shared);let(result,a,b)=heap(||factory.retire(shared,Grant{maximum_items:1,maximum_capacity_bytes:bytes,maximum_depth:1,..Default::default()}));let(owner,receipt)=result.unwrap();assert_eq!((a,b),(bytes,0));assert_eq!(receipt.retained_capacity_bytes,bytes);close(owner,copy,a,original);
  let(observer,a,b)=heap(||Arc::new(crate::default_dag_document()));let original=a-b;let nodes=observer.nodes.as_ptr();let alias=Arc::clone(&observer);let factory=DagSnapshotRetirementFactory;let bytes=factory.retirement_birth_bytes(&alias);let(result,a,b)=heap(||factory.retire(alias,Grant{maximum_items:1,maximum_capacity_bytes:bytes,maximum_depth:1,..Default::default()}));let(mut owner,_)=result.unwrap();assert_eq!((a,b),(bytes,0));let born=a;
  let demand=semio_framework_value::factory_ticket_demands(&owner,copy).unwrap();let grant=Grant{maximum_items:1,maximum_copy_bytes:copy.max(demand.copy_bytes),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let(step,a,b)=heap(||owner.close_step(grant).unwrap());assert_eq!(step.progress(),Progress::default());assert_eq!((a,b),(0,0));assert!(!owner.terminal_is_empty());assert_eq!(Arc::strong_count(&observer),2);assert_eq!(observer.nodes.as_ptr(),nodes);println!("[DEBUG] Original DAG shared aliases=2 retains same backing with zero physical release");let(_,a,b)=heap(||drop(observer));assert_eq!((a,b),(0,0));close(owner,copy,born,original);
  for index in 0..laws["retirement"]["mutations"].as_u64().unwrap(){
   let(mutation,a,b)=heap(||mutation(index));let original=a-b;let factory=DagMutationRetirementFactory;let bytes=factory.retirement_birth_bytes(&mutation);let(result,a,b)=heap(||factory.retire_owned(mutation,Grant{maximum_items:1,maximum_capacity_bytes:bytes,maximum_depth:1,..Default::default()}));let(owner,_)=result.unwrap();assert_eq!((a,b),(bytes,0));close(owner,copy,a,original);
  }
 }
}

#[test]
fn dag_opens_as_an_owned_member_through_its_own_pack_codec(){
 assert_eq!(std::any::type_name::<<DagSnapshot as MemberStoreOwner<DagMutation>>::SnapshotOpen>(),std::any::type_name::<crate::os_store::PackMemberSnapshotOpen<DagSnapshot>>());
 let snapshot=crate::default_dag_document();let encoded=crate::os_store::ArtifactPack::encode_pack(&snapshot);let decoded=<DagSnapshot as crate::os_store::ArtifactPack>::decode_pack(&encoded).unwrap();assert_eq!(decoded.nodes,snapshot.nodes);assert!(<DagSnapshot as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported());
 let(mut owner,born,released)=heap(||semio_framework_value::retirement::controlled::ControlledRetirement::new(decoded).unwrap());assert_eq!((born,released),(0,0));let grant=Grant{maximum_items:0,maximum_depth:1,..Default::default()};let(step,a,b)=heap(||owner.step(grant).unwrap());assert_eq!(step.progress(),Progress::default());assert_eq!((a,b),(0,0));
 for turn in 0..1_000_000{let copy=owner.next_copy_byte_demand().unwrap().max(7);let grant=Grant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy.max(owner.next_copy_byte_demand().unwrap())).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};owner.step(grant).unwrap();if owner.terminal_is_empty(){return;}assert!(turn<999_999);}
}

fn mutation(index:u64)->DagMutation{
 use crate::*;
 let text=||"!@/\0引用😀".to_owned();
 match index{
  0=>DagMutation::CreateNode(CreateNode{node:DagNodeSpec{id:text(),x:f64::from_bits(0x8000000000000000),..Default::default()},index:u64::MAX}),
  1=>DagMutation::DeleteNode(DeleteNode{id:text()}),
  2=>DagMutation::RenameNode(RenameNode{id:text(),new_id:text()}),
  3=>DagMutation::ChangeNodeName(ChangeNodeName{id:text(),new_name:text()}),
  4=>DagMutation::MoveNode(MoveNode{id:text(),x:f64::from_bits(0x8000000000000000),y:f64::INFINITY}),
  5=>DagMutation::ResizeNode(ResizeNode{id:text(),width:f64::NAN,height:f64::NEG_INFINITY}),
  6=>DagMutation::ChangeNodeIcon(ChangeNodeIcon{id:text(),new_icon:text()}),
  7=>DagMutation::ChangeNodeAbbreviation(ChangeNodeAbbreviation{id:text(),new_abbreviation:text()}),
  8=>DagMutation::ChangeNodeOperatorKind(ChangeNodeOperatorKind{id:text(),new_operator_kind:Some(text())}),
  9=>DagMutation::ReplaceNodeKind(ReplaceNodeKind{id:text(),new_kind:DagNodeKind::Note{text:text(),output:IoPortSpec::default()}}),
  10=>DagMutation::ReplaceNodeProperties(ReplaceNodeProperties{id:text(),new_properties:graph::manifest::PropertyBag::from([(text(),graph::manifest::PropertyValue::Object(graph::manifest::PropertyBag::from([(text(),graph::manifest::PropertyValue::String(text()))])))])}),
  11=>DagMutation::ReorderNodes(ReorderNodes{order:vec![text(),String::with_capacity(128),text()]}),
  12=>DagMutation::ConnectNodes(ConnectNodes{id:text(),source:text(),target:text(),route_style:EdgeRouteStyle::SharpSz,properties:graph::manifest::PropertyBag::from([(text(),graph::manifest::PropertyValue::Number(f64::NAN))]),index:u64::MAX}),
  13=>DagMutation::DisconnectNodes(DisconnectNodes{id:text()}),
  _=>unreachable!(),
 }
}
