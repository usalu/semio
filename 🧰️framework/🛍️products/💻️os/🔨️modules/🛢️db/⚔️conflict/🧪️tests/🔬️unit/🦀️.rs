use super::*;

//#region 🧸️Fixtures
fn hlt(physical_ms: u64, actor: u64) -> protocol::HybridLogicalTimestamp {
    protocol::HybridLogicalTimestamp { actor, physical_ms, logical: 0 }
}

fn command(id: &str, actor: u64, physical_ms: u64, kind: &str) -> CommandTouch {
    CommandTouch::new(protocol::MutationId(id.into()), protocol::ActorId(format!("actor-{actor}")), CommandKind::from(kind), hlt(physical_ms, actor))
}
//#endregion 🧸️Fixtures

//#region 🔖️CommandTouch
#[test]
fn order_key_breaks_ties_on_command_id_when_timestamps_are_equal() {
    let a = command("cmd-a", 1, 1000, "write");
    let b = command("cmd-b", 1, 1000, "write");
    assert!(a.order_key() < b.order_key(), "same timestamp -> command_id is the final tiebreak");
}
//#endregion 🔖️CommandTouch

//#region 🔖️Bloom
#[test]
fn bloom_rejects_new_requires_positive_params() {
    assert!(PathBloom::new(0, 4).is_err());
    assert!(PathBloom::new(1024, 0).is_err());
    assert!(PathBloom::new(1024, 4).is_ok());
}

#[test]
fn bloom_might_contain_has_no_false_negatives() {
    let mut bloom = PathBloom::default_sized();
    let paths = ["a/b/c", "x/y", "counter/1", "very/long/nested/path/segment"];
    for path in paths {
        bloom.insert(path);
    }
    for path in paths {
        assert!(bloom.might_contain(path), "inserted path must never be reported absent");
    }
}

#[test]
fn bloom_might_intersect_has_no_false_negatives_across_thousands_of_random_disjoint_and_overlapping_sets() {
    let mut rng_state: u64 = 0x9E3779B97F4A7C15;
    let mut next = move || {
        rng_state ^= rng_state << 13;
        rng_state ^= rng_state >> 7;
        rng_state ^= rng_state << 17;
        rng_state
    };
    for trial in 0..500 {
        let mut left = PathBloom::default_sized();
        let mut right = PathBloom::default_sized();
        let mut left_paths = Vec::new();
        let mut right_paths = Vec::new();
        for i in 0..20 {
            left_paths.push(format!("trial-{trial}/left/{i}/{}", next() % 997));
        }
        for i in 0..20 {
            right_paths.push(format!("trial-{trial}/right/{i}/{}", next() % 997));
        }
        let shares = trial % 3 == 0;
        if shares {
            right_paths.push(left_paths[0].clone());
        }
        for path in &left_paths {
            left.insert(path);
        }
        for path in &right_paths {
            right.insert(path);
        }
        let truly_shared = left_paths.iter().any(|path| right_paths.contains(path));
        if truly_shared {
            assert!(left.might_intersect(&right), "trial {trial}: a genuinely shared path must never be missed");
        }
    }
}

#[test]
fn bloom_mismatched_sizes_conservatively_report_might_intersect() {
    let a = PathBloom::new(1024, 4).unwrap();
    let b = PathBloom::new(2048, 4).unwrap();
    assert!(a.might_intersect(&b));
}
//#endregion 🔖️Bloom

//#region 🔖️KindMatrix
#[test]
fn kind_matrix_declared_commuting_pair_is_symmetric() {
    let mut matrix = CommandKindMatrix::new();
    let counter = CommandKind::from("counter.increment");
    let audit = CommandKind::from("audit.append");
    matrix.declare_commuting(&counter, &audit);
    assert!(matrix.commutes(&counter, &audit));
    assert!(matrix.commutes(&audit, &counter));
    assert!(!matrix.commutes(&counter, &CommandKind::from("other")));
}

