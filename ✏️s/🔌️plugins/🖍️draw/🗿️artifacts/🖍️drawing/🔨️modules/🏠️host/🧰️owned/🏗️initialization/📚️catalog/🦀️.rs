//! ♻️ The retained native initializer catalog closes one admitted page before empty handoff.

use store::{ArtifactStoreInitializationOwnerCatalog, SnapshotRetirementStep};
use semio_framework_value::{ValueError, ValueRefusalKind};

pub(super) fn next_initialization_catalog_close_byte_demand(owner: &Option<ArtifactStoreInitializationOwnerCatalog>) -> Result<usize, ValueError> {
    owner.as_ref().map_or(Ok(0), ArtifactStoreInitializationOwnerCatalog::next_close_byte_demand)
}

pub(super) fn close_initialization_catalog(owner: &mut Option<ArtifactStoreInitializationOwnerCatalog>, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, ValueError> {
    if maximum_items == 0 { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
    let Some(catalog) = owner.as_mut() else { return Ok(SnapshotRetirementStep::Complete); };
    let step = catalog.close_step(1, maximum_bytes)?;
    if matches!(step, SnapshotRetirementStep::Complete) {
        if !catalog.terminal_is_empty() { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "Drawing initialization catalog reported a false terminal")); }
        drop(owner.take());
    }
    Ok(step)
}
