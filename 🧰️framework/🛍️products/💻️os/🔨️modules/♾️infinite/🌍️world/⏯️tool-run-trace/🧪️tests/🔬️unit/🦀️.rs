use super::*;
use semio_framework_tool_run::{ToolRunId, ToolRunIdentity, ToolRunTraceStore};
use serde_json::Value;
use std::collections::BTreeMap;

const TRACE_PAGES: &str = include_str!("../../../../../../../../🔨️modules/⏯️tool-run/🧫️fixtures/📼️trace-pages.json");

fn fixture() -> Value {
    serde_json::from_str(TRACE_PAGES).expect("📼️ trace-pages fixture parses")
}

fn identity_from(value: &Value) -> ToolRunIdentity {
    let hex = value["baseRevision"].as_str().expect("baseRevision");
    let mut base_revision = [0u8; 32];
    for (index, byte) in base_revision.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&hex[index * 2..index * 2 + 2], 16).expect("hex");
    }
    ToolRunIdentity { id: ToolRunId { app_instance_id: value["id"]["appInstanceId"].as_u64().expect("appInstanceId") as u32, run: value["id"]["run"].as_u64().expect("run") }, generation: value["generation"].as_u64().expect("generation") as u32, base_revision }
}

fn floats<const N: usize>(value: &Value) -> [f32; N] {
    std::array::from_fn(|index| value[index].as_f64().expect("float") as f32)
}

fn op_from(value: &Value) -> ToolRunTraceOp {
    match value["op"].as_str().expect("op") {
        "clear" => ToolRunTraceOp::Clear,
        "retire" => ToolRunTraceOp::Retire { key: value["key"].as_u64().expect("key") },
        _ => {
            let subject = &value["subject"];
            let subject = match subject["kind"].as_str().expect("kind") {
                "instance3d" => ToolRunTraceSubject::Instance3d { mesh: subject["mesh"].as_u64().expect("mesh") as u32, position: floats(&subject["position"]), rotation: floats(&subject["rotation"]), scale: subject["scale"].as_f64().expect("scale") as f32 },
                "placement2d" => ToolRunTraceSubject::Placement2d { shape: subject["shape"].as_u64().expect("shape") as u32, position: floats(&subject["position"]), rotation: subject["rotation"].as_f64().expect("rotation") as f32 },
                _ => ToolRunTraceSubject::Entity { entity: subject["entity"].as_u64().expect("entity") },
            };
            ToolRunTraceOp::Upsert { key: value["key"].as_u64().expect("key"), verdict: ToolRunVerdict::parse(value["verdict"].as_str().expect("verdict")).expect("verdict name"), reason: value["reason"].as_u64().expect("reason") as u16, subject }
        }
    }
}

fn lane(delta: &ToolRunTraceDelta) -> String {
    base64_codec::base64_url_encode(delta.encode().expect("delta encodes"))
}

fn resident_of_layer(layer: &ToolRunTraceLayer) -> BTreeMap<u64, (ToolRunVerdict, ToolRunTraceSubject)> {
    layer.batches().flat_map(|(key, batch)| batch.keys.iter().zip(&batch.subjects).map(move |(record, subject)| (*record, (key.verdict(), *subject)))).collect()
}

fn resident_of_store(store: &ToolRunTraceStore) -> BTreeMap<u64, (ToolRunVerdict, ToolRunTraceSubject)> {
    store.records().map(|(key, record)| (key, (record.verdict, record.subject))).collect()
}

/// 🚚️ Delivers every pending page of `store` to `layer` through the real lane text, `budget` bytes at a
/// time, redelivering each delta twice to prove idempotence.
fn deliver(store: &ToolRunTraceStore, layer: &mut ToolRunTraceLayer, budget: usize) {
    for _ in 0..10_000 {
        let delta = store.delta_after(layer.cursor(), budget);
        let text = lane(&delta);
        layer.apply_lane(Some(&text)).expect("lane applies");
        layer.apply_delta(&delta);
        if layer.cursor().is_some_and(|cursor| cursor.page == store.next_page()) {
            return;
        }
    }
    panic!("delivery never caught up");
}

