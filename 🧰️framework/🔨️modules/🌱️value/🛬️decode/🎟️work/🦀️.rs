//! 🎟️ Decoder loans move the actual Core Work owner and preserve its original Source across turns.
use super::{NativeDecodeControl,ValueError,ValueRefusalKind};
pub use semio_framework_value_control_contract::{OriginalWorkAuthority,OriginalWorkIssuerIdentity,OriginalWorkActionIssuer,OriginalWorkLoan,OriginalWorkRecipient,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete,OriginalSourceTurnIdentity};
pub(super) use semio_framework_value_control_contract::{OriginalWork as Work,OriginalWorkState as WorkState};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"native work requires its original issuer and resolved quote")}
struct Scope<'owner,'call>{control:NativeDecodeControl<'call>,maximum:&'owner mut usize,scope_maximum:&'owner mut Option<usize>,ledger:&'owner mut Option<semio_framework_value_native_encoding::native_receiving::NativeAllocationLedger>,completed:&'owner mut usize,total:&'owner mut usize,started:&'owner mut bool,stage:&'owner mut u64,depth:&'owner mut usize,state:&'owner mut WorkState}
impl Drop for Scope<'_, '_>{fn drop(&mut self){*self.maximum=self.control.maximum_bytes;*self.scope_maximum=self.control.scope_maximum;*self.ledger=self.control.ledger.take();*self.completed=self.control.completed;*self.total=self.control.total;*self.started=self.control.started;*self.stage=self.control.stage;*self.depth=self.control.depth;*self.state=self.control.work.state.take();}}
impl NativeDecodeControl<'_>{
 /// 🧵️ Lends the actual dispatch witness only while its genuine Work Source is installed.
 pub fn with_original_source_witness<'turn,R>(&mut self,witness:semio_framework_value_control_contract::OriginalSourceTurnWitness<'turn>,operation:impl FnOnce(&mut Self,semio_framework_value_control_contract::OriginalSourceTurnWitness<'turn>)->Result<R,ValueError>)->Result<R,ValueError>{if self.original_work_source_turn()?!=Some(witness.identity()){return Err(ValueError::invariant("native decoder witness differs from its original Work Source"))}operation(self,witness)}
 /// 🪢️ Borrows the actual Work state through its sealed first-party port.
 pub fn original_work_borrow(&mut self)->semio_framework_value_control_contract::OriginalWorkBorrow<'_>{semio_framework_value_control_contract::OriginalWorkBorrow::new(&mut self.work)}
 /// 🫴️ Lends the actual disjoint Work and construction authorities to a scoped payout.
 pub fn with_original_work_and_construction<R>(&mut self,operation:impl FnOnce(&mut semio_framework_value_control_contract::OriginalWorkBorrow<'_>,&mut semio_framework_value_control_contract::OriginalConstructionLoan<'_>)->Result<R,ValueError>)->Result<R,ValueError>{let recipient=self.construction.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native construction authority is absent"))?;let mut recipient=recipient.reborrow();let mut work=semio_framework_value_control_contract::OriginalWorkBorrow::new(&mut self.work);operation(&mut work,&mut recipient)}
 /// 🧮️ Reads actual charged work from the same original operation.
 pub fn actual_work(&self)->u128{self.work.actual()}
 /// 🔐️ Validates the actual installed Source before selecting a caller-supplied work ceiling.
 pub fn validate_work_recipient(&self)->Result<(),ValueError>{self.work.recipient.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native work selection requires its actual caller loan"))?.validate_context()}
 /// 🪟️ Reads only remaining work in an already-admitted original action.
 pub fn remaining_prepared_work_units(&self)->Result<u64,ValueError>{let action=self.work.action_identity().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native construction requires an admitted original work action"))?;self.work.validate_action_source(Some(action))?;let performed=self.work.paid_for_action(Some(action))?;u128::from(self.work.reserved()).checked_sub(performed).and_then(|remaining|u64::try_from(remaining).ok()).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original native work exceeded its admitted action"))}
 /// 🧾️ Validates the actual retained Core proof against its original action.
 pub fn validate_work_proof(&self,proof:&semio_framework_value_control_contract::OriginalWorkActionProof,actual:u128)->Result<bool,ValueError>{self.work.validates(proof,actual)}
 /// 🔭️ Reads the actual installed Source loan before selecting a private producer context.
 pub fn original_work_source_turn(&self)->Result<Option<semio_framework_value_control_contract::OriginalSourceTurnIdentity>,ValueError>{let loan=self.work.recipient.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native constructor requires its original Work loan"))?;loan.validate_context()?;Ok(loan.source_turn())}
 pub fn has_work_recipient(&self)->bool{self.work.recipient.is_some()}
 pub fn has_pending_work(&self)->bool{self.work.has_live_actions()}
 pub fn prepared_work_admits(&self,amount:u64)->bool{self.work.same_issuer()&&!self.work.quote_pending()&&self.work.reserved()>=amount}
 pub fn pending_work(&self)->u64{self.work.reserved()}
 pub fn work_quote_pending(&self)->bool{self.work.quote_pending()}
 pub fn prepare_work(&mut self,additional:u64)->Result<bool,ValueError>{self.work.prepare(additional,None)}
 /// 🔁️ Recovers the recorded original quote without accepting an injected replacement reply.
 pub fn recover_work_quote(&mut self)->Result<Option<bool>,ValueError>{self.work.recover_quote()}
 pub fn cancel_work(&mut self)->Result<(),ValueError>{self.work.cancel()}
 pub(crate) fn cancel_prepared_work(&mut self){if self.work.action_identity().is_some(){self.work.cancel().expect("original work cancellation requires its issuer and resolved quote")}}
 pub fn commit_work(&mut self,actual:u128)->Result<(),ValueError>{self.work.commit(actual)}
 /// 🫴️ Reborrows the actual Source while moving its complete operation state into the shorter control.
 pub fn with_work_recipient<T,E:From<ValueError>>(&mut self,recipient:&mut OriginalWorkLoan<'_>,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->Result<T,E>)->Result<T,E>{
  self.with_work_recipient_matching(recipient,None,operation)
 }
 pub(super) fn with_work_recipient_matching<T,E:From<ValueError>>(&mut self,recipient:&mut OriginalWorkLoan<'_>,expected:Option<semio_framework_value_control_contract::OriginalWorkBindingIdentity>,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->Result<T,E>)->Result<T,E>{
  self.work.validate_recipient(recipient).map_err(E::from)?;let mut construction=self.construction.as_mut().map(|loan|loan.reborrow());let mut forwarded=match &mut self.allocation{super::allocation::Binding::Local=>None,super::allocation::Binding::Forwarded(loan)=>Some(&mut **loan)};let mut work_loan=recipient.reborrow();let mut state=self.work.state.take();state.rebind_pending(&work_loan);
  let control=NativeDecodeControl{ledger:self.ledger.take(),scope_maximum:self.scope_maximum,construction:construction.as_mut(),receiving:self.receiving,maximum_bytes:self.maximum_bytes,completed:self.completed,total:self.total,started:self.started,stage:self.stage,depth:self.depth,callback:&mut *self.callback,source:self.source,retirement:self.retirement.as_deref_mut(),allocation:match forwarded.as_mut(){None=>super::allocation::Binding::Local,Some(loan)=>super::allocation::Binding::Forwarded(&mut **loan)},work:Work{state,recipient:Some(&mut work_loan)}};
  let mut scope=Scope{control,maximum:&mut self.maximum_bytes,scope_maximum:&mut self.scope_maximum,ledger:&mut self.ledger,completed:&mut self.completed,total:&mut self.total,started:&mut self.started,stage:&mut self.stage,depth:&mut self.depth,state:&mut self.work.state};let actual=semio_framework_value_control_contract::OriginalWorkBorrow::new(&mut scope.control.work).binding_identity().map_err(E::from)?;if expected.is_some_and(|original|original!=actual){return Err(E::from(ValueError::literal(ValueRefusalKind::OwnershipLimit,"resident decoder requires its original Work and Source")))}if !scope.control.work.state.has_live_actions(){scope.control.work.state.begin_turn()}operation(&mut scope.control)
 }
}
impl<'a> NativeDecodeControl<'a>{
 /// 🎟️ Installs the same original caller loan after validating its retained issuer and Source.
 pub fn install_work_recipient(&mut self,recipient:&'a mut OriginalWorkLoan<'a>)->Result<(),ValueError>{self.work.validate_recipient(recipient)?;if self.work.recipient.is_some(){return Err(invalid())}self.work.rebind_pending(recipient);self.work.recipient=Some(recipient);Ok(())}}
