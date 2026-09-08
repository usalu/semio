mod tests {
    use super::super::kernel::{init, macrostep};
    use super::super::testing::support::{UnitToggleContext, UnitToggleEvent, UnitToggleMachine, unit_toggle_definition};
    use super::*;

    #[test]
    fn persist_then_restore_round_trips_active_state() {
        let _ = unit_toggle_definition();
        let mut sink = Vec::new();
        let mut snapshot = init::<UnitToggleMachine>((), &mut sink);
        let mut inspector = super::super::inspect::NullInspector;
        macrostep(&mut snapshot, UnitToggleEvent::Flip, &mut sink, &mut inspector);
        assert!(snapshot.matches("on"));

        let persisted = persist(&snapshot);
        assert_eq!(persisted.fingerprint, UnitToggleMachine::definition().fingerprint);
        assert!(persisted.states.iter().any(|s| s == "on"));

        let restored = restore::<UnitToggleMachine, NoMigrations>(&persisted, UnitToggleContext::default(), &[]).expect("restore should succeed");
        assert!(restored.matches("on"));
    }

    #[test]
    fn restore_rejects_fingerprint_mismatch_without_migration() {
        let mut sink = Vec::new();
        let snapshot = init::<UnitToggleMachine>((), &mut sink);
        let mut persisted = persist(&snapshot);
        persisted.fingerprint = 9999;
        let result = restore::<UnitToggleMachine, NoMigrations>(&persisted, UnitToggleContext::default(), &[]);
        assert!(matches!(result, Err(RestoreError::FingerprintMismatch)));
    }

    struct BumpFingerprint;
    impl Migration for BumpFingerprint {
        fn source_fingerprint(&self) -> u64 {
            9999
        }
        fn migrate(&self, mut snapshot: PersistedSnapshot) -> PersistedSnapshot {
            snapshot.fingerprint = UnitToggleMachine::definition().fingerprint;
            snapshot
        }
    }

    #[test]
    fn restore_applies_migration_chain_until_fingerprint_matches() {
        let mut sink = Vec::new();
        let snapshot = init::<UnitToggleMachine>((), &mut sink);
        let mut persisted = persist(&snapshot);
        persisted.fingerprint = 9999;
        let migration = BumpFingerprint;
        let migrations: &[&BumpFingerprint] = &[&migration];
        let restored = restore::<UnitToggleMachine, BumpFingerprint>(&persisted, UnitToggleContext::default(), migrations).expect("migration should bridge fingerprint");
        assert!(restored.matches("off"));
    }
}
