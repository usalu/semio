//! 🧵️ Original neutral DAG owners conserve physical custody under independently admitted currencies.
use super::*;
use semio_framework_value_testing_allocator::observe_retirement_allocations;
use semio_framework_value::{
    ArtifactOwnedValueRetirementFactory, AuthoringWallet, ErasedSnapshotRetirement,
    OriginalRetirementFailure, RetainedAllocationEffect, RetainedWorkOutcome, RetainedWorkPoll,
    RetirementDemand, SnapshotRetirementFactory, close_factory_ticket, close_owner,
    factory::FactoryProducerReply, factory_retire_snapshot, factory_ticket_demands,
    retained_clone::{OriginalEffectReply, RetainedCloneGrant as Grant, RetainedCloneProgress as Progress, RetainedCloneStep},
    retirement::{RetireOwned, arc_custody::original::SharedOriginalStorage},
};
use semio_os_infinite_directed_presentation::{DagNodeKind, EdgeRouteStyle, IoPortSpec};

fn heap<R>(operation: impl FnOnce() -> R) -> (R, usize, usize) {
    let (output, (allocated, released)) = observe_retirement_allocations(operation);
    (output, allocated, released)
}

fn grant(demand: RetirementDemand) -> Grant {
    Grant {
        maximum_items: 1,
        maximum_copy_bytes: demand.copy_bytes,
        maximum_capacity_bytes: demand.capacity_bytes,
        maximum_release_bytes: demand.release_bytes,
        maximum_depth: demand.depth,
    }
}

fn funded<R>(grant: Grant, work: u64, operation: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> R) -> (R, usize, usize) {
    AuthoringWallet::new(grant, u128::from(work))
        .run_invoked(grant, work, |effect| heap(|| operation(effect)))
        .unwrap().expect("fixture work is admitted").output
}

fn settled<T, E>(poll: RetainedWorkPoll<T, E>) -> Result<T, E> {
    match poll {
        RetainedWorkPoll::Complete(turn) => turn.result,
        RetainedWorkPoll::Pending { .. } => panic!("fixture owner did not settle synchronously"),
    }
}

type Ticket = Box<dyn ErasedSnapshotRetirement>;
type Birth = (Option<Ticket>, RetainedWorkOutcome<Progress, OriginalRetirementFailure>);

fn owned_birth<T: RetireOwned>(factory: &OwnedValueRetirementFactory<T>, value: &mut Option<T>, grant: Grant, work: u64) -> (Birth, usize, usize) {
    funded(grant, work, |effect| {
        let (mut output, mut receipt, mut attempted) = (None, None, None);
        let reply = FactoryProducerReply::borrow(&mut output, &mut receipt, &mut attempted).unwrap();
        drop(factory.retire_owned(value, effect, reply));
        (output, receipt.expect("stock owned factory records its actual reply"))
    })
}

fn shared_storage(snapshot: DagSnapshot) -> (SharedOriginalStorage<DagSnapshot>, usize) {
    let mut value = Some(snapshot);
    let mut storage = None;
    let offer = grant(SharedOriginalStorage::<DagSnapshot>::source_admission_demand().unwrap());
    let work = SharedOriginalStorage::<DagSnapshot>::source_admission_work_demand();
    let (turn, born, released) = funded(offer, work, |effect| SharedOriginalStorage::admit_from(&mut value, &mut storage, effect));
    assert!(value.is_none());
    assert_eq!((born, released), (turn.result.unwrap().progress().retained_capacity_bytes, 0));
    (storage.unwrap(), born)
}

fn shared_ticket(storage: SharedOriginalStorage<DagSnapshot>) -> (Ticket, usize) {
    let factory = SharedValueRetirementFactory::default();
    let bytes = factory.retirement_birth_bytes(&storage);
    let offer = grant(factory.retire_demands(&storage).unwrap());
    let work = factory.retire_work_demand(&storage).unwrap();
    let mut source = Some(storage);
    let (turn, born, released) = funded(offer, work, |effect| factory_retire_snapshot(&factory, &mut source, effect));
    let (ticket, receipt) = settled(turn).unwrap();
    assert!(source.is_none());
    assert_eq!((born, released), (bytes, 0));
    assert_eq!(receipt.retained_capacity_bytes, bytes);
    (ticket, born)
}

