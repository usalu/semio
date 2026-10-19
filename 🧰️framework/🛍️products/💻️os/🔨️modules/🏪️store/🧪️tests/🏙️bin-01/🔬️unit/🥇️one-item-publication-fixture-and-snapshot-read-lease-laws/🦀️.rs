/// 🥇️ One item publication fixture and snapshot read lease laws: the one item and ephemeral publication fixtures, the generation denial, the snapshot read lease capacity, the stale generation and the lease contention.

const ONE_ITEM_PUBLICATION_FIXTURE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🧫️fixtures/artifact-store-one-item-publication-v1/🔣️.json"));

const EPHEMERAL_ONE_ITEM_PUBLICATION_FIXTURE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🧫️fixtures/artifact-ephemeral-one-item-publication-v1/🔣️.json"));

struct SerdeOneItemPublicationOracle {
    maximum_items: usize,
    maximum_bytes: usize,
    maximum_retries: u8,
}

impl SerdeOneItemPublicationOracle {
    fn from_host_snapshot(fixture: &serde_json::Value) -> Self {
        Self {
            maximum_items: fixture["capacities"]["maximumWorkItems"].as_u64().expect("maximum work items") as usize,
            maximum_bytes: fixture["capacities"]["maximumRetainedBytes"].as_u64().expect("maximum retained bytes") as usize,
            maximum_retries: fixture["capacities"]["maximumRetries"].as_u64().expect("maximum retries") as u8,
        }
    }

    fn admits(&self, items: usize, bytes: usize) -> bool {
        items != 0 && items <= self.maximum_items && bytes <= self.maximum_bytes
    }

    fn freshness(base_generation: u64, live_generation: u64, base_revision: u64, live_revision: u64) -> &'static str {
        if base_generation == live_generation && base_revision == live_revision { "accepted" } else { "fault" }
    }

    fn retry(&self, attempts: u8, acknowledged: bool) -> &'static str {
        if attempts > self.maximum_retries {
            "retry-exhausted"
        } else if acknowledged {
            "complete-empty"
        } else {
            "awaiting-ack"
        }
    }
}

#[test]
fn one_item_publication_fixture_matches_the_third_party_json_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(ONE_ITEM_PUBLICATION_FIXTURE).expect("language-neutral one-item publication fixture");
    let oracle = SerdeOneItemPublicationOracle::from_host_snapshot(&fixture);
    assert_eq!(fixture["capacities"]["historyItems"], semio_framework_os_vcs::ARTIFACT_HISTORY_LEDGER_CAPACITY);
    for case in fixture["admissionCases"].as_array().expect("admission cases") {
        let actual = oracle.admits(case["workItems"].as_u64().expect("work items") as usize, case["retainedBytes"].as_u64().expect("retained bytes") as usize);
        assert_eq!(actual, case["accepted"].as_bool().expect("accepted"), "{}", case["id"]);
        assert_eq!(case["fuelAfter"], 0, "rejected preflight consumes no fuel");
    }
    for case in fixture["authorityCases"].as_array().expect("authority cases") {
        let accepted = case["authorityMatches"] == true && case["digestOwner"] == "store" && case["cursorFieldsExposed"] == false;
        assert_eq!(accepted, case["accepted"].as_bool().expect("authority acceptance"), "{}", case["id"]);
    }
    for case in fixture["lifecycleCases"].as_array().expect("lifecycle cases") {
        match case["id"].as_str().expect("case id") {
            "stale-generation" | "stale-revision" => assert_eq!(
                SerdeOneItemPublicationOracle::freshness(
                    case["baseGeneration"].as_u64().expect("base generation"),
                    case["liveGeneration"].as_u64().expect("live generation"),
                    case["baseRevision"].as_u64().expect("base revision"),
                    case["liveRevision"].as_u64().expect("live revision"),
                ),
                case["expected"].as_str().expect("expected freshness")
            ),
            "retry-then-ack" | "retry-plus-one" => {
                assert_eq!(oracle.retry(case["retryAttempts"].as_u64().expect("retry attempts") as u8, case["acknowledged"].as_bool().expect("acknowledged")), case["expected"].as_str().expect("expected retry outcome"));
            }
            "cancel-before-publish" | "interrupted-close" => assert_eq!(case["expected"], "complete-empty"),
            unexpected => panic!("unknown one-item lifecycle fixture {unexpected}"),
        }
    }
}

