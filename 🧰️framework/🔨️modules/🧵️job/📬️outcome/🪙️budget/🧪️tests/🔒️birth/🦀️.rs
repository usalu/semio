//! 📏️ The actual origin producer quote fits one configured native copy window.
use semio_framework_job_return_budget::{ORIGINAL_RETURN_BIRTH_ACTION_WORK,OriginalReturnBirthPhase,OriginalReturnBirthStage,OriginalReturnBudget,OriginalReturnOrigin,OriginalReturnOriginProducer};
use semio_framework_value_control_contract::{RetainedCloneGrant,RetainedCloneProgress,OriginalSourceTurnIdentity};
#[derive(Clone,Copy)]
struct LayoutOrigin;
impl OriginalReturnOrigin for LayoutOrigin{
 fn same_origin(&self,_:&Self)->bool{true}
 fn recipient(&self)->usize{0}
 fn source_identity(&self)->usize{0}
 fn source_turn(&self)->Option<OriginalSourceTurnIdentity>{None}
 fn granted(&self)->RetainedCloneGrant{Default::default()}
 fn fuel(&self)->u64{0}
 fn spent(&self)->RetainedCloneProgress{Default::default()}
 fn work(&self)->u128{0}
}
#[test]
fn original_origin_producer_request_fits_the_fixed_window(){
 let demand=OriginalReturnOriginProducer::<LayoutOrigin>::demand().unwrap();
 assert!(demand.copy_bytes<=4096,"actual origin producer quote {} exceeds fixed 4096",demand.copy_bytes);
 assert_eq!(OriginalReturnOriginProducer::<LayoutOrigin>::work_demand(),3);
}

#[test]
fn held_guard_belongs_only_to_its_actual_gate(){
 use semio_framework_value_retained_clone::retirement::controlled::RetainedOwnerGate;
 let cases:serde_json::Value=serde_json::from_str(include_str!("guard.json")).unwrap();let original=RetainedOwnerGate::new(11u8);let foreign=RetainedOwnerGate::new(12u8);let mut guard=original.try_lock().unwrap();
 assert_eq!(original.owns_guard(&guard),cases[0]["owned"].as_bool().unwrap());assert_eq!(foreign.owns_guard(&guard),cases[1]["owned"].as_bool().unwrap());assert_eq!(*guard,11);*guard=13;assert!(original.try_lock().is_err());drop(guard);assert_eq!(*original.try_lock().unwrap(),13);assert_eq!(*foreign.try_lock().unwrap(),12);
}

#[test]
fn every_birth_stage_quote_fits_the_fixed_window_and_a_fresh_budget_has_no_birth(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("birth.json")).unwrap();let budget=OriginalReturnBudget::<LayoutOrigin>::empty(1);
 assert_eq!(ORIGINAL_RETURN_BIRTH_ACTION_WORK,cases["actionWork"].as_u64().unwrap());
 for case in cases["stages"].as_array().unwrap(){
  let stage=match case["stage"].as_str().unwrap(){"table"=>OriginalReturnBirthStage::Table,"tableCommit"=>OriginalReturnBirthStage::TableCommit,"publisher"=>OriginalReturnBirthStage::Publisher,"publisherCommit"=>OriginalReturnBirthStage::PublisherCommit,"release"=>OriginalReturnBirthStage::Release,other=>panic!("unknown stage {other}")};
  assert_eq!(stage.minimum_work(),case["minimumWork"].as_u64().unwrap());assert_eq!(stage.item_demand() as u64,case["items"].as_u64().unwrap());
  let demand=budget.birth_demand(stage).unwrap();assert!(demand.copy_bytes>0&&demand.copy_bytes<=case["maximumCopyBytes"].as_u64().unwrap() as usize,"stage {:?} quote {} exceeds its window",stage,demand.copy_bytes);
 }
 assert_eq!([budget.origin_phase(),budget.table_phase(),budget.publisher_phase()],[OriginalReturnBirthPhase::Absent;3]);
}