#[test]
fn kind_matrix_read_only_kind_commutes_with_everything() {
    let mut matrix = CommandKindMatrix::new();
    let query = CommandKind::from("query");
    matrix.declare_read_only(&query);
    assert!(matrix.commutes(&query, &CommandKind::from("anything")));
    assert!(matrix.commutes(&CommandKind::from("anything"), &query));
}
//#endregion 🔖️KindMatrix

//#region 🔖️Lifecycle
#[test]
fn classify_is_quarantined_when_the_policy_rejects_the_worst_level() {
    let envelopes = Vec::new();
    let kind = classify(Some(protocol::Severity::Fatal), protocol::MergePolicy::LaissezFaire, envelopes.clone(), Vec::new());
    assert_eq!(kind, Some(protocol::ConflictKind::Quarantined { envelopes }));
}

#[test]
fn classify_is_degraded_when_accepted_but_still_warning_or_above() {
    let edit_ids = vec!["e1".to_string()];
    let kind = classify(Some(protocol::Severity::Warning), protocol::MergePolicy::Normal, Vec::new(), edit_ids.clone());
    assert_eq!(kind, Some(protocol::ConflictKind::Degraded { edit_ids }));
}

#[test]
fn classify_is_none_when_clean_or_below_warning() {
    assert_eq!(classify(None, protocol::MergePolicy::Vigilant, Vec::new(), Vec::new()), None);
    assert_eq!(classify(Some(protocol::Severity::Info), protocol::MergePolicy::Vigilant, Vec::new(), Vec::new()), None);
}
//#endregion 🔖️Lifecycle

//#region 🔖️Detector
#[test]
fn detects_write_write_touched_region_conflict() {
    let a = command("cmd-a", 1, 1000, "write").touch(TouchedRegion::write("artifacts/doc-1/title"));
    let b = command("cmd-b", 2, 2000, "write").touch(TouchedRegion::write("artifacts/doc-1/title"));

    let detector = ConflictDetector::new();
    let records = detector.detect(&[a, b]);
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].command_id, protocol::MutationId("cmd-b".into()), "the later command carries the record");
    assert_eq!(records[0].conflicting_with, protocol::MutationId("cmd-a".into()));
    match &records[0].kind {
        ConflictKind::TouchedRegion(regions) => assert_eq!(regions.len(), 1),
        other => panic!("expected TouchedRegion, got {other:?}"),
    }
}

#[test]
fn read_read_never_conflicts() {
    let a = command("cmd-a", 1, 1000, "read").touch(TouchedRegion::read("artifacts/doc-1/title"));
    let b = command("cmd-b", 2, 2000, "read").touch(TouchedRegion::read("artifacts/doc-1/title"));
    assert!(ConflictDetector::new().detect(&[a, b]).is_empty());
}

#[test]
fn disjoint_paths_never_conflict() {
    let a = command("cmd-a", 1, 1000, "write").touch(TouchedRegion::write("artifacts/doc-1/title"));
    let b = command("cmd-b", 2, 2000, "write").touch(TouchedRegion::write("artifacts/doc-1/body"));
    assert!(ConflictDetector::new().detect(&[a, b]).is_empty());
}

#[test]
fn kind_matrix_override_suppresses_an_otherwise_conflicting_pair() {
    let a = command("cmd-a", 1, 1000, "counter.increment").touch(TouchedRegion::write("artifacts/doc-1/counter"));
    let b = command("cmd-b", 2, 2000, "counter.increment").touch(TouchedRegion::write("artifacts/doc-1/counter"));

    let mut matrix = CommandKindMatrix::new();
    matrix.declare_commuting(&CommandKind::from("counter.increment"), &CommandKind::from("counter.increment"));
    let detector = ConflictDetector::with_matrix(matrix);
    assert!(detector.detect(&[a, b]).is_empty(), "declared-commuting kinds must skip region detection entirely");
}

