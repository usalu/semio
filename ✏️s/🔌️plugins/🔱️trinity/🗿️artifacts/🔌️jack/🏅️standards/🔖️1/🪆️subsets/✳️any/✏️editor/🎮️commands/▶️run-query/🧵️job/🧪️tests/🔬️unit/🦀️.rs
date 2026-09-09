use super::*;

    fn checkpoint(work: &JackQueryWork) -> [u8; QUERY_CHECKPOINT_BYTES] {
        let mut bytes = [0_u8; QUERY_CHECKPOINT_BYTES];
        assert_eq!(<JackQueryWork as ArtifactCommandWork<Owner>>::checkpoint(work, &mut bytes).expect("query checkpoint"), QUERY_CHECKPOINT_BYTES);
        bytes
    }

    #[test]
    fn query_ownership_checkpoint_binds_exact_operation_and_generation() {
        let mut first = JackQueryWork::new("runQuery", "RETURN 1".into(), "editor".into(), "results".into(), 501, 12);
        first.progress = 37;
        let bytes = checkpoint(&first);
        let mut exact = JackQueryWork::new("runQuery", "RETURN 1".into(), "editor".into(), "results".into(), 501, 12);
        <JackQueryWork as ArtifactCommandWork<Owner>>::restore(&mut exact, &bytes).expect("exact owner restores");
        assert_eq!(exact.replay_target, Some(37));
        let mut xor_collision = JackQueryWork::new("runQuery", "RETURN 1".into(), "editor".into(), "results".into(), 4_194_805, 13);
        assert_eq!(first.identity(), xor_collision.identity(), "fixture reproduces the old XOR collision");
        assert!(<JackQueryWork as ArtifactCommandWork<Owner>>::restore(&mut xor_collision, &bytes).is_err());
        eprintln!("[DEBUG] Jack query checkpoint rejects the prior operation/generation XOR collision");
    }

    #[test]
    fn query_ownership_checkpoint_accepts_legal_long_scan_progress() {
        let mut source = JackQueryWork::new("runQuery", "RETURN 1".into(), "editor".into(), "results".into(), 700, 21);
        source.progress = 1_000_001;
        let bytes = checkpoint(&source);
        let mut restored = JackQueryWork::new("runQuery", "RETURN 1".into(), "editor".into(), "results".into(), 700, 21);
        <JackQueryWork as ArtifactCommandWork<Owner>>::restore(&mut restored, &bytes).expect("legal long scan restores");
        assert_eq!(restored.replay_target, Some(1_000_001));
        source.progress = QUERY_REPLAY_MAXIMUM_STEPS + 1;
        let rejected = checkpoint(&source);
        let mut target = JackQueryWork::new("runQuery", "RETURN 1".into(), "editor".into(), "results".into(), 700, 21);
        assert!(<JackQueryWork as ArtifactCommandWork<Owner>>::restore(&mut target, &rejected).is_err());
        eprintln!("[DEBUG] Jack query checkpoint admits legal scans above one million and rejects progress beyond the derived admission");
    }
