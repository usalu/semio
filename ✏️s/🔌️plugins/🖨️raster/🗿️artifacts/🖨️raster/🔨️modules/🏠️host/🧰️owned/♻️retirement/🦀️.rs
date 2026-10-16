//! ♻️ raster retirement: the snapshot and mutation retirement factories, the displaced value, the retirement slot and the grant helpers.

use super::*;

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct RasterSnapshotRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<RasterSnapshot> for RasterSnapshotRetirementFactory {
    fn retirement_birth_bytes(&self, _value: &RasterSnapshot) -> usize {
        semio_framework_value::retirement::owned_retirement_birth_bytes::<RasterSnapshot>()
    }

    fn retire_owned(&self, value: RasterSnapshot, grant: RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, RasterSnapshot)> {
        semio_framework_value::retirement::admit_owned_retirement(value, grant)
    }
}

impl store::SnapshotRetirementFactory<RasterSnapshot> for RasterSnapshotRetirementFactory {
    fn retirement_birth_bytes(&self, _snapshot: &std::sync::Arc<RasterSnapshot>) -> usize {
        semio_framework_value::retirement::shared::shared_retirement_birth_bytes::<RasterSnapshot>()
    }

    fn retire(&self, snapshot: std::sync::Arc<RasterSnapshot>, grant: RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, std::sync::Arc<RasterSnapshot>)> {
        semio_framework_value::retirement::shared::admit_shared_retirement(snapshot, grant, true)
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct RasterMutationRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<RasterMutation> for RasterMutationRetirementFactory {
    fn retirement_birth_bytes(&self, _value: &RasterMutation) -> usize {
        semio_framework_value::retirement::owned_retirement_birth_bytes::<RasterMutation>()
    }

    fn retire_owned(&self, value: RasterMutation, grant: RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, RasterMutation)> {
        semio_framework_value::retirement::admit_owned_retirement(value, grant)
    }
}

/// ♻️ Every value a stepwise Raster authority displaces or abandons; the framework's owned retirement closes it under an explicit grant.
#[derive(semio_framework_value::RetireOwned)]
pub(super) enum RasterDisplaced {
    String(String),
    Layer(RasterLayerNode),
    Snapshot(RasterSnapshot),
    AssetEntry((String, RasterAssetChild)),
    ValueEntry((String, semio_framework_value::DslValue)),
    Value(semio_framework_value::DslValue),
}

/// 🗑️ One displaced owner and its admitted retirement frame; the owner stays in `pending` until a grant funds its frame.
pub(super) struct RasterRetirementSlot {
    pub(super) pending: std::mem::ManuallyDrop<Option<RasterDisplaced>>,
    pub(super) active: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl RasterRetirementSlot {
    pub(super) fn new() -> Self {
        Self { pending: std::mem::ManuallyDrop::new(None), active: std::mem::ManuallyDrop::new(None) }
    }

    pub(super) fn put(&mut self, owner: RasterDisplaced) {
        assert!(self.is_empty(), "Raster retirement slot admits one displaced owner at a time");
        *self.pending = Some(owner);
    }

    pub(super) fn is_empty(&self) -> bool {
        self.pending.is_none() && self.active.is_none()
    }

    pub(super) fn birth_demand() -> semio_framework_value::RetirementDemand {
        semio_framework_value::RetirementDemand { capacity_bytes: semio_framework_value::retirement::owned_retirement_birth_bytes::<RasterDisplaced>(), depth: 2, ..Default::default() }
    }

    pub(super) fn demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if let Some(active) = self.active.as_ref() {
            return store::os_store::snapshot_read::artifact_retirement_box_demands(active, body);
        }
        if self.pending.is_some() {
            return Ok(Self::birth_demand());
        }
        Ok(Default::default())
    }

    pub(super) fn close(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.is_empty() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth {
            return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "Raster displaced owner exceeds admitted depth"));
        }
        if raster_short(grant, demand) {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let step = if self.active.is_some() { store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active, grant)? } else { store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.pending, &mut self.active, grant)? };
        Ok(RetainedCloneStep::Progress(step.progress()))
    }
}

impl Drop for RasterRetirementSlot {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.is_empty(), "Raster retirement slot reached Drop before its displaced owner retired");
        if self.is_empty() {
            unsafe {
                std::mem::ManuallyDrop::drop(&mut self.pending);
                std::mem::ManuallyDrop::drop(&mut self.active);
            }
        }
    }
}

pub(super) fn raster_short(grant: RetainedCloneGrant, demand: semio_framework_value::RetirementDemand) -> bool {
    grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes
}

pub(super) fn raster_finish(step: RetainedCloneStep, terminal: bool) -> RetainedCloneStep {
    match step {
        RetainedCloneStep::Complete(progress) | RetainedCloneStep::Progress(progress) if terminal => RetainedCloneStep::Complete(progress),
        RetainedCloneStep::Complete(progress) | RetainedCloneStep::Progress(progress) => RetainedCloneStep::Progress(progress),
    }
}

pub(super) fn raster_nested(mut demand: semio_framework_value::RetirementDemand) -> Result<semio_framework_value::RetirementDemand, ValueError> {
    demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "Raster nested retirement depth overflow"))?;
    Ok(demand)
}

pub(super) fn raster_child_grant(grant: RetainedCloneGrant) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant }
}

pub(super) fn raster_reserve_unit(cx: &mut semio_framework_job::StepContext<'_>) -> bool {
    if cx.should_yield() || cx.fuel_remaining() == 0 {
        return false;
    }
    cx.consume_fuel(1);
    true
}
