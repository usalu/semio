use super::*;
use semio_framework_job::OriginalRetirementPump;
use semio_framework_value::{OriginalRetirementPoll, RetainedWorkPoll, retirement::{RetirementCursor,RetirementStep}};

fn effect_turn<T>(grant: RetainedCloneGrant, run: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> T) -> T {
    let mut turn = OriginalRetirementPump::new().turn(1 << 24, u64::MAX, || Some(0), grant).unwrap();
    turn.with_effect(run).unwrap().unwrap().0
}

fn receipt(poll: OriginalRetirementPoll) -> (RetainedCloneProgress, u128) {
    match poll {
        RetainedWorkPoll::Complete(turn) => (turn.result.unwrap().progress(), turn.work),
        RetainedWorkPoll::Pending { work } => (Default::default(), work),
    }
}

fn grant(demand: RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant {
        maximum_items: 1,
        maximum_copy_bytes: demand.copy_bytes,
        maximum_capacity_bytes: demand.capacity_bytes,
        maximum_release_bytes: demand.release_bytes,
        maximum_depth: demand.depth,
    }
}
#[test]
fn completion_final_header_close_reports_actual_work_separately_from_bytes(){
 let expected:serde_json::Value=serde_json::from_str(include_str!("../📐️.json")).unwrap();
 let mut cursor=Cursor{original:ManuallyDrop::new(Some(Source::Emit(Default::default(),Default::default()))),phase:18,faulted:false};
 let denied=effect_turn(Default::default(),|effect|cursor.close_step(0,effect));assert!(matches!(&denied,RetirementStep::BudgetExhausted));assert!(cursor.original.is_some());
 let clear=effect_turn(grant(cursor.next_demand(0).unwrap()),|effect|cursor.close_step(0,effect));assert!(cursor.terminal_is_empty());
 let cached=cursor.close_step(RetainedCloneGrant::default());
 let actual=serde_json::json!({"role":"completion","turns":[{"case":"denied","work":denied.work(),"items":0,"copyBytes":0},{"case":"clear","work":clear.work(),"items":1,"copyBytes":0},{"case":"cached","work":cached.work(),"items":0,"copyBytes":0}]});assert_eq!(actual,expected);assert!(matches!(clear,RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:0,..},1)));
}

