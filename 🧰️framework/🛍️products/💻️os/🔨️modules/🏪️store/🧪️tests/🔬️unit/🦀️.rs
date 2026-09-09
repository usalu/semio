use super::*;
#[cfg(test)]
#[path = "../../📦️codec/🧵️send/🧪️tests/🧵️send/🦀️.rs"]
mod native_codec_send_tests;

#[cfg(test)]
#[path = "../../🔗️backbone/✂️detach/🧪️tests/✂️detach/🦀️.rs"]
mod backbone_detach_refusal_tests;

use super::fixture_mutations::{
    demo::{AddN, DeleteN, DemoMutation, RestoreN, SetN},
    lossy::{LossyMutation, SetN as LossySetN},
    severity::{SetErrorN, SetFatalN, SetN as SeveritySetN, SetWarningN, SeverityMutation},
    timestamped::{SetN as TimestampedSetN, TimestampedMutation},
    validated::{RestoreN as ValidatedRestoreN, SetN as ValidatedSetN, ValidatedMutation},
};

pub(super) fn assert_fixture_descriptor<T: crate::os_spr::MutationLeaf>(descriptor: &str) {
    assert_eq!(serde_json::Value::from(T::DESCRIPTOR.to_value()), serde_json::from_str::<serde_json::Value>(descriptor).unwrap());
    assert!(T::DESCRIPTOR.validate().is_ok());
}

fn direct_fixture_cases() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap()
}

fn assert_fixture_case<Op>(row: &serde_json::Value)
where
    Op: Mutation<DemoSnapshot, Diff = DemoDiff> + OpText + OpBinary + PartialEq + std::fmt::Debug,
{
    let mut value = row["payload"].clone();
    value["operation"] = row["operation"].clone();
    let mutation = Op::from_value(value.into()).unwrap();
    assert!(mutation.descriptor().validate().is_ok());
    let mut unknown = serde_json::Value::from(mutation.to_value());
    unknown["unknown"] = serde_json::json!(true);
    assert!(Op::from_value(unknown.into()).is_err());
    for key in row["payload"].as_object().unwrap().keys() {
        let mut invalid = serde_json::Value::from(mutation.to_value());
        invalid[key] = serde_json::json!(true);
        assert!(Op::from_value(invalid.clone().into()).is_err());
        invalid[key] = if key == "physicalMs" { serde_json::json!(1e21) } else { serde_json::json!(2147483648i64) };
        assert!(Op::from_value(invalid.into()).is_err());
        let mut missing = serde_json::Value::from(mutation.to_value());
        missing.as_object_mut().unwrap().remove(key);
        assert_eq!(Op::from_value(missing.into()).is_ok(), row["operation"] == "restoreN" && key == "n");
    }
    let before = DemoSnapshot { n: serde_json::from_value(row["before"].clone()).unwrap() };
    let outcome = mutation.diff(&before);
    let (diff, messages) = outcome.into_parts();
    let actual = diff.apply(&before).unwrap();
    let decoded_diff = serde_json::from_value::<DemoDiff>(serde_json::to_value(&diff).unwrap()).unwrap();
    assert_eq!(decoded_diff.apply(&before).unwrap(), actual);
    assert_eq!(actual.n, serde_json::from_value::<Option<i32>>(row["after"].clone()).unwrap());
    let level = messages.first().map(|message| format!("{:?}", message.level).to_lowercase());
    assert_eq!(level.as_deref(), row["level"].as_str());
    let mut restored = actual;
    for inverse in mutation.inverse(&before).into_iter().rev() {
        restored = inverse.diff(&restored).diff().apply(&restored).unwrap();
    }
    assert_eq!(restored, before);
    assert_eq!(Op::from_value(serde_json::from_slice::<serde_json::Value>(&serde_json::to_vec(&serde_json::Value::from(mutation.to_value())).unwrap()).unwrap().into()).unwrap(), mutation);
    let text = mutation.print_op();
    assert_eq!(text.split(' ').next(), mutation.descriptor().text_opcode);
    assert_eq!(Op::parse_op(&text).unwrap(), mutation);
    let binary = mutation.encode_op().unwrap();
    assert_eq!(binary.first(), Some(&1));
    assert_eq!(binary.get(1).copied().map(u32::from), mutation.descriptor().binary_tag);
    assert_eq!(Op::decode_op(&binary).unwrap(), mutation);
    assert_eq!(mutation.timestamp(), row.get("timestamp").map(|time| HybridLogicalTimestamp::new(0, time.as_u64().unwrap())));
}

fn assert_fixture_text_codecs<Op>(fixture: &serde_json::Value, family: &str, canonical: impl Fn(&serde_json::Value) -> Op)
where
    Op: Mutation<DemoSnapshot, Diff = DemoDiff> + OpText + OpBinary + PartialEq + std::fmt::Debug,
{
    for row in fixture["textCodecs"].as_array().unwrap().iter().filter(|row| row["family"] == family) {
        let mutation = canonical(row);
        assert_eq!(mutation.print_op(), row["canonical"].as_str().unwrap());
        assert_eq!(Op::parse_op(row["canonical"].as_str().unwrap()).unwrap(), mutation);
        assert!(Op::parse_op(row["rejected"].as_str().unwrap()).is_err());
        assert_eq!(mutation.descriptor().text_opcode, Some(row["canonical"].as_str().unwrap().split(' ').next().unwrap()));
        assert_eq!(mutation.descriptor().binary_tag, row["tag"].as_u64().map(|tag| u32::try_from(tag).unwrap()));
    }
}

#[test]
fn direct_store_fixture_absence_inverse_boundary() {
    let fixture = direct_fixture_cases();
    for row in fixture["cases"].as_array().unwrap().iter().filter(|row| row["family"] == "demo") {
        assert_fixture_case::<DemoMutation>(row);
    }
    assert_fixture_text_codecs(&fixture, "demo", |row| DemoMutation::AddN(AddN { delta: row["payload"]["delta"].as_i64().unwrap() as i32 }));
    assert_fixture_text_codecs(&fixture, "severity", |row| match row["variant"].as_str().unwrap() {
        "SetN" => SeverityMutation::SetN(SeveritySetN { n: row["payload"]["n"].as_i64().unwrap() as i32 }),
        "SetWarningN" => SeverityMutation::SetWarningN(SetWarningN { n: row["payload"]["n"].as_i64().unwrap() as i32 }),
        "SetErrorN" => SeverityMutation::SetErrorN(SetErrorN { n: row["payload"]["n"].as_i64().unwrap() as i32 }),
        "SetFatalN" => SeverityMutation::SetFatalN(SetFatalN { n: row["payload"]["n"].as_i64().unwrap() as i32 }),
        variant => panic!("unexpected severity text codec variant {variant}"),
    });
    assert_fixture_text_codecs(&fixture, "validated", |row| match row["variant"].as_str().unwrap() {
        "SetN" => ValidatedMutation::SetN(ValidatedSetN { n: row["payload"]["n"].as_i64().unwrap() as i32 }),
        "RestoreN" => ValidatedMutation::RestoreN(ValidatedRestoreN { n: Some(row["payload"]["n"].as_i64().unwrap() as i32) }),
        variant => panic!("unexpected validated text codec variant {variant}"),
    });
    for json in ["{}", "{\"n\":null}"] {
        assert_eq!(serde_json::from_str::<DemoSnapshot>(json).unwrap().n, None);
    }
    for json in ["{\"n\":2147483648}", "{\"n\":-2147483649}", "{\"n\":0.5}", "{\"n\":\"1\"}"] {
        assert!(serde_json::from_str::<DemoSnapshot>(json).is_err());
    }
    let before = DemoSnapshot { n: Some(i32::MIN) };
    let mutation = DemoMutation::DeleteN(DeleteN {});
    let deleted = mutation.diff(&before).diff().apply(&before).unwrap();
    assert_eq!(deleted.n, None);
    assert_eq!(mutation.inverse(&before)[0].diff(&deleted).diff().apply(&deleted).unwrap(), before);
}

#[test]
fn direct_store_fixture_structural_diff_composition() {
    let fixture = direct_fixture_cases();
    for row in fixture["diffCases"].as_array().unwrap() {
        let before = DemoSnapshot { n: serde_json::from_value(row["before"].clone()).unwrap() };
        let mut actual = before.clone();
        let mut combined = DemoDiff::default();
        for value in row["diffs"].as_array().unwrap() {
            let diff = serde_json::from_value::<DemoDiff>(value.clone()).unwrap();
            actual = diff.apply(&actual).unwrap();
            combined.absorb(diff);
        }
        let expected = DemoSnapshot { n: serde_json::from_value(row["after"].clone()).unwrap() };
        assert_eq!(actual, expected);
        assert_eq!(combined.apply(&before).unwrap(), expected);
        let round_trip = serde_json::from_value::<DemoDiff>(serde_json::to_value(&combined).unwrap()).unwrap();
        assert_eq!(round_trip.apply(&before).unwrap(), expected);
        let restore = DemoMutation::RestoreN(RestoreN { n: before.n });
        assert_eq!(restore.diff(&actual).diff().apply(&actual).unwrap(), before);
    }
}

#[test]
fn direct_store_fixture_timestamp() {
    let fixture = direct_fixture_cases();
    for row in fixture["cases"].as_array().unwrap().iter().filter(|row| row["family"] == "timestamped") {
        assert_fixture_case::<TimestampedMutation>(row);
    }
    let op = TimestampedMutation::SetN(TimestampedSetN { n: 7, physical_ms: u64::MAX });
    assert_eq!(op.timestamp(), Some(HybridLogicalTimestamp::new(0, u64::MAX)));
    assert_eq!(TimestampedMutation::decode_op(&op.encode_op().unwrap()).unwrap(), op);
    assert_eq!(TimestampedMutation::parse_op(&op.print_op()).unwrap(), op);
    let json = serde_json::to_string(&op).unwrap();
    for invalid in ["18446744073709551616", "-1", "0.5", "1e21", "null", "\"1\""] {
        assert!(serde_json::from_str::<TimestampedMutation>(&json.replace("18446744073709551615", invalid)).is_err());
    }
}

#[test]
fn direct_store_fixture_severity_and_rejection() {
    let fixture = direct_fixture_cases();
    for row in fixture["cases"].as_array().unwrap() {
        match row["family"].as_str().unwrap() {
            "severity" => assert_fixture_case::<SeverityMutation>(row),
            "validated" => assert_fixture_case::<ValidatedMutation>(row),
            _ => {}
        }
    }
}

#[test]
fn direct_store_fixture_lossy_oracle() {
    let op = LossyMutation::SetN(LossySetN { n: 7 });
    let json = serde_json::to_value(&op).unwrap();
    assert_eq!(json, serde_json::json!({"SetN": 7}));
    let decoded = serde_json::from_value::<LossyMutation>(json).unwrap();
    assert_eq!(decoded, LossyMutation::SetN(LossySetN { n: 0 }));
    assert_ne!(decoded, op);
    assert_eq!(LossyMutation::decode_op(&op.encode_op().unwrap()).unwrap(), decoded);
    for json in ["{\"SetN\":null}", "{\"SetN\":\"7\"}", "{\"SetN\":true}", "{\"SetN\":{}}", "{\"SetN\":2147483648}", "{\"SetN\":7,\"unknown\":true}"] {
        assert!(serde_json::from_str::<LossyMutation>(json).is_err());
    }
}

const ONE_ITEM_PUBLICATION_FIXTURE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🔨️modules/🏪️store/🧫️fixtures/artifact-store-one-item-publication-v1/🔣️.json"));
const EPHEMERAL_ONE_ITEM_PUBLICATION_FIXTURE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🔨️modules/🏪️store/🧫️fixtures/artifact-ephemeral-one-item-publication-v1/🔣️.json"));

struct SerdeOneItemPublicationOracle {
    maximum_items: usize,
    maximum_bytes: usize,
    maximum_retries: u8,
}

impl SerdeOneItemPublicationOracle {
    fn from_fixture(fixture: &serde_json::Value) -> Self {
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
        if base_generation == live_generation && base_revision == live_revision {
            "accepted"
        } else {
            "fault"
        }
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
    let oracle = SerdeOneItemPublicationOracle::from_fixture(&fixture);
    assert_eq!(fixture["capacities"]["historyItems"], crate::os_vcs::ARTIFACT_HISTORY_LEDGER_CAPACITY);
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
    let source = include_str!("../../🦀️.rs");
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
    assert!(close.contains("maximum_items.min(1)"));
    assert!(close.contains("maximum_bytes"));
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
    assert_eq!(fold.matches("try_reserve_exact(").count(), 3, "the batched stage reserves its whole admitted forwards/inverse/metadata capacity exactly once");
    assert!(fold.contains("inverse.reverse();"), "a staged item's inverse block is reversed exactly as replay_mutations reverses one operation's inverse");
}

fn drain_channel_for_test(remote: &ChannelBackboneRemote) -> Result<Vec<BackboneMessage>, VcsError> {
    let mut messages = Vec::new();
    while let Some(message) = remote.try_pop_front()? {
        messages.push(message);
    }
    Ok(messages)
}

#[test]
fn snapshot_read_lease_capacity_plus_one_returns_the_exact_owner_and_every_registered_owner_retires() {
    let registry = Arc::new(SnapshotReadLeaseRegistry::new());
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
    let registry = Arc::new(SnapshotReadLeaseRegistry::new());
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
    let registry = Arc::new(SnapshotReadLeaseRegistry::new());
    let owner = Arc::new(String::from("retained"));
    let lease = registry.try_issue(owner.clone()).expect("snapshot read lease admission");
    let index = lease.index;
    let generation = lease.generation;
    let read = ErasedSnapshotRead::new(owner, lease);
    assert!(registry.contains(index, generation));
    assert!(!registry.terminal_is_empty());
    drop(read);
    assert!(registry.contains(index, generation));
    let mut retired = None;
    for _ in 0..SNAPSHOT_READ_LEASE_CAPACITY {
        if let Some(owner) = registry.try_take_one_returned::<String>().expect("one fixed cleanup probe") {
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
    let registry = Arc::new(SnapshotReadLeaseRegistry::new());
    let owner = Arc::new(7u32);
    let lease = registry.try_issue(owner.clone()).expect("first exact lease");
    let index = lease.index;
    let generation = lease.generation;
    let read = ErasedSnapshotRead::new(owner, lease);
    drop(match read.into_typed::<u32>(&registry) {
        Ok(owner) => owner,
        Err(_) => panic!("first exact retirement"),
    });
    assert!(registry.try_take(index, generation).is_err());
    assert!(registry.terminal_is_empty());
}

#[test]
fn snapshot_read_lease_contention_returns_the_exact_untouched_owner() {
    let registry = Arc::new(SnapshotReadLeaseRegistry::new());
    let owner = Arc::new(String::from("contention-owner"));
    let guard = registry.state.lock().expect("hold exact registry contention fixture");
    let rejected = match registry.try_issue(owner.clone()) {
        Ok(_) => panic!("nonblocking admission rejects while the fixed registry is contended"),
        Err(rejected) => rejected,
    };
    assert!(Arc::ptr_eq(&owner, &rejected));
    assert_eq!(Arc::strong_count(&owner), 2, "contention creates no hidden owner clone");
    drop(guard);
    let lease = registry.try_issue(rejected.clone()).expect("the same owner retries after contention");
    let read = ErasedSnapshotRead::new(rejected, lease);
    drop(match read.into_typed::<String>(&registry) {
        Ok(owner) => owner,
        Err(_) => panic!("retried exact owner handback succeeds"),
    });
    assert!(registry.terminal_is_empty());
}

#[test]
fn artifact_store_edit_retirement_is_interruptible_and_terminal_empty_after_deep_strings() {
    let edit = Edit::<DemoMutation> {
        id: "edit-owner".repeat(128),
        actor: Some("actor-owner".repeat(128)),
        forwards: Vec::new(),
        inverse: Vec::new(),
        mutation_meta: Vec::new(),
        description: Some("description".repeat(128)),
        coalesce_key: Some("coalesce".repeat(128)),
        sequence_number: 1,
        started_at: "started".repeat(128),
        finished_at: Some("finished".repeat(128)),
    };
    let mut retirement = ArtifactStoreEditRetirement::new(edit).expect("empty mutation vectors are admitted");
    let mut turns = 0;
    while !retirement.terminal_is_empty() {
        let step = retirement.close_step(1, 7).expect("bounded close step");
        match step {
            SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1 && released_bytes <= 7);
            }
            SnapshotRetirementStep::Complete => assert!(retirement.terminal_is_empty()),
            SnapshotRetirementStep::Blocked => panic!("owned edit retirement has no external wait"),
        }
        turns += 1;
        assert!(turns < 10_000, "edit retirement must make finite progress across interruptions");
    }
}

#[test]
fn artifact_store_edit_retirement_rejects_and_returns_the_exact_nonempty_mutation_owner() {
    let edit = Edit::<DemoMutation> {
        id: "edit-owner".into(),
        actor: None,
        forwards: vec![DemoMutation::SetN(SetN { n: 7 })],
        inverse: Vec::new(),
        mutation_meta: Vec::new(),
        description: None,
        coalesce_key: None,
        sequence_number: 1,
        started_at: "now".into(),
        finished_at: None,
    };
    let rejected = match ArtifactStoreEditRetirement::new(edit) {
        Err(rejected) => rejected,
        Ok(_) => panic!("a nonempty mutation vector must remain with its exact domain owner"),
    };
    assert_eq!(rejected.forwards, vec![DemoMutation::SetN(SetN { n: 7 })]);
}

#[test]
fn artifact_store_history_metadata_retirement_cursors_authors_pins_and_ids_under_each_grant() {
    let checkpoint = Checkpoint {
        id: "checkpoint".repeat(64),
        change_ids: vec!["change-a".repeat(64), "change-b".repeat(64)],
        parent_id: Some("parent".repeat(64)),
        authors: vec![Author { id: "author".repeat(64), name: "name".repeat(64), avatar: Some("avatar".repeat(64)) }],
        message: Some("message".repeat(64)),
        timestamp: "timestamp".repeat(64),
        composition_pins: vec![crate::os_vcs::CompositionPin {
            child_ref: crate::os_io::ArtifactRef { artifact_id: "child".repeat(64), dialect: crate::os_io::ArtifactDialect { artifact_kind: "kind".repeat(64), standard: "standard".repeat(64), subset: "subset".repeat(64) } },
            checkpoint_id: "child-checkpoint".repeat(64),
        }],
    };
    let mut retirement = ArtifactStoreHistoryMetadataRetirement::checkpoint(checkpoint);
    let mut turns = 0;
    while !retirement.terminal_is_empty() {
        let step = retirement.close_step(1, 11).expect("bounded history metadata close");
        match step {
            SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= 11),
            SnapshotRetirementStep::Complete => assert!(retirement.terminal_is_empty()),
            SnapshotRetirementStep::Blocked => panic!("owned history metadata has no external wait"),
        }
        turns += 1;
        assert!(turns < 10_000, "history metadata retirement must terminate across repeated interruption");
    }
}

#[test]
fn fixed_edit_message_ledger_resolves_probe_collisions_without_a_parallel_runtime_index() {
    let mut first_by_slot = BTreeMap::new();
    let mut collision = None;
    for index in 0..ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY * 2 {
        let id = format!("edit-collision-{index}");
        let slot = ArtifactEditMessageLedger::hash(&id) % ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY;
        if let Some(first) = first_by_slot.insert(slot, id.clone()) {
            collision = Some((first, id));
            break;
        }
    }
    let (first, second) = collision.expect("pigeonhole probe collision exists");
    let entries = vec![
        crate::os_spr::EditMessages { edit_id: first.clone(), messages: vec![crate::os_spr::MutationMessage::info("mutation.cascade", "first")] },
        crate::os_spr::EditMessages { edit_id: second.clone(), messages: vec![crate::os_spr::MutationMessage::info("mutation.cascade", "second")] },
    ];
    let mut ledger = ArtifactEditMessageLedger::try_from_entries(entries).unwrap_or_else(|_| panic!("colliding fixed entries probe deterministically"));
    assert_eq!(ledger.get_by_id(&first).and_then(|entry| entry.messages.first()).map(|message| message.message.as_str()), Some("first"));
    assert_eq!(ledger.get_by_id(&second).and_then(|entry| entry.messages.first()).map(|message| message.message.as_str()), Some("second"));
    while let Some(entry) = ledger.pop() {
        let mut retirement = ArtifactStoreMessageLedgerRetirement::new(entry.edit_id, entry.messages);
        while !retirement.terminal_is_empty() {
            let _ = retirement.close_step(1, ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES).expect("colliding owner bounded close");
        }
    }
}

fn drain_rejected_edit_message_ledger(mut rejected: ArtifactEditMessageLedgerRejected) {
    for _ in 0..ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY * 32 {
        match rejected.close_step(1, ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES).expect("rejected ledger bounded close") {
            SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES);
            }
            SnapshotRetirementStep::Complete => {
                assert!(rejected.terminal_is_empty());
                return;
            }
            SnapshotRetirementStep::Blocked => panic!("an owned rejected ledger has no external wait"),
        }
    }
    panic!("rejected ledger close must terminate under repeated one-owner grants");
}

#[test]
fn fixed_edit_message_ledger_rejects_duplicate_and_capacity_plus_one_with_exact_retained_owners() {
    let duplicate = vec![
        crate::os_spr::EditMessages { edit_id: "duplicate".into(), messages: vec![crate::os_spr::MutationMessage::info("mutation.duplicate", "first")] },
        crate::os_spr::EditMessages { edit_id: "duplicate".into(), messages: vec![crate::os_spr::MutationMessage::info("mutation.duplicate", "second")] },
    ];
    let duplicate = match ArtifactEditMessageLedger::try_from_entries(duplicate) {
        Err(rejected) => rejected,
        Ok(mut ledger) => {
            while ledger.pop().is_some() {}
            panic!("duplicate identities must fail before payload adoption")
        }
    };
    assert_eq!(duplicate.entries.len(), 2, "duplicate rejection retains the exact owner batch");
    drain_rejected_edit_message_ledger(duplicate);

    let plus_one = (0..=ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY).map(|index| crate::os_spr::EditMessages { edit_id: format!("edit-{index}"), messages: Vec::new() }).collect();
    let plus_one = match ArtifactEditMessageLedger::try_from_entries(plus_one) {
        Err(rejected) => rejected,
        Ok(mut ledger) => {
            while ledger.pop().is_some() {}
            panic!("capacity plus one must fail before payload adoption")
        }
    };
    assert_eq!(plus_one.entries.len(), ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY + 1, "capacity rejection retains every exact owner");
    drain_rejected_edit_message_ledger(plus_one);
}

#[test]
fn fixed_edit_message_ledger_preserves_order_and_rejects_stale_generation_after_slot_reuse() {
    let entries =
        vec![crate::os_spr::EditMessages { edit_id: "first".into(), messages: Vec::new() }, crate::os_spr::EditMessages { edit_id: "middle".into(), messages: Vec::new() }, crate::os_spr::EditMessages { edit_id: "last".into(), messages: Vec::new() }];
    let mut ledger = match ArtifactEditMessageLedger::try_from_entries(entries) {
        Ok(ledger) => ledger,
        Err(rejected) => {
            drain_rejected_edit_message_ledger(rejected);
            panic!("valid fixed ledger must be admitted")
        }
    };
    let middle = ledger.find_ticket("middle").expect("middle ticket");
    let removed = ledger.remove(middle).expect("middle owner detaches in O(1)");
    assert_eq!(removed.edit_id, "middle");
    let reused = ledger.try_push(crate::os_spr::EditMessages { edit_id: "replacement".into(), messages: Vec::new() }).unwrap_or_else(|_| panic!("released slot is reusable"));
    assert_eq!(reused.slot, middle.slot);
    assert_ne!(reused.generation, middle.generation);
    assert!(ledger.get(middle).is_none(), "stale generation cannot resolve the reused slot");
    assert_eq!(ledger.iter().map(|entry| entry.edit_id.as_str()).collect::<Vec<_>>(), vec!["first", "last", "replacement"]);
    let removed = ArtifactStoreMessageLedgerRetirement::new(removed.edit_id, removed.messages);
    let mut removed = removed;
    while !removed.terminal_is_empty() {
        let _ = removed.close_step(1, ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES).expect("removed owner bounded close");
    }
    while let Some(entry) = ledger.pop() {
        let mut retirement = ArtifactStoreMessageLedgerRetirement::new(entry.edit_id, entry.messages);
        while !retirement.terminal_is_empty() {
            let _ = retirement.close_step(1, ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES).expect("ledger owner bounded close");
        }
    }
    assert!(ledger.terminal_is_empty());
}

struct UnitOwnedRetirement {
    value: Option<()>,
}

impl ErasedSnapshotRetirement for UnitOwnedRetirement {
    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.value.take().is_some() {
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.value.is_none()
    }
}

struct UnitOwnedRetirementFactory;

impl ArtifactOwnedValueRetirementFactory<()> for UnitOwnedRetirementFactory {
    fn retire_owned(&self, value: ()) -> Box<dyn ErasedSnapshotRetirement> {
        Box::new(UnitOwnedRetirement { value: Some(value) })
    }
}

#[test]
fn displaced_owner_reservations_preserve_capacity_generation_and_interrupted_close() {
    let mut retirements = ArtifactStoreDisplacedRetirements::new();
    let large = retirements.reserve_owner_slots(ARTIFACT_STORE_DISPLACED_RETIREMENT_CAPACITY - 1).expect("exact capacity minus one is reserved");
    let mut final_slot = retirements.reserve_owner_slots(1).expect("the final exact retirement slot is independently reserved");
    assert!(retirements.reserve_owner_slots(1).is_err(), "capacity +1 cannot consume an owner");
    retirements.release_owner_slots(large).expect("unused large reservation returns atomically");
    retirements.push_owner_reserved(&mut final_slot, Box::new(UnitOwnedRetirement { value: Some(()) })).unwrap_or_else(|_| panic!("the exact final-slot reservation retains its owner"));
    retirements.release_owner_slots(final_slot).expect("consumed reservation has no live credit");
    assert_eq!(retirements.close_step(0, 0).expect("zero grant is non-destructive"), SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
    assert_eq!(retirements.close_step(1, 0).expect("one grant advances one nested owner"), SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
    assert_eq!(retirements.close_step(1, 0).expect("terminal shell is reclaimed separately"), SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
    assert_eq!(retirements.close_step(1, 0).expect("drained authority is terminal"), SnapshotRetirementStep::Complete);
    assert!(retirements.terminal_is_empty());

    let mut reservations = Vec::new();
    for _ in 0..ARTIFACT_STORE_DISPLACED_RESERVATION_CAPACITY {
        reservations.push(retirements.reserve_owner_slots(1).expect("fixed reservation registry admits its exact capacity"));
    }
    assert!(retirements.reserve_owner_slots(1).is_err(), "reservation-registry +1 fails before owner construction");
    for reservation in reservations {
        retirements.release_owner_slots(reservation).expect("each exact generation-qualified reservation returns once");
    }
    assert!(retirements.terminal_is_empty());
}

fn completed_record_owner(id: &str) -> Box<dyn ArtifactEnvelopeCompletedRecord<(), ()>> {
    Box::new(ArtifactEnvelopeCompletedRecordOwner::new(create_document_envelope("test.completed-record/v1", id, (), None), Arc::new(UnitOwnedRetirementFactory), Arc::new(UnitOwnedRetirementFactory)))
}

fn drain_completed_record(mut owner: Box<dyn ArtifactEnvelopeCompletedRecord<(), ()>>) {
    for _ in 0..10_000 {
        match owner.close_step(1, 17).expect("completed record bounded close") {
            SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= 17);
            }
            SnapshotRetirementStep::Blocked => panic!("owned completed record has no external wait"),
            SnapshotRetirementStep::Complete => {
                assert!(owner.terminal_is_empty());
                drop(owner);
                return;
            }
        }
    }
    panic!("completed record must reach terminal empty under repeated one-owner grants");
}

fn admit_completed_record(registry: &ArtifactEnvelopeCompletedRecordRegistry<(), ()>, owner: Box<dyn ArtifactEnvelopeCompletedRecord<(), ()>>) -> ArtifactEnvelopeCompletedRecordTicket {
    match registry.try_admit(owner) {
        Ok(ticket) => ticket,
        Err((fault, owner)) => {
            drain_completed_record(owner);
            panic!("completed record admission unexpectedly failed: {fault:?}")
        }
    }
}

struct TestCompletedRecordTarget {
    expected: &'static str,
    adopted: Option<Box<dyn ArtifactEnvelopeCompletedRecord<(), ()>>>,
}

impl ArtifactEnvelopeCompletedRecordTarget<(), ()> for TestCompletedRecordTarget {
    fn try_adopt_completed(&mut self, envelope: ArtifactEnvelope<(), ()>) -> Result<(), ArtifactEnvelope<(), ()>> {
        if envelope.id != self.expected || self.adopted.is_some() {
            return Err(envelope);
        }
        self.adopted = Some(Box::new(ArtifactEnvelopeCompletedRecordOwner::new(envelope, Arc::new(UnitOwnedRetirementFactory), Arc::new(UnitOwnedRetirementFactory))));
        Ok(())
    }
}

#[test]
fn completed_envelope_registry_rejects_capacity_plus_one_and_reuses_slots_with_aba_safety() {
    let registry = ArtifactEnvelopeCompletedRecordRegistry::<(), ()>::new();
    let mut tickets = Vec::new();
    for index in 0..ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY {
        tickets.push(admit_completed_record(&registry, completed_record_owner(&format!("record-{index}"))));
    }
    let (fault, rejected) = registry.try_admit(completed_record_owner("plus-one")).expect_err("capacity plus one returns exact owner");
    assert_eq!(fault, ArtifactEnvelopeCompletedRecordFault::Capacity);
    drain_completed_record(rejected);

    let stale = tickets.remove(0);
    let owner = registry.try_detach(stale).expect("exact first completed owner");
    drain_completed_record(owner);
    let reused = admit_completed_record(&registry, completed_record_owner("reused"));
    assert_eq!(reused.index(), stale.index());
    assert_ne!(reused.generation(), stale.generation());
    match registry.try_detach(stale) {
        Err(fault) => assert_eq!(fault, ArtifactEnvelopeCompletedRecordFault::Stale),
        Ok(owner) => {
            drain_completed_record(owner);
            panic!("stale completed ticket detached the reused owner")
        }
    }
    tickets.push(reused);
    for ticket in tickets {
        drain_completed_record(registry.try_detach(ticket).expect("exact completed owner drains"));
    }
    assert!(registry.terminal_is_empty());
}

#[test]
fn completed_envelope_registry_publication_is_exact_once_and_close_cursor_is_interruptible() {
    let registry = ArtifactEnvelopeCompletedRecordRegistry::<(), ()>::new();
    let ticket = admit_completed_record(&registry, completed_record_owner("take-once"));
    let mut rejected = TestCompletedRecordTarget { expected: "wrong", adopted: None };
    assert!(!registry.try_publish_to(ticket, &mut rejected).expect("consumer rejection retains exact record"));
    let mut target = TestCompletedRecordTarget { expected: "take-once", adopted: None };
    assert!(registry.try_publish_to(ticket, &mut target).expect("exact consumer atomically adopts completed record"));
    assert_eq!(registry.try_publish_to(ticket, &mut target).expect_err("completed record is exact once"), ArtifactEnvelopeCompletedRecordFault::Stale);
    drain_completed_record(target.adopted.take().expect("test target retained adopted owner"));

    let ticket = admit_completed_record(&registry, completed_record_owner("interrupted-close"));
    let mut cursor = 0;
    assert_eq!(registry.try_next_ticket(&mut cursor).expect("fixed next ticket"), Some(ticket));
    let owner = registry.try_detach(ticket).expect("close pump detaches exact record");
    drain_completed_record(owner);
    assert!(registry.terminal_is_empty());
}

pub(super) const OWNED_SCHEMA_TEST_FIELDS: &[OwnedSchemaFieldSpec] = &[
    OwnedSchemaFieldSpec { id: 1, key: "schema", required: true },
    OwnedSchemaFieldSpec { id: 2, key: "id", required: true },
    OwnedSchemaFieldSpec { id: 3, key: "quote\"", required: false },
    OwnedSchemaFieldSpec { id: 4, key: "slash\\", required: false },
    OwnedSchemaFieldSpec { id: 5, key: "control\u{0001}", required: false },
    OwnedSchemaFieldSpec { id: 6, key: "asciiA", required: false },
    OwnedSchemaFieldSpec { id: 7, key: "escaped/\u{0008}\u{000c}\n\r\t", required: false },
];

pub(super) fn owned_schema_test_cursor(chunks: &[&[u8]]) -> OwnedSchemaRecordCursor {
    let byte_count = chunks.iter().map(|chunk| chunk.len()).sum();
    let mut pages = OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits { maximum_pages: chunks.len(), maximum_bytes: byte_count }).expect("fixed schema credits");
    for chunk in chunks {
        let page = OwnedSchemaDecodePage::try_from_slice(chunk).expect("bounded test page");
        pages.admit_page(page).unwrap_or_else(|_| panic!("pre-admitted test page"));
    }
    pages.seal().expect("seal exact test extent");
    let tokens = OwnedSchemaTokenCursor::try_new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), pages).unwrap_or_else(|_| panic!("sealed token source"));
    OwnedSchemaRecordCursor::try_new(OwnedSchemaRecordSpec { fields: OWNED_SCHEMA_TEST_FIELDS }, tokens).unwrap_or_else(|_| panic!("valid fixed test schema"))
}

pub(super) fn drive_owned_schema(cursor: &mut OwnedSchemaRecordCursor, fuel: u64) -> Result<Vec<(u16, OwnedSchemaTokenKind, bool)>, OwnedSchemaDecodeDiagnostic> {
    let mut events = Vec::new();
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    for _ in 0..100_000 {
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::OperationId(1),
            semio_framework_job::Generation(1),
            semio_framework_job::StepBudget::new(fuel, u64::MAX),
            cancel.clone(),
            semio_framework_job::default_now_us,
            &mut preview_sequence,
        );
        match cursor.step(&mut context) {
            OwnedSchemaRecordStep::Pending => {}
            OwnedSchemaRecordStep::FieldToken { field_id, token, terminal } => events.push((field_id, token.kind, terminal)),
            OwnedSchemaRecordStep::Complete => return Ok(events),
            OwnedSchemaRecordStep::Fault(diagnostic) => return Err(diagnostic),
            OwnedSchemaRecordStep::Cancelled => panic!("live schema cursor cancelled unexpectedly"),
        }
    }
    panic!("schema cursor failed to terminate under repeated fixed fuel")
}

#[test]
fn owned_schema_pages_reject_capacity_plus_one_with_the_exact_page_untouched() {
    let mut pages = OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits { maximum_pages: 1, maximum_bytes: OWNED_SCHEMA_DECODE_PAGE_BYTES }).expect("one page credits");
    let first = OwnedSchemaDecodePage::try_from_slice(&[b'a'; OWNED_SCHEMA_DECODE_PAGE_BYTES]).expect("maximum page");
    pages.admit_page(first).expect("maximum page admitted");
    let rejected = OwnedSchemaDecodePage::try_from_slice(b"tail").expect("bounded rejected page");
    let (fault, rejected) = pages.admit_page(rejected).expect_err("capacity plus one rejected before adoption");
    assert_eq!(fault, OwnedSchemaDecodeAdmissionFault::PageCapacity);
    assert_eq!(rejected.len(), 4, "the exact rejected owner is returned untouched");
    assert_eq!(pages.page_count(), 1);

    let mut byte_limited = OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits { maximum_pages: 1, maximum_bytes: 3 }).expect("three-byte credits");
    let rejected = OwnedSchemaDecodePage::try_from_slice(b"four").expect("bounded page owner");
    let (fault, rejected) = byte_limited.admit_page(rejected).expect_err("aggregate byte plus one rejected before adoption");
    assert_eq!(fault, OwnedSchemaDecodeAdmissionFault::ByteCapacity);
    assert_eq!(rejected.len(), 4);
    assert!(byte_limited.terminal_is_empty());
}

#[test]
fn owned_schema_record_resumes_across_a_partial_terminal_page_and_preserves_field_order() {
    let mut first = br#"{"schema":""#.to_vec();
    first.resize(OWNED_SCHEMA_DECODE_PAGE_BYTES, b'a');
    let second = br#"","id":"document"}"#;
    let mut cursor = owned_schema_test_cursor(&[&first, second]);
    let events = drive_owned_schema(&mut cursor, 7).expect("valid cross-page record");
    assert_eq!(events.iter().filter(|(field, _, terminal)| *field == 1 && *terminal).count(), 1);
    assert_eq!(events.iter().filter(|(field, _, terminal)| *field == 2 && *terminal).count(), 1);
    assert_eq!(events.first().map(|event| event.0), Some(1));
    assert_eq!(events.last().map(|event| event.0), Some(2));
}

#[test]
fn owned_schema_record_reports_exact_invalid_utf8_unknown_and_duplicate_diagnostics() {
    let mut first = br#"{"schema":""#.to_vec();
    first.resize(OWNED_SCHEMA_DECODE_PAGE_BYTES - 1, b'a');
    first.push(0xe2);
    let invalid_tail = br#"!","id":"document"}"#;
    let invalid = drive_owned_schema(&mut owned_schema_test_cursor(&[&first, invalid_tail]), 11).expect_err("invalid continuation faults");
    assert_eq!(invalid.code, "schema-json.invalid-utf8");
    assert_eq!(invalid.offset, OWNED_SCHEMA_DECODE_PAGE_BYTES as u64);

    let unknown = drive_owned_schema(&mut owned_schema_test_cursor(&[br#"{"schema":"v1","unexpected":1,"id":"document"}"#]), 64).expect_err("unknown field faults");
    assert_eq!(unknown.code, "schema-json.unknown-field");
    assert_eq!(unknown.path.as_str(), "$");

    let duplicate = drive_owned_schema(&mut owned_schema_test_cursor(&[br#"{"schema":"v1","schema":"v2","id":"document"}"#]), 64).expect_err("duplicate field faults");
    assert_eq!(duplicate.code, "schema-json.duplicate-field");
    assert_eq!(duplicate.path.as_str(), "$.schema");

    let truncated = drive_owned_schema(&mut owned_schema_test_cursor(&[br#"{"schema":"unterminated"#]), 9).expect_err("truncated token faults");
    assert_eq!(truncated.code, "schema-json.truncated-token");
}

#[test]
fn owned_schema_record_rejects_stale_operation_and_generation_before_consuming_input() {
    let mut cursor = owned_schema_test_cursor(&[br#"{"schema":"v1","id":"document"}"#]);
    let mut preview_sequence = 0;
    let mut context = semio_framework_job::StepContext::new(
        semio_framework_job::OperationId(1),
        semio_framework_job::Generation(2),
        semio_framework_job::StepBudget::new(64, u64::MAX),
        semio_framework_job::root_cancel_token(),
        semio_framework_job::default_now_us,
        &mut preview_sequence,
    );
    let OwnedSchemaRecordStep::Fault(diagnostic) = cursor.step(&mut context) else { panic!("stale generation must fault before syntax work") };
    assert_eq!(diagnostic.code, "schema-json.stale-authority");
    assert_eq!(diagnostic.offset, 0);
    assert!(matches!(cursor.close_step(1), SnapshotRetirementStep::Pending { released_items: 1, .. }));
    assert_eq!(cursor.close_step(1), SnapshotRetirementStep::Complete);
}

fn next_owned_schema_field_token(cursor: &mut OwnedSchemaRecordCursor, expected_field: u16) -> OwnedSchemaToken {
    let mut preview_sequence = 0;
    for _ in 0..1_000 {
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::OperationId(1),
            semio_framework_job::Generation(1),
            semio_framework_job::StepBudget::new(32, u64::MAX),
            semio_framework_job::root_cancel_token(),
            semio_framework_job::default_now_us,
            &mut preview_sequence,
        );
        match cursor.step(&mut context) {
            OwnedSchemaRecordStep::Pending => {}
            OwnedSchemaRecordStep::FieldToken { field_id, token, terminal: true } if field_id == expected_field => return token,
            OwnedSchemaRecordStep::FieldToken { .. } => {}
            other => panic!("expected field token, got {other:?}"),
        }
    }
    panic!("schema field token did not arrive under repeated grants")
}

#[test]
fn owned_schema_string_unescapes_into_fixed_storage_and_rejects_semantic_byte_plus_one() {
    let mut cursor = owned_schema_test_cursor(&[br#"{"schema":"line\n\uD83D\uDE00","id":"document"}"#]);
    let token = next_owned_schema_field_token(&mut cursor, 1);
    let mut string = OwnedSchemaStringAuthority::<32>::try_new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), token, OwnedSchemaPath::field("schema").expect("bounded field path")).expect("schema token is a string");
    let mut preview_sequence = 0;
    for _ in 0..100 {
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::OperationId(1),
            semio_framework_job::Generation(1),
            semio_framework_job::StepBudget::new(3, u64::MAX),
            semio_framework_job::root_cancel_token(),
            semio_framework_job::default_now_us,
            &mut preview_sequence,
        );
        match string.step(&cursor, &mut context) {
            OwnedSchemaStringStep::Pending => {}
            OwnedSchemaStringStep::Complete => break,
            other => panic!("bounded escaped string failed: {other:?}"),
        }
    }
    assert_eq!(string.as_str(), Some("line\n😀"));
    assert_eq!(string.take_string().as_deref(), Some("line\n😀"));
    assert!(string.take_string().is_none(), "semantic string publishes exactly once");
    assert!(string.terminal_is_empty());

    let mut cursor = owned_schema_test_cursor(&[br#"{"schema":"four","id":"document"}"#]);
    let token = next_owned_schema_field_token(&mut cursor, 1);
    let mut limited = OwnedSchemaStringAuthority::<3>::try_new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), token, OwnedSchemaPath::field("schema").expect("bounded field path")).expect("schema token is a string");
    let mut preview_sequence = 0;
    let mut context = semio_framework_job::StepContext::new(
        semio_framework_job::OperationId(1),
        semio_framework_job::Generation(1),
        semio_framework_job::StepBudget::new(32, u64::MAX),
        semio_framework_job::root_cancel_token(),
        semio_framework_job::default_now_us,
        &mut preview_sequence,
    );
    let OwnedSchemaStringStep::Fault(diagnostic) = limited.step(&cursor, &mut context) else { panic!("semantic byte plus one must fault") };
    assert_eq!(diagnostic.code, "schema-json.string-byte-capacity");
    assert_eq!(diagnostic.path.as_str(), "$.schema");
    assert!(limited.terminal_is_empty(), "fixed inline rejection has no deep owner to retire");
}

#[test]
fn cancelled_owned_schema_cursor_retires_one_real_page_per_close_grant() {
    let first = [b' '; OWNED_SCHEMA_DECODE_PAGE_BYTES];
    let mut cursor = owned_schema_test_cursor(&[&first, br#"{"schema":"v1","id":"document"}"#]);
    let cancel = semio_framework_job::root_cancel_token();
    cancel.cancel_now();
    let mut preview_sequence = 0;
    let mut context =
        semio_framework_job::StepContext::new(semio_framework_job::OperationId(2), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX), cancel, semio_framework_job::default_now_us, &mut preview_sequence);
    assert_eq!(cursor.step(&mut context), OwnedSchemaRecordStep::Cancelled);
    assert!(matches!(cursor.close_step(1), SnapshotRetirementStep::Pending { released_items: 1, released_bytes } if released_bytes <= OWNED_SCHEMA_DECODE_PAGE_BYTES));
    assert!(matches!(cursor.close_step(1), SnapshotRetirementStep::Pending { released_items: 1, released_bytes } if released_bytes <= OWNED_SCHEMA_DECODE_PAGE_BYTES));
    assert_eq!(cursor.close_step(1), SnapshotRetirementStep::Complete);
    assert!(cursor.terminal_is_empty());
}

struct TestEnvelopeFieldDecoder {
    terminal: bool,
    accepted: usize,
}

impl ArtifactEnvelopeFieldDecoder<(), ()> for TestEnvelopeFieldDecoder {
    fn accept_field_token(&mut self, _field_id: u16, _token: OwnedSchemaToken, terminal: bool, _source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        cx.consume_fuel(1);
        self.accepted += 1;
        Ok(if terminal { ArtifactEnvelopeFieldDecodeStep::FieldComplete } else { ArtifactEnvelopeFieldDecodeStep::TokenComplete })
    }

    fn finish_record(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        cx.consume_fuel(1);
        self.terminal = true;
        Ok(ArtifactEnvelopeFieldDecodeStep::RecordComplete)
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<SnapshotRetirementStep, OwnedSchemaDecodeDiagnostic> {
        if maximum_items == 0 {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.terminal = true;
        Ok(SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal
    }
}

fn artifact_envelope_decode_test_cursor(chunks: &[&[u8]]) -> OwnedSchemaRecordCursor {
    let bytes = chunks.iter().map(|chunk| chunk.len()).sum();
    let mut pages = OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits { maximum_pages: chunks.len(), maximum_bytes: bytes }).expect("exact envelope decode credits");
    for chunk in chunks {
        let page = OwnedSchemaDecodePage::try_from_slice(chunk).expect("bounded envelope page");
        pages.admit_page(page).unwrap_or_else(|_| panic!("admitted envelope page"));
    }
    pages.seal().expect("sealed envelope pages");
    let tokens = OwnedSchemaTokenCursor::try_new(semio_framework_job::OperationId(91), semio_framework_job::Generation(7), pages).unwrap_or_else(|_| panic!("sealed envelope token cursor"));
    OwnedSchemaRecordCursor::try_new(artifact_envelope_owned_schema(), tokens).unwrap_or_else(|_| panic!("valid envelope schema"))
}

fn drive_test_envelope_field_return(registry: &Arc<ArtifactEnvelopeFieldDecoderRegistry<(), ()>>, detached: &mut Option<ArtifactEnvelopeReturnedFieldDecoder<(), ()>>) {
    if let Some(retirement) = detached.as_mut() {
        let step = retirement.close_step(1, ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("test app return pump closes one exact decoder owner");
        if step == SnapshotRetirementStep::Complete {
            assert!(retirement.terminal_is_empty());
            drop(detached.take());
        }
        return;
    }
    let Some(ticket) = registry.next_returned_ticket() else { return };
    *detached = Some(registry.take_returned_ticket(ticket).expect("test app return pump detaches the exact returned generation"));
}

#[test]
fn artifact_envelope_decode_withholds_success_until_field_and_page_owners_are_terminal_empty() {
    use semio_framework_job::InteractiveJob;

    let registry = ArtifactEnvelopeFieldDecoderRegistry::new();
    let mut job = ArtifactEnvelopeDecodeAuthority::<(), ()>::try_new(
        artifact_envelope_decode_test_cursor(&[br#"{"schema":"s","id":"i","vcs":{},"editMessages":[],"conflicts":[]}"#]),
        &registry,
        Box::new(TestEnvelopeFieldDecoder { terminal: false, accepted: 0 }),
    )
    .unwrap_or_else(|_| panic!("fixed field decoder admission"));
    let mut preview_sequence = 0;
    let mut detached = None;
    for turn in 0..1_000 {
        drive_test_envelope_field_return(&registry, &mut detached);
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::OperationId(91),
            semio_framework_job::Generation(7),
            semio_framework_job::StepBudget::new(4, u64::MAX),
            semio_framework_job::root_cancel_token(),
            semio_framework_job::default_now_us,
            &mut preview_sequence,
        );
        match job.step(&mut context) {
            semio_framework_job::StepOutcome::Yield => assert!(!job.terminal_is_empty(), "a yielded decode retains exact owner authority"),
            semio_framework_job::StepOutcome::Complete(_) => {
                assert!(job.terminal_is_empty(), "success is exposed only after exact field and page terminal witnesses");
                assert!(registry.terminal_is_empty(), "successful decode reclaimed its exact registered field owner");
                assert!(turn > 1, "decode and close span multiple bounded turns");
                return;
            }
            outcome => panic!("valid envelope decode produced {outcome:?}"),
        }
    }
    panic!("valid envelope decode did not terminate");
}

#[test]
fn cancelled_and_rejected_envelope_decodes_close_one_exact_owner_per_grant() {
    use semio_framework_job::InteractiveJob;

    let first = [b' '; ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
    let registry = ArtifactEnvelopeFieldDecoderRegistry::new();
    let mut cancelled = ArtifactEnvelopeDecodeAuthority::<(), ()>::try_new(
        artifact_envelope_decode_test_cursor(&[&first, br#"{"schema":"s","id":"i","vcs":{},"editMessages":[],"conflicts":[]}"#]),
        &registry,
        Box::new(TestEnvelopeFieldDecoder { terminal: false, accepted: 0 }),
    )
    .unwrap_or_else(|_| panic!("fixed cancelled decoder admission"));
    let cancel = semio_framework_job::root_cancel_token();
    cancel.cancel_now();
    let mut preview_sequence = 0;
    let mut turns = 0;
    let mut detached = None;
    loop {
        drive_test_envelope_field_return(&registry, &mut detached);
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::OperationId(91),
            semio_framework_job::Generation(7),
            semio_framework_job::StepBudget::new(1, u64::MAX),
            cancel.clone(),
            semio_framework_job::default_now_us,
            &mut preview_sequence,
        );
        turns += 1;
        match cancelled.step(&mut context) {
            semio_framework_job::StepOutcome::Yield => assert!(!cancelled.terminal_is_empty()),
            semio_framework_job::StepOutcome::Cancelled => break,
            outcome => panic!("cancelled envelope decode produced {outcome:?}"),
        }
        assert!(turns < 16);
    }
    assert!(cancelled.terminal_is_empty());
    assert!(registry.terminal_is_empty());
    assert!(turns >= 5, "field authority, two pages, and terminal shells require distinct grants");

    let diagnostic = OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.test-rejected", offset: 0, line: 1, column: 1, path: OwnedSchemaPath::ROOT };
    let mut rejected = ArtifactEnvelopeDecodeAuthority::<(), ()>::try_new(
        artifact_envelope_decode_test_cursor(&[br#"{"schema":"s","id":"i","vcs":{},"editMessages":[],"conflicts":[]}"#]),
        &registry,
        Box::new(TestEnvelopeFieldDecoder { terminal: false, accepted: 0 }),
    )
    .unwrap_or_else(|_| panic!("fixed rejected decoder admission"))
    .reject(diagnostic)
    .unwrap_or_else(|_| panic!("public exact rejection transfer"));
    let mut close_turns = 0;
    let mut detached = None;
    while !rejected.terminal_is_empty() {
        drive_test_envelope_field_return(&registry, &mut detached);
        let step = rejected.close_step(1, ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("retained decode rejection closes");
        assert!(matches!(step, SnapshotRetirementStep::Pending { .. } | SnapshotRetirementStep::Complete));
        close_turns += 1;
        assert!(close_turns < 16);
    }
    assert!(close_turns >= 3, "rejected field, page, and terminal record are independently witnessed");
    assert!(registry.terminal_is_empty());
}

#[test]
fn artifact_envelope_public_rejection_preserves_record_lease_ticket_double_return_and_generation_reuse() {
    let registry = ArtifactEnvelopeFieldDecoderRegistry::new();
    let record = artifact_envelope_decode_test_cursor(&[br#"{"schema":"s","id":"i","vcs":{},"editMessages":[],"conflicts":[]}"#]);
    let record_identity = record.tokens.pages.slots.as_ptr();
    let source = ArtifactEnvelopeDecodeAuthority::<(), ()>::try_new(record, &registry, Box::new(TestEnvelopeFieldDecoder { terminal: false, accepted: 0 })).unwrap_or_else(|_| panic!("fixed public rejection decoder admission"));
    let field_ticket = source.field_ticket;
    let registry_identity = Arc::as_ptr(&source.field_registry);
    let diagnostic = OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.public-rejected", offset: 7, line: 2, column: 3, path: OwnedSchemaPath::ROOT };
    let mut rejected = source.reject(diagnostic).unwrap_or_else(|_| panic!("public rejection returns the exact owner authority"));
    assert_eq!(rejected.record.as_ref().map(|record| record.tokens.pages.slots.as_ptr()), Some(record_identity));
    assert_eq!(rejected.fields.as_ref().map(ArtifactEnvelopeFieldDecoderLease::ticket), Some(field_ticket));
    assert_eq!(Arc::as_ptr(&rejected.field_registry), registry_identity);
    assert_eq!(rejected.diagnostic, diagnostic);
    assert!(!registry.ticket_reclaimed(field_ticket));
    assert_eq!(rejected.close_step(0, 0), Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
    assert_eq!(rejected.record.as_ref().map(|record| record.tokens.pages.slots.as_ptr()), Some(record_identity));
    assert_eq!(rejected.fields.as_ref().map(ArtifactEnvelopeFieldDecoderLease::ticket), Some(field_ticket));

    let mut detached = None;
    for _ in 0..16 {
        drive_test_envelope_field_return(&registry, &mut detached);
        let step = rejected.close_step(1, ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("public rejection closes one exact owner");
        assert!(
            matches!(step, SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= 1 && released_bytes <= ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)
                || step == SnapshotRetirementStep::Blocked
                || step == SnapshotRetirementStep::Complete
        );
        if rejected.terminal_is_empty() {
            break;
        }
    }
    drive_test_envelope_field_return(&registry, &mut detached);
    assert!(rejected.terminal_is_empty());
    assert!(registry.ticket_reclaimed(field_ticket));
    assert!(registry.terminal_is_empty());
    assert!(detached.is_none());

    let mut reused = registry.try_admit(Box::new(TestEnvelopeFieldDecoder { terminal: true, accepted: 0 })).unwrap_or_else(|_| panic!("reused field generation"));
    let reused_ticket = reused.ticket();
    assert_eq!(reused_ticket.index(), field_ticket.index());
    assert_ne!(reused_ticket.generation(), field_ticket.generation());
    assert!(reused.return_now());
    assert!(!reused.return_now());
    drop(reused);
    drive_test_envelope_field_return(&registry, &mut detached);
    drive_test_envelope_field_return(&registry, &mut detached);
    assert!(registry.ticket_reclaimed(reused_ticket));
    assert!(registry.terminal_is_empty());
}

#[test]
fn interactive_envelope_close_reclaims_every_owner_through_the_job_protocol() {
    use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep};

    let registry = ArtifactEnvelopeFieldDecoderRegistry::new();
    let mut job = ArtifactEnvelopeDecodeAuthority::<(), ()>::try_new(
        artifact_envelope_decode_test_cursor(&[br#"{"schema":"s","id":"i","vcs":{},"editMessages":[],"conflicts":[]}"#]),
        &registry,
        Box::new(TestEnvelopeFieldDecoder { terminal: false, accepted: 0 }),
    )
    .unwrap_or_else(|_| panic!("fixed close-protocol decoder admission"));
    job.begin_close();
    let mut detached = None;
    for _ in 0..16 {
        drive_test_envelope_field_return(&registry, &mut detached);
        match job.close_step(1, ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {
            InteractiveJobCloseStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES);
            }
            InteractiveJobCloseStep::Blocked => {}
            InteractiveJobCloseStep::Complete => {
                assert!(job.terminal_is_empty());
                assert!(registry.terminal_is_empty());
                return;
            }
        }
    }
    panic!("interactive envelope close did not reclaim every exact owner");
}

#[test]
fn envelope_field_registry_retains_late_return_and_generation_reuse_until_bounded_app_reclaim() {
    let registry = ArtifactEnvelopeFieldDecoderRegistry::<(), ()>::new();
    let first = registry.try_admit(Box::new(TestEnvelopeFieldDecoder { terminal: true, accepted: 0 })).unwrap_or_else(|_| panic!("first exact decoder lease"));
    let first_ticket = first.ticket();
    drop(first);
    assert_eq!(registry.next_returned_ticket(), Some(first_ticket));
    assert!(!registry.ticket_reclaimed(first_ticket));
    let mut detached = None;
    drive_test_envelope_field_return(&registry, &mut detached);
    assert!(registry.ticket_reclaimed(first_ticket));
    assert!(registry.terminal_is_empty(), "the registry is empty only after exact ownership transferred to the app close pump");
    assert!(detached.is_some(), "the detached owner remains retained outside the registry until its terminal step");
    drive_test_envelope_field_return(&registry, &mut detached);
    assert!(detached.is_none());
    assert!(registry.terminal_is_empty());

    let second = registry.try_admit(Box::new(TestEnvelopeFieldDecoder { terminal: true, accepted: 0 })).unwrap_or_else(|_| panic!("reused exact decoder lease"));
    let second_ticket = second.ticket();
    assert_eq!(second_ticket.index(), first_ticket.index());
    assert_ne!(second_ticket.generation(), first_ticket.generation());
    assert!(!registry.ticket_reclaimed(second_ticket), "an old reclaimed generation cannot certify a reused slot");
    drop(second);
    drive_test_envelope_field_return(&registry, &mut detached);
    drive_test_envelope_field_return(&registry, &mut detached);
    assert!(registry.ticket_reclaimed(second_ticket));
    assert!(registry.terminal_is_empty());
}

#[test]
fn envelope_field_registry_capacity_and_contention_return_the_exact_decoder_before_adoption() {
    let registry = ArtifactEnvelopeFieldDecoderRegistry::<(), ()>::new();
    let mut leases = Vec::with_capacity(ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY);
    for _ in 0..ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY {
        leases.push(registry.try_admit(Box::new(TestEnvelopeFieldDecoder { terminal: true, accepted: 0 })).unwrap_or_else(|_| panic!("fixed decoder capacity admits its exact maximum")));
    }
    let (fault, rejected) = match registry.try_admit(Box::new(TestEnvelopeFieldDecoder { terminal: true, accepted: 0 })) {
        Ok(_) => panic!("fixed decoder capacity plus one must fail closed"),
        Err(rejected) => rejected,
    };
    assert_eq!(fault, ArtifactEnvelopeFieldDecoderRegistryFault::Capacity);
    assert!(rejected.terminal_is_empty());
    drop(rejected);

    while let Some(lease) = leases.pop() {
        drop(lease);
        let mut detached = None;
        drive_test_envelope_field_return(&registry, &mut detached);
        drive_test_envelope_field_return(&registry, &mut detached);
        assert!(detached.is_none());
    }
    assert!(registry.terminal_is_empty());

    let state = registry.state.lock().expect("test owns exact registry contention guard");
    let (fault, rejected) = match registry.try_admit(Box::new(TestEnvelopeFieldDecoder { terminal: true, accepted: 0 })) {
        Ok(_) => panic!("contended decoder admission must fail closed"),
        Err(rejected) => rejected,
    };
    assert_eq!(fault, ArtifactEnvelopeFieldDecoderRegistryFault::Contended);
    assert!(rejected.terminal_is_empty());
    drop(rejected);
    drop(state);
    assert!(registry.terminal_is_empty());
}

#[test]
fn snapshot_read_double_return_is_counted_once_and_reclaimed_once() {
    let registry = Arc::new(SnapshotReadLeaseRegistry::new());
    let owner = Arc::new(11u32);
    let mut lease = registry.try_issue(owner.clone()).expect("exact lease");
    assert!(lease.return_now());
    assert!(!lease.return_now(), "the same generation cannot be returned twice");
    assert_eq!(registry.returned.load(std::sync::atomic::Ordering::Acquire), 1);
    let mut reclaimed = None;
    for _ in 0..SNAPSHOT_READ_LEASE_CAPACITY {
        if let Some(owner) = registry.try_take_one_returned::<u32>().expect("one fixed return probe") {
            reclaimed = Some(owner);
            break;
        }
    }
    assert_eq!(reclaimed.as_deref(), Some(&11));
    assert_eq!(registry.returned.load(std::sync::atomic::Ordering::Acquire), 0);
    drop(reclaimed);
    drop(owner);
    drop(lease);
    assert!(registry.terminal_is_empty());
}

#[test]
fn stale_snapshot_read_generation_cannot_aba_remove_a_reused_slot() {
    let registry = Arc::new(SnapshotReadLeaseRegistry::new());
    let owner = Arc::new(1u16);
    let first = registry.try_issue(owner.clone()).expect("first exact lease");
    let stale_index = first.index;
    let stale_generation = first.generation;
    let first_read = ErasedSnapshotRead::new(owner, first);
    drop(match first_read.into_typed::<u16>(&registry) {
        Ok(owner) => owner,
        Err(_) => panic!("first exact lease retires"),
    });

    let mut reads = Vec::with_capacity(SNAPSHOT_READ_LEASE_CAPACITY);
    let mut reused_generation = None;
    for value in 0..SNAPSHOT_READ_LEASE_CAPACITY {
        let owner = Arc::new(value as u16);
        let lease = registry.try_issue(owner.clone()).expect("fixed ring re-admits every slot");
        if lease.index == stale_index {
            reused_generation = Some(lease.generation);
        }
        reads.push(ErasedSnapshotRead::new(owner, lease));
    }
    assert!(reused_generation.is_some_and(|generation| generation != stale_generation));
    assert!(registry.try_take(stale_index, stale_generation).is_err(), "stale generation cannot remove the reused exact slot");
    for read in reads {
        drop(match read.into_typed::<u16>(&registry) {
            Ok(owner) => owner,
            Err(_) => panic!("every reused exact lease retires"),
        });
    }
    assert!(registry.terminal_is_empty());
}

struct ArtifactStore<P, Mutation>(super::ArtifactStore<P, Mutation>)
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + super::Mutation<P>;

impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue + ArtifactPack + Send + 'static,
    Mutation: Clone + ToValue + FromValue + super::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    async fn new(envelope: ArtifactEnvelope<P, Mutation>) -> Self {
        Self(super::ArtifactStore::new(envelope).await.expect("test fixture history is valid"))
    }

    async fn current_checkpoint_id(&self) -> Option<&str> {
        self.0.current_checkpoint_id()
    }
}

impl<P, Mutation> std::ops::Deref for ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + super::Mutation<P>,
{
    type Target = super::ArtifactStore<P, Mutation>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<P, Mutation> std::ops::DerefMut for ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + super::Mutation<P>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<P, Mutation> SpaceMember for ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue + ArtifactPack + crate::os_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + super::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    async fn document_id(&self) -> &str {
        SpaceMember::document_id(&self.0).await
    }
    fn artifact_ref(&self) -> Option<crate::os_io::ArtifactRef> {
        SpaceMember::artifact_ref(&self.0)
    }
    fn owner_ref(&self) -> Option<OwnerRef> {
        SpaceMember::owner_ref(&self.0)
    }
    fn child_restore_projection(&self) -> Result<ChildRestoreProjection<'_>, ChildRestoreProjectionError> {
        SpaceMember::child_restore_projection(&self.0)
    }
    fn one_item_publication_identity(&self) -> (u64, [u8; 32]) {
        SpaceMember::one_item_publication_identity(&self.0)
    }
    fn one_item_wire_publication_supported(&self) -> bool {
        SpaceMember::one_item_wire_publication_supported(&self.0)
    }
    fn begin_one_item_wire_publication(&self, request: MemberStoreOneItemWireRequest) -> Result<Box<dyn ErasedMemberStoreOneItemPublication>, ArtifactStoreBatchAdmissionRejected<MemberStoreOneItemWire>> {
        SpaceMember::begin_one_item_wire_publication(&self.0, request)
    }
    fn advance_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemAdvance, String> {
        SpaceMember::advance_one_item_publication(&mut self.0, publication, grant)
    }
    fn prepare_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
        SpaceMember::prepare_one_item_publication(&mut self.0, publication, grant)
    }
    fn abort_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, String> {
        SpaceMember::abort_one_item_publication(&mut self.0, publication, grant)
    }
    fn snapshot_read_erased_now(&self) -> Result<ErasedSnapshotRead, String> {
        SpaceMember::snapshot_read_erased_now(&self.0)
    }
    async fn snapshot_read_erased(&self) -> Result<ErasedSnapshotRead, String> {
        SpaceMember::snapshot_read_erased(&self.0).await
    }
    fn retire_snapshot_read_erased(&mut self, snapshot: ErasedSnapshotRead) -> Result<Box<dyn ErasedSnapshotRetirement>, SnapshotRetirementRejected> {
        SpaceMember::retire_snapshot_read_erased(&mut self.0, snapshot)
    }
    fn take_returned_snapshot_read_retirement(&mut self) -> Result<Option<Box<dyn ErasedSnapshotRetirement>>, String> {
        SpaceMember::take_returned_snapshot_read_retirement(&mut self.0)
    }
    fn snapshot_read_leases_terminal_is_empty(&self) -> bool {
        SpaceMember::snapshot_read_leases_terminal_is_empty(&self.0)
    }
    fn close_owned_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        SpaceMember::close_owned_step(&mut self.0, maximum_items, maximum_bytes)
    }
    fn close_owned_terminal_is_empty(&self) -> bool {
        SpaceMember::close_owned_terminal_is_empty(&self.0)
    }
    fn content_revision_now(&self) -> [u8; 32] {
        SpaceMember::content_revision_now(&self.0)
    }
    async fn content_revision(&self) -> [u8; 32] {
        SpaceMember::content_revision(&self.0).await
    }
    async fn is_dirty(&self) -> bool {
        SpaceMember::is_dirty(&self.0).await
    }
    // 🎯️ Overrides the trait default (`MergePolicy::default()`, always `Normal`) the same way
    // the REAL `super::ArtifactStore`'s own `impl SpaceMember` block does (see that block's
    // `merge_policy` for the full rationale) — without this override, `.merge_policy()` called
    // on THIS wrapper (whether directly, or generically inside `CompositionCoordinator::
    // dispatch_group`'s `M: SpaceMember` body) silently reads the trait's `Normal`-only default
    // instead of whatever `set_merge_policy` set on the real inner store.
    async fn merge_policy(&self) -> crate::os_spr::MergePolicy {
        SpaceMember::merge_policy(&self.0).await
    }
    async fn commit_checkpoint(&mut self, message: String, authors: Vec<Author>) -> Result<String, VcsError> {
        SpaceMember::commit_checkpoint(&mut self.0, message, authors).await
    }
    async fn current_checkpoint_id(&self) -> Option<String> {
        SpaceMember::current_checkpoint_id(&self.0).await
    }
    async fn current_alternative_id(&self) -> Option<String> {
        SpaceMember::current_alternative_id(&self.0).await
    }
    async fn checkout(&mut self, checkpoint_id: &str, alternative_id: &str) -> Result<(), VcsError> {
        SpaceMember::checkout(&mut self.0, checkpoint_id, alternative_id).await
    }
    async fn create_alternative(&mut self, name: String) -> Result<String, VcsError> {
        SpaceMember::create_alternative(&mut self.0, name).await
    }
    async fn last_local_edit_timestamp(&self) -> Option<HybridLogicalTimestamp> {
        SpaceMember::last_local_edit_timestamp(&self.0).await
    }
    async fn last_undone_local_edit_timestamp(&self) -> Option<HybridLogicalTimestamp> {
        SpaceMember::last_undone_local_edit_timestamp(&self.0).await
    }
    async fn undo(&mut self) -> Result<(), VcsError> {
        SpaceMember::undo(&mut self.0).await
    }
    async fn redo(&mut self) -> Result<(), VcsError> {
        SpaceMember::redo(&mut self.0).await
    }
    async fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    async fn preview_wire(&self, ops: &[Vec<u8>]) -> Vec<crate::os_spr::MutationMessage> {
        SpaceMember::preview_wire(&self.0, ops).await
    }
    async fn dispatch_wire(&mut self, command: &[u8]) -> Result<CommandReceipt, VcsError> {
        SpaceMember::dispatch_wire(&mut self.0, command).await
    }
    async fn dispatch_wire_with_policy(&mut self, command: &[u8], policy: crate::os_spr::MergePolicy) -> Result<CommandReceipt, VcsError> {
        SpaceMember::dispatch_wire_with_policy(&mut self.0, command, policy).await
    }
    async fn tail_group_id(&self) -> Option<String> {
        SpaceMember::tail_group_id(&self.0).await
    }
    async fn tail_edit_id(&self) -> Option<String> {
        SpaceMember::tail_edit_id(&self.0).await
    }
    async fn redo_tail(&self) -> Option<(String, Option<String>)> {
        SpaceMember::redo_tail(&self.0).await
    }
    async fn stamp_tail_group_id(&mut self, group_id: &str) -> Result<(), VcsError> {
        SpaceMember::stamp_tail_group_id(&mut self.0, group_id).await
    }
    async fn stamp_tail_origin(&mut self, origin: crate::os_spr::MutationOrigin) -> Result<(), VcsError> {
        SpaceMember::stamp_tail_origin(&mut self.0, origin).await
    }
    async fn set_owner(&mut self, owner: Option<OwnerRef>) {
        SpaceMember::set_owner(&mut self.0, owner).await;
    }
    async fn document_pack_bytes(&self) -> Result<Vec<u8>, VcsError> {
        SpaceMember::document_pack_bytes(&self.0).await
    }
    async fn envelope_pack_bytes(&self) -> Result<Vec<u8>, VcsError> {
        SpaceMember::envelope_pack_bytes(&self.0).await
    }
    async fn pack_at_checkpoint(&self, checkpoint_id: &str) -> Result<Vec<u8>, VcsError> {
        SpaceMember::pack_at_checkpoint(&self.0, checkpoint_id).await
    }
}

/// 🧪️ Exact factories for the three typed coordinator fixtures; empty genesis remains invalid.
macro_rules! fixture_member_factory {
    ($mutation:ty) => {
        impl MemberFactory for ArtifactStore<DemoSnapshot, $mutation> {
            const OPEN_DECLARATIONS: &'static [MemberOpenDeclaration] = &[MemberOpenDeclaration { kind: "s.stdio.mesh", standard: "1", subset: "*", schema: "demo/v1" }];
            type Open = UnsupportedMemberFactoryOpen<Self>;
            fn begin_open(request: MemberOpenRequest) -> Result<Self::Open, MemberOpenAdmissionError> {
                UnsupportedMemberFactoryOpen::begin(request)
            }

            async fn create(id: &str, dialect: &crate::os_io::ArtifactDialect, initial_pack: &[u8]) -> Result<Self, VcsError> {
                if dialect != &demo_child_dialect() {
                    return Err(VcsError::ValidationFailed("unregistered fixture member dialect".into()));
                }
                Ok(Self(create_member_store("demo/v1", id, dialect, initial_pack).await?))
            }
            async fn open(expected: &crate::os_io::ArtifactRef, owner: Option<&OwnerRef>, envelope_pack: &[u8]) -> Result<Self, VcsError> {
                if expected.dialect != demo_child_dialect() {
                    return Err(VcsError::ValidationFailed("unregistered fixture member dialect".into()));
                }
                Ok(Self(open_member_store("demo/v1", expected, owner, envelope_pack).await?))
            }
        }
    };
}
fixture_member_factory!(DemoMutation);
fixture_member_factory!(ValidatedMutation);
fixture_member_factory!(SeverityMutation);

#[derive(Clone, Debug, PartialEq, Serialize, ToValue, Deserialize, FromValue, crate::os_dsl::DslArtifact)]
#[dsl(id = "demo.doc", extension = "demo")]
pub(crate) struct DemoSnapshot {
    pub(super) n: Option<i32>,
}

impl crate::os_schema_composition::ArtifactCompositionFields for DemoSnapshot {
    fn visit_child_refs<'a, V: crate::os_schema_composition::ChildRefVisitor<'a>>(&'a self, _visitor: &mut V) -> Result<(), V::Error> {
        Ok(())
    }
}

impl Default for DemoSnapshot {
    fn default() -> Self {
        Self { n: Some(0) }
    }
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6).
impl ArtifactDsl for DemoSnapshot {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let body = match semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = crate::os_dsl::parse(body, &Self::__dsl_spec(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let record = self.__dsl_to_record();
        let body = crate::os_dsl::print(&record, &Self::__dsl_spec(), crate::os_dsl::JoinMode::Document);
        let envelope = semio_format::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), semio_format::Component::Dsl, 1).expect("valid envelope_id");
        semio_format::wrap_text(&envelope, &body)
    }
}
std::thread_local! {
    static MEMBER_SNAPSHOT_DECODE_COUNT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// 📦️ Handcrafted ArtifactPack (P6).
impl ArtifactPack for DemoSnapshot {
    fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let inner = pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = semio_format::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), semio_format::Component::Pack, 1).map_err(|e| PackError::Schema(e.to_string()))?;
        Ok(semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError> {
        MEMBER_SNAPSHOT_DECODE_COUNT.with(|count| count.set(count.get() + 1));
        let (envelope, inner) = semio_format::unwrap_binary(bytes).map_err(|e| PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as ArtifactDsl>::envelope_id() {
            return Err(PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _report) = pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(text_error_to_pack_error)
    }
    fn record_spec() -> Option<crate::os_dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️ArtifactCodec

struct DemoSnapshotRetirement(Option<Arc<DemoSnapshot>>);

impl ErasedSnapshotRetirement for DemoSnapshotRetirement {
    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.0.take().is_some() {
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.0.is_none()
    }
}

struct DemoSnapshotRetirementFactory;

impl SnapshotRetirementFactory<DemoSnapshot> for DemoSnapshotRetirementFactory {
    fn retire(&self, snapshot: Arc<DemoSnapshot>) -> Box<dyn ErasedSnapshotRetirement> {
        Box::new(DemoSnapshotRetirement(Some(snapshot)))
    }
}

struct ExactDemoSnapshotRetirement {
    owner: Option<Arc<DemoSnapshot>>,
    value: Option<DemoSnapshot>,
    completed: Arc<std::sync::atomic::AtomicUsize>,
}

impl ErasedSnapshotRetirement for ExactDemoSnapshotRetirement {
    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(owner) = self.owner.take() {
            return match Arc::try_unwrap(owner) {
                Ok(value) => {
                    self.value = Some(value);
                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                Err(owner) => {
                    self.owner = Some(owner);
                    Ok(SnapshotRetirementStep::Blocked)
                }
            };
        }
        if self.value.take().is_some() {
            self.completed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.owner.is_none() && self.value.is_none()
    }
}

impl Drop for ExactDemoSnapshotRetirement {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "exact demo snapshot retirement reached Drop with a retained root");
    }
}

struct ExactDemoSnapshotRetirementFactory(Arc<std::sync::atomic::AtomicUsize>);

impl SnapshotRetirementFactory<DemoSnapshot> for ExactDemoSnapshotRetirementFactory {
    fn retire(&self, snapshot: Arc<DemoSnapshot>) -> Box<dyn ErasedSnapshotRetirement> {
        Box::new(ExactDemoSnapshotRetirement { owner: Some(snapshot), value: None, completed: Arc::clone(&self.0) })
    }
}

struct DemoInitialSnapshotRetirement(Option<DemoSnapshot>);

impl ErasedSnapshotRetirement for DemoInitialSnapshotRetirement {
    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.0.take().is_some() {
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.0.is_none()
    }
}

struct DemoInitialSnapshotRetirementFactory;

impl ArtifactOwnedValueRetirementFactory<DemoSnapshot> for DemoInitialSnapshotRetirementFactory {
    fn retire_owned(&self, value: DemoSnapshot) -> Box<dyn ErasedSnapshotRetirement> {
        Box::new(DemoInitialSnapshotRetirement(Some(value)))
    }
}

struct DemoMutationRetirement<Mutation>(Option<Mutation>);

impl<Mutation: Send> ErasedSnapshotRetirement for DemoMutationRetirement<Mutation> {
    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.0.take().is_some() {
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.0.is_none()
    }
}

struct DemoMutationRetirementFactory;

impl<Mutation: Send + 'static> ArtifactOwnedValueRetirementFactory<Mutation> for DemoMutationRetirementFactory {
    fn retire_owned(&self, value: Mutation) -> Box<dyn ErasedSnapshotRetirement> {
        Box::new(DemoMutationRetirement(Some(value)))
    }
}

fn drive_retirement_terminal(mut retirement: Box<dyn ErasedSnapshotRetirement>) {
    let mut turns = 0;
    loop {
        match retirement.close_step(1, 7).expect("retained owner closes within its exact grant") {
            SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= 7),
            SnapshotRetirementStep::Blocked => panic!("fixture owner has no external wait"),
            SnapshotRetirementStep::Complete => {
                assert!(retirement.terminal_is_empty());
                break;
            }
        }
        turns += 1;
        assert!(turns < 10_000, "retained owner must reach terminal empty across interruptions");
    }
}

#[semio_framework_async_macros::async_test]
async fn artifact_store_snapshot_roots_and_final_envelope_transfer_in_exact_close_order() {
    let envelope = create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "root-close", DemoSnapshot { n: Some(7) }, None);
    let mut store = super::ArtifactStore::new(envelope).await.expect("valid empty-history store");
    store.install_member_store_owners_exact(DemoSnapshot::member_store_owners());
    *store.tail_undo_cache = Some(("tail-owner".repeat(64), Arc::clone(&*store.current)));

    let tail_id = store.close_take_runtime_string_retirement(ArtifactStoreCloseStringLane::TailUndoEditId).expect("tail id retires before its snapshot");
    drive_retirement_terminal(tail_id);
    assert!(matches!(store.close_take_tail_snapshot_retirement().expect("shared tail root classification"), ArtifactStoreSnapshotRootClose::ReleasedShared));

    let current = store.close_take_current_snapshot_retirement().expect("current root transfer").expect("current root exists exactly once");
    drive_retirement_terminal(current);
    assert!(store.close_take_current_snapshot_retirement().expect("detached current probe").is_none());

    while let Some(retirement) = store.close_take_envelope_metadata_string_retirement() {
        drive_retirement_terminal(retirement);
    }
    assert!(store.close_structural_owners_terminal_is_empty(), "empty history has no unresolved structural owner");
    let initial = store.close_take_final_envelope_retirement().expect("final envelope transfer").expect("initial snapshot owner exists exactly once");
    drive_retirement_terminal(initial);
    assert!(store.close_take_final_envelope_retirement().expect("detached envelope probe").is_none());
    assert!(store.owned_roots_terminal_is_empty());
}

struct DemoStoreOwnedDisposer<Mutation>(PhantomData<fn() -> Mutation>);

impl<Mutation> ArtifactStoreOwnedDisposer<DemoSnapshot, Mutation> for DemoStoreOwnedDisposer<Mutation>
where
    Mutation: Clone + ToValue + FromValue + super::Mutation<DemoSnapshot>,
{
    fn close_step(&mut self, _store: &mut ArtifactStoreCloseView<'_, DemoSnapshot, Mutation>, _maximum_items: usize, _maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        Ok(SnapshotRetirementStep::Blocked)
    }

    fn terminal_is_empty(&self, _store: &super::ArtifactStore<DemoSnapshot, Mutation>) -> bool {
        false
    }

    fn close_uninstalled_step(&mut self, maximum_items: usize) -> Result<SnapshotRetirementStep, String> {
        Ok(if maximum_items == 0 { SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 } } else { SnapshotRetirementStep::Complete })
    }

    fn uninstalled_terminal_is_empty(&self) -> bool {
        true
    }
}

impl MemberStoreOwner<DemoMutation> for DemoSnapshot {
    type SnapshotOpen = UnsupportedMemberSnapshotOpen<Self>;

    fn member_store_owners() -> MemberStoreOwners<Self, DemoMutation> {
        demo_closable_store_owners()
    }
}

// `impl crate::os_store::ArtifactPack for DemoSnapshot` is now generated automatically by
// `#[derive(crate::os_dsl::DslArtifact)]` above (see dsl/derive/rs/lib.rs's `🔖️DslArtifact` region) —
// same seam as its `impl crate::os_store::ArtifactDsl for DemoSnapshot` sibling.

#[derive(Clone, Debug, Default, PartialEq, Serialize, ToValue, Deserialize, FromValue)]
pub(crate) struct DemoDiff {
    pub(super) n: Option<DemoFieldChange>,
}

#[derive(Clone, Debug, PartialEq, Serialize, ToValue, Deserialize, FromValue)]
pub(crate) struct DemoFieldChange {
    value: Option<i32>,
}

impl DemoDiff {
    pub(super) fn value(value: Option<i32>) -> Self {
        Self { n: Some(DemoFieldChange { value }) }
    }
}

impl MutationDiff<DemoSnapshot> for DemoDiff {
    fn apply(&self, snapshot: &DemoSnapshot) -> crate::os_spr::MutationApplyResult<DemoSnapshot> {
        Ok(DemoSnapshot { n: self.n.as_ref().map_or(snapshot.n, |change| change.value) })
    }

    fn absorb(&mut self, other: Self) {
        if other.n.is_some() {
            self.n = other.n;
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
pub(crate) struct LossyDiff {}

impl MutationDiff<DemoSnapshot> for LossyDiff {
    fn apply(&self, snapshot: &DemoSnapshot) -> crate::os_spr::MutationApplyResult<DemoSnapshot> {
        Ok(snapshot.clone())
    }
    fn absorb(&mut self, _other: Self) {}
}

#[test]
fn rejected_history_mutation_and_metadata_owners_close_under_one_item_grants() {
    let edit = Edit {
        id: "rejected-edit".repeat(32),
        actor: Some("actor".repeat(64)),
        forwards: vec![DemoMutation::SetN(SetN { n: 7 })],
        inverse: vec![DemoMutation::SetN(SetN { n: 3 })],
        mutation_meta: Vec::new(),
        description: Some("description".repeat(64)),
        coalesce_key: Some("coalesce".repeat(64)),
        sequence_number: 1,
        started_at: "started".repeat(64),
        finished_at: Some("finished".repeat(64)),
    };
    drive_retirement_terminal(Box::new(ArtifactStoreDecodedEditRetirement::new(edit, Arc::new(DemoMutationRetirementFactory))));
    drive_retirement_terminal(Box::new(ArtifactStoreHistoryMetadataRetirement::change(Change {
        id: "change".repeat(64),
        edit_ids: vec!["edit-a".repeat(64), "edit-b".repeat(64)],
        description: Some("metadata".repeat(64)),
        saved_at: "timestamp".repeat(64),
    })));
}

//#region 🔖️OpCodec
/// 🎞️ Handcrafted OpText (P6).
impl OpText for DemoMutation {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = crate::os_dsl::parse(line, &spec_fn(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Inline })?;
                return <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(crate::os_dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        crate::os_dsl::print(&record, &spec_fn(), crate::os_dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl OpBinary for DemoMutation {
    fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(crate::os_spr::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1)();
        let body = crate::os_pack::encode_record_body(&spec, &record, &PackEncodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        crate::os_pack::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = crate::os_pack::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(crate::os_spr::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(crate::os_spr::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = crate::os_pack::decode_record_body(body, &spec, &PackDecodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
        let record_offset = reader.position() as u64;
        <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| crate::os_spr::ProtocolError::Malformed { what: "op record", offset: record_offset, detail: error.to_string() })
    }
}
//#endregion 🔖️OpCodec

//#region 📬️OneItemPublicationLaws
//#region 🧩️RetainedMemberPublicationLaws
crate::space_members! {
    pub enum RetainedTestMembers, RetainedTestMembersOpen {
        First("s.test.member", "v1", "first", "demo/v1") => (DemoSnapshot, DemoMutation),
        Second("s.test.member", "v1", "second", "demo/v1") => (DemoSnapshot, DemoMutation),
    }
}

fn member_publication_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/📢️member-publication.json")).expect("language-neutral member publication fixture")
}

fn member_dialect_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧩️composition/🪪️member-dialect/🧫️fixtures/🔣️.json")).expect("neutral member dialect corpus")
}

fn close_member_dialect_fixture<M: SpaceMember>(member: &mut M) {
    for _ in 0..65_536 {
        match member.close_owned_step(1, 4096).expect("exact fixture member bounded close") {
            SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1 && released_bytes <= 4096);
            }
            SnapshotRetirementStep::Blocked => panic!("fixture member has no external owner"),
            SnapshotRetirementStep::Complete => {
                assert!(member.close_owned_terminal_is_empty());
                return;
            }
        }
    }
    panic!("fixture member retirement did not finish");
}

fn close_member_dialect_envelope(envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation>) {
    let mut retirement = ArtifactStoreEnvelopeRetirement::new(envelope, Arc::new(DemoInitialSnapshotRetirementFactory), Arc::new(DemoMutationRetirementFactory));
    for _ in 0..65_536 {
        match retirement.close_step(1, 4096).expect("fixture envelope retirement") {
            SnapshotRetirementStep::Complete => {
                assert!(retirement.terminal_is_empty());
                return;
            }
            SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= 4096),
            SnapshotRetirementStep::Blocked => panic!("fixture envelope has no external owner"),
        }
    }
    panic!("fixture envelope retirement did not finish");
}

#[semio_framework_async_macros::async_test]
async fn member_factory_closed_dialect_matches_neutral_admission_corpus() {
    let fixture = member_dialect_fixture();
    let seed = DemoSnapshot { n: Some(7) };
    for row in fixture["cases"].as_array().unwrap() {
        let dialect: crate::os_io::ArtifactDialect = serde_json::from_value(row["requested"].clone()).unwrap();
        let expected = crate::os_io::ArtifactRef { artifact_id: "child-1".into(), dialect };
        MEMBER_SNAPSHOT_DECODE_COUNT.set(0);
        let result = if row["operation"] == "create" {
            RetainedTestMembers::create(&expected.artifact_id, &expected.dialect, &seed.encode_pack()).await
        } else {
            let mut envelope = create_document_envelope::<DemoSnapshot, DemoMutation>(row["persisted"]["schema"].as_str().unwrap(), "child-1", seed.clone(), None);
            envelope.dialect = serde_json::from_value(row["persisted"]["dialect"].clone()).unwrap();
            let files = print_document_pack(&envelope).await.unwrap();
            let packed = encode_document_pack_bytes(&files.pack, &files.spr).await;
            close_member_dialect_envelope(envelope);
            RetainedTestMembers::open(&expected, None, &packed).await
        };
        assert_eq!(MEMBER_SNAPSHOT_DECODE_COUNT.get(), usize::from(row["accepted"].as_bool().unwrap()), "{}: rejected identity must not hydrate a typed snapshot", row["id"]);
        assert_eq!(result.is_ok(), row["accepted"].as_bool().unwrap(), "{}: {:?}", row["id"], result.as_ref().err());
        if let Ok(mut member) = result {
            let variant = match &member {
                RetainedTestMembers::First(_) => "First",
                RetainedTestMembers::Second(_) => "Second",
            };
            assert_eq!(variant, row["variant"].as_str().unwrap());
            assert_eq!(member.document_id().await, "child-1");
            let restored = DemoSnapshot::decode_pack(&member.document_pack_bytes().await.unwrap()).unwrap();
            assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::to_value(&seed).unwrap());
            assert_eq!(member.artifact_ref().as_ref(), Some(&expected));
            assert_eq!(member.owner_ref(), None);
            assert!(member.child_restore_projection().expect("member exposes its exact typed snapshot projection").is_empty());
            close_member_dialect_fixture(&mut member);
        }
    }
    let expected = crate::os_io::ArtifactRef { artifact_id: "child-1".into(), dialect: serde_json::from_value(fixture["bindings"][0]["dialect"].clone()).unwrap() };
    for bytes in [&[][..], &[1][..], &[0xff, 0xff][..]] {
        MEMBER_SNAPSHOT_DECODE_COUNT.set(0);
        assert!(RetainedTestMembers::open(&expected, None, bytes).await.is_err());
        assert_eq!(MEMBER_SNAPSHOT_DECODE_COUNT.get(), 0);
    }
    eprintln!("[DEBUG] member factory closed dialect: 13 neutral vectors, 3 malformed frames, real typed create/open and serde values");
}

#[semio_framework_async_macros::async_test]
async fn member_factory_closed_dialect_rejects_identity_and_owner_substitution() {
    let fixture = member_dialect_fixture();
    let expected = crate::os_io::ArtifactRef { artifact_id: "child-1".into(), dialect: serde_json::from_value(fixture["bindings"][0]["dialect"].clone()).unwrap() };
    let owner = OwnerRef { parent: crate::os_io::ArtifactRef { artifact_id: "parent-1".into(), dialect: crate::os_io::ArtifactDialect::parse_coordinate("s.test.parent@v1/*").unwrap() }, slot: "content".into(), child_id: "child-1".into() };
    for row in fixture["identityCases"].as_array().unwrap() {
        let mut envelope = create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", row["persistedId"].as_str().unwrap(), DemoSnapshot { n: Some(7) }, None);
        envelope.dialect = Some(expected.dialect.clone());
        if !row["persistedOwner"].is_null() {
            let persisted = &row["persistedOwner"];
            envelope.owner = Some(OwnerRef {
                parent: crate::os_io::ArtifactRef { artifact_id: persisted["parentId"].as_str().unwrap().into(), dialect: crate::os_io::ArtifactDialect::parse_coordinate(persisted["parentDialect"].as_str().unwrap()).unwrap() },
                slot: persisted["slot"].as_str().unwrap().into(),
                child_id: persisted["childId"].as_str().unwrap().into(),
            });
        }
        let files = print_document_pack(&envelope).await.unwrap();
        let packed = encode_document_pack_bytes(&files.pack, &files.spr).await;
        close_member_dialect_envelope(envelope);
        let expected_owner = row["expectedOwned"].as_bool().unwrap().then_some(&owner);
        MEMBER_SNAPSHOT_DECODE_COUNT.set(0);
        let result = RetainedTestMembers::open(&expected, expected_owner, &packed).await;
        assert_eq!(MEMBER_SNAPSHOT_DECODE_COUNT.get(), usize::from(row["accepted"].as_bool().unwrap()), "{}: rejected owner must not hydrate a typed snapshot", row["id"]);
        assert_eq!(result.is_ok(), row["accepted"].as_bool().unwrap(), "{}: {:?}", row["id"], result.as_ref().err());
        if let Ok(mut member) = result {
            assert_eq!(member.artifact_ref().as_ref(), Some(&expected));
            assert_eq!(member.owner_ref().as_ref(), expected_owner);
            close_member_dialect_fixture(&mut member);
        }
    }
    eprintln!("[DEBUG] member factory exact identity: 9 neutral owner/id vectors over separate persisted envelopes");
}

fn close_erased_member_publication(publication: &mut dyn ErasedMemberStoreOneItemPublication, grant: ArtifactStoreOneItemGrant) {
    publication.begin_close();
    for _ in 0..65_536 {
        match publication.close_step(grant).expect("one retained member close unit") {
            SnapshotRetirementStep::Complete => {
                assert!(publication.terminal_is_empty());
                return;
            }
            SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= grant.maximum_items);
                assert!(released_bytes <= grant.maximum_bytes);
            }
            SnapshotRetirementStep::Blocked => panic!("admitted member close must progress under the production grant"),
        }
    }
    panic!("member publication did not retire its exact owners");
}

fn retained_demo_member_owners() -> MemberStoreOwners<DemoSnapshot, DemoMutation> {
    demo_closable_store_owners().with_one_item_preparation(Arc::new(DemoOneItemPreparationFactory::admissible())).with_one_item_wire_preparation(Arc::new(DemoMemberWirePreparationFactory))
}

#[test]
fn member_open_partial_parse_and_initialization_owners_retire_exactly() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    struct Observed {
        inner: Box<dyn ErasedSnapshotRetirement>,
        count: Arc<AtomicUsize>,
        counted: bool,
    }
    impl ErasedSnapshotRetirement for Observed {
        fn close_step(&mut self, items: usize, bytes: usize) -> Result<SnapshotRetirementStep, String> {
            let step = self.inner.close_step(items, bytes)?;
            if matches!(step, SnapshotRetirementStep::Complete) && !self.counted {
                assert!(self.inner.terminal_is_empty());
                self.count.fetch_add(1, Ordering::SeqCst);
                self.counted = true;
            }
            Ok(step)
        }
        fn terminal_is_empty(&self) -> bool {
            self.counted && self.inner.terminal_is_empty()
        }
    }
    struct SnapshotFactory(Arc<AtomicUsize>);
    impl ArtifactOwnedValueRetirementFactory<DemoSnapshot> for SnapshotFactory {
        fn retire_owned(&self, value: DemoSnapshot) -> Box<dyn ErasedSnapshotRetirement> {
            Box::new(Observed { inner: DemoInitialSnapshotRetirementFactory.retire_owned(value), count: self.0.clone(), counted: false })
        }
    }
    struct MutationFactory(Arc<AtomicUsize>);
    impl ArtifactOwnedValueRetirementFactory<DemoMutation> for MutationFactory {
        fn retire_owned(&self, value: DemoMutation) -> Box<dyn ErasedSnapshotRetirement> {
            Box::new(Observed { inner: DemoMutationRetirementFactory.retire_owned(value), count: self.0.clone(), counted: false })
        }
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧩️composition/🚪️open/🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["retention"].as_array().unwrap() {
        let snapshots = Arc::new(AtomicUsize::new(0));
        let mutations = Arc::new(AtomicUsize::new(0));
        let owners = MemberStoreOwners::new(Arc::new(DemoSnapshotRetirementFactory), Arc::new(SnapshotFactory(snapshots.clone())), Arc::new(MutationFactory(mutations.clone())), Box::new(ArtifactStoreCursorDisposer::new()));
        let mut pages = OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits { maximum_pages: 1, maximum_bytes: 3 }).unwrap();
        pages.admit_page(OwnedSchemaDecodePage::try_from_slice(&[1, 97, 83]).unwrap()).unwrap();
        pages.seal().unwrap();
        let expected = crate::os_io::ArtifactRef { artifact_id: "member-retained".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.test.member".into(), standard: "1".into(), subset: "*".into() } };
        let request = MemberOpenRequest::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), 1000, expected, None, pages).admit(1).unwrap_or_else(|_| panic!("admitted request fixture"));
        let mut retained = member_open::MemberStoreOpenRetained::new(request, owners);
        let stage = row["stage"].as_str().unwrap();
        if stage != "input" {
            let history = crate::os_spr::HistoryLog {
                doc_id: "member-retained".into(),
                schema: "demo/v1".into(),
                edits: vec![crate::os_spr::HistoryEdit {
                    id: "raw-edit".into(),
                    actor: Some("raw-actor".into()),
                    started_at: "started".into(),
                    finished_at: Some("finished".into()),
                    coalesce_key: Some("key".into()),
                    description: Some("Grüße-😀".repeat(512)),
                    ops: vec![crate::os_spr::OpPayload { text: None, binary: Some(vec![0xff; 129]) }],
                    inverse: Vec::new(),
                    meta: Some(vec![crate::os_spr::HistoryOpMeta {
                        dependencies: vec!["dependency".into()],
                        payload_hash: Some([3; 32]),
                        origin: crate::os_spr::MutationOrigin::Transaction { initiator: crate::os_spr::ForeignTarget { artifact_id: "initiator".into(), artifact_kind: "s.test.member".into(), dialect: Some("1/*".into()) } },
                        ..Default::default()
                    }]),
                }],
                composition: Some(crate::os_spr::HistoryComposition {
                    owner: Some(("parent".into(), "slot".into(), "member-retained".into())),
                    dialect: Some(("s.test.member".into(), "1".into(), "*".into())),
                    checkpoint_pins: vec![("unknown-checkpoint".into(), vec![("invalid-uri".into(), "pin".into())])],
                }),
                ..Default::default()
            };
            retained.stage_history(history).unwrap_or_else(|_| panic!("raw decoded model stays owned before typed hydration"));
            let packed = DemoSnapshot { n: Some(7) }.encode_pack();
            let initial = DemoSnapshot::decode_pack(&packed).unwrap();
            if matches!(stage, "envelope" | "initialization") {
                let envelope = create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "member-retained", initial, None).into_owners();
                retained.stage_envelope(envelope).unwrap_or_else(|_| panic!("unique parsed owner slot"));
                if stage == "initialization" {
                    let current = DemoSnapshot::decode_pack(&packed).unwrap();
                    let runtime = ArtifactStoreInitializationRuntime::new("member-retained", "demo/v1", current, [7; 32]);
                    retained.stage_runtime(runtime).unwrap_or_else(|_| panic!("unique initialized owner slot"));
                }
            } else {
                retained.stage_initial(initial).unwrap_or_else(|_| panic!("unique typed snapshot slot"));
            }
        }
        if matches!(stage, "forward" | "inverse") {
            let encoded = DemoMutation::SetN(SetN { n: 7 }).encode_op().unwrap();
            let forwards = vec![DemoMutation::decode_op(&encoded).unwrap()];
            let inverse = if stage == "inverse" { vec![DemoMutation::decode_op(&encoded).unwrap()] } else { Vec::new() };
            retained
                .stage_edit(Edit {
                    id: "pending-edit".into(),
                    actor: Some("actor".into()),
                    forwards,
                    inverse,
                    mutation_meta: Vec::new(),
                    description: Some("retained during malformed next record".into()),
                    coalesce_key: None,
                    sequence_number: 1,
                    started_at: "now".into(),
                    finished_at: None,
                })
                .unwrap_or_else(|_| panic!("unique partial edit slot"));
            assert!(DemoMutation::decode_op(&[0xff]).is_err(), "next malformed operation must not consume the retained edit");
        }
        let before = retained.retained_typed_owners();
        if row["cancelled"].as_bool().unwrap() {
            let cancel = semio_framework_job::root_cancel_token();
            cancel.cancel_now();
            let mut sequence = 0;
            let cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(1, 999), cancel, || Some(1), &mut sequence);
            assert_eq!(retained.check_step_authority(&cx), Err(MemberOpenDiagnostic::Cancelled));
        } else {
            retained.reject(if stage == "initialization" { MemberOpenDiagnostic::Initialization } else { MemberOpenDiagnostic::Malformed });
        }
        assert_eq!(retained.retained_input_bytes(), 3);
        assert_eq!(retained.retained_typed_owners(), before);
        assert!(matches!(retained.close_step(0, 0).unwrap(), SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
        assert_eq!(retained.retained_typed_owners(), before);
        assert_eq!((snapshots.load(Ordering::SeqCst), mutations.load(Ordering::SeqCst)), (0, 0));
        drive_retirement_terminal(Box::new(retained));
        assert_eq!(snapshots.load(Ordering::SeqCst), row["snapshots"].as_u64().unwrap() as usize, "{}", row["id"]);
        assert_eq!(mutations.load(Ordering::SeqCst), row["mutations"].as_u64().unwrap() as usize, "{}", row["id"]);
    }
    eprintln!("[DEBUG] member-open retained ownership: six interruption stages, exact snapshot/mutation factory terminal counts, zero-budget preservation; parser activation remains separate");
}

struct DemoMemberWirePreparationFactory;

struct DemoMemberWirePreparation {
    request: Option<ArtifactStoreOneItemPreparationRequest<DemoSnapshot, MemberStoreOneItemWire>>,
    wire: Option<MemberStoreOneItemWire>,
    inner: Option<Box<dyn ArtifactStoreOneItemPreparation<DemoSnapshot, DemoMutation>>>,
    offset: usize,
    magnitude: u32,
    negative: bool,
    state: u8,
    closing: bool,
}

impl MemberStoreOneItemWirePreparationFactory<DemoSnapshot, DemoMutation> for DemoMemberWirePreparationFactory {
    fn preflight(&self, wire: &MemberStoreOneItemWire, description: Option<&str>, lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String> {
        if wire.schema != "demo.member.json-number" || wire.bytes.is_empty() || description.is_some_and(|description| description.len() > 256) || lane != HistoryLane::Document {
            return Err("demo retained member wire schema or metadata is not admitted".into());
        }
        Ok(ArtifactStoreOneItemFootprint { work_items: wire.bytes.len().saturating_add(3), retained_bytes: wire.bytes.len().saturating_add(1024) })
    }

    fn begin(
        &self,
        request: ArtifactStoreOneItemPreparationRequest<DemoSnapshot, MemberStoreOneItemWire>,
    ) -> Result<Box<dyn ArtifactStoreOneItemPreparation<DemoSnapshot, DemoMutation>>, ArtifactStoreOneItemPreparationRequest<DemoSnapshot, MemberStoreOneItemWire>> {
        Ok(Box::new(DemoMemberWirePreparation { request: Some(request), wire: None, inner: None, offset: 0, magnitude: 0, negative: false, state: 0, closing: false }))
    }
}

impl DemoMemberWirePreparation {
    fn parse_byte(&mut self, byte: u8) -> Result<(), String> {
        let whitespace = matches!(byte, b' ' | b'\t' | b'\n' | b'\r');
        match self.state {
            0 if whitespace => {}
            0 if byte == b'-' => {
                self.negative = true;
                self.state = 1;
            }
            0 | 1 if byte.is_ascii_digit() => {
                self.magnitude = u32::from(byte - b'0');
                self.state = if byte == b'0' { 2 } else { 3 };
            }
            3 if byte.is_ascii_digit() => {
                self.magnitude = self.magnitude.checked_mul(10).and_then(|value| value.checked_add(u32::from(byte - b'0'))).ok_or_else(|| "member number overflows i32".to_string())?;
                if self.magnitude > i32::MAX as u32 + u32::from(self.negative) {
                    return Err("member number overflows i32".into());
                }
            }
            2..=4 if whitespace => self.state = 4,
            _ => return Err("member wire is not an exact JSON integer".into()),
        }
        Ok(())
    }

    fn close_wire(wire: &mut MemberStoreOneItemWire, grant: ArtifactStoreOneItemGrant) -> Option<SnapshotRetirementStep> {
        if !wire.bytes.is_empty() {
            if grant.maximum_bytes == 0 {
                return Some(SnapshotRetirementStep::Blocked);
            }
            wire.bytes.pop();
            return Some(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 1 });
        }
        if let Some(scalar) = wire.schema.chars().next_back() {
            if scalar.len_utf8() > grant.maximum_bytes {
                return Some(SnapshotRetirementStep::Blocked);
            }
            wire.schema.pop();
            return Some(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: scalar.len_utf8() });
        }
        None
    }
}

impl ArtifactStoreOneItemPreparation<DemoSnapshot, DemoMutation> for DemoMemberWirePreparation {
    fn advance(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.closing {
            return Ok(ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if let Some(inner) = self.inner.as_mut() {
            return match inner.advance(grant)? {
                ArtifactStoreOneItemPreparationStep::Prepared(_) => Ok(ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint())),
                ArtifactStoreOneItemPreparationStep::Progress(_) => Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint())),
                ArtifactStoreOneItemPreparationStep::Blocked => Ok(ArtifactStoreOneItemPreparationStep::Blocked),
            };
        }
        if let Some(byte) = self.request.as_ref().and_then(|request| request.mutation.bytes.get(self.offset)).copied() {
            self.parse_byte(byte)?;
            self.offset += 1;
            return Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint()));
        }
        if !matches!(self.state, 2..=4) {
            return Err("member wire ended without an integer".into());
        }
        let request = self.request.take().ok_or_else(|| "member wire lost its request".to_string())?;
        let n = if self.negative { -(i64::from(self.magnitude)) as i32 } else { self.magnitude as i32 };
        self.wire = Some(request.mutation);
        let typed = ArtifactStoreOneItemPreparationRequest {
            operation: request.operation,
            generation: request.generation,
            base_revision: request.base_revision,
            lane: request.lane,
            authority: request.authority,
            description: request.description,
            base: request.base,
            mutation: DemoMutation::SetN(SetN { n }),
        };
        self.inner = Some(DemoOneItemPreparationFactory::admissible().begin(typed).unwrap_or_else(|_| panic!("demo typed owner accepts its exact decoded input")));
        Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint()))
    }

    fn checkpoint(&self) -> ArtifactStoreOneItemCheckpoint {
        let inner = self.inner.as_ref().map_or_else(ArtifactStoreOneItemCheckpoint::default, |inner| inner.checkpoint());
        ArtifactStoreOneItemCheckpoint { cursor: self.offset as u32 + inner.cursor, completed_items: self.offset as u32 + inner.completed_items, completed_bytes: self.offset as u64, digest: inner.digest }
    }

    fn prepared(&self) -> Option<&ArtifactStoreOneItemPrepared<DemoSnapshot, DemoMutation>> {
        self.inner.as_ref().and_then(|inner| inner.prepared())
    }

    fn take_prepared(&mut self) -> Option<ArtifactStoreOneItemPrepared<DemoSnapshot, DemoMutation>> {
        self.inner.as_mut().and_then(|inner| inner.take_prepared())
    }

    fn cancel(&mut self) {
        self.closing = true;
        if let Some(inner) = self.inner.as_mut() {
            inner.cancel();
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(inner) = self.inner.as_mut() {
            inner.begin_close();
        }
    }

    fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(inner) = self.inner.as_mut() {
            let step = inner.close_step(grant)?;
            if step != SnapshotRetirementStep::Complete {
                return Ok(step);
            }
            if !inner.terminal_is_empty() {
                return Err("member typed owner falsely reported terminal".into());
            }
            self.inner = None;
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(wire) = self.wire.as_mut() {
            if let Some(step) = Self::close_wire(wire, grant) {
                return Ok(step);
            }
            self.wire = None;
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(request) = self.request.as_mut() {
            if let Some(step) = Self::close_wire(&mut request.mutation, grant) {
                return Ok(step);
            }
            if let Some(description) = request.description.as_mut().filter(|value| !value.is_empty()) {
                let bytes = description.chars().next_back().unwrap().len_utf8();
                if bytes > grant.maximum_bytes {
                    return Ok(SnapshotRetirementStep::Blocked);
                }
                description.pop();
                return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: bytes });
            }
            let bytes = request.authority.actor.len() + request.authority.group_id.as_ref().map_or(0, String::len);
            if bytes > grant.maximum_bytes {
                return Ok(SnapshotRetirementStep::Blocked);
            }
            let request = self.request.take().unwrap();
            assert!(request.base.return_to_registry());
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: bytes });
        }
        Ok(SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.request.is_none() && self.wire.is_none() && self.inner.is_none()
    }
}

#[semio_framework_async_macros::async_test]
async fn retained_member_publication_preserves_order_group_identity_and_exact_maximum_grant_progress() {
    let fixture = member_publication_fixture();
    let grant = ArtifactStoreOneItemGrant { maximum_items: fixture["maximumItems"].as_u64().unwrap() as usize, maximum_bytes: fixture["maximumBytes"].as_u64().unwrap() as usize };
    let mut first = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "first", DemoSnapshot { n: Some(0) }, None)).await;
    let mut second = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "second", DemoSnapshot { n: Some(0) }, None)).await;
    first.install_member_store_owners_exact(retained_demo_member_owners());
    second.install_member_store_owners_exact(retained_demo_member_owners());
    let mut members = [RetainedTestMembers::First(first.0), RetainedTestMembers::Second(second.0)];
    let mut observed = Vec::new();
    for (sequence, row) in fixture["orderedMembers"].as_array().unwrap().iter().enumerate() {
        let index = usize::from(row["id"] == "second");
        let member = &mut members[index];
        assert!(member.one_item_wire_publication_supported());
        let (generation, revision) = member.one_item_publication_identity();
        let mut bytes = row["wire"].as_str().unwrap().as_bytes().to_vec();
        bytes.resize(bytes.len() + row["paddingBytes"].as_u64().unwrap() as usize, b' ');
        let oracle: i32 = serde_json::from_slice(&bytes).expect("independent third-party JSON decoder");
        let byte_count = bytes.len();
        let wire_ptr = bytes.as_ptr();
        let request = MemberStoreOneItemWireRequest {
            operation: semio_framework_job::OperationId(700 + sequence as u64),
            expected_generation: generation,
            expected_revision: revision,
            actor: "member-test".into(),
            group_id: Some(fixture["groupId"].as_str().unwrap().into()),
            wire: MemberStoreOneItemWire { schema: fixture["wireSchema"].as_str().unwrap().into(), bytes },
            description: None,
        };
        let mut publication = member.begin_one_item_wire_publication(request).unwrap_or_else(|_| panic!("exact member factory admits owned wire"));
        assert_eq!(publication.progress().completed_bytes, 0);
        assert!(matches!(member.advance_one_item_publication(&mut *publication, ArtifactStoreOneItemGrant { maximum_items: 0, maximum_bytes: 4096 }), Ok(ArtifactStoreOneItemAdvance::Blocked)));
        let mut published = false;
        for _ in 0..byte_count + 32 {
            let before = publication.progress();
            match member.advance_one_item_publication(&mut *publication, grant).expect("bounded retained member unit") {
                ArtifactStoreOneItemAdvance::Published(receipt) => {
                    assert_eq!(receipt.generation_after, generation + 1);
                    published = true;
                    break;
                }
                ArtifactStoreOneItemAdvance::Progress(after) => assert!(after.completed_bytes.saturating_sub(before.completed_bytes) <= grant.maximum_bytes as u64),
                step => panic!("member publication did not progress: {step:?}"),
            }
        }
        assert!(published);
        assert!(publication.retry());
        assert!(matches!(member.advance_one_item_publication(&mut *publication, grant), Ok(ArtifactStoreOneItemAdvance::AwaitingAck(_))));
        assert_eq!(member.one_item_publication_identity().0, generation + 1);
        assert!(publication.acknowledge());
        assert!(!publication.acknowledge());
        close_erased_member_publication(&mut *publication, grant);
        let concrete = member.as_any_mut().await.downcast_mut::<super::ArtifactStore<DemoSnapshot, DemoMutation>>().unwrap();
        assert_eq!(concrete.snapshot().unwrap().n, Some(oracle));
        assert_eq!(oracle as i64, row["expected"].as_i64().unwrap());
        assert_eq!(SpaceMember::tail_group_id(concrete).await.as_deref(), fixture["groupId"].as_str());
        observed.push(oracle);
        eprintln!("[DEBUG] retained member sequence={sequence} wire={wire_ptr:p} bytes={byte_count} value={oracle}");
    }
    assert_eq!(observed, vec![17, -23, 42]);
    for member in &mut members {
        for _ in 0..4096 {
            if member.close_owned_step(grant.maximum_items, grant.maximum_bytes).unwrap() == SnapshotRetirementStep::Complete {
                break;
            }
        }
        assert!(member.close_owned_terminal_is_empty());
    }
}

#[semio_framework_async_macros::async_test]
async fn retained_member_publication_rejects_wrong_owner_staleness_and_cancels_without_commit() {
    let grant = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4096 };
    let mut member = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "same-id", DemoSnapshot { n: Some(0) }, None)).await;
    let mut wrong = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "same-id", DemoSnapshot { n: Some(0) }, None)).await;
    member.install_member_store_owners_exact(retained_demo_member_owners());
    wrong.install_member_store_owners_exact(demo_closable_store_owners());
    let request = |generation, revision| MemberStoreOneItemWireRequest {
        operation: semio_framework_job::OperationId(800),
        expected_generation: generation,
        expected_revision: revision,
        actor: "member-test".into(),
        group_id: Some("cancel-group".into()),
        wire: MemberStoreOneItemWire { schema: "demo.member.json-number".into(), bytes: b"123".to_vec() },
        description: None,
    };
    let missing = request(wrong.generation(), wrong.content_revision_now());
    let original = missing.wire.bytes.as_ptr();
    let rejected = wrong.begin_one_item_wire_publication(missing).err().expect("missing member factory fails closed");
    assert_eq!(rejected.mutations.first().expect("rejection returns its exact admitted owners").bytes.as_ptr(), original, "rejection returns the exact input owner");
    let mut stale = member.begin_one_item_wire_publication(request(member.generation(), member.content_revision_now())).unwrap_or_else(|_| panic!("owned wire admits"));
    assert!(wrong.advance_one_item_publication(&mut *stale, grant).is_err(), "matching schema/id/revision cannot impersonate the exact store owner");
    assert_eq!(stale.progress().completed_bytes, 0);
    let mut replacement = member.begin_member_apply_batch(semio_framework_job::OperationId(801), member.generation(), member.content_revision_now(), "member-test".into(), vec![DemoMutation::SetN(SetN { n: 9 })], None).unwrap();
    for _ in 0..16 {
        if matches!(member.advance_apply_batch(&mut replacement, grant).unwrap(), ArtifactStoreOneItemAdvance::Published(_)) {
            break;
        }
    }
    assert!(replacement.acknowledge());
    close_durable_publication(&mut replacement);
    assert!(member.advance_one_item_publication(&mut *stale, grant).is_err());
    close_erased_member_publication(&mut *stale, grant);
    for steps in [0, 1, 3, 5] {
        let before = member.one_item_publication_identity();
        let mut cancelled = member.begin_one_item_wire_publication(request(before.0, before.1)).unwrap_or_else(|_| panic!("cancel owner admits"));
        for _ in 0..steps {
            assert!(!matches!(member.advance_one_item_publication(&mut *cancelled, grant).unwrap(), ArtifactStoreOneItemAdvance::Published(_)));
        }
        cancelled.begin_close();
        close_erased_member_publication(&mut *cancelled, grant);
        assert_eq!(member.one_item_publication_identity(), before);
        assert_eq!(member.snapshot().unwrap().n, Some(9));
    }
    close_demo_artifact_store(&mut member);
    close_demo_artifact_store(&mut wrong);
    eprintln!("[DEBUG] retained member wrong-owner/stale/cancel laws reached terminal-empty");
}

#[semio_framework_async_macros::async_test]
async fn retained_member_group_preparation_reserves_real_history_without_partial_visibility_and_aborts_stale_owners() {
    let grant = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4096 };
    let mut members = [
        ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "group-first", DemoSnapshot { n: Some(0) }, None)).await,
        ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "group-second", DemoSnapshot { n: Some(0) }, None)).await,
    ];
    for member in &mut members {
        member.install_member_store_owners_exact(retained_demo_member_owners());
    }
    let roots = [members[0].snapshot_root(), members[1].snapshot_root()];
    let before = [members[0].one_item_publication_identity(), members[1].one_item_publication_identity()];
    let mut publications = Vec::new();
    for (index, member) in members.iter().enumerate() {
        publications.push(
            member
                .begin_one_item_wire_publication(MemberStoreOneItemWireRequest {
                    operation: semio_framework_job::OperationId(900),
                    expected_generation: before[index].0,
                    expected_revision: before[index].1,
                    actor: "group-test".into(),
                    group_id: Some("reserved-group".into()),
                    wire: MemberStoreOneItemWire { schema: "demo.member.json-number".into(), bytes: if index == 0 { b"17".to_vec() } else { b"23".to_vec() } },
                    description: None,
                })
                .unwrap_or_else(|_| panic!("group member wire admits")),
        );
    }
    for index in 0..members.len() {
        let mut reserved = false;
        for _ in 0..32 {
            let step = members[index].prepare_one_item_publication(&mut *publications[index], grant).expect("one group preparation or reservation turn");
            for other in 0..members.len() {
                assert_eq!(members[other].one_item_publication_identity(), before[other]);
                assert!(members[other].envelope().vcs.edits.is_empty());
                assert!(members[other].applied_edit_ids().is_empty());
                assert_eq!(members[other].snapshot().unwrap().n, Some(0));
                assert!(Arc::ptr_eq(&roots[other], &members[other].snapshot_root()));
            }
            if matches!(step, ArtifactStoreOneItemPreparationStep::Prepared(_)) {
                reserved = true;
                break;
            }
        }
        assert!(reserved, "member must reach real history and retirement reservation under maximum grant");
        assert!(members[index].advance_one_item_publication(&mut *publications[index], grant).is_err(), "ordinary publication cannot consume a group-reserved candidate");
    }
    let mut competing = members[0].begin_member_apply_batch(semio_framework_job::OperationId(901), before[0].0, before[0].1, "other-test".into(), vec![DemoMutation::SetN(SetN { n: 99 })], None).unwrap();
    for _ in 0..16 {
        if competing.phase() == ArtifactStoreOneItemPublicationPhase::Publishing {
            break;
        }
        members[0].advance_apply_batch(&mut competing, grant).unwrap();
    }
    assert!(members[0].advance_apply_batch(&mut competing, grant).is_err(), "the real reserved edit slot excludes a competing append before any visible mutation");
    close_durable_publication(&mut competing);
    members[1].invalidate_after_external_resource_change().unwrap();
    assert!(members[1].prepare_one_item_publication(&mut *publications[1], grant).is_err(), "freshness is revalidated even after reservation");
    assert!(members[1].abort_one_item_publication(&mut *publications[0], grant).is_err(), "only the exact member can release its reservation");
    for index in 0..members.len() {
        publications[index].begin_close();
        assert_eq!(publications[index].close_step(grant).unwrap(), SnapshotRetirementStep::Blocked, "generic close cannot silently abandon live member reservations");
        for _ in 0..128 {
            if members[index].abort_one_item_publication(&mut *publications[index], grant).unwrap() == SnapshotRetirementStep::Complete {
                break;
            }
        }
        assert!(publications[index].terminal_is_empty());
        assert_eq!(members[index].snapshot().unwrap().n, Some(0));
        assert!(members[index].envelope().vcs.edits.is_empty());
        let reservation = members[index].reserve_edit_history_slot().expect("aborted group returns the exact ledger reservation");
        assert!(members[index].envelope.vcs.edits.cancel_reservation(reservation.history).is_ok());
        members[index].displaced_retirements.release_owner_slots(reservation.rejected_owner).unwrap();
    }
    drop(roots);
    for member in &mut members {
        close_demo_artifact_store(member);
    }
    eprintln!("[DEBUG] two-member retained preparation reserved real slots, exposed no edits, and aborted stale/cancelled owners without undo");
}
//#endregion 🧩️RetainedMemberPublicationLaws

pub(super) struct DemoOneItemPreparationFactory {
    footprint: ArtifactStoreOneItemFootprint,
    published_root: Arc<Mutex<Option<std::sync::Weak<DemoSnapshot>>>>,
    forge_digest: bool,
}

impl DemoOneItemPreparationFactory {
    pub(super) fn admissible() -> Self {
        Self { footprint: ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes: 512 }, published_root: Arc::new(Mutex::new(None)), forge_digest: false }
    }

    fn forged_digest() -> Self {
        Self { forge_digest: true, ..Self::admissible() }
    }
}

struct DemoOneItemPreparation {
    base: Option<SnapshotRead<DemoSnapshot>>,
    mutation: Option<DemoMutation>,
    description: Option<String>,
    authority: Option<Arc<ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<ArtifactStoreOneItemPrepared<DemoSnapshot, DemoMutation>>,
    checkpoint: ArtifactStoreOneItemCheckpoint,
    published_root: Arc<Mutex<Option<std::sync::Weak<DemoSnapshot>>>>,
    forge_digest: bool,
    active_id_retirement: Option<ArtifactStoreStringRetirement>,
    cancelled: bool,
    closing: bool,
}

impl ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation> for DemoOneItemPreparationFactory {
    fn preflight(&self, _mutation: &DemoMutation, _description: Option<&str>, _lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String> {
        Ok(self.footprint)
    }

    fn begin(&self, request: ArtifactStoreOneItemPreparationRequest<DemoSnapshot, DemoMutation>) -> Result<Box<dyn ArtifactStoreOneItemPreparation<DemoSnapshot, DemoMutation>>, ArtifactStoreOneItemPreparationRequest<DemoSnapshot, DemoMutation>> {
        Ok(Box::new(DemoOneItemPreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            prepared: None,
            checkpoint: ArtifactStoreOneItemCheckpoint::default(),
            published_root: self.published_root.clone(),
            forge_digest: self.forge_digest,
            active_id_retirement: None,
            cancelled: false,
            closing: false,
        }))
    }
}

impl ArtifactStoreOneItemPreparation<DemoSnapshot, DemoMutation> for DemoOneItemPreparation {
    fn advance(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() {
            return Ok(ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.cancelled {
            return Ok(ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "demo preparation lost its base root".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "demo preparation lost its mutation".to_string())?;
        let outcome = mutation.diff(base.get());
        if outcome.worst_level().is_some_and(|level| level >= crate::os_dsl::Severity::Error) {
            return Err("demo retained mutation rejected against its base".into());
        }
        let post_snapshot = Arc::new(outcome.diff().apply(base.get()).map_err(|error| error.to_string())?);
        let inverse = mutation.inverse(base.get());
        let authority = self.authority.as_ref().ok_or_else(|| "demo preparation lost its live Store authority".to_string())?;
        let sequence_number = authority.next_sequence_number;
        let id = format!("retained-one-item-{sequence_number}");
        let forward = mutation;
        let edit = Edit {
            id: id.clone(),
            actor: Some(authority.actor.clone()),
            forwards: vec![forward],
            inverse,
            mutation_meta: vec![MutationMeta {
                mutation_id: Some(MutationId(format!("{id}#0"))),
                dependencies: Vec::new(),
                base_version: 0,
                author_id: Some(ActorId(authority.actor.clone())),
                timestamp: authority.next_clock,
                undo_policy: UndoPolicy::ExactBaseOnly,
                payload_hash: None,
                semantic_kind: None,
                label: None,
                group_id: authority.group_id.clone(),
                origin: Default::default(),
            }],
            description: self.description.take(),
            coalesce_key: None,
            sequence_number,
            started_at: String::new(),
            finished_at: None,
        };
        *self.published_root.lock().expect("root witness lock") = Some(Arc::downgrade(&post_snapshot));
        let mut prepared = authority.prepare_one_item(edit, post_snapshot)?;
        if self.forge_digest {
            prepared.edit_digest[0] ^= 0xff;
        }
        let edit_digest = prepared.edit_digest;
        self.checkpoint = ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: edit_digest };
        self.prepared = Some(prepared);
        Ok(ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&ArtifactStoreOneItemPrepared<DemoSnapshot, DemoMutation>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<ArtifactStoreOneItemPrepared<DemoSnapshot, DemoMutation>> {
        self.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() {
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            assert!(base.return_to_registry());
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(active) = self.active_id_retirement.as_mut() {
            let step = active.close_step(grant.maximum_items.min(1), grant.maximum_bytes)?;
            if step != SnapshotRetirementStep::Complete {
                return Ok(step);
            }
            assert!(active.terminal_is_empty());
            self.active_id_retirement = None;
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(id) = self.description.take() {
            self.active_id_retirement = Some(ArtifactStoreStringRetirement::new(id));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            if grant.maximum_bytes < authority.actor.len().saturating_add(authority.group_id.as_ref().map_or(0, String::len)) {
                return Ok(SnapshotRetirementStep::Blocked);
            }
            drop(self.authority.take());
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        self.mutation = None;
        Ok(SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.prepared.is_none() && self.description.is_none() && self.authority.is_none() && self.active_id_retirement.is_none()
    }
}

struct DemoEphemeralPreparationFactory {
    footprint: ArtifactStoreOneItemFootprint,
    published_root: Arc<Mutex<Option<std::sync::Weak<DemoSnapshot>>>>,
}

impl DemoEphemeralPreparationFactory {
    fn admissible() -> Self {
        Self { footprint: ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: 64 }, published_root: Arc::new(Mutex::new(None)) }
    }
}

struct DemoEphemeralPreparation {
    request: Option<ArtifactEphemeralOneItemPreparationRequest<DemoSnapshot, DemoMutation>>,
    prepared: Option<ArtifactEphemeralOneItemPrepared<DemoSnapshot>>,
    checkpoint: ArtifactStoreOneItemCheckpoint,
    published_root: Arc<Mutex<Option<std::sync::Weak<DemoSnapshot>>>>,
    cancelled: bool,
    closing: bool,
}

impl ArtifactEphemeralOneItemPreparationFactory<DemoSnapshot, DemoMutation> for DemoEphemeralPreparationFactory {
    fn preflight(&self, _mutation: &DemoMutation) -> Result<ArtifactStoreOneItemFootprint, String> {
        Ok(self.footprint)
    }

    fn begin(
        &self,
        request: ArtifactEphemeralOneItemPreparationRequest<DemoSnapshot, DemoMutation>,
    ) -> Result<Box<dyn ArtifactEphemeralOneItemPreparation<DemoSnapshot, DemoMutation>>, ArtifactEphemeralOneItemPreparationRequest<DemoSnapshot, DemoMutation>> {
        Ok(Box::new(DemoEphemeralPreparation { request: Some(request), prepared: None, checkpoint: ArtifactStoreOneItemCheckpoint::default(), published_root: self.published_root.clone(), cancelled: false, closing: false }))
    }
}

impl ArtifactEphemeralOneItemPreparation<DemoSnapshot, DemoMutation> for DemoEphemeralPreparation {
    fn advance(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled {
            return Ok(ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let request = self.request.take().ok_or_else(|| "ephemeral preparation lost its request".to_string())?;
        let outcome = request.mutation.diff(&request.base);
        if outcome.worst_level().is_some_and(|level| level >= crate::os_dsl::Severity::Error) {
            return Err("demo ephemeral mutation rejected against its base".into());
        }
        let next_root = Arc::new(outcome.diff().apply(&request.base).map_err(|error| error.to_string())?);
        *self.published_root.lock().expect("ephemeral root witness lock") = Some(Arc::downgrade(&next_root));
        self.prepared = Some(ArtifactEphemeralOneItemPrepared { next_root });
        self.checkpoint = ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: [7; 32] };
        Ok(ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&ArtifactEphemeralOneItemPrepared<DemoSnapshot>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<ArtifactEphemeralOneItemPrepared<DemoSnapshot>> {
        self.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.request.take().is_some() {
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.request.is_none() && self.prepared.is_none()
    }
}

fn close_durable_publication(publication: &mut ArtifactStoreBatchPublication<DemoSnapshot, DemoMutation>) {
    for _ in 0..4_096 {
        let step = publication.close_step(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 512 }).expect("durable publication closes");
        if step == SnapshotRetirementStep::Complete {
            assert!(publication.terminal_is_empty());
            return;
        }
    }
    panic!("durable publication did not reach terminal empty");
}

fn close_demo_artifact_store(store: &mut ArtifactStore<DemoSnapshot, DemoMutation>) {
    for _ in 0..4_096 {
        let step = SpaceMember::close_owned_step(store, 1, 512).expect("demo artifact store closes under its bounded owner grant");
        if step == SnapshotRetirementStep::Complete {
            assert!(SpaceMember::close_owned_terminal_is_empty(store));
            return;
        }
    }
    panic!("demo artifact store did not reach its exact terminal-empty witness");
}

#[semio_framework_async_macros::async_test]
async fn apply_undo_redo_transfers_each_snapshot_root_to_one_exact_retirement_owner() {
    let completed = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "exact-tail-transfer", DemoSnapshot { n: Some(0) }, None)).await;
    store.install_member_store_owners_exact(MemberStoreOwners::new(
        Arc::new(ExactDemoSnapshotRetirementFactory(Arc::clone(&completed))),
        Arc::new(DemoInitialSnapshotRetirementFactory),
        Arc::new(DemoMutationRetirementFactory),
        Box::new(ArtifactStoreCursorDisposer::<DemoSnapshot, DemoMutation>::new()),
    ));
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 7 })], description: None }).await.expect("apply");
    store.dispatch(ArtifactCommand::Undo).await.expect("undo");
    store.dispatch(ArtifactCommand::Redo).await.expect("redo");
    assert_eq!(store.snapshot_ref().n, Some(7));
    close_demo_artifact_store(&mut store);
    assert_eq!(completed.load(std::sync::atomic::Ordering::SeqCst), 3, "displaced post-apply, retained pre-redo tail, and live post-redo roots each retire exactly once");
}

#[semio_framework_async_macros::async_test]
async fn store_close_waits_for_a_live_snapshot_read_then_retires_its_returned_owner() {
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "live-reader-close", DemoSnapshot { n: Some(0) }, None)).await;
    store.install_member_store_owners_exact(demo_closable_store_owners());
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 9 })], description: None }).await.expect("apply");
    let read = store.snapshot_read().expect("exact live snapshot read");
    let mut blocked = false;
    for _ in 0..4_096 {
        match SpaceMember::close_owned_step(&mut store, 1, 512).expect("store close advances to the reader boundary") {
            SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= 512),
            SnapshotRetirementStep::Blocked => {
                blocked = true;
                break;
            }
            SnapshotRetirementStep::Complete => panic!("a live snapshot read must retain its exact root"),
        }
    }
    assert!(blocked, "store close observes the live reader before detaching its current root");
    drop(read);
    close_demo_artifact_store(&mut store);
}

#[test]
fn returned_snapshot_read_releases_an_alias_before_the_displaced_root_and_retires_a_unique_fallback() {
    let completed = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let canonical = Arc::new(DemoSnapshot { n: Some(1) });
    let mut displaced = ExactDemoSnapshotRetirement { owner: Some(Arc::clone(&canonical)), value: None, completed: Arc::clone(&completed) };
    assert_eq!(displaced.close_step(1, 512).expect("displaced root observes the returned alias"), SnapshotRetirementStep::Blocked);
    let mut returned = ReturnedSnapshotReadRetirement::new(canonical, Arc::new(DemoInitialSnapshotRetirementFactory));
    assert_eq!(returned.close_step(1, 512).expect("returned read releases only its alias"), SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
    assert!(returned.terminal_is_empty());
    drop(returned);
    drive_retirement_terminal(Box::new(displaced));
    assert_eq!(completed.load(std::sync::atomic::Ordering::SeqCst), 1);

    let mut unique = ReturnedSnapshotReadRetirement::new(Arc::new(DemoSnapshot { n: Some(2) }), Arc::new(DemoInitialSnapshotRetirementFactory));
    assert_eq!(unique.close_step(1, 512).expect("last returned read transfers its unique value"), SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
    drive_retirement_terminal(Box::new(unique));
}

#[semio_framework_async_macros::async_test]
async fn store_close_releases_a_returned_read_before_its_displaced_root() {
    let completed = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "returned-before-displaced", DemoSnapshot { n: Some(0) }, None)).await;
    store.install_member_store_owners_exact(MemberStoreOwners::new(
        Arc::new(ExactDemoSnapshotRetirementFactory(Arc::clone(&completed))),
        Arc::new(DemoInitialSnapshotRetirementFactory),
        Arc::new(DemoMutationRetirementFactory),
        Box::new(ArtifactStoreCursorDisposer::<DemoSnapshot, DemoMutation>::new()),
    ));
    let read = store.snapshot_read().expect("read captures the pre-edit root");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 9 })], description: None }).await.expect("apply displaces the captured root");
    drop(read);
    close_demo_artifact_store(&mut store);
    assert_eq!(completed.load(std::sync::atomic::Ordering::SeqCst), 2, "the displaced and current roots retire exactly once each");
}

pub(super) fn demo_closable_store_owners() -> MemberStoreOwners<DemoSnapshot, DemoMutation> {
    MemberStoreOwners::new(Arc::new(DemoSnapshotRetirementFactory), Arc::new(DemoInitialSnapshotRetirementFactory), Arc::new(DemoMutationRetirementFactory), Box::new(ArtifactStoreCursorDisposer::<DemoSnapshot, DemoMutation>::new()))
}

fn close_ephemeral_publication(publication: &mut ArtifactEphemeralOneItemPublication<DemoSnapshot, DemoMutation>) {
    for _ in 0..16 {
        let step = publication.close_step(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 8 }).expect("ephemeral publication closes");
        if step == SnapshotRetirementStep::Complete {
            assert!(publication.terminal_is_empty());
            return;
        }
    }
    panic!("ephemeral publication did not reach terminal empty");
}

#[semio_framework_async_macros::async_test]
async fn artifact_store_one_item_single_retry_ack_and_move_only_root_preserve_generation_revision_and_history() {
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "retained-single", DemoSnapshot { n: Some(0) }, None)).await;
    store.install_member_store_owners_exact(demo_closable_store_owners());
    let generation = store.generation_now();
    let revision = store.content_revision_now();
    let factory = Arc::new(DemoOneItemPreparationFactory::admissible());
    let admitted: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = factory.clone();
    let mut publication = store
        .begin_apply_batch(semio_framework_job::OperationId(1), generation, revision, "retained-test".into(), vec![DemoMutation::SetN(SetN { n: 7 })], Some("single".into()), HistoryLane::Document, Some(&admitted))
        .expect("explicit domain factory admits");
    let grant = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 512 };
    let receipt = loop {
        if let ArtifactStoreOneItemAdvance::Published(receipt) = store.advance_apply_batch(&mut publication, grant).expect("one bounded durable step") {
            break receipt;
        }
    };
    assert_eq!(receipt, LaneItemReceipt { generation_before: generation, generation_after: generation + 1 });
    assert_eq!(store.snapshot().expect("published snapshot"), DemoSnapshot { n: Some(7) });
    assert_eq!(store.applied_edit_ids().len(), 1);
    assert_ne!(store.content_revision_now(), revision);
    let prepared_root = factory.published_root.lock().expect("root witness lock").as_ref().and_then(std::sync::Weak::upgrade).expect("published root remains owned");
    assert!(Arc::ptr_eq(&prepared_root, &store.snapshot_root()), "commit moves the exact prepared Arc root");
    assert!(publication.retry());
    assert!(publication.retry());
    assert!(!publication.retry());
    assert_eq!(store.generation_now(), generation + 1, "retry never republishes");
    assert!(publication.acknowledge());
    close_durable_publication(&mut publication);
    close_demo_artifact_store(&mut store);
}

//#region 🧺️BatchedPublication
/// 🧺️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B: drives one batched publication to its receipt
/// under a bounded per-turn grant, exactly as a host actor tick drives it.
async fn publish_demo_batch(
    store: &mut ArtifactStore<DemoSnapshot, DemoMutation>,
    operation: u64,
    mutations: Vec<DemoMutation>,
    description: Option<String>,
) -> (ArtifactStoreBatchPublication<DemoSnapshot, DemoMutation>, Result<LaneItemReceipt, VcsError>) {
    let factory: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = Arc::new(DemoOneItemPreparationFactory::admissible());
    let mut publication = store
        .begin_apply_batch(semio_framework_job::OperationId(operation), store.generation_now(), store.content_revision_now(), "retained-test".into(), mutations, description, HistoryLane::Document, Some(&factory))
        .unwrap_or_else(|rejected| panic!("batched admission: {}", rejected.reason));
    let grant = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 512 };
    for _ in 0..65_536 {
        match store.advance_apply_batch(&mut publication, grant) {
            Ok(ArtifactStoreOneItemAdvance::Published(receipt)) => return (publication, Ok(receipt)),
            Ok(_) => {}
            Err(error) => return (publication, Err(error)),
        }
    }
    panic!("batched publication never reached its receipt inside its bounded turn budget");
}

#[semio_framework_async_macros::async_test]
async fn artifact_store_reset_preserves_capacity_for_retained_batch_publication() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔁️reset-publication/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let initial = row["initial"].as_i64().unwrap() as i32;
        let after = row["after"].as_i64().unwrap() as i32;
        let fresh = || create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "reset-publication", DemoSnapshot { n: Some(initial) }, None);
        let mut store = ArtifactStore::new(fresh()).await;
        store.install_member_store_owners_exact(demo_closable_store_owners());
        store.reset(fresh(), Vec::new(), Vec::new()).await.expect("empty-history reset");
        let (mut publication, receipt) = publish_demo_batch(&mut store, 41, vec![DemoMutation::SetN(SetN { n: after })], None).await;
        assert!(publication.acknowledge() || receipt.is_err());
        publication.begin_close();
        close_durable_publication(&mut publication);
        let outcome = receipt.map(|_| serde_json::to_value(store.snapshot_ref().n).unwrap());
        close_demo_artifact_store(&mut store);
        assert_eq!(outcome.unwrap(), row["after"]);
    }
    eprintln!("[DEBUG] Store reset: two neutral empty-history reloads accept a retained durable batch and match serde projections");
}

/// 🧺️ ONE gesture of 200 mutations is ONE `Edit` in ONE ledger slot and ONE undo step — the
/// `ARTIFACT_HISTORY_LEDGER_CAPACITY = 64` ceiling that used to fault a `setActiveExample` load at
/// its 65th mutation is structurally out of reach, and the staged edit is byte-for-byte the edit
/// `ArtifactCommand::Apply` records for the same mutation list.
#[semio_framework_async_macros::async_test]
async fn artifact_store_batch_publication_stages_two_hundred_mutations_into_one_ledger_slot_and_one_undo_step() {
    const ITEMS: usize = 200;
    const { assert!(ITEMS > crate::os_vcs::ARTIFACT_HISTORY_LEDGER_CAPACITY, "the fixture must exceed the fixed edit ledger capacity to prove the ceiling is gone") };
    // 🧮️ `SetN`, not `AddN`: `AddN` emits one info message per operation and 200 of them exceed the
    // edit-message byte authority `ArtifactCommand::Apply` records — a limit of the message lane, not
    // of the staged edit, and the oracle below must apply the very same list.
    let mutations = (0..ITEMS).map(|index| DemoMutation::SetN(SetN { n: index as i32 + 1 })).collect::<Vec<_>>();
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "batched-gesture", DemoSnapshot { n: Some(0) }, None)).await;
    store.install_member_store_owners_exact(demo_closable_store_owners());
    let generation = store.generation_now();
    let (mut publication, receipt) = publish_demo_batch(&mut store, 1, mutations.clone(), Some("one gesture".into())).await;
    let receipt = receipt.expect("a two-hundred item gesture publishes once");
    assert_eq!(receipt, LaneItemReceipt { generation_before: generation, generation_after: generation + 1 });
    assert_eq!(publication.admitted_items(), ITEMS);
    assert_eq!(store.snapshot_ref().n, Some(ITEMS as i32), "every admitted mutation folded against the running post root");
    assert_eq!(store.applied_edit_ids().len(), 1, "one gesture consumes exactly one applied-edit slot");
    assert_eq!(store.envelope.vcs.edits.len(), 1, "one gesture consumes exactly one history ledger slot");
    let staged = store.envelope.vcs.edits.last().expect("staged gesture edit").clone();
    assert_eq!(staged.forwards.len(), ITEMS);
    assert_eq!(staged.mutation_meta.len(), ITEMS);
    assert_eq!(staged.inverse.len(), ITEMS);
    assert!(publication.acknowledge());
    close_durable_publication(&mut publication);

    let mut oracle = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "batched-oracle", DemoSnapshot { n: Some(0) }, None)).await;
    oracle.install_member_store_owners_exact(demo_closable_store_owners());
    oracle.dispatch(ArtifactCommand::Apply { mutations, description: Some("one gesture".into()) }).await.expect("the batched command oracle applies the same list");
    let expected = oracle.envelope.vcs.edits.last().expect("oracle edit");
    assert_eq!(staged.forwards, expected.forwards, "a staged gesture records the same forwards ArtifactCommand::Apply does");
    assert_eq!(staged.inverse, expected.inverse, "a staged gesture records the same inverse ordering ArtifactCommand::Apply does");
    assert_eq!(oracle.snapshot_ref().n, store.snapshot_ref().n);
    close_demo_artifact_store(&mut oracle);

    let post = store.snapshot_ref().clone();
    let mut backwards = post.clone();
    for operation in staged.inverse.iter().rev() {
        backwards = MutationDiff::apply(operation.diff(&backwards).diff(), &backwards).expect("staged inverse applies tail-first");
    }
    assert_eq!(backwards.n, Some(0), "the staged inverse is applied in reverse order of the forwards");
    let mut forwards = post;
    for operation in &staged.inverse {
        forwards = MutationDiff::apply(operation.diff(&forwards).diff(), &forwards).expect("staged inverse applies");
    }
    assert_ne!(forwards.n, Some(0), "consuming the staged inverse head-first is NOT the undo of the gesture");

    store.dispatch(ArtifactCommand::Undo).await.expect("one gesture is one undo step");
    assert_eq!(store.snapshot_ref().n, Some(0), "one undo reverts the whole gesture");
    assert!(store.applied_edit_ids().is_empty());
    store.dispatch(ArtifactCommand::Redo).await.expect("one gesture is one redo step");
    assert_eq!(store.snapshot_ref().n, Some(ITEMS as i32));
    close_demo_artifact_store(&mut store);
}

/// 🧺️ The single-mutation publication is literally the `N = 1` case of the same machine: same
/// phases, same receipt, one ledger slot.
#[semio_framework_async_macros::async_test]
async fn artifact_store_batch_publication_of_one_mutation_is_the_single_item_case() {
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "batched-single", DemoSnapshot { n: Some(0) }, None)).await;
    store.install_member_store_owners_exact(demo_closable_store_owners());
    let (mut publication, receipt) = publish_demo_batch(&mut store, 2, vec![DemoMutation::SetN(SetN { n: 7 })], None).await;
    receipt.expect("a single-item gesture publishes");
    assert_eq!(publication.admitted_items(), 1);
    assert_eq!(store.snapshot_ref().n, Some(7));
    let staged = store.envelope.vcs.edits.last().expect("staged edit");
    assert_eq!(staged.forwards, vec![DemoMutation::SetN(SetN { n: 7 })]);
    assert_eq!(staged.inverse, vec![DemoMutation::RestoreN(RestoreN { n: Some(0) })]);
    assert!(publication.acknowledge());
    close_durable_publication(&mut publication);
    close_demo_artifact_store(&mut store);
}

/// 🧺️ A gesture whose fourth mutation cannot prepare against the running post root commits
/// NOTHING: no ledger slot, no applied edit, no root replacement, no generation bump — and the
/// half-staged edit still closes to its exact terminal-empty witness.
#[semio_framework_async_macros::async_test]
async fn artifact_store_batch_rejection_mid_batch_leaves_no_partial_edit_and_still_retires_exactly() {
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "batched-rejection", DemoSnapshot { n: Some(0) }, None)).await;
    store.install_member_store_owners_exact(demo_closable_store_owners());
    let generation = store.generation_now();
    let revision = store.content_revision_now();
    let root = store.snapshot_root();
    let mutations = vec![DemoMutation::SetN(SetN { n: 1 }), DemoMutation::AddN(AddN { delta: 2 }), DemoMutation::DeleteN(DeleteN {}), DemoMutation::SetN(SetN { n: 5 })];
    let (mut publication, outcome) = publish_demo_batch(&mut store, 3, mutations, None).await;
    let error = outcome.expect_err("the fourth mutation cannot prepare against a deleted target");
    assert!(error.to_string().contains("demo retained mutation rejected against its base"), "{error}");
    assert!(publication.staged_items() < publication.admitted_items(), "the batch faulted before every item was staged");
    assert_eq!(store.generation_now(), generation, "a rejected gesture never bumps the generation");
    assert_eq!(store.content_revision_now(), revision);
    assert!(Arc::ptr_eq(&root, &store.snapshot_root()), "a rejected gesture never replaces the root");
    assert!(store.applied_edit_ids().is_empty(), "a rejected gesture leaves no partial applied edit");
    assert!(store.envelope.vcs.edits.is_empty(), "a rejected gesture consumes no history ledger slot");
    publication.begin_close();
    close_durable_publication(&mut publication);
    close_demo_artifact_store(&mut store);
}

/// 🧺️ Cancelling a gesture mid-flight retires the already-staged forwards, inverses, metadata and
/// running post root one owner per bounded turn, and never publishes.
#[semio_framework_async_macros::async_test]
async fn artifact_store_batch_cancel_mid_flight_retires_every_staged_owner_without_publishing() {
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "batched-cancel", DemoSnapshot { n: Some(0) }, None)).await;
    store.install_member_store_owners_exact(demo_closable_store_owners());
    let generation = store.generation_now();
    let root = store.snapshot_root();
    let factory: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = Arc::new(DemoOneItemPreparationFactory::admissible());
    let mut publication = store
        .begin_apply_batch(semio_framework_job::OperationId(4), generation, store.content_revision_now(), "retained-test".into(), (0..32).map(|_| DemoMutation::AddN(AddN { delta: 1 })).collect(), None, HistoryLane::Document, Some(&factory))
        .expect("a thirty-two item gesture admits");
    let grant = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 512 };
    while publication.staged_items() < 3 {
        store.advance_apply_batch(&mut publication, grant).expect("bounded staging turn");
    }
    assert!(store.cancel_apply_batch(&mut publication));
    assert!(matches!(store.advance_apply_batch(&mut publication, grant), Ok(ArtifactStoreOneItemAdvance::Blocked)));
    close_durable_publication(&mut publication);
    assert_eq!(store.generation_now(), generation, "a cancelled gesture never publishes");
    assert!(Arc::ptr_eq(&root, &store.snapshot_root()));
    assert!(store.envelope.vcs.edits.is_empty());
    close_demo_artifact_store(&mut store);
}
//#endregion 🧺️BatchedPublication

#[semio_framework_async_macros::async_test]
async fn retained_latest_wins_cold_rebase_preserves_admitted_cursor_capacity_for_next_publication() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🔌️plugin/🧫️fixtures/🔗️tool-latest-wins-integration.json")).unwrap();
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "cold-retained-rebase", DemoSnapshot { n: Some(0) }, None)).await;
    store.install_member_store_owners_exact(demo_closable_store_owners());
    let capacity = store.envelope.cursor.as_ref().unwrap().applied_edit_ids.capacity();
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 13 })], description: None }).await.unwrap();
    assert_eq!(store.envelope.cursor.as_ref().unwrap().applied_edit_ids.capacity(), capacity);
    assert_eq!(serde_json::json!(store.generation_now()), fixture["rebase"]["afterGeneration"]);
    let revision = store.content_revision_now();
    let factory: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = Arc::new(DemoOneItemPreparationFactory::admissible());
    let mut publication = store.begin_apply_batch(semio_framework_job::OperationId(93), store.generation_now(), revision, "fixture".into(), vec![DemoMutation::SetN(SetN { n: 97 })], None, HistoryLane::Document, Some(&factory)).unwrap();
    let grant = ArtifactStoreOneItemGrant { maximum_items: fixture["maximumItems"].as_u64().unwrap() as usize, maximum_bytes: fixture["maximumBytes"].as_u64().unwrap() as usize };
    for _ in 0..64 {
        if matches!(store.advance_apply_batch(&mut publication, grant).unwrap(), ArtifactStoreOneItemAdvance::Published(_)) {
            break;
        }
    }
    assert_eq!(serde_json::to_value(store.snapshot_root().as_ref()).unwrap(), serde_json::json!({ "n": 97 }));
    assert_eq!(store.generation_now(), 2);
    assert_eq!(store.envelope.cursor.as_ref().unwrap().applied_edit_ids.capacity(), capacity);
    assert!(publication.acknowledge());
    close_durable_publication(&mut publication);
    close_demo_artifact_store(&mut store);
    eprintln!("[DEBUG] cold cursor reconstruction preserved its admitted capacity for a real retained second publication");
}

#[semio_framework_async_macros::async_test]
async fn artifact_store_one_item_digest_helper_matches_validation_and_rejects_forged_cursor_history() {
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "retained-digest-authority", DemoSnapshot { n: Some(0) }, None)).await;
    store.install_member_store_owners_exact(demo_closable_store_owners());
    let generation = store.generation_now();
    let revision = store.content_revision_now();
    let root = store.snapshot_root();
    let forged: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = Arc::new(DemoOneItemPreparationFactory::forged_digest());
    let mut publication = store
        .begin_apply_batch(semio_framework_job::OperationId(11), generation, revision, "retained-test".into(), vec![DemoMutation::SetN(SetN { n: 12 })], Some("forged digest".into()), HistoryLane::Document, Some(&forged))
        .expect("Store-minted immutable authority admits the domain owner");
    let grant = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 512 };
    while publication.preparation.as_ref().and_then(|owner| owner.prepared()).is_none() {
        assert!(matches!(store.advance_apply_batch(&mut publication, grant), Ok(ArtifactStoreOneItemAdvance::Progress(_))));
    }
    let authority = publication.authority.as_ref().expect("publication retains Store authority");
    let prepared = publication.preparation.as_ref().and_then(|owner| owner.prepared()).expect("domain prepared candidate");
    assert_eq!(authority.operation(), semio_framework_job::OperationId(11));
    assert_eq!(authority.generation(), semio_framework_job::Generation(generation));
    assert_eq!(authority.base_revision(), revision);
    assert_eq!(authority.base_applied_edit_count(), 0);
    let canonical = authority.prepared_edit_digest(&prepared.edit).expect("Store helper accepts its exact semantic edit");
    assert_eq!(canonical, CursorRevisionAccumulator::edit_digest(&prepared.edit), "public helper and private Store validation share one canonical digest law");
    assert_ne!(prepared.edit_digest(), canonical, "hostile domain altered only its exposed candidate digest");
    assert!(store.advance_apply_batch(&mut publication, grant).is_err(), "Store recomputation rejects a domain-forged digest before publication");
    publication.begin_close();
    close_durable_publication(&mut publication);
    assert_eq!(store.generation_now(), generation);
    assert_eq!(store.content_revision_now(), revision);
    assert!(Arc::ptr_eq(&root, &store.snapshot_root()), "forged cursor/history authority cannot replace the root");
    close_demo_artifact_store(&mut store);
}

#[semio_framework_async_macros::async_test]
async fn artifact_store_one_item_stale_saturation_and_cancel_leave_root_generation_and_revision_unchanged() {
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "retained-rejections", DemoSnapshot { n: Some(3) }, None)).await;
    store.install_member_store_owners_exact(demo_closable_store_owners());
    let generation = store.generation_now();
    let revision = store.content_revision_now();
    let root = store.snapshot_root();
    let admissible: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = Arc::new(DemoOneItemPreparationFactory::admissible());
    let stale = store.begin_apply_batch(semio_framework_job::OperationId(2), generation + 1, revision, "retained-test".into(), vec![DemoMutation::SetN(SetN { n: 9 })], None, HistoryLane::Document, Some(&admissible));
    assert!(stale.is_err());

    let mut cancelled =
        store.begin_apply_batch(semio_framework_job::OperationId(3), generation, revision, "retained-test".into(), vec![DemoMutation::SetN(SetN { n: 8 })], None, HistoryLane::Document, Some(&admissible)).expect("fresh publication admits");
    assert!(store.cancel_apply_batch(&mut cancelled));
    close_durable_publication(&mut cancelled);
    assert_eq!(store.generation_now(), generation);
    assert_eq!(store.content_revision_now(), revision);
    assert!(Arc::ptr_eq(&root, &store.snapshot_root()));

    let saturated_ids = (0..crate::os_vcs::ARTIFACT_HISTORY_LEDGER_CAPACITY).map(|index| format!("saturated-{index}")).collect::<Vec<_>>();
    store.applied_edit_ids.extend(saturated_ids.iter().cloned());
    store.envelope.cursor.as_mut().expect("initialized cursor").applied_edit_ids.extend(saturated_ids.iter().cloned());
    store.revision_accumulator.applied.extend((0..crate::os_vcs::ARTIFACT_HISTORY_LEDGER_CAPACITY).map(|_| CursorRevisionRecord { id_digest: [1; 32], edit_digest: [2; 32], prefix_digest: [3; 32] }));
    let mut saturated = store
        .begin_apply_batch(semio_framework_job::OperationId(4), generation, revision, "retained-test".into(), vec![DemoMutation::SetN(SetN { n: 10 })], None, HistoryLane::Document, Some(&admissible))
        .expect("capacity is validated by retained commit preflight");
    let grant = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 512 };
    let saturation = (0..64).find_map(|_| store.advance_apply_batch(&mut saturated, grant).err()).expect("maximum plus one fails before store mutation");
    assert!(saturation.to_string().contains("preinstalled fixed applied and revision capacity"), "the ledger ceiling is what refuses the commit: {saturation}");
    saturated.begin_close();
    close_durable_publication(&mut saturated);
    assert_eq!(store.generation_now(), generation);
    assert_eq!(store.content_revision_now(), revision);
    assert!(Arc::ptr_eq(&root, &store.snapshot_root()));
    close_demo_artifact_store(&mut store);
}

struct FaultingEphemeralTask {
    invalid_receipt: bool,
    closing: bool,
}

impl ArtifactEphemeralPreparationTask<DemoSnapshot, DemoMutation> for FaultingEphemeralTask {
    fn advance(&mut self, _: &DemoSnapshot, _: &mut Option<DemoMutation>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactEphemeralPreparationTaskStep<DemoSnapshot>, String> {
        if !self.invalid_receipt {
            return Err("injected construction failure".into());
        }
        Ok(ArtifactEphemeralPreparationTaskStep::Prepared { root: DemoSnapshot { n: Some(99) }, checkpoint: ArtifactStoreOneItemCheckpoint { completed_items: 1, completed_bytes: grant.maximum_bytes as u64 + 1, ..Default::default() } })
    }
    fn begin_close(&mut self) {
        self.closing = true;
    }
    fn close_step(&mut self, _: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, String> {
        assert!(self.closing);
        Ok(SnapshotRetirementStep::Complete)
    }
    fn terminal_is_empty(&self) -> bool {
        self.closing
    }
}

#[test]
fn ephemeral_transfer_preparation_faults_close_presence_and_transient_owners() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🫧️preparation-fault/🔣️.json")).unwrap();
    let factory = ArtifactEphemeralTaskPreparationFactory::new(
        |_| Ok(ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: 64 }),
        |_, mutation| Ok(Box::new(FaultingEphemeralTask { invalid_receipt: matches!(mutation, DemoMutation::SetN(SetN { n: 1 })), closing: false })),
        Arc::new(DemoInitialSnapshotRetirementFactory),
        Arc::new(DemoMutationRetirementFactory),
    );
    for row in fixture["cases"].as_array().unwrap() {
        let mutation = || DemoMutation::SetN(SetN { n: row["mutation"].as_i64().unwrap() as i32 });
        let grant = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 7 };
        let mut presence = PresenceStore::<DemoSnapshot, DemoMutation>::new(DemoSnapshot { n: Some(0) });
        let presence_root_factory: Arc<dyn SnapshotRetirementFactory<DemoSnapshot>> = Arc::new(DemoSnapshotRetirementFactory);
        presence.install_local_retirement_factory(presence_root_factory.clone()).unwrap();
        let mut presence_publication = presence.begin_publish_one(semio_framework_job::OperationId(1), 0, mutation(), Some(&factory), Some(presence_root_factory)).unwrap();
        let mut transient = TransientStore::<DemoSnapshot, DemoMutation>::new(DemoSnapshot { n: Some(0) });
        let mut transient_publication = transient.begin_publish_one_leased(semio_framework_job::OperationId(2), 0, mutation(), &factory, Arc::new(DemoInitialSnapshotRetirementFactory)).unwrap();
        let presence_error = presence.advance_publish_one(&mut presence_publication, grant).unwrap_err();
        let transient_error = transient.advance_publish_one(&mut transient_publication, grant).unwrap_err();
        assert_eq!(presence_error, row["fault"].as_str().unwrap());
        assert_eq!(transient_error, presence_error);
        for publication in [&mut presence_publication, &mut transient_publication] {
            assert_eq!(publication.fault(), Some(presence_error.as_str()));
            assert_eq!(format!("{:?}", publication.phase()), fixture["phase"].as_str().unwrap());
            assert_eq!(publication.close_step(ArtifactStoreOneItemGrant { maximum_items: 0, maximum_bytes: 4096 }).unwrap(), SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            for _ in 0..256 {
                match publication.close_step(grant).unwrap() {
                    SnapshotRetirementStep::Complete => break,
                    SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= 7),
                    SnapshotRetirementStep::Blocked => panic!("isolated fault owner must close"),
                }
            }
            assert!(publication.terminal_is_empty());
        }
        assert_eq!(serde_json::to_value(presence.local().n).unwrap(), fixture["initial"]);
        assert_eq!(serde_json::to_value(transient.current_root().n).unwrap(), fixture["initial"]);
        let mut retirement = presence.begin_retirement(Arc::new(DemoSnapshot { n: Some(0) }), |value| value.n == Some(0)).ok().unwrap();
        for _ in 0..256 {
            if retirement.close_step(1, 7).unwrap() == SnapshotRetirementStep::Complete {
                break;
            }
        }
        assert!(retirement.terminal_is_empty());
    }
    eprintln!("[DEBUG] ephemeral preparation faults: task errors and invalid prepared receipts close both presence and transient owners while preserving neutral state");
}

#[semio_framework_async_macros::async_test]
async fn presence_and_transient_one_item_publications_are_retained_stale_safe_cancelable_and_exactly_closed() {
    let grant = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 64 };
    let presence_factory = DemoEphemeralPreparationFactory::admissible();
    let mut presence = PresenceStore::<DemoSnapshot, DemoMutation>::new(DemoSnapshot { n: Some(0) });
    let root_factory: Arc<dyn SnapshotRetirementFactory<DemoSnapshot>> = Arc::new(DemoSnapshotRetirementFactory);
    presence.install_local_retirement_factory(root_factory.clone()).unwrap();
    let old_presence_root = Arc::downgrade(&presence.local);
    let mut presence_publication = presence.begin_publish_one(semio_framework_job::OperationId(5), 0, DemoMutation::SetN(SetN { n: 5 }), Some(&presence_factory), Some(root_factory.clone())).expect("presence factory admits");
    let presence_receipt = loop {
        if let ArtifactStoreOneItemAdvance::Published(receipt) = presence.advance_publish_one(&mut presence_publication, grant).expect("presence step") {
            break receipt;
        }
    };
    assert_eq!(presence_receipt, LaneItemReceipt { generation_before: 0, generation_after: 1 });
    assert_eq!(presence.local().n, Some(5));
    let presence_root = presence_factory.published_root.lock().expect("presence witness").as_ref().and_then(std::sync::Weak::upgrade).expect("presence root");
    assert!(std::ptr::eq(presence_root.as_ref(), presence.local()));
    assert!(presence_publication.acknowledge());
    assert!(old_presence_root.upgrade().is_some(), "displaced presence root remains retained before bounded close");
    let _ = presence_publication.close_step(grant).expect("presence close advances one owner");
    assert!(old_presence_root.upgrade().is_some(), "first close unit cannot destroy the displaced root");
    close_ephemeral_publication(&mut presence_publication);
    for _ in 0..2048 {
        if presence.maintenance_local_reads_step(1, 4096).unwrap() == SnapshotRetirementStep::Complete {
            break;
        }
    }
    assert!(old_presence_root.upgrade().is_none(), "bounded retirement releases the displaced presence root");
    assert!(presence.begin_publish_one(semio_framework_job::OperationId(6), 0, DemoMutation::SetN(SetN { n: 9 }), Some(&presence_factory), Some(Arc::new(DemoSnapshotRetirementFactory))).is_err());

    let transient_factory = DemoEphemeralPreparationFactory::admissible();
    let mut transient = TransientStore::<DemoSnapshot, DemoMutation>::new(DemoSnapshot { n: Some(2) });
    let root = transient.current_root();
    let mut cancelled = transient.begin_publish_one(semio_framework_job::OperationId(7), 0, DemoMutation::SetN(SetN { n: 6 }), Some(&transient_factory), Some(Arc::new(DemoSnapshotRetirementFactory))).expect("transient factory admits");
    assert!(transient.cancel_publish_one(&mut cancelled));
    close_ephemeral_publication(&mut cancelled);
    assert_eq!(transient.generation_now(), 0);
    assert!(Arc::ptr_eq(&root, &transient.current_root()));
    drop(root);
    let old_transient_root = Arc::downgrade(&transient.current_root());
    let mut transient_publication =
        transient.begin_publish_one(semio_framework_job::OperationId(8), 0, DemoMutation::SetN(SetN { n: 11 }), Some(&transient_factory), Some(Arc::new(DemoSnapshotRetirementFactory))).expect("transient publication admits");
    let transient_receipt = loop {
        if let ArtifactStoreOneItemAdvance::Published(receipt) = transient.advance_publish_one(&mut transient_publication, grant).expect("transient step") {
            break receipt;
        }
    };
    assert_eq!(transient_receipt, LaneItemReceipt { generation_before: 0, generation_after: 1 });
    assert_eq!(transient.current().await.n, Some(11));
    let transient_root = transient_factory.published_root.lock().expect("transient witness").as_ref().and_then(std::sync::Weak::upgrade).expect("transient root");
    assert!(Arc::ptr_eq(&transient_root, &transient.current_root()));
    assert!(transient_publication.acknowledge());
    assert!(old_transient_root.upgrade().is_some(), "displaced transient root remains retained before bounded close");
    let _ = transient_publication.close_step(grant).expect("transient close advances one owner");
    assert!(old_transient_root.upgrade().is_some(), "first close unit cannot destroy the displaced transient root");
    close_ephemeral_publication(&mut transient_publication);
    assert!(old_transient_root.upgrade().is_none(), "bounded retirement releases the displaced transient root");
    let oversized =
        DemoEphemeralPreparationFactory { footprint: ArtifactStoreOneItemFootprint { work_items: ARTIFACT_STORE_ONE_ITEM_MAXIMUM_WORK_ITEMS + 1, retained_bytes: ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES }, published_root: Arc::new(Mutex::new(None)) };
    assert!(transient.begin_publish_one(semio_framework_job::OperationId(9), 1, DemoMutation::SetN(SetN { n: 7 }), Some(&oversized), Some(Arc::new(DemoSnapshotRetirementFactory))).is_err());
    assert!(transient.begin_publish_one(semio_framework_job::OperationId(10), 1, DemoMutation::SetN(SetN { n: 7 }), None, Some(Arc::new(DemoSnapshotRetirementFactory))).is_err());
    drop(presence_root);
    let mut close = presence.begin_retirement(Arc::new(DemoSnapshot { n: Some(0) }), |value| value.n == Some(0)).ok().unwrap();
    for _ in 0..2048 {
        if close.close_step(1, 4096).unwrap() == SnapshotRetirementStep::Complete {
            break;
        }
    }
    assert!(close.terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
#[should_panic(expected = "terminal-empty witness")]
async fn artifact_store_one_item_drop_rejects_an_unclosed_publication_owner() {
    let store = Box::leak(Box::new(ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "retained-drop", DemoSnapshot { n: Some(0) }, None)).await));
    store.install_member_store_owners_exact(demo_closable_store_owners());
    let publication = store
        .begin_apply_batch(
            semio_framework_job::OperationId(10),
            store.generation_now(),
            store.content_revision_now(),
            "retained-test".into(),
            vec![DemoMutation::SetN(SetN { n: 1 })],
            None,
            HistoryLane::Document,
            Some(&(Arc::new(DemoOneItemPreparationFactory::admissible()) as Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>>)),
        )
        .expect("publication admits");
    drop(publication);
}

/// 🪲️ A failing body that still holds a live, non-terminal `ArtifactStore` must surface as ONE
/// ordinary panic. Unguarded, the `Drop` witness fired during the unwind, and that double panic
/// became `thread caused non-unwinding panic. aborting.` — a `SIGABRT` that erased libtest's
/// summary for every other test in the binary. Sibling law to
/// `artifact_store_one_item_drop_rejects_an_unclosed_publication_owner` directly above, which
/// proves the witness stays strict when nothing is unwinding.
#[semio_framework_async_macros::async_test]
async fn a_panicking_body_holding_a_live_store_unwinds_instead_of_aborting_the_process() {
    let store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "unwind-witness", DemoSnapshot { n: Some(0) }, None)).await;
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let _live = store;
        panic!("simulated assertion failure with a live store in scope");
    }));
    std::panic::set_hook(previous);
    let payload = outcome.expect_err("the simulated failure must unwind, not abort the process");
    assert_eq!(payload.downcast_ref::<String>().map(String::as_str), Some("simulated assertion failure with a live store in scope"));
}
//#endregion 📬️OneItemPublicationLaws

//#region 📸️CompoundEnvelopeReadLaws
struct GroupReadTriggerSnapshot {
    value: i32,
    commit: Option<Arc<Mutex<crate::os_vcs::ArtifactGroupVisibilityOwner>>>,
    reads: Arc<std::sync::atomic::AtomicUsize>,
}

impl ToValue for GroupReadTriggerSnapshot {
    fn to_value(&self) -> DslValue {
        self.reads.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        if let Some(owner) = self.commit.as_ref() {
            owner.lock().expect("injected serializer decision").commit();
        }
        self.value.to_value()
    }
}

fn group_read_fixture_edit(id: &str) -> Edit<()> {
    Edit { id: id.into(), actor: None, forwards: Vec::new(), inverse: Vec::new(), mutation_meta: Vec::new(), description: None, coalesce_key: None, sequence_number: 0, started_at: String::new(), finished_at: None }
}

fn group_read_fixture_envelope(snapshot: GroupReadTriggerSnapshot) -> ArtifactEnvelope<GroupReadTriggerSnapshot, ()> {
    let mut edits = ArtifactHistoryLedger::new();
    edits.try_push(group_read_fixture_edit("initial")).unwrap();
    ArtifactEnvelope::from_owners(ArtifactEnvelopeOwners {
        schema: "group-read/v1".into(),
        id: "group-reader".into(),
        vcs: ArtifactVcs { initial_snapshot: snapshot, edits, changes: ArtifactHistoryLedger::new(), checkpoints: ArtifactHistoryLedger::new(), alternatives: ArtifactHistoryLedger::new() },
        backbone: None,
        active_alternative_id: None,
        cursor: Some(ArtifactCursor::new(vec!["initial".into()], Vec::new(), None)),
        dialect: None,
        migrated_from: None,
        owner: None,
        lanes: BTreeMap::new(),
        edit_messages: ArtifactEditMessageLedger::new(),
        conflicts: Vec::new(),
    })
}

fn close_group_read_fixture(envelope: ArtifactEnvelope<GroupReadTriggerSnapshot, ()>) {
    let mut owners = envelope.into_owners();
    let mut cursor = ArtifactStoreCursorRetirement::new(owners.cursor.take().unwrap());
    for _ in 0..1_024 {
        let step = cursor.close_step(1, 4_096).unwrap();
        if step == SnapshotRetirementStep::Complete {
            break;
        }
        if let SnapshotRetirementStep::Pending { released_items, released_bytes } = step {
            assert!(released_items <= 1 && released_bytes <= 4_096);
        }
    }
    assert!(cursor.terminal_is_empty());
    while owners.vcs.edits.pop().is_some() {}
}

#[test]
fn retained_group_envelope_read_captures_history_and_cursor_before_serializer_commit() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📖️group-read.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let owner = Arc::new(Mutex::new(crate::os_vcs::ArtifactGroupVisibilityOwner::new()));
        let view = owner.lock().unwrap().view();
        let inject_commit = case["capture"] == "pending" && case["decision"] == "committed";
        let reads = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let mut envelope = group_read_fixture_envelope(GroupReadTriggerSnapshot { value: 0, commit: inject_commit.then(|| owner.clone()), reads: Arc::clone(&reads) });
        let prepared_ids: Vec<String> = serde_json::from_value(fixture["prepared"]["appliedEditIds"].clone()).unwrap();
        envelope.cursor.as_mut().unwrap().stage_group_owned(ArtifactCursorOwners { applied_edit_ids: prepared_ids.clone(), redo_edit_ids: Vec::new(), checkpoint_id: None }, &view).unwrap();
        for id in &prepared_ids[1..] {
            let reservation = envelope.vcs.edits.reserve_group_one(&view).unwrap();
            envelope.vcs.edits.stage_group_reserved(reservation, group_read_fixture_edit(id), &view).unwrap();
        }
        if case["capture"] == "committed" {
            assert!(owner.lock().unwrap().commit());
        }
        let read = envelope.capture_read().unwrap();
        if case["decision"] == "aborted" {
            assert!(owner.lock().unwrap().abort());
        }
        let captured: serde_json::Value = serde_json::from_str(&crate::os_pack::json::to_json_string(&read)).unwrap();
        let fresh: serde_json::Value = serde_json::from_str(&crate::os_pack::json::to_json_string(&envelope.capture_read().unwrap())).unwrap();
        assert_eq!(reads.load(std::sync::atomic::Ordering::SeqCst), 2);
        for (value, selector) in [(&captured, "captured"), (&fresh, "fresh")] {
            let expected = &fixture[case[selector].as_str().unwrap()];
            let actual_ids: Vec<_> = value["vcs"]["edits"].as_array().unwrap().iter().map(|edit| edit["id"].clone()).collect();
            assert_eq!(serde_json::json!(actual_ids), expected["history"], "{} {selector}", case["id"]);
            assert_eq!(value["cursor"]["appliedEditIds"], expected["appliedEditIds"], "{} {selector}", case["id"]);
        }
        drop(read);
        let retired_cursor = if view.committed() {
            envelope.vcs.edits.adopt_group(&view).unwrap();
            envelope.cursor.as_mut().unwrap().adopt_group_owned(&view).unwrap()
        } else {
            owner.lock().unwrap().abort();
            while envelope.vcs.edits.abort_group_one(&view).unwrap().is_some() {}
            envelope.cursor.as_mut().unwrap().abort_group_owned(&view).unwrap()
        };
        let mut retirement = ArtifactStoreCursorRetirement::new(ArtifactCursor::from_owners(retired_cursor));
        for _ in 0..1_024 {
            if retirement.close_step(1, 4_096).unwrap() == SnapshotRetirementStep::Complete {
                break;
            }
        }
        assert!(retirement.terminal_is_empty());
        close_group_read_fixture(envelope);
        eprintln!("[DEBUG] compound envelope read {} serialized one captured history/cursor decision across injected commit", case["id"]);
    }
}

#[test]
fn retained_group_envelope_read_rejects_foreign_cursor_visibility_before_serialization() {
    let mut history_owner = crate::os_vcs::ArtifactGroupVisibilityOwner::new();
    let history = history_owner.view();
    let mut cursor_owner = crate::os_vcs::ArtifactGroupVisibilityOwner::new();
    let cursor = cursor_owner.view();
    let reads = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let mut envelope = group_read_fixture_envelope(GroupReadTriggerSnapshot { value: 0, commit: None, reads: Arc::clone(&reads) });
    let reservation = envelope.vcs.edits.reserve_group_one(&history).unwrap();
    envelope.vcs.edits.stage_group_reserved(reservation, group_read_fixture_edit("foreign"), &history).unwrap();
    envelope.cursor.as_mut().unwrap().stage_group_owned(ArtifactCursorOwners::default(), &cursor).unwrap();
    assert!(envelope.capture_read().is_err());
    assert!(envelope.capture_read().map(|read| crate::os_pack::json::to_json_string(&read)).is_err());
    assert_eq!(reads.load(std::sync::atomic::Ordering::SeqCst), 0);
    history_owner.abort();
    cursor_owner.abort();
    while envelope.vcs.edits.abort_group_one(&history).unwrap().is_some() {}
    drop(envelope.cursor.as_mut().unwrap().abort_group_owned(&cursor).unwrap());
    close_group_read_fixture(envelope);
}
//#endregion 📸️CompoundEnvelopeReadLaws

#[semio_framework_async_macros::async_test]
async fn retained_group_cursor_shares_history_visibility_and_retires_displaced_roots() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎯️group-cursor.json")).expect("group cursor fixture");
    let mut cursor: ArtifactCursor = serde_json::from_value(fixture["before"].clone()).expect("independent old cursor");
    let next: ArtifactCursorOwners = serde_json::from_value(fixture["after"].clone()).expect("independent next cursor");
    let mut owner = crate::os_vcs::ArtifactGroupVisibilityOwner::new();
    let view = owner.view();
    let mut history = ArtifactHistoryLedger::new();
    history.try_push("initial".to_string()).expect("old history");
    cursor.stage_group_owned(next, &view).expect("retained prepared cursor");
    for id in ["member-a", "member-b"] {
        let reservation = history.reserve_group_one(&view).expect("one history reservation");
        history.stage_group_reserved(reservation, id.to_string(), &view).expect("one prepared history record");
        assert_eq!(serde_json::to_value(&cursor).expect("old cursor serialization"), fixture["before"]);
        assert_eq!(cursor.applied_edit_ids.len(), 1);
        assert_eq!(history.len(), 1);
    }
    let held_before_commit: &ArtifactCursorOwners = &cursor;
    let held_history = history.iter();
    assert!(owner.commit());
    assert_eq!(held_before_commit.applied_edit_ids, vec!["initial"]);
    assert_eq!(held_history.cloned().collect::<Vec<_>>(), vec!["initial"]);
    assert_eq!(serde_json::to_value(&cursor).expect("committed cursor serialization"), fixture["after"]);
    assert_eq!(cursor.applied_edit_ids, history.iter().cloned().collect::<Vec<_>>());
    assert!(cursor.abort_group_owned(&view).is_err());
    let displaced = cursor.adopt_group_owned(&view).expect("exact displaced cursor owner");
    history.adopt_group(&view).expect("nonpublishing history adoption");
    assert_eq!(serde_json::to_value(&cursor).expect("adopted cursor serialization"), fixture["after"]);
    let mut retirement = ArtifactStoreCursorRetirement::new(ArtifactCursor::from_owners(displaced));
    let grant = (fixture["maximumItems"].as_u64().expect("item grant") as usize, fixture["maximumBytes"].as_u64().expect("byte grant") as usize);
    for _ in 0..1024 {
        match retirement.close_step(grant.0, grant.1).expect("bounded displaced cursor close") {
            SnapshotRetirementStep::Complete => break,
            SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= grant.0 && released_bytes <= grant.1),
            SnapshotRetirementStep::Blocked => panic!("cursor root owns every retirement byte"),
        }
    }
    assert!(retirement.terminal_is_empty());
    while history.pop().is_some() {}
    assert!(history.terminal_is_empty());
    let mut abort_owner = crate::os_vcs::ArtifactGroupVisibilityOwner::new();
    let abort_view = abort_owner.view();
    let mut wrong_owner = crate::os_vcs::ArtifactGroupVisibilityOwner::new();
    let wrong_view = wrong_owner.view();
    let next: ArtifactCursorOwners = serde_json::from_value(fixture["before"].clone()).expect("cancelled cursor owner");
    cursor.stage_group_owned(next, &abort_view).expect("unpublished next cursor");
    assert!(wrong_owner.abort());
    assert!(cursor.abort_group_owned(&wrong_view).is_err());
    assert!(abort_owner.abort());
    let cancelled = cursor.abort_group_owned(&abort_view).expect("returned exact uncommitted owner");
    assert_eq!(serde_json::to_value(cancelled).expect("cancelled owner oracle"), fixture["before"]);
    assert_eq!(serde_json::to_value(cursor).expect("unchanged committed cursor"), fixture["after"]);
}

#[semio_framework_async_macros::async_test]
async fn retained_group_cursor_empty_base_and_dropped_publisher_return_every_staged_owner() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎯️group-cursor.json")).expect("group cursor fixture");
    let mut cursor = ArtifactCursor::default();
    let mut history = ArtifactHistoryLedger::<String>::new();
    let owner = crate::os_vcs::ArtifactGroupVisibilityOwner::new();
    let view = owner.view();
    let next: ArtifactCursorOwners = serde_json::from_value(fixture["after"].clone()).expect("prepared cursor");
    cursor.stage_group_owned(next, &view).expect("staged empty-base cursor");
    #[cfg(not(target_arch = "wasm32"))]
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| cursor.applied_edit_ids.push("forbidden".into()))).is_err(), "DerefMut cannot expose either root while staged");
    let reservation = history.reserve_group_one(&view).expect("reserved but not staged slot");
    assert!(cursor.applied_edit_ids.is_empty() && history.is_empty());
    assert_eq!(serde_json::to_value(&cursor).expect("empty cursor"), serde_json::json!({}));
    drop(owner);
    assert!(!view.pending() && !view.committed(), "dropping the unique publisher aborts the decision without dropping staged payloads");
    history.cancel_reservation(reservation).expect("exact cancelled reservation");
    assert_eq!(history.abort_group_one(&view), Ok(None));
    assert!(history.terminal_is_empty());
    let cancelled = cursor.abort_group_owned(&view).expect("exact staged cursor survives publisher drop");
    assert_eq!(serde_json::to_value(&cancelled).expect("cancelled owner"), fixture["after"]);
    let mut retirement = ArtifactStoreCursorRetirement::new(ArtifactCursor::from_owners(cancelled));
    for _ in 0..1024 {
        match retirement.close_step(1, 4096).expect("cancelled cursor retirement") {
            SnapshotRetirementStep::Complete => break,
            SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= 4096),
            SnapshotRetirementStep::Blocked => panic!("cancelled cursor must remain closeable"),
        }
    }
    assert!(retirement.terminal_is_empty());
    assert!(cursor.applied_edit_ids.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn canonical_runtime_seed_retains_duplicate_owners_and_preflights_before_building() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🌱️runtime-seed.json")).expect("runtime seed fixture");
    let ids: Vec<MutationId> = serde_json::from_value::<Vec<String>>(fixture["identities"].clone()).expect("fixture identities").into_iter().map(MutationId).collect();
    let expected: Vec<String> = serde_json::from_value(fixture["applied"].clone()).expect("independent fixture applied identities");
    let retired: Vec<String> = serde_json::from_value(fixture["retired"].clone()).expect("independent fixture duplicate identities");
    super::ArtifactStore::<DemoSnapshot, DemoMutation>::preflight_runtime_seed(&ids).expect("duplicate identities are valid existing authority");
    let (mut dag, duplicates) = super::ArtifactStore::<DemoSnapshot, DemoMutation>::adopt_runtime_seed(ids);
    assert_eq!(duplicates, retired);
    let mut applied = Vec::new();
    while let Some(crate::os_spr::MutationDagCloseOwner::Identity(id)) = dag.take_one_close_owner() {
        applied.push(id);
    }
    applied.reverse();
    assert_eq!(applied, expected);
    assert!(dag.terminal_is_empty());
    let mut retirement = ArtifactStoreStringVectorRetirement::new(duplicates);
    let maximum_items = fixture["maximumItems"].as_u64().expect("item grant") as usize;
    let maximum_bytes = fixture["maximumBytes"].as_u64().expect("byte grant") as usize;
    for _ in 0..1024 {
        match retirement.close_step(maximum_items, maximum_bytes).expect("exact rejected identity retirement") {
            SnapshotRetirementStep::Complete => break,
            SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= maximum_items && released_bytes <= maximum_bytes),
            SnapshotRetirementStep::Blocked => panic!("owned seed identities must progress"),
        }
    }
    assert!(retirement.terminal_is_empty());
    let oversized = vec![MutationId("x".repeat(crate::os_spr::causal::MUTATION_DAG_IDENTIFIER_BYTES + 1))];
    assert!(super::ArtifactStore::<DemoSnapshot, DemoMutation>::preflight_runtime_seed(&oversized).is_err());
    let saturated: Vec<_> = (0..=crate::os_spr::causal::MUTATION_DAG_CAPACITY).map(|index| MutationId(index.to_string())).collect();
    assert!(super::ArtifactStore::<DemoSnapshot, DemoMutation>::preflight_runtime_seed(&saturated).is_err());
}

#[semio_framework_async_macros::async_test]
async fn canonical_revision_distinguishes_interior_aba_across_load_and_reset() {
    let mut original = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "revision-aba", DemoSnapshot { n: Some(0) }, None)).await;
    original.install_member_store_owners_exact(demo_closable_store_owners());
    for n in [1, 2, 3] {
        original.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n })], description: None }).await.expect("seed revision edit");
    }
    let original_revision = original.content_revision().await;
    let mut changed = owned_test_envelope(&original).await;
    changed.vcs.edits[1].forwards = vec![DemoMutation::SetN(SetN { n: 99 })];
    changed.vcs.edits[1].inverse = vec![DemoMutation::SetN(SetN { n: 1 })];
    let applied: Vec<String> = changed.vcs.edits.iter().map(|edit| edit.id.clone()).collect();
    changed.cursor = Some(ArtifactCursor::new(applied.clone(), Vec::new(), None));

    let mut loaded = ArtifactStore::new(changed).await;
    loaded.install_member_store_owners_exact(demo_closable_store_owners());
    assert_eq!(original.snapshot().expect("original snapshot").n, loaded.snapshot().expect("loaded snapshot").n, "interior ABA keeps the same materialized endpoint");
    assert_ne!(original_revision, loaded.content_revision().await, "canonical identity must cover the changed interior edit, not only cursor endpoints");

    original.reset(owned_test_envelope(&loaded).await, applied, Vec::new()).await.expect("reset changed history");
    assert_eq!(original.content_revision().await, loaded.content_revision().await, "reset reconstruction must recover the canonical loaded identity");
    close_demo_artifact_store(&mut loaded);
    close_demo_artifact_store(&mut original);
}

/// @emoji 🛰️ Builds a foreign {@link MutationEnvelope} (as if authored by `actor` on another peer) by

/// applying `operation` in a throwaway peer store and stamping the envelope's actor id.
async fn foreign_mutation_envelope(actor: &str, operation: DemoMutation) -> crate::os_spr::MutationEnvelope {
    let mut peer = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None)).await;
    peer.dispatch(ArtifactCommand::Apply { mutations: vec![operation], description: None }).await.expect("peer apply");
    let edit = peer.envelope().vcs.edits.last().expect("peer edit").clone();
    let document_id = ArtifactId(peer.envelope().id.clone());
    let schema = SchemaId(peer.envelope().schema.clone());
    let mut envelopes = crate::os_spr::mutation_envelope_from_edit::<DemoSnapshot, DemoMutation>(&edit, &document_id, &schema).expect("operation envelope");
    let mut envelope = envelopes.pop().expect("exactly one op envelope for a single-op edit");
    envelope.actor = ActorId(actor.to_string());
    envelope
}

#[semio_framework_async_macros::async_test]
async fn rejected_remote_ingest_keeps_state_and_dag_unpoisoned() {
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None)).await;
    let valid = foreign_mutation_envelope("peer", DemoMutation::SetN(SetN { n: 7 })).await;
    let mut malformed = valid.clone();
    malformed.diff.payload = vec![0xff];
    let before = owned_test_envelope(&store).await;

    assert!(store.ingest_remote(malformed).await.is_err(), "malformed remote data must reject before committing the DAG or history");
    assert_eq!(store.envelope(), &before);
    assert_eq!(store.snapshot().expect("unchanged snapshot"), DemoSnapshot { n: Some(0) });

    store.ingest_remote(valid).await.expect("the rejected envelope must not poison its mutation id in the DAG");
    assert_eq!(store.snapshot().expect("accepted snapshot"), DemoSnapshot { n: Some(7) });
}

#[semio_framework_async_macros::async_test]
async fn remote_ingest_requires_duplicate_mutation_payload_equivalence() {
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None)).await;
    let accepted = foreign_mutation_envelope("peer", DemoMutation::SetN(SetN { n: 2 })).await;
    store.ingest_remote(accepted.clone()).await.expect("first remote envelope");
    let before = owned_test_envelope(&store).await;

    store.ingest_remote(accepted.clone()).await.expect("an exact duplicate is idempotent");
    let mut conflict = foreign_mutation_envelope("peer", DemoMutation::SetN(SetN { n: 9 })).await;
    conflict.mutation_id = accepted.mutation_id.clone();
    let error = store.ingest_remote(conflict).await.expect_err("the same mutation id may not carry a different payload");
    assert!(matches!(error, VcsError::ValidationFailed(message) if message.contains("conflicts with its established payload")));
    assert_eq!(store.envelope(), &before);
    assert_eq!(store.snapshot().expect("unchanged snapshot"), DemoSnapshot { n: Some(2) });
}

#[semio_framework_async_macros::async_test]
async fn snapshot_merge_preflights_every_conflict_before_committing() {
    let mut local = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None)).await;
    local.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("local edit");
    local.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("local checkpoint".into()), authors: Vec::new() }).await.expect("local checkpoint");
    local.dispatch(ArtifactCommand::CreateAlternative { name: "local".into() }).await.expect("local alternative");
    let local_alternative = local.envelope().vcs.alternatives[0].id.clone();
    let before = owned_test_envelope(&local).await;

    let mut remote = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None)).await;
    remote.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 9 })], description: None }).await.expect("remote edit");
    remote.0.envelope.vcs.alternatives.try_push(Alternative { id: local_alternative, name: "conflicting remote alternative".into(), checkpoint_ids: Vec::new() }).expect("test alternative fits the fixed history ledger");
    let files = print_document_pack(remote.envelope()).await.expect("remote pack");

    assert!(local.merge_remote_snapshot(&files.pack, &files.spr).is_err(), "a late registry conflict must reject the whole snapshot merge");
    assert_eq!(local.envelope(), &before);
    assert_eq!(local.snapshot().expect("unchanged snapshot"), DemoSnapshot { n: Some(1) });
}

/// 🐛️ HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS w3-g: the store-level minimal
/// reproduction of `two_instances_converge_on_disjoint_edits_via_backbone`'s "invalid edit
/// reference" fault. `b` learns about `a`'s edit only via `ingest_remote`, which reconstructs it
/// under the WIRE per-op id (`edit_from_operation_envelope`'s `id: envelope.mutation_id.0`, the
/// `"{edit_id}#{opIndex}"` scheme from `mutation_ids_for_edit`) — never `a`'s own real `Edit.id`.
/// `b`'s own `CommitCheckpoint` right after that ingest is self-consistent (its `Change`
/// references the id it actually stored the edit under), so it succeeds — this is NOT yet the
/// bug. The bug surfaces once `a` commits its OWN checkpoint (`Change.edit_ids` naming `a`'s
/// edit under `a`'s real, un-suffixed id) and relays a full snapshot — exactly what
/// `flush_outbound(is_apply: false)` does for every structural command including
/// `CommitCheckpoint`. `merge_remote_snapshot`'s `batch.is_empty()` fast path (reached because
/// `b` already recognizes `a`'s edit as "known" via its wire-id-derived operation identity)
/// merges `a`'s `Change` in verbatim without reconciling ids, so `validate_durable_history`
/// rightly rejects it: `b`'s own `vcs.edits` never gained an entry under the bare id `a`'s
/// `Change` names.
#[semio_framework_async_macros::async_test]
async fn checkpoint_after_ingesting_a_remote_edit_stays_valid_once_the_sender_s_own_checkpoint_snapshot_arrives() {
    let mut a = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None)).await;
    a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("a's local edit");
    let a_edit = a.envelope().vcs.edits.last().expect("a has an edit").clone();
    let document_id = ArtifactId(a.envelope().id.clone());
    let schema = SchemaId(a.envelope().schema.clone());
    let wire_envelopes = crate::os_spr::mutation_envelope_from_edit::<DemoSnapshot, DemoMutation>(&a_edit, &document_id, &schema).expect("encode a's edit for the wire");

    let mut b = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None)).await;
    for envelope in wire_envelopes {
        b.ingest_remote(envelope).await.expect("b ingests a's edit over the wire");
    }
    b.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("b checkpoint".into()), authors: Vec::new() }).await.expect("b's own checkpoint, self-consistent so far — not yet the bug");
    assert!(!b.envelope().vcs.changes.last().expect("b minted a change").edit_ids.is_empty(), "b's checkpoint must actually cover the ingested edit");

    a.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("a checkpoint".into()), authors: Vec::new() }).await.expect("a's own checkpoint");
    let files = print_document_pack(a.envelope()).await.expect("a's pack");

    b.merge_remote_snapshot(&files.pack, &files.spr).expect("b must absorb a's checkpoint even though b only knows a's edit under its wire id");

    for change in &b.envelope().vcs.changes {
        for edit_id in &change.edit_ids {
            assert!(b.envelope().vcs.edits.iter().any(|edit| edit.id == *edit_id), "change {} references edit {edit_id}, which does not exist in b's own vcs.edits", change.id);
        }
    }
}

//#region 🔖️MergePolicyTests
// 🎯️ `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C6 acceptance —
// two-peer merge-policy/conflict tests, written standalone (not depending on 1-D's
// `📡️spr/🧪️testkit` `🔖️Laws` helpers landing first).

/// 🛰️ Builds a `MutationEnvelope` with an explicit HLC (bypassing wall-clock timing) so arrival
/// order and HLC order can be controlled independently — the two-peer tests below need both.
fn mutation_envelope_at(actor: &str, mutation_id: &str, operation: DemoMutation, hlc: HybridLogicalTimestamp, dependencies: Vec<MutationId>) -> crate::os_spr::MutationEnvelope {
    crate::os_spr::MutationEnvelope {
        mutation_id: MutationId(mutation_id.to_string()),
        document_id: ArtifactId("demo".to_string()),
        actor: ActorId(actor.to_string()),
        dependencies,
        diff: crate::os_spr::ArtifactDiff { schema: SchemaId("demo/v1".to_string()), payload: operation.encode_op().expect("encode demo mutation") },
        inverse: crate::os_spr::InverseMutation { schema: SchemaId("demo/v1".to_string()), payload: Vec::new() },
        timestamp: hlc,
    }
}

async fn fresh_demo_store() -> ArtifactStore<DemoSnapshot, DemoMutation> {
    ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None)).await
}

async fn owned_test_envelope<P, Mutation>(store: &ArtifactStore<P, Mutation>) -> ArtifactEnvelope<P, Mutation>
where
    P: Clone + ArtifactDsl + ArtifactPack + PartialEq + std::fmt::Debug + ToValue + FromValue + Send + 'static,
    Mutation: Clone + OpText + OpBinary + super::Mutation<P> + PartialEq + ToValue + FromValue + Send + 'static,
{
    let files = print_document_pack(store.envelope()).await.expect("print owned test envelope");
    parse_document_pack::<P, Mutation>(&files.pack, &files.spr).await.expect("parse owned test envelope").envelope
}

/// 🛰️ A `DeleteN` at an earlier HLC and a `SetN` at a later HLC — replayed in HLC order, the
/// `SetN` lands on an already-deleted target and raises `mutation.target-missing` (Error).
async fn modify_vs_delete_envelopes() -> (crate::os_spr::MutationEnvelope, crate::os_spr::MutationEnvelope) {
    let delete = mutation_envelope_at("deleter", "op-delete", DemoMutation::DeleteN(DeleteN {}), HybridLogicalTimestamp::new(1, 100), Vec::new());
    let modify = mutation_envelope_at("modifier", "op-modify", DemoMutation::SetN(SetN { n: 42 }), HybridLogicalTimestamp::new(2, 200), Vec::new());
    (delete, modify)
}

#[semio_framework_async_macros::async_test]
async fn modify_vs_delete_quarantines_under_normal_and_vigilant() {
    for policy in [crate::os_spr::MergePolicy::Normal, crate::os_spr::MergePolicy::Vigilant] {
        let mut store = fresh_demo_store().await;
        store.set_merge_policy(policy);
        let (delete, modify) = modify_vs_delete_envelopes().await;
        store.ingest_remote(delete).await.expect("the delete alone raises no message and always applies");
        let pre_merge = store.snapshot().expect("pre-merge snapshot");
        let pre_merge_ids = store.applied_edit_ids().to_vec();
        let report = store.ingest_remote(modify).await.expect("a policy rejection is a MergeReport, not an Err");
        assert!(!report.accepted, "{policy:?} must reject an Error-level modify-vs-delete conflict");
        assert_eq!(report.worst, Some(crate::os_dsl::Severity::Error));
        assert_eq!(store.snapshot().expect("unchanged snapshot"), pre_merge, "{policy:?}: state stays pre-merge on reject");
        assert_eq!(store.applied_edit_ids(), pre_merge_ids.as_slice(), "{policy:?}: applied_edit_ids stays pre-merge on reject");
        let conflict_id = report.conflict.clone().expect("a reject must raise a conflict");
        let conflict = store.conflicts().iter().find(|conflict| conflict.id == conflict_id).expect("conflict recorded on the store");
        assert_eq!(conflict.status, crate::os_spr::ConflictStatus::Open);
        assert!(matches!(conflict.kind, crate::os_spr::ConflictKind::Quarantined { .. }), "{policy:?}: a rejected batch quarantines, it never degrades");
        assert!(store.open_conflicts().any(|conflict| conflict.id == conflict_id));
    }
}

#[semio_framework_async_macros::async_test]
async fn modify_vs_delete_applies_under_laissez_faire_with_a_degraded_conflict() {
    let mut store = fresh_demo_store().await;
    store.set_merge_policy(crate::os_spr::MergePolicy::LaissezFaire);
    let (delete, modify) = modify_vs_delete_envelopes().await;
    store.ingest_remote(delete).await.expect("delete applies cleanly");
    let report = store.ingest_remote(modify).await.expect("LaissezFaire only rejects Fatal");
    assert!(report.accepted);
    assert_eq!(report.worst, Some(crate::os_dsl::Severity::Error));
    assert_eq!(store.snapshot().expect("snapshot").n, None, "the modify's part is absent — LAW 2 (an Error message ⇒ no change to its target)");
    assert!(report.replayed.iter().any(|edit_messages| edit_messages.messages.iter().any(|message| message.code.0 == "mutation.target-missing")), "an Error message must be reported");
    let conflict_id = report.conflict.expect("worst >= Warning must raise a Degraded conflict");
    let conflict = store.conflicts().iter().find(|conflict| conflict.id == conflict_id).expect("conflict recorded");
    assert!(matches!(conflict.kind, crate::os_spr::ConflictKind::Degraded { .. }));
    assert_eq!(conflict.status, crate::os_spr::ConflictStatus::Open);
}

#[semio_framework_async_macros::async_test]
async fn chronological_determinism_any_arrival_order_converges() {
    let a = mutation_envelope_at("actor-a", "op-a", DemoMutation::SetN(SetN { n: 10 }), HybridLogicalTimestamp::new(1, 100), Vec::new());
    let b = mutation_envelope_at("actor-b", "op-b", DemoMutation::SetN(SetN { n: 20 }), HybridLogicalTimestamp::new(2, 300), Vec::new());

    let mut forward = fresh_demo_store().await;
    forward.ingest_remote(a.clone()).await.expect("a");
    forward.ingest_remote(b.clone()).await.expect("b");

    let mut reversed = fresh_demo_store().await;
    reversed.ingest_remote(b).await.expect("b");
    reversed.ingest_remote(a).await.expect("a");

    assert_eq!(forward.snapshot().expect("snapshot"), reversed.snapshot().expect("snapshot"));
    assert_eq!(forward.applied_edit_ids(), reversed.applied_edit_ids(), "both must land in the same HLC order regardless of arrival order");
    assert_eq!(forward.applied_edit_ids(), &["op-a".to_string(), "op-b".to_string()]);
    assert_eq!(forward.conflicts().len(), reversed.conflicts().len());
    assert!(forward.conflicts().is_empty(), "two non-conflicting SetN edits raise no conflict");
}

#[semio_framework_async_macros::async_test]
async fn empty_store_snapshot_merge_replays_hlc_order_and_preserves_local_policy() {
    let mut remote = fresh_demo_store().await;
    let later = mutation_envelope_at("later-peer", "later-op", DemoMutation::SetN(SetN { n: 20 }), HybridLogicalTimestamp::new(2, 300), Vec::new());
    let earlier = mutation_envelope_at("earlier-peer", "earlier-op", DemoMutation::SetN(SetN { n: 10 }), HybridLogicalTimestamp::new(1, 100), Vec::new());
    remote.ingest_remote(later).await.expect("remote receives later edit first");
    remote.ingest_remote(earlier).await.expect("remote receives earlier edit second");
    let files = remote.snapshot_pack().await.expect("remote snapshot");

    let mut local = fresh_demo_store().await;
    local.set_merge_policy(crate::os_spr::MergePolicy::Vigilant);
    local.merge_remote_snapshot(&files.pack, &files.spr).expect("empty local history adopts valid remote history");

    assert_eq!(local.0.merge_policy(), crate::os_spr::MergePolicy::Vigilant, "the receiving store's local-only policy is never serialized or overwritten");
    assert_eq!(local.snapshot().expect("adopted snapshot"), DemoSnapshot { n: Some(20) });
    assert_eq!(local.applied_edit_ids(), &["earlier-op".to_string(), "later-op".to_string()], "adoption replays authoritative history by HLC rather than arrival order");
    assert_eq!(local.envelope().edit_messages, remote.envelope().edit_messages);
    assert_eq!(local.conflicts(), remote.conflicts());
}

/// 🎯️ w3-g id-domain unification: a single-op edit's wire id now literally EQUALS the edit's own
/// real id (`stamp_primary_operation_identity`), so there is nothing left to "remap" for this
/// case — `remap_snapshot_message_ledger` degenerates to an identity lookup. Kept (renamed from
/// `..._remaps_the_durable_message_ledger_to_the_wire_edit_id`, which asserted the now-fixed
/// divergence as its own fixture precondition) as the durable proof that a single-op edit's
/// message ledger survives ingest-then-snapshot under that shared id; the genuinely divergent
/// multi-op case is covered separately by
/// `operations_then_snapshot_partitions_a_multi_forward_ledger_by_wire_edit`.
#[semio_framework_async_macros::async_test]
async fn operations_then_snapshot_keeps_the_durable_message_ledger_on_the_shared_edit_id() {
    let mut remote = fresh_demo_store().await;
    remote.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 7 })], description: Some("remote source edit".into()) }).await.expect("remote apply");
    let source_edit = remote.envelope().vcs.edits.last().expect("source edit").clone();
    let document_id = ArtifactId(remote.envelope().id.clone());
    let schema = SchemaId(remote.envelope().schema.clone());
    let operation = crate::os_spr::mutation_envelope_from_edit::<DemoSnapshot, DemoMutation>(&source_edit, &document_id, &schema).expect("wire operation").pop().expect("one operation");
    assert_eq!(source_edit.id, operation.mutation_id.0, "a single-op edit's wire id must equal its own real id");
    let durable_message = crate::os_spr::MutationMessage::info("mutation.cascade", "remote diagnostic").at(["n"]).at_op(0);
    remote.0.envelope.edit_messages = ArtifactEditMessageLedger::from_preflighted_entries(vec![crate::os_spr::EditMessages { edit_id: source_edit.id.clone(), messages: vec![durable_message.clone()] }]);

    let mut local = fresh_demo_store().await;
    local.ingest_remote(operation.clone()).await.expect("operations delivery");
    let local_edit_id = operation.mutation_id.0.clone();
    let files = remote.snapshot_pack().await.expect("snapshot delivery");
    local.merge_remote_snapshot(&files.pack, &files.spr).expect("snapshot converges after operations");

    assert_eq!(local.envelope().vcs.edits.len(), 1, "snapshot must not duplicate the wire operation");
    let messages = local.envelope().edit_messages.iter().collect::<Vec<_>>();
    assert_eq!(messages, vec![&crate::os_spr::EditMessages { edit_id: local_edit_id, messages: vec![durable_message] }]);
}

#[semio_framework_async_macros::async_test]
async fn operations_then_snapshot_partitions_a_multi_forward_ledger_by_wire_edit() {
    let mut remote = fresh_demo_store().await;
    remote.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 7 }), DemoMutation::SetN(SetN { n: 8 })], description: Some("two source operations".into()) }).await.expect("remote apply");
    let source_edit = remote.envelope().vcs.edits.last().expect("source edit").clone();
    let document_id = ArtifactId(remote.envelope().id.clone());
    let schema = SchemaId(remote.envelope().schema.clone());
    let operations = crate::os_spr::mutation_envelope_from_edit::<DemoSnapshot, DemoMutation>(&source_edit, &document_id, &schema).expect("wire operations");
    assert_eq!(operations.len(), 2, "fixture source edit has two independent wire operations");
    remote.0.envelope.edit_messages = ArtifactEditMessageLedger::from_preflighted_entries(vec![crate::os_spr::EditMessages {
        edit_id: source_edit.id.clone(),
        messages: vec![crate::os_spr::MutationMessage::info("mutation.cascade", "first source diagnostic").at(["n"]).at_op(0), crate::os_spr::MutationMessage::info("mutation.cascade", "second source diagnostic").at(["n"]).at_op(1)],
    }]);

    let mut local = fresh_demo_store().await;
    for operation in &operations {
        local.ingest_remote(operation.clone()).await.expect("operations delivery");
    }
    let files = remote.snapshot_pack().await.expect("snapshot delivery");
    local.merge_remote_snapshot(&files.pack, &files.spr).expect("snapshot converges after operations");

    assert_eq!(local.envelope().vcs.edits.len(), 2, "snapshot must not restore the multi-forward source edit beside its two wire edits");
    assert_eq!(local.envelope().edit_messages.len(), 2, "one source ledger is deterministically split into its two established wire owners");
    for (entry, operation) in local.envelope().edit_messages.iter().zip(&operations) {
        assert_eq!(entry.edit_id, operation.mutation_id.0);
        assert_eq!(entry.messages.len(), 1);
        assert_eq!(entry.messages[0].op_index, Some(0), "the local one-operation edit owns index zero after redistribution");
    }
    assert_eq!(local.envelope().edit_messages.iter().next().expect("first message owner").messages[0].message, "first source diagnostic");
    assert_eq!(local.envelope().edit_messages.iter().nth(1).expect("second message owner").messages[0].message, "second source diagnostic");
}

#[semio_framework_async_macros::async_test]
async fn snapshot_ledger_remap_rejects_ambiguous_established_operation_ownership() {
    let mut remote = fresh_demo_store().await;
    remote.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 7 }), DemoMutation::SetN(SetN { n: 8 })], description: None }).await.expect("remote apply");
    let source_edit = remote.envelope().vcs.edits.last().expect("source edit").clone();
    let document_id = ArtifactId(remote.envelope().id.clone());
    let schema = SchemaId(remote.envelope().schema.clone());
    let operations = crate::os_spr::mutation_envelope_from_edit::<DemoSnapshot, DemoMutation>(&source_edit, &document_id, &schema).expect("wire operations");

    let mut local = fresh_demo_store().await;
    for operation in operations {
        local.ingest_remote(operation).await.expect("operations delivery");
    }
    let mut duplicate = local.envelope().vcs.edits.first().expect("first wire edit").clone();
    duplicate.id = "ambiguous-wire-owner".into();
    local.0.envelope.vcs.edits.try_push(duplicate).expect("test edit fits the fixed history ledger");

    assert!(matches!(local.snapshot_ledger_targets(&source_edit), Err(VcsError::ValidationFailed(message)) if message.contains("ambiguous established edit ownership")));
}

#[semio_framework_async_macros::async_test]
async fn empty_store_snapshot_merge_rejects_document_or_schema_mismatch_without_mutation() {
    for (schema, document_id) in [("demo/v1", "foreign"), ("foreign/v1", "demo")] {
        let remote = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>(schema, document_id, DemoSnapshot { n: Some(0) }, None)).await;
        let files = remote.snapshot_pack().await.expect("foreign snapshot");
        let mut target = fresh_demo_store().await;
        let before = owned_test_envelope(&target).await;
        let generation = target.generation();
        assert!(matches!(target.merge_remote_snapshot(&files.pack, &files.spr), Err(VcsError::ValidationFailed(_))));
        assert_eq!(target.envelope(), &before);
        assert_eq!(target.generation(), generation);
    }
}

#[semio_framework_async_macros::async_test]
async fn quarantine_accept_equals_laissez_faire_result() {
    let mut quarantined = fresh_demo_store().await;
    quarantined.set_merge_policy(crate::os_spr::MergePolicy::Normal);
    let (delete, modify) = modify_vs_delete_envelopes().await;
    quarantined.ingest_remote(delete.clone()).await.expect("delete applies cleanly");
    let reject_report = quarantined.ingest_remote(modify.clone()).await.expect("reject is a report");
    assert!(!reject_report.accepted);
    let conflict_id = reject_report.conflict.expect("conflict raised");
    quarantined.resolve_conflict(&conflict_id.0, crate::os_spr::ConflictResolution::Accept).await.expect("accept");

    let mut laissez_faire = fresh_demo_store().await;
    laissez_faire.set_merge_policy(crate::os_spr::MergePolicy::LaissezFaire);
    laissez_faire.ingest_remote(delete).await.expect("delete applies cleanly");
    laissez_faire.ingest_remote(modify).await.expect("modify applies under LaissezFaire");

    assert_eq!(quarantined.snapshot().expect("snapshot"), laissez_faire.snapshot().expect("snapshot"));
    assert_eq!(quarantined.applied_edit_ids(), laissez_faire.applied_edit_ids());
    assert_eq!(quarantined.conflicts().iter().filter(|conflict| conflict.status == crate::os_spr::ConflictStatus::Open).count(), 0, "no Open conflict remains — accept must not raise a second conflict");
    assert_eq!(quarantined.conflicts().iter().find(|conflict| conflict.id == conflict_id).expect("original conflict kept").status, crate::os_spr::ConflictStatus::Accepted);
}

#[semio_framework_async_macros::async_test]
async fn quarantine_discard_preserves_state() {
    let mut store = fresh_demo_store().await;
    store.set_merge_policy(crate::os_spr::MergePolicy::Normal);
    let (delete, modify) = modify_vs_delete_envelopes().await;
    store.ingest_remote(delete).await.expect("delete applies cleanly");
    let pre_discard = store.snapshot().expect("pre-discard snapshot");
    let pre_discard_ids = store.applied_edit_ids().to_vec();
    let reject_report = store.ingest_remote(modify).await.expect("reject is a report");
    let conflict_id = reject_report.conflict.expect("conflict raised");
    store.resolve_conflict(&conflict_id.0, crate::os_spr::ConflictResolution::Discard).await.expect("discard");
    assert_eq!(store.snapshot().expect("snapshot"), pre_discard, "a discarded batch must never be applied");
    assert_eq!(store.applied_edit_ids(), pre_discard_ids.as_slice());
    assert_eq!(store.conflicts().iter().find(|conflict| conflict.id == conflict_id).expect("conflict kept").status, crate::os_spr::ConflictStatus::Discarded);
}

#[semio_framework_async_macros::async_test]
async fn ledger_matches_a_fresh_replay_of_the_same_envelopes() {
    let (delete, modify) = modify_vs_delete_envelopes().await;
    let modify_edit_id = modify.mutation_id.0.clone();

    let mut first = fresh_demo_store().await;
    first.set_merge_policy(crate::os_spr::MergePolicy::LaissezFaire);
    first.ingest_remote(delete.clone()).await.expect("delete applies cleanly");
    first.ingest_remote(modify.clone()).await.expect("modify applies under LaissezFaire");

    let mut replay = fresh_demo_store().await;
    replay.set_merge_policy(crate::os_spr::MergePolicy::LaissezFaire);
    replay.ingest_remote(delete).await.expect("delete applies cleanly");
    replay.ingest_remote(modify).await.expect("modify applies under LaissezFaire");

    assert_eq!(first.messages_for_edit(&modify_edit_id), replay.messages_for_edit(&modify_edit_id));
    assert!(!first.messages_for_edit(&modify_edit_id).is_empty(), "the modify edit must have raised a message");
    assert_eq!(first.snapshot().expect("snapshot"), replay.snapshot().expect("snapshot"));
}

#[semio_framework_async_macros::async_test]
async fn applied_edit_ids_stay_sorted_by_hlc_after_a_backdated_remote_insert() {
    let mut store = fresh_demo_store().await;
    // Local edits get large physical-ms HLCs (the local clock ticks off the real wall clock).
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("local apply 1");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], description: None }).await.expect("local apply 2");
    let local_ids = store.applied_edit_ids().to_vec();

    // A remote edit stamped with a tiny HLC — guaranteed to sort before both local edits.
    let backdated = mutation_envelope_at("backdated-actor", "op-backdated", DemoMutation::SetN(SetN { n: 99 }), HybridLogicalTimestamp::new(9, 1), Vec::new());
    store.ingest_remote(backdated).await.expect("backdated insert");

    assert_eq!(store.applied_edit_ids()[0], "op-backdated", "the backdated edit must sort before both local edits");
    assert_eq!(&store.applied_edit_ids()[1..], local_ids.as_slice());

    let envelope = store.envelope();
    let hlcs: Vec<HybridLogicalTimestamp> = store.applied_edit_ids().iter().map(|id| envelope.vcs.edits.iter().find(|edit| edit.id == *id).and_then(|edit| edit.mutation_meta.first()).map(|meta| meta.timestamp).expect("meta")).collect();
    let mut cmp_keys = Vec::with_capacity(hlcs.len());
    for hlc in &hlcs {
        cmp_keys.push(hlc.cmp_key());
    }
    assert!(cmp_keys.windows(2).all(|pair| pair[0] <= pair[1]), "applied_edit_ids must stay HLC-sorted: {hlcs:?}");
}

//#region 🔖️TestkitLawWiring
// 🎯️ G2 verification barrier (26/08/16 MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-
// CONFLICTS) — the tests above assert the same scenarios by hand; these route the SAME
// scenarios through the frozen `📡️spr/🧪️testkit` `🔖️Merge`/`🔖️Conflict` law helpers so the
// laws are proven to hold against this store's real `ingest_remote`/`resolve_conflict`/
// `messages_for_edit`/`.spr` codec, not only against testkit's own synthetic self-tests.

#[semio_framework_async_macros::async_test]
async fn testkit_law_modify_vs_delete_holds_under_normal_and_vigilant() {
    for policy in [crate::os_spr::MergePolicy::Normal, crate::os_spr::MergePolicy::Vigilant] {
        let mut store = fresh_demo_store().await;
        store.set_merge_policy(policy);
        let (delete, modify) = modify_vs_delete_envelopes().await;
        store.ingest_remote(delete).await.expect("delete applies cleanly");
        let pre_merge = store.snapshot().expect("pre-merge snapshot");
        let report = store.ingest_remote(modify).await.expect("a policy rejection is a MergeReport, not an Err");
        let post_merge = store.snapshot().expect("post-merge snapshot");
        crate::os_spr::testkit::assert_modify_vs_delete(policy, &pre_merge, &post_merge, &report, store.conflicts(), |snapshot: &DemoSnapshot| snapshot.n.is_some()).await;
    }
}

#[semio_framework_async_macros::async_test]
async fn testkit_law_modify_vs_delete_holds_under_laissez_faire() {
    let mut store = fresh_demo_store().await;
    store.set_merge_policy(crate::os_spr::MergePolicy::LaissezFaire);
    let (delete, modify) = modify_vs_delete_envelopes().await;
    store.ingest_remote(delete).await.expect("delete applies cleanly");
    let pre_merge = store.snapshot().expect("pre-merge snapshot");
    let report = store.ingest_remote(modify).await.expect("LaissezFaire only rejects Fatal");
    let post_merge = store.snapshot().expect("post-merge snapshot");
    crate::os_spr::testkit::assert_modify_vs_delete(crate::os_spr::MergePolicy::LaissezFaire, &pre_merge, &post_merge, &report, store.conflicts(), |snapshot: &DemoSnapshot| snapshot.n.is_some()).await;
}

#[semio_framework_async_macros::async_test]
async fn testkit_law_chronological_determinism_holds_for_a_real_modify_vs_delete_batch() {
    let (delete, modify) = modify_vs_delete_envelopes().await;
    let envelopes = [delete, modify];
    crate::os_spr::testkit::assert_chronological_determinism(envelopes.len(), 7, 6, async |order| {
        let mut store = fresh_demo_store().await;
        for &index in order {
            store.ingest_remote(envelopes[index].clone()).await.expect("real store ingest must not hard-error even when the batch ends up quarantined");
        }
        let snapshot = store.snapshot().expect("snapshot");
        let applied = store.applied_edit_ids().to_vec();
        let conflict_ids: Vec<crate::os_spr::ConflictId> = store.conflicts().iter().map(|conflict| conflict.id.clone()).collect();
        (snapshot, applied, conflict_ids)
    })
    .await;
}

#[semio_framework_async_macros::async_test]
async fn testkit_law_quarantine_accept_equals_laissez_faire_via_real_store() {
    let mut quarantined = fresh_demo_store().await;
    quarantined.set_merge_policy(crate::os_spr::MergePolicy::Normal);
    let (delete, modify) = modify_vs_delete_envelopes().await;
    quarantined.ingest_remote(delete.clone()).await.expect("delete applies cleanly");
    let reject_report = quarantined.ingest_remote(modify.clone()).await.expect("reject is a report");
    let conflict_id = reject_report.conflict.expect("conflict raised");
    quarantined.resolve_conflict(&conflict_id.0, crate::os_spr::ConflictResolution::Accept).await.expect("accept");

    let mut laissez_faire = fresh_demo_store().await;
    laissez_faire.set_merge_policy(crate::os_spr::MergePolicy::LaissezFaire);
    laissez_faire.ingest_remote(delete).await.expect("delete applies cleanly");
    laissez_faire.ingest_remote(modify).await.expect("modify applies under LaissezFaire");

    let accepted_state = quarantined.snapshot().expect("accepted snapshot");
    let laissez_faire_state = laissez_faire.snapshot().expect("laissez-faire snapshot");
    crate::os_spr::testkit::assert_quarantine_accept_equals_laissez_faire(&accepted_state, &laissez_faire_state).await;
}

#[semio_framework_async_macros::async_test]
async fn testkit_law_quarantine_discard_preserves_state_via_real_store() {
    let mut store = fresh_demo_store().await;
    store.set_merge_policy(crate::os_spr::MergePolicy::Normal);
    let (delete, modify) = modify_vs_delete_envelopes().await;
    store.ingest_remote(delete).await.expect("delete applies cleanly");
    let pre_discard = store.snapshot().expect("pre-discard snapshot");
    let reject_report = store.ingest_remote(modify.clone()).await.expect("reject is a report");
    let conflict_id = reject_report.conflict.expect("conflict raised");
    store.resolve_conflict(&conflict_id.0, crate::os_spr::ConflictResolution::Discard).await.expect("discard");
    let post_discard = store.snapshot().expect("post-discard snapshot");
    // `relayed`: every edit id this store's persisted history (`applied_edit_ids`) could ever
    // ship onward via `flush_outbound`/`snapshot_pack` — a discarded batch is only `seed_
    // applied` on the dag, never added to `applied_edit_ids`/`vcs.edits`, so it can never appear
    // here; this is the real set flush_outbound draws from, not a fabricated stand-in.
    let relayed = store.applied_edit_ids().to_vec();
    crate::os_spr::testkit::assert_quarantine_discard_preserves_state(&pre_discard, &post_discard, std::slice::from_ref(&modify.mutation_id.0), &relayed).await;
}

#[semio_framework_async_macros::async_test]
async fn testkit_law_ledger_matches_replay_via_real_store() {
    let (delete, modify) = modify_vs_delete_envelopes().await;
    let modify_edit_id = modify.mutation_id.0.clone();

    let mut first = fresh_demo_store().await;
    first.set_merge_policy(crate::os_spr::MergePolicy::LaissezFaire);
    first.ingest_remote(delete.clone()).await.expect("delete applies cleanly");
    first.ingest_remote(modify.clone()).await.expect("modify applies under LaissezFaire");

    let mut replay = fresh_demo_store().await;
    replay.set_merge_policy(crate::os_spr::MergePolicy::LaissezFaire);
    replay.ingest_remote(delete).await.expect("delete applies cleanly");
    replay.ingest_remote(modify).await.expect("modify applies under LaissezFaire");

    let mut ledger = HashMap::new();
    ledger.insert(modify_edit_id.clone(), first.messages_for_edit(&modify_edit_id).to_vec());
    let mut replayed = HashMap::new();
    replayed.insert(modify_edit_id.clone(), replay.messages_for_edit(&modify_edit_id).to_vec());
    assert!(!ledger[&modify_edit_id].is_empty(), "the modify edit must have raised a message for this law to be meaningful");

    crate::os_spr::testkit::assert_ledger_matches_replay(&ledger, &replayed).await;
}

#[semio_framework_async_macros::async_test]
async fn testkit_law_conflict_spr_round_trip_via_real_store() {
    let mut store = fresh_demo_store().await;
    store.set_merge_policy(crate::os_spr::MergePolicy::Normal);
    let (delete, modify) = modify_vs_delete_envelopes().await;
    store.ingest_remote(delete).await.expect("delete applies cleanly");
    let report = store.ingest_remote(modify).await.expect("reject is a report");
    let conflict_id = report.conflict.expect("conflict raised");
    let conflict = store.conflicts().iter().find(|conflict| conflict.id == conflict_id).expect("conflict recorded").clone();

    let pack_bytes = DemoSnapshot { n: Some(0) }.encode_pack();
    let encode = async |conflict: &crate::os_spr::Conflict| -> Vec<u8> {
        let mut envelope = create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "conflict-round-trip", DemoSnapshot { n: Some(0) }, None);
        envelope.conflicts = vec![conflict.clone()];
        print_document_spr(&envelope).await.expect("encode conflict via the real .spr codec")
    };
    let decode = async |bytes: &[u8]| -> crate::os_spr::Conflict {
        let parsed = parse_document_spr::<DemoSnapshot, DemoMutation>(&pack_bytes, bytes).await.expect("decode conflict via the real .spr codec");
        parsed.envelope.conflicts.first().cloned().expect("one conflict round-tripped")
    };
    crate::os_spr::testkit::assert_conflict_spr_round_trip(&conflict, encode, decode).await;
}
//#endregion 🔖️TestkitLawWiring
//#endregion 🔖️MergePolicyTests

//#region 🔖️RobustnessTests
// 🎯️ `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` J1 — security/
// robustness audit findings HIGH-1, HIGH-2, MEDIUM-3, MEDIUM-4 (`📓️j1-robustness-fixes.md`).
// Every test here FAILS without its corresponding fix.

#[semio_framework_async_macros::async_test]
async fn replay_suffix_partitioned_errors_loudly_on_a_ghost_edit_id_instead_of_silently_dropping_it() {
    // HIGH-1: an id named in `order[k..]` but absent from `edits` means `applied_edit_ids`/
    // `vcs.edits` fell out of sync (crash/recovery, a partially-applied ingest) — a bare
    // `continue` here used to silently compute a WRONG snapshot instead of failing loudly.
    let edits: HashMap<String, Edit<DemoMutation>> = HashMap::new();
    let order = vec!["ghost-edit".to_string()];
    let error = super::ArtifactStore::<DemoSnapshot, DemoMutation>::replay_suffix_partitioned(&DemoSnapshot { n: Some(0) }, &order, 0, &edits, crate::os_spr::MergePolicy::Normal).expect_err("a ghost edit id must be a loud, typed VcsError");
    assert_eq!(error, VcsError::UnknownEdit("ghost-edit".into()));
}

#[semio_framework_async_macros::async_test]
async fn replay_suffix_errors_loudly_on_a_ghost_edit_id_instead_of_silently_dropping_it() {
    // HIGH-1, sibling function — `replay_suffix` is `merge_remote_snapshot`'s own replay
    // primitive and shares the exact same bare-`continue` defect before this fix.
    let edits: HashMap<String, Edit<DemoMutation>> = HashMap::new();
    let order = vec!["ghost-edit".to_string()];
    let error = super::ArtifactStore::<DemoSnapshot, DemoMutation>::replay_suffix(&DemoSnapshot { n: Some(0) }, &order, 0, &edits).expect_err("a ghost edit id must be a loud, typed VcsError");
    assert_eq!(error, VcsError::UnknownEdit("ghost-edit".into()));
}

#[semio_framework_async_macros::async_test]
async fn edits_for_ids_errors_loudly_on_a_ghost_edit_id_instead_of_silently_filtering_it() {
    // HIGH-2: minting a `ConflictId` from a `filter_map` that silently drops missing ids can
    // (if every id is missing) hash an EMPTY mutation-id set into a content-addressed conflict
    // id — a content address that addresses no content, so unrelated conflicts can collide on
    // it. `edits_for_ids` is the strict replacement both `ingest_remote` mint sites now use.
    let edits: HashMap<String, Edit<DemoMutation>> = HashMap::new();
    let error = super::ArtifactStore::<DemoSnapshot, DemoMutation>::edits_for_ids(&["ghost-edit".to_string()], &edits).expect_err("an id that resolves to nothing must fail loudly, never vanish from the mutation-id set");
    assert_eq!(error, VcsError::UnknownEdit("ghost-edit".into()));
}

/// 🛰️ A synthetic `Open` conflict for capacity/pruning fixtures — content doesn't matter, only
/// that its `ConflictId`/timestamp are distinct per `seed` and it never gets touched by real
/// replay logic (these tests push it directly onto `envelope.conflicts`, bypassing `ingest_
/// remote`/`resolve_conflict` entirely, exactly like the existing hand-built conflict fixtures
/// in `🔖️PreviewWireTests` above).
async fn synthetic_open_conflict(seed: u64) -> crate::os_spr::Conflict {
    let kind = crate::os_spr::ConflictKind::Degraded { edit_ids: vec![format!("synthetic-edit-{seed}")] };
    let artifact = ArtifactId("demo".into());
    let mutation_ids = vec![MutationId(format!("synthetic-op-{seed}"))];
    let timestamp = HybridLogicalTimestamp::new(seed, seed);
    crate::os_spr::Conflict {
        id: crate::os_spr::ConflictId::new(&kind, &artifact, &mutation_ids, &timestamp).await,
        kind,
        status: crate::os_spr::ConflictStatus::Open,
        messages: Vec::new(),
        actors: vec![ActorId(format!("synthetic-actor-{seed}"))],
        timestamp,
    }
}

#[semio_framework_async_macros::async_test]
async fn conflict_retirement_cursors_quarantined_payloads_messages_actors_and_ids() {
    let mut envelope = foreign_mutation_envelope("conflict-actor", DemoMutation::SetN(SetN { n: 9 })).await;
    envelope.dependencies = (0..32).map(|index| MutationId(format!("dependency-{index}-{}", "d".repeat(128)))).collect();
    envelope.diff.payload = vec![7; 16 * 1024];
    envelope.inverse.payload = vec![3; 16 * 1024];
    let kind = crate::os_spr::ConflictKind::Quarantined { envelopes: vec![envelope] };
    let timestamp = HybridLogicalTimestamp::new(41, 43);
    let conflict = crate::os_spr::Conflict {
        id: crate::os_spr::ConflictId::new(&kind, &ArtifactId("conflict-document".repeat(64)), &[MutationId("conflict-mutation".repeat(64))], &timestamp).await,
        kind,
        status: crate::os_spr::ConflictStatus::Open,
        messages: vec![crate::os_spr::MutationMessage::fatal("mutation.conflict", "message".repeat(256)).at(["target".repeat(256)])],
        actors: vec![ActorId("actor".repeat(256))],
        timestamp,
    };
    let mut retirement = ArtifactStoreConflictRetirement::new(conflict);
    let mut turns = 0;
    while !retirement.terminal_is_empty() {
        match retirement.close_step(1, 13).expect("bounded conflict close") {
            SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= 13),
            SnapshotRetirementStep::Blocked => panic!("owned conflict retirement has no external wait"),
            SnapshotRetirementStep::Complete => assert!(retirement.terminal_is_empty()),
        }
        turns += 1;
        assert!(turns < 100_000, "deep conflict retirement terminates across repeated interruption");
    }
}

#[semio_framework_async_macros::async_test]
async fn ingest_remote_refuses_a_new_open_conflict_once_the_backlog_is_at_capacity() {
    // MEDIUM-3: a peer that keeps sending a batch this replica keeps quarantining can grow
    // `envelope.conflicts` without bound (the dag never advances on quarantine, so the SAME
    // envelope is eligible for redelivery forever). Hitting the open-conflict cap must be a
    // loud, typed refusal — atomic, nothing applied — never a silent drop or overwrite.
    let mut store = fresh_demo_store().await;
    store.set_merge_policy(crate::os_spr::MergePolicy::Normal);
    let (delete, modify) = modify_vs_delete_envelopes().await;
    store.ingest_remote(delete).await.expect("the delete alone raises no message and always applies");

    let cap = super::ArtifactStore::<DemoSnapshot, DemoMutation>::OPEN_CONFLICT_CAP;
    for seed in 0..cap as u64 {
        store.0.envelope.conflicts.push(synthetic_open_conflict(seed).await);
    }
    assert_eq!(store.open_conflicts().count(), cap, "fixture must actually be at capacity for this test to be meaningful");

    let before = owned_test_envelope(&store).await;
    let before_snapshot = store.snapshot().expect("pre-attempt snapshot");
    let before_ids = store.applied_edit_ids().to_vec();

    let error = store.ingest_remote(modify).await.expect_err("minting one more Open conflict past the cap must be a loud, typed refusal");
    assert!(matches!(&error, VcsError::ValidationFailed(message) if message.contains("capacity")), "got {error:?}");
    assert_eq!(store.envelope(), &before, "a refused mint must be fully atomic — nothing about the attempted batch applied");
    assert_eq!(store.snapshot().expect("snapshot"), before_snapshot);
    assert_eq!(store.applied_edit_ids(), before_ids.as_slice());
    assert_eq!(store.open_conflicts().count(), cap, "the backlog must stay exactly at capacity, never silently grow past it");
}

#[semio_framework_async_macros::async_test]
async fn resolved_conflicts_are_pruned_oldest_first_once_the_ledger_exceeds_its_cap_while_open_ones_survive() {
    // MEDIUM-3: resolved (`Accepted`/`Discarded`) conflicts are closed historical facts, so
    // unlike `Open` ones they are prunable — oldest push order evicted first, capped, while an
    // `Open` conflict is never touched no matter how far over cap the resolved backlog grows.
    let mut store = fresh_demo_store().await;
    let cap = super::ArtifactStore::<DemoSnapshot, DemoMutation>::RESOLVED_CONFLICT_CAP;
    let open = synthetic_open_conflict(u64::MAX).await;
    store.0.envelope.conflicts.push(open.clone());
    let overflow: u64 = 20;
    let resolved_count = cap as u64 + overflow;
    for seed in 0..resolved_count {
        let mut resolved = synthetic_open_conflict(seed).await;
        resolved.status = crate::os_spr::ConflictStatus::Accepted;
        store.0.envelope.conflicts.push(resolved);
    }
    assert_eq!(store.conflicts().len(), resolved_count as usize + 1);

    store.0.prune_resolved_conflicts().expect("bounded resolved-conflict retirement");

    assert_eq!(store.conflicts().len(), cap, "pruning must bring the ledger back down to the cap");
    assert!(store.conflicts().iter().any(|conflict| conflict.id == open.id && conflict.status == crate::os_spr::ConflictStatus::Open), "the Open conflict must never be evicted, no matter how far over cap the resolved backlog grows");
    let surviving_seeds: HashSet<u64> = store.conflicts().iter().filter(|conflict| conflict.status == crate::os_spr::ConflictStatus::Accepted).map(|conflict| conflict.timestamp.physical_ms).collect();
    assert_eq!(surviving_seeds.len(), cap - 1);
    for evicted_seed in 0..=overflow {
        assert!(!surviving_seeds.contains(&evicted_seed), "seed {evicted_seed} was among the oldest resolved conflicts and must be pruned first");
    }
    assert!(surviving_seeds.contains(&(resolved_count - 1)), "the newest resolved conflict must survive pruning");
}

#[semio_framework_async_macros::async_test]
async fn quarantine_message_clearing_is_correct_for_a_mixed_new_and_retroactive_batch() {
    // MEDIUM-4: one `ingest_remote` batch that quarantines BOTH a brand-new edit (`op-c`, never
    // committed before) AND a previously-committed edit a rewind now retroactively invalidates
    // (`op-a`, which carried a REAL non-empty `mutation.cascade` message from its first commit)
    // — proving `replace_edit_messages(.., empty)` clears the stale ledger entry correctly even
    // when it runs in the same pass as a never-populated one, regardless of which kind an id is.
    let mut store = fresh_demo_store().await;
    store.set_merge_policy(crate::os_spr::MergePolicy::Normal);

    let a = mutation_envelope_at("actor-a", "op-a", DemoMutation::AddN(AddN { delta: 5 }), HybridLogicalTimestamp::new(1, 100), Vec::new());
    store.ingest_remote(a).await.expect("op-a commits cleanly the first time, on a fresh n=0 target");
    assert!(!store.messages_for_edit("op-a").is_empty(), "fixture must carry real prior ledger content for this test to be meaningful");
    assert_eq!(store.snapshot().expect("snapshot"), DemoSnapshot { n: Some(5) });

    // `op-b` (earlier HLC than `op-a`) forces a rewind that replays `op-a` against a DELETED
    // target the second time around. `op-c` (later HLC, brand new) is submitted FIRST but
    // depends on `op-b`, so it buffers in the dag and is released alongside `op-b` in the SAME
    // `ingest_remote` call/batch — the mixed-kind scenario MEDIUM-4 asks for.
    let b = mutation_envelope_at("actor-b", "op-b", DemoMutation::DeleteN(DeleteN {}), HybridLogicalTimestamp::new(1, 50), Vec::new());
    let c = mutation_envelope_at("actor-c", "op-c", DemoMutation::AddN(AddN { delta: 1 }), HybridLogicalTimestamp::new(2, 150), vec![MutationId("op-b".into())]);
    store.ingest_remote(c).await.expect("op-c buffers behind its unmet dependency on op-b");
    let report = store.ingest_remote(b).await.expect("op-b's arrival releases op-b and op-c into the same batch");

    assert!(!report.accepted, "op-c never committed");
    assert_eq!(store.applied_edit_ids(), &["op-b".to_string()], "only op-b committed — op-a dropped out on retroactive invalidation, op-c never entered");
    assert_eq!(store.snapshot().expect("snapshot"), DemoSnapshot { n: None });

    assert!(store.messages_for_edit("op-a").is_empty(), "op-a's stale non-empty ledger entry must be cleared, not left stale, once it is retroactively quarantined");
    assert!(store.messages_for_edit("op-c").is_empty(), "op-c never committed, so it must never gain a ledger entry");
    assert!(store.envelope().edit_messages.iter().all(|entry| entry.edit_id != "op-a"), "a cleared entry must be fully removed from the durable ledger, not left behind as an empty Vec");
    assert!(store.envelope().edit_messages.iter().all(|entry| entry.edit_id != "op-c"), "op-c must never appear in the durable ledger at all");

    let conflict_id = report.conflict.expect("the mixed quarantine batch must raise one conflict");
    let conflict = store.conflicts().iter().find(|conflict| conflict.id == conflict_id).expect("conflict recorded on the store");
    assert_eq!(conflict.status, crate::os_spr::ConflictStatus::Open);
    match &conflict.kind {
        crate::os_spr::ConflictKind::Quarantined { envelopes } => assert_eq!(envelopes.len(), 2, "both op-a (retroactive) and op-c (new) must be quarantined TOGETHER in one conflict"),
        other => panic!("expected a Quarantined conflict, got {other:?}"),
    }
    assert_eq!(store.conflicts().len(), 1, "op-a's clean first commit never raised a conflict of its own — only this one mixed-batch conflict must exist");
}
//#endregion 🔖️RobustnessTests

#[semio_framework_async_macros::async_test]
async fn composition_pins_rederive_checkpoint_identity_without_partial_mutation() {
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None)).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("checkpoint".into()), authors: Vec::new() }).await.expect("checkpoint");
    let original_checkpoint_id = store.envelope().vcs.checkpoints[0].id.clone();
    let before = owned_test_envelope(&store).await;
    let invalid = crate::os_vcs::CompositionPin { child_ref: crate::os_io::ArtifactRef { artifact_id: String::new(), dialect: demo_child_dialect() }, checkpoint_id: "child-checkpoint".into() };

    assert!(store.set_checkpoint_composition_pins(&original_checkpoint_id, vec![invalid]).await.is_err());
    assert_eq!(store.envelope(), &before);

    let pin = crate::os_vcs::CompositionPin { child_ref: crate::os_io::ArtifactRef { artifact_id: "child".into(), dialect: demo_child_dialect() }, checkpoint_id: "child-checkpoint".into() };
    store.set_checkpoint_composition_pins(&original_checkpoint_id, vec![pin]).await.expect("valid pin update");
    let rederived = &store.envelope().vcs.checkpoints[0];
    assert_ne!(rederived.id, original_checkpoint_id);
    assert_eq!(rederived.composition_pins.len(), 1);
    assert!(super::ArtifactStore::<DemoSnapshot, DemoMutation>::new(owned_test_envelope(&store).await).await.is_ok(), "a persisted pinned checkpoint must validate its rederived identity");
}

#[semio_framework_async_macros::async_test]
async fn materialize_replays_forward_mutations() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(1));
    assert_eq!(store.envelope().vcs.edits.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trip() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply");
    store.dispatch(ArtifactCommand::Undo).await.expect("undo");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(0));
    store.dispatch(ArtifactCommand::Redo).await.expect("redo");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(1));
}

//#region 🔖️HistoryLaneTests
#[semio_framework_async_macros::async_test]
async fn history_lane_defaults_to_document() {
    assert_eq!(HistoryLane::default(), HistoryLane::Document);
}

/// @emoji 🛤️ The design's headline acceptance case: undoing after an interleaved run of
/// document/interaction edits reverts the last DOCUMENT edit, skipping past trailing (and even
/// mid-history) `Interaction`-lane entries in both directions, which stay applied throughout.
#[semio_framework_async_macros::async_test]
async fn history_lane_default_undo_and_redo_skip_interaction_entries() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply doc1");
    let doc1_id = store.applied_edit_ids()[0].clone();
    store.dispatch(ArtifactCommand::ApplyInLane { mutations: vec![DemoMutation::SetN(SetN { n: 100 })], description: None, lane: HistoryLane::Interaction }).await.expect("apply interaction1");
    let interaction1_id = store.applied_edit_ids()[1].clone();
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], description: None }).await.expect("apply doc2");
    let doc2_id = store.applied_edit_ids()[2].clone();
    store.dispatch(ArtifactCommand::ApplyInLane { mutations: vec![DemoMutation::SetN(SetN { n: 200 })], description: None, lane: HistoryLane::Interaction }).await.expect("apply interaction2");
    let interaction2_id = store.applied_edit_ids()[3].clone();

    assert_eq!(store.envelope().lanes.get(&interaction1_id), Some(&HistoryLane::Interaction));
    assert_eq!(store.envelope().lanes.get(&interaction2_id), Some(&HistoryLane::Interaction));
    assert!(store.envelope().lanes.get(&doc1_id).is_none(), "an ordinary Document-lane edit never gets a `lanes` entry (sparse ledger)");
    assert!(store.envelope().lanes.get(&doc2_id).is_none());

    // Default undo skips the TRAILING interaction2 edit to revert doc2 instead.
    store.dispatch(ArtifactCommand::Undo).await.expect("undo skips interaction2 to revert doc2");
    assert_eq!(store.applied_edit_ids(), &[doc1_id.clone(), interaction1_id.clone(), interaction2_id.clone()], "doc2 removed; both interaction edits remain applied");
    assert_eq!(store.redo_edit_ids(), std::slice::from_ref(&doc2_id));

    // A second default undo reverts doc1 — the only remaining Document-lane entry — even though
    // it now sits BEFORE two still-applied interaction edits in `applied_edit_ids`.
    store.dispatch(ArtifactCommand::Undo).await.expect("undo doc1 despite interaction edits between it and the tail");
    assert_eq!(store.applied_edit_ids(), &[interaction1_id.clone(), interaction2_id.clone()]);
    assert_eq!(store.redo_edit_ids(), &[doc2_id.clone(), doc1_id.clone()]);

    // Default redo mirrors it: restores doc1 first (nearest Document entry in the redo stack),
    // then doc2, never touching either interaction edit's own applied/redo membership.
    store.dispatch(ArtifactCommand::Redo).await.expect("redo doc1");
    assert_eq!(store.applied_edit_ids(), &[interaction1_id.clone(), interaction2_id.clone(), doc1_id.clone()]);
    assert_eq!(store.redo_edit_ids(), std::slice::from_ref(&doc2_id));
    store.dispatch(ArtifactCommand::Redo).await.expect("redo doc2");
    assert_eq!(store.applied_edit_ids(), &[interaction1_id.clone(), interaction2_id.clone(), doc1_id.clone(), doc2_id.clone()]);
    assert!(store.redo_edit_ids().is_empty());
}

/// @emoji 🛤️ The completing half of the mechanism: `UndoInLane`/`RedoInLane` walk a NON-`Document`
/// lane explicitly and independently of the document lane's own cursor position.
#[semio_framework_async_macros::async_test]
async fn history_lane_undo_in_lane_and_redo_in_lane_walk_only_the_requested_lane() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply doc");
    store.dispatch(ArtifactCommand::ApplyInLane { mutations: vec![DemoMutation::SetN(SetN { n: 99 })], description: None, lane: HistoryLane::Interaction }).await.expect("apply interaction");
    let doc_id = store.applied_edit_ids()[0].clone();
    let interaction_id = store.applied_edit_ids()[1].clone();

    // Explicit lane-scoped undo reverts ONLY the interaction edit, leaving the document edit
    // applied — the mirror image of default `Undo` skipping it.
    store.dispatch(ArtifactCommand::UndoInLane { lane: HistoryLane::Interaction }).await.expect("undo in interaction lane");
    assert_eq!(store.applied_edit_ids(), std::slice::from_ref(&doc_id));
    assert_eq!(store.redo_edit_ids(), std::slice::from_ref(&interaction_id));
    assert_eq!(store.snapshot().expect("snapshot").n, Some(1), "reverting the interaction edit restores the document edit's own value");

    // Redoing the Document lane from here has nothing to redo — only the Interaction lane's
    // cursor moved, proving the two lanes' redo stacks are independent, not one shared position.
    assert_eq!(store.dispatch(ArtifactCommand::RedoInLane { lane: HistoryLane::Document }).await.unwrap_err(), VcsError::NothingToRedo);

    store.dispatch(ArtifactCommand::RedoInLane { lane: HistoryLane::Interaction }).await.expect("redo in interaction lane");
    assert_eq!(store.applied_edit_ids(), &[doc_id.clone(), interaction_id.clone()]);
    assert!(store.redo_edit_ids().is_empty());
}

/// @emoji 🛤️ Acceptance: a history made ENTIRELY of `Interaction`-lane edits is a no-op for
/// default `Undo` (no `Document`-lane entry exists at all), while the lane-scoped API still
/// reaches them.
#[semio_framework_async_macros::async_test]
async fn history_lane_default_undo_is_a_no_op_when_every_edit_is_interaction_lane() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::ApplyInLane { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None, lane: HistoryLane::Interaction }).await.expect("apply interaction1");
    store.dispatch(ArtifactCommand::ApplyInLane { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], description: None, lane: HistoryLane::Interaction }).await.expect("apply interaction2");
    assert_eq!(store.applied_edit_ids().len(), 2);

    let error = store.dispatch(ArtifactCommand::Undo).await.unwrap_err();
    assert_eq!(error, VcsError::NothingToUndo, "no Document-lane entry exists to undo; both interaction edits must stay untouched");
    assert_eq!(store.applied_edit_ids().len(), 2, "default undo must not remove either interaction edit");

    // The explicit lane-scoped API can still walk them.
    store.dispatch(ArtifactCommand::UndoInLane { lane: HistoryLane::Interaction }).await.expect("undo in interaction lane");
    assert_eq!(store.applied_edit_ids().len(), 1);
    assert_eq!(store.snapshot().expect("snapshot").n, Some(1));
}

/// @emoji 🛤️ `Interaction`-lane entries are ordinary persisted `Edit`s — they survive a plain
/// owned pack+SPR round trip, and a reloaded store's default undo still skips them.
#[semio_framework_async_macros::async_test]
async fn history_lane_interaction_entries_survive_owned_document_round_trip() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply doc");
    let doc_id = store.applied_edit_ids()[0].clone();
    store.dispatch(ArtifactCommand::ApplyInLane { mutations: vec![DemoMutation::SetN(SetN { n: 42 })], description: None, lane: HistoryLane::Interaction }).await.expect("apply interaction");
    let interaction_id = store.applied_edit_ids()[1].clone();
    assert_eq!(store.envelope().lanes.get(&interaction_id), Some(&HistoryLane::Interaction));

    let files = print_document_pack(store.envelope()).await.expect("owned document encode");
    let reloaded_envelope = parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr).await.expect("owned document decode").envelope;
    assert_eq!(reloaded_envelope.lanes.get(&interaction_id), Some(&HistoryLane::Interaction), "lane tag must survive the owned document round trip");

    let mut reloaded = ArtifactStore::new(reloaded_envelope).await;
    assert_eq!(reloaded.applied_edit_ids(), store.applied_edit_ids(), "reload seeds applied_edit_ids from the persisted cursor, same as any other edit");
    reloaded.dispatch(ArtifactCommand::Undo).await.expect("undo on the reloaded store still skips the interaction edit");
    assert_eq!(reloaded.applied_edit_ids(), std::slice::from_ref(&interaction_id), "the document edit was removed; the interaction edit is the only one left applied");
    assert!(reloaded.redo_edit_ids().contains(&doc_id), "the reverted document edit now sits on the redo stack");
}
//#endregion 🔖️HistoryLaneTests

//#region 🔖️InteractionStoreTests
/// @emoji 🕹️ `InteractionStore::apply` mutates the local hover value and bumps `generation`,
/// mirroring `PresenceStore`/`TransientStore` — the same `Mutation<S>::diff().apply()` seam,
/// reused here with the file's existing `DemoSnapshot`/`DemoMutation` fixtures standing in for
/// an app's hover-shaped type.
#[semio_framework_async_macros::async_test]
async fn interaction_store_apply_updates_hover_and_bumps_generation() {
    let mut store = InteractionStore::<DemoSnapshot, DemoMutation>::new(DemoSnapshot { n: Some(0) });
    assert_eq!(store.generation().await, 0);
    assert_eq!(store.hover().await.n, Some(0));

    store.apply(&[DemoMutation::SetN(SetN { n: 7 })]).await.expect("valid interaction mutation");
    assert_eq!(store.hover().await.n, Some(7), "apply routes through Mutation::diff/Diff::apply like PresenceStore");
    assert_eq!(store.generation().await, 1);

    store.apply(&[]).await.expect("empty interaction batch");
    assert_eq!(store.generation().await, 1, "an empty mutation batch must not bump generation, same as PresenceStore/TransientStore");
}

/// @emoji 🔄️ `reset` discards the current hover outright (a host clears hover when a
/// view/window closes) and still bumps `generation` so a pending broadcast reflects the clear.
#[semio_framework_async_macros::async_test]
async fn interaction_store_reset_discards_hover_and_bumps_generation() {
    let mut store = InteractionStore::<DemoSnapshot, DemoMutation>::new(DemoSnapshot { n: Some(0) });
    store.apply(&[DemoMutation::SetN(SetN { n: 3 })]).await.expect("valid interaction mutation");
    assert_eq!(store.generation().await, 1);

    store.reset(DemoSnapshot { n: Some(0) }).await;
    assert_eq!(store.hover().await.n, Some(0));
    assert_eq!(store.generation().await, 2, "reset bumps generation even though the value returns to default");
}

/// @emoji 🏗️ `Default` seeds from `S::default()`, same convention as `PresenceStore`/`TransientStore`.
#[semio_framework_async_macros::async_test]
async fn interaction_store_default_seeds_from_hover_default() {
    let store = InteractionStore::<DemoSnapshot, DemoMutation>::default();
    assert_eq!(store.hover().await.n, Some(0));
    assert_eq!(store.generation().await, 0);
}
//#endregion 🔖️InteractionStoreTests

//#region 🔖️ImmutableOperationRoots
#[semio_framework_async_macros::async_test]
async fn artifact_snapshot_root_is_o1_and_generation_stable_until_the_next_event() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    let generation = store.generation_now();
    let before = store.snapshot_root();
    let same = store.snapshot_root();
    assert!(Arc::ptr_eq(&before, &same), "capturing an immutable operation root must only retain the existing Arc");
    assert_eq!(generation, store.generation_now());

    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 7 })], description: None }).await.expect("apply");
    let after = store.snapshot_root();
    assert!(!Arc::ptr_eq(&before, &after), "a committed event publishes a new immutable root");
    assert_eq!(before.n, Some(0), "an already-admitted operation retains its exact pre-event snapshot");
    assert_eq!(after.n, Some(7));
    assert_eq!(store.content_revision_now(), store.content_revision().await);
}

#[semio_framework_async_macros::async_test]
async fn presence_local_read_is_o1_and_never_clones_the_payload_at_capture() {
    let mut store = PresenceStore::<DemoSnapshot, DemoMutation>::new(DemoSnapshot { n: Some(1) });
    let factory: Arc<dyn SnapshotRetirementFactory<DemoSnapshot>> = Arc::new(DemoSnapshotRetirementFactory);
    store.install_local_retirement_factory(factory.clone()).unwrap();
    let before = store.local_read().unwrap();
    assert!(std::ptr::eq(before.get(), store.local()));
    assert_eq!(store.generation_now(), 0);

    store.apply(&[DemoMutation::SetN(SetN { n: 2 })]).await.expect("presence apply");
    let after = store.local_read().unwrap();
    assert!(!std::ptr::eq(before.get(), after.get()));
    assert_eq!(before.n, Some(1));
    assert_eq!(after.n, Some(2));
    assert_eq!(store.generation_now(), 1);
    drop(before);
    drop(after);
    let mut close = store.begin_retirement(Arc::new(DemoSnapshot { n: Some(0) }), |value| value.n == Some(0)).ok().unwrap();
    for _ in 0..2048 {
        if close.close_step(1, 4096).unwrap() == SnapshotRetirementStep::Complete {
            break;
        }
    }
    assert!(close.terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn transient_root_is_o1_and_retains_the_exact_pre_reset_value() {
    let mut store = TransientStore::<DemoSnapshot, DemoMutation>::new(DemoSnapshot { n: Some(3) });
    let before = store.current_root();
    assert!(Arc::ptr_eq(&before, &store.current_root()));

    store.reset(DemoSnapshot { n: Some(4) }).await;
    let after = store.current_root();
    assert!(!Arc::ptr_eq(&before, &after));
    assert_eq!(before.n, Some(3));
    assert_eq!(after.n, Some(4));
    assert_eq!(store.generation_now(), 1);
}
//#endregion 🔖️ImmutableOperationRoots

#[semio_framework_async_macros::async_test]
async fn apply_computes_backwards_from_pre_state() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 5 })], description: None }).await.expect("apply");
    let edit = &store.envelope().vcs.edits[0];
    assert_eq!(edit.inverse, vec![DemoMutation::RestoreN(RestoreN { n: Some(0) })]);
}

#[semio_framework_async_macros::async_test]
async fn commit_checkpoint_wraps_edits_into_change() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("init".into()), authors: vec![Author { id: "a1".into(), name: "Alice".into(), avatar: None }] }).await.expect("commit");
    assert_eq!(store.envelope().vcs.changes.len(), 1);
    assert_eq!(store.envelope().vcs.checkpoints.len(), 1);
    assert_eq!(store.envelope().vcs.checkpoints[0].message, Some("init".into()));
}

#[semio_framework_async_macros::async_test]
async fn checkout_checkpoint_restores_applied_edits() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("c1".into()), authors: Vec::new() }).await.expect("commit");
    let checkpoint_id = store.envelope().vcs.checkpoints[0].id.clone();
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 9 })], description: None }).await.expect("apply2");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(9));
    store.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id }).await.expect("checkout");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(1));
}

#[semio_framework_async_macros::async_test]
async fn alternatives_switch_restores_checkpoint_chain() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply");
    store.dispatch(ArtifactCommand::CreateAlternative { name: "branch-a".into() }).await.expect("create alternative");
    let alt_id = store.envelope().vcs.alternatives[0].id.clone();
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], description: None }).await.expect("apply on branch");
    store.dispatch(ArtifactCommand::SwitchAlternative { alternative_id: alt_id }).await.expect("switch");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(1));
}

#[semio_framework_async_macros::async_test]
async fn checkout_old_checkpoint_then_commit_creates_a_fork() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("c1".into()), authors: Vec::new() }).await.expect("commit c1");
    let c1 = store.envelope().vcs.checkpoints[0].id.clone();
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], description: None }).await.expect("apply");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("c2".into()), authors: Vec::new() }).await.expect("commit c2");
    store.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: c1.clone() }).await.expect("checkout c1");
    assert_eq!(store.current_checkpoint_id().await, Some(c1.as_str()));
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 9 })], description: None }).await.expect("apply");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("fork".into()), authors: Vec::new() }).await.expect("commit fork");
    let children: Vec<&Checkpoint> = store.envelope().vcs.checkpoints.iter().filter(|checkpoint| checkpoint.parent_id.as_deref() == Some(c1.as_str())).collect();
    assert_eq!(children.len(), 2, "checking out an old checkpoint before committing must fork, not extend the trunk");
}

#[semio_framework_async_macros::async_test]
async fn create_alternative_appends_commits_to_its_own_checkpoint_chain() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("root".into()), authors: Vec::new() }).await.expect("commit root");
    store.dispatch(ArtifactCommand::CreateAlternative { name: "feature-a".into() }).await.expect("create alternative");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], description: None }).await.expect("apply");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("branch commit".into()), authors: Vec::new() }).await.expect("commit on branch");
    assert_eq!(store.envelope().vcs.alternatives[0].checkpoint_ids.len(), 2);
    assert_eq!(store.envelope().vcs.checkpoints.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn history_columns_orders_newest_first_and_labels_trunk_root() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("c1".into()), authors: Vec::new() }).await.expect("commit c1");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], description: None }).await.expect("apply");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("c2".into()), authors: Vec::new() }).await.expect("commit c2");
    let columns = store.history_columns().await;
    assert_eq!(columns.len(), 2);
    assert_eq!(columns[0].description, Some("c2".into()), "newest checkpoint must be first");
    assert_eq!(columns[0].lane, 0);
    assert_eq!(columns[0].labels, vec!["main".to_string()], "newest unlabeled row falls back to main");
    assert!(columns[1].labels.is_empty(), "only the newest row gets the main fallback");
    let json = serde_json::to_string(&test_support::SerdeValue(&columns[0].to_value())).expect("serialize");
    assert!(json.contains("checkpointId"), "wire format must be camelCase: {json}");
}

#[semio_framework_async_macros::async_test]
async fn history_columns_assigns_distinct_lanes_and_pulls_main_only_descendants_to_trunk() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("root".into()), authors: Vec::new() }).await.expect("commit root");
    let root = store.envelope().vcs.checkpoints[0].id.clone();

    store.dispatch(ArtifactCommand::CreateAlternative { name: "feature-a".into() }).await.expect("create feature-a");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], description: None }).await.expect("apply");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("a1".into()), authors: Vec::new() }).await.expect("commit a1");

    store.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: root.clone() }).await.expect("checkout root");
    store.dispatch(ArtifactCommand::CreateAlternative { name: "feature-b".into() }).await.expect("create feature-b");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 3 })], description: None }).await.expect("apply");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("b1".into()), authors: Vec::new() }).await.expect("commit b1");

    store.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: root.clone() }).await.expect("checkout root again");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 4 })], description: None }).await.expect("apply");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("main resumed".into()), authors: Vec::new() }).await.expect("commit main resumed");

    let columns = store.history_columns().await;
    assert_eq!(columns.len(), 4, "root + a1 + b1 + main-resumed");
    let by_message: HashMap<String, &HistoryColumn> = columns.iter().filter_map(|column| column.description.clone().map(|description| (description, column))).collect();
    assert_eq!(by_message["root"].lane, 0, "root has no parent, lane 0");
    assert_eq!(by_message["main resumed"].lane, 0, "commit with no alternative stays on the trunk");
    let a_lane = by_message["a1"].lane;
    let b_lane = by_message["b1"].lane;
    assert_ne!(a_lane, 0, "a1 belongs to an alternative, not the trunk");
    assert_ne!(b_lane, 0, "b1 belongs to an alternative, not the trunk");
    assert_ne!(a_lane, b_lane, "distinct alternatives must get distinct swimlanes");

    let root_children: Vec<&HistoryColumn> = columns.iter().filter(|column| column.parent_checkpoint_id.as_deref() == Some(root.as_str())).collect();
    assert_eq!(root_children.len(), 3, "root forked three ways: a1, b1, main-resumed");
}

#[semio_framework_async_macros::async_test]
async fn backbone_message_binary_round_trips_every_variant() {
    let snapshot = BackboneMessage::Snapshot { pack: vec![1, 2, 3], spr: Vec::new() };
    assert_eq!(BackboneMessage::decode_op(&snapshot.encode_op().unwrap()).unwrap(), snapshot);

    let envelope = sample_envelope_for_backbone_test().await;
    let operations = BackboneMessage::Mutations { envelopes: crate::os_spr::encode_envelopes(&[envelope.clone(), envelope]) };
    assert_eq!(BackboneMessage::decode_op(&operations.encode_op().unwrap()).unwrap(), operations);

    let ack = BackboneMessage::Ack { op_ids: vec!["op-1".to_string(), "op-2".to_string()] };
    assert_eq!(BackboneMessage::decode_op(&ack.encode_op().unwrap()).unwrap(), ack);

    let empty_ack = BackboneMessage::Ack { op_ids: Vec::new() };
    assert_eq!(BackboneMessage::decode_op(&empty_ack.encode_op().unwrap()).unwrap(), empty_ack);
}

async fn sample_envelope_for_backbone_test() -> crate::os_spr::MutationEnvelope {
    crate::os_spr::MutationEnvelope {
        mutation_id: MutationId("op-1".to_string()),
        document_id: ArtifactId("doc-1".to_string()),
        actor: ActorId("actor-1".to_string()),
        dependencies: Vec::new(),
        diff: crate::os_spr::ArtifactDiff { schema: SchemaId("demo/v1".to_string()), payload: vec![1, 2, 3] },
        inverse: crate::os_spr::InverseMutation { schema: SchemaId("demo/v1".to_string()), payload: Vec::new() },
        timestamp: HybridLogicalTimestamp { actor: 0, physical_ms: 0, logical: 0 },
    }
}

#[semio_framework_async_macros::async_test]
async fn no_backbone_by_default() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    assert!(envelope.backbone.is_none(), "a fresh document has no attached backbone");
    let store = ArtifactStore::new(envelope).await;
    assert!(store.backbone_ref().is_none());
}

#[semio_framework_async_macros::async_test]
async fn memory_backbone_pair_propagates_edits_bidirectionally() {
    let (backbone_a, backbone_b) = MemoryBackbone::pair("peer-a", "peer-b").await;
    let envelope_a: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let envelope_b: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store_a = ArtifactStore::new(envelope_a).await;
    let mut store_b = ArtifactStore::new(envelope_b).await;
    store_a.attach_backbone(Backbones::Memory(backbone_a)).await.expect("attach a");
    store_b.attach_backbone(Backbones::Memory(backbone_b)).await.expect("attach b");

    store_a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply on a");
    store_b.tick().await.expect("tick b");
    assert_eq!(store_b.snapshot().expect("snapshot b").n, Some(1), "b receives a's edit");

    store_b.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], description: None }).await.expect("apply on b");
    store_a.tick().await.expect("tick a");
    assert_eq!(store_a.snapshot().expect("snapshot a").n, Some(2), "a receives b's edit");
}

#[semio_framework_async_macros::async_test]
async fn backbone_retirement_blocks_for_live_peer_then_drains_one_owned_message_or_byte_grant() {
    let (mut local, peer) = MemoryBackbone::pair("close-local", "close-peer").await;
    local.send(BackboneMessage::Snapshot { pack: vec![1; 32 * 1024], spr: vec![2; 32 * 1024] }).await.expect("seed deep close queue");
    let mut retirement = ArtifactStoreBackboneRetirement::new(Backbones::Memory(local));
    let mut observed_blocked = false;
    for _ in 0..32 {
        match retirement.close_step(1, 17).expect("bounded pre-peer close") {
            SnapshotRetirementStep::Blocked => {
                observed_blocked = true;
                break;
            }
            SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= 17),
            SnapshotRetirementStep::Complete => panic!("a live peer still retains both exact queue authorities"),
        }
    }
    assert!(observed_blocked);
    drop(peer);
    let mut turns = 0;
    while !retirement.terminal_is_empty() {
        match retirement.close_step(1, 17).expect("bounded post-peer close") {
            SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= 17),
            SnapshotRetirementStep::Blocked => panic!("no external queue owner remains after peer close"),
            SnapshotRetirementStep::Complete => assert!(retirement.terminal_is_empty()),
        }
        turns += 1;
        assert!(turns < 100_000, "backbone queue and payload retirement terminates across interruptions");
    }
}

#[semio_framework_async_macros::async_test]
async fn detach_backbone_stops_synchronizing_but_keeps_the_wip_graph() {
    let (backbone_a, backbone_b) = MemoryBackbone::pair("peer-a", "peer-b").await;
    let mut store_a = ArtifactStore::new(create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None)).await;
    let mut store_b: ArtifactStore<DemoSnapshot, DemoMutation> = ArtifactStore::new(create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None)).await;
    store_a.attach_backbone(Backbones::Memory(backbone_a)).await.expect("attach a");
    store_b.attach_backbone(Backbones::Memory(backbone_b)).await.expect("attach b");
    store_a.detach_backbone().expect("detach source backbone");
    assert!(store_a.backbone_ref().is_none());

    store_a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 9 })], description: None }).await.expect("apply after detach still works on the in-memory graph");
    assert_eq!(store_a.snapshot().expect("snapshot a").n, Some(9));
    store_b.tick().await.expect("tick b");
    assert_eq!(store_b.snapshot().expect("snapshot b").n, Some(0), "detached edits never reach the peer");
}

#[semio_framework_async_macros::async_test]
async fn loaded_envelope_with_stale_backbone_ref_never_auto_attaches() {
    let stale = || create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, Some(ArtifactBackboneRef { uri: "folder:///nonexistent/path".into() }));
    let mut store = ArtifactStore::new(stale()).await;
    assert!(!store.tick().await.expect("tick with no live backbone is a no-operation"), "no backbone was ever attached, so there is nothing to pump");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply works purely against the in-memory graph");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(1));

    store.reset(stale(), Vec::new(), Vec::new()).await.expect("reset");
    assert!(!store.tick().await.expect("tick after set_state with no live backbone is a no-operation"), "set_state must not resurrect IO from a stale backbone descriptor either");
}

#[semio_framework_async_macros::async_test]
async fn document_codec_of_round_trips_dsl_and_pack_and_edit_text() {
    let codec = ArtifactCodec::of::<DemoSnapshot, DemoMutation>("test.document-codec-roundtrip/v1");
    assert_eq!(codec.schema, "test.document-codec-roundtrip/v1");
    assert_eq!(codec.extension, "demo");

    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("test.document-codec-roundtrip/v1", "demo", DemoSnapshot { n: Some(4) }, None);
    let text_files = print_document_text(&envelope).await;
    drop(envelope.into_owners());
    let text_files = text_files.expect("print document text");

    let (pack_files, dsl_mirror) = (codec.compile_dsl)(&text_files.dsl, &text_files.ops).await.expect("codec compile_dsl");
    assert_eq!(dsl_mirror, DemoSnapshot { n: Some(4) }.print_dsl(), "dsl mirror matches the initial snapshot's print_dsl");

    let mirrored = (codec.print_mirror)(&pack_files.pack, &pack_files.spr).await.expect("codec print_mirror");
    assert_eq!(mirrored.dsl, dsl_mirror, "print_mirror's dsl text agrees with compile_dsl's own mirror, no JSON round trip");

    let document_id = ArtifactId("demo".to_string());
    let schema = SchemaId("test.document-codec-roundtrip/v1".to_string());
    let edit = Edit {
        id: "edit-1".into(),
        actor: Some("peer".into()),
        forwards: vec![DemoMutation::SetN(SetN { n: 9 })],
        inverse: vec![DemoMutation::SetN(SetN { n: 4 })],
        mutation_meta: Vec::new(),
        description: None,
        coalesce_key: None,
        sequence_number: 1,
        started_at: "0".into(),
        finished_at: None,
    };
    let mut op_envelopes = crate::os_spr::mutation_envelope_from_edit::<DemoSnapshot, DemoMutation>(&edit, &document_id, &schema).expect("op envelopes");
    let op_envelope = op_envelopes.pop().expect("exactly one op envelope for a single-op edit");
    let edit_text = (codec.edit_text_from_envelope)(&op_envelope).await.expect("codec edit_text_from_envelope");
    assert!(edit_text.contains("set-n"), "edit text contains the printed op line: {edit_text:?}");
    assert!(!edit_text.contains('\n') || edit_text.trim_end_matches('\n').lines().count() <= 2, "one header line + one op line: {edit_text:?}");

    preflight_document_codecs(std::slice::from_ref(&codec)).await.expect("preflight accepts an unclaimed full descriptor without publishing it");
    assert!(document_codec("test.document-codec-roundtrip/v1").await.expect("registry availability").is_none(), "preflight must not publish a codec");
    register_document_codec(codec).expect("first document codec registration");
    assert!(document_codec("test.document-codec-roundtrip/v1").await.expect("registry availability").is_some(), "registered codec is discoverable by schema string");
    assert!(document_codec("no-such-schema").await.expect("registry availability").is_none());
}

#[semio_framework_async_macros::async_test]
async fn register_document_codec_rejects_a_duplicate_schema_without_replacing_the_first() {
    let first = ArtifactCodec::of::<DemoSnapshot, DemoMutation>("test.duplicate-id-probe/v1");
    let second = ArtifactCodec { pack_schema_hash: [7u8; 32], ..first.clone() };
    assert_ne!(first.pack_schema_hash, second.pack_schema_hash, "fixture precondition: the two codecs must be distinguishable");

    register_document_codec(first.clone()).expect("first registration");
    register_document_codec(first.clone()).expect("an identical descriptor and executable is idempotent");
    let conflict = match register_document_codec(second).expect_err("a schema collision must reject rather than replace") {
        DocumentCodecRegistryError::Conflict(conflict) => conflict,
        DocumentCodecRegistryError::Unavailable => panic!("document codec registry unavailable"),
    };
    assert_eq!(conflict.schema, "test.duplicate-id-probe/v1");
    let resolved = document_codec("test.duplicate-id-probe/v1").await.expect("registry availability").expect("still registered after the second call");
    assert_eq!(resolved.pack_schema_hash, first.pack_schema_hash, "the first codec remains authoritative after a conflict");
}

#[semio_framework_async_macros::async_test]
async fn dialect_migration_preflight_and_batch_commit_are_conflict_free_or_noop() {
    // 🚫️async: E4 fn-pointer slot — `DialectMigration.migrate_pack` is `fn(&[u8]) ->
    // Result<Vec<u8>, String>` (unnameable if async) — see R9/E4.
    fn append_marker(bytes: &[u8]) -> Result<Vec<u8>, String> {
        Ok([bytes, b"-migrated"].concat())
    }

    let from = crate::os_io::ArtifactDialect { artifact_kind: "test.runtime-migration".into(), standard: "1".into(), subset: "*".into() };
    let to = crate::os_io::ArtifactDialect { artifact_kind: "test.runtime-migration".into(), standard: "2".into(), subset: "*".into() };
    let migration = DialectMigration { from: from.clone(), to: to.clone(), lossless: true, migrate_pack: append_marker };
    preflight_dialect_migrations(std::slice::from_ref(&migration)).await.expect("preflight accepts an unclaimed dialect pair without mutation");
    assert!(matches!(migrate_document(&from, &to, b"seed").await, Err(DialectMigrationError::Missing { .. })), "preflight must not publish a migration");
    register_dialect_migrations(vec![migration.clone()]).expect("batch migration registration");
    assert_eq!(migrate_document(&from, &to, b"seed").await.expect("registered migration"), b"seed-migrated");

    let conflict = DialectMigration { lossless: false, ..migration };
    assert!(matches!(preflight_dialect_migrations(std::slice::from_ref(&conflict)).await, Err(DialectMigrationRegistryError::Conflict { .. })), "preflight must reject a descriptor change before batch commit");
    assert_eq!(migrate_document(&from, &to, b"seed").await.expect("first migration remains authoritative"), b"seed-migrated");
}

async fn projection_probe(store: &ArtifactStore<DemoSnapshot, DemoMutation>, cause: ArtifactProjectionCause) -> ArtifactProjectionResult<Option<i32>, DemoDiff> {
    let event = store.projection_event(cause, Some(DemoSnapshot { n: Some(-1) }), ArtifactProjectionCacheMode::ValidatePrevious, "deterministic").expect("capture projection event");
    assert_eq!(event.previous, Some(DemoSnapshot { n: Some(-1) }));
    assert_eq!(event.cache_mode, ArtifactProjectionCacheMode::ValidatePrevious);
    event.result(event.state.n, None).await
}

async fn assert_projection_is_stale(store: &ArtifactStore<DemoSnapshot, DemoMutation>, result: ArtifactProjectionResult<Option<i32>, DemoDiff>) {
    assert!(matches!(store.accept_projection_result(result), Err(StaleArtifactProjection { .. })), "a result for an older projection stamp must be rejected");
}

#[semio_framework_async_macros::async_test]
async fn projection_result_gate_rejects_results_after_every_invalidating_store_transition() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "projection", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;

    let before_apply = projection_probe(&store, ArtifactProjectionCause::Apply).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply");
    assert_projection_is_stale(&store, before_apply).await;

    let before_undo = projection_probe(&store, ArtifactProjectionCause::Undo).await;
    store.dispatch(ArtifactCommand::Undo).await.expect("undo");
    assert_projection_is_stale(&store, before_undo).await;

    let before_redo = projection_probe(&store, ArtifactProjectionCause::Redo).await;
    store.dispatch(ArtifactCommand::Redo).await.expect("redo");
    assert_projection_is_stale(&store, before_redo).await;

    let before_remote = projection_probe(&store, ArtifactProjectionCause::RemoteIngest).await;
    store.dispatch(ArtifactCommand::IngestRemote { envelope: foreign_mutation_envelope("projection-peer", DemoMutation::SetN(SetN { n: 2 })).await }).await.expect("remote ingest");
    assert_projection_is_stale(&store, before_remote).await;

    let before_reset = projection_probe(&store, ArtifactProjectionCause::Reset).await;
    let reset_envelope = owned_test_envelope(&store).await;
    let reset_applied = store.applied_edit_ids().to_vec();
    let reset_redo = store.redo_edit_ids().to_vec();
    store.reset(reset_envelope, reset_applied, reset_redo).await.expect("reset");
    assert_projection_is_stale(&store, before_reset).await;

    store.dispatch(ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() }).await.expect("checkpoint");
    let checkpoint_id = store.current_checkpoint_id().await.expect("checkpoint id").to_string();
    let before_checkout = projection_probe(&store, ArtifactProjectionCause::Checkout).await;
    store.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id }).await.expect("checkout");
    assert_projection_is_stale(&store, before_checkout).await;
}

#[semio_framework_async_macros::async_test]
async fn projection_result_gate_rejects_results_after_dependency_and_checkpoint_transitions() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "projection-dependencies", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;

    let before_replay = projection_probe(&store, ArtifactProjectionCause::Replay).await;
    assert_eq!(store.invalidate_after_replay().expect("replay invalidation").cause, ArtifactProjectionCause::Replay);
    assert_projection_is_stale(&store, before_replay).await;

    let before_policy = projection_probe(&store, ArtifactProjectionCause::PolicyChange).await;
    assert_eq!(store.invalidate_after_policy_change().expect("policy invalidation").cause, ArtifactProjectionCause::PolicyChange);
    assert_projection_is_stale(&store, before_policy).await;

    let before_resource = projection_probe(&store, ArtifactProjectionCause::ExternalResourceChange).await;
    assert_eq!(store.invalidate_after_external_resource_change().expect("resource invalidation").cause, ArtifactProjectionCause::ExternalResourceChange);
    assert_projection_is_stale(&store, before_resource).await;

    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply before checkpoint");
    let before_checkpoint = projection_probe(&store, ArtifactProjectionCause::Checkpoint).await;
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() }).await.expect("non-empty checkpoint");
    assert_projection_is_stale(&store, before_checkpoint).await;
    assert_eq!(store.last_projection_invalidation().expect("checkpoint invalidation").cause, ArtifactProjectionCause::Checkpoint);

    let generation = store.generation();
    let error = store.dispatch(ArtifactCommand::PruneDrafts).await.expect_err("draft pruning is explicitly unavailable");
    assert!(matches!(error, VcsError::ValidationFailed(_)));
    assert_eq!(store.generation(), generation, "a rejected prune cannot invalidate or report a success");
}

#[semio_framework_async_macros::async_test]
// 🎞️ The final paragraph this test used to have (`ValidatedMutation`/`Mutation::validate`
// rejecting `n < 0` before persisting) is gone — `Mutation::validate` is deleted (§C4/C10); the
// real outcome-messages/`MergePolicy` rejection this replaces is lane 1-A's C6 work.
async fn reset_and_apply_reject_malformed_history_before_persisting() {
    let fresh = || create_document_envelope("demo/v1", "reset-invalid", DemoSnapshot { n: Some(0) }, None);
    let mut malformed_constructor = fresh();
    malformed_constructor.cursor = Some(ArtifactCursor::new(vec!["missing".into()], Vec::new(), None));
    assert!(matches!(super::ArtifactStore::new(malformed_constructor).await, Err(VcsError::UnknownEdit(id)) if id == "missing"), "construction must reject malformed cursor history before any mutation applies");

    let mut legacy_seed = super::ArtifactStore::new(fresh()).await.expect("valid seed history");
    legacy_seed.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 3 })], description: None }).await.expect("seed edit");
    let files = print_document_pack(legacy_seed.envelope()).await.expect("owned history encode");
    let mut cursorless = parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr).await.expect("owned history decode").envelope;
    cursorless.cursor = None;
    assert_eq!(super::ArtifactStore::new(cursorless).await.expect("cursorless authoritative history is validated and folded").snapshot().expect("cursorless snapshot"), DemoSnapshot { n: Some(3) });
    let mut duplicate_history = parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr).await.expect("second owned history decode").envelope;
    duplicate_history.cursor = None;
    let duplicate_edit = duplicate_history.vcs.edits[0].clone();
    duplicate_history.vcs.edits.try_push(duplicate_edit).expect("duplicate test edit fits the fixed history ledger");
    assert!(matches!(super::ArtifactStore::new(duplicate_history).await, Err(VcsError::ValidationFailed(message)) if message.contains("repeats authoritative edit")), "duplicate authoritative edits cannot be hidden by first-match replay");

    let mut store = ArtifactStore::new(fresh()).await;
    let generation = store.generation();
    assert!(matches!(store.reset(fresh(), vec!["missing".into()], Vec::new()).await, Err(VcsError::UnknownEdit(id)) if id == "missing"));
    assert_eq!(store.generation(), generation, "failed reset must preserve the live store");
}

#[semio_framework_async_macros::async_test]
async fn attach_reconciles_a_pushed_snapshot() {
    let (channel, remote) = ChannelBackbone::pair("chan").await;
    let seeded: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut seed_store = ArtifactStore::new(seeded).await;
    seed_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 5 })], description: None }).await.expect("apply");
    let seed_files = seed_store.snapshot_pack().await.expect("seed snapshot");
    remote.push(BackboneMessage::Snapshot { pack: seed_files.pack, spr: seed_files.spr }).await.expect("push snapshot");

    let fresh: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(fresh).await;
    store.attach_backbone(Backbones::Channel(channel)).await.expect("attach reconciles the pushed snapshot");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(5), "adopted the pushed snapshot's edit");
}

#[semio_framework_async_macros::async_test]
async fn channel_backbone_round_trips_between_store_and_actor() {
    let (channel, remote) = ChannelBackbone::pair("chan").await;
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.attach_backbone(Backbones::Channel(channel)).await.expect("attach");
    let attach_flush = drain_channel_for_test(&remote).expect("drain attach");
    assert!(attach_flush.iter().any(|message| matches!(message, BackboneMessage::Snapshot { .. })), "attach flushes a snapshot to the actor end: {attach_flush:?}");

    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 4 })], description: None }).await.expect("apply");
    let outbound = drain_channel_for_test(&remote).expect("drain apply");
    assert!(outbound.iter().any(|message| matches!(message, BackboneMessage::Mutations { .. })), "a local apply is sent outbound as mutations: {outbound:?}");

    remote.push(BackboneMessage::Mutations { envelopes: crate::os_spr::encode_envelopes(&[foreign_mutation_envelope("peer", DemoMutation::SetN(SetN { n: 8 })).await]) }).await.expect("push inbound operations");
    store.tick().await.expect("tick");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(8), "store ingests the actor's inbound operations");
}

#[semio_framework_async_macros::async_test]
async fn channel_backbone_remote_pops_one_exact_owner_in_fifo_order_and_preserves_contention() {
    let (mut channel, remote) = ChannelBackbone::pair("chan").await;
    channel.send(BackboneMessage::Ack { op_ids: vec!["first".into()] }).await.expect("queue first exact owner");
    channel.send(BackboneMessage::Ack { op_ids: vec!["second".into()] }).await.expect("queue second exact owner");
    let guard = remote.outbound.lock().expect("hold outbound authority for contention fixture");
    assert!(matches!(remote.try_pop_front(), Err(VcsError::Backbone(message)) if message.contains("contended")));
    assert_eq!(guard.len(), 2, "contention leaves both exact owners untouched");
    drop(guard);
    assert!(matches!(remote.try_pop_front().expect("first bounded pop"), Some(BackboneMessage::Ack { op_ids }) if op_ids == vec!["first"]));
    assert!(matches!(remote.try_pop_front().expect("second bounded pop"), Some(BackboneMessage::Ack { op_ids }) if op_ids == vec!["second"]));
    assert!(remote.try_pop_front().expect("terminal bounded pop").is_none());
}

#[semio_framework_async_macros::async_test]
async fn pump_acks_ingested_operations() {
    let (channel, remote) = ChannelBackbone::pair("chan").await;
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.attach_backbone(Backbones::Channel(channel)).await.expect("attach");
    let _ = drain_channel_for_test(&remote).expect("drain attach snapshot");

    let inbound = foreign_mutation_envelope("peer", DemoMutation::SetN(SetN { n: 7 })).await;
    let mutation_id = inbound.mutation_id.0.clone();
    remote.push(BackboneMessage::Mutations { envelopes: crate::os_spr::encode_envelopes(&[inbound]) }).await.expect("push inbound operations");
    store.tick().await.expect("tick");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(7), "ingested the inbound operation");

    let outbound = drain_channel_for_test(&remote).expect("drain ack");
    assert!(outbound.iter().any(|message| matches!(message, BackboneMessage::Ack { op_ids } if op_ids == &vec![mutation_id.clone()])), "successful operations ingest emits an Ack for the ingested operation ids: {outbound:?}");
}

#[semio_framework_async_macros::async_test]
async fn exact_base_only_undo_refuses_a_foreign_tail() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("local apply");
    store.dispatch(ArtifactCommand::IngestRemote { envelope: foreign_mutation_envelope("peer", DemoMutation::SetN(SetN { n: 2 })).await }).await.expect("ingest foreign");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(2), "foreign edit sits at the tail");

    let error = store.dispatch(ArtifactCommand::UndoWithPolicy { policy: UndoPolicy::ExactBaseOnly, semantic_command: None }).await.expect_err("undo must refuse a foreign tail");
    assert!(matches!(error, VcsError::ForeignEdit(_)), "got {error:?}");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(2), "the timeline is untouched after refusal");
}

#[semio_framework_async_macros::async_test]
async fn transform_against_concurrent_undo_skips_over_a_foreign_tail() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("local apply");
    let local_edit_id = store.applied_edit_ids()[0].clone();
    let foreign = foreign_mutation_envelope("peer", DemoMutation::SetN(SetN { n: 2 })).await;
    let foreign_id = foreign.mutation_id.0.clone();
    store.dispatch(ArtifactCommand::IngestRemote { envelope: foreign }).await.expect("ingest foreign");
    assert_eq!(store.applied_edit_ids().len(), 2, "local + foreign are both applied");

    store.dispatch(ArtifactCommand::UndoWithPolicy { policy: UndoPolicy::TransformAgainstConcurrent, semantic_command: None }).await.expect("transform undo removes the local edit from mid-timeline");
    assert_eq!(store.applied_edit_ids(), std::slice::from_ref(&foreign_id), "only the local edit is removed; the concurrent foreign edit stays applied");
    assert_eq!(store.redo_edit_ids(), std::slice::from_ref(&local_edit_id), "the local edit is on the redo stack");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(2), "snapshot re-materializes from the foreign edit alone");

    store.dispatch(ArtifactCommand::Redo).await.expect("redo brings the local edit back");
    assert_eq!(store.applied_edit_ids().len(), 2);
    assert_eq!(store.snapshot().expect("snapshot").n, Some(1), "redo re-applies the local edit at the tail");
}

#[semio_framework_async_macros::async_test]
async fn compensating_undo_dispatches_semantic_command() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 5 })], description: None }).await.expect("apply");
    let undo_apply = ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 0 })], description: Some("compensate".into()) };
    store.dispatch(ArtifactCommand::UndoWithPolicy { policy: UndoPolicy::CompensatingAction, semantic_command: Some(Box::new(undo_apply)) }).await.expect("compensating undo");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(0));
}

#[semio_framework_async_macros::async_test]
async fn edit_mutations_exposes_the_latest_edit() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    assert!(store.edit_mutations().is_none(), "no edits yet");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 5 })], description: None }).await.expect("apply");
    let (forwards, inverse, meta) = store.edit_mutations().expect("edit operations");
    assert_eq!(forwards, &[DemoMutation::SetN(SetN { n: 5 })]);
    assert_eq!(inverse, &[DemoMutation::RestoreN(RestoreN { n: Some(0) })], "inverse restores the pre-state");
    assert_eq!(meta.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn amend_last_absorbs_into_matching_coalesce_key() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::AmendLast { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], coalesce_key: Some("drag".into()) }).await.expect("first amend");
    store.dispatch(ArtifactCommand::AmendLast { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], coalesce_key: Some("drag".into()) }).await.expect("second amend");
    assert_eq!(store.envelope().vcs.edits.len(), 1, "coalesced into a single edit");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(2));
    store.dispatch(ArtifactCommand::Undo).await.expect("undo");
    assert_eq!(store.snapshot().expect("snapshot after undo").n, Some(0), "undo restores pre-gesture state in one step");
}

#[semio_framework_async_macros::async_test]
async fn amend_last_incremental_path_matches_full_replay_over_many_amends() {
    // 🪢️ Regression guard for the incremental `AmendLast` path (see `AmendCache`): many sequential
    // amends into the same coalesced edit — e.g. a long slider drag — must still produce exactly the
    // same edit (forwards/inverse/mutation_meta length, final snapshot, one-step undo) as the
    // previous full-replay-every-time implementation, just without re-replaying history each time.
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    for n in 1..=50 {
        store.dispatch(ArtifactCommand::AmendLast { mutations: vec![DemoMutation::SetN(SetN { n })], coalesce_key: Some("drag".into()) }).await.expect("amend");
    }
    assert_eq!(store.envelope().vcs.edits.len(), 1, "still a single coalesced edit");
    let edit = store.envelope().vcs.edits.last().expect("edit");
    assert_eq!(edit.forwards.len(), 50);
    assert_eq!(edit.inverse.len(), 50);
    assert_eq!(edit.mutation_meta.len(), 50);
    assert_eq!(store.snapshot().expect("snapshot").n, Some(50));
    store.dispatch(ArtifactCommand::Undo).await.expect("undo");
    assert_eq!(store.snapshot().expect("snapshot after undo").n, Some(0), "one undo reverts the whole 50-step coalesced gesture");
}

#[semio_framework_async_macros::async_test]
async fn amend_last_incremental_cache_survives_undo_redo_round_trip() {
    // 🪢️ Undo/redo only move edit ids between `applied_edit_ids`/`redo_edit_ids` — they never mutate
    // an edit's own `forwards`, so a cached post-snapshot keyed by `(edit_id, forwards_len)` stays
    // valid across an undo immediately followed by a redo of the very same coalesced edit.
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::AmendLast { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], coalesce_key: Some("drag".into()) }).await.expect("first amend");
    store.dispatch(ArtifactCommand::Undo).await.expect("undo");
    store.dispatch(ArtifactCommand::Redo).await.expect("redo");
    store.dispatch(ArtifactCommand::AmendLast { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], coalesce_key: Some("drag".into()) }).await.expect("amend after undo/redo");
    assert_eq!(store.envelope().vcs.edits.len(), 1, "still coalesced into the original edit");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(2));
    store.dispatch(ArtifactCommand::Undo).await.expect("undo again");
    assert_eq!(store.snapshot().expect("snapshot after undo").n, Some(0));
}

#[semio_framework_async_macros::async_test]
async fn amend_last_starts_new_edit_when_coalesce_key_differs() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::AmendLast { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], coalesce_key: Some("drag-a".into()) }).await.expect("first drag");
    store.dispatch(ArtifactCommand::AmendLast { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], coalesce_key: Some("drag-b".into()) }).await.expect("second drag");
    assert_eq!(store.envelope().vcs.edits.len(), 2, "distinct gestures are separate edits");
}

#[semio_framework_async_macros::async_test]
async fn amend_last_does_not_absorb_into_committed_edit() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::AmendLast { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], coalesce_key: Some("drag".into()) }).await.expect("amend");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() }).await.expect("commit");
    store.dispatch(ArtifactCommand::AmendLast { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], coalesce_key: Some("drag".into()) }).await.expect("amend after commit");
    assert_eq!(store.envelope().vcs.edits.len(), 2, "committed edits are never amended, even with a matching coalesce key");
}

#[semio_framework_async_macros::async_test]
async fn assert_subset_harness_fidelity_and_inference_helpers() {
    test_support::assert_import_export_fidelity_bytes(b"fixture", b"fixture", test_support::IoFidelityClass::Exact).await;
    test_support::assert_import_export_fidelity_bytes(b"fixture", b"other", test_support::IoFidelityClass::Canonical).await;
    test_support::assert_inference_determinism(&7_i32, &7_i32).await;
}

#[semio_framework_async_macros::async_test]
#[should_panic(expected = "exact fidelity requires byte-identical export")]
async fn assert_import_export_fidelity_bytes_exact_rejects_divergence() {
    test_support::assert_import_export_fidelity_bytes(b"fixture", b"other", test_support::IoFidelityClass::Exact).await;
}

#[semio_framework_async_macros::async_test]
async fn test_support_round_trip_helpers_pass_for_demo_operation() {
    test_support::assert_operation_round_trip(&DemoSnapshot { n: Some(4) }, DemoMutation::SetN(SetN { n: 9 })).await;
    test_support::assert_store_roundtrip(DemoSnapshot { n: Some(4) }, DemoMutation::SetN(SetN { n: 9 })).await;

    let edit = Edit::<DemoMutation> {
        id: "edit-command-envelope".into(),
        actor: Some("actor-fallback".into()),
        forwards: vec![DemoMutation::SetN(SetN { n: 9 })],
        inverse: vec![DemoMutation::SetN(SetN { n: 4 })],
        mutation_meta: vec![MutationMeta {
            mutation_id: Some(MutationId("op-a".into())),
            dependencies: vec![MutationId("op-0".into())],
            base_version: 0,
            author_id: Some(ActorId("actor-explicit".into())),
            timestamp: HybridLogicalTimestamp::new(1, 1000),
            undo_policy: UndoPolicy::ExactBaseOnly,
            payload_hash: None,
            semantic_kind: None,
            label: None,
            group_id: None,
            origin: Default::default(),
        }],
        description: None,
        coalesce_key: None,
        sequence_number: 1,
        started_at: "2026-07-27T00:00:00Z".into(),
        finished_at: None,
    };
    test_support::assert_command_envelope_round_trip::<DemoSnapshot, DemoMutation>(&edit, &ArtifactId("doc-command-envelope".into()), &SchemaId("demo/v1".into())).await;
}

/// @emoji 🪤️ Proves `assert_command_envelope_round_trip` is not a trivially-true check: a hand-rolled
/// `Mutation` whose `Deserialize` impl silently drops its own field (encodes `n` faithfully but
/// always decodes to `n: 0`) must trip law (2) of the doc comment on
/// `assert_command_envelope_round_trip` — the same "deliberately lossy impl" pattern
/// `protocol_testkit`'s `op_text_round_trip_panics_on_a_lossy_impl` uses for `assert_op_text_round_trip`.
#[semio_framework_async_macros::async_test]
#[should_panic(expected = "did not decode back into an equal forward operation")]
async fn command_envelope_round_trip_panics_on_a_lossy_operation() {
    let edit = Edit::<LossyMutation> {
        id: "edit-lossy".into(),
        actor: None,
        forwards: vec![LossyMutation::SetN(LossySetN { n: 7 })],
        inverse: vec![],
        mutation_meta: vec![],
        description: None,
        coalesce_key: None,
        sequence_number: 0,
        started_at: "2026-07-27T00:00:00Z".into(),
        finished_at: None,
    };
    test_support::assert_command_envelope_round_trip::<DemoSnapshot, LossyMutation>(&edit, &ArtifactId("doc-lossy".into()), &SchemaId("lossy/v1".into())).await;
}

// `DemoSnapshot`'s `crate::os_store::ArtifactDsl` impl and `DemoMutation`'s `crate::os_store::OpText` impl are now
// generated by `#[derive(crate::os_dsl::DslArtifact)]`/`#[derive(crate::os_dsl::DslOps)]` on the type definitions
// themselves (see `DemoSnapshot`/`DemoMutation` above) — the `dsl_schema` grammar replaces
// this crate's own hand-rolled `"n <value>"`/`"set-n <value>"` printer/parser.

#[semio_framework_async_macros::async_test]
async fn demo_dsl_round_trips() {
    test_support::assert_dsl_round_trip(&DemoSnapshot { n: Some(42) });
}

#[semio_framework_async_macros::async_test]
async fn demo_dsl_pack_equivalence() {
    test_support::assert_dsl_pack_equivalence(&DemoSnapshot { n: Some(42) });
}

#[semio_framework_async_macros::async_test]
async fn demo_op_text_round_trips() {
    test_support::assert_op_line_round_trip(&DemoMutation::SetN(SetN { n: 7 }));
}

#[semio_framework_async_macros::async_test]
async fn demo_op_binary_round_trips_and_matches_text() {
    let operation = DemoMutation::SetN(SetN { n: 7 });
    let encoded = operation.encode_op().expect("op encode");
    let encoded_again = operation.encode_op().expect("op re-encode");
    assert_eq!(encoded, encoded_again, "op binary encoding must be deterministic");
    assert_eq!(encoded[0], pack_rt::OP_BINARY_FORMAT);
    let decoded = DemoMutation::decode_op(&encoded).expect("op decode");
    assert_eq!(decoded, operation);
    let via_text = DemoMutation::parse_op(&operation.print_op()).expect("op parse");
    assert_eq!(via_text, decoded, "binary and text round trips diverged");
}

#[semio_framework_async_macros::async_test]
async fn demo_op_binary_rejects_unknown_format_and_ordinal() {
    let operation = DemoMutation::SetN(SetN { n: 7 });
    let mut wrong_format = operation.encode_op().expect("op encode");
    wrong_format[0] = 9;
    assert!(DemoMutation::decode_op(&wrong_format).is_err(), "format 9 must be rejected");
    let out_of_range = [pack_rt::OP_BINARY_FORMAT, 0x7E];
    assert!(DemoMutation::decode_op(&out_of_range).is_err(), "ordinal beyond declared variants must be rejected");
}

#[semio_framework_async_macros::async_test]
async fn print_edit_lines_emits_one_indented_line_per_forward_op() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply");
    let edit = store.envelope().vcs.edits.last().expect("edit");
    let printed = print_edit_lines(edit).await.expect("print edit lines");
    assert!(printed.starts_with("edit "), "got {printed:?}");
    assert!(printed.contains("\n  set-n n=1\n"));
}

#[semio_framework_async_macros::async_test]
async fn document_text_round_trips_after_apply_and_checkpoint() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 3 })], description: Some("bump".into()) }).await.expect("apply");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("c1".into()), authors: vec![Author { id: "a1".into(), name: "Alice".into(), avatar: None }] }).await.expect("commit");
    test_support::assert_document_text_round_trip(&store).await;
    test_support::assert_document_pack_round_trip(&store).await;
}

#[semio_framework_async_macros::async_test]
async fn parse_document_text_rejects_invalid_op_line_with_span() {
    let files = ArtifactTextFiles { dsl: "n=0\n".to_string(), ops: "doc demo schema=demo/v1\nedit e1 sequence=1 started=\"1\"\n  not-an-op\n".to_string() };
    let error = parse_document_text::<DemoSnapshot, DemoMutation>(&files.dsl, &files.ops).await.unwrap_err();
    assert_eq!(error.span.line, 3);
}

/// @emoji 🩺️ Stresses the stateful `current`/`tail_undo_cache` fast paths — multi-op edits, amend
/// gestures, undo/redo, and a checkpoint (cold-path recompute) all interleaved — against the
/// full-replay differential oracle, so any divergence between the incremental paths and a
/// from-scratch replay fails loudly here rather than surfacing as a silent snapshot bug later.
#[semio_framework_async_macros::async_test]
async fn stateful_current_matches_full_replay_across_interleaved_commands() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;

    // Multi-operation edit: current must fold both ops, matching a from-scratch replay.
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 }), DemoMutation::SetN(SetN { n: 2 })], description: None }).await.expect("apply multi-op edit");
    test_support::assert_live_equals_replay(&store).await;
    assert_eq!(store.snapshot().expect("snapshot").n, Some(2));

    // Amend gesture: the first `AmendLast` cannot merge into the preceding `Apply`-created edit
    // (`Apply` never sets a `coalesce_key`, so it can never match), so it starts a NEW edit; the
    // second `AmendLast` shares that edit's key and merges into it — two edits total, the second
    // one carrying two coalesced increments (3 then 4).
    store.dispatch(ArtifactCommand::AmendLast { mutations: vec![DemoMutation::SetN(SetN { n: 3 })], coalesce_key: Some("drag".into()) }).await.expect("amend 1");
    store.dispatch(ArtifactCommand::AmendLast { mutations: vec![DemoMutation::SetN(SetN { n: 4 })], coalesce_key: Some("drag".into()) }).await.expect("amend 2");
    test_support::assert_live_equals_replay(&store).await;
    assert_eq!(store.snapshot().expect("snapshot").n, Some(4));
    assert_eq!(store.envelope().vcs.edits.len(), 2, "the amend gesture started its own edit, not a third");

    // Undo the whole amended edit (O(1) tail-cache path) restores the `Apply`-edit's state, not
    // the initial snapshot — only the amend gesture's edit is undone here.
    store.dispatch(ArtifactCommand::Undo).await.expect("undo");
    test_support::assert_live_equals_replay(&store).await;
    assert_eq!(store.snapshot().expect("snapshot").n, Some(2));
    store.dispatch(ArtifactCommand::Redo).await.expect("redo");
    test_support::assert_live_equals_replay(&store).await;
    assert_eq!(store.snapshot().expect("snapshot").n, Some(4));

    // Checkpoint (cold path through `checkout_checkpoint_internal` is NOT exercised by commit
    // itself, but a following apply + a second, older undo still must agree with replay).
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("c1".into()), authors: Vec::new() }).await.expect("commit");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 5 })], description: None }).await.expect("apply after checkpoint");
    test_support::assert_live_equals_replay(&store).await;
    store.dispatch(ArtifactCommand::Undo).await.expect("undo after checkpoint");
    test_support::assert_live_equals_replay(&store).await;
    assert_eq!(store.snapshot().expect("snapshot").n, Some(4));
}

//#region 🏛️SpaceTests
/// @emoji ⏱️ Like `DemoMutation` but with an explicit, test-controlled `timestamp()` override, so
/// undo-ordering-by-HLT tests don't depend on real wall-clock resolution.

//#region 🔖️OpCodec
/// 🎞️ Handcrafted OpText (P6).
impl OpText for TimestampedMutation {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = crate::os_dsl::parse(line, &spec_fn(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Inline })?;
                return <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(crate::os_dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        crate::os_dsl::print(&record, &spec_fn(), crate::os_dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl OpBinary for TimestampedMutation {
    fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(crate::os_spr::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1)();
        let body = crate::os_pack::encode_record_body(&spec, &record, &PackEncodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        crate::os_pack::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = crate::os_pack::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(crate::os_spr::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(crate::os_spr::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = crate::os_pack::decode_record_body(body, &spec, &PackDecodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
        let record_offset = reader.position() as u64;
        <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| crate::os_spr::ProtocolError::Malformed { what: "op record", offset: record_offset, detail: error.to_string() })
    }
}
//#endregion 🔖️OpCodec

/// @emoji 🪄️ Downcasts a registered `dyn SpaceMember` back to its concrete demo store.
async fn demo_member<'a, Mutation: self::Mutation<DemoSnapshot> + 'static, M: SpaceMember + 'static>(host: &'a mut SpaceHost<M>, document_id: &str) -> &'a mut ArtifactStore<DemoSnapshot, Mutation> {
    host.member_mut(document_id).await.expect("member registered").as_any_mut().await.downcast_mut::<ArtifactStore<DemoSnapshot, Mutation>>().expect("concrete member type matches")
}

#[semio_framework_async_macros::async_test]
async fn register_space_documents_registers_manifest_collections_and_artifacts_together() {
    // 🎯️ Every member below gets at least one uncommitted edit (dirty), mirroring
    // `space_checkpoint_commits_dirty_members_and_pins_their_checkpoints`'s `member_a` — a fresh
    // member with zero edits and zero checkpoints has no `current_checkpoint_id` yet, which
    // `commit_space_checkpoint` requires of every registered member (dirty ones are auto-committed,
    // already-clean ones just need a prior checkpoint).
    let mut manifest = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "space-manifest", DemoSnapshot { n: Some(0) }, None)).await;
    manifest.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply manifest edit");
    let mut collection_a = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "collection-a", DemoSnapshot { n: Some(0) }, None)).await;
    collection_a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], description: None }).await.expect("apply collection a edit");
    let mut collection_b = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "collection-b", DemoSnapshot { n: Some(0) }, None)).await;
    collection_b.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 3 })], description: None }).await.expect("apply collection b edit");
    let mut artifact_a = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "artifact-a", DemoSnapshot { n: Some(0) }, None)).await;
    artifact_a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 7 })], description: None }).await.expect("apply artifact edit");

    let mut host = SpaceHost::new(create_document_envelope(&format!("{S_SPACE_HISTORY_SCHEMA}/v1"), "studio", SpaceHistorySnapshot::default(), None)).await.expect("valid space host history");
    host.register_space_documents(manifest, vec![collection_a, collection_b], vec![artifact_a]).await;

    assert!(host.member("space-manifest").await.is_some(), "manifest registered");
    assert!(host.member("collection-a").await.is_some(), "collection a registered");
    assert!(host.member("collection-b").await.is_some(), "collection b registered");
    assert!(host.member("artifact-a").await.is_some(), "artifact registered");

    let space_checkpoint_id = host.commit_space_checkpoint("initial space checkpoint".into(), Vec::new()).await.expect("commit space checkpoint");
    let snapshot = host.meta_snapshot().await.expect("meta snapshot");
    let checkpoint = snapshot.checkpoints.iter().find(|checkpoint| checkpoint.id == space_checkpoint_id).expect("checkpoint recorded");
    assert_eq!(checkpoint.members.len(), 4, "manifest + 2 collections + 1 artifact all pinned atomically in one space checkpoint");
    let pinned_ids: HashSet<&str> = checkpoint.members.iter().map(|pin| pin.document_id.as_str()).collect();
    assert_eq!(pinned_ids, HashSet::from(["space-manifest", "collection-a", "collection-b", "artifact-a"]));
}

#[semio_framework_async_macros::async_test]
async fn space_checkpoint_commits_dirty_members_and_pins_their_checkpoints() {
    let mut member_a = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "member-a", DemoSnapshot { n: Some(0) }, None)).await;
    member_a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply a");

    let mut member_b = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "member-b", DemoSnapshot { n: Some(0) }, None)).await;
    member_b.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 5 })], description: None }).await.expect("apply b");
    member_b.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("b-init".into()), authors: Vec::new() }).await.expect("commit b upfront, so it starts clean");
    let member_b_checkpoint = member_b.current_checkpoint_id().await.expect("b checkpoint").to_string();

    let mut host = SpaceHost::new(create_document_envelope(&format!("{S_SPACE_HISTORY_SCHEMA}/v1"), "studio", SpaceHistorySnapshot::default(), None)).await.expect("valid space host history");
    host.register_member(member_a).await;
    host.register_member(member_b).await;

    let space_checkpoint_id = host.commit_space_checkpoint("studio init".into(), vec![Author { id: "a1".into(), name: "Alice".into(), avatar: None }]).await.expect("commit space checkpoint");

    let snapshot = host.meta_snapshot().await.expect("meta snapshot");
    assert_eq!(snapshot.checkpoints.len(), 1);
    let checkpoint = &snapshot.checkpoints[0];
    assert_eq!(checkpoint.id, space_checkpoint_id);
    assert_eq!(checkpoint.members.len(), 2, "pins one entry per registered member");
    let pin_b = checkpoint.members.iter().find(|pin| pin.document_id == "member-b").expect("pin b");
    assert_eq!(pin_b.checkpoint_id, member_b_checkpoint, "clean member reuses its existing checkpoint");
    assert!(!host.member("member-a").await.expect("member a").is_dirty().await, "dirty member-a is committed (and therefore clean) by the space checkpoint");
}

#[semio_framework_async_macros::async_test]
async fn space_vcs_host_meta_document_is_backbone_attachable_and_detachable() {
    let (backbone_a, backbone_b) = MemoryBackbone::pair("studio-a", "studio-b").await;
    let mut host_a = SpaceHost::new(create_document_envelope(&format!("{S_SPACE_HISTORY_SCHEMA}/v1"), "studio", SpaceHistorySnapshot::default(), None)).await.expect("valid first space host history");
    let mut host_b = SpaceHost::<ArtifactStore<DemoSnapshot, DemoMutation>>::new(create_document_envelope(&format!("{S_SPACE_HISTORY_SCHEMA}/v1"), "studio", SpaceHistorySnapshot::default(), None)).await.expect("valid second space host history");
    assert!(host_a.backbone_ref().await.is_none(), "default is unattached, like any other ArtifactStore");

    host_a.attach_backbone(Backbones::Memory(backbone_a)).await.expect("attach a");
    host_b.attach_backbone(Backbones::Memory(backbone_b)).await.expect("attach b");
    assert!(host_a.backbone_ref().await.is_some());

    let mut member = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "member-a", DemoSnapshot { n: Some(0) }, None)).await;
    member.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply on member, so it's dirty and can be committed");
    host_a.register_member(member).await;
    host_a.commit_space_checkpoint("studio init".into(), Vec::new()).await.expect("commit space checkpoint on a");

    host_b.tick().await.expect("tick b");
    assert_eq!(host_b.meta_snapshot().await.expect("meta snapshot b").checkpoints.len(), 1, "the space-wide checkpoint replicates through the meta-document's backbone");

    host_a.detach_backbone().await.expect("detach source host backbone");
    assert!(host_a.backbone_ref().await.is_none());
    host_a.commit_space_checkpoint("studio offline".into(), Vec::new()).await.expect("meta history keeps working purely in memory once detached");
    host_b.tick().await.expect("tick b again");
    assert_eq!(host_b.meta_snapshot().await.expect("meta snapshot b unchanged").checkpoints.len(), 1, "detached space edits never reach the peer");
}

#[semio_framework_async_macros::async_test]
async fn space_checkout_checkpoint_fans_out_and_restores_pinned_member_state() {
    let member_a = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "member-a", DemoSnapshot { n: Some(0) }, None)).await;
    let mut host = SpaceHost::new(create_document_envelope(&format!("{S_SPACE_HISTORY_SCHEMA}/v1"), "studio", SpaceHistorySnapshot::default(), None)).await.expect("valid space host history");
    host.register_member(member_a).await;

    demo_member::<DemoMutation, _>(&mut host, "member-a").await.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply 1");
    let space_checkpoint_1 = host.commit_space_checkpoint("first".into(), Vec::new()).await.expect("commit 1");

    demo_member::<DemoMutation, _>(&mut host, "member-a").await.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], description: None }).await.expect("apply 2");
    host.commit_space_checkpoint("second".into(), Vec::new()).await.expect("commit 2");
    assert_eq!(demo_member::<DemoMutation, _>(&mut host, "member-a").await.snapshot().expect("snapshot").n, Some(2), "member reflects the second space checkpoint before checking out the first");

    host.checkout_space_checkpoint(&space_checkpoint_1).await.expect("checkout space checkpoint 1");
    assert_eq!(demo_member::<DemoMutation, _>(&mut host, "member-a").await.snapshot().expect("snapshot").n, Some(1), "checking out the first space checkpoint fans out and restores member-a's pinned state");
}

#[semio_framework_async_macros::async_test]
async fn space_switch_alternative_fans_out_and_restores_pinned_member_state() {
    let member_a = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "member-a", DemoSnapshot { n: Some(0) }, None)).await;
    let mut host = SpaceHost::new(create_document_envelope(&format!("{S_SPACE_HISTORY_SCHEMA}/v1"), "studio", SpaceHistorySnapshot::default(), None)).await.expect("valid space host history");
    host.register_member(member_a).await;

    demo_member::<DemoMutation, _>(&mut host, "member-a").await.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply 1");
    host.commit_space_checkpoint("root".into(), Vec::new()).await.expect("commit root");

    let alt_id = host.create_space_alternative("branch-a".into()).await.expect("create alternative");

    demo_member::<DemoMutation, _>(&mut host, "member-a").await.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], description: None }).await.expect("apply 2 (uncommitted at the studio level)");
    assert_eq!(demo_member::<DemoMutation, _>(&mut host, "member-a").await.snapshot().expect("snapshot").n, Some(2), "uncommitted edit is live before switching");

    host.switch_space_alternative(&alt_id).await.expect("switch alternative fans out to its pinned checkpoint");
    assert_eq!(demo_member::<DemoMutation, _>(&mut host, "member-a").await.snapshot().expect("snapshot").n, Some(1), "switching alternatives restores each member to its pinned checkpoint, discarding the uncommitted edit");
}

#[semio_framework_async_macros::async_test]
async fn space_undo_and_redo_target_the_member_with_the_most_recent_local_edit_by_hlt() {
    let mut member_early = ArtifactStore::new(create_document_envelope::<DemoSnapshot, TimestampedMutation>("demo-ts/v1", "member-early", DemoSnapshot { n: Some(0) }, None)).await;
    member_early.dispatch(ArtifactCommand::Apply { mutations: vec![TimestampedMutation::SetN(TimestampedSetN { n: 1, physical_ms: 1_000 })], description: None }).await.expect("apply early");

    let mut member_late = ArtifactStore::new(create_document_envelope::<DemoSnapshot, TimestampedMutation>("demo-ts/v1", "member-late", DemoSnapshot { n: Some(0) }, None)).await;
    member_late.dispatch(ArtifactCommand::Apply { mutations: vec![TimestampedMutation::SetN(TimestampedSetN { n: 9, physical_ms: 2_000 })], description: None }).await.expect("apply late");

    let mut host = SpaceHost::new(create_document_envelope(&format!("{S_SPACE_HISTORY_SCHEMA}/v1"), "studio", SpaceHistorySnapshot::default(), None)).await.expect("valid space host history");
    host.register_member(member_early).await;
    host.register_member(member_late).await;

    host.undo().await.expect("space undo targets the member with the higher HLT");
    assert_eq!(demo_member::<TimestampedMutation, _>(&mut host, "member-early").await.snapshot().expect("early snapshot").n, Some(1), "earlier local edit (lower HLT) is untouched");
    assert_eq!(demo_member::<TimestampedMutation, _>(&mut host, "member-late").await.snapshot().expect("late snapshot").n, Some(0), "later local edit (higher HLT) is the one undone");

    host.redo().await.expect("studio redo targets the most recently undone edit");
    assert_eq!(demo_member::<TimestampedMutation, _>(&mut host, "member-late").await.snapshot().expect("late snapshot after redo").n, Some(9), "redo restores the member's most recently undone edit");
}

#[semio_framework_async_macros::async_test]
// 🎞️ `Mutation::reconcile`/`reconcile_with_last`/`snapshot_with_conflicts` are all deleted
// (26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS §C4/C6/C10) — concurrent-
// merge arbitration is now `ingest_remote`/`resolve_conflict` against first-class `Conflict`s.
async fn snapshot_matches_materialize_and_conflicts_stay_empty_absent_remote_ingestion() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 3 })], description: None }).await.expect("apply");
    let replayed = materialize_document_snapshot(store.envelope(), store.applied_edit_ids()).await.expect("replay");
    assert_eq!(replayed.n, Some(3));
    assert_eq!(store.snapshot().expect("snapshot").n, Some(3));
    assert!(store.conflicts().is_empty(), "no remote ingestion happened, so the store's conflict buffer stays empty");
    assert!(store.open_conflicts().next().is_none());
}

#[semio_framework_async_macros::async_test]
async fn space_history_op_round_trips() {
    let checkpoint = SpaceCheckpoint {
        id: "sc-1".into(),
        parent_id: None,
        message: "root".into(),
        authors: Vec::new(),
        timestamp: HybridLogicalTimestamp::new(0, 1),
        members: vec![SpaceMemberPin { document_id: "member-a".into(), checkpoint_id: "cp-1".into(), alternative_id: String::new() }],
    };
    test_support::assert_operation_round_trip(&SpaceHistorySnapshot::default(), SpaceHistoryMutation::CommitSpaceCheckpoint(CommitSpaceCheckpoint { checkpoint: checkpoint.clone() })).await;

    let with_checkpoint = SpaceHistorySnapshot { checkpoints: vec![checkpoint], alternatives: Vec::new(), active_alternative_id: None };
    let alternative = SpaceAlternative { id: "sa-1".into(), name: "branch".into(), checkpoint_ids: vec!["sc-1".into()] };
    test_support::assert_operation_round_trip(&with_checkpoint, SpaceHistoryMutation::CreateSpaceAlternative(CreateSpaceAlternative { alternative })).await;

    let with_alternative_active = SpaceHistorySnapshot {
        alternatives: vec![SpaceAlternative { id: "sa-1".into(), name: "branch".into(), checkpoint_ids: vec!["sc-1".into()] }, SpaceAlternative { id: "sa-other".into(), name: "other".into(), checkpoint_ids: vec!["sc-1".into()] }],
        active_alternative_id: Some("sa-1".into()),
        ..with_checkpoint.clone()
    };
    test_support::assert_operation_round_trip(&with_alternative_active, SpaceHistoryMutation::SwitchSpaceAlternative(SwitchSpaceAlternative { alternative_id: "sa-other".into() })).await;
    test_support::assert_operation_round_trip(&with_checkpoint, SpaceHistoryMutation::RemoveSpaceCheckpoint(RemoveSpaceCheckpoint { checkpoint_id: "sc-1".into() })).await;
    test_support::assert_operation_round_trip(&with_alternative_active, SpaceHistoryMutation::RemoveSpaceAlternative(RemoveSpaceAlternative { alternative_id: "sa-other".into() })).await;
    test_support::assert_operation_round_trip(&with_alternative_active, SpaceHistoryMutation::RemoveSpaceAlternative(RemoveSpaceAlternative { alternative_id: "sa-1".into() })).await;
    test_support::assert_operation_round_trip(&with_alternative_active, SpaceHistoryMutation::RestoreActiveSpaceAlternative(RestoreActiveSpaceAlternative { alternative_id: None })).await;
}

//#endregion 🏛️StudioTests

//#region 🔖️PreviewWireTests
/// @emoji 🧪️ Fixture producing one message per non-clean variant, each using one of the frozen
/// seven `mutation.*` codes (`📋️contract-freeze.md` §C2's table) — `preview_wire`'s and
/// `CompositionCoordinator` phase 1's shared dry-run fixture. `WarnN` ⇒ `mutation.clamped`
/// (Warning, non-empty diff), `ErrorN` ⇒ `mutation.target-missing` (Error, empty diff — LAW 2),
/// `FatalN` ⇒ `mutation.invariant` (Fatal, empty diff — LAW 1), `CleanN` ⇒ silent.

/// 🎞️ Handcrafted OpText (P6).
impl OpText for SeverityMutation {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = crate::os_dsl::parse(line, &spec_fn(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Inline })?;
                return <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(crate::os_dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        crate::os_dsl::print(&record, &spec_fn(), crate::os_dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl OpBinary for SeverityMutation {
    fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(crate::os_spr::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1)();
        let body = crate::os_pack::encode_record_body(&spec, &record, &PackEncodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        crate::os_pack::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = crate::os_pack::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(crate::os_spr::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(crate::os_spr::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = crate::os_pack::decode_record_body(body, &spec, &PackDecodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
        let record_offset = reader.position() as u64;
        <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| crate::os_spr::ProtocolError::Malformed { what: "op record", offset: record_offset, detail: error.to_string() })
    }
}

impl MemberStoreOwner<SeverityMutation> for DemoSnapshot {
    type SnapshotOpen = UnsupportedMemberSnapshotOpen<Self>;

    fn member_store_owners() -> MemberStoreOwners<Self, SeverityMutation> {
        MemberStoreOwners::new(Arc::new(DemoSnapshotRetirementFactory), Arc::new(DemoInitialSnapshotRetirementFactory), Arc::new(DemoMutationRetirementFactory), Box::new(DemoStoreOwnedDisposer::<SeverityMutation>(PhantomData)))
    }
}

/// 🛰️ Builds an explicitly stamped Severity envelope for policy and quarantine law tests.
fn severity_mutation_envelope_at(document_id: &str, actor: &str, mutation_id: &str, operation: SeverityMutation, timestamp: HybridLogicalTimestamp) -> crate::os_spr::MutationEnvelope {
    crate::os_spr::MutationEnvelope {
        mutation_id: MutationId(mutation_id.to_string()),
        document_id: ArtifactId(document_id.to_string()),
        actor: ActorId(actor.to_string()),
        dependencies: Vec::new(),
        diff: crate::os_spr::ArtifactDiff { schema: SchemaId("demo/v1".to_string()), payload: operation.encode_op().expect("encode severity mutation") },
        inverse: crate::os_spr::InverseMutation { schema: SchemaId("demo/v1".to_string()), payload: Vec::new() },
        timestamp,
    }
}

#[semio_framework_async_macros::async_test]
async fn document_text_round_trips_authoritative_metadata_messages_conflicts_and_cursor() {
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", "severity-text", DemoSnapshot { n: Some(0) }, None)).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![SeverityMutation::SetWarningN(SetWarningN { n: 3 })], description: Some("durable warning".into()) }).await.expect("apply warning");
    let edit = store.envelope().vcs.edits.last().expect("one durable edit").clone();
    let messages = store.envelope().edit_messages.last().expect("durable outcome ledger").messages.clone();
    let kind = crate::os_spr::ConflictKind::Degraded { edit_ids: vec![edit.id.clone()] };
    let mutation_ids = stable_mutation_ids_for_edit(&edit).await.expect("stable operation identity");
    let timestamp = edit.mutation_meta.first().expect("metadata timestamp").timestamp;
    let document_id = ArtifactId(store.envelope().id.clone());
    store.0.envelope.conflicts.push(crate::os_spr::Conflict {
        id: crate::os_spr::ConflictId::new(&kind, &document_id, &mutation_ids, &timestamp).await,
        kind,
        status: crate::os_spr::ConflictStatus::Open,
        messages,
        actors: vec![ActorId(edit.actor.clone().expect("edit actor"))],
        timestamp,
    });
    let files = print_document_text(store.envelope()).await.expect("print full-fidelity text");
    for record in ["inverse ", "metadata ", "message ", "conflict ", "cursor "] {
        assert!(files.ops.lines().any(|line| line.starts_with(record)), "missing {record:?} record: {}", files.ops);
    }
    let parsed = parse_document_text::<DemoSnapshot, SeverityMutation>(&files.dsl, &files.ops).await.expect("parse full-fidelity text");
    assert_eq!(&parsed.envelope, store.envelope());
    assert_eq!(parsed.snapshot, store.snapshot().expect("live snapshot"));
}

/// @emoji 🔬️ `print_ops_log`'s `metadata `/`message `/`conflict ` records now build their `data`
/// payload with `crate::os_pack::json::to_json_string` instead of `serde_json::to_string`
/// (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/02) — direct proof the two
/// encode the same ordered first-party values byte-identically, exercising both the
/// sparse (every `Option`/default field omitted) and dense (every optional field populated,
/// including `MutationOrigin`'s two non-default enum-variant-with-fields shapes) ends of each
/// type's `skip_serializing_if` surface.
#[test]
fn ops_log_records_to_json_string_match_serde_json_byte_for_byte() {
    let sparse_meta = MutationMeta {
        mutation_id: None,
        dependencies: Vec::new(),
        base_version: 1,
        author_id: None,
        timestamp: HybridLogicalTimestamp::new(1, 100),
        undo_policy: UndoPolicy::ExactBaseOnly,
        payload_hash: None,
        semantic_kind: None,
        label: None,
        group_id: None,
        origin: crate::os_spr::MutationOrigin::Owner,
    };
    let dense_meta = MutationMeta {
        mutation_id: Some(MutationId("mutation-1".into())),
        dependencies: vec![MutationId("mutation-0".into())],
        base_version: 7,
        author_id: Some(ActorId("actor-1".into())),
        timestamp: HybridLogicalTimestamp::new(9, 300),
        undo_policy: UndoPolicy::TransformAgainstConcurrent,
        payload_hash: Some(crate::os_spr::PayloadHash([7u8; 32])),
        semantic_kind: Some(SchemaId("demo/v1#set-n".into())),
        label: Some("set n".into()),
        group_id: Some("group-1".into()),
        origin: crate::os_spr::MutationOrigin::Contributed { plugin_id: "plugin-a".into(), mutation_id: SchemaId("mutation-src".into()), payload_hash: crate::os_spr::PayloadHash([3u8; 32]) },
    };
    let transaction_meta =
        MutationMeta { origin: crate::os_spr::MutationOrigin::Transaction { initiator: crate::os_spr::ForeignTarget { artifact_id: "artifact-1".into(), artifact_kind: "demo".into(), dialect: Some("v1".into()) } }, ..dense_meta.clone() };
    for meta in [&sparse_meta, &dense_meta, &transaction_meta] {
        let mine = crate::os_pack::json::to_json_string(meta);
        let theirs = serde_json::to_string(&test_support::SerdeValue(&meta.to_value())).expect("serde_json encodes MutationMeta fields");
        assert_eq!(mine, theirs, "MutationMeta's ToValue/pack::json bridge diverged from serde_json for origin={:?}", meta.origin);
    }

    let sparse_message = crate::os_spr::MutationMessage { level: crate::os_dsl::Severity::Warning, code: crate::os_dsl::FaultCode("mutation.no-op".into()), message: "no-op".into(), target: Vec::new(), op_index: None };
    let dense_message = crate::os_spr::MutationMessage { level: crate::os_dsl::Severity::Fatal, code: crate::os_dsl::FaultCode("mutation.invariant".into()), message: "n invariant violated".into(), target: vec!["n".into()], op_index: Some(2) };
    for message in [&sparse_message, &dense_message] {
        let mine = crate::os_pack::json::to_json_string(message);
        let theirs = serde_json::to_string(&test_support::SerdeValue(&message.to_value())).expect("serde_json encodes MutationMessage fields");
        assert_eq!(mine, theirs, "MutationMessage's ToValue/pack::json bridge diverged from serde_json for op_index={:?}", message.op_index);
    }

    let conflict = crate::os_spr::Conflict {
        id: crate::os_spr::ConflictId("conflict-1".into()),
        kind: crate::os_spr::ConflictKind::Degraded { edit_ids: vec!["edit-1".into(), "edit-2".into()] },
        status: crate::os_spr::ConflictStatus::Open,
        messages: vec![sparse_message, dense_message],
        actors: vec![ActorId("actor-1".into()), ActorId("actor-2".into())],
        timestamp: HybridLogicalTimestamp::new(9, 300),
    };
    let mine = crate::os_pack::json::to_json_string(&conflict);
    let theirs = serde_json::to_string(&test_support::SerdeValue(&conflict.to_value())).expect("serde_json encodes Conflict fields");
    assert_eq!(mine, theirs, "Conflict's ToValue/pack::json bridge diverged from serde_json");
}

#[semio_framework_async_macros::async_test]
async fn document_text_rejects_missing_metadata_and_unknown_cursor_without_synthesis() {
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", "strict-text", DemoSnapshot { n: Some(0) }, None)).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![SeverityMutation::SetWarningN(SetWarningN { n: 3 })], description: None }).await.expect("apply warning");
    let files = print_document_text(store.envelope()).await.expect("print strict text");
    let missing_metadata = files.ops.lines().filter(|line| !line.starts_with("metadata ")).collect::<Vec<_>>().join("\n");
    assert!(matches!(parse_document_text::<DemoSnapshot, SeverityMutation>(&files.dsl, &missing_metadata).await, Err(error) if error.message.contains("no metadata records")));

    let unknown_cursor = files.ops.lines().map(|line| if line.starts_with("cursor ") { "cursor applied=[ unknown-edit ] redo=[]".to_string() } else { line.to_string() }).collect::<Vec<_>>().join("\n");
    assert!(matches!(parse_document_text::<DemoSnapshot, SeverityMutation>(&files.dsl, &unknown_cursor).await, Err(error) if error.message.contains("unknown or duplicate applied edit")));
}

#[semio_framework_async_macros::async_test]
async fn document_text_preserves_non_contiguous_edit_sequences_and_rejects_invalid_ones() {
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", "strict-sequence", DemoSnapshot { n: Some(0) }, None)).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![SeverityMutation::SetN(SeveritySetN { n: 1 })], description: None }).await.expect("first edit");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![SeverityMutation::SetN(SeveritySetN { n: 2 })], description: None }).await.expect("second edit");
    store.0.envelope.vcs.edits[0].sequence_number = 4;
    store.0.envelope.vcs.edits[1].sequence_number = 17;

    let files = print_document_text(store.envelope()).await.expect("print strict text");
    let parsed = parse_document_text::<DemoSnapshot, SeverityMutation>(&files.dsl, &files.ops).await.expect("parse strict text");
    assert_eq!(parsed.envelope.vcs.edits.iter().map(|edit| edit.sequence_number).collect::<Vec<_>>(), vec![4, 17]);

    let invalid = files.ops.replacen("sequence=4", "sequence=-1", 1);
    assert!(matches!(parse_document_text::<DemoSnapshot, SeverityMutation>(&files.dsl, &invalid).await, Err(error) if error.message.contains("invalid edit sequence -1")));
}

#[semio_framework_async_macros::async_test]
async fn persisted_conflict_requires_a_global_owned_operation_index() {
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", "conflict-index", DemoSnapshot { n: Some(0) }, None)).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![SeverityMutation::SetWarningN(SetWarningN { n: 3 })], description: None }).await.expect("apply warning");
    let edit = store.envelope().vcs.edits.last().expect("one edit").clone();
    let kind = crate::os_spr::ConflictKind::Degraded { edit_ids: vec![edit.id.clone()] };
    let mutation_ids = stable_mutation_ids_for_edit(&edit).await.expect("stable mutation ids");
    let timestamp = edit.mutation_meta.first().expect("timestamp").timestamp;
    let mut messages = store.envelope().edit_messages.last().expect("outcome message").messages.clone();
    messages[0].op_index = Some(1);
    let document_id = ArtifactId(store.envelope().id.clone());
    store.0.envelope.conflicts.push(crate::os_spr::Conflict {
        id: crate::os_spr::ConflictId::new(&kind, &document_id, &mutation_ids, &timestamp).await,
        kind,
        status: crate::os_spr::ConflictStatus::Open,
        messages,
        actors: vec![ActorId(edit.actor.clone().expect("actor"))],
        timestamp,
    });
    assert!(matches!(validate_persisted_conflicts(store.envelope()).await, Err(VcsError::ValidationFailed(message)) if message.contains("operation index")));
}

#[semio_framework_async_macros::async_test]
async fn conflict_generation_and_validation_canonicalize_repeated_actors() {
    let document_id = "repeated-conflict-actor";
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", document_id, DemoSnapshot { n: Some(0) }, None)).await;
    let clean = severity_mutation_envelope_at(document_id, "same-peer", "same-clean", SeverityMutation::SetN(SeveritySetN { n: 1 }), HybridLogicalTimestamp::new(1, 100));
    let mut fatal = severity_mutation_envelope_at(document_id, "same-peer", "same-fatal", SeverityMutation::SetFatalN(SetFatalN { n: 2 }), HybridLogicalTimestamp::new(2, 200));
    fatal.dependencies = vec![clean.mutation_id.clone()];

    store.ingest_remote(fatal).await.expect("dependent fatal is buffered");
    let report = store.ingest_remote(clean).await.expect("ready batch is reported");
    let conflict_id = report.conflict.expect("fatal batch is quarantined");
    assert!(!report.accepted);
    let conflict = store.conflicts().iter().find(|conflict| conflict.id == conflict_id).expect("durable conflict");
    assert_eq!(conflict.actors, vec![ActorId("same-peer".into())]);
    validate_persisted_conflicts(store.envelope()).await.expect("generated actors use the canonical unique participant set");

    let mut malformed = owned_test_envelope(&store).await;
    malformed.conflicts[0].actors.push(ActorId("same-peer".into()));
    assert!(matches!(validate_persisted_conflicts(&malformed).await, Err(VcsError::ValidationFailed(message)) if message.contains("malformed actor identities")));
}

#[semio_framework_async_macros::async_test]
async fn quarantined_accept_is_atomic_when_a_later_envelope_remains_fatal() {
    let document_id = "severity-atomic";
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", document_id, DemoSnapshot { n: Some(0) }, None)).await;
    let clean = severity_mutation_envelope_at(document_id, "clean-peer", "clean-op", SeverityMutation::SetN(SeveritySetN { n: 1 }), HybridLogicalTimestamp::new(1, 100));
    let fatal = severity_mutation_envelope_at(document_id, "fatal-peer", "fatal-op", SeverityMutation::SetFatalN(SetFatalN { n: 2 }), HybridLogicalTimestamp::new(2, 200));
    let kind = crate::os_spr::ConflictKind::Quarantined { envelopes: vec![clean.clone(), fatal.clone()] };
    let timestamp = HybridLogicalTimestamp::new(9, 300);
    let mutation_ids = vec![clean.mutation_id.clone(), fatal.mutation_id.clone()];
    let conflict_id = crate::os_spr::ConflictId::new(&kind, &ArtifactId(document_id.to_string()), &mutation_ids, &timestamp).await;
    store.0.envelope.conflicts.push(crate::os_spr::Conflict {
        id: conflict_id.clone(),
        kind,
        status: crate::os_spr::ConflictStatus::Open,
        messages: vec![crate::os_spr::MutationMessage {
            level: crate::os_dsl::Severity::Fatal,
            code: crate::os_dsl::FaultCode("mutation.invariant".to_string()),
            message: "n invariant violated".to_string(),
            target: vec!["n".to_string()],
            op_index: Some(0),
        }],
        actors: vec![clean.actor.clone(), fatal.actor.clone()],
        timestamp,
    });
    validate_persisted_conflicts(store.envelope()).await.expect("well-formed quarantine fixture");
    let before = owned_test_envelope(&store).await;
    let generation = store.generation();

    let report = store.resolve_conflict(&conflict_id.0, crate::os_spr::ConflictResolution::Accept).await.expect("fatal outcome is reported, not an infrastructure error");

    assert!(!report.accepted);
    assert_eq!(report.replayed.len(), 2, "the aggregate report must retain the clean and fatal replay reports");
    assert_eq!(store.envelope(), &before, "the candidate's earlier clean mutation must never leak through a later fatal rejection");
    assert_eq!(store.snapshot().expect("unchanged snapshot"), DemoSnapshot { n: Some(0) });
    assert_eq!(store.generation(), generation, "a rejected candidate does not invalidate the source store");
    assert_eq!(store.conflicts().iter().find(|conflict| conflict.id == conflict_id).expect("original conflict retained").status, crate::os_spr::ConflictStatus::Open);
}

#[semio_framework_async_macros::async_test]
async fn empty_store_snapshot_policy_rejection_keeps_remote_history_quarantined() {
    let document_id = "severity-snapshot";
    let fatal = severity_mutation_envelope_at(document_id, "fatal-peer", "fatal-op", SeverityMutation::SetFatalN(SetFatalN { n: 2 }), HybridLogicalTimestamp::new(2, 200));
    let mut remote = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", document_id, DemoSnapshot { n: Some(0) }, None)).await;
    remote.0.envelope.vcs.edits.try_push(edit_from_operation_envelope::<SeverityMutation>(&fatal).await.expect("fatal edit")).expect("test edit fits the fixed history ledger");
    remote.0.envelope.cursor = Some(ArtifactCursor::new(vec![fatal.mutation_id.0.clone()], Vec::new(), None));
    let files = remote.snapshot_pack().await.expect("remote snapshot");

    let mut local = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", document_id, DemoSnapshot { n: Some(0) }, None)).await;
    let before = owned_test_envelope(&local).await;
    assert!(matches!(local.merge_remote_snapshot(&files.pack, &files.spr), Err(VcsError::Rejected { policy: crate::os_spr::MergePolicy::Normal, .. })));
    assert_eq!(local.snapshot().expect("local content remains unchanged"), DemoSnapshot { n: Some(0) });
    assert!(local.envelope().vcs.edits.is_empty(), "rejected remote edits are never adopted into local history");
    assert_eq!(local.conflicts().len(), 1);
    assert_eq!(local.conflicts()[0].status, crate::os_spr::ConflictStatus::Open);
    assert_eq!(local.envelope().id, before.id);
    assert_eq!(local.envelope().schema, before.schema);
}

/// @emoji 🧪️ `preview_wire`'s headline law (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-
/// FIRST-CLASS-CONFLICTS` §C6): its output is exactly what independently folding the same ops
/// through the diff engine (`apply_mutation`, the same primitive `replay_mutations` itself calls
/// on the real apply path) computes, stamped with `op_index`, and the live store is byte-
/// identical before and after — a pure dry run all the way through, never applying anything even
/// though it threads state forward internally to preview op `i` against `0..i`'s outcome.
#[semio_framework_async_macros::async_test]
async fn preview_wire_reports_the_same_messages_the_real_apply_would_produce_and_changes_nothing() {
    let envelope: ArtifactEnvelope<DemoSnapshot, SeverityMutation> = create_document_envelope("demo/v1", "preview-demo", DemoSnapshot { n: Some(0) }, None);
    let store = ArtifactStore::new(envelope).await;
    let before = owned_test_envelope(&store).await;

    let ops: Vec<Vec<u8>> = vec![
        SeverityMutation::SetN(SeveritySetN { n: 1 }).encode_op().expect("encode clean"),
        SeverityMutation::SetWarningN(SetWarningN { n: 2 }).encode_op().expect("encode warn"),
        SeverityMutation::SetErrorN(SetErrorN { n: 99 }).encode_op().expect("encode error"),
        SeverityMutation::SetFatalN(SetFatalN { n: 99 }).encode_op().expect("encode fatal"),
    ];

    let messages = store.preview_wire(&ops).await;

    // 🧮️ Independently folds the SAME ops through the exact primitive the real apply path
    // (`replay_mutations`) itself calls (`apply_mutation`) — the ground truth `preview_wire`
    // must match. `replay_mutations` does not yet surface its own messages (lane 1-A's pending
    // C6 work, see `📓️w1-e-report.md`), so this recomputation is the closest available proof.
    let mut expected = Vec::new();
    let mut running = DemoSnapshot { n: Some(0) };
    for (index, op) in ops.iter().enumerate() {
        let mutation = SeverityMutation::decode_op(op).expect("decode");
        let (next, op_messages) = apply_mutation(&running, &mutation).expect("preview fixture diff applies");
        for message in op_messages {
            expected.push(message.at_op(index as u32));
        }
        running = next;
    }
    assert_eq!(messages, expected, "preview_wire must report exactly the messages the real diff engine computes");
    assert_eq!(messages.len(), 3, "one message each for warn/error/fatal; the clean op is silent");
    assert_eq!(messages[0].level, crate::os_dsl::Severity::Warning);
    assert_eq!(messages[0].op_index, Some(1));
    assert_eq!(messages[1].level, crate::os_dsl::Severity::Error);
    assert_eq!(messages[1].op_index, Some(2));
    assert_eq!(messages[2].level, crate::os_dsl::Severity::Fatal);
    assert_eq!(messages[2].op_index, Some(3));

    assert_eq!(store.envelope(), &before, "preview_wire is a pure dry run: the live store is byte-identical afterward");
    assert_eq!(store.snapshot().expect("snapshot unaffected"), DemoSnapshot { n: Some(0) }, "preview_wire never advances the live cursor");
}

#[semio_framework_async_macros::async_test]
async fn preview_wire_reports_a_fatal_message_for_undecodable_op_bytes_and_stops_there() {
    let envelope: ArtifactEnvelope<DemoSnapshot, SeverityMutation> = create_document_envelope("demo/v1", "preview-malformed", DemoSnapshot { n: Some(0) }, None);
    let store = ArtifactStore::new(envelope).await;
    let ops: Vec<Vec<u8>> = vec![SeverityMutation::SetN(SeveritySetN { n: 1 }).encode_op().expect("encode"), vec![0xff, 0xff, 0xff]];

    let messages = store.preview_wire(&ops).await;

    assert_eq!(messages.len(), 1, "the clean op is silent; the malformed op reports one structural message and stops");
    assert_eq!(messages[0].level, crate::os_dsl::Severity::Fatal);
    assert_eq!(messages[0].code.0, "mutation.invariant");
    assert_eq!(messages[0].op_index, Some(1));
}

#[semio_framework_async_macros::async_test]
async fn spr_round_trip_preserves_edit_messages_and_conflicts() {
    let initial = DemoSnapshot { n: Some(0) };
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", "durable-outcomes", initial.clone(), None)).await;
    store.set_merge_policy(crate::os_spr::MergePolicy::LaissezFaire);
    let receipt = store.dispatch(ArtifactCommand::Apply { mutations: vec![SeverityMutation::SetWarningN(SetWarningN { n: 2 })], description: None }).await.expect("warning is accepted");
    let edit_id = receipt.edit_ids.first().expect("one durable edit").clone();
    let messages = store.messages_for_edit(&edit_id).to_vec();
    let edit = store.envelope().vcs.edits.iter().find(|edit| edit.id == edit_id).expect("durable edit");
    let mutation_ids = stable_mutation_ids_for_edit(edit).await.expect("durable edit carries operation identity");
    let actors = vec![ActorId(edit.actor.clone().expect("durable edit has an actor"))];
    let timestamp = edit.mutation_meta.first().expect("durable edit carries timestamp").timestamp;
    let kind = crate::os_spr::ConflictKind::Degraded { edit_ids: vec![edit_id.clone()] };
    let artifact_id = ArtifactId(store.envelope().id.clone());
    let conflict_id = crate::os_spr::ConflictId::new(&kind, &artifact_id, &mutation_ids, &timestamp).await;
    store.0.envelope.conflicts.push(crate::os_spr::Conflict { id: conflict_id, kind, status: crate::os_spr::ConflictStatus::Open, messages: messages.clone(), actors, timestamp });

    let pack = initial.encode_pack();
    let spr = print_document_spr(store.envelope()).await.expect("outcome history encodes");
    let parsed = parse_document_spr::<DemoSnapshot, SeverityMutation>(&pack, &spr).await.expect("outcome history decodes");
    assert_eq!(parsed.envelope.edit_messages, store.envelope().edit_messages);
    assert_eq!(parsed.envelope.conflicts, store.envelope().conflicts);

    let restored = ArtifactStore::new(parsed.envelope).await;
    assert_eq!(restored.messages_for_edit(&edit_id), messages);
    assert_eq!(restored.conflicts(), store.conflicts());
}

#[semio_framework_async_macros::async_test]
async fn spr_parse_rejects_history_without_authoritative_operation_metadata() {
    let initial = DemoSnapshot { n: Some(0) };
    let history = crate::os_spr::HistoryLog {
        doc_id: "metadata-required".to_string(),
        schema: "demo/v1".to_string(),
        edits: vec![crate::os_spr::HistoryEdit {
            id: "edit-1".to_string(),
            actor: Some("author".to_string()),
            started_at: String::new(),
            finished_at: None,
            coalesce_key: None,
            description: None,
            ops: vec![crate::os_spr::OpPayload { text: None, binary: Some(SeverityMutation::SetN(SeveritySetN { n: 1 }).encode_op().expect("encode")) }],
            inverse: Vec::new(),
            meta: None,
        }],
        ..Default::default()
    };
    let spr = crate::os_spr::encode_history(&history, &crate::os_spr::EncodeOptions::default()).await.expect("encode fixture");
    let error = parse_document_spr::<DemoSnapshot, SeverityMutation>(&initial.encode_pack(), &spr).await.expect_err("authoritative history must never synthesize operation identity");
    assert!(error.message.contains("authoritative operation metadata"));
}
//#endregion 🔖️PreviewWireTests

//#region 🔖️TextFormatHelpers
#[semio_framework_async_macros::async_test]
async fn ops_author_conversion_drops_avatar_matching_the_ops_text_format() {
    let author = Author { id: "a1".into(), name: "Alice".into(), avatar: Some("http://example/a1.png".into()) };
    let round_tripped: Author = OpsAuthor::from(&author).into();
    assert_eq!(round_tripped, Author { id: "a1".into(), name: "Alice".into(), avatar: None }, "OpsAuthor never carries avatar — it is not part of the .ops text format");
}

#[semio_framework_async_macros::async_test]
async fn ops_header_line_checkpoint_round_trips_including_delimiter_and_quote_characters_in_authors() {
    let header = OpsHeaderLine::Checkpoint {
        id: "c1".to_string(),
        at: "18".to_string(),
        changes: vec!["ch1".to_string(), "ch2".to_string()],
        parent: None,
        by: vec![OpsAuthor { id: "a:1,x".to_string(), name: "Alice, A. \"the great\"".to_string() }, OpsAuthor { id: "b2".to_string(), name: "Bob".to_string() }],
        message: Some("first \"checkpoint\"".to_string()),
    };
    let printed = header.print_op();
    assert!(!printed.contains('\n'), "print_op must be one line: {printed:?}");
    assert!(!printed.contains("parent="), "an absent optional field must be omitted, not printed as a '-' placeholder: {printed}");
    let parsed = OpsHeaderLine::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op failed for {printed:?}: {e}"));
    assert_eq!(parsed, header, "OpsHeaderLine::Checkpoint round trip diverged for {printed:?}");
}

#[semio_framework_async_macros::async_test]
async fn ops_header_line_edit_round_trips_including_a_quoted_description() {
    let header = OpsHeaderLine::Edit { id: "e1".to_string(), sequence: 42, started: "1".to_string(), actor: None, finished: None, key: None, description: Some("hello \"world\"".to_string()) };
    let printed = header.print_op();
    assert!(!printed.contains('\n'), "print_op must be one line: {printed:?}");
    assert!(!printed.contains("actor="), "an absent optional field must be omitted: {printed}");
    let parsed = OpsHeaderLine::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op failed for {printed:?}: {e}"));
    assert_eq!(parsed, header, "OpsHeaderLine::Edit round trip diverged for {printed:?}");
}

#[semio_framework_async_macros::async_test]
async fn ops_header_line_cursor_round_trips_the_full_applied_and_redo_lists() {
    let header = OpsHeaderLine::Cursor { applied: vec!["e1".to_string(), "e3".to_string()], redo: vec!["e2".to_string()], checkpoint: Some("ck-1".to_string()) };
    let printed = header.print_op();
    assert!(!printed.contains('\n'), "print_op must be one line: {printed:?}");
    let parsed = OpsHeaderLine::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op failed for {printed:?}: {e}"));
    assert_eq!(parsed, header, "OpsHeaderLine::Cursor round trip diverged for {printed:?}");
}

#[semio_framework_async_macros::async_test]
async fn ops_header_line_parse_op_rejects_a_line_with_no_known_keyword() {
    let error = OpsHeaderLine::parse_op("not a structural line").unwrap_err();
    assert!(error.message.contains("unknown operation line"), "got {error:?}");
}

#[semio_framework_async_macros::async_test]
async fn parse_document_text_rejects_a_header_line_missing_its_required_positional_id() {
    let files = ArtifactTextFiles { dsl: "n=0\n".to_string(), ops: "active\n".to_string() };
    let error = parse_document_text::<DemoSnapshot, DemoMutation>(&files.dsl, &files.ops).await.unwrap_err();
    assert!(error.message.contains("expected Text"), "got {error:?}");
    assert_eq!(error.span.line, 1);
}

#[semio_framework_async_macros::async_test]
async fn parse_document_text_rejects_an_unknown_header_line_keyword() {
    let files = ArtifactTextFiles { dsl: "n=0\n".to_string(), ops: "doc demo schema=demo/v1\nbogus id=x\n".to_string() };
    let error = parse_document_text::<DemoSnapshot, DemoMutation>(&files.dsl, &files.ops).await.unwrap_err();
    assert!(error.message.contains("unknown operation line"), "got {error:?}");
    assert_eq!(error.span.line, 2);
}

#[semio_framework_async_macros::async_test]
async fn document_text_round_trips_with_an_active_alternative_and_a_quoted_description() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: Some("said \"hi\" and used a \\ backslash".into()) }).await.expect("apply");
    store.dispatch(ArtifactCommand::CreateAlternative { name: "branch \"a\"".into() }).await.expect("create alternative (auto-commits and activates it)");
    assert!(store.envelope().active_alternative_id.is_some(), "precondition: an alternative is active");
    let files = print_document_text(store.envelope()).await.expect("print document text");
    assert!(files.ops.lines().any(|line| line.starts_with("active ")), "an active alternative must print an `active` header line: {}", files.ops);
    test_support::assert_document_text_round_trip(&store).await;
    test_support::assert_document_pack_round_trip(&store).await;
}

#[semio_framework_async_macros::async_test]
async fn document_text_round_trips_a_cursor_after_undo_then_apply_interleaving() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply e1");
    store.dispatch(ArtifactCommand::Undo).await.expect("undo e1");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], description: None }).await.expect("apply e2");
    // e1 (undone, in redo) precedes e2 (applied) in file order — exactly the interleaving a
    // single tail-edit marker cannot represent (see HistoryCursor's doc).
    assert_eq!(store.applied_edit_ids().len(), 1, "only e2 is applied");
    let files = print_document_text(store.envelope()).await.expect("print document text");
    assert!(files.ops.lines().any(|line| line.starts_with("cursor ")), "a synced cursor must print a `cursor` header line: {}", files.ops);
    let parsed = parse_document_text::<DemoSnapshot, DemoMutation>(&files.dsl, &files.ops).await.unwrap_or_else(|error| panic!("parse document text failed: {error}"));
    assert_eq!(parsed.envelope.cursor, store.envelope().cursor.clone(), "cursor diverged across a print/parse round trip");
    assert_eq!(parsed.snapshot.n, Some(2), "restored snapshot must reflect only the applied edit (e2), not both");
}

/// @emoji 🔐️ The save→load→undo proof (contract's runtime-behavior requirement): a store's
/// undo/redo position survives a full pack+spr save/load cycle, not just its snapshot value.
#[semio_framework_async_macros::async_test]
async fn save_load_undo_proof_pack_spr_round_trip_preserves_undo_redo_position() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply e1");
    let post_e1 = store.snapshot().expect("post-e1 snapshot");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], description: None }).await.expect("apply e2");
    let post_e2 = store.snapshot().expect("post-e2 snapshot");
    store.dispatch(ArtifactCommand::Undo).await.expect("undo e2");
    assert_eq!(store.snapshot().expect("live snapshot"), post_e1, "precondition: live store is back at post-e1");
    test_support::assert_live_equals_replay(&store).await;

    // Save: print_document_pack persists pack (initial snapshot) + spr (real inverse/meta,
    // AND the cursor reflecting exactly "e1 applied, e2 in redo").
    let pack_files = print_document_pack(store.envelope()).await.expect("print document pack");
    assert!(!pack_files.spr.is_empty(), "spr bytes must be non-empty once an edit exists");

    // Load: a FRESH store built only from persisted bytes — no access to the original `store`.
    let parsed: ParsedDocumentText<DemoSnapshot, DemoMutation> = parse_document_pack(&pack_files.pack, &pack_files.spr).await.unwrap_or_else(|error| panic!("parse document pack failed: {error}"));
    assert_eq!(parsed.snapshot, post_e1, "loaded snapshot must equal post-e1, proving undo position survived the save");
    let mut reloaded = ArtifactStore::new(parsed.envelope).await;
    assert_eq!(reloaded.snapshot().expect("reloaded snapshot"), post_e1, "ArtifactStore::new must seed live state from the persisted cursor");
    assert_eq!(reloaded.applied_edit_ids(), store.applied_edit_ids(), "applied_edit_ids must survive the round trip");
    test_support::assert_live_equals_replay(&reloaded).await;

    // Redo restores e2 — proving the redo stack (not just applied_edit_ids) survived.
    reloaded.dispatch(ArtifactCommand::Redo).await.expect("redo e2 after reload");
    assert_eq!(reloaded.snapshot().expect("post-redo snapshot"), post_e2);
    test_support::assert_live_equals_replay(&reloaded).await;

    // Undo twice from here reaches the true initial state.
    reloaded.dispatch(ArtifactCommand::Undo).await.expect("undo e2 again");
    reloaded.dispatch(ArtifactCommand::Undo).await.expect("undo e1");
    assert_eq!(reloaded.snapshot().expect("final snapshot"), DemoSnapshot { n: Some(0) });
    test_support::assert_live_equals_replay(&reloaded).await;
}

#[semio_framework_async_macros::async_test]
async fn document_codecs_share_complete_authoritative_history_validation() {
    let envelope = create_document_envelope("demo/v1", "codec-validation", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("checkpoint".into()), authors: Vec::new() }).await.expect("commit");
    let text = print_document_text(store.envelope()).await.expect("text fixture");
    let duplicate_change = text.ops.lines().find(|line| line.starts_with("change ")).expect("one persisted change");
    let malformed_text = format!("{}\n{duplicate_change}", text.ops);
    assert!(matches!(parse_document_text::<DemoSnapshot, DemoMutation>(&text.dsl, &malformed_text).await, Err(error) if error.message.contains("repeats authoritative change")));

    let pack = print_document_pack(store.envelope()).await.expect("binary fixture");
    let mut history = crate::os_spr::decode_history(&pack.spr, &crate::os_spr::DecodeOptions::default()).await.expect("decode history");
    history.changes.push(history.changes.first().expect("one persisted change").clone());
    let malformed_spr = crate::os_spr::encode_history(&history, &crate::os_spr::EncodeOptions { write_backwards_section: true, ..crate::os_spr::EncodeOptions::default() }).await.expect("encode malformed history");
    assert!(matches!(parse_document_spr::<DemoSnapshot, DemoMutation>(&pack.pack, &malformed_spr).await, Err(error) if error.message.contains("repeats authoritative change")));
}

//#endregion 🔖️TextFormatHelpers

//#region 🔖️CommandErrorPaths
#[semio_framework_async_macros::async_test]
async fn apply_with_no_mutations_is_rejected() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    let error = store.dispatch(ArtifactCommand::Apply { mutations: Vec::new(), description: None }).await.unwrap_err();
    assert_eq!(error, VcsError::EmptyApply);
}

#[semio_framework_async_macros::async_test]
async fn amend_last_with_no_mutations_is_rejected() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    let error = store.dispatch(ArtifactCommand::AmendLast { mutations: Vec::new(), coalesce_key: None }).await.unwrap_err();
    assert_eq!(error, VcsError::EmptyApply);
}

#[semio_framework_async_macros::async_test]
async fn undo_with_nothing_applied_is_rejected() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    assert_eq!(store.dispatch(ArtifactCommand::Undo).await.unwrap_err(), VcsError::NothingToUndo);
}

#[semio_framework_async_macros::async_test]
async fn redo_with_nothing_undone_is_rejected() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    assert_eq!(store.dispatch(ArtifactCommand::Redo).await.unwrap_err(), VcsError::NothingToRedo);
}

#[semio_framework_async_macros::async_test]
async fn checkout_of_an_unknown_checkpoint_is_rejected() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    let error = store.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: "nope".into() }).await.unwrap_err();
    assert_eq!(error, VcsError::UnknownChange("nope".into()));
}

#[semio_framework_async_macros::async_test]
async fn switch_to_an_unknown_alternative_is_rejected() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    let error = store.dispatch(ArtifactCommand::SwitchAlternative { alternative_id: "nope".into() }).await.unwrap_err();
    assert_eq!(error, VcsError::UnknownAlternative("nope".into()));
}

#[semio_framework_async_macros::async_test]
async fn malformed_alternative_checkpoint_pin_is_rejected_at_construction() {
    let mut envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    envelope.vcs.alternatives.try_push(Alternative { id: "alt-dangling".into(), name: "dangling".into(), checkpoint_ids: vec!["checkpoint-that-was-never-recorded".into()] }).expect("test alternative fits the fixed history ledger");
    let error = match super::ArtifactStore::<DemoSnapshot, DemoMutation>::new(envelope).await {
        Ok(_) => panic!("the alternative's pinned checkpoint id must actually exist"),
        Err(error) => error,
    };
    assert!(matches!(error, VcsError::ValidationFailed(message) if message.contains("alt-dangling") && message.contains("checkpoint-that-was-never-recorded")));
}

#[semio_framework_async_macros::async_test]
async fn create_alternative_with_no_edits_and_no_checkpoints_is_rejected() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    let error = store.dispatch(ArtifactCommand::CreateAlternative { name: "x".into() }).await.unwrap_err();
    assert_eq!(error, VcsError::NoCheckpoint, "the auto-commit has nothing pending, so there is still no checkpoint to branch from");
}

#[semio_framework_async_macros::async_test]
async fn compensating_undo_without_a_semantic_command_is_rejected() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply");
    let error = store.dispatch(ArtifactCommand::UndoWithPolicy { policy: UndoPolicy::CompensatingAction, semantic_command: None }).await.unwrap_err();
    assert!(matches!(error, VcsError::Backbone(_)), "got {error:?}");
}

#[semio_framework_async_macros::async_test]
async fn materialize_document_snapshot_rejects_an_unknown_edit_id() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let error = materialize_document_snapshot(&envelope, &["missing-edit".to_string()]).await.unwrap_err();
    assert_eq!(error, VcsError::UnknownEdit("missing-edit".into()));
}

#[semio_framework_async_macros::async_test]
async fn dispatch_text_applies_a_command_block_and_snapshot_json_reflects_it() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    let command_text = print_command(&ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 7 })], description: None }).await.expect("print command");
    store.dispatch_text(&command_text).await.expect("dispatch text");
    assert_eq!(store.snapshot_json().expect("snapshot json"), serde_json::to_string(&DemoSnapshot { n: Some(7) }).unwrap());

    let error = store.dispatch_text("not a command").await.unwrap_err();
    assert!(matches!(error, VcsError::Deserialize(_)), "got {error:?}");
}

#[semio_framework_async_macros::async_test]
async fn dispatch_binary_applies_an_encoded_command_and_rejects_wrong_format() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    let command_bytes = ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 7 })], description: None }.encode_op().expect("encode command");
    store.dispatch_binary(&command_bytes).await.expect("dispatch binary");
    assert_eq!(store.snapshot_json().expect("snapshot json"), serde_json::to_string(&DemoSnapshot { n: Some(7) }).unwrap());

    let mut wrong_format = command_bytes.clone();
    wrong_format[0] = 9;
    let error = store.dispatch_binary(&wrong_format).await.unwrap_err();
    assert!(matches!(error, VcsError::Deserialize(_)), "got {error:?}");
}

#[semio_framework_async_macros::async_test]
async fn command_text_binary_equivalence_holds_for_every_document_command_variant() {
    let commands: Vec<ArtifactCommand<DemoMutation>> = vec![
        ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 7 })], description: Some("set n".to_string()) },
        ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 7 })], description: None },
        ArtifactCommand::Undo,
        ArtifactCommand::Redo,
        ArtifactCommand::UndoWithPolicy { policy: UndoPolicy::ExactBaseOnly, semantic_command: None },
        ArtifactCommand::UndoWithPolicy { policy: UndoPolicy::TransformAgainstConcurrent, semantic_command: None },
        ArtifactCommand::UndoWithPolicy { policy: UndoPolicy::CompensatingAction, semantic_command: Some(Box::new(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 0 })], description: None })) },
        ArtifactCommand::CommitCheckpoint { message: Some("checkpoint".to_string()), authors: vec![Author { id: "u1".to_string(), name: "Ueli Saluz".to_string(), avatar: None }] },
        ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() },
        ArtifactCommand::CreateAlternative { name: "branch".to_string() },
        ArtifactCommand::SwitchAlternative { alternative_id: "alt-1".to_string() },
        ArtifactCommand::CheckoutCheckpoint { checkpoint_id: "ck-1".to_string() },
        ArtifactCommand::AmendLast { mutations: vec![DemoMutation::SetN(SetN { n: 3 })], coalesce_key: Some("drag".to_string()) },
        ArtifactCommand::ApplyInLane { mutations: vec![DemoMutation::SetN(SetN { n: 9 })], description: Some("select".to_string()), lane: HistoryLane::Interaction },
        ArtifactCommand::ApplyInLane { mutations: vec![DemoMutation::SetN(SetN { n: 9 })], description: None, lane: HistoryLane::Document },
        ArtifactCommand::AmendLastInLane { mutations: vec![DemoMutation::SetN(SetN { n: 4 })], coalesce_key: Some("hover".to_string()), lane: HistoryLane::Interaction },
        ArtifactCommand::UndoInLane { lane: HistoryLane::Interaction },
        ArtifactCommand::RedoInLane { lane: HistoryLane::Interaction },
    ];
    for command in &commands {
        test_support::assert_command_text_binary_equivalence(command).await;
    }
}

//#endregion 🔖️CommandErrorPaths

//#region 🔖️ReconcileAlternative
#[semio_framework_async_macros::async_test]
async fn reconcile_alternative_requires_an_existing_checkpoint() {
    let mut envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let error = reconcile_alternative(&mut envelope, "reconciled", None, Vec::new()).await.unwrap_err();
    assert_eq!(error, VcsError::NoCheckpoint);
}

#[semio_framework_async_macros::async_test]
async fn reconcile_alternative_pins_the_latest_checkpoint_and_optionally_records_a_reconciliation_checkpoint() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("c1".into()), authors: Vec::new() }).await.expect("commit");
    let base_checkpoint_id = store.envelope().vcs.checkpoints[0].id.clone();

    let mut without_message = owned_test_envelope(&store).await;
    let alt_id = reconcile_alternative(&mut without_message, "no-record", None, Vec::new()).await.expect("reconcile without message");
    assert_eq!(without_message.vcs.alternatives.last().unwrap().checkpoint_ids, vec![base_checkpoint_id.clone()]);
    assert_eq!(without_message.vcs.checkpoints.len(), 1, "no checkpoint_message means no new checkpoint is recorded");
    assert!(!alt_id.is_empty());

    let mut with_message = owned_test_envelope(&store).await;
    let authors = vec![Author { id: "a1".into(), name: "Alice".into(), avatar: None }];
    reconcile_alternative(&mut with_message, "recorded", Some("merged concurrent work".into()), authors.clone()).await.expect("reconcile with message");
    assert_eq!(with_message.vcs.checkpoints.len(), 2, "a checkpoint_message appends one reconciliation checkpoint");
    let recorded_checkpoint = with_message.vcs.checkpoints.last().unwrap();
    assert_eq!(recorded_checkpoint.parent_id, Some(base_checkpoint_id));
    assert_eq!(recorded_checkpoint.authors, authors);
    assert_eq!(recorded_checkpoint.message, Some("reconciled".into()), "the reconciliation checkpoint's own message is fixed, distinct from the change description");
    assert_eq!(with_message.vcs.changes.last().unwrap().description, Some("merged concurrent work".into()), "the passed checkpoint_message becomes the change's description");
}

#[semio_framework_async_macros::async_test]
async fn commit_checkpoint_mints_distinct_content_addressed_ids_for_distinct_commits() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply 1");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("first".into()), authors: Vec::new() }).await.expect("commit 1");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], description: None }).await.expect("apply 2");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("second".into()), authors: Vec::new() }).await.expect("commit 2");

    let ids: Vec<&str> = store.envelope().vcs.checkpoints.iter().map(|checkpoint| checkpoint.id.as_str()).collect();
    assert_eq!(ids.len(), 2);
    assert_ne!(ids[0], ids[1], "two distinct commits must mint two distinct checkpoint ids");
    assert!(ids.iter().all(|id| id.starts_with("ck-")));
}

#[semio_framework_async_macros::async_test]
async fn merge_base_finds_the_nearest_common_ancestor_across_a_fork() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply root");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("root".into()), authors: Vec::new() }).await.expect("commit root");
    let root_id = store.envelope().vcs.checkpoints[0].id.clone();

    store.dispatch(ArtifactCommand::CreateAlternative { name: "feature-a".into() }).await.expect("create feature-a");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], description: None }).await.expect("apply a");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("a1".into()), authors: Vec::new() }).await.expect("commit a1");
    let a1_id = store.envelope().vcs.checkpoints.last().unwrap().id.clone();

    store.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: root_id.clone() }).await.expect("checkout root");
    store.dispatch(ArtifactCommand::CreateAlternative { name: "feature-b".into() }).await.expect("create feature-b");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 3 })], description: None }).await.expect("apply b");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("b1".into()), authors: Vec::new() }).await.expect("commit b1");
    let b1_id = store.envelope().vcs.checkpoints.last().unwrap().id.clone();

    assert_eq!(merge_base(store.envelope(), &a1_id, &b1_id).await, Some(root_id.clone()), "a1 and b1 forked at root");
    assert_eq!(merge_base(store.envelope(), &a1_id, &root_id).await, Some(root_id.clone()), "root is its own descendant's merge-base");
    assert_eq!(merge_base(store.envelope(), &root_id, &root_id).await, Some(root_id), "a checkpoint is its own merge-base");
}

#[semio_framework_async_macros::async_test]
async fn merge_base_is_none_for_a_dangling_unknown_checkpoint_id() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("root".into()), authors: Vec::new() }).await.expect("commit");
    let root_id = store.envelope().vcs.checkpoints[0].id.clone();

    assert_eq!(merge_base(store.envelope(), &root_id, "unknown-checkpoint").await, None, "an id absent from the checkpoint list shares no ancestry with anything");
}

//#endregion 🔖️ContentAddressedCheckpointAndMergeBase

//#region 🔖️RemoteSnapshotMerge
#[semio_framework_async_macros::async_test]
async fn snapshot_merge_into_a_nonempty_store_adds_only_the_new_remote_edits_and_records() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("local apply");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("local".into()), authors: Vec::new() }).await.expect("local commit");

    let mut remote_store = ArtifactStore::new(owned_test_envelope(&store).await).await;
    remote_store.reset(owned_test_envelope(&store).await, store.applied_edit_ids().to_vec(), Vec::new()).await.expect("reset remote");
    remote_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], description: None }).await.expect("remote apply");
    remote_store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("remote".into()), authors: Vec::new() }).await.expect("remote commit");

    let (channel, remote_end) = ChannelBackbone::pair("chan").await;
    store.attach_backbone(Backbones::Channel(channel)).await.expect("attach");
    let _ = drain_channel_for_test(&remote_end).expect("drain attach snapshot");
    let remote_files = remote_store.snapshot_pack().await.expect("remote snapshot");
    remote_end.push(BackboneMessage::Snapshot { pack: remote_files.pack, spr: remote_files.spr }).await.expect("push snapshot");
    store.tick().await.expect("tick merges the pushed snapshot");

    assert_eq!(store.envelope().vcs.edits.len(), 2, "the shared original edit is deduped, only the new remote edit is added");
    assert_eq!(store.envelope().vcs.checkpoints.len(), 2, "the remote's new checkpoint is merged in by id");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(2), "current folds in the newly merged edit's forwards");
}

//#endregion 🔖️RemoteSnapshotMerge

//#region 🔖️SpaceMemberCheckoutRouting
#[semio_framework_async_macros::async_test]
async fn space_member_checkout_switches_at_the_alternative_tip_and_falls_back_to_checkout_when_stale() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply");
    store.dispatch(ArtifactCommand::CreateAlternative { name: "feature".into() }).await.expect("create alternative (auto-commits since no checkpoint existed yet)");
    let alt_id = store.envelope().vcs.alternatives[0].id.clone();
    let tip = store.envelope().vcs.alternatives[0].checkpoint_ids.last().expect("alt has a tip").clone();

    SpaceMember::checkout(&mut store, &tip, &alt_id).await.expect("checkout at the tip routes through SwitchAlternative");
    assert_eq!(store.envelope().active_alternative_id, Some(alt_id.clone()), "switching to the tip keeps it active");

    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], description: None }).await.expect("apply on branch");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("c2".into()), authors: Vec::new() }).await.expect("commit c2, advancing the alt's tip past `tip`");

    SpaceMember::checkout(&mut store, &tip, &alt_id).await.expect("checkout of the now-stale tip falls back to CheckoutCheckpoint");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(1), "restored the old checkpoint's state");
    assert_eq!(store.envelope().active_alternative_id, None, "the checked-out checkpoint is no longer any alternative's tip, so nothing is active");
}

//#endregion 🔖️SpaceMemberCheckoutRouting

//#region 🔖️BackbonePorts
#[semio_framework_async_macros::async_test]
async fn memory_backbone_port_round_trips_and_reports_a_missing_file() {
    let port = MemoryBackbonePort::new().await;
    let error = port.read("file://nowhere").await.unwrap_err();
    assert!(matches!(error, VcsError::Backbone(_)), "got {error:?}");
    port.write("file://a", "payload-1").await.expect("write");
    assert_eq!(port.read("file://a").await.expect("read"), "payload-1");
    port.write("file://a", "payload-2").await.expect("overwrite");
    assert_eq!(port.read("file://a").await.expect("read after overwrite"), "payload-2", "write is an upsert");
}

#[semio_framework_async_macros::async_test]
async fn local_storage_backbone_port_falls_back_to_its_in_memory_store() {
    let port = LocalStorageBackbonePort::new().await;
    let error = port.read("local://missing").await.unwrap_err();
    assert!(matches!(error, VcsError::Backbone(_)), "got {error:?}");
    port.write("local://a", "value").await.expect("write falls back to the in-memory store");
    assert_eq!(port.read("local://a").await.expect("read falls back too"), "value");

    let defaulted = LocalStorageBackbonePort::default();
    assert!(defaulted.read("local://a").await.is_err(), "Default constructs its own independent fallback store");
}

//#endregion 🔖️BackbonePorts

//#region 🔖️PackValueFixtures
async fn pack_value_fixture_corpus() -> Vec<(&'static str, DslValue)> {
    vec![
        ("null", DslValue::Null),
        ("bool_true", DslValue::Bool(true)),
        ("bool_false", DslValue::Bool(false)),
        ("int_zero", DslValue::uint(0)),
        ("int_negative_one", DslValue::int(-1)),
        ("float_pi", DslValue::float(3.14)),
        ("float_whole_number", DslValue::float(2.0)),
        ("string_empty", DslValue::String(String::new())),
        ("string_escapes", DslValue::String("hello\nworld with \"quotes\"".into())),
        ("array_empty", DslValue::Array(vec![])),
        ("array_ints", DslValue::Array(vec![DslValue::uint(1), DslValue::uint(2), DslValue::uint(3)])),
        ("object_empty", DslValue::Object(vec![])),
        ("object_mixed", DslValue::object([("a".into(), DslValue::uint(1)), ("b".into(), DslValue::Array(vec![DslValue::Bool(true), DslValue::Null]))])),
        (
            "nested_deep",
            DslValue::object([("a".into(), DslValue::object([("b".into(), DslValue::object([("c".into(), DslValue::Array(vec![DslValue::uint(1), DslValue::uint(2), DslValue::object([("d".into(), DslValue::String("leaf".into()))])]))]))]))]),
        ),
    ]
}

/// @emoji 🎯️ Variant-exact `DslValue` equality. `Number`'s own `PartialEq` calls a fitting
/// `UInt`/`Int` pair equal, so it cannot witness that the wire preserved the WRITER's variant —
/// which is exactly what `TAG_UINT` versus `TAG_INT` decides, and what the canonical bytes and
/// every hash over them depend on. `Float` compares by bits so `-0.0` and `NaN` are exact too.
// 🚫️async: E1 pure recursive comparison consumed inside sync `.all()`/`.is_some_and()` closures
// that cannot themselves be async — see R9/R10 residue shape 1
fn dsl_value_variant_exact_eq(a: &DslValue, b: &DslValue) -> bool {
    match (a, b) {
        (DslValue::Number(crate::os_dsl::Number::UInt(x)), DslValue::Number(crate::os_dsl::Number::UInt(y))) => x == y,
        (DslValue::Number(crate::os_dsl::Number::Int(x)), DslValue::Number(crate::os_dsl::Number::Int(y))) => x == y,
        (DslValue::Number(crate::os_dsl::Number::Float(x)), DslValue::Number(crate::os_dsl::Number::Float(y))) => x.to_bits() == y.to_bits(),
        (DslValue::Number(_), DslValue::Number(_)) => false,
        (DslValue::Array(x), DslValue::Array(y)) => x.len() == y.len() && x.iter().zip(y).all(|(a, b)| dsl_value_variant_exact_eq(a, b)),
        (DslValue::Object(x), DslValue::Object(y)) => x.len() == y.len() && x.iter().all(|(k, v)| y.iter().find(|(ok, _)| ok == k).is_some_and(|(_, ov)| dsl_value_variant_exact_eq(v, ov))),
        _ => a == b,
    }
}

/// @emoji 🔣️ Reads one `semio.pack.dynamic-integer/v1` value node — integers arrive as exact
/// decimal STRINGS and floats as little-endian hex, so no fixture number ever passes through a
/// JSON `f64` on its way into the corpus.
// 🚫️async: E1 pure recursive fixture reader consumed inside sync iterator closures — see R9
fn dynamic_integer_fixture_value(node: &serde_json::Value) -> DslValue {
    if let Some(text) = node.get("uint").and_then(|v| v.as_str()) {
        return DslValue::uint(text.parse::<u64>().expect("corpus uint is an exact u64"));
    }
    if let Some(text) = node.get("int").and_then(|v| v.as_str()) {
        return DslValue::int(text.parse::<i64>().expect("corpus int is an exact i64"));
    }
    if let Some(text) = node.get("f64LeHex").and_then(|v| v.as_str()) {
        let mut bits = [0u8; 8];
        for (index, byte) in bits.iter_mut().enumerate() {
            *byte = u8::from_str_radix(&text[index * 2..index * 2 + 2], 16).expect("corpus f64LeHex byte");
        }
        return DslValue::Number(crate::os_dsl::Number::Float(f64::from_le_bytes(bits)));
    }
    if let Some(items) = node.get("list").and_then(|v| v.as_array()) {
        return DslValue::Array(items.iter().map(dynamic_integer_fixture_value).collect());
    }
    if let Some(entries) = node.get("map").and_then(|v| v.as_array()) {
        return DslValue::Object(entries.iter().map(|entry| (entry[0].as_str().expect("corpus map key").to_string(), dynamic_integer_fixture_value(&entry[1]))).collect());
    }
    panic!("unsupported corpus value node {node}");
}

// 🚫️async: E1 pure hex reader consumed inside sync iterator closures — see R9
fn dynamic_integer_fixture_bytes(hex: &str) -> Vec<u8> {
    (0..hex.len() / 2).map(|index| u8::from_str_radix(&hex[index * 2..index * 2 + 2], 16).expect("corpus wireHex byte")).collect()
}

/// @emoji 🧾️ Hex-dumps `pack_rt::encode_pack_value` over a representative `DslValue`
/// corpus — ground truth for `HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS`'s TS
/// `PackValueCodec` mirror (`framework/product/os/ts/index.ts`). Run with `--nocapture` to
/// capture the printed `name -> hex` lines; also asserts `decode_pack_value(encode_pack_value(v))
/// == v` for every entry so the corpus is never accidentally out of date with the real codec.
#[semio_framework_async_macros::async_test]
async fn pack_value_fixture_corpus_hex_dump() {
    for (name, value) in pack_value_fixture_corpus().await {
        let bytes = pack_rt::encode_pack_value(&value);
        let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        println!("[pack_value_fixture] {name} ({} bytes) -> {hex}", bytes.len());
        let decoded = pack_rt::decode_pack_value(&bytes).expect("decode_pack_value");
        assert!(dsl_value_variant_exact_eq(&decoded, &value), "round-trip mismatch for fixture {name}: {decoded:?} != {value:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn interaction_state_pack_matches_first_party_value_and_json_oracle() {
    let oracle: serde_json::Value = serde_json::from_str(include_str!("../../🎒️interaction-state-pack.json")).unwrap();
    let value = DslValue::from(&oracle);
    let state = <protocol::InteractionState as FromValue>::from_value(value.clone()).unwrap();
    let encoded = <protocol::InteractionState as ArtifactPack>::encode_pack(&state);
    assert_eq!(encoded, <DslValue as ArtifactPack>::encode_pack(&value));
    assert_eq!(encoded, <serde_json::Value as ArtifactPack>::encode_pack(&oracle));
    let decoded = <protocol::InteractionState as ArtifactPack>::decode_pack(&encoded).unwrap();
    assert_eq!(decoded, state);
    assert_eq!(serde_json::Value::from(&<protocol::InteractionState as ToValue>::to_value(&decoded)), oracle);
}

/// @emoji 🪶️ Hex-dumps `pack_rt::encode_wire_value` over the SAME fixture corpus — ground
/// truth for the container-less wire codec mirror in TS.
#[semio_framework_async_macros::async_test]
async fn pack_wire_value_fixture_corpus_hex_dump() {
    for (name, value) in pack_value_fixture_corpus().await {
        let bytes = pack_rt::encode_wire_value(&value);
        let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        println!("[pack_wire_value_fixture] {name} ({} bytes) -> {hex}", bytes.len());
        let decoded = pack_rt::decode_wire_value(&bytes).expect("decode_wire_value");
        assert!(dsl_value_variant_exact_eq(&decoded, &value), "round-trip mismatch for fixture {name}: {decoded:?} != {value:?}");
    }
}

/// @emoji 🔢️ Exact-variant law for the dynamic `Shape::Value` grammar over the shared,
/// language-neutral `semio.pack.dynamic-integer/v1` corpus
/// (`💻️os/🧫️fixtures/🎒️pack-dynamic-integer-v1`, whose `wireHex` is independently generated
/// by a third-party LEB128 oracle). Asserts the emitted TAG, the exact `Number` variant, and
/// byte equality — `DslValue` equality alone cannot witness any of the three, because
/// `Number::PartialEq` calls `UInt(7)` and `Int(7)` equal while their wire tags differ.
#[semio_framework_async_macros::async_test]
async fn pack_wire_value_preserves_integer_variants_at_u64_i64_boundaries() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🎒️pack-dynamic-integer-v1/🔣️.json")).expect("dynamic integer corpus parses");
    assert_eq!(corpus["schema"].as_str(), Some("semio.pack.dynamic-integer/v1"));
    assert_eq!(corpus["version"].as_u64(), Some(1));
    let accept = corpus["accept"].as_array().expect("accept rows");
    assert_eq!(accept.len(), 6, "the corpus is bounded at six accept rows");

    for row in accept {
        let id = row["id"].as_str().expect("row id");
        let value = dynamic_integer_fixture_value(&row["value"]);
        let expected = dynamic_integer_fixture_bytes(row["wireHex"].as_str().expect("row wireHex"));
        let bytes = pack_rt::encode_wire_value(&value);
        assert_eq!(bytes, expected, "{id}: encode_wire_value must be byte-exact against the neutral corpus");
        let tag = bytes[4];
        let expected_tag = match row["variant"].as_str() {
            Some("uint") => 0x04u8,
            Some("int") => 0x03,
            Some("float") => 0x05,
            Some("map") => 0x10,
            other => panic!("{id}: unsupported corpus variant {other:?}"),
        };
        assert_eq!(tag, expected_tag, "{id}: the dynamic value's leading tag carries the writer's variant");
        let decoded = pack_rt::decode_wire_value(&bytes).expect("decode_wire_value");
        assert!(dsl_value_variant_exact_eq(&decoded, &value), "{id}: {decoded:?} is not variant-exact against {value:?}");
        assert_eq!(pack_rt::encode_wire_value(&decoded), expected, "{id}: re-encoding the decoded value is canonical and idempotent");
    }

    assert!(matches!(pack_rt::decode_wire_value(&dynamic_integer_fixture_bytes("000101110403")).expect("uint 3"), DslValue::Number(crate::os_dsl::Number::UInt(3))));
    assert!(matches!(pack_rt::decode_wire_value(&pack_rt::encode_wire_value(&DslValue::int(7))).expect("int 7"), DslValue::Number(crate::os_dsl::Number::Int(7))), "a positive Int keeps TAG_INT rather than collapsing onto TAG_UINT");
    assert_ne!(pack_rt::encode_wire_value(&DslValue::int(7)), pack_rt::encode_wire_value(&DslValue::uint(7)), "Int(7) and UInt(7) are equal under Number::PartialEq but are distinct on the wire");
    let beyond_exact_f64 = 9_007_199_254_740_993u64;
    assert_ne!(beyond_exact_f64 as f64 as u64, beyond_exact_f64, "the f64 widening this law replaced is not injective past 2^53");

    for row in corpus["reject"].as_array().expect("reject rows") {
        let id = row["id"].as_str().expect("row id");
        let bytes = dynamic_integer_fixture_bytes(row["wireHex"].as_str().expect("row wireHex"));
        match row["outcome"].as_str() {
            Some("decode-error") => {
                assert!(pack_rt::decode_wire_value(&bytes).is_err(), "{id}: a truncated or overlong varint must fail the reader");
            }
            Some("noncanonical") => {
                let decoded = pack_rt::decode_wire_value(&bytes).expect("the shared permissive varint reader admits redundant encodings");
                assert!(dsl_value_variant_exact_eq(&decoded, &dynamic_integer_fixture_value(&row["decodes"])), "{id}: {decoded:?} is not the documented decode");
                assert_ne!(pack_rt::encode_wire_value(&decoded), bytes, "{id}: a canonical-bytes caller rejects it because encode(decode(bytes)) != bytes");
            }
            other => panic!("{id}: unsupported corpus outcome {other:?}"),
        }
    }
}
//#endregion 🔖️PackValueFixtures

//#region 🔖️CompositionTests

impl OpText for ValidatedMutation {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = crate::os_dsl::parse(line, &spec_fn(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Inline })?;
                return <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(crate::os_dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        crate::os_dsl::print(&record, &spec_fn(), crate::os_dsl::JoinMode::Inline)
    }
}

impl OpBinary for ValidatedMutation {
    fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(crate::os_spr::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1)();
        let body = crate::os_pack::encode_record_body(&spec, &record, &PackEncodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        crate::os_pack::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = crate::os_pack::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(crate::os_spr::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(crate::os_spr::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = crate::os_pack::decode_record_body(body, &spec, &PackDecodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
        let record_offset = reader.position() as u64;
        <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| crate::os_spr::ProtocolError::Malformed { what: "op record", offset: record_offset, detail: error.to_string() })
    }
}

/// @emoji 🛂️ The composition fixture's negative value is a fatal outcome, so the dry-run
/// rejects it before group dispatch can alter either member's history.

impl MemberStoreOwner<ValidatedMutation> for DemoSnapshot {
    type SnapshotOpen = UnsupportedMemberSnapshotOpen<Self>;

    fn member_store_owners() -> MemberStoreOwners<Self, ValidatedMutation> {
        MemberStoreOwners::new(Arc::new(DemoSnapshotRetirementFactory), Arc::new(DemoInitialSnapshotRetirementFactory), Arc::new(DemoMutationRetirementFactory), Box::new(DemoStoreOwnedDisposer::<ValidatedMutation>(PhantomData)))
    }
}

/// 🎯️ The dialect every composition fixture below mints children under.
fn demo_child_dialect() -> crate::os_io::ArtifactDialect {
    crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.mesh".into(), standard: "1".into(), subset: "*".into() }
}

#[semio_framework_async_macros::async_test]
async fn artifact_child_dsl_field_round_trips_via_pack_and_value() {
    let target = crate::os_io::ArtifactRef { artifact_id: "child-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.mesh".into(), standard: "87a".into(), subset: "mesh".into() } };
    let child: ArtifactChild<DemoSnapshot> = ArtifactChild::new("child-1".into(), target);

    let spec = artifact_child_spec();
    let record = artifact_child_to_record(&child);
    let bytes = crate::os_pack::encode_record_body(&spec, &record, &PackEncodeOptions::default()).expect("encode");
    let (decoded_record, _report) = crate::os_pack::decode_record_body(&bytes, &spec, &PackDecodeOptions::default()).expect("decode");
    let decoded: ArtifactChild<DemoSnapshot> = artifact_child_from_record(&decoded_record).expect("from_record");
    assert_eq!(decoded, child);

    let value = <ArtifactChild<DemoSnapshot> as crate::os_dsl::DslField>::to_value(&child);
    let via_field = <ArtifactChild<DemoSnapshot> as crate::os_dsl::DslField>::from_value(&value).expect("from_value");
    assert_eq!(via_field, child);

    assert_eq!(child.to_child_ref("mesh-slot").await, ChildRef { slot: "mesh-slot".into(), child_id: "child-1".into(), target: child.target.clone() });
}

#[semio_framework_async_macros::async_test]
async fn artifact_child_required_owner_distinguishes_absence_and_wrong_type() {
    let target = crate::os_io::ArtifactRef { artifact_id: "child-1".into(), dialect: demo_child_dialect() };
    let absent: ArtifactChild<DemoSnapshot> = ArtifactChild::new("child-1".into(), target.clone());
    assert_eq!(absent.require_local_owner::<String>(), Err(ArtifactChildMaterializationError::Absent));

    let typed: ArtifactChild<DemoSnapshot> = ArtifactChild::new("child-1".into(), target).with_local_owner(Arc::new(String::from("content")));
    assert_eq!(typed.require_local_owner::<u64>(), Err(ArtifactChildMaterializationError::WrongType));
    assert_eq!(typed.require_local_owner::<String>().unwrap().as_str(), "content");
}

#[semio_framework_async_macros::async_test]
async fn owner_ref_dsl_field_round_trips_via_pack() {
    let owner = OwnerRef {
        parent: crate::os_io::ArtifactRef { artifact_id: "parent-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.object".into(), standard: "1".into(), subset: "*".into() } },
        slot: "mesh-slot".into(),
        child_id: "child-1".into(),
    };
    let spec = owner_ref_spec();
    let record = owner_ref_to_record(&owner);
    let bytes = crate::os_pack::encode_record_body(&spec, &record, &PackEncodeOptions::default()).expect("encode");
    let (decoded_record, _report) = crate::os_pack::decode_record_body(&bytes, &spec, &PackDecodeOptions::default()).expect("decode");
    let decoded = owner_ref_from_record(&decoded_record).expect("from_record");
    assert_eq!(decoded, owner);
}

#[semio_framework_async_macros::async_test]
async fn artifact_link_dsl_field_round_trips_every_link_pin_variant() {
    let target = crate::os_io::ArtifactRef { artifact_id: "linked-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.image".into(), standard: "1".into(), subset: "*".into() } };
    let pins = vec![LinkPin::Head, LinkPin::Checkpoint { id: "ck-abc123".into() }, LinkPin::Snapshot { blob: BlobRef { hash: "deadbeef".into(), size: 42, media_type: "image/png".into() } }];
    for pin in pins {
        let link = ArtifactLink { target: target.clone(), pin: pin.clone(), role: "cover-image".into() };
        let spec = artifact_link_spec();
        let record = artifact_link_to_record(&link);
        let bytes = crate::os_pack::encode_record_body(&spec, &record, &PackEncodeOptions::default()).expect("encode");
        let (decoded_record, _report) = crate::os_pack::decode_record_body(&bytes, &spec, &PackDecodeOptions::default()).expect("decode");
        let decoded = artifact_link_from_record(&decoded_record).expect("from_record");
        assert_eq!(decoded, link, "round trip diverged for pin variant {pin:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn artifact_refs_defaults_to_empty_for_a_leaf_snapshot() {
    struct LeafSnapshot;
    impl ArtifactRefs for LeafSnapshot {}
    let snapshot = LeafSnapshot;
    assert!(snapshot.child_refs().await.is_empty());
    assert!(snapshot.links().await.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn typed_child_store_factory_round_trips_a_child_through_create_persist_open() {
    let dialect = demo_child_dialect();

    let mut child = create_member_store::<DemoSnapshot, DemoMutation>("demo/v1", "child-round-trip", &dialect, &DemoSnapshot { n: Some(7) }.encode_pack()).await.expect("create");
    assert!(child.snapshot_retirement_factory.is_some(), "generated member creation must install the exact snapshot retirement factory before returning ownership");
    assert!(child.owned_disposer_installed(), "generated member creation must install the exact whole-store bounded disposer before returning ownership");
    child.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 9 })], description: Some("bump".into()) }).await.expect("apply");
    child.dispatch(ArtifactCommand::Undo).await.expect("undo");
    child.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 11 })], description: None }).await.expect("re-apply");

    let files = print_document_pack(child.envelope()).await.expect("print");
    let persisted = encode_document_pack_bytes(&files.pack, &files.spr).await;
    let expected = crate::os_io::ArtifactRef { artifact_id: "child-round-trip".into(), dialect };
    let mut reopened = open_member_store::<DemoSnapshot, DemoMutation>("demo/v1", &expected, None, &persisted).await.expect("open");

    assert!(reopened.snapshot_retirement_factory.is_some(), "generated member reopen must install the exact snapshot retirement factory before returning ownership");
    assert!(reopened.owned_disposer_installed(), "generated member reopen must install the exact whole-store bounded disposer before returning ownership");
    assert_eq!(reopened.envelope().id, "child-round-trip");
    assert_eq!(reopened.snapshot().expect("head snapshot"), child.snapshot().expect("head snapshot"), "reopened child's live content diverged from the persisted one");
    assert_eq!(reopened.snapshot().expect("head snapshot"), DemoSnapshot { n: Some(11) }, "reopen restored the wrong cursor position");
    close_member_dialect_fixture(&mut reopened);
    close_member_dialect_fixture(&mut child);
}

#[semio_framework_async_macros::async_test]
async fn member_factory_wrapper_cannot_bypass_exact_create_or_open_owner_catalog() {
    type Wrapped = ArtifactStore<DemoSnapshot, DemoMutation>;
    let dialect = demo_child_dialect();
    let mut created = <Wrapped as MemberFactory>::create("wrapped-member", &dialect, &DemoSnapshot { n: Some(3) }.encode_pack()).await.expect("wrapper create");
    assert!(created.snapshot_retirement_factory.is_some());
    assert!(created.initial_snapshot_retirement_factory.is_some());
    assert!(created.mutation_retirement_factory.is_some());
    assert!(created.owned_disposer_installed());
    let files = print_document_pack(created.envelope()).await.expect("print wrapped member");
    let persisted = encode_document_pack_bytes(&files.pack, &files.spr).await;
    let expected = crate::os_io::ArtifactRef { artifact_id: "wrapped-member".into(), dialect };
    let mut reopened = <Wrapped as MemberFactory>::open(&expected, None, &persisted).await.expect("wrapper open");
    assert!(reopened.snapshot_retirement_factory.is_some());
    assert!(reopened.initial_snapshot_retirement_factory.is_some());
    assert!(reopened.mutation_retirement_factory.is_some());
    assert!(reopened.owned_disposer_installed());
    close_member_dialect_fixture(&mut reopened);
    close_member_dialect_fixture(&mut created);
}

#[semio_framework_async_macros::async_test]
async fn member_close_rejects_missing_owner_and_preserves_the_installed_disposer_until_terminal() {
    let mut missing = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "missing-member-owner", DemoSnapshot { n: Some(1) }, None)).await;
    assert!(matches!(SpaceMember::close_owned_step(&mut missing, 1, 4_096), Err(reason) if reason.contains("no owner-supplied bounded disposer")));

    missing.install_member_store_owners_exact(DemoSnapshot::member_store_owners());
    assert!(matches!(SpaceMember::close_owned_step(&mut missing, 1, 4_096), Ok(SnapshotRetirementStep::Pending { released_items, released_bytes }) if released_items <= 1 && released_bytes <= 4_096));
    assert!(missing.owned_disposer_installed(), "the admitted owner remains retained until terminal close");
    assert!(!SpaceMember::close_owned_terminal_is_empty(&missing));
    assert!(
        matches!(missing.install_snapshot_retirement_factory(Arc::new(DemoSnapshotRetirementFactory)), Err(VcsError::Deserialize(reason)) if reason.contains("already installed")),
        "a second or wrong snapshot retirement factory must never replace the exact retained owner"
    );
    close_member_dialect_fixture(&mut missing);
}

#[semio_framework_async_macros::async_test]
async fn typed_child_store_factory_rejects_empty_genesis_and_dialect_less_owned_child() {
    assert!(matches!(create_member_store::<DemoSnapshot, DemoMutation>("demo/v1", "child-empty", &demo_child_dialect(), &[]).await, Err(VcsError::Deserialize(_))), "an empty genesis pack must never silently default");

    // 🏠️ owner ⇒ dialect: an envelope that is somebody's child but names no dialect cannot be
    // typed by its parent, so `open` must fail closed rather than hand back an untypable member.
    let mut envelope = create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "child-no-dialect", DemoSnapshot { n: Some(1) }, None);
    envelope.owner = Some(OwnerRef { parent: crate::os_io::ArtifactRef { artifact_id: "parent".into(), dialect: demo_child_dialect() }, slot: "mesh".into(), child_id: "child-no-dialect".into() });
    let files = print_document_pack(&envelope).await.expect("print");
    let orphan = encode_document_pack_bytes(&files.pack, &files.spr).await;
    let expected = crate::os_io::ArtifactRef { artifact_id: "child-no-dialect".into(), dialect: demo_child_dialect() };
    assert!(matches!(open_member_store::<DemoSnapshot, DemoMutation>("demo/v1", &expected, envelope.owner.as_ref(), &orphan).await, Err(VcsError::Deserialize(_))), "an owned child with no dialect must fail closed");
}

#[semio_framework_async_macros::async_test]
async fn pack_at_checkpoint_reads_history_without_moving_the_live_cursor() {
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "pinned-target", DemoSnapshot { n: Some(1) }, None)).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], description: None }).await.expect("apply");
    let checkpoint = SpaceMember::commit_checkpoint(&mut store, "v1".into(), Vec::new()).await.expect("checkpoint");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 3 })], description: None }).await.expect("apply after checkpoint");

    let historical = DemoSnapshot::decode_pack(&store.pack_at_checkpoint(&checkpoint).await.expect("pack at checkpoint")).expect("decode");
    let live = DemoSnapshot::decode_pack(&store.document_pack_bytes().await.expect("head pack")).expect("decode");
    assert_eq!(historical, DemoSnapshot { n: Some(2) }, "checkpoint read did not return the pinned content");
    assert_eq!(live, DemoSnapshot { n: Some(3) }, "reading a checkpoint moved the live cursor");
    assert!(matches!(store.pack_at_checkpoint("no-such-checkpoint").await, Err(VcsError::UnknownChange(_))));
}

#[semio_framework_async_macros::async_test]
async fn member_link_resolver_resolves_head_checkpoint_and_degrades_snapshot_pins() {
    struct FixtureDirectory {
        member: ArtifactStore<DemoSnapshot, DemoMutation>,
    }
    impl MemberDirectory for FixtureDirectory {
        async fn head_pack(&self, artifact_id: &str) -> Option<Result<Vec<u8>, VcsError>> {
            if artifact_id == "linked-doc" {
                Some(self.member.document_pack_bytes().await)
            } else {
                None
            }
        }
        async fn checkpoint_pack(&self, artifact_id: &str, checkpoint_id: &str) -> Option<Result<Vec<u8>, VcsError>> {
            if artifact_id == "linked-doc" {
                Some(self.member.pack_at_checkpoint(checkpoint_id).await)
            } else {
                None
            }
        }
    }

    let mut member = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "linked-doc", DemoSnapshot { n: Some(1) }, None)).await;
    member.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 42 })], description: None }).await.expect("apply");
    let pinned = SpaceMember::commit_checkpoint(&mut member, "pinned".into(), Vec::new()).await.expect("checkpoint");
    member.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 99 })], description: None }).await.expect("apply after pin");

    let resolver = MemberLinkResolver::new(FixtureDirectory { member }).await;
    let target = crate::os_io::ArtifactRef { artifact_id: "linked-doc".into(), dialect: demo_child_dialect() };
    let link_of = |pin: LinkPin| ArtifactLink { target: target.clone(), pin, role: "representation".into() };
    let decode = async |state: LinkState| match state {
        LinkState::Resolved { pack_bytes, .. } => DemoSnapshot::decode_pack(&pack_bytes).expect("decode"),
        other => panic!("expected Resolved, found {other:?}"),
    };

    assert_eq!(decode(resolver.resolve(&link_of(LinkPin::Head)).await).await, DemoSnapshot { n: Some(99) }, "a Head pin must follow the target's live tip");
    assert_eq!(decode(resolver.resolve(&link_of(LinkPin::Checkpoint { id: pinned })).await).await, DemoSnapshot { n: Some(42) }, "a Checkpoint pin must keep resolving to the pinned history, not the tip");

    let blob = BlobRef { hash: "deadbeef".into(), size: 3, media_type: "application/octet-stream".into() };
    assert!(matches!(resolver.resolve(&link_of(LinkPin::Snapshot { blob })).await, LinkState::PinnedOnly { .. }), "a snapshot pin with no blob store must degrade to PinnedOnly, never Missing");

    let absent = ArtifactLink { target: crate::os_io::ArtifactRef { artifact_id: "gone".into(), dialect: demo_child_dialect() }, pin: LinkPin::Head, role: "representation".into() };
    assert_eq!(resolver.resolve(&absent).await, LinkState::Missing);
}

#[semio_framework_async_macros::async_test]
async fn link_resolver_reports_resolved_missing_and_pinned_only_states() {
    struct DemoResolver;
    impl LinkResolver for DemoResolver {
        async fn resolve(&self, link: &ArtifactLink) -> LinkState {
            match link.role.as_str() {
                "known" => LinkState::Resolved { pack_bytes: vec![1, 2, 3], dialect: link.target.dialect.clone() },
                "pinned" => LinkState::PinnedOnly { blob: BlobRef { hash: "deadbeef".into(), size: 3, media_type: "application/octet-stream".into() } },
                _ => LinkState::Missing,
            }
        }
    }
    let target = crate::os_io::ArtifactRef { artifact_id: "linked-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.image".into(), standard: "1".into(), subset: "*".into() } };
    let resolver = DemoResolver;
    assert!(matches!(resolver.resolve(&ArtifactLink { target: target.clone(), pin: LinkPin::Head, role: "known".into() }).await, LinkState::Resolved { .. }));
    assert!(matches!(resolver.resolve(&ArtifactLink { target: target.clone(), pin: LinkPin::Head, role: "pinned".into() }).await, LinkState::PinnedOnly { .. }));
    assert!(matches!(resolver.resolve(&ArtifactLink { target, pin: LinkPin::Head, role: "gone".into() }).await, LinkState::Missing));
}

#[semio_framework_async_macros::async_test]
async fn member_factory_closed_dialect_parent_projection_matches_neutral_corpus() {
    use crate::os_schema_composition::{ArtifactCompositionFields, ChildRefFields, ChildRefVisitor, ChildSlotSpec};
    struct Parent<'a>(&'a [serde_json::Value]);
    fn fields(row: &serde_json::Value) -> ChildRefFields<'_> {
        ChildRefFields {
            child_id: row["childId"].as_str().unwrap(),
            artifact_id: row["artifactId"].as_str().unwrap(),
            artifact_kind: row["artifactKind"].as_str().unwrap(),
            standard: row["standard"].as_str().unwrap(),
            subset: row["subset"].as_str().unwrap(),
        }
    }
    impl ArtifactCompositionFields for Parent<'_> {
        fn child_slots() -> &'static [ChildSlotSpec] {
            &[ChildSlotSpec { name: "single", kind: "s.test.member", many: false }, ChildSlotSpec { name: "many", kind: "s.test.member", many: true }, ChildSlotSpec { name: "other", kind: "s.test.member", many: false }]
        }
        fn visit_child_refs<'a, V: ChildRefVisitor<'a>>(&'a self, visitor: &mut V) -> Result<(), V::Error> {
            for row in self.0 {
                visitor.step()?;
                let slot = match row["slot"].as_str().unwrap() {
                    "single" => "single",
                    "many" => "many",
                    "other" => "other",
                    _ => "unknown",
                };
                visitor.child(slot, fields(row))?;
            }
            Ok(())
        }
    }
    let fixture = member_dialect_fixture();
    let slots = Parent::child_slots();
    for (actual, expected) in slots.iter().zip(fixture["projectionSlots"].as_array().unwrap()) {
        assert_eq!(actual.name, expected["name"].as_str().unwrap());
        assert_eq!(actual.kind, expected["kind"].as_str().unwrap());
        assert_eq!(actual.many, expected["many"].as_bool().unwrap());
    }
    let rows = fixture["projectionCases"].as_array().unwrap();
    assert_eq!(rows.len(), 15);
    for row in rows {
        let parent = Parent(row["parent"].as_array().unwrap());
        let projection = ChildRestoreProjection::from_snapshot(&parent);
        assert_eq!(projection.is_ok(), row["projected"].as_bool().unwrap(), "{}", row["id"]);
        let admitted = projection.as_ref().is_ok_and(|projection| projection.admit_complete(row["incoming"].as_array().unwrap().iter().map(|row| (row["slot"].as_str().unwrap(), fields(row)))).is_ok());
        assert_eq!(admitted, row["accepted"].as_bool().unwrap(), "{}", row["id"]);
    }
    eprintln!("[DEBUG] member parent projection: 15 neutral loaded-parent and complete-set admission rows");
}

#[semio_framework_async_macros::async_test]
async fn member_factory_closed_dialect_graph_admission_matches_neutral_corpus() {
    let fixture = member_dialect_fixture();
    let rows = fixture["graphCases"].as_array().expect("neutral graph rows");
    assert_eq!(rows.len(), 7);
    for row in rows {
        let id = row["id"].as_str().unwrap();
        let mut graph = CompositionGraph::new().await;
        for edge in row["edges"].as_array().unwrap() {
            graph.insert_owns(edge["parent"].as_str().unwrap(), edge["slot"].as_str().unwrap(), edge["child"].as_str().unwrap()).await.expect("valid initial forest");
        }
        let previous = graph.owns.clone();
        let candidate = &row["candidate"];
        let parent = candidate["parent"].as_str().unwrap();
        let slot = candidate["slot"].as_str().unwrap();
        let child = candidate["child"].as_str().unwrap();
        let admission = graph.admit_owns(parent, slot, child).await;
        assert_eq!(admission.is_ok(), row["accepted"].as_bool().unwrap(), "{id}");
        assert_eq!(graph.owns, previous, "{id}: admission is read-only");
        if let Ok(admission) = admission {
            assert_eq!(admission.inserts(), row["inserted"].as_bool().unwrap(), "{id}");
            graph.commit_owns_admitted(admission);
            assert_eq!(graph.owner_of(child).await, Some(parent), "{id}");
            assert_eq!(graph.slot_of(child).await, Some(slot), "{id}");
            assert!(!graph.admit_owns(parent, slot, child).await.unwrap().inserts(), "{id}: exact repeat cannot erase or replace an existing edge");
        }
    }
    let graph = CompositionGraph::new().await;
    let mut foreign = CompositionGraph::new().await;
    let ticket = graph.admit_owns("parent", "slot", "child").await.unwrap();
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| foreign.commit_owns_admitted(ticket))).is_err());
    assert!(foreign.owns.is_empty());
    let mut graph = CompositionGraph::new().await;
    let ticket = graph.admit_owns("parent", "slot", "child").await.unwrap();
    graph.insert_owns("child", "ancestor", "parent").await.unwrap();
    let previous = graph.owns.clone();
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| graph.commit_owns_admitted(ticket))).is_err());
    assert_eq!(graph.owns, previous);
    eprintln!("[DEBUG] member graph native admission: {} neutral vectors", rows.len());
}

#[semio_framework_async_macros::async_test]
async fn member_factory_closed_dialect_graph_sync_preserves_prior_state_on_rejection() {
    struct Refs {
        children: Vec<ChildRef>,
        links: Vec<ArtifactLink>,
    }
    impl ArtifactRefs for Refs {
        async fn child_refs(&self) -> Vec<ChildRef> {
            self.children.clone()
        }
        async fn links(&self) -> Vec<ArtifactLink> {
            self.links.clone()
        }
    }
    let child_ref = |slot: &str, child: &str| ChildRef { slot: slot.into(), child_id: child.into(), target: crate::os_io::ArtifactRef { artifact_id: child.into(), dialect: demo_child_dialect() } };
    let link_ref = |target: &str| ArtifactLink { target: crate::os_io::ArtifactRef { artifact_id: target.into(), dialect: demo_child_dialect() }, pin: LinkPin::Head, role: "reference".into() };
    let fixture = member_dialect_fixture();
    let rows = fixture["graphCases"].as_array().unwrap();
    assert_eq!(rows.len(), 7);
    for row in rows {
        let id = row["id"].as_str().unwrap();
        let parent = row["candidate"]["parent"].as_str().unwrap();
        let child = row["candidate"]["child"].as_str().unwrap();
        let slot = row["candidate"]["slot"].as_str().unwrap();
        let mut graph = CompositionGraph::new().await;
        for edge in row["edges"].as_array().unwrap() {
            graph.insert_owns(edge["parent"].as_str().unwrap(), edge["slot"].as_str().unwrap(), edge["child"].as_str().unwrap()).await.unwrap();
        }
        graph.insert_owns(parent, "previous", "retained-prior-child").await.unwrap();
        graph.insert_link(parent, "retained-prior-link").await.unwrap();
        let prior_owns = graph.owns.clone();
        let prior_links = graph.links.clone();
        let prior_generation = graph.owns_generation;
        let refs = Refs { children: vec![child_ref(slot, child)], links: Vec::new() };
        let result = graph.sync_member(parent, &refs).await;
        assert_eq!(result.is_ok(), row["syncAccepted"].as_bool().unwrap(), "{id}");
        if result.is_ok() {
            let mut expected = prior_owns;
            expected.retain(|_, (owner, _)| owner != parent);
            expected.insert(child.into(), (parent.into(), slot.into()));
            assert_eq!(graph.owns, expected, "{id}");
            assert!(!graph.links.contains_key(parent));
        } else {
            assert_eq!(graph.owns, prior_owns, "{id}");
            assert_eq!(graph.links, prior_links, "{id}");
            assert_eq!(graph.owns_generation, prior_generation, "{id}");
        }
    }
    let mut graph = CompositionGraph::new().await;
    graph.insert_owns("parent", "previous", "previous-child").await.unwrap();
    graph.insert_link("cycle-source", "parent").await.unwrap();
    graph.insert_link("parent", "previous-link").await.unwrap();
    let prior_owns = graph.owns.clone();
    let prior_links = graph.links.clone();
    for refs in [Refs { children: vec![child_ref("next", "next-child")], links: vec![link_ref("cycle-source")] }, Refs { children: vec![child_ref("first", "duplicated"), child_ref("second", "duplicated")], links: Vec::new() }] {
        assert!(graph.sync_member("parent", &refs).await.is_err());
        assert_eq!(graph.owns, prior_owns);
        assert_eq!(graph.links, prior_links);
    }
    eprintln!("[DEBUG] graph sync: seven neutral replacement vectors and late link/child-across-slots rejection preserve prior graph state");
}

#[semio_framework_async_macros::async_test]
async fn composition_graph_owns_forest_rejects_second_owner_and_cycle() {
    let mut graph = CompositionGraph::new().await;
    graph.insert_owns("parent-a", "slot-1", "child-1").await.expect("first owner ok");
    graph.insert_owns("parent-a", "slot-1", "child-1").await.expect("idempotent re-own ok");

    let error = graph.insert_owns("parent-b", "slot-1", "child-1").await.expect_err("second owner must be rejected");
    assert!(error.contains("already owned"), "unexpected message: {error}");

    graph.insert_owns("child-1", "slot-x", "grandchild-1").await.expect("child owns grandchild ok");
    assert!(graph.would_cycle_owns("grandchild-1", "parent-a").await, "parent-a is an ancestor of grandchild-1 via child-1");
    let cycle_error = graph.insert_owns("grandchild-1", "slot-y", "parent-a").await.expect_err("cycle must be rejected");
    assert!(cycle_error.contains("cycle"), "unexpected message: {cycle_error}");

    assert_eq!(graph.owner_of("child-1").await, Some("parent-a"));
    assert_eq!(graph.slot_of("child-1").await, Some("slot-1"));
    graph.remove_owns("child-1").await;
    assert_eq!(graph.owner_of("child-1").await, None);
}

#[semio_framework_async_macros::async_test]
async fn composition_graph_links_reject_cycle_but_allow_converging_dag_edges() {
    let mut graph = CompositionGraph::new().await;
    graph.insert_link("a", "b").await.expect("a->b ok");
    graph.insert_link("b", "c").await.expect("b->c ok");
    graph.insert_link("a", "c").await.expect("a->c ok, a converging (non-cyclic) edge");

    let error = graph.insert_link("c", "a").await.expect_err("closing the loop must be rejected");
    assert!(error.contains("cycle"), "unexpected message: {error}");

    assert_eq!(HashSet::<String>::from_iter(graph.links_from("a").await), HashSet::from(["b".to_string(), "c".to_string()]));
    graph.remove_link("a", "b").await;
    assert_eq!(graph.links_from("a").await, vec!["c".to_string()]);
}

#[semio_framework_async_macros::async_test]
async fn mint_child_id_converges_across_two_replicas_and_varies_by_ordinal_and_slot() {
    let parent_id = "parent-1";
    let slot = "mesh-slot";
    let ops: Vec<Vec<u8>> = vec![DemoMutation::SetN(SetN { n: 7 }).encode_op().expect("encode")];
    let fingerprint_replica_1 = concat_ops_fingerprint(&ops).await;
    let fingerprint_replica_2 = concat_ops_fingerprint(&ops).await;
    assert_eq!(fingerprint_replica_1, fingerprint_replica_2, "identical ops must fingerprint identically");

    let id_replica_1 = mint_child_id(parent_id, slot, &fingerprint_replica_1, 0).await;
    let id_replica_2 = mint_child_id(parent_id, slot, &fingerprint_replica_2, 0).await;
    assert_eq!(id_replica_1, id_replica_2, "two replicas performing the identical genesis must converge on the identical child id");

    let id_different_ordinal = mint_child_id(parent_id, slot, &fingerprint_replica_1, 1).await;
    assert_ne!(id_replica_1, id_different_ordinal, "a different ordinal must mint a different id");

    let id_different_slot = mint_child_id(parent_id, "other-slot", &fingerprint_replica_1, 0).await;
    assert_ne!(id_replica_1, id_different_slot, "a different slot must mint a different id");
}

/// @emoji 🧪️ One fatal preview anywhere ⇒ nothing applied on ANY member, parent included.
#[semio_framework_async_macros::async_test]
async fn dispatch_group_validate_all_atomicity_one_bad_member_applies_nothing() {
    let parent_ref = crate::os_io::ArtifactRef { artifact_id: "parent-atomic-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let child_ref = crate::os_io::ArtifactRef { artifact_id: "child-atomic-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, ValidatedMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;
    let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, ValidatedMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;

    let mut coordinator = CompositionCoordinator::new().await;
    coordinator.graph_mut().await.insert_owns(&parent_ref.artifact_id, "slot-1", &child_ref.artifact_id).await.expect("seed ownership");

    let good_op = ValidatedMutation::SetN(ValidatedSetN { n: 5 }).encode_op().expect("encode good op");
    let bad_op = ValidatedMutation::SetN(ValidatedSetN { n: -1 }).encode_op().expect("encode bad op");
    let parent_ops = vec![good_op];
    let child_dispatch = ChildDispatch { child: child_ref.clone(), ops: vec![bad_op], op_schema: SchemaId("demo/v1".into()), labels: vec!["bad".into()] };
    let mut children = [(&mut child_store, child_dispatch)];

    let result = coordinator.dispatch_group(&parent_ref, &mut parent_store, &mut children, parent_ops, Vec::new(), GroupMeta::default());
    match result.await {
        Ok(_) => panic!("expected the group dispatch to fail phase-1 validation, but it succeeded"),
        // 🎞️ `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C6: an
        // ordinary mutation-level (message-based) rejection now travels as `VcsError::Rejected`,
        // not `ValidationFailed` (reserved for structural failures only — see its own doc
        // comment). The default policy (`Normal`, via `SpaceMember::merge_policy`'s trait
        // default, since neither member's policy is configured here) rejects Fatal.
        Err(VcsError::Rejected { policy, messages }) => {
            assert_eq!(policy, crate::os_spr::MergePolicy::Normal);
            assert!(messages.iter().any(|message| message.level == crate::os_dsl::Severity::Fatal));
        }
        Err(other) => panic!("expected Rejected, got a different VcsError: {other}"),
    }
    assert!(parent_store.envelope().vcs.edits.is_empty(), "parent must have zero edits after a failed group dispatch");
    assert!(child_store.envelope().vcs.edits.is_empty(), "child must have zero edits after a failed group dispatch");
}

/// @emoji 🧪️ TASK 2's ownership-check law: `dispatch_group` refuses to touch a `ChildDispatch`
/// whose claimed parent the coordinator's own `CompositionGraph` does not currently track —
/// zero side effects, same as any other phase-1 failure.
#[semio_framework_async_macros::async_test]
async fn dispatch_group_rejects_a_child_the_graph_does_not_track_as_owned() {
    let parent_ref = crate::os_io::ArtifactRef { artifact_id: "parent-unowned-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let child_ref = crate::os_io::ArtifactRef { artifact_id: "child-unowned-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, ValidatedMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;
    let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, ValidatedMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;
    let coordinator = CompositionCoordinator::new();
    // Deliberately NOT seeding `coordinator.graph_mut().insert_owns(..)` — the graph has no
    // record that `parent_ref` owns `child_ref`.

    let op = ValidatedMutation::SetN(ValidatedSetN { n: 1 }).encode_op().expect("encode");
    let child_dispatch = ChildDispatch { child: child_ref.clone(), ops: vec![op], op_schema: SchemaId("demo/v1".into()), labels: Vec::new() };
    let mut children = [(&mut child_store, child_dispatch)];

    let mut coordinator = coordinator.await;
    let result = coordinator.dispatch_group(&parent_ref, &mut parent_store, &mut children, Vec::new(), Vec::new(), GroupMeta::default());
    match result.await {
        Ok(_) => panic!("expected an OwnershipViolation, but the dispatch succeeded"),
        Err(VcsError::OwnershipViolation(_)) => {}
        Err(other) => panic!("expected OwnershipViolation, got a different VcsError: {other}"),
    }
    assert!(child_store.envelope().vcs.edits.is_empty());
}

/// @emoji 🧪️ Directly exercises `CompositionCoordinator::compensate` (private, visible to this
/// nested test module) — the reverse-order rollback `dispatch_group`'s phase 2 falls back to on
/// a late failure. Proves the order (parent first, then children in reverse dispatch order) and
/// that a clean rollback restores every member's pre-group snapshot.
#[semio_framework_async_macros::async_test]
async fn compensate_undoes_applied_members_in_reverse_order() {
    let parent_ref = crate::os_io::ArtifactRef { artifact_id: "parent-comp-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let child_a_ref = crate::os_io::ArtifactRef { artifact_id: "child-comp-a".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };
    let child_b_ref = crate::os_io::ArtifactRef { artifact_id: "child-comp-b".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;
    parent_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply parent");
    let parent_edit_id = parent_store.envelope().vcs.edits.last().expect("parent edit").id.clone();

    let mut child_a = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &child_a_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;
    child_a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], description: None }).await.expect("apply a");
    let child_a_edit_id = child_a.envelope().vcs.edits.last().expect("a edit").id.clone();

    let mut child_b = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &child_b_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;
    child_b.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 3 })], description: None }).await.expect("apply b");
    let child_b_edit_id = child_b.envelope().vcs.edits.last().expect("b edit").id.clone();

    let dispatch_a = ChildDispatch { child: child_a_ref.clone(), ops: Vec::new(), op_schema: SchemaId("demo/v1".into()), labels: Vec::new() };
    let dispatch_b = ChildDispatch { child: child_b_ref.clone(), ops: Vec::new(), op_schema: SchemaId("demo/v1".into()), labels: Vec::new() };
    let mut children = [(&mut child_a, dispatch_a), (&mut child_b, dispatch_b)];
    let applied_children = vec![(0usize, child_a_edit_id), (1usize, child_b_edit_id)];

    let report = CompositionCoordinator::compensate(&parent_ref, &mut parent_store, &mut children, &applied_children, Some(&parent_edit_id)).await;

    assert!(report.skipped.is_empty(), "every member should have undone cleanly");
    assert_eq!(report.undone.len(), 3);
    assert_eq!(report.undone[0].0.artifact_id, parent_ref.artifact_id, "parent undone first");
    assert_eq!(report.undone[1].0.artifact_id, child_b_ref.artifact_id, "then the LAST-dispatched child");
    assert_eq!(report.undone[2].0.artifact_id, child_a_ref.artifact_id, "then the first-dispatched child");

    assert_eq!(parent_store.snapshot().expect("parent snapshot").n, Some(0), "parent's edit was undone");
    assert_eq!(child_a.snapshot().expect("a snapshot").n, Some(0), "child a's edit was undone");
    assert_eq!(child_b.snapshot().expect("b snapshot").n, Some(0), "child b's edit was undone");
}

/// @emoji 🧪️ TASK 2's "if compensation itself fails" law: a member whose own rollback errors is
/// recorded in `GroupUndoReport.skipped` (never panics, never aborts compensating the rest),
/// and `fold_compensation_error` upgrades the original failure into `VcsError::CompensationFailed`
/// carrying both facts.
#[semio_framework_async_macros::async_test]
async fn compensate_reports_skipped_when_a_members_own_undo_fails_and_folds_to_compensation_failed() {
    let parent_ref = crate::os_io::ArtifactRef { artifact_id: "parent-comp-fail-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let child_ref = crate::os_io::ArtifactRef { artifact_id: "child-comp-fail-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    // Parent has NOTHING applied, so `parent.undo()` deterministically fails with
    // `NothingToUndo` — simulating a member whose own rollback errors mid-compensation.
    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;
    let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;
    child_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 4 })], description: None }).await.expect("apply child");
    let child_edit_id = child_store.envelope().vcs.edits.last().expect("child edit").id.clone();

    let dispatch = ChildDispatch { child: child_ref.clone(), ops: Vec::new(), op_schema: SchemaId("demo/v1".into()), labels: Vec::new() };
    let mut children = [(&mut child_store, dispatch)];
    let applied_children = vec![(0usize, child_edit_id)];

    let report = CompositionCoordinator::compensate(&parent_ref, &mut parent_store, &mut children, &applied_children, Some("bogus-parent-edit-id")).await;

    assert_eq!(report.skipped.len(), 1, "the parent's own failed undo must be recorded, not panic");
    assert_eq!(report.skipped[0].0.artifact_id, parent_ref.artifact_id);
    assert!(matches!(&report.skipped[0].1, VcsError::NothingToUndo));
    assert_eq!(report.undone.len(), 1, "the child must still be undone despite the parent's rollback failure");
    assert_eq!(child_store.snapshot().expect("child snapshot").n, Some(0));

    let folded = fold_compensation_error(VcsError::ValidationFailed("late failure".into()), report).await;
    assert!(matches!(folded, VcsError::CompensationFailed(_)), "a non-empty skipped list must fold into CompensationFailed, got {folded}");
}

/// @emoji 🧪️ TASK 2's "deterministic child id minting across two simulated replicas" law,
/// end-to-end through `dispatch_group` itself (not just the bare `mint_child_id` helper): two
/// independent coordinators/parents dispatching the IDENTICAL genesis converge on the identical
/// minted child id and the identical `invocation_id`.
#[semio_framework_async_macros::async_test]
async fn dispatch_group_mints_genesis_child_ids_deterministically_across_replicas() {
    let parent_ref = crate::os_io::ArtifactRef { artifact_id: "parent-genesis-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let genesis = vec![ChildGenesis { slot: "mesh-slot".into(), dialect: demo_child_dialect(), initial_pack: DemoSnapshot { n: Some(0) }.encode_pack() }];
    let parent_ops: Vec<Vec<u8>> = vec![DemoMutation::SetN(SetN { n: 1 }).encode_op().expect("encode")];

    let mut parent_1 = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;
    let mut coordinator_1 = CompositionCoordinator::new().await;
    // 🎯️ Empty array literal: with `dispatch_group`'s `Mp`/`Mc` split, `Mc` no longer has
    // `parent`'s type to piggyback inference off of, so an empty `children` list needs an
    // explicit element type — the same local test wrapper `parent_1` uses.
    let mut children_1: [(&mut ArtifactStore<DemoSnapshot, DemoMutation>, ChildDispatch); 0] = [];
    let receipt_1 = coordinator_1.dispatch_group(&parent_ref, &mut parent_1, &mut children_1, parent_ops.clone(), genesis.clone(), GroupMeta::default()).await.expect("replica 1 dispatch");

    let mut parent_2 = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;
    let coordinator_2 = CompositionCoordinator::new();
    let mut children_2: [(&mut ArtifactStore<DemoSnapshot, DemoMutation>, ChildDispatch); 0] = [];
    let receipt_2 = coordinator_2.await.dispatch_group(&parent_ref, &mut parent_2, &mut children_2, parent_ops, genesis, GroupMeta::default()).await.expect("replica 2 dispatch");

    assert_eq!(receipt_1.created_children.len(), 1);
    assert_eq!(receipt_2.created_children.len(), 1);
    assert_eq!(receipt_1.created_children[0].0.artifact_id, receipt_2.created_children[0].0.artifact_id, "two replicas performing the identical genesis must mint the identical child id");
    assert_eq!(receipt_1.invocation_id, receipt_2.invocation_id, "two replicas performing the identical composite gesture must converge on the identical invocation id");

    assert_eq!(parent_1.snapshot().expect("parent snapshot").n, Some(1));
    assert_eq!(coordinator_1.graph().await.owner_of(&receipt_1.created_children[0].0.artifact_id).await, Some(parent_ref.artifact_id.as_str()));
    assert_eq!(receipt_1.member_edits.len(), 1, "only the parent got a real edit here — no existing children were dispatched");
    assert_eq!(receipt_1.member_edits[0].0.artifact_id, parent_ref.artifact_id);
}

/// @emoji 🧪️ TASK 2's group-undo law: a member whose tail belongs to a DIFFERENT (foreign)
/// group is skipped, never aborting the rest of the group's undo.
#[semio_framework_async_macros::async_test]
async fn undo_group_skips_a_foreign_tail_member_but_still_undoes_the_rest() {
    let group_id = "group-xyz";
    let parent_ref = crate::os_io::ArtifactRef { artifact_id: "parent-undo-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let child_ref = crate::os_io::ArtifactRef { artifact_id: "child-undo-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };
    let foreign_ref = crate::os_io::ArtifactRef { artifact_id: "foreign-undo-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;
    parent_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply parent");
    parent_store.stamp_tail_group_id(group_id).await.expect("stamp parent");

    let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;
    child_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], description: None }).await.expect("apply child");
    child_store.stamp_tail_group_id(group_id).await.expect("stamp child");

    let mut foreign_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &foreign_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;
    foreign_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 3 })], description: None }).await.expect("apply foreign");
    foreign_store.stamp_tail_group_id("some-other-group").await.expect("stamp foreign");

    let mut children = [(&child_ref, &mut child_store), (&foreign_ref, &mut foreign_store)];

    let report = CompositionCoordinator::undo_group(&parent_ref, &mut parent_store, &mut children, group_id).await;

    assert_eq!(report.undone.len(), 2, "parent + child both belong to the group and must be undone");
    assert_eq!(report.skipped.len(), 1, "the foreign member must be skipped, not abort the group");
    assert_eq!(report.skipped[0].0.artifact_id, foreign_ref.artifact_id);
    assert!(matches!(&report.skipped[0].1, VcsError::ForeignEdit(_)));

    assert_eq!(parent_store.snapshot().expect("parent snapshot").n, Some(0), "parent's group edit was undone");
    assert_eq!(child_store.snapshot().expect("child snapshot").n, Some(0), "child's group edit was undone");
    assert_eq!(foreign_store.snapshot().expect("foreign snapshot").n, Some(3), "the foreign member's own edit must be left untouched");
}

/// @emoji 🧪️ `redo_group`'s mirror of the same law: a foreign-group member's redo stack is left
/// untouched while a matching member is reapplied.
#[semio_framework_async_macros::async_test]
async fn redo_group_skips_a_foreign_tail_member_but_still_redoes_the_rest() {
    let group_id = "group-redo-1";
    let parent_ref = crate::os_io::ArtifactRef { artifact_id: "parent-redo-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let foreign_ref = crate::os_io::ArtifactRef { artifact_id: "foreign-redo-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;
    parent_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply parent");
    parent_store.stamp_tail_group_id(group_id).await.expect("stamp parent");
    parent_store.dispatch(ArtifactCommand::Undo).await.expect("undo parent, seeding its redo stack");

    let mut foreign_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &foreign_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;
    foreign_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 9 })], description: None }).await.expect("apply foreign");
    foreign_store.stamp_tail_group_id("some-other-group").await.expect("stamp foreign");
    foreign_store.dispatch(ArtifactCommand::Undo).await.expect("undo foreign, seeding its redo stack");

    let mut children = [(&foreign_ref, &mut foreign_store)];
    let report = CompositionCoordinator::redo_group(&parent_ref, &mut parent_store, &mut children, group_id).await;

    assert_eq!(report.undone.len(), 1, "only the matching-group member is redone");
    assert_eq!(report.undone[0].0.artifact_id, parent_ref.artifact_id);
    assert_eq!(report.skipped.len(), 1);
    assert_eq!(report.skipped[0].0.artifact_id, foreign_ref.artifact_id);

    assert_eq!(parent_store.snapshot().expect("parent snapshot").n, Some(1), "parent's edit was reapplied");
    assert_eq!(foreign_store.snapshot().expect("foreign snapshot").n, Some(0), "the foreign member's redo stack was left untouched");
}

//#region 🔖️TransactionPeerTests
/// @emoji 🧪️ W1-C's headline `Peer`-relation law (contract-freeze §5): two artifacts with NO
/// ownership relation commit through `dispatch_peer_group` as ONE atomic transaction — both
/// members end up carrying the SAME `MutationMeta.group_id` (the shared minted
/// `invocation_id`), the peer's tail edit is stamped `MutationOrigin::Transaction { initiator }`,
/// and the initiator's own edit stays at its ordinary `Owner` default (it is not foreign to
/// itself). Deliberately does NOT seed any `CompositionGraph::insert_owns` edge — `Peer` never
/// consults `owner_of` at all.
#[semio_framework_async_macros::async_test]
async fn dispatch_peer_group_commits_both_members_with_one_shared_group_id() {
    let initiator_ref = crate::os_io::ArtifactRef { artifact_id: "peer-initiator-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let peer_ref = crate::os_io::ArtifactRef { artifact_id: "peer-member-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut initiator_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &initiator_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;
    let mut peer_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &peer_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;
    let coordinator = TransactionCoordinator::new();

    let initiator_ops: Vec<Vec<u8>> = vec![DemoMutation::SetN(SetN { n: 1 }).encode_op().expect("encode initiator op")];
    let peer_op = DemoMutation::SetN(SetN { n: 2 }).encode_op().expect("encode peer op");
    let peer_dispatch = ChildDispatch { child: peer_ref.clone(), ops: vec![peer_op], op_schema: SchemaId("demo/v1".into()), labels: vec!["peer".into()] };
    let mut peers = [(&mut peer_store, peer_dispatch)];

    let receipt = coordinator.await.dispatch_peer_group(&initiator_ref, &mut initiator_store, &mut peers, initiator_ops, GroupMeta::default()).await.expect("peer transaction dispatch");

    assert_eq!(receipt.member_edits.len(), 2, "both the initiator and the one peer got a real edit");
    assert_eq!(initiator_store.snapshot().expect("initiator snapshot").n, Some(1));
    assert_eq!(peer_store.snapshot().expect("peer snapshot").n, Some(2));

    let initiator_group_id = initiator_store.tail_group_id().await.expect("initiator tail group id");
    let peer_group_id = peer_store.tail_group_id().await.expect("peer tail group id");
    assert_eq!(initiator_group_id, peer_group_id, "both members share the SAME minted invocation id as their group id");
    assert_eq!(initiator_group_id, receipt.invocation_id);

    let initiator_origin = initiator_store.envelope().vcs.edits.last().expect("initiator edit").mutation_meta.last().expect("initiator meta").origin.clone();
    assert_eq!(initiator_origin, crate::os_spr::MutationOrigin::Owner, "the initiator's own edit is not foreign to itself, so it stays at the ordinary Owner default");

    let peer_origin = peer_store.envelope().vcs.edits.last().expect("peer edit").mutation_meta.last().expect("peer meta").origin.clone();
    match peer_origin {
        crate::os_spr::MutationOrigin::Transaction { initiator } => assert_eq!(initiator.artifact_id, initiator_ref.artifact_id, "the peer's origin names the real initiator"),
        other => panic!("expected MutationOrigin::Transaction on the peer's tail edit, got {other:?}"),
    }
}

/// @emoji 🧪️ W1-C's compensation law for `Peer`: `compensate` is relation-agnostic (see its own
/// doc comment) — exercising it with a two-PEER (no ownership) scenario proves the SAME
/// reverse-order rollback `dispatch_peer_group`'s phase 2 falls back to on a late failure works
/// identically to the `Owned` case `compensate_undoes_applied_members_in_reverse_order` already
/// covers. Peer B is deliberately left with no applied edit at all, modeling "the SECOND
/// member's own dispatch failed" — the exact trigger `dispatch_peer_group`'s real error branch
/// calls `compensate` from, passing only the members that DID get applied (the initiator + peer
/// A) as `applied_children`/`parent_applied`.
#[semio_framework_async_macros::async_test]
async fn compensate_undoes_applied_peer_members_in_reverse_order() {
    let initiator_ref = crate::os_io::ArtifactRef { artifact_id: "peer-comp-initiator".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let peer_a_ref = crate::os_io::ArtifactRef { artifact_id: "peer-comp-a".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };
    let peer_b_ref = crate::os_io::ArtifactRef { artifact_id: "peer-comp-b".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut initiator_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &initiator_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;
    initiator_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], description: None }).await.expect("apply initiator");
    let initiator_edit_id = initiator_store.envelope().vcs.edits.last().expect("initiator edit").id.clone();

    let mut peer_a = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &peer_a_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;
    peer_a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], description: None }).await.expect("apply peer a — the member the transaction already reached");
    let peer_a_edit_id = peer_a.envelope().vcs.edits.last().expect("a edit").id.clone();

    let mut peer_b = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &peer_b_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;

    let dispatch_a = ChildDispatch { child: peer_a_ref.clone(), ops: Vec::new(), op_schema: SchemaId("demo/v1".into()), labels: Vec::new() };
    let dispatch_b = ChildDispatch { child: peer_b_ref.clone(), ops: Vec::new(), op_schema: SchemaId("demo/v1".into()), labels: Vec::new() };
    let mut peers = [(&mut peer_a, dispatch_a), (&mut peer_b, dispatch_b)];
    let applied_so_far = vec![(0usize, peer_a_edit_id)];

    let report = TransactionCoordinator::compensate(&initiator_ref, &mut initiator_store, &mut peers, &applied_so_far, Some(&initiator_edit_id)).await;

    assert!(report.skipped.is_empty(), "every already-applied member should have undone cleanly");
    assert_eq!(report.undone.len(), 2, "the initiator and peer A (the only two that were actually applied) are both rolled back");
    assert_eq!(report.undone[0].0.artifact_id, initiator_ref.artifact_id, "initiator undone first");
    assert_eq!(report.undone[1].0.artifact_id, peer_a_ref.artifact_id, "then the one applied peer");

    assert_eq!(initiator_store.snapshot().expect("initiator snapshot").n, Some(0), "initiator's edit was compensated");
    assert_eq!(peer_a.snapshot().expect("a snapshot").n, Some(0), "peer A's edit was compensated");
}

/// @emoji 🧪️ Task 2's group-undo law, exercised through a REAL `Peer` transaction end-to-end:
/// `undo_group` (unmodified — see its own doc comment on being relation-agnostic) reverses BOTH
/// members of a `dispatch_peer_group` group as ONE, using the exact `invocation_id` that call
/// minted, with no code path specific to `Peer` needed.
#[semio_framework_async_macros::async_test]
async fn undo_group_reverses_both_members_of_a_real_peer_transaction() {
    let initiator_ref = crate::os_io::ArtifactRef { artifact_id: "peer-undo-initiator".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let peer_ref = crate::os_io::ArtifactRef { artifact_id: "peer-undo-member".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut initiator_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &initiator_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;
    let mut peer_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &peer_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;
    let coordinator = TransactionCoordinator::new();

    let initiator_ops = vec![DemoMutation::SetN(SetN { n: 5 }).encode_op().expect("encode initiator op")];
    let peer_op = DemoMutation::SetN(SetN { n: 7 }).encode_op().expect("encode peer op");
    let peer_dispatch = ChildDispatch { child: peer_ref.clone(), ops: vec![peer_op], op_schema: SchemaId("demo/v1".into()), labels: Vec::new() };
    let receipt = {
        let mut peers = [(&mut peer_store, peer_dispatch)];
        coordinator.await.dispatch_peer_group(&initiator_ref, &mut initiator_store, &mut peers, initiator_ops, GroupMeta::default()).await.expect("peer transaction dispatch")
    };
    assert_eq!(initiator_store.snapshot().expect("initiator snapshot").n, Some(5));
    assert_eq!(peer_store.snapshot().expect("peer snapshot").n, Some(7));

    let mut children = [(&peer_ref, &mut peer_store)];
    let report = TransactionCoordinator::undo_group(&initiator_ref, &mut initiator_store, &mut children, &receipt.invocation_id).await;

    assert!(report.skipped.is_empty(), "both real transaction members must belong to the group");
    assert_eq!(report.undone.len(), 2);
    assert_eq!(initiator_store.snapshot().expect("initiator snapshot after undo").n, Some(0));
    assert_eq!(peer_store.snapshot().expect("peer snapshot after undo").n, Some(0));
}

/// @emoji 🧪️ `Peer`'s cycle guard law (`MemberRelation::Peer`'s doc comment): a SECOND, separate
/// `dispatch_peer_group` call that would close a link cycle across the coordinator's persisted
/// `Links` graph is rejected — `CompositionGraph::would_cycle_links`, the same primitive
/// `would_cycle_owns` is to `Owned`'s genesis cycle guard. Transaction 1 (A initiates, B is the
/// peer) succeeds and records a `Links` edge A -> B; transaction 2 (B initiates, A is the peer)
/// would close A -> B -> A and is rejected with zero side effects.
#[semio_framework_async_macros::async_test]
async fn dispatch_peer_group_rejects_a_transaction_that_would_close_a_peer_link_cycle() {
    let artifact_a_ref = crate::os_io::ArtifactRef { artifact_id: "peer-cycle-a".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let artifact_b_ref = crate::os_io::ArtifactRef { artifact_id: "peer-cycle-b".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut store_a = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &artifact_a_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;
    let mut store_b = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &artifact_b_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;
    let mut coordinator = TransactionCoordinator::new().await;

    let op_ab = DemoMutation::SetN(SetN { n: 1 }).encode_op().expect("encode a->b op");
    let dispatch_ab = ChildDispatch { child: artifact_b_ref.clone(), ops: vec![op_ab], op_schema: SchemaId("demo/v1".into()), labels: Vec::new() };
    {
        let mut peers = [(&mut store_b, dispatch_ab)];
        coordinator.dispatch_peer_group(&artifact_a_ref, &mut store_a, &mut peers, Vec::new(), GroupMeta::default()).await.expect("first transaction A -> B");
    }
    assert_eq!(store_b.snapshot().expect("b snapshot").n, Some(1));

    let op_ba = DemoMutation::SetN(SetN { n: 9 }).encode_op().expect("encode b->a op");
    let dispatch_ba = ChildDispatch { child: artifact_a_ref.clone(), ops: vec![op_ba], op_schema: SchemaId("demo/v1".into()), labels: Vec::new() };
    let mut peers = [(&mut store_a, dispatch_ba)];
    let result = coordinator.dispatch_peer_group(&artifact_b_ref, &mut store_b, &mut peers, Vec::new(), GroupMeta::default());
    match result.await {
        Ok(_) => panic!("expected a CompositionCycle rejection, but the dispatch succeeded"),
        Err(VcsError::CompositionCycle(_)) => {}
        Err(other) => panic!("expected CompositionCycle, got a different VcsError: {other}"),
    }
    assert_eq!(store_a.snapshot().expect("a snapshot").n, Some(0), "A must have zero new edits after a rejected cycle");
}

/// @emoji 🧪️ W1-C's "Owned reproduces today's behaviour EXACTLY" law: `CompositionCoordinator`
/// (the `TransactionCoordinator` alias) dispatching through the ordinary `Owned`-relation
/// `dispatch_group` produces EXACTLY what it did before `MemberRelation` existed — including
/// that `MutationMeta.origin` is NEVER touched (stays the ordinary `Apply`-assigned `Owner`
/// default), because `stamp_tail_origin` is only ever called on the `Peer` path. Every other
/// `dispatch_group_*`/`compensate_*`/`undo_group_*`/`redo_group_*` test above this region
/// already re-proves the rest of `Owned`'s behaviour (ownership check, genesis, atomicity,
/// compensation order) is untouched, unmodified, still green.
#[semio_framework_async_macros::async_test]
async fn dispatch_group_owned_path_never_stamps_a_transaction_origin() {
    let parent_ref = crate::os_io::ArtifactRef { artifact_id: "parent-owned-origin-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let child_ref = crate::os_io::ArtifactRef { artifact_id: "child-owned-origin-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;
    let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;

    let mut coordinator = CompositionCoordinator::new().await;
    coordinator.graph_mut().await.insert_owns(&parent_ref.artifact_id, "slot-1", &child_ref.artifact_id).await.expect("seed ownership");

    let parent_ops = vec![DemoMutation::SetN(SetN { n: 1 }).encode_op().expect("encode parent op")];
    let child_op = DemoMutation::SetN(SetN { n: 2 }).encode_op().expect("encode child op");
    let dispatch = ChildDispatch { child: child_ref.clone(), ops: vec![child_op], op_schema: SchemaId("demo/v1".into()), labels: Vec::new() };
    let mut children = [(&mut child_store, dispatch)];

    let receipt = coordinator.dispatch_group(&parent_ref, &mut parent_store, &mut children, parent_ops, Vec::new(), GroupMeta::default()).await.expect("owned dispatch");

    assert_eq!(receipt.member_edits.len(), 2);
    let parent_origin = parent_store.envelope().vcs.edits.last().expect("parent edit").mutation_meta.last().expect("parent meta").origin.clone();
    let child_origin = child_store.envelope().vcs.edits.last().expect("child edit").mutation_meta.last().expect("child meta").origin.clone();
    assert_eq!(parent_origin, crate::os_spr::MutationOrigin::Owner, "Owned relation never stamps a Transaction origin on the parent");
    assert_eq!(child_origin, crate::os_spr::MutationOrigin::Owner, "Owned relation never stamps a Transaction origin on an owned child either");
}
//#endregion 🔖️TransactionPeerTests

//#region 🔖️PhasePolicyTests
/// @emoji 🧪️ Normal (the default policy) rejects an Error-level message exactly like it already
/// rejects Fatal (`dispatch_group_validate_all_atomicity_one_bad_member_applies_nothing` above)
/// — the SAME all-or-nothing law, now driven by `reject_if_policy_rejects` off the UNIONED
/// `preview_wire` messages rather than an immediate per-member `Err`. Exercises lane 1-A's REAL
/// `ArtifactStore::set_merge_policy` (§C6, landed) end to end — no test-only policy fixture
/// needed now that it exists.
#[semio_framework_async_macros::async_test]
async fn dispatch_group_phase1_rejects_under_normal_when_a_member_yields_an_error_and_nothing_applies() {
    let parent_ref = crate::os_io::ArtifactRef { artifact_id: "policy-parent-normal".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let child_ref = crate::os_io::ArtifactRef { artifact_id: "policy-child-normal".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    // 🎯️ `dispatch_group<Mp: SpaceMember, Mc: SpaceMember + MemberFactory>`: `MemberFactory` is
    // only required on the CHILDREN type (`Mc`) now — the parent/`Mp` bound dropped it (see
    // `dispatch_group`'s own doc comment for why: only children are ever genesis-constructed).
    // `parent_store` still uses this module's test wrapper regardless: `set_merge_policy`/
    // `merge_policy` are inherent methods the wrapper's own `impl SpaceMember` block
    // deliberately does not re-declare, so calling them on the wrapper autoderefs (via
    // `Deref`/`DerefMut`) straight to the real type's inherent method — no `SpaceMember`
    // vtable indirection, no `Normal`-default trap.
    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;
    parent_store.set_merge_policy(crate::os_spr::MergePolicy::Normal);
    assert_eq!(parent_store.merge_policy().await, crate::os_spr::MergePolicy::Normal, "Normal is also the default, but set it explicitly so this test does not rely on that");
    let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;

    let mut coordinator = CompositionCoordinator::new().await;
    coordinator.graph_mut().await.insert_owns(&parent_ref.artifact_id, "slot-1", &child_ref.artifact_id).await.expect("seed ownership");

    let parent_ops = vec![SeverityMutation::SetN(SeveritySetN { n: 1 }).encode_op().expect("encode parent op")];
    let child_op = SeverityMutation::SetErrorN(SetErrorN { n: 99 }).encode_op().expect("encode child op");
    let dispatch = ChildDispatch { child: child_ref.clone(), ops: vec![child_op], op_schema: SchemaId("demo/v1".into()), labels: Vec::new() };
    let mut children = [(&mut child_store, dispatch)];

    let result = coordinator.dispatch_group(&parent_ref, &mut parent_store, &mut children, parent_ops, Vec::new(), GroupMeta::default());
    match result.await {
        Ok(_) => panic!("expected Normal policy to reject an Error-level message, but the dispatch succeeded"),
        Err(VcsError::Rejected { policy, messages }) => {
            assert_eq!(policy, crate::os_spr::MergePolicy::Normal, "the rejection must name the policy that actually rejected it");
            assert!(messages.iter().any(|message| message.level == crate::os_dsl::Severity::Error), "the rejection must carry the Error message that triggered it");
        }
        Err(other) => panic!("expected Rejected, got a different VcsError: {other}"),
    }
    assert!(parent_store.envelope().vcs.edits.is_empty(), "parent must have zero edits after a policy-rejected group dispatch");
    assert!(child_store.envelope().vcs.edits.is_empty(), "child must have zero edits after a policy-rejected group dispatch");
}

/// @emoji 🧪️ The SAME Error-level scenario `dispatch_group_phase1_rejects_under_normal_when_a_
/// member_yields_an_error_and_nothing_applies` rejects is accepted end-to-end under
/// `LaissezFaire` (only `Fatal` is rejected) — both members get a real edit, and the child's own
/// diff stays empty (§C2 LAW 2: an `Error` message's diff carries no change for the target) even
/// though the group as a whole succeeded.
///
/// 🎯️ Sets `LaissezFaire` on BOTH members, not just the parent: `reject_if_policy_rejects` is
/// only the coordinator's GROUP-level gate (checked once, against the parent's own policy,
/// before Phase 2 starts). Phase 2 still dispatches through each member's REAL `dispatch_wire`,
/// which (lane 1-A's now-landed C6 `ArtifactStore::dispatch`) independently enforces THAT
/// member's own `merge_policy()` — a child left at the Normal default would still reject its own
/// Error-level op right here, even though the coordinator's gate already accepted the group.
#[semio_framework_async_macros::async_test]
async fn dispatch_group_phase1_accepts_the_same_error_scenario_under_laissez_faire() {
    let parent_ref = crate::os_io::ArtifactRef { artifact_id: "policy-parent-lf".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let child_ref = crate::os_io::ArtifactRef { artifact_id: "policy-child-lf".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    // 🎯️ `dispatch_group<Mp: SpaceMember, Mc: SpaceMember + MemberFactory>`: `MemberFactory` is
    // only required on the CHILDREN type (`Mc`) now — the parent/`Mp` bound dropped it (see
    // `dispatch_group`'s own doc comment for why: only children are ever genesis-constructed).
    // `parent_store` still uses this module's test wrapper regardless: `set_merge_policy`/
    // `merge_policy` are inherent methods the wrapper's own `impl SpaceMember` block
    // deliberately does not re-declare, so calling them on the wrapper autoderefs (via
    // `Deref`/`DerefMut`) straight to the real type's inherent method — no `SpaceMember`
    // vtable indirection, no `Normal`-default trap.
    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;
    parent_store.set_merge_policy(crate::os_spr::MergePolicy::LaissezFaire);
    let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;
    child_store.set_merge_policy(crate::os_spr::MergePolicy::LaissezFaire);

    let mut coordinator = CompositionCoordinator::new().await;
    coordinator.graph_mut().await.insert_owns(&parent_ref.artifact_id, "slot-1", &child_ref.artifact_id).await.expect("seed ownership");

    let parent_ops = vec![SeverityMutation::SetN(SeveritySetN { n: 1 }).encode_op().expect("encode parent op")];
    let child_op = SeverityMutation::SetErrorN(SetErrorN { n: 99 }).encode_op().expect("encode child op");
    let dispatch = ChildDispatch { child: child_ref.clone(), ops: vec![child_op], op_schema: SchemaId("demo/v1".into()), labels: Vec::new() };
    let mut children = [(&mut child_store, dispatch)];

    let receipt = coordinator.dispatch_group(&parent_ref, &mut parent_store, &mut children, parent_ops, Vec::new(), GroupMeta::default()).await.expect("LaissezFaire accepts an Error-level message");

    assert_eq!(receipt.member_edits.len(), 2, "both parent and child got a real edit despite the child's Error message");
    assert_eq!(parent_store.snapshot().expect("parent snapshot").n, Some(1), "the parent's own clean op still applied");
    assert_eq!(child_store.snapshot().expect("child snapshot").n, Some(0), "the child's Error-level op's diff carries no change for its target (LAW 2), even though the group was accepted");
    assert!(receipt.messages.iter().any(|message| message.code.0 == "mutation.target-missing" && message.level == crate::os_dsl::Severity::Error), "the child's Error message must still be reported, even on acceptance");
}

/// @emoji 🧪️ `Vigilant` rejects a plain `Warning` — the strictest of the three policies, and the
/// one level `Normal` (the OTHER two policy tests above/below use) would have accepted.
#[semio_framework_async_macros::async_test]
async fn dispatch_group_phase1_rejects_under_vigilant_on_a_members_warning() {
    let parent_ref = crate::os_io::ArtifactRef { artifact_id: "policy-parent-vigilant".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let child_ref = crate::os_io::ArtifactRef { artifact_id: "policy-child-vigilant".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    // 🎯️ `dispatch_group<Mp: SpaceMember, Mc: SpaceMember + MemberFactory>`: `MemberFactory` is
    // only required on the CHILDREN type (`Mc`) now — the parent/`Mp` bound dropped it (see
    // `dispatch_group`'s own doc comment for why: only children are ever genesis-constructed).
    // `parent_store` still uses this module's test wrapper regardless: `set_merge_policy`/
    // `merge_policy` are inherent methods the wrapper's own `impl SpaceMember` block
    // deliberately does not re-declare, so calling them on the wrapper autoderefs (via
    // `Deref`/`DerefMut`) straight to the real type's inherent method — no `SpaceMember`
    // vtable indirection, no `Normal`-default trap.
    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;
    parent_store.set_merge_policy(crate::os_spr::MergePolicy::Vigilant);
    let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;

    let mut coordinator = CompositionCoordinator::new().await;
    coordinator.graph_mut().await.insert_owns(&parent_ref.artifact_id, "slot-1", &child_ref.artifact_id).await.expect("seed ownership");

    let child_op = SeverityMutation::SetWarningN(SetWarningN { n: 5 }).encode_op().expect("encode child op");
    let dispatch = ChildDispatch { child: child_ref.clone(), ops: vec![child_op], op_schema: SchemaId("demo/v1".into()), labels: Vec::new() };
    let mut children = [(&mut child_store, dispatch)];

    let result = coordinator.dispatch_group(&parent_ref, &mut parent_store, &mut children, Vec::new(), Vec::new(), GroupMeta::default());
    match result.await {
        Ok(_) => panic!("expected Vigilant policy to reject a Warning-level message, but the dispatch succeeded"),
        Err(VcsError::Rejected { policy, messages }) => {
            assert_eq!(policy, crate::os_spr::MergePolicy::Vigilant, "the rejection must name the policy that actually rejected it");
            assert!(messages.iter().any(|message| message.level == crate::os_dsl::Severity::Warning), "the rejection must carry the Warning message that triggered it");
        }
        Err(other) => panic!("expected Rejected, got a different VcsError: {other}"),
    }
    assert!(child_store.envelope().vcs.edits.is_empty(), "child must have zero edits after a policy-rejected group dispatch");
}

/// @emoji 🧪️ `GroupReceipt.messages` carries the FULL union (both parent's own and the child's),
/// each `target` prefixed with the ORIGINATING member's own `crate::os_io::ArtifactRef::
/// to_uri()` — the discipline that lets a caller with several members in flight tell messages
/// apart. Parent-first ordering matches phase 1's own collection order.
#[semio_framework_async_macros::async_test]
async fn group_receipt_messages_contains_the_union_with_member_path_prefixed_targets() {
    let parent_ref = crate::os_io::ArtifactRef { artifact_id: "policy-parent-union".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let child_ref = crate::os_io::ArtifactRef { artifact_id: "policy-child-union".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    // 🎯️ `dispatch_group<Mp: SpaceMember, Mc: SpaceMember + MemberFactory>`: `MemberFactory` is
    // only required on the CHILDREN type (`Mc`) now — the parent/`Mp` bound dropped it (see
    // `dispatch_group`'s own doc comment for why: only children are ever genesis-constructed).
    // `parent_store` still uses this module's test wrapper regardless: `set_merge_policy`/
    // `merge_policy` are inherent methods the wrapper's own `impl SpaceMember` block
    // deliberately does not re-declare, so calling them on the wrapper autoderefs (via
    // `Deref`/`DerefMut`) straight to the real type's inherent method — no `SpaceMember`
    // vtable indirection, no `Normal`-default trap.
    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;
    parent_store.set_merge_policy(crate::os_spr::MergePolicy::LaissezFaire);
    let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: Some(0) }, None)).await;

    let mut coordinator = CompositionCoordinator::new().await;
    coordinator.graph_mut().await.insert_owns(&parent_ref.artifact_id, "slot-1", &child_ref.artifact_id).await.expect("seed ownership");

    let parent_ops = vec![SeverityMutation::SetWarningN(SetWarningN { n: 3 }).encode_op().expect("encode parent op")];
    let child_op = SeverityMutation::SetWarningN(SetWarningN { n: 5 }).encode_op().expect("encode child op");
    let dispatch = ChildDispatch { child: child_ref.clone(), ops: vec![child_op], op_schema: SchemaId("demo/v1".into()), labels: Vec::new() };
    let mut children = [(&mut child_store, dispatch)];

    let receipt = coordinator.dispatch_group(&parent_ref, &mut parent_store, &mut children, parent_ops, Vec::new(), GroupMeta::default()).await.expect("LaissezFaire accepts a Warning-level message");

    assert_eq!(receipt.messages.len(), 2, "one Warning from the parent, one from the child");
    assert_eq!(receipt.messages[0].level, crate::os_dsl::Severity::Warning);
    assert_eq!(receipt.messages[0].code.0, "mutation.clamped");
    assert_eq!(receipt.messages[0].target, vec![parent_ref.to_uri()], "the parent's own message is prefixed with the parent's own path, collected before any child's");
    assert_eq!(receipt.messages[1].level, crate::os_dsl::Severity::Warning);
    assert_eq!(receipt.messages[1].code.0, "mutation.clamped");
    assert_eq!(receipt.messages[1].target, vec![child_ref.to_uri()], "the child's message is prefixed with the CHILD's own path, not the parent's");
}
//#endregion 🔖️PhasePolicyTests
//#endregion 🔖️CompositionTests