fn close(owner: Ticket, copy: usize, born: usize, original: usize) {
    let mut slot = Some(owner);
    let mut allocated = born;
    let mut released = 0;
    for _ in 0..1_000_000 {
        let owner = slot.as_ref().unwrap();
        let body = copy.max(owner.next_copy_byte_demand().unwrap());
        let (demand, a, b) = heap(|| factory_ticket_demands(owner, body).unwrap());
        assert_eq!((a, b), (0, 0));
        let offer = grant(RetirementDemand { copy_bytes: body.max(demand.copy_bytes), ..demand });
        let terminal = owner.terminal_is_empty();
        let work = if terminal { 1 } else { owner.next_computational_work(body).unwrap() };
        if terminal && demand.release_bytes > 0 {
            let denied = Grant { maximum_release_bytes: demand.release_bytes - 1, ..offer };
            let (turn, a, b) = funded(denied, work, |effect| close_factory_ticket(&mut slot, body, effect));
            assert_eq!(settled(turn).unwrap().progress(), Progress::default());
            assert_eq!((a, b), (0, 0));
            assert!(slot.is_some());
        }
        let (turn, a, b) = funded(offer, work, |effect| close_factory_ticket(&mut slot, body, effect));
        let receipt = settled(turn).unwrap().progress();
        assert!(receipt.fits(offer));
        assert_eq!((receipt.retained_capacity_bytes, receipt.released_bytes), (a, b));
        allocated += a;
        released += b;
        if slot.is_none() {
            assert_eq!(released, original + allocated);
            let (turn, a, b) = funded(Grant::default(), 0, |effect| close_factory_ticket(&mut slot, 0, effect));
            assert!(matches!(settled(turn).unwrap(), RetainedCloneStep::Complete(_)));
            assert_eq!((a, b), (0, 0));
            return;
        }
    }
    panic!("DAG retirement never reached actual terminal emptiness");
}

