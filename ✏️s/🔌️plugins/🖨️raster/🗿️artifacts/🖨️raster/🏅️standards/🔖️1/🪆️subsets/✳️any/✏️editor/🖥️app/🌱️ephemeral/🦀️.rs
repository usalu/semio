//! 🌱️ Raster initializes its real shared inline presence and genuinely empty local transient lane under the original caller.
use super::*;
use semio_plugin::original_ephemeral::{self,OriginalFixedStoreInitial,FixedStoreInitial,OriginalEphemeralDeclaration,OriginalPresenceOwners,OriginalTransientOwners};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind};
use std::{mem::size_of,sync::Arc};
pub struct RasterPresenceInitial;
impl OriginalFixedStoreInitial for RasterPresenceInitial{
 type State=RasterPresence;
 fn initial_state_demands()->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:2*size_of::<RasterPresence>(),depth:1,..Default::default()})}
 fn initial_state(grant:RetainedCloneGrant)->Result<(RasterPresence,RetainedCloneProgress),ValueError>{let d=Self::initial_state_demands()?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Raster original presence requires its fixed inline source grant"))}Ok((RasterPresence::default(),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
}
pub struct RasterEphemeralDeclaration;
impl OriginalEphemeralDeclaration<semio_plugin::EditorApp<RasterPlayApp>> for RasterEphemeralDeclaration{
 type PresenceInitial=FixedStoreInitial<RasterPresenceInitial>;
 type TransientInitial=FixedStoreInitial<original_ephemeral::EmptyTransientInitial>;
 fn presence_owners_demands()->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:2*size_of::<OriginalPresenceOwners<semio_plugin::EditorApp<RasterPlayApp>>>(),capacity_bytes:semio_framework_value::factory_arc_birth_bytes::<semio_framework_value::SharedValueRetirementFactory<RasterPresence>>()+size_of::<semio_plugin::PresenceStoreOwnedDisposer<RasterPresence>>(),depth:2,..Default::default()})}
 fn presence_owners(initial:&Arc<RasterPresence>)->OriginalPresenceOwners<semio_plugin::EditorApp<RasterPlayApp>>{let factory=semio_plugin::bounded_transient_root_retirement_factory::<RasterPresence>();let disposer=semio_plugin::PresenceStoreOwnedDisposer::new(initial.clone(),crate::presence::raster_presence_is_terminal_empty).unwrap_or_else(|_|unreachable!("Raster presence owns only inline scalar fields"));OriginalPresenceOwners{local:factory.clone(),peer:factory,disposer:Box::new(disposer),preparation:None}}
 fn transient_owners_demands()->Result<RetirementDemand,ValueError>{original_ephemeral::empty_transient_owners_demands::<semio_plugin::EditorApp<RasterPlayApp>>()}
 fn transient_owners()->OriginalTransientOwners<semio_plugin::EditorApp<RasterPlayApp>>{original_ephemeral::empty_transient_owners::<semio_plugin::EditorApp<RasterPlayApp>>()}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
