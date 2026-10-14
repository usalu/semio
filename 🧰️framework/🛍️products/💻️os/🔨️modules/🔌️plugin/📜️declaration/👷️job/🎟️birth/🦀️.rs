/// 🎟️ Actual concrete and shared producer birth retain original inputs on refused grants.
impl ArtifactReservedToolJob {
    /// 📏️ Quotes the exact inline gate and shared state allocation on the current target.
    pub fn original_shared_frame_bytes()->usize{semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<semio_framework_value::retirement::controlled::RetainedOwnerGate<ArtifactReservedToolJobState>>()}

    /// 📐️ Quotes only the shared frame when its concrete body already has physical custody.
    pub fn original_boxed_birth_demands()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<ArtifactReservedToolJobState>()+size_of::<Self>(),capacity_bytes:Self::original_shared_frame_bytes(),depth:1,..Default::default()}}

    /// 📊️ Quotes the exact concrete box and shared frame before typed source donation.
    pub fn original_birth_demands<J:ArtifactReservedJob+Send+'static>()->RetirementDemand{let shared=Self::original_boxed_birth_demands();RetirementDemand{copy_bytes:shared.copy_bytes+size_of::<J>(),capacity_bytes:shared.capacity_bytes+size_of::<J>(),..shared}}

    /// 🫴️ Receives a physically admitted original body only after shared frame permission.
    pub fn admit_boxed_original(source:&mut Option<Box<dyn ArtifactReservedJob+Send>>,receiving:&mut Option<Self>,cx:&mut StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        cx.original_source_parent()?;
        if receiving.is_some(){if source.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original reserved birth receiver already owns a different input").into())}return Ok(true)}
        let demand=Self::original_boxed_birth_demands();let grant=cx.retained_grant();
        if source.is_none()||cx.should_yield()||grant.maximum_items==0||!crate::bounded_ownership::plugin_grant_funds(demand,grant){return Ok(false)}
        let state=ArtifactReservedToolJobState{inner:ManuallyDrop::new(source.take()),operation:None};
        *receiving=Some(Self{state:ManuallyDrop::new(Some(std::sync::Arc::new(semio_framework_value::retirement::controlled::RetainedOwnerGate::new(state))))});
        cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)
    }

    /// 🧱️ Admits one exact concrete body box and its shared frame without a scratch allocation.
    pub fn admit_original<J:ArtifactReservedJob+Send+'static>(source:&mut Option<J>,receiving:&mut Option<Self>,cx:&mut StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        cx.original_source_parent()?;
        if receiving.is_some(){if source.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original reserved birth receiver already owns a different input").into())}return Ok(true)}
        let demand=Self::original_birth_demands::<J>();let grant=cx.retained_grant();
        if source.is_none()||cx.should_yield()||cx.fuel_remaining()<2||grant.maximum_items<2||!crate::bounded_ownership::plugin_grant_funds(demand,grant){return Ok(false)}
        let layout=std::alloc::Layout::new::<J>();
        let pointer=if layout.size()==0{std::ptr::NonNull::<J>::dangling()}else{std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}.cast::<J>()).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"original reserved concrete frame allocation refused"))?};
        let body:Box<dyn ArtifactReservedJob+Send>=unsafe{pointer.as_ptr().write(source.take().unwrap());Box::from_raw(pointer.as_ptr())};
        let state=ArtifactReservedToolJobState{inner:ManuallyDrop::new(Some(body)),operation:None};
        *receiving=Some(Self{state:ManuallyDrop::new(Some(std::sync::Arc::new(semio_framework_value::retirement::controlled::RetainedOwnerGate::new(state))))});
        cx.consume_retained(RetainedCloneProgress{copied_items:2,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()})?;cx.consume_fuel(2);Ok(true)
    }
}
