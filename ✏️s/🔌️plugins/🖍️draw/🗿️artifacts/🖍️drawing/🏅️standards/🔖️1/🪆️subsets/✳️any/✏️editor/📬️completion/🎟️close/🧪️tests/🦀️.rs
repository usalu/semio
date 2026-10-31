use super::*;
use semio_framework_value::retirement::{RetirementCursor,RetirementStep};
#[test]
fn completion_final_header_close_reports_actual_work_separately_from_bytes(){
 let expected:serde_json::Value=serde_json::from_str(include_str!("../../📐️.json")).unwrap();
 let mut cursor=Cursor{original:ManuallyDrop::new(Some(Source::Emit(Default::default(),Default::default()))),phase:18,faulted:false};
 let denied=cursor.close_step(RetainedCloneGrant::default());assert!(matches!(&denied,RetirementStep::BudgetExhausted));assert!(cursor.original.is_some());
 let clear=cursor.close_step(RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()});assert!(cursor.terminal_is_empty());
 let cached=cursor.close_step(RetainedCloneGrant::default());
 let actual=serde_json::json!({"role":"completion","turns":[{"case":"denied","work":denied.work(),"items":0,"copyBytes":0},{"case":"clear","work":clear.work(),"items":1,"copyBytes":0},{"case":"cached","work":cached.work(),"items":0,"copyBytes":0}]});assert_eq!(actual,expected);assert!(matches!(clear,RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:0,..},1)));
}

#[test]
fn actual_drawing_completion_admission_posts_allocator_work_and_keeps_denied_input(){
 let profile:serde_json::Value=serde_json::from_str(include_str!("../../📥️admit/📐️.json")).unwrap();let stock=&profile["stock"];
 let mut value=Some(Value::Emit(Ok(Default::default()),Default::default()));
 let denied=admit(&mut value,RetainedCloneGrant::default());assert_eq!(denied.work,profile["work"]["denied"].as_u64().unwrap());assert!(matches!(denied.result,Ok(None)));assert_eq!(value.is_some(),profile["inputRetainedOnDenial"].as_bool().unwrap());
 let maximum_turns=stock["maximumTurns"].as_u64().unwrap();let mut stock=RetainedCloneGrant{maximum_items:stock["items"].as_u64().unwrap()as usize,maximum_copy_bytes:stock["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:stock["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:stock["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:stock["depth"].as_u64().unwrap()as usize};
 let turn=admit(&mut value,RetainedCloneGrant{maximum_items:1,..stock});assert_eq!(turn.work,profile["work"]["admitted"].as_u64().unwrap());let(mut owner,p)=turn.result.unwrap().unwrap();assert!(value.is_none());assert_eq!(p.copied_bytes,profile["headerCopyBytes"].as_u64().unwrap()as usize);assert!(p.retained_capacity_bytes>0);
 stock.maximum_items-=p.copied_items;stock.maximum_copy_bytes-=p.copied_bytes;stock.maximum_capacity_bytes-=p.retained_capacity_bytes;stock.maximum_release_bytes-=p.released_bytes;
 for _ in 0..maximum_turns{if owner.terminal_is_empty(){break;}let turn=owner.close_step(RetainedCloneGrant{maximum_items:1,..stock});let p=match turn.result{Ok(step)=>step.progress(),Err(error)=>panic!("completion close refused: {}",error)};assert!(p.fits(stock));stock.maximum_items-=p.copied_items;stock.maximum_copy_bytes-=p.copied_bytes;stock.maximum_capacity_bytes-=p.retained_capacity_bytes;stock.maximum_release_bytes-=p.released_bytes;}
 assert!(owner.terminal_is_empty());let cached=owner.close_step(Default::default());assert_eq!(cached.work,0);assert_eq!(cached.result.unwrap().progress(),RetainedCloneProgress::default());drop(owner);
 let absent=admit(&mut value,Default::default());assert_eq!(absent.work,profile["work"]["absent"].as_u64().unwrap());assert!(matches!(absent.result,Ok(None)));
}

#[test]
fn genuine_preview_host_forwards_drawing_admission_cpu_before_finishing_close(){
 let row:serde_json::Value=serde_json::from_str(include_str!("../../👷️preview/📐️.json")).unwrap();let profile=&row["stock"];
 let mut stock=RetainedCloneGrant{maximum_items:profile["items"].as_u64().unwrap()as usize,maximum_copy_bytes:profile["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:profile["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:profile["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:profile["depth"].as_u64().unwrap()as usize};
 let mut job=semio_framework_plugin_job::ChildEmissionPreviewJob::<App>{emit:None,ephemeral:None,completion:None,retirement:None,retirement_payload:Some(Value::Emit(Ok(Default::default()),Default::default())),retirement_fault:None,rejection:None,completion_retirement:None,maximum_bytes:1<<20,wire_mode:false,fault_publication:Default::default(),closing:true};
 let denied=job.close_owned_turn(Default::default());assert_eq!(denied.work,row["deniedWork"].as_u64().unwrap());assert_eq!(denied.result.unwrap(),RetainedCloneProgress::default());assert!(job.retirement_payload.is_some());assert!(job.retirement.is_none());
 let mut first=true;
 for _ in 0..profile["maximumTurns"].as_u64().unwrap(){if semio_framework_job::InteractiveJob::terminal_is_empty(&job){break;}let turn=job.close_owned_turn(RetainedCloneGrant{maximum_items:1,..stock});if first{assert_eq!(turn.work,row["admissionWork"].as_u64().unwrap());assert!(job.retirement_payload.is_none());assert!(job.retirement.is_some());first=false;}let p=turn.result.unwrap();assert!(p.fits(stock));stock.maximum_items-=p.copied_items;stock.maximum_copy_bytes-=p.copied_bytes;stock.maximum_capacity_bytes-=p.retained_capacity_bytes;stock.maximum_release_bytes-=p.released_bytes;}
 assert!(semio_framework_job::InteractiveJob::terminal_is_empty(&job));let cached=job.close_owned_turn(Default::default());assert_eq!(cached.work,row["cachedWork"].as_u64().unwrap());assert_eq!(cached.result.unwrap(),RetainedCloneProgress::default());drop(job);
}
