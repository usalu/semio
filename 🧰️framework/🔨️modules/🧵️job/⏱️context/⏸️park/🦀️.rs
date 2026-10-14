//! ⏸️ An actual caller delegates its remaining wallet once and retains the owned issuing recipient through await.
use super::{StepContext,StepBudget,OriginalCallerSourceParent,OriginalCallerSourceLoan,OriginalRetainedLoanIdentity,OriginalTurnFailure,RetainedCloneGrant,RetainedCloneProgress,InteractiveJobCloseStep,ValueError,ValueRefusalKind,ClockStride,retained_work::RetainedWorkBudget};
use std::mem::ManuallyDrop;

struct OwnedTurnWallet{fuel:u64,deadline:u64,grant:RetainedCloneGrant,clock:fn()->Option<u64>,epoch:u64,clock_observation:ClockStride,payload_page_granted:bool,retained_work:RetainedWorkBudget,stage:&'static str}

/// 🧳️ Owns the paid original source alias and one non-renewable wallet before issuing its context.
pub struct OriginalOwnedCallerTurn{source:ManuallyDrop<Option<OriginalCallerSourceParent>>,loan:ManuallyDrop<Option<OriginalCallerSourceLoan>>,wallet:Option<OwnedTurnWallet>,sequence:u64,recipient:RetainedCloneProgress,issued:bool,initial_fuel:u64,work_units_spent:u128,receiving_valid:bool,initial_grant:RetainedCloneGrant,deadline:u64,epoch:u64,clock:fn()->Option<u64>}

impl OriginalCallerSourceParent{
    /// 🎟️ Pays the actual source loan before moving the remaining work and physical currencies out of this donor.
    pub fn park_owned_turn(&self,cx:&mut StepContext<'_>)->Result<Option<OriginalOwnedCallerTurn>,OriginalTurnFailure>{
        let epoch=cx.original_source_epoch().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"owned async turn requires its genuinely issued source epoch"))?;
        let Some(loan)=self.admit_loan(cx)?else{return Ok(None)};
        let grant=cx.retained_grant();let fuel=cx.fuel_remaining();let deadline=cx.deadline_us();let clock=cx.clock_source();let posted=cx.retained_progress();let sequence=std::mem::take(cx.preview_sequence);let clock_observation=cx.clock.get();let payload_page_granted=cx.payload_page_granted;let retained_work=cx.retained_work;let stage=cx.stage;
        cx.fuel_remaining=0;cx.retained_work=RetainedWorkBudget::new(super::retained_work::NO_RETAINED_WORK);cx.retained=RetainedCloneGrant{maximum_items:posted.copied_items,maximum_copy_bytes:posted.copied_bytes,maximum_capacity_bytes:posted.retained_capacity_bytes,maximum_release_bytes:posted.released_bytes,maximum_depth:0};
        Ok(Some(OriginalOwnedCallerTurn{source:ManuallyDrop::new(Some(loan.into_borrowed_parent())),loan:ManuallyDrop::new(None),wallet:Some(OwnedTurnWallet{fuel,deadline,grant,clock,epoch,clock_observation,payload_page_granted,retained_work,stage}),sequence,recipient:Default::default(),issued:false,initial_fuel:fuel,work_units_spent:0,receiving_valid:true,initial_grant:grant,deadline,epoch,clock}))
    }
}

