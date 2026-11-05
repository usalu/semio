/// 🧯️ A real installed history disposer may refuse without relinquishing its original Store custody.
struct FlowCloseRefusalDisposer {
    original: Option<Box<dyn semio_os_kernel::os_store::ArtifactStoreOwnedDisposer<FlowHostSnapshot, FlowMutation>>>,
    refuse_once: bool,
}

impl FlowCloseRefusalDisposer {
    fn release_original(&mut self) -> RetainedCloneStep {
        let original = self.original.take().unwrap();
        let released_bytes = std::mem::size_of_val(original.as_ref());
        drop(original);
        RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() })
    }

    fn nested_demand(mut demand: semio_framework_value::RetirementDemand) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "test history disposer depth overflow"))?;
        Ok(demand)
    }

    fn child(grant: RetainedCloneGrant) -> RetainedCloneGrant {
        RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant }
    }
}

impl semio_os_kernel::os_store::ArtifactStoreOwnedDisposer<FlowHostSnapshot, FlowMutation> for FlowCloseRefusalDisposer {
    fn close_step(&mut self, view: &mut semio_os_kernel::os_store::ArtifactStoreCloseView<'_, FlowHostSnapshot, FlowMutation>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let demand = self.demands(&*view.store, grant.maximum_copy_bytes)?;
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        if grant.maximum_depth < demand.depth {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "test history disposer requires its quoted depth"));
        }
        if self.refuse_once {
            self.refuse_once = false;
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed, "injected history retirement allocation refusal"));
        }
        let Some(original) = self.original.as_mut() else { return Ok(RetainedCloneStep::Complete(Default::default())) };
        if original.terminal_is_empty(&*view.store) { return Ok(self.release_original()); }
        original.close_step(view, Self::child(grant)).map(|step| RetainedCloneStep::Progress(step.progress()))
    }

    fn terminal_is_empty(&self, _store: &dyn semio_os_kernel::os_store::ArtifactStoreCloseHost<FlowHostSnapshot, FlowMutation>) -> bool {
        self.original.is_none()
    }

    fn demands(&self, store: &dyn semio_os_kernel::os_store::ArtifactStoreCloseHost<FlowHostSnapshot, FlowMutation>, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        let Some(original) = self.original.as_ref() else { return Ok(Default::default()) };
        if original.terminal_is_empty(store) {
            return Ok(semio_framework_value::RetirementDemand { release_bytes: std::mem::size_of_val(original.as_ref()), depth: 1, ..Default::default() });
        }
        Self::nested_demand(original.demands(store, body)?)
    }

    fn close_uninstalled_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let demand = self.uninstalled_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        if grant.maximum_depth < demand.depth {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "test uninstalled history disposer requires its quoted depth"));
        }
        let Some(original) = self.original.as_mut() else { return Ok(RetainedCloneStep::Complete(Default::default())) };
        if original.uninstalled_terminal_is_empty() { return Ok(self.release_original()); }
        original.close_uninstalled_step(Self::child(grant)).map(|step| RetainedCloneStep::Progress(step.progress()))
    }

    fn uninstalled_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        let Some(original) = self.original.as_ref() else { return Ok(Default::default()) };
        if original.uninstalled_terminal_is_empty() {
            return Ok(semio_framework_value::RetirementDemand { release_bytes: std::mem::size_of_val(original.as_ref()), depth: 1, ..Default::default() });
        }
        Self::nested_demand(original.uninstalled_demands(body)?)
    }

    fn uninstalled_terminal_is_empty(&self) -> bool { self.original.is_none() }
}

/// 🏭️ Decorates the exact canonical history authority; no Store source or installed factory is fabricated.
fn flow_history_with_close_refusal(baseline: FlowHostSnapshot) -> Result<FlowStore, semio_os_kernel::os_vcs::VcsError> {
    let mut store = (semio_framework_artifact_flow_flow::FLOW_HISTORY_STORE_FACTORY.construct)(baseline)?;
    let original = store.owners.owned_disposer.take().unwrap();
    *store.owners.owned_disposer = Some(Box::new(FlowCloseRefusalDisposer { original: Some(original), refuse_once: true }));
    Ok(store)
}

/// 🏠️ A valid detached host with a canonical initialized history Store and one recoverable authority refusal.
fn flow_host_with_close_refusal() -> FlowHost {
    let mut host = FlowHost::from_host_snapshot(FlowHostSnapshot::default()).with_history_store_factory(semio_os_kernel::os_store::OriginalStoreGenesisFactory {
        construct: flow_history_with_close_refusal,
        envelope: semio_framework_artifact_flow_flow::FLOW_HISTORY_STORE_FACTORY.envelope,
    });
    let baseline = host.host_snapshot.clone();
    assert!(host.history_store_from_baseline(baseline).is_some());
    host
}

