//! 📦️ Original native resources move with their component tree and retain the same arena pages.

use semio_framework_job::{RetainedCloneGrant, RetainedCloneProgress, RetirementDemand};
pub use ui_scene::math::{Mesh3dResourceIdentity, Mesh3dResourceLoan, Mesh3dResourceOwner};
use ui_scene::math::{Mesh3dFault, Mesh3dOwnershipStep};

#[path = "🤝️loans/🦀️.rs"]
mod loans;
pub use loans::*;

pub const UI_NATIVE_RESOURCE_CAPACITY: usize = ui_scene::math::MESH3D_AUTHORITY_CAPACITY;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiNativeResourceFailure {
    Mesh(Mesh3dFault),
    Storage(ui_contract::PagedListError),
    Allocation(ui_contract::PagedListAllocationError),
}

impl UiNativeResourceFailure {
    pub fn retained_progress(self) -> RetainedCloneProgress {
        match self { Self::Allocation(error) => RetainedCloneProgress { copied_items: usize::from(error.allocated_bytes != 0), retained_capacity_bytes: error.allocated_bytes, ..Default::default() }, _ => RetainedCloneProgress::default() }
    }
}

#[derive(Debug)]
pub struct UiNativeMeshResource {
    pub surface: ui_contract::UiText,
    pub mesh_key: ui_contract::UiText,
    pub owner: Mesh3dResourceOwner,
}

#[derive(Debug, Default)]
pub struct UiNativeResources {
    rows: ui_contract::UiFixedList<UiNativeMeshResource, UI_NATIVE_RESOURCE_CAPACITY>,
    pending_failure: Option<UiNativeResourceTurnFailure>,
}

impl UiNativeResources {
    pub fn len(&self) -> usize { self.rows.len() }

    pub fn row(&self, index: usize) -> Option<&UiNativeMeshResource> { self.rows.get(index) }

    pub fn admission_demands(&self) -> Result<RetirementDemand, UiNativeResourceFailure> {
        Ok(RetirementDemand { capacity_bytes: if self.rows.has_reserved_slot() { 0 } else { self.rows.next_allocation_bytes().map_err(UiNativeResourceFailure::Storage)? }, depth: 1, ..Default::default() })
    }

    pub fn admit_row(&mut self, source: &mut Option<UiNativeMeshResource>, grant: RetainedCloneGrant) -> Result<Mesh3dOwnershipStep, UiNativeResourceFailure> {
        if source.is_none() { return Ok(Mesh3dOwnershipStep { complete: true, ..Default::default() }); }
        if self.rows.len() == UI_NATIVE_RESOURCE_CAPACITY { return Err(UiNativeResourceFailure::Mesh(Mesh3dFault::ItemCapacity)); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(Mesh3dOwnershipStep::default()); }
        if !self.rows.has_reserved_slot() {
            let bytes = self.rows.next_allocation_bytes().map_err(UiNativeResourceFailure::Storage)?;
            if grant.maximum_capacity_bytes < bytes { return Ok(Mesh3dOwnershipStep::default()); }
            let step = self.rows.try_reserve_one(grant.maximum_capacity_bytes).map_err(UiNativeResourceFailure::Allocation)?;
            return Ok(Mesh3dOwnershipStep { progress: RetainedCloneProgress { copied_items: usize::from(step.progressed), retained_capacity_bytes: step.allocated_bytes, ..Default::default() }, ..Default::default() });
        }
        if let Err(original) = self.rows.try_push_reserved(source.take().unwrap()) {
            *source = Some(original);
            return Err(UiNativeResourceFailure::Mesh(Mesh3dFault::ItemCapacity));
        }
        Ok(Mesh3dOwnershipStep { complete: true, progress: RetainedCloneProgress { copied_items: 1, ..Default::default() } })
    }

