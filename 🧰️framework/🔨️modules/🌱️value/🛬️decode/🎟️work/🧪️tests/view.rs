use semio_framework_value_native_decoding::*;
struct Stock{remaining:u64,calls:u64}
impl OriginalWorkRecipient for Stock{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn admits<'reply,'turn,'source>(&mut self,_:&mut (),quote:OriginalWorkQuote<'reply>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'reply>where 'source:'turn{self.calls+=1;quote.complete(Ok(observed.checked_add(additional).is_some_and(|work|work<=self.remaining)))}
 fn settle_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
 fn recover_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
}
fn available(native:&NativeDecodeControl<'_>,stock:u64)->u64{stock-u64::try_from(native.actual_work()).unwrap()-native.pending_work()}

#[test]
fn actual_original_work_view_preserves_prepaid_reservations_and_nested_baseline(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../view.json")).unwrap();let stock=law["stock"].as_u64().unwrap();let reserved=law["reserved"].as_u64().unwrap();let committed=law["committed"].as_u64().unwrap();let cancelled=law["cancelled"].as_u64().unwrap();
 let mut observer=|_|true;let mut native=NativeDecodeControl::new(0,&mut observer);assert!(native.prepare_work(1).is_err());assert_eq!(native.actual_work(),0);
 let mut authority=OriginalWorkAuthority::new(Stock{remaining:stock,calls:0});let mut views=[0u64;5];
 authority.with_loan(&mut (),|loan|native.with_work_recipient(loan,|native|{
  views[0]=available(native,stock);assert!(native.prepare_work(reserved)?);views[1]=available(native,stock);native.commit_work(u128::from(committed))?;views[2]=available(native,stock);
  let mut observer=|_|true;native.scoped_observer(&mut observer,|native|{views[3]=available(native,stock);Ok::<_,ValueError>(())})?;
  assert!(native.prepare_work(cancelled)?);native.cancel_work()?;views[4]=available(native,stock);Ok::<_,ValueError>(())
 })).unwrap().unwrap();
 assert!(!authority.has_live_actions());assert_eq!(authority.inspect(|backend|backend.calls),law["admissionCalls"].as_u64().unwrap());assert_eq!(native.actual_work(),u128::from(law["actual"].as_u64().unwrap()));assert_eq!(serde_json::json!(views),law["views"]);
 authority.with_loan(&mut (),|loan|native.with_work_recipient(loan,|native|{assert_eq!(available(native,stock),law["nextScope"].as_u64().unwrap());Ok::<_,ValueError>(())})).unwrap().unwrap();
 assert_eq!(authority.inspect(|backend|backend.calls),law["admissionCalls"].as_u64().unwrap());assert_eq!(native.actual_work(),u128::from(committed));assert!(!authority.has_live_actions());assert!(authority.into_recipient().is_ok());
}
