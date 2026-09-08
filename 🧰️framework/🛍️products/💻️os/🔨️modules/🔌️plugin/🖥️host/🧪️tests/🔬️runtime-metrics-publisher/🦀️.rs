use super::RuntimeMetricsPublisher;
use semio_framework_actor::{ActivationEvent, ActorKind, Envelope, Kernel, Lane, Origin, PackageId, Payload, ShardId, ShardKind, TurnResult, TurnStatus, Usage};
use std::collections::HashMap;

async fn env(to: semio_framework_actor::ActorId, lane: Lane, seq: u64) -> Envelope {
    Envelope { to, from: Origin::Kernel, lane, seq, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Event { bytes: vec![1] } }
}

async fn ok_turn() -> TurnResult {
    TurnResult {
        ui_patches: vec![],
        effects: vec![],
        lifecycle_receipt: None,
        ui_patch_receipt: None,
        command_ingress: vec![],
        cold_pair_ingress: semio_framework_actor::cold_pair::ColdPairIngressStatus::Idle,
        next_wake: None,
        status: TurnStatus::Idle,
        usage: Usage { fuel: 10, wall_us: 5, memory_bytes: 512 },
    }
}

/// 📈️ Drives a real `Kernel` (not a fake) through one turn, confirms the 2Hz gate (500ms), and
/// decodes the published bytes back into a `RuntimeMetricsSnapshot` to prove the payload round-
/// trips and carries the heartbeat overlay this file is responsible for.
#[semio_framework_async_macros::async_test]
async fn maybe_sample_gates_at_2hz_and_overlays_heartbeat_age_from_the_host() {
    let mut kernel = Kernel::new(ShardKind::Native, 1, 0, 4).await;
    let actor = kernel.activate(PackageId("s.cad".into()), 1, ActorKind::PluginApp { plugin: PackageId("s.cad".into()), app_id: "editor".into(), instance_id: 0 }, Lane::Interactive, None, ActivationEvent::Manual).await;
    kernel.submit(&env(actor, Lane::Interactive, 1).await).await;
    kernel.tick(0).await;
    kernel.complete(actor, &ok_turn().await, 0).await.unwrap();

    let mut publisher = RuntimeMetricsPublisher::new();
    let heartbeats: HashMap<ShardId, u64> = [(ShardId(0), 400u64)].into_iter().collect();

    let first = publisher.maybe_sample(&kernel, 1_000, &heartbeats).await.expect("never published yet must be due");
    let mut pos = 0usize;
    let decoded = semio_framework_actor::RuntimeMetricsSnapshot::pack_decode(&first, &mut pos).await.unwrap();
    assert_eq!(pos, first.len());
    assert_eq!(decoded.actors.len(), 1);
    assert_eq!(decoded.actors[0].metrics.turns, 1);
    let shard_row = decoded.shards.iter().find(|row| row.shard == ShardId(0)).expect("shard 0 row present");
    assert_eq!(shard_row.metrics.heartbeat_age_ms, 600, "1_000 - 400 heartbeat overlay");

    assert!(publisher.maybe_sample(&kernel, 1_200, &heartbeats).await.is_none(), "200ms since last publish is inside the 500ms window");
    assert!(publisher.maybe_sample(&kernel, 1_500, &heartbeats).await.is_some(), "exactly the 500ms interval must fire again");
}

//#region 🔖️ScaleFixture
/// 🧫️ The 50×50 scale fixture registry (M2's own data source: 50 plugins × 50 extensions each,
/// 2550 records, 7 `scaleFixture.profile` behaviour profiles) — see this module's own doc comment
/// for why `include_str!` (not `std::fs`) is how a `#[cfg(test)]` block reads it without tripping
/// the crate-purity grep this repo's acceptance criteria runs unconditionally over `component.rs`
/// files (this one is the HOST, not the pure `🎭️actor` crate, but the same discipline is followed
/// here since the file is right next to it and easy to mistake for one).
const SCALE_FIXTURE_REGISTRY_JSON: &str = include_str!("../../../../../🧫️fixtures/⚖️scale/🤖️generated/📇️registry/🔣️.json");

#[derive(serde::Deserialize)]
struct ScaleFixtureRegistry {
    #[serde(rename = "recordCount")]
    record_count: u32,
    records: Vec<ScaleFixtureRecord>,
}

#[derive(serde::Deserialize)]
struct ScaleFixtureRecord {
    id: String,
    kind: String,
    #[serde(rename = "parentId")]
    parent_id: Option<String>,
    #[serde(rename = "scaleFixture")]
    scale_fixture: ScaleFixtureProfile,
}

#[derive(serde::Deserialize)]
struct ScaleFixtureProfile {
    profile: String,
}

/// 🧮️ `"scale-fixture-plugin-0007"` → `7` — every record's OWN `plugin_ordinal` for `Kernel::
/// activate` is its ancestor plugin's numeric suffix (extensions share their parent plugin's
/// ordinal, matching `ActorId`'s bit-packed "which plugin family" semantics).
async fn plugin_ordinal_from_id(plugin_id: &str) -> u16 {
    plugin_id.rsplit('-').next().and_then(|suffix| suffix.parse::<u16>().ok()).unwrap_or(0)
}