#[test]
fn the_layer_reproduces_every_fixture_residency_case_through_the_lane() {
    let fixture = fixture();
    for case in fixture["residency"].as_array().expect("residency cases") {
        let identity = identity_from(&case["identity"]);
        let mut store = ToolRunTraceStore::with_limits(identity, case["capacity"].as_u64().expect("capacity") as usize, case["compactFloor"].as_u64().expect("compactFloor") as usize);
        let mut layer = ToolRunTraceLayer::default();
        for page in case["pages"].as_array().expect("pages") {
            let ops: Vec<ToolRunTraceOp> = page.as_array().expect("page ops").iter().map(op_from).collect();
            store.apply_ops(&ops);
            deliver(&store, &mut layer, 1);
        }
        let expected: BTreeMap<u64, ToolRunVerdict> = case["resident"].as_array().expect("resident").iter().map(|row| (row["key"].as_u64().expect("key"), ToolRunVerdict::parse(row["verdict"].as_str().expect("verdict")).expect("verdict name"))).collect();
        let actual: BTreeMap<u64, ToolRunVerdict> = resident_of_layer(&layer).into_iter().map(|(key, (verdict, _))| (key, verdict)).collect();
        assert_eq!(actual, expected, "{}", case["name"]);
        assert_eq!(resident_of_layer(&layer), resident_of_store(&store), "{}", case["name"]);
        let mut cold = ToolRunTraceLayer::default();
        cold.apply_lane(Some(&lane(&store.delta_after(None, usize::MAX)))).expect("cold lane applies");
        assert_eq!(resident_of_layer(&cold), resident_of_store(&store), "{} cold resend", case["name"]);
    }
}

#[test]
fn a_redelivered_or_stale_lane_is_idempotent_and_a_new_generation_clears() {
    let fixture = fixture();
    let delta = &fixture["deltas"][0]["delta"];
    let identity = identity_from(&delta["identity"]);
    let hex = fixture["deltas"][0]["hex"].as_str().expect("hex");
    let bytes: Vec<u8> = (0..hex.len()).step_by(2).map(|index| u8::from_str_radix(&hex[index..index + 2], 16).expect("hex")).collect();
    let text = base64_codec::base64_url_encode(&bytes);
    let mut layer = ToolRunTraceLayer::default();
    let first = layer.apply_lane(Some(&text)).expect("fixture delta applies");
    assert!(first.cleared);
    assert_eq!(layer.cursor(), Some(ToolRunTraceCursor { run: identity.id.run, generation: identity.generation, page: delta["next"].as_u64().expect("next") as u32 }));
    let resident = resident_of_layer(&layer);
    assert_eq!(layer.apply_lane(Some(&text)).expect("same lane"), ToolRunTraceLayerApply::default());
    assert_eq!(layer.apply_lane(None).expect("idle"), ToolRunTraceLayerApply::default());
    assert_eq!(resident_of_layer(&layer), resident);
    assert_eq!(layer.apply_lane(Some("!!")), Err(ToolRunTraceLaneFault::Base64));
    assert_eq!(resident_of_layer(&layer), resident, "a malformed lane keeps the previous records");

    let mut store = ToolRunTraceStore::with_limits(identity.next_generation(), 16, 4);
    store.apply_ops(&[ToolRunTraceOp::Upsert { key: 9, verdict: ToolRunVerdict::Success, reason: 0, subject: ToolRunTraceSubject::Entity { entity: 9 } }]);
    let mut stale_cursor_delta = store.delta_after(layer.cursor(), usize::MAX);
    stale_cursor_delta.clear = false;
    assert!(layer.apply_delta(&stale_cursor_delta).cleared, "a generation change clears even without the flag");
    assert_eq!(resident_of_layer(&layer), resident_of_store(&store));
}

/// 🎲️ Deterministic LCG so the randomized law is reproducible without a `rand` dependency.
struct Lcg(u64);

impl Lcg {
    fn next(&mut self, bound: u64) -> u64 {
        self.0 = self.0.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        (self.0 >> 33) % bound
    }
}

#[test]
fn randomized_runs_with_clear_retire_eviction_and_compaction_stay_in_lockstep_with_the_ledger_store() {
    let identity = ToolRunIdentity::new(ToolRunId { app_instance_id: 1, run: 7 }, [3; 32]);
    let mut lcg = Lcg(0x5EED_7001);
    for round in 0..24 {
        let mut store = ToolRunTraceStore::with_limits(identity, 48, 8);
        let mut layer = ToolRunTraceLayer::default();
        for _ in 0..60 {
            let ops: Vec<ToolRunTraceOp> = (0..lcg.next(12) + 1)
                .map(|_| match lcg.next(20) {
                    0 => ToolRunTraceOp::Clear,
                    1..=4 => ToolRunTraceOp::Retire { key: lcg.next(96) },
                    _ => ToolRunTraceOp::Upsert {
                        key: lcg.next(96),
                        verdict: ToolRunVerdict::ALL[lcg.next(4) as usize],
                        reason: 0,
                        subject: ToolRunTraceSubject::Instance3d { mesh: lcg.next(3) as u32, position: [lcg.next(10) as f32, 0.0, 0.0], rotation: [0.0, 0.0, 0.0, 1.0], scale: 1.0 },
                    },
                })
                .collect();
            store.apply_ops(&ops);
            deliver(&store, &mut layer, 64 + (round * 37) as usize);
            assert_eq!(resident_of_layer(&layer), resident_of_store(&store), "round {round}");
        }
    }
}

