//! 🧪️ Shared SQL stages consume the original work authority once.
use super::*;
use semio_framework_value::RetainedWorkOutcome;

struct Loan{remaining:u64,actual:u64,refusal:Option<ValueError>}
impl SqliteSnapshotWorkRecipient for Loan{
 fn admits(&mut self,_:u64,additional:u64)->std::result::Result<bool,ValueError>{Ok(additional<=self.remaining)}
 fn committed(&mut self,actual:u64)->std::result::Result<(),ValueError>{self.remaining-=actual;self.actual+=actual;match self.refusal.take(){Some(error)=>Err(error),None=>Ok(())}}
}
#[test]
fn stages_share_original_work_and_retain_refused_output(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("work.json")).unwrap();
 let mut loan=Loan{remaining:fixture["grant"].as_u64().unwrap(),actual:0,refusal:None};
 let mut progress=|_|true;let mut control=SqliteSnapshotControl::new(&mut progress,SqliteDatabaseLimits::default());
 let mut failure=None;control.install_work_recipient(&mut loan,&mut failure).unwrap();
 for(index,units)in fixture["stages"].as_array().unwrap().iter().enumerate(){
  let units=units.as_u64().unwrap();let turn=control.allocation_stage(SqliteSnapshotPhase::EncodeNative,|_,_,_,work|{
   let admitted=work.admits(0,units).unwrap();assert_eq!(admitted,fixture["accepted"][index].as_bool().unwrap());
   let result=if admitted{work.committed(units)}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original stage work denied"))};
   (RetainedWorkOutcome{result,work:if admitted{units}else{0}},0)
  });
  assert_eq!(turn.result.is_ok(),index<2);
 }
 assert_eq!(control.performed_work(),fixture["actual"].as_u64().unwrap());drop(control);
 assert_eq!(loan.actual,fixture["actual"].as_u64().unwrap());assert_eq!(loan.remaining,0);
 let error=ValueError::new(ValueRefusalKind::Canceled,fixture["postRefusal"].as_str().unwrap().to_owned());let pointer=error.message.as_ptr();
 let mut loan=Loan{remaining:1,actual:0,refusal:Some(error)};let mut failure=None;let mut control=SqliteSnapshotControl::new(&mut progress,SqliteDatabaseLimits::default());control.install_work_recipient(&mut loan,&mut failure).unwrap();
 let mut output=Vec::new();let turn=control.allocation_stage(SqliteSnapshotPhase::EncodeNative,|_,_,_,work|{
  assert!(work.admits(0,1).unwrap());output=fixture["output"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect();
  (RetainedWorkOutcome{result:work.committed(1),work:1},1)
 });
 assert_eq!(serde_json::to_value(&output).unwrap(),fixture["output"]);assert_eq!(turn.work,1);assert_eq!(control.performed_work(),1);
 match turn.result{Err(SqliteStageFailure::WorkSettlement{operation:Err(_),failure:Some(failure),allocation:Some((_,receipt)),..})=>{assert_eq!(failure.post_refusal.unwrap().message.as_ptr(),pointer);assert_eq!((failure.before,failure.actual),(0,1));assert!(failure.post_completed);assert_eq!((receipt.declared,receipt.actual),(1,Some(0)));},_=>panic!("original work and allocation refusals must remain owned")}
}
#[test]
fn original_work_post_refusal_survives_unwind_in_caller_slot(){
 let mut progress=|_|true;let error=ValueError::new(ValueRefusalKind::Canceled,"owned work refusal".to_owned());let pointer=error.message.as_ptr();let mut loan=Loan{remaining:1,actual:0,refusal:Some(error)};let mut failure=None;let mut control=SqliteSnapshotControl::new(&mut progress,SqliteDatabaseLimits::default());control.install_work_recipient(&mut loan,&mut failure).unwrap();let unwind=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{let _:RetainedWorkOutcome<(),SqliteStageFailure<(),ValueError>>=control.allocation_stage(SqliteSnapshotPhase::EncodeNative,|_,_,_,work|{assert!(work.admits(0,1).unwrap());assert!(work.committed(1).is_err());panic!("after original work post")});}));assert!(unwind.is_err());assert_eq!(control.performed_work(),1);drop(control);let failure=failure.take().expect("caller retains work refusal through unwind");assert_eq!((failure.before,failure.actual),(0,1));assert!(failure.post_completed);assert_eq!(failure.post_refusal.unwrap().message.as_ptr(),pointer);assert_eq!((loan.actual,loan.remaining),(1,0));
}
#[test]
fn original_work_post_panic_keeps_raw_effect_in_caller_slot(){
 struct PanicLoan;
 impl SqliteSnapshotWorkRecipient for PanicLoan{fn admits(&mut self,_:u64,_:u64)->std::result::Result<bool,ValueError>{Ok(true)}fn committed(&mut self,_:u64)->std::result::Result<(),ValueError>{panic!("original work recipient unwound")}}
 let mut progress=|_|true;let mut loan=PanicLoan;let mut failure=None;let mut control=SqliteSnapshotControl::new(&mut progress,SqliteDatabaseLimits::default());control.install_work_recipient(&mut loan,&mut failure).unwrap();let unwind=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{let _:RetainedWorkOutcome<(),SqliteStageFailure<(),ValueError>>=control.allocation_stage(SqliteSnapshotPhase::EncodeNative,|_,_,_,work|{assert!(work.admits(0,1).unwrap());let _=work.committed(1);(RetainedWorkOutcome{result:Ok(()),work:1},0)});}));assert!(unwind.is_err());assert_eq!(control.performed_work(),1);drop(control);let failure=failure.take().expect("caller retains the raw committed effect");assert_eq!((failure.before,failure.actual),(0,1));assert!(!failure.post_completed);assert!(failure.post_refusal.is_none());
}
#[test]
fn overflow_keeps_raw_work_and_original_post_refusal(){
 let mut progress=|_|true;let error=ValueError::new(ValueRefusalKind::Canceled,"overflow post refused".to_owned());let pointer=error.message.as_ptr();let mut loan=Loan{remaining:1,actual:0,refusal:Some(error)};let mut failure=None;let mut control=SqliteSnapshotControl::new(&mut progress,SqliteDatabaseLimits::default());control.install_work_recipient(&mut loan,&mut failure).unwrap();control.ledger.performed_work=u64::MAX;let turn=control.allocation_stage(SqliteSnapshotPhase::EncodeNative,|_,_,_,work|(RetainedWorkOutcome{result:work.committed(1),work:1},0));assert_eq!(turn.work,0);match turn.result{Err(SqliteStageFailure::WorkSettlement{failure:Some(failure),work,..})=>{assert_eq!((work.before,work.after,work.actual,work.declared),(u64::MAX,u64::MAX,Some(0),1));{assert_eq!((failure.before,failure.actual),(u64::MAX,1));assert!(failure.post_completed);assert_eq!(failure.post_refusal.unwrap().message.as_ptr(),pointer);}},_=>panic!("raw overflow and original refusal must both survive")};assert_eq!(control.performed_work(),u64::MAX);
}
#[test]
fn stage_without_original_work_refuses_before_effect(){
 let mut progress=|_|true;let mut control=SqliteSnapshotControl::new(&mut progress,SqliteDatabaseLimits::default());let mut mutated=false;
 let turn=control.allocation_stage(SqliteSnapshotPhase::EncodeNative,|_,_,_,work|{
  let result=work.admits(0,1).map(|allowed|{if allowed{mutated=true;}});
  (RetainedWorkOutcome{result,work:0},0)
 });
 assert!(!mutated);assert_eq!(control.performed_work(),0);assert!(matches!(turn.result,Err(SqliteStageFailure::Operation(_))));
}
#[test]
fn original_work_loan_cannot_reset_prior_sql_effects(){
 let mut progress=|_|true;let mut loan=Loan{remaining:3,actual:0,refusal:None};let mut failure=None;let mut control=SqliteSnapshotControl::new(&mut progress,SqliteDatabaseLimits::default());control.record_performed_work(1).unwrap();assert_eq!(control.install_work_recipient(&mut loan,&mut failure).unwrap_err().kind,ValueRefusalKind::InvariantViolated);assert_eq!(control.performed_work(),1);drop(control);assert_eq!((loan.remaining,loan.actual),(3,0));assert!(failure.is_none());
}
#[test]
fn mismatched_declared_work_keeps_actual_ledger_and_original_output(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("work.json")).unwrap();let mut progress=|_|true;let mut control=SqliteSnapshotControl::new(&mut progress,SqliteDatabaseLimits::default());let output=fixture["output"].clone();let expected=output.clone();let turn=control.allocation_stage(SqliteSnapshotPhase::EncodeNative,|_,_,_,_|(RetainedWorkOutcome::<_,ValueError>{result:Ok(output),work:fixture["mismatchDeclared"].as_u64().unwrap()},0));assert_eq!(turn.work,0);match turn.result{Err(SqliteStageFailure::WorkSettlement{operation:Ok(output),failure:None,work,allocation:None})=>{assert_eq!(output,expected);assert_eq!((work.before,work.after,work.actual,work.declared),(0,0,Some(0),fixture["mismatchDeclared"].as_u64().unwrap()));},_=>panic!("original output and both work receipts must survive")}
}
