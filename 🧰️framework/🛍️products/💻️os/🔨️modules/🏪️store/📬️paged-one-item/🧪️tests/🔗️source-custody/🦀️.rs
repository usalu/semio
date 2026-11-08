//! 🧷️ Actual paged owners retain genuine source bindings and the exact original issuer pair.
use super::*;
use semio_os_store_one_item_batch::paged_one_item::{PagedOneItemEdit,PagedOneItemEditStep,PagedOneItemPreparationFactory};
use semio_framework_value_snapshot_projection::snapshot_projection::post::{PagedOneItemPostMode,PagedOneItemPostRead,PagedOneItemPost};
use semio_framework_value::{ArtifactOwnedValueRetirementFactory,SnapshotRetirementFactory,RetirementDemand,ErasedSnapshotRetirement};
use semio_framework_value::retained_clone::{RetainedCloneRef,RetainedCloneBinding,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};

thread_local! {
    static BEFORE:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};
    static PAYLOAD:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};
    static ISSUER:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};
    static POST:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};
    static SNAPSHOT_ISSUER:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};
    static TITLE_ROOT:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};
    static CALLS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};
    static SNAPSHOT_CALLS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};
    static EDIT_CALLS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};
    static REFUSE_EDIT:std::cell::Cell<bool>=const{std::cell::Cell::new(false)};
    static REFUSE_ISSUER:std::cell::Cell<bool>=const{std::cell::Cell::new(false)};
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct ExactMutationIssuer;
impl ArtifactOwnedValueRetirementFactory<DocumentMutation> for ExactMutationIssuer{fn retire_owned_work_demand(&self,_:&DocumentMutation)->Result<u64,semio_framework_value::ValueError>{Ok(1)}
    fn retirement_birth_bytes(&self,value:&DocumentMutation)->usize{OwnedValueRetirementFactory::<DocumentMutation>::default().retirement_birth_bytes(value)}
    fn retire_owned_demands(&self,value:&DocumentMutation)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{semio_framework_value::ArtifactOwnedValueRetirementFactory::<DocumentMutation>::retire_owned_demands(&OwnedValueRetirementFactory::<DocumentMutation>::default(),value)}
    fn retire_owned<'reply>(&self,value:&mut Option<DocumentMutation>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,mut reply:semio_framework_value::factory::FactoryProducerReply<'reply,dyn semio_framework_value::ErasedSnapshotRetirement,semio_framework_value::OriginalRetirementFailure>)->semio_framework_value::factory::FactoryProducerReplyComplete<'reply,semio_framework_value::OriginalRetirementFailure>{
        CALLS.set(CALLS.get()+1);
        assert_eq!(std::ptr::from_ref(self) as usize,ISSUER.get());
        let DocumentMutation::SetText{text,..}=value.as_ref().expect("original mutation resident") else{panic!("original mutation changed")};
        assert_eq!(text.as_ptr() as usize,PAYLOAD.get());
        if REFUSE_ISSUER.replace(false){let result=match reply.performed(effect,1){Ok(_)=>Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"exact original issuer refuses once").into()),Err(error)=>Err(error.into())};return reply.complete(semio_framework_value::RetainedWorkOutcome{result,work:1});}
        semio_framework_value::ArtifactOwnedValueRetirementFactory::<DocumentMutation>::retire_owned(&OwnedValueRetirementFactory::<DocumentMutation>::default(),value,effect,reply)
    }
    fn recover_retire_owned<'reply>(&self,value:&mut Option<DocumentMutation>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:semio_framework_value::factory::FactoryProducerReply<'reply,dyn semio_framework_value::ErasedSnapshotRetirement,semio_framework_value::OriginalRetirementFailure>)->semio_framework_value::factory::FactoryProducerReplyComplete<'reply,semio_framework_value::OriginalRetirementFailure>{semio_framework_value::ArtifactOwnedValueRetirementFactory::<DocumentMutation>::recover_retire_owned(&OwnedValueRetirementFactory::<DocumentMutation>::default(),value,effect,reply)}
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct ExactSnapshotIssuer;
impl SnapshotRetirementFactory<Document> for ExactSnapshotIssuer{
    fn retirement_birth_bytes(&self,value:&Arc<Document>)->usize{SharedValueRetirementFactory::<Document>::default().retirement_birth_bytes(value)}
    fn retire_demands(&self,value:&Arc<Document>)->Result<semio_framework_value::RetirementDemand,ValueError>{semio_framework_value::SnapshotRetirementFactory::<Document>::retire_demands(&SharedValueRetirementFactory::<Document>::default(),value)}
    fn retire_work_demand(&self,value:&Arc<Document>)->Result<u64,ValueError>{semio_framework_value::SnapshotRetirementFactory::<Document>::retire_work_demand(&SharedValueRetirementFactory::<Document>::default(),value)}
    fn retire<'reply>(&self,value:&mut Option<Arc<Document>>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:semio_framework_value::factory::FactoryProducerReply<'reply,dyn ErasedSnapshotRetirement,semio_framework_value::OriginalRetirementFailure>)->semio_framework_value::factory::FactoryProducerReplyComplete<'reply,semio_framework_value::OriginalRetirementFailure>{
        SNAPSHOT_CALLS.set(SNAPSHOT_CALLS.get()+1);
        assert_eq!(std::ptr::from_ref(self) as usize,SNAPSHOT_ISSUER.get());
        let resident=value.as_ref().expect("original snapshot resident");
        let identity=Arc::as_ptr(resident) as usize;if POST.get()==0{POST.set(identity);}else{assert_eq!(identity,POST.get());}
        assert_eq!(resident.title.identity(),TITLE_ROOT.get());
        semio_framework_value::SnapshotRetirementFactory::<Document>::retire(&SharedValueRetirementFactory::<Document>::default(),value,effect,reply)
    }
    fn recover_retire<'reply>(&self,value:&mut Option<Arc<Document>>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:semio_framework_value::factory::FactoryProducerReply<'reply,dyn ErasedSnapshotRetirement,semio_framework_value::OriginalRetirementFailure>)->semio_framework_value::factory::FactoryProducerReplyComplete<'reply,semio_framework_value::OriginalRetirementFailure>{semio_framework_value::SnapshotRetirementFactory::<Document>::recover_retire(&SharedValueRetirementFactory::<Document>::default(),value,effect,reply)}
}