#[test]
fn draws_are_one_instanced_batch_per_mesh_and_verdict_with_counts_equal_to_resident_records() {
    let identity = ToolRunIdentity::new(ToolRunId { app_instance_id: 2, run: 1 }, [0; 32]);
    let mut store = ToolRunTraceStore::with_limits(identity, 1_024, 64);
    let mut lcg = Lcg(0xD2A3);
    let ops: Vec<ToolRunTraceOp> = (0..400)
        .map(|index| ToolRunTraceOp::Upsert {
            key: index,
            verdict: ToolRunVerdict::ALL[lcg.next(4) as usize],
            reason: 0,
            subject: if index % 10 == 0 { ToolRunTraceSubject::Entity { entity: index } } else { ToolRunTraceSubject::Instance3d { mesh: lcg.next(4) as u32, position: [index as f32, 0.0, 0.0], rotation: [0.0, 0.0, 0.0, 1.0], scale: 0.5 } },
        })
        .chain((0..50).map(|key| ToolRunTraceOp::Retire { key: key * 3 }))
        .collect();
    store.apply_ops(&ops);
    let mut layer = ToolRunTraceLayer::default();
    deliver(&store, &mut layer, 512);
    let palette = ToolRunTracePalette::from_theme(&Theme::dark());
    let draws = layer.draws(&palette, ToolRunTraceVisibility::default());

    let mut expected: BTreeMap<(u32, u8), usize> = BTreeMap::new();
    for (_, record) in store.records() {
        if let ToolRunTraceSubject::Instance3d { mesh, .. } = record.subject {
            *expected.entry((mesh, record.verdict.ordinal())).or_default() += 1;
        }
    }
    let actual: BTreeMap<(u32, u8), usize> = draws.iter().map(|draw| ((draw.mesh, draw.verdict.ordinal()), draw.instances.len())).collect();
    assert_eq!(actual, expected);
    assert_eq!(draws.len(), expected.len(), "exactly one draw per (mesh, verdict)");
    let highlighted: Vec<&Instance3d> = draws.iter().flat_map(|draw| &draw.instances).filter(|instance| instance.selected).collect();
    assert_eq!(highlighted.len(), usize::from(layer.newest_testing().is_some()));
    if let Some(newest) = layer.newest_testing() {
        assert_eq!(highlighted[0].id, format!("toolRunTrace:{newest}"));
        assert_eq!(store.record(newest).expect("newest resident").verdict, ToolRunVerdict::Testing);
    }
    for draw in &draws {
        let tone = palette.color(draw.verdict);
        for instance in &draw.instances {
            assert_eq!(instance.color[..3], [tone.r, tone.g, tone.b]);
            assert!(instance.color[3] <= tone.a && instance.color[3] >= tone.a * tool_run::FADE_FLOOR_OPACITY as f32 - f32::EPSILON);
        }
    }
    assert_eq!(palette.testing.a, tool_run::TESTING_OPACITY as f32);

    let rejected_hidden = layer.draws(&palette, ToolRunTraceVisibility { rejected: false, ..Default::default() });
    assert!(rejected_hidden.iter().all(|draw| !draw.verdict.is_rejected()));
    assert_eq!(rejected_hidden.iter().map(|draw| draw.instances.len()).sum::<usize>(), layer.count(ToolRunVerdict::Testing) + layer.count(ToolRunVerdict::Success) - layer.batches().filter(|(key, _)| key.family == ToolRunTraceFamily::Entity && !key.verdict().is_rejected()).map(|(_, batch)| batch.keys.len()).sum::<usize>());
}

#[test]
fn fade_starts_opaque_and_bottoms_out_at_the_floor_token() {
    assert_eq!(tool_run_trace_fade(0), 1.0);
    assert!(tool_run_trace_fade(1) < 1.0);
    assert_eq!(tool_run_trace_fade(tool_run::FADE_RECORDS as u64), tool_run::FADE_FLOOR_OPACITY as f32);
    assert_eq!(tool_run_trace_fade(u64::MAX / 2), tool_run::FADE_FLOOR_OPACITY as f32);
    assert_eq!(tool_run_provisional_color(&Theme::light())[3], tool_run::PROVISIONAL_OPACITY as f32);
    assert!(TOOL_RUN_TRACE_LANE_BYTES_MAX >= TOOL_RUN_TRACE_PAGE_BYTES_MAX * 16 * 4 / 3);
}