/// 🧾️ Reads the language-neutral contract with the independent third-party JSON parser.
fn flow_retirement_contract() -> serde_json::Value {
    serde_json::from_str(include_str!("contract.json")).unwrap()
}

/// 🎟️ Uses the same exact per-turn quote as the cold driver, without lowering any supplied currency.
fn flow_retirement_contract_grant(owner: &semio_os_flow_host_core::FlowHostRetirement) -> RetainedCloneGrant {
    let copy = owner.next_close_copy_byte_demand().unwrap().max(4096);
    let demand = owner.close_demands(copy).unwrap();
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth }
}

/// 🪜️ A recoverable installed-history refusal preserves the retained host and resumes the same custody.
#[test]
fn retained_host_close_preserves_custody_after_a_real_history_authority_refusal() {
    let law = flow_retirement_contract();
    let mut owner = semio_os_flow_host_core::FlowHostRetirement::new(flow_host_with_close_refusal());
    let mut refusals = 0;
    for _ in 0..law["maximumTurns"].as_u64().unwrap() {
        let grant = flow_retirement_contract_grant(&owner);
        match owner.close_step(grant) {
            Ok(step) => assert!(step.progress().fits(grant)),
            Err(error) => {
                assert_eq!(format!("{:?}", error.kind), law["refusalKind"].as_str().unwrap());
                assert!(error.retained_progress().fits(grant));
                assert!(!owner.terminal_is_empty());
                refusals += 1;
            }
        }
        if owner.terminal_is_empty() { break; }
    }
    assert_eq!(owner.terminal_is_empty(), law["retainedTerminal"].as_bool().unwrap());
    assert_eq!(refusals, law["expectedRefusals"].as_u64().unwrap());
}

/// 🧊️ The fallible cold API must surface a valid authority refusal instead of dropping nonterminal custody.
#[test]
fn cold_host_close_returns_a_real_history_authority_refusal_without_panicking() {
    let law = flow_retirement_contract();
    let result = flow_host_with_close_refusal().retire_cold();
    assert_eq!(result.is_err(), law["coldReturnsRefusal"].as_bool().unwrap());
    let refusal = match result { Err(refusal) => refusal, Ok(()) => panic!("original authority refusal was omitted") };
    let (error, mut owner) = refusal.into_parts();
    assert_eq!(format!("{:?}", error.kind), law["refusalKind"].as_str().unwrap());
    for _ in 0..law["maximumTurns"].as_u64().unwrap() { if owner.terminal_is_empty() { break; } let grant = flow_retirement_contract_grant(&owner); assert!(owner.close_step(grant).unwrap().progress().fits(grant)); }
    assert!(owner.terminal_is_empty());
}

#[test]
fn cold_host_scope_returns_the_same_body_output_and_refused_original_owner(){
 let law=flow_retirement_contract();let body=Box::new(String::from("original body outcome"));let pointer=body.as_ref()as*const String;let mut calls=0;
 let outcome=flow_host_with_close_refusal().with_cold(|_|{calls+=1;body});
 assert_eq!(outcome.output.as_ref()as*const String,pointer);assert_eq!(calls,law["bodyTransfers"].as_u64().unwrap());
 let(error,mut owner)=match outcome.close{Err(refusal)=>refusal.into_parts(),Ok(())=>panic!("original authority refusal was omitted")};
 assert_eq!(format!("{:?}",error.kind),law["refusalKind"].as_str().unwrap());
 for _ in 0..law["maximumTurns"].as_u64().unwrap(){if owner.terminal_is_empty(){break;}let grant=flow_retirement_contract_grant(&owner);assert!(owner.close_step(grant).unwrap().progress().fits(grant));}
 assert!(owner.terminal_is_empty());assert_eq!(outcome.output.as_ref()as*const String,pointer);
}

#[test]
fn funded_job_host_close_slot_keeps_original_custody_and_refusal_receipt(){
 let law=flow_retirement_contract();let mut host=Some(flow_host_with_close_refusal());let mut slot=FlowHostCloseSlot::default();
 assert!(slot.retain(&mut host,RetainedCloneGrant::default()).is_err());assert!(host.is_some());assert!(slot.terminal_is_empty());
 let d=FlowHostCloseSlot::intake_demand();let intake=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
 let(receipt,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||slot.retain(&mut host,intake).unwrap());assert!(receipt.fits(intake));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(host.is_none());
 let mut refusals=0;for _ in 0..law["maximumTurns"].as_u64().unwrap(){if slot.terminal_is_empty(){break;}let d=slot.next_demand(4096).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes.max(4096),maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
  let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||slot.close_step(grant));let receipt=match result{Ok(step)=>step.progress(),Err(error)=>{refusals+=1;assert_eq!(format!("{:?}",error.kind),law["refusalKind"].as_str().unwrap());assert!(!slot.terminal_is_empty());error.retained_progress()}};assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));
 }assert!(slot.terminal_is_empty());assert_eq!(refusals,law["expectedRefusals"].as_u64().unwrap());
}