#[test]
fn constraint_conflict_detected_across_disjoint_touched_paths() {
    let a = command("cmd-a", 1, 1000, "create-user").touch(TouchedRegion::write("artifacts/doc-1")).claim("unique/email/alice@example.com");
    let b = command("cmd-b", 2, 2000, "create-user").touch(TouchedRegion::write("artifacts/doc-2")).claim("unique/email/alice@example.com");

    let records = ConflictDetector::new().detect(&[a, b]);
    assert_eq!(records.len(), 1, "disjoint overlay paths must not mask the shared constraint claim");
    assert_eq!(records[0].kind, ConflictKind::Constraint("unique:unique/email/alice@example.com".to_string()));
}

#[test]
fn single_parent_constraint_conflicts_only_on_diverging_parent() {
    let a = command("cmd-a", 1, 1000, "reparent").touch(TouchedRegion::write("artifacts/doc-1")).claim_parent("node-1", "parent-a");
    let b = command("cmd-b", 2, 2000, "reparent").touch(TouchedRegion::write("artifacts/doc-2")).claim_parent("node-1", "parent-b");
    let records = ConflictDetector::new().detect(&[a, b]);
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].kind, ConflictKind::Constraint("single-parent:node-1".to_string()));

    let c = command("cmd-c", 1, 1000, "reparent").touch(TouchedRegion::write("artifacts/doc-3")).claim_parent("node-1", "parent-a");
    let d = command("cmd-d", 2, 2000, "reparent").touch(TouchedRegion::write("artifacts/doc-4")).claim_parent("node-1", "parent-a");
    assert!(ConflictDetector::new().detect(&[c, d]).is_empty(), "the same (child, parent) claimed twice is not a conflict");
}

#[test]
fn non_overlapping_interval_constraint_conflicts_only_when_ranges_actually_overlap() {
    let a = command("cmd-a", 1, 1000, "schedule").touch(TouchedRegion::write("artifacts/doc-1")).claim_interval("track-1", 0, 10);
    let overlapping = command("cmd-b", 2, 2000, "schedule").touch(TouchedRegion::write("artifacts/doc-2")).claim_interval("track-1", 5, 15);
    let records = ConflictDetector::new().detect(&[a.clone(), overlapping]);
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].kind, ConflictKind::Constraint("interval:track-1".to_string()));

    let touching = command("cmd-c", 3, 3000, "schedule").touch(TouchedRegion::write("artifacts/doc-3")).claim_interval("track-1", 10, 20);
    assert!(ConflictDetector::new().detect(&[a.clone(), touching]).is_empty(), "touching (non-overlapping) intervals must not conflict");

    let other_track = command("cmd-d", 4, 4000, "schedule").touch(TouchedRegion::write("artifacts/doc-4")).claim_interval("track-2", 0, 10);
    assert!(ConflictDetector::new().detect(&[a, other_track]).is_empty(), "different tracks never conflict");
}

#[test]
fn detection_result_is_independent_of_input_order() {
    let a = command("cmd-a", 1, 1000, "write").touch(TouchedRegion::write("artifacts/doc-1/x"));
    let b = command("cmd-b", 2, 2000, "write").touch(TouchedRegion::write("artifacts/doc-1/x"));
    let c = command("cmd-c", 3, 3000, "write").touch(TouchedRegion::write("artifacts/doc-1/x"));

    let forward = ConflictDetector::new().detect(&[a.clone(), b.clone(), c.clone()]);
    let shuffled = ConflictDetector::new().detect(&[c, a, b]);
    assert_eq!(forward, shuffled, "the detected conflict set must not depend on caller-supplied batch order");
    assert_eq!(forward.len(), 3, "three mutually-conflicting commands -> 3 pairwise records");
}

#[test]
fn priority_side_is_the_earlier_command_by_timestamp_regardless_of_batch_order() {
    let later = command("cmd-later", 1, 5000, "write").touch(TouchedRegion::write("p"));
    let earlier = command("cmd-earlier", 1, 1000, "write").touch(TouchedRegion::write("p"));

    let records = ConflictDetector::new().detect(&[later, earlier]);
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].conflicting_with, protocol::MutationId("cmd-earlier".into()));
    assert_eq!(records[0].command_id, protocol::MutationId("cmd-later".into()));
}
//#endregion 🔖️Detector
