//! 🩻️ Staged slice of the `maintenance-ladder` band of the mounted artifact instance: its `instance-stores` members (WP6 step S0 of the instance family).
//!
//! 🪜️ The live-maintenance rotation of one hosted app: priority and pressure units first, then a fair fixed round robin
//! in which an empty stage never consumes the call. Every unit quotes its next demand on all axes and spends the caller's
//! unchanged grant on exactly one owner; the same module drives the envelope-ingress, store-replacement and archive-load
//! frontiers for both the live rotation and the close ladder.
use super::*;
use crate::tool_jobs::time_travel;
use semio_framework_value::{RetirementDemand, ValueError, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};

impl<A: ArtifactApp> InstanceStores<A> {
    pub(crate) fn config_lane_under_pressure(&self) -> bool {
        self.config_store.maintenance_retirements_under_pressure() || self.draft_store.maintenance_retirements_under_pressure() || self.interaction_store.maintenance_retirements_under_pressure() || self.window_config_store.maintenance_retirements_under_pressure()
    }
}

impl<A: ArtifactApp> PeerRoster<A> {
    pub(crate) fn maintenance_peer_roster_generation(&self) -> Option<u64> {
        let generation = self.peer_roster_processed_generation.checked_add(1)?;
        if self.peer_roster_publications.get(generation).is_some() { Some(generation) } else { self.orphaned_peer_roster_generation() }
    }
}