#[test]
fn ephemeral_one_item_publication_fixture_matches_the_third_party_json_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(EPHEMERAL_ONE_ITEM_PUBLICATION_FIXTURE).expect("language-neutral ephemeral publication fixture");
    assert_eq!(fixture["lanes"], serde_json::json!(["presence", "transient"]));
    for case in fixture["admissionCases"].as_array().expect("ephemeral admission cases") {
        let work_items = case["workItems"].as_u64().expect("work items") as usize;
        let retained_bytes = case["retainedBytes"].as_u64().expect("retained bytes") as usize;
        let factories = case["preparationFactory"].as_bool().expect("preparation factory") && case["retirementFactory"].as_bool().expect("retirement factory");
        let admitted = factories && ArtifactStoreOneItemFootprint { work_items, retained_bytes }.is_admissible();
        assert_eq!(admitted, case["accepted"].as_bool().expect("accepted"), "{}", case["id"]);
    }
    for case in fixture["lifecycleCases"].as_array().expect("ephemeral lifecycle cases") {
        let expected = case["expected"].as_str().expect("expected");
        match case["id"].as_str().expect("case id") {
            "stale-generation" => assert_eq!(expected, "fault"),
            "single-publication" => assert_eq!(case["generationDelta"], 1),
            "cancel" | "retry-ack" | "interrupted-close" => assert_eq!(expected, "complete-empty"),
            unexpected => panic!("unknown ephemeral lifecycle fixture {unexpected}"),
        }
    }
}

#[test]
fn one_item_publication_source_denies_generic_or_unbounded_shortcuts() {
    let source = ["../../../../⚙️engine/🧱️store-components/📥️lifted/🏪️store/🦀️.rs", "../../../../⚙️engine/📥️lifted/🏪️store/🦀️.rs", "../../../../🎮️one-item/📬️publication/🦀️.rs"].iter().map(|file| std::fs::read_to_string(fixture_path!(file)).unwrap_or_else(|error| panic!("read {file}: {error}"))).collect::<String>();
    let authority_start = source.find("pub struct ArtifactStoreOneItemLiveAuthority").expect("private live authority");
    let authority_end = source[authority_start..].find("impl ArtifactStoreOneItemLiveAuthority").map(|offset| authority_start + offset).expect("authority helper");
    let authority_shape = &source[authority_start..authority_end];
    for forbidden in ["pub operation:", "pub generation:", "pub base_revision:", "pub base_applied_edit_count:", "pub next_sequence_number:", "pub next_clock:", "pub actor:"] {
        assert!(!authority_shape.contains(forbidden), "live authority exposes raw fabrication field {forbidden}");
    }
    let prepared_start = source.find("pub struct ArtifactStoreOneItemPrepared").expect("sealed prepared envelope");
    let prepared_end = source[prepared_start..].find("impl<P, Mutation> ArtifactStoreOneItemPrepared").map(|offset| prepared_start + offset).expect("prepared accessor");
    let prepared_shape = &source[prepared_start..prepared_end];
    assert!(!prepared_shape.contains("pub edit:"));
    assert!(!prepared_shape.contains("pub edit_digest:"));
    assert!(!prepared_shape.contains("cursor"), "domain-prepared envelope cannot carry cursor or history fabrication fields");
    let start = source.find("pub fn advance_apply_batch(").expect("retained batched advance");
    let end = source[start..].find("pub fn cancel_apply_batch(").map(|offset| start + offset).expect("retained batched cancel");
    let advance = &source[start..end];
    for forbidden in [".diff(", ".inverse(", "apply_mutation(", "replay_mutations(", "apply_command(", "flush_outbound(", "try_reserve_exact(", "post_snapshot.clone()", "current.as_ref().clone()", ".to_vec()"] {
        assert!(!advance.contains(forbidden), "batched advance contains forbidden shortcut {forbidden}");
    }
    let close_start = source.find("pub fn close_step(&mut self, grant: ArtifactStoreOneItemGrant)").expect("one-item close");
    let close_end = source[close_start..].find("pub fn terminal_is_empty").map(|offset| close_start + offset).expect("one-item terminal witness");
    let close = &source[close_start..close_end];
    assert!(close.contains("maximum_items: 1"), "a close turn hands each nested owner exactly one item");
    assert!(close.contains("maximum_release_bytes") && close.contains("maximum_copy_bytes") && close.contains("maximum_capacity_bytes"), "a close turn gates on every independent byte axis");
    assert!(close.contains("terminal_is_empty"));
    assert!(!close.contains(".clear()"));
    assert!(!advance.contains("prepared_edit_digest("), "Store commit validation must not reserialize retained edits");
    let fold_start = source.find("fn fold_batch_item(").expect("retained batched fold");
    let fold_end = source[fold_start..].find("fn empty_batch_stage(").map(|offset| fold_start + offset).expect("retained batched stage shell");
    let fold = &source[fold_start..fold_end];
    assert!(fold.contains("authority.validate_prepared(candidate)"), "Store validation must verify its private exact-owner seal before folding an item");
    for forbidden in [".diff(", "apply_mutation(", "replay_mutations(", "apply_command(", "flush_outbound(", "post_snapshot.clone()", "current.as_ref().clone()", ".to_vec()"] {
        assert!(!fold.contains(forbidden), "batched fold contains forbidden shortcut {forbidden}");
    }
    assert_eq!(fold.matches("try_reserve_exact(").count(), 1, "the batched stage reserves every owner's whole admitted capacity through one exact reservation");
    assert_eq!(fold.matches("reserve_original!(&mut").count(), 3, "the batched stage reserves its unit flags, forwards and metadata capacity exactly once each; the inverse pages reserve through their own exact page stack");
    assert!(fold.contains("original.inverse.pop()") && fold.contains("target.inverse.push_reserved(item)"), "a staged item's inverse block is popped from the original tail and pushed in order, which reverses it exactly as replay_mutations reverses one operation's inverse");
}