#[test]
fn neutral_fixture_retires_exact_mutation_shared_snapshot_and_final_snapshot_owners() {
    let source = include_str!("../../../🌿️vcs/🧪️tests/🧫️fixtures/🔣️mutations.json");
    let oracle: serde_json::Value = serde_json::from_str(source).unwrap();
    let parsed = semio_framework_pack_json::parse(source, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&parsed)).unwrap(), oracle);
    let laws: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();

    for copy in laws["retirement"]["copyBytes"].as_array().unwrap() {
        let copy = copy.as_u64().unwrap() as usize;
        let (snapshot, a, b) = heap(|| crate::sqlite_snapshot_tests::full(0x8000000000000000));
        let original = a - b;
        let pointer = snapshot.nodes.as_ptr();
        let factory = OwnedValueRetirementFactory::default();
        let bytes = factory.retirement_birth_bytes(&snapshot);
        let offer = grant(factory.retire_owned_demands(&snapshot).unwrap());
        let work = factory.retire_owned_work_demand(&snapshot).unwrap();
        let mut value = Some(snapshot);
        for denied in [
            Grant { maximum_items: 0, ..offer },
            Grant { maximum_capacity_bytes: bytes - 1, ..offer },
            Grant { maximum_depth: 0, ..offer },
        ] {
            let ((ticket, receipt), a, b) = owned_birth(&factory, &mut value, denied, work);
            assert!(ticket.is_none());
            assert_eq!(receipt.result.unwrap(), Progress::default());
            assert_eq!(receipt.work, 0);
            assert_eq!((a, b), (0, 0));
            assert_eq!(value.as_ref().unwrap().nodes.as_ptr(), pointer);
        }
        let ((ticket, receipt), a, b) = owned_birth(&factory, &mut value, offer, work);
        assert!(value.is_none());
        assert_eq!((a, b), (bytes, 0));
        assert_eq!(receipt.result.unwrap().retained_capacity_bytes, bytes);
        close(ticket.unwrap(), copy, a, original);

        let (snapshot, a, b) = heap(|| crate::sqlite_snapshot_tests::full(0x8000000000000000));
        let (shared, backing) = shared_storage(snapshot);
        let (ticket, born) = shared_ticket(shared);
        close(ticket, copy, born, a - b + backing);

        let (snapshot, a, b) = heap(crate::default_dag_document);
        let (mut observer, backing) = shared_storage(snapshot);
        let original = a - b + backing;
        let nodes = observer.get().nodes.as_ptr();
        let mut alias = None;
        let duplicate = grant(SharedOriginalStorage::<DagSnapshot>::duplicate_demand());
        let work = observer.duplicate_work_demand().unwrap();
        let (turn, a, b) = funded(duplicate, work, |effect| observer.duplicate_into(&mut alias, effect));
        assert!(turn.result.unwrap().progress().fits(duplicate));
        assert_eq!((a, b), (0, 0));
        let (owner, born) = shared_ticket(alias.unwrap());
        let body = copy.max(owner.next_copy_byte_demand().unwrap());
        let demand = factory_ticket_demands(&owner, body).unwrap();
        let work = owner.next_computational_work(body).unwrap();
        let denied = Grant { maximum_copy_bytes: demand.copy_bytes - 1, ..grant(demand) };
        let mut slot = Some(owner);
        let (turn, a, b) = funded(denied, work, |effect| close_factory_ticket(&mut slot, body, effect));
        assert_eq!(settled(turn).unwrap().progress(), Progress::default());
        assert_eq!((a, b), (0, 0));
        assert!(!slot.as_ref().unwrap().terminal_is_empty());
        assert_eq!(observer.aliases(), 2);
        assert_eq!(observer.get().nodes.as_ptr(), nodes);
        let offer = grant(observer.retire_into_demand().unwrap());
        let work = observer.retire_work_demand().unwrap();
        let mut elected = None;
        let (receipt, a, b) = funded(offer, work, |effect| {
            let mut receipt = None;
            let reply = OriginalEffectReply::borrow(&mut receipt).unwrap();
            drop(observer.retire_into::<ValueError>(&mut elected, effect, reply).unwrap());
            receipt.expect("observer alias retires in its original reply").unwrap()
        });
        assert!(receipt.progress().fits(offer));
        assert!(observer.terminal_is_empty());
        assert!(elected.is_none());
        assert_eq!((a, b), (0, 0));
        close(slot.unwrap(), copy, born, original);

        for index in 0..laws["retirement"]["mutations"].as_u64().unwrap() {
            let (mutation, a, b) = heap(|| mutation(index));
            let original = a - b;
            let factory = OwnedValueRetirementFactory::default();
            let bytes = factory.retirement_birth_bytes(&mutation);
            let offer = grant(factory.retire_owned_demands(&mutation).unwrap());
            let work = factory.retire_owned_work_demand(&mutation).unwrap();
            let mut value = Some(mutation);
            let ((ticket, receipt), a, b) = owned_birth(&factory, &mut value, offer, work);
            assert!(value.is_none());
            assert_eq!((a, b), (bytes, 0));
            assert_eq!(receipt.result.unwrap().retained_capacity_bytes, bytes);
            close(ticket.unwrap(), copy, a, original);
        }
    }
}