#[derive(Default)]
struct SourceProbeEdit{before:Option<RetainedCloneBinding>,mutation:Option<RetainedCloneBinding>,post:Option<RetainedCloneBinding>,closing:bool,last_work:u64}
impl PagedOneItemEdit<Document,DocumentMutation> for SourceProbeEdit{
    const PREFIX:&'static str="original-source";
    fn recognizes(_:&DocumentMutation)->bool{true}
    fn preflight(_:&DocumentMutation)->Result<usize,String>{Ok(1)}
    fn post_mode(&self)->PagedOneItemPostMode{PagedOneItemPostMode::Original}
    fn advance_demands(&self,before:RetainedCloneRef<'_,Document>,mutation:RetainedCloneRef<'_,DocumentMutation>,post:PagedOneItemPostRead<'_,Document>,_:usize,_maximum_work_units:u64)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:if self.before.is_none(){before.binding_copy_bytes()}else if self.mutation.is_none(){mutation.binding_copy_bytes()}else if self.post.is_none(){post.original()?.binding_copy_bytes()}else{0},depth:1,..Default::default()})}
    fn advance(&mut self,before:RetainedCloneRef<'_,Document>,mutation:RetainedCloneRef<'_,DocumentMutation>,post:PagedOneItemPost<'_,Document>,body:usize,grant:RetainedCloneGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<PagedOneItemEditStep<DocumentMutation>,semio_framework_value::OriginalReceivingPrimary>{
        self.last_work=0;
        let post=post.original()?;
        assert_ne!(post.source_identity(),before.source_identity());
        assert_eq!(post.get().title.identity(),TITLE_ROOT.get());
        if grant.maximum_items==0{return Ok(PagedOneItemEditStep::Progress(Default::default()));}
        EDIT_CALLS.set(EDIT_CALLS.get()+1);
        assert_eq!(before.get() as *const Document as usize,BEFORE.get());
        let DocumentMutation::SetText{text,..}=mutation.get() else{panic!("genuine original mutation variant changed")};
        assert_eq!(text.as_ptr() as usize,PAYLOAD.get());
        let turn=effect::with_effect(grant,effect::CPU,|e|before.bind(&mut self.before,e)).0;self.last_work+=u64::try_from(turn.work).unwrap_or(u64::MAX);if let Some(progress)=turn.result?{return Ok(PagedOneItemEditStep::Progress(progress));}
        let turn=effect::with_effect(grant,effect::CPU,|e|mutation.bind(&mut self.mutation,e)).0;self.last_work+=u64::try_from(turn.work).unwrap_or(u64::MAX);if let Some(progress)=turn.result?{return Ok(PagedOneItemEditStep::Progress(progress));}
        let turn=effect::with_effect(grant,effect::CPU,|e|post.bind(&mut self.post,e)).0;self.last_work+=u64::try_from(turn.work).unwrap_or(u64::MAX);if let Some(progress)=turn.result?{return Ok(PagedOneItemEditStep::Progress(progress));}
        self.last_work+=1;
        Ok(if REFUSE_EDIT.get(){PagedOneItemEditStep::RefusedOutcome(RetainedCloneProgress{copied_items:1,..Default::default()},Vec::new())}else{PagedOneItemEditStep::CompleteRows(RetainedCloneProgress{copied_items:1,..Default::default()},Vec::new())})
    }
    fn last_turn_work(&self)->u64{self.last_work}
    fn has_pending_receiving_recovery(&self)->bool{false}
    fn pending_receiving_demand(&self)->Option<RetirementDemand>{None}
    fn pending_receiving_recovery(&mut self)->Option<semio_framework_value::native_decoding::construction::NativeRecoveryLoan<'_>>{None}
    fn finish_receiving_recovery(&mut self,_:&mut semio_framework_value::native_decoding::NativeRetirementDecodeControl<'_,'_>)->semio_framework_value::RetainedWorkOutcome<RetainedCloneProgress,semio_framework_value::OriginalReceivingPrimary>{semio_framework_value::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"source probe never retains a pending receiving frame").into()),work:0}}
    fn begin_close(&mut self){self.closing=true;}
    fn close_demands(&self,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<DocumentMutation>>,body:usize)->Result<RetirementDemand,ValueError>{
        assert_eq!(Arc::as_ptr(factory).cast::<()>() as usize,ISSUER.get());
        let binding=if self.before.is_some(){&self.before}else if self.mutation.is_some(){&self.mutation}else{&self.post};
        Ok(RetirementDemand{copy_bytes:RetainedCloneBinding::copy_demand(binding)?,capacity_bytes:RetainedCloneBinding::capacity_demand(binding,body)?,release_bytes:RetainedCloneBinding::release_demand(binding)?,depth:RetainedCloneBinding::depth_demand(binding)?})
    }
    fn close_work_demand(&self,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<DocumentMutation>>,body:usize)->Result<u64,ValueError>{
        self.close_demands(factory,body)?;let binding=if self.before.is_some(){&self.before}else if self.mutation.is_some(){&self.mutation}else{&self.post};RetainedCloneBinding::close_work_demand(binding,body)
    }
    fn close_step(&mut self,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<DocumentMutation>>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll{
        use semio_framework_value::RetirementPollExit;
        let result=(||->Result<RetainedCloneStep,RetirementPollExit>{
            assert!(self.closing);let grant=effect.grant();self.close_demands(factory,grant.maximum_copy_bytes)?;
            let step=RetirementPollExit::resume(semio_framework_value::retained_clone::close_retained_binding(if self.before.is_some(){&mut self.before}else if self.mutation.is_some(){&mut self.mutation}else{&mut self.post},effect))?;self.last_work=u64::try_from(effect.performed_work()).unwrap_or(u64::MAX);
            Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
        })();
        RetirementPollExit::settle(effect,result)
    }
    fn terminal_is_empty(&self)->bool{self.closing&&self.before.is_none()&&self.mutation.is_none()&&self.post.is_none()}
}

