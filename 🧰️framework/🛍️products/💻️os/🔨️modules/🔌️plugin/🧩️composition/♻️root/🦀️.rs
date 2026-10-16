use semio_framework_plugin_ownership::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS;
use semio_framework_plugin_ownership::bounded_ownership::ArtifactFixedRegistry;
use semio_framework_plugin_child::ChildContentBorrowedOwners;
use semio_framework_plugin_child::ChildContentRetirement;
use semio_framework_plugin_decl_views::child_view::ChildContentView;
use semio_framework_plugin_child::ChildMemberRegistry;
use semio_framework_plugin_child::ChildMemberRetirement;
use semio_framework::Fault;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_value::retained_clone::RetainedCloneStep;
use semio_framework_value::RetirementDemand;
use store::SpaceMember;
use semio_framework_value::ValueError;
use semio_framework_plugin_child::original_content_step_owners;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_decl_views::child_view::ChildContentRetainers;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use store::MemberFactory;
/// 📐️ Quotes original child custody, disjoint borrowed retainers, or a separate terminal frame.
#[doc(hidden)]
pub fn original_child_root_retirement_demands<M: SpaceMember>(registry: &ArtifactFixedRegistry<ChildContentRetirement>, children: &ChildMemberRegistry<M>, retiring: Option<&ArtifactFixedRegistry<ChildMemberRetirement<M>>>, cursor: usize, body: usize, closing: bool) -> Result<Option<RetirementDemand>, ValueError> {
    let Some((_, generation)) = registry.next_id_from(cursor) else {
        let bytes = registry.empty_backing_byte_demand().unwrap_or(0);
        return Ok((closing && bytes != 0).then_some(RetirementDemand { copy_bytes: size_of::<ArtifactFixedRegistry<ChildContentRetirement>>(), release_bytes: bytes, depth: 1, ..Default::default() }));
    };
    let retirement = registry.get(generation).expect("selected original child retirement remains occupied");
    if retirement.terminal_is_empty() { return Ok(Some(RetirementDemand { copy_bytes: size_of::<(u64, ChildContentRetirement)>(), depth: 1, ..Default::default() })); }
    let reserved = ChildContentBorrowedOwners::copy_reservation();
    let demand = retirement.retirement_demands(children, retiring, body.saturating_sub(reserved))?;
    let copy_bytes = reserved.checked_add(demand.copy_bytes).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "original child retirement copy extent overflowed"))?;
    let depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "original child retirement depth overflowed"))?;
    Ok(Some(RetirementDemand { copy_bytes, depth, ..demand }))
}

/// 🌳️ Advances one paid original child unit, retaining registry custody after child completion.
#[doc(hidden)]
pub fn advance_original_child_root_retirement<M: SpaceMember>(registry: &mut ArtifactFixedRegistry<ChildContentRetirement>, children: &mut ChildMemberRegistry<M>, retiring: Option<&mut ArtifactFixedRegistry<ChildMemberRetirement<M>>>, current: &ChildContentView, member: Option<&ChildContentView>, review: Option<&ChildContentView>, cursor: &mut usize, grant: RetainedCloneGrant, closing: bool) -> Result<RetainedCloneStep, Fault> {
    let Some(demand) = original_child_root_retirement_demands(registry, children, retiring.as_deref(), *cursor, grant.maximum_copy_bytes, closing).map_err(|error| Fault::from(error.into_message()))? else { return Ok(RetainedCloneStep::Complete(Default::default())); };
    if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth { return Ok(RetainedCloneStep::Progress(Default::default())); }
    let Some((index, generation)) = registry.next_id_from(*cursor) else {
        let released_bytes = registry.empty_backing_byte_demand().expect("original empty child registry remains empty");
        let backing = std::mem::take(&mut registry.slots);
        registry.allocation_admitted = false;
        drop(backing);
        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<ArtifactFixedRegistry<ChildContentRetirement>>(), released_bytes, ..Default::default() }));
    };
    if registry.get(generation).is_some_and(ChildContentRetirement::terminal_is_empty) {
        let retirement = registry.remove(generation).expect("terminal original child registry identity remains occupied");
        drop(retirement);
        *cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<(u64, ChildContentRetirement)>(), ..Default::default() }));
    }
    let reserved = ChildContentBorrowedOwners::copy_reservation();
    let child_grant = RetainedCloneGrant { maximum_copy_bytes:grant.maximum_copy_bytes-reserved, maximum_depth:grant.maximum_depth-1, ..grant };
    let (retirement, owners) = original_content_step_owners(registry, generation, member, review).expect("original occupied child registry supplies disjoint retainers");
    let step = retirement.close_step(children, retiring, current, &owners, child_grant)?;
    let step = match step { RetainedCloneStep::Complete(progress)=>RetainedCloneStep::Progress(progress), step=>step };
    let child = step.progress();
    if !child.fits(child_grant) {
        return Err(Fault::from("original child retirement exceeded its remaining five-axis caller grant"));
    }
    *cursor = index;
    Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_bytes: child.copied_bytes + reserved, ..child }))
}