    pub fn close_demands(&self, work: usize) -> Result<RetirementDemand, UiNativeResourceFailure> {
        if let Some(row) = self.rows.len().checked_sub(1).and_then(|index| self.rows.get(index)) {
            if !row.owner.terminal_is_empty() { return row.owner.close_demands(work).map_err(UiNativeResourceFailure::Mesh); }
            return Ok(RetirementDemand { depth: self.rows.next_pop_depth_demand().map_err(UiNativeResourceFailure::Storage)?, ..Default::default() });
        }
        Ok(RetirementDemand { release_bytes: self.rows.next_release_allocation_bytes().map_err(UiNativeResourceFailure::Storage)?, depth: self.rows.next_release_depth_demand().map_err(UiNativeResourceFailure::Storage)?, ..Default::default() })
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<Mesh3dOwnershipStep, UiNativeResourceFailure> {
        if self.terminal_is_empty() { return Ok(Mesh3dOwnershipStep { complete: true, ..Default::default() }); }
        if let Some(row) = self.rows.last_mut() {
            if !row.owner.terminal_is_empty() { let mut step = row.owner.close_step(grant).map_err(UiNativeResourceFailure::Mesh)?; step.complete = false; return Ok(step); }
            let depth = self.rows.next_pop_depth_demand().map_err(UiNativeResourceFailure::Storage)?;
            if grant.maximum_items == 0 || grant.maximum_depth < depth { return Ok(Mesh3dOwnershipStep::default()); }
            self.rows.pop();
            return Ok(Mesh3dOwnershipStep { progress: RetainedCloneProgress { copied_items: 1, ..Default::default() }, ..Default::default() });
        }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || grant.maximum_release_bytes < demand.release_bytes { return Ok(Mesh3dOwnershipStep::default()); }
        let step = self.rows.release_empty_page(grant.maximum_release_bytes).map_err(UiNativeResourceFailure::Storage)?;
        Ok(Mesh3dOwnershipStep { complete: self.terminal_is_empty(), progress: RetainedCloneProgress { copied_items: usize::from(step.progressed), released_bytes: step.released_allocation_bytes, ..Default::default() } })
    }

    pub fn close_step_on_original(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<bool>, semio_framework_job::OriginalTurnFailure> {
        if self.pending_failure.is_some() { return admit_native_resource_failure(&mut self.pending_failure, cx); }
        if self.rows.terminal_is_empty() { return Ok(Some(true)); }
        let grant = cx.retained_grant();
        if cx.should_yield() || grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(None); }
        let demand = match self.close_demands(grant.maximum_copy_bytes) {
            Ok(demand) => demand,
            Err(primary) => { self.pending_failure = Some(UiNativeResourceTurnFailure::Native { primary, receipt_refusal: None }); return admit_native_resource_failure(&mut self.pending_failure, cx); }
        };
        if demand.copy_bytes > grant.maximum_copy_bytes || demand.capacity_bytes > grant.maximum_capacity_bytes || demand.release_bytes > grant.maximum_release_bytes || demand.depth > grant.maximum_depth { return Ok(None); }
        let result = self.close_step(grant);
        let progress = match &result { Ok(step) => step.progress, Err(primary) => primary.retained_progress() };
        if progress != RetainedCloneProgress::default() || result.is_err() { cx.consume_fuel(1); }
        let receipt_refusal = cx.consume_retained(progress).err();
        match (result, receipt_refusal) {
            (Ok(_), None) => Ok(Some(self.terminal_is_empty())),
            (Err(primary), receipt_refusal) => { self.pending_failure = Some(UiNativeResourceTurnFailure::Native { primary, receipt_refusal }); Ok(None) },
            (Ok(_), Some(primary)) => { self.pending_failure = Some(UiNativeResourceTurnFailure::Receipt(primary)); Ok(None) },
        }
    }

    pub fn terminal_is_empty(&self) -> bool { self.rows.terminal_is_empty() && self.pending_failure.is_none() }
}

impl Drop for UiNativeResources {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "component native resources dropped before original paid close"); }
}


semio_framework_value::artifact_retire_leaf!(UiNativeResourceFailure);

impl semio_framework_job::OriginalDomainFailure for UiNativeResourceFailure {
    fn domain_code(&self) -> &str { match self { Self::Mesh(_) => "ui.nativeResource.mesh", Self::Storage(_) => "ui.nativeResource.storage", Self::Allocation(_) => "ui.nativeResource.allocation" } }
    fn domain_message(&self) -> &str { match self { Self::Mesh(fault) => mesh_fault_name(*fault), Self::Storage(fault) => fault.reason, Self::Allocation(fault) => fault.reason } }
    fn domain_retained_progress(&self) -> RetainedCloneProgress { RetainedCloneProgress::default() }
}

fn mesh_fault_name(fault: Mesh3dFault) -> &'static str {
    match fault { Mesh3dFault::Busy => "busy", Mesh3dFault::Closing => "closing", Mesh3dFault::ItemCapacity => "itemCapacity", Mesh3dFault::PageCapacity => "pageCapacity", Mesh3dFault::ByteCapacity => "byteCapacity", Mesh3dFault::Schema => "schema", Mesh3dFault::Stale => "stale", Mesh3dFault::Order => "order", Mesh3dFault::Incomplete => "incomplete" }
}

fn native_projection_refusal() -> semio_framework_value::ValueError {
    semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "native resource failure projection ordinal changed")
}