fn returned_step(result:Result<Option<RetainedCloneStep>,semio_framework_value::RetirementPollExit>)->Option<RetainedCloneStep>{match result{Ok(step)=>step,Err(semio_framework_value::RetirementPollExit::Failed(failure))=>panic!("returned custody close failed: {failure:?}"),Err(semio_framework_value::RetirementPollExit::Pending(_))=>panic!("returned custody close stayed pending")}}
fn law()->Value{serde_json::from_str(include_str!("../🧫️fixtures/🔗️source-custody/🔣️.json")).unwrap()}
fn policy(law:&Value)->ArtifactStoreOneItemGrant{ArtifactStoreOneItemGrant{maximum_items:1,maximum_copy_bytes:law["copyBytes"].as_u64().unwrap() as usize,maximum_capacity_bytes:law["capacityBytes"].as_u64().unwrap() as usize,maximum_release_bytes:law["releaseBytes"].as_u64().unwrap() as usize,maximum_depth:law["depth"].as_u64().unwrap() as usize}}

fn original_owner(law:&Value)->(Box<dyn ArtifactStoreOneItemPreparation<Document,DocumentMutation>>,Arc<Document>,Arc<dyn ArtifactOwnedValueRetirementFactory<DocumentMutation>>,Arc<dyn SnapshotRetirementFactory<Document>>){
    let root=Arc::new(document(&law["base"]));BEFORE.set(Arc::as_ptr(&root) as usize);TITLE_ROOT.set(root.title.identity());POST.set(0);
    let original=mutation(&law["mutation"]);let DocumentMutation::SetText{text,..}=&original else{unreachable!()};PAYLOAD.set(text.as_ptr() as usize);
    let mutations:Arc<dyn ArtifactOwnedValueRetirementFactory<DocumentMutation>>=Arc::new(ExactMutationIssuer);ISSUER.set(Arc::as_ptr(&mutations).cast::<()>() as usize);
    let snapshots:Arc<dyn SnapshotRetirementFactory<Document>>=Arc::new(ExactSnapshotIssuer);SNAPSHOT_ISSUER.set(Arc::as_ptr(&snapshots).cast::<()>() as usize);
    let registry=SnapshotReadRegistryHandle::new();let lease=registry.try_issue(Arc::clone(&root)).unwrap_or_else(|_|panic!("original source lease"));
    let authority=Arc::new(ArtifactStoreOneItemLiveAuthority::mint(semio_framework_job::OperationId(1),semio_framework_job::Generation(1),[0;32],0,1,semio_os_spr::HybridLogicalTimestamp::new(1,0),"actor".into(),None,None,None));
    let request=ArtifactStoreOneItemPreparationRequest{operation:semio_framework_job::OperationId(1),generation:semio_framework_job::Generation(1),base_revision:[0;32],lane:HistoryLane::Document,authority,base:SnapshotRead::new(Arc::clone(&root),lease),mutation:original,mutation_retirement:Arc::clone(&mutations),snapshot_retirement:Arc::clone(&snapshots)};
    let issuer=PagedOneItemPreparationFactory::<Document,DocumentMutation,SourceProbeEdit>::default();
    let(owner,receipt)=issuer.begin(request,policy(law)).unwrap_or_else(|_|panic!("original source owner birth"));assert!(receipt.fits(policy(law).retained_grant()));
    (owner,root,mutations,snapshots)
}

