//! ♻️ The retained native initializer catalog closes one admitted page before empty handoff.

use store::ArtifactStoreInitializationOwnerCatalog;
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneStep}};

pub(super) fn initialization_catalog_close_demands(owner: &Option<ArtifactStoreInitializationOwnerCatalog>) -> Result<RetirementDemand, ValueError> {
    owner.as_ref().map_or(Ok(RetirementDemand::default()), ArtifactStoreInitializationOwnerCatalog::close_demands)
}

pub(super) fn close_initialization_catalog(owner: &mut Option<ArtifactStoreInitializationOwnerCatalog>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
    let Some(catalog) = owner.as_mut() else { return Ok(RetainedCloneStep::Complete(Default::default())); };
    let step = catalog.close_step(grant)?;
    if matches!(step, RetainedCloneStep::Complete(_)) {
        if !catalog.terminal_is_empty() { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "Drawing initialization catalog reported a false terminal")); }
        drop(owner.take());
    }
    Ok(step)
}