/// 🛣️ Deterministic, arbitrary profile→lane mapping — realism, not a claimed design contract (the
/// scale fixture itself is silent on lane assignment).
async fn lane_for_profile(profile: &str) -> Lane {
    match profile {
        "ui" | "stateful" => Lane::Interactive,
        "cpu" | "crash" => Lane::UserVisible,
        "io" | "hang" => Lane::Background,
        _ => Lane::Maintenance,
    }
}

/// 📈️ T1 runtime evidence at scale (`📓️terra-T1-report.md`'s acceptance criteria): activates
/// every one of the fixture's 2550 real records through a real `Kernel` (not a fake), drives a
/// deterministic sample through `submit`/`tick`/`complete` (including a `Faulted` turn for the
/// `"crash"` profile), then asserts `RuntimeMetricsPublisher::maybe_sample`'s decoded snapshot
/// reflects it — row count, package count, and the specific driven actors' turns/traps.
#[semio_framework_async_macros::async_test]
async fn runtime_metrics_publisher_reflects_the_2550_record_scale_fixture_registry() {
    let registry: ScaleFixtureRegistry = serde_json::from_str(SCALE_FIXTURE_REGISTRY_JSON).expect("scale fixture registry must be valid JSON matching the documented shape");
    assert_eq!(registry.record_count as usize, registry.records.len(), "recordCount header must match the actual records array length");
    assert_eq!(registry.record_count, 2550, "the documented 50 plugins x (1 + 50 extensions) fixture shape");

    let mut kernel = Kernel::new(ShardKind::Native, 8, 0, 256).await;
    let mut actor_ids = HashMap::new();
    let mut crash_profile_actor = None;

    for record in &registry.records {
        let plugin_id = record.parent_id.as_deref().unwrap_or(&record.id);
        let ordinal = plugin_ordinal_from_id(plugin_id);
        let lane = lane_for_profile(&record.scale_fixture.profile);
        let (package, kind) = if record.kind == "plugin" {
            (PackageId(record.id.clone()), ActorKind::PluginApp { plugin: PackageId(record.id.clone()), app_id: "main".into(), instance_id: 0 })
        } else {
            (PackageId(plugin_id.to_string()), ActorKind::Extension { plugin: PackageId(plugin_id.to_string()), extension_id: record.id.clone() })
        };
        let actor = kernel.activate(package, ordinal.await, kind, lane.await, None, ActivationEvent::Manual).await;
        if record.scale_fixture.profile == "crash" && crash_profile_actor.is_none() {
            crash_profile_actor = Some(actor);
        }
        actor_ids.insert(record.id.clone(), actor);
    }
    assert_eq!(actor_ids.len(), 2550, "every record activated exactly one distinct actor");

    // 🎬️ Drive a deterministic sample so the snapshot has non-zero activity to assert on: the
    // first plugin gets a clean turn, the first "crash" profile actor gets a `Faulted` turn.
    let first_record = &registry.records[0];
    let first_actor = actor_ids[&first_record.id];
    kernel.submit(&env(first_actor, lane_for_profile(&first_record.scale_fixture.profile).await, 1).await).await;
    kernel.tick(0).await;
    kernel.complete(first_actor, &ok_turn().await, 1).await.unwrap();

    let crash_actor = crash_profile_actor.expect("fixture has at least one \"crash\" profile record");
    kernel.submit(&env(crash_actor, Lane::UserVisible, 1).await).await;
    kernel.tick(1).await;
    let faulted = TurnResult {
        ui_patches: vec![],
        effects: vec![],
        lifecycle_receipt: None,
        ui_patch_receipt: None,
        command_ingress: vec![],
        cold_pair_ingress: semio_framework_actor::cold_pair::ColdPairIngressStatus::Idle,
        next_wake: None,
        status: TurnStatus::Faulted { detail: b"scale-fixture crash profile".to_vec() },
        usage: Usage { fuel: 5, wall_us: 3, memory_bytes: 256 },
    };
    kernel.complete(crash_actor, &faulted, 2).await.unwrap();

    let mut publisher = RuntimeMetricsPublisher::new();
    let payload = publisher.maybe_sample(&kernel, 10_000, &HashMap::new()).await.expect("first sample is always due");
    let mut pos = 0usize;
    let snapshot = semio_framework_actor::RuntimeMetricsSnapshot::pack_decode(&payload, &mut pos).await.unwrap();
    assert_eq!(pos, payload.len());

    assert_eq!(snapshot.kernel.actors, 2550);
    assert_eq!(snapshot.kernel.packages, 50, "50 plugins, each extension's package folds into its parent plugin's PackageId");
    assert_eq!(snapshot.actors.len(), 2550);

    let first_row = snapshot.actors.iter().find(|row| row.id == first_actor).expect("first record's row present");
    assert_eq!(first_row.metrics.turns, 1);
    assert_eq!(first_row.status, semio_framework_actor::ActorStatus::Active);

    let crash_row = snapshot.actors.iter().find(|row| row.id == crash_actor).expect("crash-profile record's row present");
    assert_eq!(crash_row.metrics.turns, 1);
    assert_eq!(crash_row.metrics.traps, 1, "the Faulted turn must register as a trap");

    let total_shard_actors: u32 = snapshot.shards.iter().map(|row| row.metrics.actors).sum();
    assert_eq!(total_shard_actors, 2550, "every one of the 2550 actors is counted on exactly one of the 8 shards");
}
//#endregion 🔖️ScaleFixture
