//! 🧪️ Test and testkit hooks that reach private StepContext state; compiled only for tests and the `testkit` feature.
    use super::*;
    thread_local!{static NEXT_LOAN_REFUSAL:std::cell::Cell<bool>=const{std::cell::Cell::new(false)};static NEXT_CANCEL_REFUSAL:std::cell::Cell<bool>=const{std::cell::Cell::new(false)};}
    pub use semio_framework_job_receiving::testkit::{with_native_decode_progress,observe_native_decode_progress};
    pub fn set_loan_receipt_refusal(value:bool){NEXT_LOAN_REFUSAL.with(|flag|flag.set(value))}
    pub fn set_cancel_receipt_refusal(value:bool){NEXT_CANCEL_REFUSAL.with(|flag|flag.set(value))}
    pub(crate) fn narrow_loan_receipt(cx:&mut StepContext<'_>){if NEXT_LOAN_REFUSAL.with(|flag|flag.replace(false)){cx.retained.maximum_items=cx.retained_progress().copied_items}}
    pub(crate) fn narrow_cancel_receipt(cx:&mut StepContext<'_>){if NEXT_CANCEL_REFUSAL.with(|flag|flag.replace(false)){cx.retained_policy=Some((RetainedCloneGrant::default(),cx.retained_progress()))}}
    pub fn renew_payload_page_grant(cx:&mut StepContext<'_>){cx.payload_page_granted=false}

    fn renew_local_turn(cx: &mut StepContext<'_>, fuel: u64, grant: RetainedCloneGrant, current: RetainedCloneProgress) {
        cx.fuel_remaining = fuel;
        cx.payload_page_granted = false;
        cx.retained = RetainedCloneGrant {
            maximum_items: current.copied_items.saturating_add(grant.maximum_items),
            maximum_copy_bytes: current.copied_bytes.saturating_add(grant.maximum_copy_bytes),
            maximum_capacity_bytes: current.retained_capacity_bytes.saturating_add(grant.maximum_capacity_bytes),
            maximum_release_bytes: current.released_bytes.saturating_add(grant.maximum_release_bytes),
            maximum_depth: grant.maximum_depth,
        };
    }

    /// 🧷️ Refreshes the fixture's local turn while the original return stock and its claims stay live.
    pub fn renew_pinned_turn(cx: &mut StepContext<'_>, fuel: u64, grant: RetainedCloneGrant) -> Result<(), ValueError> {
        let budget = cx.return_budget.as_ref().filter(|budget| budget.get().is_bound() && !budget.get().renewable())
            .ok_or_else(|| ValueError::invariant("fixture turn requires a pinned original return source"))?;
        budget.get().remaining()?;
        let current = retained_delta(*cx.retained_progress, cx.reserved_return_progress);
        let fuel = fuel.min(cx.original_total_fuel_remaining());
        let grant = retained_min(grant, cx.original_source_total_grant());
        renew_local_turn(cx, fuel, grant, current);
        Ok(())
    }

    pub fn renew_turn(cx:&mut StepContext<'_>,fuel:u64,grant:RetainedCloneGrant)->Result<(),ValueError>{
        let current=retained_delta(*cx.retained_progress,cx.reserved_return_progress);
        if let Some(budget)=cx.return_budget.as_mut().filter(|budget|budget.get().is_bound()){budget.get_mut().renew_turn()?}
        let refill=fuel.saturating_sub(cx.fuel_remaining);
        renew_local_turn(cx, fuel, grant, current);
        cx.original_total_retained_start=current;
        match cx.return_budget.as_ref().filter(|budget|budget.get().is_bound()){
            Some(budget)=>{let(retained,total)=budget.get().remaining()?;cx.original_total_retained=retained;cx.original_total_fuel=total;cx.original_total_work_start=cx.work_units_spent.saturating_sub(u128::from(cx.reserved_return_work));}
            None=>{cx.original_total_fuel=cx.original_total_fuel.saturating_add(refill);}
        }
        Ok(())
    }
    thread_local!{static ISSUED_EVENT_STOCKS:std::cell::RefCell<Vec<(u64,RetainedCloneGrant)>>=const{std::cell::RefCell::new(Vec::new())};}
    /// 🎫️ Issues the context one fresh initiating-event stock as the host does per native event: the previous event must have stayed within its own, and the stock now issued is recorded under the context's clock binding.
    pub fn issue_initiating_event(cx:&mut StepContext<'_>,grant:RetainedCloneGrant)->Result<(),ValueError>{
        let current=retained_delta(*cx.retained_progress,cx.reserved_return_progress);
        if !retained_delta(current,cx.issued_start).fits(cx.issued_ceiling.0){return Err(ValueError::invariant("the previous initiating event exceeded its issued stock"))}
        let raw=*cx.retained_progress;let add=|spent:usize,issued:usize|spent.checked_add(issued).ok_or_else(||ValueError::invariant("issued initiating-event stock overflows"));
        let issued=RetainedCloneGrant{maximum_items:add(raw.copied_items,grant.maximum_items)?,maximum_copy_bytes:add(raw.copied_bytes,grant.maximum_copy_bytes)?,maximum_capacity_bytes:add(raw.retained_capacity_bytes,grant.maximum_capacity_bytes)?,maximum_release_bytes:add(raw.released_bytes,grant.maximum_release_bytes)?,maximum_depth:grant.maximum_depth};
        cx.issued_start=current;cx.issued_ceiling.0=grant;
        if let Some(binding)=cx.original_clock_binding{ISSUED_EVENT_STOCKS.with(|stocks|{let mut stocks=stocks.borrow_mut();stocks.retain(|(known,_)|*known!=binding);stocks.push((binding,issued))})}
        Ok(())
    }
    /// 🧾️ Takes the absolute stock last issued to the context bound to `binding`, if any initiating event was issued to it.
    pub fn take_issued_event_stock(binding:u64)->Option<RetainedCloneGrant>{ISSUED_EVENT_STOCKS.with(|stocks|{let mut stocks=stocks.borrow_mut();let index=stocks.iter().position(|(known,_)|*known==binding)?;Some(stocks.swap_remove(index).1)})}
    pub fn source_return_budget_is_bound(parent:&OriginalCallerSourceParent)->Result<bool,ValueError>{Ok(parent.return_source()?.try_borrow().ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"actual source budget is borrowed"))?.get().is_bound())}
    pub fn return_budget_is_inherited(cx:&StepContext<'_>)->bool{matches!(cx.return_budget.as_ref(),Some(original_return_source::OriginalReturnBudgetReference::Borrowed(_)))}

    pub use super::original_job_continuation::source as original_return_source;
    pub fn operation_mut<'a>(cx:&'a mut StepContext<'_>)->&'a mut OperationId{&mut cx.operation}
    pub fn retained_grant_mut<'a>(cx:&'a mut StepContext<'_>)->&'a mut RetainedCloneGrant{&mut cx.retained}
    pub fn retained_progress_mut<'a>(cx:&'a mut StepContext<'_>)->&'a mut RetainedCloneProgress{cx.retained_progress}
    /// 🔁️ The snapshot of the actual Source budget once bound and pinned by a live claim or held currency, which then cannot renew its turn.
    pub fn return_source_pinned(cx:&StepContext<'_>)->Option<original_return_source::OriginalReturnBudgetSnapshot>{cx.return_budget.as_ref().filter(|budget|budget.get().is_bound()&&!budget.get().renewable()).map(|budget|budget.get().snapshot())}
    pub fn return_budget_snapshot(cx:&StepContext<'_>)->original_return_source::OriginalReturnBudgetSnapshot{cx.return_budget.as_ref().expect("actual original source budget").get().snapshot()}
    pub fn request_return_publisher(cx:&mut StepContext<'_>){if let Some(budget)=cx.return_budget.as_mut(){budget.get_mut().publisher_requested=true}}
    pub fn return_table_present(cx:&StepContext<'_>)->bool{cx.return_budget.as_ref().is_some_and(|budget|budget.get().table().is_some())}
    pub fn return_budget_is_bound(cx:&StepContext<'_>)->bool{cx.return_budget.as_ref().is_some_and(|budget|budget.get().is_bound())}
    pub fn original_total_retained_start(cx:&StepContext<'_>)->RetainedCloneProgress{cx.original_total_retained_start}
    pub fn return_source_is_borrowed(parent:&OriginalCallerSourceParent)->Result<bool,ValueError>{Ok(parent.return_source()?.try_borrow().is_none())}
    pub fn admit_return_origin(cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{cx.original_source_parent()?.admit_return_origin(cx)}

    pub fn with_original_work_refusals<T>(cx:&mut StepContext<'_>,operation:Option<OperationId>,capacity:Option<usize>,run:impl FnOnce(&mut StepContext<'_>)->T)->T{
        struct Restore<'a>{context:*mut StepContext<'a>,operation:OperationId,retained:RetainedCloneGrant}
        impl Drop for Restore<'_>{fn drop(&mut self){unsafe{(*self.context).operation=self.operation;(*self.context).retained=self.retained}}}
        let restore=Restore{context:std::ptr::addr_of_mut!(*cx),operation:cx.operation,retained:cx.retained};
        if let Some(operation)=operation{cx.operation=operation}if let Some(capacity)=capacity{cx.retained.maximum_capacity_bytes=capacity}
        let value=run(cx);drop(restore);value
    }

    pub fn with_retained_receipt_policy<T>(cx:&mut StepContext<'_>,run:impl FnOnce(&mut StepContext<'_>)->T)->T{let original=cx.retained_policy;let result=run(cx);cx.retained_policy=original;result}
    pub fn refuse_current_retained_receipt(cx:&mut StepContext<'_>){cx.retained_policy=Some((RetainedCloneGrant{maximum_items:0,..cx.retained_grant()},cx.retained_progress()));}

    pub fn payload_ledger(operation:OperationId,generation:Generation)->Arc<JobPayloadOperationLedger>{Arc::new(JobPayloadOperationLedger::new(operation,generation))}
    pub fn ledger_demands(ledger:&Option<Arc<JobPayloadOperationLedger>>)->RetirementDemand{semio_framework_job_payload::ledger_demands(ledger)}
    pub fn close_ledger(ledger:&mut Option<Arc<JobPayloadOperationLedger>>,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->semio_framework_value_retained_clone::RetainedWorkOutcome{semio_framework_job_payload::close_ledger(ledger,effect)}
