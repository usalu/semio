//! 🧪️ One caller work loan observes actual storage and validation effects across nested decoder scopes.
use super::*;
struct Loan{remaining:u64,calls:u64}
impl NativeDecodeWorkRecipient for Loan{fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{self.calls+=1;Ok(observed.checked_add(additional).is_some_and(|total|total<=self.remaining))}}
#[test]fn actual_work_loan_denial_nested_effect_and_failure_keep_one_original_baseline(){
 let f:serde_json::Value=serde_json::from_str(include_str!("../📐️.json")).unwrap();let mut callback=|_|true;let mut native=NativeDecodeControl::new(f["stock"]["capacityBytes"].as_u64().unwrap()as usize,&mut callback);
 assert!(native.prepare_work(1).is_err());assert_eq!(native.actual_work(),0);let mut denied=Loan{remaining:0,calls:0};native.with_work_recipient(&mut denied,|native|{assert!(!native.prepare_work(1)?);assert_eq!(native.actual_work(),0);Ok::<_,ValueError>(())}).unwrap();
 let mut loan=Loan{remaining:f["stock"]["fuel"].as_u64().unwrap(),calls:0};let mut body=Vec::<u8>::new();let result=native.with_work_recipient(&mut loan,|native|{
  assert!(native.prepare_work(1)?);body.try_reserve_exact(2).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"actual work law allocation failed"))?;native.commit_work(1)?;
  let mut observer=|_|true;native.scoped_observer(&mut observer,|native|{assert!(native.prepare_work(1)?);body.extend_from_slice(&[3,7]);native.commit_work(1)?;Ok::<_,ValueError>(())})?;
  let mut replacement=Loan{remaining:100,calls:0};assert!(native.with_work_recipient(&mut replacement,|_|Ok::<_,ValueError>(())).is_err());
  assert!(native.prepare_work(1)?);let result=std::str::from_utf8(&[255]);native.commit_work(1)?;assert!(result.is_err());assert_eq!(body,serde_json::from_value::<Vec<u8>>(f["body"].clone()).unwrap());assert!(native.prepare_work(1)?);drop(std::mem::take(&mut body));native.commit_work(1)?;assert!(!native.prepare_work(1)?);Err::<(),_>(ValueError::literal(ValueRefusalKind::InvalidValue,"actual UTF-8 probe failed"))
 });assert!(result.is_err());assert_eq!(native.actual_work(),f["expected"]["work"].as_u64().unwrap());assert_eq!(loan.remaining,4);assert!(body.is_empty());assert_eq!(native.actual_work(),4);
}
