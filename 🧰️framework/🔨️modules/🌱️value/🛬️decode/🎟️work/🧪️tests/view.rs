use semio_framework_value_native_decoding::{NativeDecodeControl,NativeDecodeWorkIssuer,NativeDecodeWorkRecipient,ValueError};
struct Stock{remaining:u64,calls:u64}
impl NativeDecodeWorkRecipient for Stock{
 fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{self.remaining.checked_sub(observed).ok_or_else(||ValueError::invariant("view fixture exceeded its original scoped work"))}
 fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{self.calls+=1;Ok(observed.checked_add(additional).is_some_and(|work|work<=self.remaining))}
}

#[test]
fn actual_original_work_view_preserves_prepaid_reservations_and_nested_baseline(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../view.json")).unwrap();let stock=law["stock"].as_u64().unwrap();let reserved=law["reserved"].as_u64().unwrap();let committed=law["committed"].as_u64().unwrap();let cancelled=law["cancelled"].as_u64().unwrap();
 let mut observer=|_|true;let mut native=NativeDecodeControl::new(0,&mut observer);assert!(native.available_work_units().is_err());assert_eq!(native.actual_work(),0);
 let mut backend=Stock{remaining:stock,calls:0};let mut issuer=NativeDecodeWorkIssuer::new();let mut views=[0u64;5];
 {let mut loan=issuer.loan(&mut backend);native.with_work_recipient(&mut loan,|native|{
  views[0]=native.available_work_units()?;assert!(native.prepare_work(reserved)?);views[1]=native.available_work_units()?;native.commit_work(committed)?;views[2]=native.available_work_units()?;
  let mut observer=|_|true;native.scoped_observer(&mut observer,|native|{views[3]=native.available_work_units()?;Ok::<_,ValueError>(())})?;
  assert!(native.prepare_work(cancelled)?);native.cancel_work()?;views[4]=native.available_work_units()?;Ok::<_,ValueError>(())
 }).unwrap();}
 assert!(!issuer.has_pending_work());assert_eq!(backend.calls,law["admissionCalls"].as_u64().unwrap());assert_eq!(native.actual_work(),law["actual"].as_u64().unwrap());assert_eq!(serde_json::json!(views),law["views"]);
 backend.remaining=backend.remaining.checked_sub(native.actual_work()).unwrap();
 {let mut loan=issuer.loan(&mut backend);native.with_work_recipient(&mut loan,|native|{assert_eq!(native.available_work_units()?,law["nextScope"].as_u64().unwrap());Ok::<_,ValueError>(())}).unwrap();}
 assert_eq!(backend.calls,law["admissionCalls"].as_u64().unwrap());assert_eq!(native.actual_work(),committed);assert!(!issuer.has_pending_work());
}
