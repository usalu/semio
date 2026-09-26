//! 🤝️ Language-agnostic two-client document scenario fixture — Rust twin of the TS runner.
//! Validates the shared JSON fixture shape so both languages pin the same scenario.

#[cfg(test)]
mod tests {
    use serde_json::Value;

    const FIXTURE: &str = include_str!("../../🧫️fixtures/🤝️two-client-document-v1/🔣️.json");

    #[test]
    fn two_client_document_fixture_names_the_shared_scenario() {
        let fixture: Value = serde_json::from_str(FIXTURE).expect("fixture json");
        assert_eq!(fixture["schema"], "semio.hub.two-client-document-scenario/v1");
        assert_eq!(fixture["version"], 1);
        assert_eq!(fixture["authors"].as_array().map(|a| a.len()), Some(2));
        assert_eq!(fixture["command"]["diffSchema"], "db.pathmap.v1");
        assert!(fixture["steps"].as_array().map(|s| s.len()).unwrap_or(0) >= 8);
        assert_eq!(fixture["expectations"]["bReceivesCommandsEnvelope"], true);
        assert_eq!(fixture["expectations"]["presencePeerActorMatchesA"], true);
        assert_eq!(fixture["expectations"]["presenceJoinReplayShowsA"], true);
        assert_eq!(fixture["expectations"]["lateJoinerCatchesUp"], true);
        assert_eq!(fixture["expectations"]["presenceLeaveDropsPeer"], true);
        assert_eq!(fixture["expectations"]["presenceLeaseExpiryStripsA"], true);
        assert_eq!(fixture["presence"]["leaseTtlMs"], super::super::PRESENCE_LEASE_TTL_MS);
        assert_eq!(fixture["expectations"]["reconnectDeliversMissedOrRebootstrap"], true);
        assert_eq!(fixture["expectations"]["restartPreservesArtifactAndFrontier"], true);
        assert_eq!(fixture["expectations"]["frontierDocumentIdIsPlainArtifactId"], true);
        assert_eq!(fixture["expectations"]["gracefulShutdownClosesSocketsAndReleasesWriters"], true);
        assert_eq!(fixture["expectations"]["crashReleasesWritersPerBackendContract"], true);
        assert_eq!(fixture["expectations"]["revocationEndsAgentSession"], true);
        assert_eq!(fixture["agent"]["closeCode"], 4401);
        assert_eq!(fixture["agent"]["refusedStatus"], 401);
        assert!(fixture["agent"]["revocationWithinMs"].as_u64().unwrap_or(u64::MAX) <= 5_000);
        assert_eq!(fixture["shutdown"]["closeCode"], super::super::HUB_SHUTDOWN_CLOSE_CODE);
        assert!(fixture["shutdown"]["gracefulExitWithinMs"].as_u64().unwrap_or(0) >= (super::super::SOCKET_DRAIN_DEADLINE + super::super::DATABASE_SHUTDOWN_DEADLINE).as_millis() as u64);
    }
}