#[test]
fn dag_opens_as_an_owned_member_through_its_own_pack_codec() {
    assert_eq!(
        std::any::type_name::<<crate::DagMemberStoreProducer as MemberStoreProducer<DagSnapshot, DagMutation>>::SnapshotOpen>(),
        std::any::type_name::<crate::os_store::OriginalRecordMemberSnapshotOpen<DagSnapshot>>(),
    );
    let snapshot = crate::default_dag_document();
    let encoded = pack::snapshot::ArtifactPack::encode_pack(&snapshot);
    let decoded = <DagSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&encoded).unwrap();
    assert_eq!(decoded.nodes, snapshot.nodes);
    assert!(DagSnapshot::controlled_retirement_supported());
    let (mut owner, born, released) = heap(|| semio_framework_value::retirement::controlled::ControlledRetirement::new(decoded).unwrap());
    assert_eq!((born, released), (0, 0));

    let denied = Grant { maximum_items: 0, maximum_depth: 1, ..Default::default() };
    let work = owner.next_computational_work(0).unwrap();
    let (turn, a, b) = funded(denied, work, |effect| close_owner(&mut owner, 0, effect));
    assert_eq!(settled(turn).unwrap().progress(), Progress::default());
    assert_eq!((a, b), (0, 0));

    for _ in 0..1_000_000 {
        let body = owner.next_copy_byte_demand().unwrap().max(7);
        let demand = owner.next_demand(body).unwrap();
        let work = owner.next_computational_work(body).unwrap();
        let offer = grant(RetirementDemand { copy_bytes: body.max(demand.copy_bytes), ..demand });
        let (turn, _, _) = funded(offer, work, |effect| close_owner(&mut owner, body, effect));
        assert!(settled(turn).unwrap().progress().fits(offer));
        if owner.terminal_is_empty() { return; }
    }
    panic!("owned Dag snapshot did not retire within the fixture turn limit");
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
  10=>DagMutation::ReplaceNodeProperties(ReplaceNodeProperties{id:text(),new_properties:semio_framework_graph_manifest::PropertyBag::from([(text(),semio_framework_graph_manifest::PropertyValue::Object(semio_framework_graph_manifest::PropertyBag::from([(text(),semio_framework_graph_manifest::PropertyValue::String(text()))])))])}),
  11=>DagMutation::ReorderNodes(ReorderNodes{moves:vec![semio_framework_artifact_infinite_dag_vcs::NodeMove{id:text(),to:0},semio_framework_artifact_infinite_dag_vcs::NodeMove{id:String::with_capacity(128),to:1},semio_framework_artifact_infinite_dag_vcs::NodeMove{id:text(),to:2}]}),
  12=>DagMutation::ConnectNodes(ConnectNodes{id:text(),source:text(),target:text(),route_style:EdgeRouteStyle::SharpSz,properties:semio_framework_graph_manifest::PropertyBag::from([(text(),semio_framework_graph_manifest::PropertyValue::Number(f64::NAN))]),index:u64::MAX}),
  13=>DagMutation::DisconnectNodes(DisconnectNodes{id:text()}),
  _=>unreachable!(),
 }
}

fn every_kind_snapshot()->DagSnapshot{
 use semio_framework_graph_manifest::{PropertyBag,PropertyValue};
 use semio_os_infinite_directed_presentation::{DagExpandedPaths,DagMedia,DagMediaKind,DagPreviewContent,DagHostSnapshotEdge,DagNodeSpec};
 use semio_framework_value::DslValue;
 let port=|id:&str|IoPortSpec{id:id.into(),label:id.into(),default:Some(DslValue::Array(vec![DslValue::Bool(true),DslValue::String("x".into())])),value:Some(DslValue::Number(semio_framework_value::Number::Float(1.5))),cardinality:"*".into(),visible:false,..IoPortSpec::simple(id,id)};
 let bag=PropertyBag::from([("alpha".to_owned(),PropertyValue::Array(vec![PropertyValue::Null,PropertyValue::Bool(false),PropertyValue::Number(2.5),PropertyValue::String("s".into())])),("beta".to_owned(),PropertyValue::Object(PropertyBag::from([("inner".to_owned(),PropertyValue::String("v".into()))])))]);
 let kinds=vec![
  DagNodeKind::Computation{inputs:vec![port("a"),port("b")],outputs:vec![port("c")],variadic_inputs:true,variadic_outputs:false},
  DagNodeKind::Slider{min:0.0,max:9.0,step:1.0,value:3.0,output:port("o")},
  DagNodeKind::Select{options:vec!["x".into(),"y".into()],selected:1,output:port("o")},
  DagNodeKind::Screen{media:Some(DagMedia{kind:DagMediaKind::Svg,src:"m.svg".into()}),input:port("i")},
  DagNodeKind::Note{text:"hello".into(),output:port("o")},
  DagNodeKind::Image{src:"p.png".into(),output:port("o")},
  DagNodeKind::Preview{content:DagPreviewContent::Tree{json:DslValue::Object(vec![("k".into(),DslValue::Array(vec![DslValue::Null]))])},expanded:DagExpandedPaths::from(["/a".to_owned(),"/b".to_owned()]),input:port("i")},
  DagNodeKind::Action{label:"go".into(),input:port("i")},
  DagNodeKind::Export{label:"out".into(),format:"json".into(),input:port("i")},
  DagNodeKind::Cluster{inputs:vec![port("a")],outputs:vec![port("c")]},
  DagNodeKind::AppInstance{instance_id:"i1".into(),plugin_id:"p".into(),app_id:"a".into(),icon:"ic".into(),inputs:vec![port("a")],outputs:vec![]},
 ];
 let nodes=kinds.into_iter().enumerate().map(|(index,kind)|DagNodeSpec{id:format!("n{index}"),name:format!("Node{index}"),abbreviation:"Nd".into(),icon:"i".into(),x:index as f64,y:-1.5,operator_kind:(index%2==0).then(||"op".to_owned()),properties:if index==0{bag.clone()}else{PropertyBag::new()},kind,..Default::default()}).collect();
 DagSnapshot{schema:crate::DAG_DOCUMENT_SCHEMA.into(),nodes,edges:vec![DagHostSnapshotEdge{id:"e0".into(),source:"n0".into(),target:"n1".into(),properties:bag,..Default::default()}]}
}

