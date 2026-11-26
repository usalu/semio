use crate::{FactoryRetirement,FactoryAuthority,factory_arc_birth_bytes,retained_clone::{RetainedCloneGrant,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,RetainedAllocationEffect,OriginalEffectReply,RetirementDemand}};
use semio_framework_value_clone_source_testing::{admit_owned,observe_fixture_source_allocations,FixtureCloseStep,FixtureRecoveryStep,FixtureSource};
use semio_framework_value_retirement::OriginalRetirementFailure as Failure;
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
use std::sync::Arc;
use semio_framework_value_factory::original_factory_fixture::*;
const MAXIMUM_WORK:u64=4096;
static DBG:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);
fn bank()->EffectBank{EffectBank::new(Wallet::new(RetainedCloneGrant{maximum_items:usize::MAX,maximum_copy_bytes:usize::MAX,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:usize::MAX},1<<62))}
fn funded(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
/// 🪢️ Runs one funded leaf through the fixture's real original bank and reports its progress and physical heap.
fn leaf(bank:&mut EffectBank,grant:RetainedCloneGrant,units:u64,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,Failure>)->(RetainedCloneStep,(usize,usize)){
    let((poll,actual),heap)=crate::observe_retirement_allocations(||bank.run(grant,units,|effect|{let before=effect.performed_work();let result=operation(effect);RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work:effect.performed_work()-before})}));
    let RetainedWorkPoll::Complete(turn)=poll else{panic!("factory source leaf retained an action")};let step=turn.result.unwrap();if let Some(actual)=actual{assert_eq!(actual,heap.0)}assert_ne!(bank.take_admitted(),Some(false));bank.post(step.progress(),grant,heap);(step,heap)
}
#[test]
fn factory_authority_keeps_original_typed_source_until_admitted_close_and_reports_all_physical_frees(){
    use crate::retained_clone::{RetainedClone,RetainedCloneCursor,RetainedCloneSource};
    let fixture:serde_json::Value=serde_json::from_str(crate::text("♻️retirement/🏭️factory/🔗️authority/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    use std::io::Write;let mut oracle=std::process::Command::new("bun").args(["-e","const x=JSON.parse(await Bun.stdin.text());console.log(JSON.stringify([...new TextEncoder().encode(x.fixture.payload)]));"]).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn().unwrap();oracle.stdin.take().unwrap().write_all(serde_json::json!({"fixture":fixture}).to_string().as_bytes()).unwrap();let output=oracle.wait_with_output().unwrap();assert!(output.status.success());let reference:Vec<u8>=serde_json::from_slice(&output.stdout).unwrap();assert_eq!(reference,fixture["payload"].as_str().unwrap().as_bytes());
    for work in fixture["copyGrants"].as_array().unwrap(){let work=work.as_u64().unwrap()as usize;
        let mut payload=Vec::with_capacity(fixture["payloadCapacity"].as_u64().unwrap()as usize);payload.extend_from_slice(&reference);let payload_capacity=payload.capacity();let factory=Arc::new(OwnedFactory{payload});let factory_extent=factory_arc_birth_bytes::<OwnedFactory>();
        let original=String::from("source");let original_bytes=original.capacity();
        let authority=FactoryAuthority::new(factory.clone());let pointer=Arc::as_ptr(&factory);let mut bank=bank();
        let (refused,heap)=observe_fixture_source_allocations(||{let mut recovery=admit_owned(&mut bank,original,authority,Default::default()).result.err().unwrap();for _ in 0..1000{match recovery.resume(crate::effect::STOCK){FixtureRecoveryStep::Complete=>break,FixtureRecoveryStep::Progress=>{},other=>panic!("factory admission handback {other:?}")}}let mut returned=None;assert!(matches!(recovery.finish_into(&mut returned,crate::effect::STOCK),FixtureRecoveryStep::Complete));returned.unwrap()});assert_eq!(heap,(0,0));let (_,original,authority)=refused;assert_eq!(authority.factory_address(),pointer as*const()as usize);
        let capacity=RetainedCloneSource::<String>::constructor_capacity_bytes::<FactoryAuthority>();let (admitted,heap)=observe_fixture_source_allocations(||admit_owned(&mut bank,original,authority,grant(capacity,0,1)).result);let(source,receipt)=admitted.unwrap_or_else(|_|panic!("source authority admission"));assert_eq!(heap,(receipt.retained_capacity_bytes,0));let mut born=receipt.retained_capacity_bytes;let mut physical=0;
        let mut source=FixtureSource::from_admitted(source,bank).unwrap_or_else(|_|panic!("admitted source returns an idle bank"));
        let mut cursor=String::retained_clone_cursor();let advance=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:work,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};
        source.with_bank(|original,bank|leaf(bank,advance,MAXIMUM_WORK,|effect|{let mut answer=None;let reply=OriginalEffectReply::borrow(&mut answer).map_err(Failure::from)?;drop(cursor.advance_into(original,work,MAXIMUM_WORK,effect,reply).map_err(Failure::from)?);answer.expect("string advance reply")})).unwrap();drop(factory);
        source.with_bank(|_,bank|{let start=funded(cursor.begin_close_demand().unwrap());let units=cursor.begin_close_work_demand().unwrap();leaf(bank,start,units,|effect|cursor.begin_close_paid(effect).result.map(RetainedCloneStep::Progress).map_err(Failure::from))}).unwrap();
        let mut pending=(0usize,0usize);
        for _ in 0..100000{
            if source.terminal_is_empty()&&cursor.terminal_is_empty(){break;}
            if !source.terminal_is_empty(){
                let d=source.next_close_demand(work).unwrap();let grant=funded(d);assert_eq!(source.next_close_demand(grant.maximum_copy_bytes).unwrap(),d);
                let(state,heap)=observe_fixture_source_allocations(||source.close_with(work,grant).map_err(|failure|format!("{failure:?}")));pending=(pending.0+heap.0,pending.1+heap.1);
                let p=match state.unwrap(){FixtureCloseStep::Pending=>continue,FixtureCloseStep::Progress(p)=>p,FixtureCloseStep::Complete=>Default::default(),other=>panic!("factory source close {other:?}")};
                if DBG.fetch_add(1,std::sync::atomic::Ordering::Relaxed)<60{eprintln!("[DEBUG] src close d={d:?} p={p:?}");}assert!(p.fits(grant));assert_eq!(pending,(p.retained_capacity_bytes,p.released_bytes));born+=pending.0;physical+=pending.1;pending=(0,0);
            }else{
                let d=cursor.close_demands(work).unwrap();let grant=funded(d);assert_eq!(cursor.close_demands(grant.maximum_copy_bytes).unwrap(),d);let units=cursor.close_work_demand(work).unwrap();
                let(step,heap)=source.with_bank(|_,bank|leaf(bank,grant,units,|effect|{let mut answer=None;let reply=OriginalEffectReply::borrow(&mut answer).map_err(Failure::from)?;drop(cursor.close_into(work,effect,reply).map_err(Failure::from)?);answer.expect("string close reply")})).unwrap();
                let p=step.progress();assert!(p.fits(grant));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));born+=heap.0;physical+=heap.1;
            }
        }
        assert!(source.terminal_is_empty()&&cursor.terminal_is_empty());assert_eq!(original_bytes+factory_extent+payload_capacity+born,physical);
    }
}