fn close_exact<S:'static,M:'static>(owner:&mut Box<dyn ArtifactStoreOneItemPreparation<S,M>>,maximum:usize){
    owner.begin_close();
    let mut last=None;
    for _ in 0..maximum{
        if owner.terminal_is_empty(){return;}
        let copy=owner.next_close_copy_byte_demand().unwrap();
        let grant=ArtifactStoreOneItemGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_close_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_close_release_byte_demand().unwrap(),maximum_depth:owner.next_close_depth_demand().unwrap()};
        match effect::settled(effect::with_effect(grant.retained_grant(),effect::CPU,|effect|owner.close_step(effect)).0).result{Ok(step)=>{last=Some((grant,step.progress()));assert!(step.progress().fits(grant.retained_grant()));},Err(error)=>{assert_eq!(error.primary().or(error.work_refusal()).map(|cause|cause.kind),Some(ValueRefusalKind::OwnershipLimit));assert!(error.retained_progress().fits(grant.retained_grant()));}}
    }
    panic!("original mutation source retained owners past exact close budget: {last:?}, checkpoint {:?}",owner.checkpoint());
}

fn original_cold_owner<S>(root:&Arc<S>,mutation:DocumentMutation,grant:ArtifactStoreOneItemGrant)->(Box<dyn ArtifactStoreOneItemPreparation<S,DocumentMutation>>,Arc<dyn ArtifactOwnedValueRetirementFactory<DocumentMutation>>,Arc<dyn SnapshotRetirementFactory<S>>)
where S:Clone+semio_framework_value::retirement::RetireOwned+Send+Sync+'static,DocumentMutation:ColdMutation<S>{
    let mutations:Arc<dyn ArtifactOwnedValueRetirementFactory<DocumentMutation>>=Arc::new(OwnedValueRetirementFactory::<DocumentMutation>::default());
    let snapshots:Arc<dyn SnapshotRetirementFactory<S>>=Arc::new(SharedValueRetirementFactory::<S>::default());
    let registry=SnapshotReadRegistryHandle::new();let lease=registry.try_issue(Arc::clone(root)).unwrap_or_else(|_|panic!("cold original source lease"));
    let authority=Arc::new(ArtifactStoreOneItemLiveAuthority::mint(semio_framework_job::OperationId(1),semio_framework_job::Generation(1),[0;32],0,1,semio_os_spr::HybridLogicalTimestamp::new(1,0),"actor".into(),None,None,None));
    let request=ArtifactStoreOneItemPreparationRequest{operation:semio_framework_job::OperationId(1),generation:semio_framework_job::Generation(1),base_revision:[0;32],lane:HistoryLane::Document,authority,base:SnapshotRead::new(Arc::clone(root),lease),mutation,mutation_retirement:Arc::clone(&mutations),snapshot_retirement:Arc::clone(&snapshots)};
    let factory=paged::ColdMutationApplyFactory::<S,DocumentMutation>::default();
    let(owner,_)=ArtifactStoreOneItemPreparationFactory::begin(&factory,request,grant).unwrap_or_else(|_|panic!("cold original owner birth"));
    (owner,mutations,snapshots)
}