#[test]
fn actual_drawing_completion_admission_posts_allocator_work_and_keeps_denied_input(){
 let profile:serde_json::Value=serde_json::from_str(include_str!("../📥️admit/📐️.json")).unwrap();let stock=&profile["stock"];
 let mut value=Some(Value::Emit(Ok(Default::default()),Default::default()));
 let denied=effect_turn(Default::default(),|effect|admit(&mut value,effect));assert_eq!(denied.work,u128::from(profile["work"]["denied"].as_u64().unwrap()));assert!(matches!(denied.result,Ok(None)));assert_eq!(value.is_some(),profile["inputRetainedOnDenial"].as_bool().unwrap());
 let maximum_turns=stock["maximumTurns"].as_u64().unwrap();let mut stock=RetainedCloneGrant{maximum_items:stock["items"].as_u64().unwrap()as usize,maximum_copy_bytes:stock["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:stock["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:stock["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:stock["depth"].as_u64().unwrap()as usize};
 let admission = grant(demand(value.as_ref().unwrap()).unwrap());
 let refused = effect_turn(RetainedCloneGrant { maximum_copy_bytes: admission.maximum_copy_bytes - 1, ..admission }, |effect| admit(&mut value,effect));
 assert_eq!(refused.work,0);assert!(matches!(refused.result,Ok(None)));assert!(value.is_some());
 let turn=effect_turn(admission,|effect|admit(&mut value,effect));assert_eq!(turn.work,u128::from(profile["work"]["admitted"].as_u64().unwrap()));let(mut owner,p)=turn.result.unwrap().unwrap();assert!(value.is_none());assert_eq!(p.copied_bytes == size_of::<Option<DrawingCompletionPayload>>() + p.retained_capacity_bytes, profile["headerCopyIncludesSourceAndFrame"].as_bool().unwrap());assert!(p.retained_capacity_bytes>0);
 stock.maximum_items-=p.copied_items;stock.maximum_copy_bytes-=p.copied_bytes;stock.maximum_capacity_bytes-=p.retained_capacity_bytes;stock.maximum_release_bytes-=p.released_bytes;
 for _ in 0..maximum_turns{if owner.terminal_is_empty(){break;}let body=stock.maximum_copy_bytes;let(p,_)=receipt(effect_turn(grant(owner.next_demand(body).unwrap()),|effect|owner.close_step(body,effect)));assert!(p.fits(stock));stock.maximum_items-=p.copied_items;stock.maximum_copy_bytes-=p.copied_bytes;stock.maximum_capacity_bytes-=p.retained_capacity_bytes;stock.maximum_release_bytes-=p.released_bytes;}
 assert!(owner.terminal_is_empty());let (cached,work)=receipt(effect_turn(Default::default(),|effect|owner.close_step(0,effect)));assert_eq!(work,0);assert_eq!(cached,RetainedCloneProgress::default());drop(owner);
 let absent=effect_turn(Default::default(),|effect|admit(&mut value,effect));assert_eq!(absent.work,u128::from(profile["work"]["absent"].as_u64().unwrap()));assert!(matches!(absent.result,Ok(None)));
}

#[test]
fn genuine_preview_host_forwards_drawing_admission_cpu_before_finishing_close(){
 let row:serde_json::Value=serde_json::from_str(include_str!("../👷️preview/📐️.json")).unwrap();let profile=&row["stock"];
 let mut stock=RetainedCloneGrant{maximum_items:profile["items"].as_u64().unwrap()as usize,maximum_copy_bytes:profile["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:profile["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:profile["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:profile["depth"].as_u64().unwrap()as usize};
 let mut job=semio_plugin_job::ChildEmissionPreviewJob::<App>{emit:None,ephemeral:None,completion:None,retirement:None,retirement_payload:Some(Value::Emit(Ok(Default::default()),Default::default())),retirement_fault:None,rejection:None,completion_retirement:None,maximum_bytes:1<<20,wire_mode:false,fault_publication:Default::default(),closing:true};
 let(denied,work)=receipt(effect_turn(Default::default(),|effect|job.close_owned_turn(effect)));assert_eq!(work,u128::from(row["deniedWork"].as_u64().unwrap()));assert_eq!(denied,RetainedCloneProgress::default());assert!(job.retirement_payload.is_some());assert!(job.retirement.is_none());
 let mut first=true;
 for _ in 0..profile["maximumTurns"].as_u64().unwrap(){if semio_framework_job::InteractiveJob::terminal_is_empty(&job){break;}let selected=grant(job.close_owned_demands(stock.maximum_copy_bytes).unwrap());let(p,work)=receipt(effect_turn(selected,|effect|job.close_owned_turn(effect)));if first{assert_eq!(work,u128::from(row["admissionWork"].as_u64().unwrap()));assert!(job.retirement_payload.is_none());assert!(job.retirement.is_some());first=false;}assert!(p.fits(stock));stock.maximum_items-=p.copied_items;stock.maximum_copy_bytes-=p.copied_bytes;stock.maximum_capacity_bytes-=p.retained_capacity_bytes;stock.maximum_release_bytes-=p.released_bytes;}
 assert!(semio_framework_job::InteractiveJob::terminal_is_empty(&job));let(cached,work)=receipt(effect_turn(Default::default(),|effect|job.close_owned_turn(effect)));assert_eq!(work,u128::from(row["cachedWork"].as_u64().unwrap()));assert_eq!(cached,RetainedCloneProgress::default());drop(job);
}
