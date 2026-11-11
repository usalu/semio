//! 🎟️ Every fixture supplies its original issuer, source admission, clock and work before a once-paid caller is born.
use semio_framework_job_context::{CancelToken,JobPayloadAuthority,OperationId,Generation,RetainedCloneGrant,RetainedCloneProgress,StepContext,StepBudget,OriginalTurnFailure,OriginalCallerSourceParent,OriginalCallerSourceAdmission};
use semio_framework_value_control_contract::{RetainedAllocationEffect,RetainedWorkOutcome,RetainedWorkPoll};
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
use std::mem::ManuallyDrop;
use semio_framework_job_context::{RetainedCloneStep,RetirementDemand};
use semio_framework_value_retained_clone::{ValueError,ValueRefusalKind};
pub struct OriginalJobFixtureCaller{operation:OperationId,generation:Generation,parent:ManuallyDrop<Option<OriginalCallerSourceParent>>,sequence:u64,clock:fn()->Option<u64>,clock_binding:u64,deadline:u64,fuel:u64,clock_stride:semio_framework_job_context::ClockStride,stock:RetainedCloneGrant,spent:RetainedCloneProgress,born:usize,audit:bool,roots_released:usize,last:RetainedCloneProgress,business:RetainedCloneProgress,birth:semio_framework_job_native_turn::OriginalJobReturnBirth,dispatch:Option<semio_framework_job_context::OriginalSourceTurn>}
impl OriginalJobFixtureCaller{
    pub fn admit(operation:OperationId,generation:Generation,source:RetainedCloneGrant,clock:fn()->Option<u64>,deadline:u64,business_fuel:u64)->Self{
        let mut fuel=2+OriginalCallerSourceAdmission::backing_work_demand();let mut spent=RetainedCloneProgress::default();let mut born=0;let mut clock_stride=semio_framework_job_context::ClockStride::new();
        assert!(fixture_work_open(clock,deadline,fuel,&mut clock_stride));let grant=fixture_remaining(source,spent);
        let(turn,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||CancelToken::admit_root(grant));let(cancel,receipt)=turn.result.unwrap().unwrap();assert!(!heap.overflowed);assert!(receipt.fits(grant));assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(heap.requested_bytes,heap.released_bytes));spent=spent.checked_add(receipt).unwrap();born+=heap.requested_bytes;
        assert!(fixture_work_open(clock,deadline,fuel,&mut clock_stride));let grant=fixture_remaining(source,spent);
        let mut authority_actual=None;let(turn,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||JobPayloadAuthority::admit(operation,generation,grant,&mut authority_actual));assert_eq!(authority_actual,Some(heap.requested_bytes));let(authority,receipt)=turn.result.unwrap().unwrap();assert!(!heap.overflowed);assert!(receipt.fits(grant));assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(heap.requested_bytes,heap.released_bytes));spent=spent.checked_add(receipt).unwrap();born+=heap.requested_bytes;
        assert!(fixture_work_open(clock,deadline,fuel,&mut clock_stride));let grant=fixture_remaining(source,spent);
        assert!(fuel>=OriginalCallerSourceParent::admission_work_demand());
        let mut authority=Some(authority);let mut cancel=Some(cancel);let mut admission=OriginalCallerSourceAdmission::new();let mut parent=None;
        let mut bank=EffectBank::new(Wallet::from_profile(&serde_json::json!({"items":grant.maximum_items,"copyBytes":grant.maximum_copy_bytes,"capacityBytes":grant.maximum_capacity_bytes,"releaseBytes":grant.maximum_release_bytes,"depth":grant.maximum_depth,"fuel":fuel})));
        let mut paid=|spent:RetainedCloneProgress,fuel:u64,demand:RetirementDemand,work:u64,run:&mut dyn FnMut(&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome|{let issued=fixture_remaining(source,spent);let issued=RetainedCloneGrant{maximum_items:issued.maximum_items.min(demand.depth.max(1)),maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth,..issued};assert!(fuel>=work);let((poll,actual),heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||bank.run(issued,work,|effect|RetainedWorkPoll::Complete(run(effect))));let RetainedWorkPoll::Complete(turn)=poll else{panic!("original fixture Source birth stayed pending")};let step=turn.result.expect("original fixture Source birth step");bank.post(step.progress(),issued,heap);assert_eq!(bank.take_admitted(),Some(true));assert_eq!(actual,if demand.capacity_bytes==0{None}else{Some(demand.capacity_bytes)});(step.progress(),turn.work,heap)};
        let(receipt,work,heap)=paid(spent,fuel,OriginalCallerSourceAdmission::prepare_demand(),1,&mut|e|admission.prepare(&mut authority,&mut cancel,e));fuel-=u64::try_from(work).expect("original source work fits its u64 fuel");spent=spent.checked_add(receipt).unwrap();born+=heap.0;
        let(receipt,work,heap)=paid(spent,fuel,OriginalCallerSourceAdmission::backing_demand().expect("original Source backing quote"),OriginalCallerSourceAdmission::backing_work_demand(),&mut|e|admission.admit_backing(e));fuel-=u64::try_from(work).expect("original source work fits its u64 fuel");spent=spent.checked_add(receipt).unwrap();born+=heap.0;
        let(receipt,work,heap)=paid(spent,fuel,OriginalCallerSourceAdmission::publish_demand(),1,&mut|e|admission.publish(&mut parent,e));fuel-=u64::try_from(work).expect("original source work fits its u64 fuel");spent=spent.checked_add(receipt).unwrap();born+=heap.0;
        let parent=parent.expect("original fixture Source parent published");assert!(spent.fits(source));
        Self{operation,generation,parent:ManuallyDrop::new(Some(parent)),sequence:0,clock,clock_binding:semio_framework_trace::allocate_operation_id().0,deadline,fuel:business_fuel,clock_stride,stock:source,spent,born,audit:false,roots_released:0,last:RetainedCloneProgress::default(),business:RetainedCloneProgress::default(),birth:Default::default(),dispatch:None}
    }
    /// 🌱️ Advances the resident return birth driver turn by turn until the Source parent has its table and publisher.
    /// 🎫️ Keeps one Source dispatch for every later turn, as the production owned caller does across Pending.
    pub fn retain_dispatch(&mut self)->Result<(),ValueError>{let turn=self.parent.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original fixture parent has already closed"))?.issue_dispatch()?;self.dispatch=Some(turn);Ok(())}
    pub fn birth_returns(&mut self,grant:RetainedCloneGrant){
        let mut birth=std::mem::take(&mut self.birth);
        for _ in 0..64{let step=self.turn(grant,|cx|Ok(birth.advance(cx)?)).expect("original fixture return birth turn");if matches!(step,semio_framework_job_native_turn::OriginalJobReturnBirthTurn::Born{..}){break}assert!(!matches!(step,semio_framework_job_native_turn::OriginalJobReturnBirthTurn::Unaffordable{..}),"original fixture return birth is unaffordable")}
        self.birth=birth;
    }
    /// 🪪️ One decoder prerequisite turn, the Source loan, the work issuer, then the return table, funded by the given grant.
    pub fn admit_decoder_step(&mut self,decoder:&mut semio_framework_job_context::OriginalJobDecodeOwner,grant:RetainedCloneGrant)->bool{
        self.turn(grant,|cx|{if !decoder.admit_step(cx)?{return Ok(false)}Ok(cx.admit_original_return_table()?)}).expect("original decoder prerequisite turn")
    }
    /// 🪪️ Admits a decoder's prerequisites, each turn funded by exactly the prerequisite quote.
    pub fn admitted_decoder(&mut self)->semio_framework_job_context::OriginalJobDecodeOwner{
        let(grant,_)=semio_framework_job_native_turn::original_decoder_prerequisite_quote().expect("bounded decoder prerequisite quote");let mut decoder=semio_framework_job_context::OriginalJobDecodeOwner::empty();
        for _ in 0..4{if self.admit_decoder_step(&mut decoder,grant){return decoder}}panic!("original decoder prerequisites must admit within their quoted turns")
    }
    /// 🧮️ The Source birth's own quoted copy bytes: the finite stock a fixture caller needs beyond what its later turns copy.
    pub fn birth_copy_bytes()->usize{OriginalCallerSourceAdmission::prepare_demand().copy_bytes+OriginalCallerSourceAdmission::backing_demand().expect("original Source backing quote").copy_bytes+OriginalCallerSourceAdmission::publish_demand().copy_bytes}
    /// 🔁️ The turn grant that pays the origin refresh of a Source with a return table first and leaves `business` for the callback; narrow to `business` with a loan inside the turn.
    pub fn refreshed(business:RetainedCloneGrant)->RetainedCloneGrant{semio_framework_job_native_turn::original_return_refresh_turn_quote().and_then(|quote|quote.beneath(business,0)).expect("original origin refresh quote").0}
    /// 🔁️ One turn funded with its origin refresh quote whose callback sees only `business` after the refresh is paid.
    pub fn refreshed_turn<R>(&mut self,business:RetainedCloneGrant,advance:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>{self.refreshed_original(business,None,advance)}
    /// 🔁️ [`Self::refreshed_turn`] borrowing the explicitly supplied original decoder.
    pub fn refreshed_turn_with_decoder<R>(&mut self,business:RetainedCloneGrant,decoder:&mut semio_framework_job_context::OriginalJobDecodeOwner,advance:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>{self.refreshed_original(business,Some(decoder),advance)}
    fn refreshed_original<R>(&mut self,business:RetainedCloneGrant,decoder:Option<&mut semio_framework_job_context::OriginalJobDecodeOwner>,advance:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>{
        let mut spent=RetainedCloneProgress::default();
        let result=self.turn_original(Self::refreshed(business),decoder,true,|cx|{let mut value=None;cx.with_retained_loan(business,|cx|{let before=cx.retained_progress();let result=advance(cx);spent=fixture_delta(before,cx.retained_progress());value=Some(result?);Ok(())})?;Ok(value.expect("original fixture callback returned its value"))});
        self.business=spent;result
    }
    /// 🫧️ One turn that pays no origin refresh, for owners that never touch the Source's return table.
    pub fn bare_turn<R>(&mut self,grant:RetainedCloneGrant,advance:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>{self.turn_original(grant,None,false,advance)}
    /// 🧾️ What the callback of the last [`Self::refreshed_turn`] alone charged, after the turn paid its origin refresh.
    pub fn business_receipt(&self)->RetainedCloneProgress{self.business}
    /// 🎫️ The grant that funds the one atomic turn which births a Source's return table.
    pub fn table_birth()->RetainedCloneGrant{semio_framework_job_native_turn::original_return_table_birth_quote().expect("bounded return table birth quote").standalone()}
    /// 🌱️ Admits the return table of this Source, each turn funded by exactly its birth quote; returns the retained capacity those turns paid.
    pub fn admit_return_table(&mut self)->usize{
        let quote=semio_framework_job_native_turn::original_return_table_birth_quote().expect("bounded return table birth quote").standalone();let mut paid=0;
        for _ in 0..4{let born=self.turn(quote,|cx|Ok(cx.admit_original_return_table()?)).expect("original return table birth turn");paid+=self.last.retained_capacity_bytes;if born{return paid}}
        panic!("original return table must admit within its quoted turns")
    }
    pub fn born_bytes(&self)->usize{self.born}
    /// 🪪️ The genuine Source parent this caller retains until its terminal closure.
    pub fn source_parent(&self)->&OriginalCallerSourceParent{self.parent.as_ref().expect("original fixture caller retains its parent")}
    /// 🔬️ Asserts after every turn that its heap allocations equal its receipt and, at the end, that every retained byte was released.
    pub fn audit_heap(&mut self){self.audit=true}
    pub fn original_clock_binding(&self)->u64{self.clock_binding}
    pub fn cancel(&self)->&CancelToken{self.parent.as_ref().expect("original fixture caller retains its parent").cancellation().expect("original fixture caller retains its root")}
    pub fn admit_alias(&mut self,source:RetainedCloneGrant)->CancelToken{let mut alias=None;assert!(self.turn(source,|cx|cx.admit_original_cancel_alias(&mut alias)).expect("original fixture alias turn"));let alias=alias.expect("original fixture alias admitted");assert!(alias.is_same_node(self.cancel()));assert_eq!(self.last,RetainedCloneProgress{copied_items:1,..Default::default()});alias}
    pub fn turn<R>(&mut self,grant:RetainedCloneGrant,advance:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>{
        self.turn_original(grant,None,true,advance)
    }
    /// 🛬️ Borrows the same caller turn and its explicitly supplied original decoder without issuing authority.
    pub fn turn_with_decoder<R>(&mut self,grant:RetainedCloneGrant,decoder:&mut semio_framework_job_context::OriginalJobDecodeOwner,advance:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>{self.turn_original(grant,Some(decoder),true,advance)}
    fn turn_original<R>(&mut self,grant:RetainedCloneGrant,decoder:Option<&mut semio_framework_job_context::OriginalJobDecodeOwner>,serve:bool,advance:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>{
        let before=self.spent;let issued_fuel=self.fuel;let mut value=None;let mut served=0usize;self.clock_stride.begin((self.clock)());
        let parent=self.parent.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original fixture parent has already closed"))?;let budget=StepBudget::new(self.fuel,self.deadline,self.stock);let mut cx=match self.dispatch.as_ref(){Some(turn)=>parent.borrow_context_for_turn(turn,budget,self.clock,&mut self.sequence,&mut self.spent)?,None=>parent.borrow_context(budget,self.clock,&mut self.sequence,&mut self.spent)?};
        if let Some(decoder)=decoder{cx=cx.with_original_decoder(decoder);}
        cx.bind_original_clock_binding(self.clock_binding)?;cx.restore_clock_stride(self.clock_stride);assert_eq!(cx.operation(),self.operation);assert_eq!(cx.generation(),self.generation);
        let birth=&mut self.birth;let run=||std::panic::catch_unwind(std::panic::AssertUnwindSafe(||cx.with_retained_loan(grant,|cx|{if serve{let funded=cx.retained_progress().retained_capacity_bytes;birth.service(cx)?;served+=cx.retained_progress().retained_capacity_bytes-funded;}value=Some(advance(cx)?);Ok(())})));let(result,observed)=if self.audit{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(run);(result,Some(heap))}else{(run(),None)};
        self.fuel=cx.original_source_fuel_remaining().min(issued_fuel);self.clock_stride=cx.clock_stride();let reserved=cx.reserved_return_progress();self.born+=served;drop(cx);self.last=fixture_delta(before,self.spent);if let Some(heap)=observed{assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(self.last.retained_capacity_bytes,self.last.released_bytes),"an allocation of this turn was not receipted in it")}
        match result{Ok(result)=>{if result.is_ok(){assert!(fixture_delta(reserved,self.last).fits(grant));assert!(self.spent.fits(self.stock));}result.map(|()|value.expect("original fixture callback returned its original value"))},Err(payload)=>std::panic::resume_unwind(payload)}
    }
    /// 🏦️ Funds the Source's own physical closure after its return is settled, without changing what its admissions could use.
    /// ⛽️ Funds the host budget that the owner-less retirement pump spends after the turns of this caller.
    /// 🏦️ Funds the stock shortfall of the Source root's own next closure demand, so a finite caller never starves its resident return table's close.
    pub fn fund_retirement(&mut self){let demand=self.retirement_demands().unwrap_or_default();let available=self.remaining_grant();let work=self.parent.as_ref().and_then(|parent|parent.retirement_work_demand().ok()).unwrap_or(0);self.fund_fuel(work.saturating_sub(self.fuel));self.fund_closure(demand.copy_bytes.saturating_sub(available.maximum_copy_bytes),demand.depth.saturating_sub(available.maximum_items))}
    pub fn fund_fuel(&mut self,fuel:u64){self.fuel=self.fuel.checked_add(fuel).expect("original fixture retirement fuel")}
    pub fn fund_closure(&mut self,copy_bytes:usize,items:usize){self.stock.maximum_copy_bytes=self.stock.maximum_copy_bytes.checked_add(copy_bytes).expect("original fixture closure funding");self.stock.maximum_items=self.stock.maximum_items.checked_add(items).expect("original fixture closure funding")}
    /// 🪢️ Lends the caller's whole unspent stock and fuel as one long-lived context; pair it with [`Self::settle_lent_fuel`] once the context is dropped.
    pub fn lend_context(&mut self)->StepContext<'_>{let parent=self.parent.as_ref().expect("original fixture parent remains retained while its context is lent");let budget=StepBudget::new(self.fuel,self.deadline,self.stock);let mut cx=parent.borrow_context(budget,self.clock,&mut self.sequence,&mut self.spent).expect("original fixture parent lends its single context");cx.bind_original_clock_binding(self.clock_binding).expect("original fixture clock binds to its genuine source context");cx}
    /// ⛽️ Records the fuel a lent context left, so later retirement spends only what remains.
    pub fn settle_lent_fuel(&mut self,remaining:u64){self.fuel=self.fuel.min(remaining)}
    pub fn stock(&self)->RetainedCloneGrant{self.stock}
    pub fn cumulative_receipt(&self)->RetainedCloneProgress{self.spent}
    pub fn remaining_grant(&self)->RetainedCloneGrant{fixture_remaining(self.stock,self.spent)}
    pub fn fuel_remaining(&self)->u64{self.fuel}
    pub fn terminal_is_empty(&self)->bool{self.parent.is_none()&&self.birth.terminal_is_empty()}
    pub fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{self.parent.as_ref().map_or(Ok(Default::default()),OriginalCallerSourceParent::retirement_demands)}
    pub fn receipt(&self)->RetainedCloneProgress{self.last}
    /// 🪢️ Runs one operation inside one actual original effect of a real caller turn so controlled and reply-shaped children see their producer invocation; an unaffordable grant hands the operation back untouched.
    pub fn with_effect<R,F:FnOnce(&mut RetainedAllocationEffect<'_>)->R>(&mut self,grant:RetainedCloneGrant,operation:F)->Result<R,F>{
        let mut operation=Some(operation);
        let value=self.turn(grant,|cx|{let Some((value,performed))=cx.with_original_effect(grant,|_,effect|(operation.take().expect("original fixture operation runs once"))(effect))? else{return Ok(None)};cx.consume_fuel(u64::try_from(performed).unwrap_or(u64::MAX));Ok(Some(value))}).expect("original fixture effect turn");
        value.ok_or_else(||operation.take().expect("unaffordable original fixture operation stays unrun"))
    }
    pub fn return_alias(&mut self,token:CancelToken,grant:RetainedCloneGrant){
        assert!(token.is_same_node(self.cancel()));let mut owner=semio_framework_async::CancelTokenRetirement::from_token(token);self.turn(grant,|cx|{let grant=cx.retained_grant();let Some((turn,_))=cx.with_original_effect(grant,|cx,effect|owner.return_alias_step(cx.original_cancel_token(),effect))?else{return Ok(())};let result=turn.result.map_err(|error|match error{semio_framework_async::CancelTokenRetirementError::Refused(error)=>error,semio_framework_async::CancelTokenRetirementError::Blocked(_)=>ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original fixture alias remains blocked")});semio_framework_job_context::receive_original_retained_work(cx,semio_framework_value_retained_clone::RetainedWorkOutcome{result,work:turn.work}).map(|_|())}).unwrap();assert!(owner.terminal_is_empty());
    }


    pub fn retire_step(&mut self,requested:RetainedCloneGrant)->Result<RetainedCloneStep,OriginalTurnFailure>{
        self.last=Default::default();if self.parent.is_none(){return Ok(RetainedCloneStep::Complete(Default::default()))}
        if requested.maximum_items==0||requested.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
        if !self.birth.terminal_is_empty()&&self.cancel().is_cancelled_now(){let mut birth=std::mem::take(&mut self.birth);let own=self.remaining_grant();let done=self.turn_original(own,None,false,|cx|Ok(birth.close_step(cx)?));self.birth=birth;done?;return Ok(RetainedCloneStep::Progress(self.last))}if self.fuel==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
        let available=self.remaining_grant();let grant=RetainedCloneGrant{maximum_items:requested.maximum_items.min(available.maximum_items).min(usize::try_from(self.fuel).unwrap_or(usize::MAX)),maximum_copy_bytes:requested.maximum_copy_bytes.min(available.maximum_copy_bytes),maximum_capacity_bytes:requested.maximum_capacity_bytes.min(available.maximum_capacity_bytes),maximum_release_bytes:requested.maximum_release_bytes.min(available.maximum_release_bytes),maximum_depth:requested.maximum_depth.min(available.maximum_depth)};
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
        let parent=&mut self.parent;let(outcome,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||semio_framework_job_context::OriginalRetirementPump::new().turn(self.fuel,self.deadline,self.clock,grant).and_then(|mut pump|pump.with_effect(|effect|{if parent.as_ref().unwrap().terminal_is_empty(){if !effect.performed(1).expect("original fixture Source metadata removal work"){return RetainedWorkPoll::Pending{work:0}}drop(parent.take());RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1})}else{match parent.as_mut().unwrap().close_step(effect){RetainedWorkPoll::Complete(turn)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map_err(OriginalTurnFailure::from),work:turn.work}),RetainedWorkPoll::Pending{work}=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work})}}})));
        let Some((poll,_))=outcome.expect("original fixture retirement pump") else{return Ok(RetainedCloneStep::Progress(Default::default()))};
        let RetainedWorkPoll::Complete(turn)=poll else{panic!("original fixture Source close stayed pending")};
        self.fuel=self.fuel.saturating_sub(u64::try_from(turn.work).unwrap_or(u64::MAX));let result=turn.result;let receipt=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),heap);self.last=receipt;self.born=self.born.checked_add(receipt.retained_capacity_bytes).expect("original close scratch accounting");self.roots_released=self.roots_released.checked_add(receipt.released_bytes).expect("original root release accounting");
        let debit=self.spent.checked_add(receipt).and_then(|total|{self.spent=total;if total.fits(self.stock)&&receipt.fits(grant){Ok(())}else{Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original fixture source close exceeds initiating stock").with_retained_progress(receipt))}});
        match(result,debit){(Ok(step),Ok(()))=>Ok(if self.parent.is_none(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())}),(Err(original),Ok(()))=>Err(original.into()),(Ok(_),Err(original))=>Err(original.into()),(Err(original),Err(refusal))=>{let mut failure=OriginalTurnFailure::from(original);failure.receipt_refusal=Some(refusal);Err(failure)}}
    }
    /// 🎯️ Closes every root with one grant per step equal to the exact quote of that step's own retirement demand.
    pub fn retire_exact(mut self)->usize{
        let mut turns=0;while !self.terminal_is_empty(){let d=self.retirement_demands().expect("original fixture root close quote");let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth.max(1)};let step=self.retire_step(grant).expect("original fixture source-root close");assert!(step.progress().fits(grant),"original fixture root close exceeded its exact quote");turns+=1;assert!(turns<=64,"original fixture root closure exhausted its initiating authority");}
        if self.audit{assert_eq!(self.spent.retained_capacity_bytes,self.spent.released_bytes,"every retained byte is released by its receipted close")}else{assert_eq!(self.roots_released,self.born,"the receipted close releases every born root")};self.roots_released
    }
    /// 🏦️ The grant widened to what the Source root's own closure demands of one closing turn, so a quote-sized grant never starves the caller's own retirement.
    fn closure_grant(&self,grant:RetainedCloneGrant)->RetainedCloneGrant{let closure=self.retirement_demands().unwrap_or_default();RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes.max(closure.copy_bytes),maximum_capacity_bytes:grant.maximum_capacity_bytes.max(closure.capacity_bytes),maximum_release_bytes:grant.maximum_release_bytes.max(closure.release_bytes),maximum_depth:grant.maximum_depth.max(closure.depth),..grant}}
    pub fn retire(mut self,source:RetainedCloneGrant)->usize{
        while !self.terminal_is_empty(){self.fund_retirement();let funded=self.closure_grant(source);let fuel=self.fuel;let step=self.retire_step(funded).expect("original fixture source-root close");assert!(step.progress().fits(funded));assert!(step.progress()!=RetainedCloneProgress::default()||self.fuel<fuel,"original fixture root closure stalled on its quoted funding: fuel {} funded {funded:?} remaining {:?} demand {:?} work {:?} clock {:?} deadline {} parent_empty {:?} birth_empty {}",self.fuel,self.remaining_grant(),self.retirement_demands(),self.parent.as_ref().map(|parent|parent.retirement_work_demand()),(self.clock)(),self.deadline,self.parent.as_ref().map(|parent|parent.terminal_is_empty()),self.birth.terminal_is_empty());}if self.audit{assert_eq!(self.spent.retained_capacity_bytes,self.spent.released_bytes,"every retained byte is released by its receipted close")}else{assert_eq!(self.roots_released,self.born,"the receipted close releases every born root")};self.roots_released
    }

}
impl Drop for OriginalJobFixtureCaller{fn drop(&mut self){assert!(std::thread::panicking()||self.parent.is_none(),"original fixed fixture caller requires paid terminal closure")}}

fn fixture_work_open(clock:fn()->Option<u64>,deadline:u64,fuel:u64,stride:&mut semio_framework_job_context::ClockStride)->bool{if fuel==0{return false}stride.begin(clock());stride.read(clock).is_some_and(|now|now<deadline)}
fn fixture_remaining(stock:RetainedCloneGrant,spent:RetainedCloneProgress)->RetainedCloneGrant{assert!(spent.fits(stock),"original fixture recipient exceeds its one issued stock");RetainedCloneGrant{maximum_items:stock.maximum_items-spent.copied_items,maximum_copy_bytes:stock.maximum_copy_bytes-spent.copied_bytes,maximum_capacity_bytes:stock.maximum_capacity_bytes-spent.retained_capacity_bytes,maximum_release_bytes:stock.maximum_release_bytes-spent.released_bytes,..stock}}
fn fixture_delta(before:RetainedCloneProgress,after:RetainedCloneProgress)->RetainedCloneProgress{RetainedCloneProgress{copied_items:after.copied_items.checked_sub(before.copied_items).expect("original fixture metadata recipient is monotone"),copied_bytes:after.copied_bytes.checked_sub(before.copied_bytes).expect("original fixture copy recipient is monotone"),retained_capacity_bytes:after.retained_capacity_bytes.checked_sub(before.retained_capacity_bytes).expect("original fixture capacity recipient is monotone"),released_bytes:after.released_bytes.checked_sub(before.released_bytes).expect("original fixture release recipient is monotone")}}

/// 🏦️ Runs one funded original action against the bank, posts its exact physical receipt once and returns the Work verdict.
pub fn bank_run<E>(bank:&mut EffectBank,issued:RetainedCloneGrant,work:u64,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<RetainedCloneStep,E>)->(RetainedWorkPoll<RetainedCloneStep,E>,Option<bool>){
    let((poll,_),heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||bank.run(issued,work,run));let admitted=bank.take_admitted();
    if let(RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(step),..}),Some(true))=(&poll,admitted){bank.post(step.progress(),issued,heap)}
    (poll,admitted)
}
/// 🪙️ Runs one quoted original birth or close step to completion under the bank and returns its progress.
pub fn bank_step(bank:&mut EffectBank,demand:RetirementDemand,work:u64,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome)->RetainedCloneStep{
    let issued=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
    let(poll,admitted)=bank_run(bank,issued,work,|effect|RetainedWorkPoll::Complete(run(effect)));let RetainedWorkPoll::Complete(turn)=poll else{panic!("original fixture step stayed pending")};assert_eq!(admitted,Some(true));turn.result.expect("original fixture step")
}
/// 🌱️ Births one Source parent over an existing payload authority and cancellation root under the bank's own wallet.
pub fn bank_source_parent(bank:&mut EffectBank,authority:JobPayloadAuthority,cancel:CancelToken)->OriginalCallerSourceParent{
    let mut authority=Some(authority);let mut cancel=Some(cancel);let mut admission=OriginalCallerSourceAdmission::new();let mut parent=None;
    bank_step(bank,OriginalCallerSourceAdmission::prepare_demand(),1,|effect|admission.prepare(&mut authority,&mut cancel,effect));
    bank_step(bank,OriginalCallerSourceAdmission::backing_demand().expect("original Source backing quote"),OriginalCallerSourceAdmission::backing_work_demand(),|effect|admission.admit_backing(effect));
    bank_step(bank,OriginalCallerSourceAdmission::publish_demand(),1,|effect|admission.publish(&mut parent,effect));
    parent.expect("original fixture Source parent published")
}