impl semio_framework_job::OriginalDomainProjection for UiNativeResourceFailure {
    fn projection_view(&self, path: &[usize]) -> Result<semio_framework_job::OriginalDomainView<'_>, semio_framework_value::ValueError> {
        use semio_framework_job::OriginalDomainView as V;
        match (self, path) {
            (_, []) => Ok(V::IntrinsicObject(2)),
            (Self::Mesh(_), [0]) => Ok(V::Text("mesh")),
            (Self::Storage(_), [0]) => Ok(V::Text("storage")),
            (Self::Allocation(_), [0]) => Ok(V::Text("allocation")),
            (Self::Mesh(fault), [1]) => Ok(V::Text(mesh_fault_name(*fault))),
            (Self::Storage(_), [1]) => Ok(V::IntrinsicObject(2)),
            (Self::Allocation(_), [1]) => Ok(V::IntrinsicObject(3)),
            (Self::Storage(fault), [1, 0]) => Ok(V::Text(fault.kind.as_str())),
            (Self::Allocation(fault), [1, 0]) => Ok(V::Text(fault.kind.as_str())),
            (Self::Storage(fault), [1, 1]) => Ok(V::Text(fault.reason)),
            (Self::Allocation(fault), [1, 1]) => Ok(V::Text(fault.reason)),
            (Self::Allocation(fault), [1, 2]) => Ok(V::UInt(fault.allocated_bytes as u64)),
            _ => Err(native_projection_refusal()),
        }
    }
    fn projection_key(&self, path: &[usize], index: usize) -> Result<&str, semio_framework_value::ValueError> {
        let keys: &[&str] = match (self, path) { (_, []) => &["kind", "source"], (Self::Storage(_), [1]) => &["kind", "reason"], (Self::Allocation(_), [1]) => &["kind", "reason", "allocatedBytes"], _ => return Err(native_projection_refusal()) };
        keys.get(index).copied().ok_or_else(native_projection_refusal)
    }
}

#[derive(Debug, semio_framework_value::RetireOwned)]
enum UiNativeResourceTurnFailure {
    Native { primary: UiNativeResourceFailure, receipt_refusal: Option<semio_framework_value::ValueError> },
    Receipt(semio_framework_value::ValueError),
}

impl semio_framework_job::OriginalDomainFailure for UiNativeResourceTurnFailure {
    fn domain_code(&self) -> &str { match self { Self::Native { primary, .. } => semio_framework_job::OriginalDomainFailure::domain_code(primary), Self::Receipt(_) => "ui.nativeResource.receipt" } }
    fn domain_message(&self) -> &str { match self { Self::Native { primary, .. } => semio_framework_job::OriginalDomainFailure::domain_message(primary), Self::Receipt(primary) => &primary.message } }
    fn domain_retained_progress(&self) -> RetainedCloneProgress { RetainedCloneProgress::default() }
}

impl semio_framework_job::OriginalDomainProjection for UiNativeResourceTurnFailure {
    fn projection_view(&self, path: &[usize]) -> Result<semio_framework_job::OriginalDomainView<'_>, semio_framework_value::ValueError> {
        use semio_framework_job::OriginalDomainView as V;
        match (self, path) {
            (Self::Native { receipt_refusal, .. }, []) => Ok(V::IntrinsicObject(1 + usize::from(receipt_refusal.is_some()))),
            (Self::Receipt(_), []) => Ok(V::IntrinsicObject(1)),
            (Self::Native { primary, .. }, [0, rest @ ..]) => semio_framework_job::OriginalDomainProjection::projection_view(primary, rest),
            (Self::Native { receipt_refusal: Some(refusal), .. }, [1, rest @ ..]) | (Self::Receipt(refusal), [0, rest @ ..]) => semio_framework_job::project_original_value_error(refusal, rest),
            _ => Err(native_projection_refusal()),
        }
    }
    fn projection_key(&self, path: &[usize], index: usize) -> Result<&str, semio_framework_value::ValueError> {
        match (self, path) {
            (Self::Native { receipt_refusal, .. }, []) => ["primary", "receiptRefusal"].get(index).copied().filter(|_| index == 0 || receipt_refusal.is_some()).ok_or_else(native_projection_refusal),
            (Self::Receipt(_), []) if index == 0 => Ok("primary"),
            (Self::Native { primary, .. }, [0, rest @ ..]) => semio_framework_job::OriginalDomainProjection::projection_key(primary, rest, index),
            (Self::Native { receipt_refusal: Some(refusal), .. }, [1, rest @ ..]) | (Self::Receipt(refusal), [0, rest @ ..]) => semio_framework_job::key_original_value_error(refusal, rest, index),
            _ => Err(native_projection_refusal()),
        }
    }
}

fn admit_native_resource_failure(source: &mut Option<UiNativeResourceTurnFailure>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<bool>, semio_framework_job::OriginalTurnFailure> {
    match semio_framework_job::admit_original_domain_failure(source, cx)? { Some(failure) => Err(failure), None => Ok(None) }
}