#[test]
fn dag_member_receives_every_node_kind_natively_from_its_original_text_record(){
 use crate::os_store::{MemberOpenRequest,MemberSnapshotOpenOperation,MemberSnapshotOpenStep,OriginalRecordMemberSnapshotOpen};
 use crate::os_store::owned_schema_decode::{OwnedSchemaDecodeCredits,OwnedSchemaDecodePage,OwnedSchemaDecodePages,OWNED_SCHEMA_DECODE_PAGE_BYTES};
 use semio_framework_job::{OriginalJobDecodeOwner,OperationId,Generation,StepBudget,StepContext,root_cancel_token};
 let expected=every_kind_snapshot();let bytes=pack::snapshot::ArtifactPack::encode_pack(&expected);
 let mut frame=Vec::new();let mut length=bytes.len();loop{let byte=(length&127)as u8;length>>=7;frame.push(byte|if length==0{0}else{128});if length==0{break}}frame.extend_from_slice(&bytes);frame.push(0);
 let mut pages=OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits{maximum_pages:frame.len().div_ceil(OWNED_SCHEMA_DECODE_PAGE_BYTES),maximum_bytes:frame.len()}).unwrap();
 for page in frame.chunks(OWNED_SCHEMA_DECODE_PAGE_BYTES){pages.admit_page(OwnedSchemaDecodePage::try_from_slice(page).unwrap()).unwrap();}pages.seal().unwrap();
 let document=semio_framework_artifact_reference::ArtifactRef{artifact_id:"dag-original".into(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"dag.dag".into(),standard:"1".into(),subset:"*".into()}};
 let request=MemberOpenRequest::new(OperationId(91),Generation(7),1000,document,None,pages,semio_os_spr::ActorId("author".into())).admit(1).unwrap_or_else(|_|panic!("original DAG request"));
 let grant=Grant{maximum_items:1,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:256};
 let mut open=OriginalRecordMemberSnapshotOpen::<DagSnapshot>::begin(request).unwrap_or_else(|_|panic!("original DAG open"));
 let cancel=root_cancel_token();let mut native=OriginalJobDecodeOwner::empty();let mut sequence=0;let mut ready=false;
 for _ in 0..100000{let mut receipt=Default::default();let mut cx=StepContext::new(OperationId(91),Generation(7),StepBudget::new(1<<24,999,grant),cancel.clone(),||Some(1),&mut sequence,&mut receipt).with_original_decoder(&mut native);
  match open.step(&mut cx,grant).unwrap_or_else(|_|panic!("original DAG step")){MemberSnapshotOpenStep::Ready=>{ready=true;break},MemberSnapshotOpenStep::Pending(_)=>{},MemberSnapshotOpenStep::Rejected(diagnostic)=>panic!("original DAG rejected: {diagnostic:?}")}}
 assert!(ready);
 let mut receipt=Default::default();let mut cx=StepContext::new(OperationId(91),Generation(7),StepBudget::new(1<<24,999,grant),cancel.clone(),||Some(1),&mut sequence,&mut receipt);
 let(snapshot,mut original)=open.take_ready(&mut cx).unwrap_or_else(|_|panic!("original DAG take")).unwrap();
 assert_eq!(snapshot,expected);
 while !original.terminal_is_empty(){original.close_step(grant).unwrap();}
 for _ in 0..100000{if open.terminal_is_empty(){break}open.close_step(grant).unwrap();}
 assert!(open.terminal_is_empty());
 while !native.terminal_is_empty(){native.close_step(grant).unwrap();}
}