impl OriginalOwnedCallerTurn{
    /// 📏️ The actual delegated grant remains independent from all later polling callbacks.
    pub fn original_grant(&self)->RetainedCloneGrant{self.initial_grant}
    pub fn original_fuel(&self)->u64{self.initial_fuel}
    pub fn original_deadline_us(&self)->u64{self.deadline}
    pub fn original_epoch(&self)->u64{self.epoch}
    pub fn original_clock(&self)->fn()->Option<u64>{self.clock}
    /// 🤝️ The owning future borrows one actual source context continuously through the asynchronous producer.
    pub async fn execute<R>(mut self,run:impl for<'turn> AsyncFnOnce(&mut StepContext<'turn>)->Result<R,OriginalTurnFailure>)->OriginalOwnedCallerReturn<R>{
        let mut receiving_refusal=None;
        let original=if self.issued{Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"owned original turn cannot issue its wallet twice").into())}else{
            self.issued=true;let wallet=self.wallet.take().expect("one sealed original wallet remains before its only issue");let budget=StepBudget{fuel:wallet.fuel,deadline_us:wallet.deadline,retained:wallet.grant};
            let source=self.source.as_ref().expect("paid original source remains owned through its context");
            match source.borrow_context(budget,wallet.clock,&mut self.sequence,&mut self.recipient){
                Err(error)=>{self.wallet=Some(wallet);Err(error.into())}
                Ok(mut cx)=>{
                    cx.clock.set(wallet.clock_observation);cx.payload_page_granted=wallet.payload_page_granted;cx.retained_work=wallet.retained_work;cx.stage=wallet.stage;let bound=cx.bind_original_source_epoch(wallet.epoch);let identity=OriginalRetainedLoanIdentity::capture(&cx);let cancel=cx.original_cancel_token()as*const _ as usize;
                    let result=match bound{Ok(())=>run(&mut cx).await,Err(error)=>Err(error.into())};
                    let matched=identity.matches(&cx)&&cx.original_source_epoch()==Some(wallet.epoch)&&cancel==cx.original_cancel_token()as*const _ as usize&&cx.fuel_remaining()<=wallet.fuel;
                    self.work_units_spent=cx.original_work_units_spent();self.receiving_valid=matched;
                    if !matched{receiving_refusal=Some(ValueError::literal(ValueRefusalKind::InvariantViolated,"owned async producer replaced its actual issuing context"));}
                    self.wallet=Some(OwnedTurnWallet{fuel:if matched{cx.fuel_remaining()}else{0},clock_observation:cx.clock.get(),payload_page_granted:cx.payload_page_granted,retained_work:cx.retained_work,stage:cx.stage,..wallet});result
                }
            }
        };
        OriginalOwnedCallerReturn{turn:self,original:ManuallyDrop::new(Some(original)),receiving_refusal:ManuallyDrop::new(receiving_refusal)}
    }
    fn validate_polling_context(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{if let Some(source)=self.source.as_ref(){source.validate_borrowed_context(cx)}else if let Some(loan)=self.loan.as_ref(){loan.validate_context(cx)}else{Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"owned original reply source returned before its output transfer").into())}}
    fn terminal_is_empty(&self)->bool{self.source.is_none()&&self.loan.is_none()}
    fn close_source(&mut self,parent:&OriginalCallerSourceParent,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
        if self.source.is_some(){let source=self.source.take().unwrap();match source.into_source_loan(){Ok(loan)=>*self.loan=Some(loan),Err(source)=>{*self.source=Some(source);return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"owned async source cannot return while another source frontier remains").into())}}}
        let Some(loan)=self.loan.as_mut()else{return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})};
        let step=loan.close_step(parent,cx)?;if loan.terminal_is_empty(){self.loan.take();}Ok(step)
    }
}
impl Drop for OriginalOwnedCallerTurn{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"owned original turn reached Drop before its paid source loan return")}}

/// 📬️ Whole semantic return and independent receiving refusal remain owned together after their exact physical effects.
#[derive(Debug,semio_framework_value::RetireOwned)]
pub struct OriginalOwnedCallerOutcome<R>{pub original:Result<R,OriginalTurnFailure>,pub receiving_refusal:Option<ValueError>}
/// 🪢️ Carries the unchanged issuing epoch, exact physical/work receipt and still-live source until paid caller transfer.
pub struct OriginalOwnedCallerReturn<R>{turn:OriginalOwnedCallerTurn,original:ManuallyDrop<Option<Result<R,OriginalTurnFailure>>>,receiving_refusal:ManuallyDrop<Option<ValueError>>}
impl<R> OriginalOwnedCallerReturn<R>{
    pub fn validate_original_receiver(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{self.turn.validate_polling_context(cx)}
    pub fn original(&self)->Option<&Result<R,OriginalTurnFailure>>{self.original.as_ref()}
    pub fn receiving_refusal(&self)->Option<&ValueError>{self.receiving_refusal.as_ref()}
    pub fn receipt(&self)->RetainedCloneProgress{self.turn.recipient}
    pub fn work_units_spent(&self)->u128{self.turn.work_units_spent}
    pub fn fuel_remaining(&self)->Option<u64>{self.turn.receiving_valid.then(||self.turn.wallet.as_ref().map_or(0,|wallet|wallet.fuel))}
    pub fn original_epoch(&self)->u64{self.turn.epoch}
    pub fn original_deadline_us(&self)->u64{self.turn.deadline}
    pub fn original_grant(&self)->RetainedCloneGrant{self.turn.initial_grant}
    /// 📤️ Polling authority pays only the transfer of this complete already-debited original return.
    pub fn take_original_on_caller(&mut self,cx:&mut StepContext<'_>)->Result<Option<OriginalOwnedCallerOutcome<R>>,OriginalTurnFailure>{
        if self.original.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"owned async reply was already transferred").into())}
        self.turn.validate_polling_context(cx)?;let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
        cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);
        *cx.preview_sequence=(*cx.preview_sequence).max(self.turn.sequence);if let Some(wallet)=self.turn.wallet.as_ref(){if cx.latest_us().is_none_or(|now|wallet.clock_observation.latest_us().is_some_and(|returned|returned>now)){cx.clock.set(wallet.clock_observation);}}
        Ok(Some(OriginalOwnedCallerOutcome{original:self.original.take().unwrap(),receiving_refusal:self.receiving_refusal.take()}))
    }
    /// ♻️ Its exact original source returns to the still-retained actual parent under paid caller metadata.
    pub fn close_source(&mut self,parent:&OriginalCallerSourceParent,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{if self.original.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"owned original source stays retained until its output transfer").into())}self.turn.close_source(parent,cx)}
    pub fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.receiving_refusal.is_none()&&self.turn.terminal_is_empty()}
}
impl<R> Drop for OriginalOwnedCallerReturn<R>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"owned async reply reached Drop before original transfer and paid source return")}}