/// 🧹️ Takes one returned owner through the grant-admitting probe, handing back the exact registered `Arc`.
fn take_one_returned<T: Send + Sync + 'static>(registry: &semio_framework_value_snapshot_read::SnapshotReadRegistryHandle) -> Result<Option<Arc<T>>, SnapshotReadLeaseRefusal> {
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 1 };
    registry.try_admit_one_returned::<T, Arc<T>>(grant, |owner, _| Ok((owner, RetainedCloneProgress::default()))).map(|(owner, _)| owner)
}


#[test]
fn snapshot_read_lease_capacity_plus_one_returns_the_exact_owner_and_every_registered_owner_retires() {
    let registry = semio_framework_value_snapshot_read::SnapshotReadRegistryHandle::new();
    let mut reads = Vec::with_capacity(SNAPSHOT_READ_LEASE_CAPACITY);
    for index in 0..SNAPSHOT_READ_LEASE_CAPACITY {
        let owner = Arc::new(index);
        let lease = registry.try_issue(owner.clone()).expect("fixed snapshot read lease admits its exact capacity");
        reads.push(ErasedSnapshotRead::new(owner, lease));
    }
    let rejected = Arc::new(usize::MAX);
    let returned = match registry.try_issue(rejected.clone()) {
        Ok(_) => panic!("fixed snapshot read lease rejects capacity plus one"),
        Err(returned) => returned,
    };
    assert!(Arc::ptr_eq(&rejected, &returned));
    for read in reads {
        let owner = match read.into_typed::<usize>(&registry) {
            Ok(owner) => owner,
            Err(_) => panic!("exact registered snapshot lease retires"),
        };
        drop(owner);
    }
    assert!(registry.terminal_is_empty());
}

#[test]
fn snapshot_commit_authority_rejects_stale_generation_and_revision() {
    let registry = semio_framework_value_snapshot_read::SnapshotReadRegistryHandle::new();
    let first = [7; 32];
    let second = [9; 32];
    assert!(registry.publish_authority(4, first));
    assert!(registry.authority_matches(4, first));
    assert!(!registry.authority_matches(5, first));
    assert!(!registry.authority_matches(4, second));
    assert!(registry.publish_authority(5, second));
    assert!(!registry.authority_matches(4, first));
    assert!(registry.authority_matches(5, second));
}

#[test]
fn dropped_snapshot_read_remains_observable_until_one_slot_per_step_cleanup_takes_its_guard() {
    let registry = semio_framework_value_snapshot_read::SnapshotReadRegistryHandle::new();
    let owner = Arc::new(String::from("retained"));
    let lease = registry.try_issue(owner.clone()).expect("snapshot read lease admission");
    let read = ErasedSnapshotRead::new(owner, lease);
    let (_, index, generation) = read.lease_identity().unwrap();
    assert!(registry.contains(index, generation));
    assert!(!registry.terminal_is_empty());
    drop(read);
    assert!(registry.contains(index, generation));
    let mut retired = None;
    for _ in 0..SNAPSHOT_READ_LEASE_CAPACITY {
        if let Some(owner) = registry.try_admit_one_returned::<String,_>(physical_test_close_grant(),|original,_|Ok((original,RetainedCloneProgress{copied_items:1,..Default::default()}))).map(|(original,progress)|{assert!(progress.fits(physical_test_close_grant()));original}).expect("one fixed cleanup probe") {
            retired = Some(owner);
            break;
        }
    }
    assert_eq!(retired.as_deref().map(String::as_str), Some("retained"));
    drop(retired);
    assert!(registry.terminal_is_empty());
}

#[test]
fn stale_snapshot_read_generation_never_removes_a_reused_slot() {
    let registry = semio_framework_value_snapshot_read::SnapshotReadRegistryHandle::new();
    let owner = Arc::new(7u32);
    let lease = registry.try_issue(owner.clone()).expect("first exact lease");
    let read = ErasedSnapshotRead::new(owner, lease);
    let (_, index, generation) = read.lease_identity().unwrap();
    drop(match read.into_typed::<u32>(&registry) {
        Ok(owner) => owner,
        Err(_) => panic!("first exact retirement"),
    });
    assert!(registry.try_take(index, generation).is_err());
    assert!(registry.terminal_is_empty());
}