#[test]
fn paged_original_cold_refusal_reports_actual_work_and_resets_on_denial_and_cancellation(){
    let law=law();let grant=policy(&law);let index=law["coldRefusalIndex"].as_u64().unwrap() as usize;
    let mut oracle=std::process::Command::new("bun").args(["-e",r#"import {Database} from "bun:sqlite";const law=JSON.parse(await Bun.stdin.text());const db=new Database(":memory:");db.exec("create table nodes(id text primary key)");for(const row of law.base.nodes)db.query("insert into nodes values (?)").run(row.id);console.log(JSON.stringify(db.query("select ? > count(*) as refused from nodes").get(law.coldRefusalIndex).refused===1));"#]).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn().unwrap();
    use std::io::Write;oracle.stdin.take().unwrap().write_all(law.to_string().as_bytes()).unwrap();let result=oracle.wait_with_output().unwrap();assert!(result.status.success());assert_eq!(serde_json::from_slice::<Value>(&result.stdout).unwrap(),law["expected"]["coldRefused"]);
    let root=Arc::new(law["base"]["nodes"].as_array().unwrap().len() as i64);
    let(mut owner,mutations,snapshots)=original_cold_owner(&root,DocumentMutation::Insert{index,id:String::new(),text:String::new()},grant);
    let mut receiver=Receiver::new(grant.retained_grant());
    INVERSE_CALLS.set(0);DIFF_CALLS.set(0);DIFF_RETIRES.set(0);
    let error=receiver.run(grant.retained_grant(),|control|owner.advance(grant.maximum_copy_bytes,grant,control)).unwrap_err();
    assert_eq!(error.kind,ValueRefusalKind::InvalidValue,"{error:?}");assert_eq!(error.retained_progress(),RetainedCloneProgress::default());assert_eq!(owner.last_turn_work(),(DIFF_CALLS.get()+INVERSE_CALLS.get()+DIFF_RETIRES.get()) as u64);assert_eq!(owner.last_turn_work(),law["expected"]["coldRefusalWork"].as_u64().unwrap());
    let denied=ArtifactStoreOneItemGrant{maximum_items:0,..grant};
    assert!(matches!(receiver.run(grant.retained_grant(),|control|owner.advance(grant.maximum_copy_bytes,denied,control)).unwrap(),ArtifactStoreOneItemPreparationStep::Blocked));assert_eq!(owner.last_turn_work(),law["expected"]["deniedWork"].as_u64().unwrap());
    owner.cancel();assert!(matches!(receiver.run(grant.retained_grant(),|control|owner.advance(grant.maximum_copy_bytes,grant,control)).unwrap(),ArtifactStoreOneItemPreparationStep::Blocked));assert_eq!(owner.last_turn_work(),law["expected"]["cancelledWork"].as_u64().unwrap());
    close_exact(&mut owner,law["maximumTurns"].as_u64().unwrap() as usize);drop(owner);receiver.finish();
    drain_fixture(root);drain_fixture(semio_framework_value::FactoryAuthority::new(mutations));drain_fixture(semio_framework_value::FactoryAuthority::new(snapshots));
}

#[test]
fn cold_publication_semantics_preserve_diagnostics_and_refuse_before_inverse_and_apply(){
    let scope=law();let grant=policy(&scope);let maximum=scope["maximumTurns"].as_u64().unwrap() as usize;
    let admission:Value=serde_json::from_str(include_str!("../../🧬️mutation-apply/🧪️tests/🧫️fixtures/🚦️admission/🔣️.json")).unwrap();
    let mut oracle=std::process::Command::new("bun").args(["-e",r#"import {Database} from "bun:sqlite";const law=JSON.parse(await Bun.stdin.text());const db=new Database(":memory:");console.log(JSON.stringify(law.cases.map(row=>db.query("select ? in ('info','warning') as admitted").get(row.level).admitted===1)));"#]).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn().unwrap();
    use std::io::Write;oracle.stdin.take().unwrap().write_all(admission.to_string().as_bytes()).unwrap();let result=oracle.wait_with_output().unwrap();assert!(result.status.success());assert_eq!(serde_json::from_slice::<Vec<bool>>(&result.stdout).unwrap(),admission["cases"].as_array().unwrap().iter().map(|case|case["applied"].as_bool().unwrap()).collect::<Vec<_>>());
    for case in admission["cases"].as_array().unwrap(){
        let root=Arc::new(plain_document_json(&scope["base"]));
        let mutation=DocumentMutation::Diagnostic{level:case["level"].as_str().unwrap().into(),code:case["code"].as_str().unwrap().into(),message:case["message"].as_str().unwrap().into()};
        let(mut owner,mutations,snapshots)=original_cold_owner(&root,mutation,grant);let mut receiver=Receiver::new(grant.retained_grant());let mut prepared=None;let mut refused=false;
        APPLY_CALLS.set(0);INVERSE_CALLS.set(0);DIFF_CALLS.set(0);DIFF_RETIRES.set(0);
        for _ in 0..maximum{
            match receiver.run(grant.retained_grant(),|control|owner.advance(grant.maximum_copy_bytes,grant,control)){
                Ok(ArtifactStoreOneItemPreparationStep::Prepared(..))=>{prepared=owner.take_prepared();break;},
                Ok(ArtifactStoreOneItemPreparationStep::Progress(..))=>{},
                Ok(ArtifactStoreOneItemPreparationStep::Blocked)=>panic!("funded cold publication stalled: checkpoint={:?}, work={}, grant={grant:?}",owner.checkpoint(),owner.last_turn_work()),
                Err(_)=>{refused=true;break;},
            }
        }
        let applied=prepared.is_some();let calls=(DIFF_CALLS.get(),INVERSE_CALLS.get(),APPLY_CALLS.get(),DIFF_RETIRES.get());
        let messages=prepared.as_ref().map_or_else(||owner.outcome_messages(),|value|value.outcome_messages()).iter().map(|message|serde_json::from_str::<Value>(&semio_framework_pack_json::to_json_string(message)).unwrap()).collect::<Vec<_>>();
        if let Some(prepared)=prepared{
            let(mut retirement,_)=prepared.admit_retirement(Arc::clone(&mutations),Arc::clone(&snapshots),grant.retained_grant()).unwrap_or_else(|_|panic!("cold diagnostic candidate retirement"));
            for _ in 0..maximum{if retirement.terminal_is_empty(){break;}effect::settle(grant.retained_grant(),|e|retirement.close_step(e)).result.unwrap();}assert!(retirement.terminal_is_empty());
        }
        close_exact(&mut owner,maximum);drop(owner);receiver.finish();
        assert_eq!(serde_json::to_value(root.as_ref()).unwrap(),scope["base"]);drain_fixture(root);drain_fixture(semio_framework_value::FactoryAuthority::new(mutations));drain_fixture(semio_framework_value::FactoryAuthority::new(snapshots));
        assert_eq!(applied,case["applied"].as_bool().unwrap());assert_eq!(refused,!applied);
        assert_eq!(calls,(1,case["inverseCalls"].as_u64().unwrap() as usize,case["applyCalls"].as_u64().unwrap() as usize,1));
        let mut expected=case.as_object().unwrap().clone();for key in ["applied","post","applyCalls","inverseCalls"]{expected.remove(key);}assert_eq!(messages,vec![Value::Object(expected)]);
    }
}

#[test]
fn paged_original_mutation_sources_keep_genuine_refs_and_exact_issuer_through_all_outcomes(){
 let mut _original_receiving_callback=|_:semio_framework_value_native_owned::native_decoding::NativeDecodeProgress|true;
 let mut _original_receiving=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(1usize<<30,&mut _original_receiving_callback);
    let law=law();let maximum=law["maximumTurns"].as_u64().unwrap() as usize;
    let schema:Value=serde_json::from_str(include_str!("../../🧬️schema/🔗️source-custody/🔣️.json")).unwrap();
    let mut oracle=std::process::Command::new("bun").args(["-e",r#"import Ajv from "ajv";import {Database} from "bun:sqlite";const {schema,law}=JSON.parse(await Bun.stdin.text());const validate=new Ajv({strict:true}).compile(schema);if(!validate(law)||validate({...law,cases:[]}))throw Error(JSON.stringify(validate.errors));const db=new Database(":memory:");const rows=law.base.nodes.map(row=>[row.id,row.text]);db.exec("create table nodes(id text primary key,payload text not null)");for(const row of rows)db.query("insert into nodes values (?,?)").run(...row);const original=db.query("select id,payload from nodes order by rowid").all();console.log(JSON.stringify(original.map(row=>({id:row.id,text:row.payload}))));"#]).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn().unwrap();
    use std::io::Write;oracle.stdin.take().unwrap().write_all(json!({"schema":schema,"law":law}).to_string().as_bytes()).unwrap();let result=oracle.wait_with_output().unwrap();assert!(result.status.success());let independent:Value=serde_json::from_slice(&result.stdout).unwrap();assert_eq!(independent,law["base"]["nodes"]);
    for row in law["cases"].as_array().unwrap(){
        CALLS.set(0);SNAPSHOT_CALLS.set(0);EDIT_CALLS.set(0);REFUSE_EDIT.set(row["refuseEdit"].as_bool().unwrap());REFUSE_ISSUER.set(row["refuseIssuerOnce"].as_bool().unwrap());
        let(mut owner,root,mutations,snapshots)=original_owner(&law);let mut receiver=Receiver::new(policy(&law).retained_grant());let mut prepared=None;let mut refused=false;
        for _ in 0..maximum{
            let calls=EDIT_CALLS.get();
            let(result,heap)=receiver.run(policy(&law).retained_grant(),|control|Ok::<_,ValueError>(observe(||owner.advance(policy(&law).maximum_copy_bytes,ArtifactStoreOneItemGrant{maximum_items:0,..policy(&law)},control)))).unwrap();
            assert!(matches!(result.unwrap(),ArtifactStoreOneItemPreparationStep::Blocked));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(EDIT_CALLS.get(),calls);
            assert_eq!(owner.last_turn_work(),law["expected"]["deniedWork"].as_u64().unwrap());
            match receiver.run(policy(&law).retained_grant(),|control|{let before=control.actual_work();let result=owner.advance(policy(&law).maximum_copy_bytes,policy(&law),control);assert_eq!(owner.last_turn_work()>=control.actual_work()-before,law["expected"]["reportedWorkCoversNative"].as_bool().unwrap());assert!(owner.last_turn_work()>0,"funded original preparation at {:?}: {result:?}",owner.checkpoint());result}){
                Ok(ArtifactStoreOneItemPreparationStep::Prepared(..))=>{prepared=owner.take_prepared();break;}
                Ok(ArtifactStoreOneItemPreparationStep::Progress(_,receipt))=>assert!(receipt.fits(policy(&law).retained_grant())),
                Ok(ArtifactStoreOneItemPreparationStep::Blocked)=>panic!("funded original source action stalled"),
                Err(error)=>{assert_eq!(error.kind,ValueRefusalKind::InvalidValue);refused=true;break;}
            }
        }
        assert_eq!(refused,row["refuseEdit"].as_bool().unwrap());assert_eq!(serde_json::to_value(root.as_ref()).unwrap(),law["base"]);
        if let Some(prepared)=prepared{
            POST.set(Arc::as_ptr(prepared.post_snapshot()) as usize);assert_eq!(serde_json::to_value(prepared.post_snapshot().as_ref()).unwrap(),law["base"]);
            let(mut ticket,receipt)=prepared.admit_retirement(Arc::clone(&mutations),Arc::clone(&snapshots),policy(&law).retained_grant()).unwrap_or_else(|_|panic!("exact source candidate retirement"));assert!(receipt.fits(policy(&law).retained_grant()));
            for _ in 0..maximum{if ticket.terminal_is_empty(){break;}let copy=ticket.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:ticket.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:ticket.next_release_byte_demand().unwrap(),maximum_depth:ticket.next_depth_demand().unwrap()};assert!(effect::settle(grant,|e|ticket.close_step(e)).result.unwrap().progress().fits(grant));}assert!(ticket.terminal_is_empty());
        }
        close_exact(&mut owner,maximum);assert_eq!(CALLS.get(),row["expectedMutationCalls"].as_u64().unwrap() as usize);assert_eq!(SNAPSHOT_CALLS.get(),row["expectedSnapshotCalls"].as_u64().unwrap() as usize);assert_eq!(observe(||drop(owner)).1.requested_bytes,0);receiver.finish();
        drain_fixture(root);drain_fixture(semio_framework_value::FactoryAuthority::new(mutations));drain_fixture(semio_framework_value::FactoryAuthority::new(snapshots));
    }
    for stop in law["cancelAt"].as_array().unwrap(){
        CALLS.set(0);SNAPSHOT_CALLS.set(0);EDIT_CALLS.set(0);REFUSE_EDIT.set(false);REFUSE_ISSUER.set(false);
        let(mut owner,root,mutations,snapshots)=original_owner(&law);let mut receiver=Receiver::new(policy(&law).retained_grant());
        for _ in 0..stop.as_u64().unwrap(){assert!(!matches!(receiver.run(policy(&law).retained_grant(),|control|owner.advance(policy(&law).maximum_copy_bytes,policy(&law),control)).unwrap(),ArtifactStoreOneItemPreparationStep::Prepared(..)));}
        owner.cancel();assert!(matches!(receiver.run(policy(&law).retained_grant(),|control|owner.advance(policy(&law).maximum_copy_bytes,policy(&law),control)).unwrap(),ArtifactStoreOneItemPreparationStep::Blocked));assert_eq!(owner.last_turn_work(),law["expected"]["cancelledWork"].as_u64().unwrap());close_exact(&mut owner,maximum);assert_eq!(CALLS.get(),1);assert_eq!(serde_json::to_value(root.as_ref()).unwrap(),law["base"]);assert_eq!(observe(||drop(owner)).1.requested_bytes,0);receiver.finish();
        drain_fixture(root);drain_fixture(semio_framework_value::FactoryAuthority::new(mutations));drain_fixture(semio_framework_value::FactoryAuthority::new(snapshots));
    }
}      
#[test]
fn paged_failed_seal_keeps_actual_returned_edit_post_and_exact_issuer_pair_until_funded_close(){
    use semio_os_store_one_item_batch::paged_one_item::test_support::{retain_seal_returns,returned_edit_demands,close_returned_edit,returned_post_demands,close_returned_post};
    let law=law();let maximum=law["maximumTurns"].as_u64().unwrap() as usize;
    CALLS.set(0);SNAPSHOT_CALLS.set(0);REFUSE_ISSUER.set(false);
    let authority=Arc::new(ArtifactStoreOneItemLiveAuthority::mint(semio_framework_job::OperationId(1),semio_framework_job::Generation(1),[0;32],0,1,semio_os_spr::HybridLogicalTimestamp::new(1,0),"actor".into(),None,None,None));
    let authority_address=Arc::as_ptr(&authority) as usize;
    let original=mutation(&law["mutation"]);let DocumentMutation::SetText{text,..}=&original else{unreachable!()};PAYLOAD.set(text.as_ptr() as usize);
    let edit=authority.next_edit(original,Vec::new());
    let forward_address=edit.forwards.as_ptr() as usize;
    let post=Arc::new(document(&law["base"]));POST.set(Arc::as_ptr(&post) as usize);TITLE_ROOT.set(post.title.identity());
    let mutations:Arc<dyn ArtifactOwnedValueRetirementFactory<DocumentMutation>>=Arc::new(ExactMutationIssuer);ISSUER.set(Arc::as_ptr(&mutations).cast::<()>() as usize);
    let snapshots:Arc<dyn SnapshotRetirementFactory<Document>>=Arc::new(ExactSnapshotIssuer);SNAPSHOT_ISSUER.set(Arc::as_ptr(&snapshots).cast::<()>() as usize);
    let snapshot_address=Arc::as_ptr(&snapshots).cast::<()>() as usize;
    let birth=semio_os_store_one_item_publication::lifted_canonical_edit::ArtifactStoreOneItemSealer::<Document,DocumentMutation>::constructor_demand();
    let denied=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:semio_os_store_one_item_publication::lifted_canonical_edit::ArtifactStoreOneItemSealer::<Document,DocumentMutation>::constructor_copy_byte_demand()-1,maximum_capacity_bytes:birth.capacity_bytes,maximum_depth:birth.depth,..Default::default()};
    let(result,heap)=observe(||authority.begin_one_item_seal(edit,post,mutations,snapshots,denied));
    let(error,authority,edit,post,mutations,snapshots)=match result{Err(returned)=>returned,Ok(_)=>panic!("one-below seal admission accepted")};
    assert_eq!(error.kind,ValueRefusalKind::WorkLimit);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    assert_eq!(Arc::as_ptr(&authority) as usize,authority_address);assert_eq!(edit.forwards.as_ptr() as usize,forward_address);assert_eq!(Arc::as_ptr(&post) as usize,POST.get());
    assert_eq!(Arc::as_ptr(&mutations).cast::<()>() as usize,ISSUER.get());assert_eq!(Arc::as_ptr(&snapshots).cast::<()>() as usize,snapshot_address);
    let(mut retained_authority,mut retained_edit,mut retained_post,mut retained_mutations,mut retained_snapshots)=(None,None,None,None,None);
    let(_,heap)=observe(||retain_seal_returns(&mut retained_authority,&mut retained_edit,&mut retained_post,&mut retained_mutations,&mut retained_snapshots,(authority,edit,post,mutations,snapshots)));
    assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    let mut edit_close=None;
    for _ in 0..maximum{
        if retained_edit.is_none()&&edit_close.is_none(){break;}
        let demand=returned_edit_demands(&retained_edit,&edit_close,0).unwrap();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
        let calls=CALLS.get();
        for axis in 0..5{
            if (axis==1&&grant.maximum_copy_bytes==0)||(axis==2&&grant.maximum_capacity_bytes==0)||(axis==3&&grant.maximum_release_bytes==0)||(axis==4&&grant.maximum_depth==0){continue;}
            let denied=match axis{0=>RetainedCloneGrant{maximum_items:0,..grant},1=>RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-1,..grant},2=>RetainedCloneGrant{maximum_capacity_bytes:grant.maximum_capacity_bytes-1,..grant},3=>RetainedCloneGrant{maximum_release_bytes:grant.maximum_release_bytes-1,..grant},_=>RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}};
            let((step,heap),_)=effect::with_effect(denied,effect::CPU,|e|observe(||close_returned_edit(&mut retained_edit,&mut edit_close,&mut retained_mutations,e)));
            assert_eq!(returned_step(step).unwrap().progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(CALLS.get(),calls);
            assert_eq!(returned_edit_demands(&retained_edit,&edit_close,0).unwrap(),demand);
        }
        assert!(returned_step(effect::with_effect(grant,effect::CPU,|e|close_returned_edit(&mut retained_edit,&mut edit_close,&mut retained_mutations,e)).0).unwrap().progress().fits(grant));
    }
    assert!(retained_edit.is_none()&&edit_close.is_none()&&retained_mutations.is_none());
    let demand=returned_post_demands(&retained_post,&retained_snapshots).unwrap();
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
    let mut post_close=None;
    let denied=RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant};
    let((step,heap),_)=effect::with_effect(denied,effect::CPU,|e|observe(||close_returned_post(&mut retained_post,&retained_snapshots,&mut post_close,e)));
    assert_eq!(returned_step(step).unwrap().progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(SNAPSHOT_CALLS.get(),0);assert_eq!(Arc::as_ptr(retained_post.as_ref().unwrap()) as usize,POST.get());
    assert!(returned_step(effect::with_effect(grant,effect::CPU,|e|close_returned_post(&mut retained_post,&retained_snapshots,&mut post_close,e)).0).unwrap().progress().fits(grant));
    for _ in 0..maximum{
        let Some(ticket)=post_close.as_ref()else{break;};
        let demand=semio_framework_value_snapshot_read::artifact_retirement_box_demands(ticket,4096).unwrap();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
        assert!(effect::settle(grant,|e|semio_framework_value_snapshot_read::artifact_retirement_box_close_step(&mut post_close,e)).result.unwrap().progress().fits(grant));
    }
    assert!(post_close.is_none()&&retained_post.is_none());assert_eq!(CALLS.get(),law["expected"]["failedSealMutationCalls"].as_u64().unwrap() as usize);assert_eq!(SNAPSHOT_CALLS.get(),law["expected"]["failedSealSnapshotCalls"].as_u64().unwrap() as usize);
    let mut factory_close=Some(semio_framework_value::FactoryAuthority::new(retained_snapshots.take().unwrap()));
    for _ in 0..maximum{
        let Some(owner)=factory_close.as_mut()else{break;};
        let demand=owner.demands(0).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
        assert!(effect::settle(grant,|e|owner.step(e)).result.unwrap().progress().fits(grant));if owner.terminal_is_empty(){factory_close=None;}
    }
    assert!(factory_close.is_none());
    let authority=retained_authority.take().unwrap();
    let mut authority_close=semio_framework_value::retirement::arc_custody::SharedControlledRetirement::new(authority);
    for _ in 0..maximum{
        if authority_close.terminal_is_empty(){break;}
        let copy=authority_close.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:authority_close.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:authority_close.next_release_byte_demand().unwrap(),maximum_depth:authority_close.next_depth_demand().unwrap()};
        assert!(effect::settle(grant,|e|authority_close.close_step(e)).result.unwrap().progress().fits(grant));
    }
    assert!(authority_close.terminal_is_empty());assert_eq!(observe(||drop((edit_close,post_close,authority_close,factory_close))).1.released_bytes,0);
}
