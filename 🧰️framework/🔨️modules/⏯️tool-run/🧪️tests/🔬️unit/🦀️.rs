//! 🔬️ Rust reducer, ring, store and codecs against the three shared fixtures.

use super::*;
use serde_json::{json, Value};

const LIFECYCLE: &str = include_str!("../../🧫️fixtures/⚖️lifecycle-law.json");
const TRACE_PAGES: &str = include_str!("../../🧫️fixtures/📼️trace-pages.json");
const TICKS: &str = include_str!("../../🧫️fixtures/🎞️ticks.json");

fn fixture(text: &str) -> Value {
    serde_json::from_str(text).expect("fixture parses")
}

fn u64_of(value: &Value) -> u64 {
    value.as_u64().expect("u64")
}

fn u32_of(value: &Value) -> u32 {
    u32::try_from(u64_of(value)).expect("u32")
}

fn u16_of(value: &Value) -> u16 {
    u16::try_from(u64_of(value)).expect("u16")
}

fn f32_of(value: &Value) -> f32 {
    value.as_f64().expect("number") as f32
}

fn text(value: &Value) -> &str {
    value.as_str().expect("string")
}

fn hex(value: &Value) -> Vec<u8> {
    let text = text(value);
    (0..text.len()).step_by(2).map(|index| u8::from_str_radix(&text[index..index + 2], 16).expect("hex")).collect()
}

fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn identity(value: &Value) -> ToolRunIdentity {
    ToolRunIdentity { id: ToolRunId { app_instance_id: u32_of(&value["id"]["appInstanceId"]), run: u64_of(&value["id"]["run"]) }, generation: u32_of(&value["generation"]), base_revision: hex(&value["baseRevision"]).try_into().expect("32 bytes") }
}

fn slot(value: &Value) -> Option<ToolRunSlot> {
    (!value.is_null()).then(|| ToolRunSlot { run: u64_of(&value["run"]), generation: u32_of(&value["generation"]), state: ToolRunState::parse(text(&value["state"])).expect("state") })
}

fn event(value: &Value) -> ToolRunEvent {
    let run = || u64_of(&value["run"]);
    let generation = || u32_of(&value["generation"]);
    match text(&value["type"]) {
        "start" => ToolRunEvent::Start { run: run() },
        "jobAdmitted" => ToolRunEvent::JobAdmitted { run: run(), generation: generation() },
        "pause" => ToolRunEvent::Pause { run: run(), generation: generation() },
        "resume" => ToolRunEvent::Resume { run: run(), generation: generation() },
        "step" => ToolRunEvent::Step { run: run(), generation: generation() },
        "jobComplete" => ToolRunEvent::JobComplete { run: run(), generation: generation() },
        "jobFault" => ToolRunEvent::JobFault { run: run(), generation: generation() },
        "settingsChanged" => ToolRunEvent::SettingsChanged { run: run() },
        "baseChanged" => ToolRunEvent::BaseChanged { run: run() },
        "finalize" => ToolRunEvent::Finalize { run: run(), generation: generation() },
        "publicationComplete" => ToolRunEvent::PublicationComplete { run: run(), generation: generation() },
        "revalidationConflicts" => ToolRunEvent::RevalidationConflicts { run: run(), generation: generation() },
        "storeRejected" => ToolRunEvent::StoreRejected { run: run(), generation: generation() },
        "abort" => ToolRunEvent::Abort { run: run(), generation: generation(), publishing: value["publishing"].as_bool().expect("publishing") },
        "abortComplete" => ToolRunEvent::AbortComplete { run: run(), generation: generation() },
        "dismiss" => ToolRunEvent::Dismiss { run: run() },
        "closed" => ToolRunEvent::Closed,
        other => panic!("unknown event {other}"),
    }
}

fn matrix_event(key: ToolRunEventKey, run: u64, generation: u32) -> ToolRunEvent {
    use ToolRunEventKey as K;
    match key {
        K::Start => ToolRunEvent::Start { run: run + 1 },
        K::JobAdmitted => ToolRunEvent::JobAdmitted { run, generation },
        K::Pause => ToolRunEvent::Pause { run, generation },
        K::Resume => ToolRunEvent::Resume { run, generation },
        K::Step => ToolRunEvent::Step { run, generation },
        K::JobComplete => ToolRunEvent::JobComplete { run, generation },
        K::JobFault => ToolRunEvent::JobFault { run, generation },
        K::SettingsChanged => ToolRunEvent::SettingsChanged { run },
        K::BaseChanged => ToolRunEvent::BaseChanged { run },
        K::Finalize => ToolRunEvent::Finalize { run, generation },
        K::PublicationComplete => ToolRunEvent::PublicationComplete { run, generation },
        K::RevalidationConflicts => ToolRunEvent::RevalidationConflicts { run, generation },
        K::StoreRejected => ToolRunEvent::StoreRejected { run, generation },
        K::Abort => ToolRunEvent::Abort { run, generation, publishing: false },
        K::AbortWhilePublishing => ToolRunEvent::Abort { run, generation, publishing: true },
        K::AbortComplete => ToolRunEvent::AbortComplete { run, generation },
        K::Dismiss => ToolRunEvent::Dismiss { run },
        K::Closed => ToolRunEvent::Closed,
    }
}

fn slot_state_name(slot: Option<ToolRunSlot>) -> &'static str {
    slot.map_or("none", |slot| slot.state.as_str())
}

fn transition_json(transition: ToolRunTransition) -> Value {
    json!({ "slot": transition.slot.map(|slot| json!({ "run": slot.run, "generation": slot.generation, "state": slot.state.as_str() })), "effect": transition.effect.as_str() })
}

fn step(value: &Value) -> ToolRunStep {
    let args = value["args"].as_array().expect("args").iter().map(|arg| if let Some(unsigned) = arg.get("unsigned") { ToolRunStepArg::Unsigned(u64_of(unsigned)) } else { ToolRunStepArg::Float(arg["float"].as_f64().expect("float")) }).collect();
    ToolRunStep { sequence: u64_of(&value["sequence"]), kind: ToolRunStepKind::parse(text(&value["kind"])).expect("kind"), stage: u16_of(&value["stage"]), reason: u16_of(&value["reason"]), subject: value.get("subject").map(u64_of), repeat: u32_of(&value["repeat"]), args }
}

fn subject(value: &Value) -> ToolRunTraceSubject {
    let floats = |key: &str| value[key].as_array().expect("floats").iter().map(f32_of).collect::<Vec<_>>();
    match text(&value["kind"]) {
        "instance3d" => {
            let (position, rotation) = (floats("position"), floats("rotation"));
            ToolRunTraceSubject::Instance3d { mesh: u32_of(&value["mesh"]), position: [position[0], position[1], position[2]], rotation: [rotation[0], rotation[1], rotation[2], rotation[3]], scale: f32_of(&value["scale"]) }
        }
        "placement2d" => {
            let position = floats("position");
            ToolRunTraceSubject::Placement2d { shape: u32_of(&value["shape"]), position: [position[0], position[1]], rotation: f32_of(&value["rotation"]) }
        }
        _ => ToolRunTraceSubject::Entity { entity: u64_of(&value["entity"]) },
    }
}

fn op(value: &Value) -> ToolRunTraceOp {
    match text(&value["op"]) {
        "upsert" => ToolRunTraceOp::Upsert { key: u64_of(&value["key"]), verdict: ToolRunVerdict::parse(text(&value["verdict"])).expect("verdict"), reason: u16_of(&value["reason"]), subject: subject(&value["subject"]) },
        "retire" => ToolRunTraceOp::Retire { key: u64_of(&value["key"]) },
        _ => ToolRunTraceOp::Clear,
    }
}

fn ops(value: &Value) -> Vec<ToolRunTraceOp> {
    value.as_array().expect("ops").iter().map(op).collect()
}

fn page(value: &Value) -> ToolRunTracePage {
    ToolRunTracePage { identity: identity(&value["identity"]), page: u32_of(&value["page"]), ops: ops(&value["ops"]) }
}

fn delta(value: &Value) -> ToolRunTraceDelta {
    ToolRunTraceDelta { identity: identity(&value["identity"]), clear: value["clear"].as_bool().expect("clear"), next: u32_of(&value["next"]), pages: value["pages"].as_array().expect("pages").iter().map(page).collect() }
}

fn progress(value: &Value) -> ToolRunProgress {
    ToolRunProgress {
        identity: identity(&value["identity"]),
        sequence: u64_of(&value["sequence"]),
        state: ToolRunState::parse(text(&value["state"])).expect("state"),
        stage: u16_of(&value["stage"]),
        completed: u64_of(&value["completed"]),
        total: value.get("total").map(u64_of),
        counters: value["counters"].as_array().expect("counters").iter().map(|counter| ToolRunCounter { counter: u16_of(&counter["counter"]), value: u64_of(&counter["value"]) }).collect(),
        units_per_second: f32_of(&value["unitsPerSecond"]),
        conflicts: u32_of(&value["conflicts"]),
        steps: ToolRunStepRing::from_steps(value["steps"].as_array().expect("steps").iter().map(step).collect()).expect("ring"),
    }
}

fn tick(value: &Value) -> ToolRunTick {
    ToolRunTick {
        identity: identity(&value["identity"]),
        sequence: u64_of(&value["sequence"]),
        progress: value.get("progress").map(progress),
        steps: value["steps"].as_array().expect("steps").iter().map(step).collect(),
        trace: value["trace"].as_array().expect("trace").iter().map(page).collect(),
        append_ops: value["appendOps"].as_array().expect("appendOps").iter().map(hex).collect(),
        append_entities: value["appendEntities"].as_array().expect("appendEntities").iter().map(u64_of).collect(),
        retract_to: value.get("retractTo").map(u32_of),
        payload: value.get("payload").map(hex),
    }
}

fn store_for(case: &Value) -> (ToolRunTraceStore, Vec<ToolRunTraceApply>) {
    let mut store = ToolRunTraceStore::with_limits(identity(&case["identity"]), case["capacity"].as_u64().expect("capacity") as usize, case["compactFloor"].as_u64().expect("floor") as usize);
    let applied = case["pages"].as_array().expect("pages").iter().map(|page_ops| store.apply_ops(&ops(page_ops))).collect();
    (store, applied)
}

//#region 🔖️Lifecycle
#[test]
fn lifecycle_matrix_covers_every_state_and_event_pair_exactly_once() {
    let law = fixture(LIFECYCLE);
    let states: Vec<&str> = law["states"].as_array().expect("states").iter().map(text).collect();
    assert_eq!(states, ToolRunState::ALL.map(ToolRunState::as_str));
    let keys: Vec<&str> = law["eventKeys"].as_array().expect("keys").iter().map(text).collect();
    assert_eq!(keys, ToolRunEventKey::ALL.map(ToolRunEventKey::as_str));
    let pairs: std::collections::HashSet<(String, String)> = law["matrix"].as_array().expect("matrix").iter().map(|row| (text(&row["from"]).to_string(), text(&row["event"]).to_string())).collect();
    assert_eq!(pairs.len(), 10 * 18);
}

#[test]
fn reducer_matches_every_lifecycle_matrix_row() {
    let law = fixture(LIFECYCLE);
    for row in law["matrix"].as_array().expect("matrix") {
        let from = text(&row["from"]);
        let current = ToolRunState::parse(from).map(|state| ToolRunSlot { run: 7, generation: 2, state });
        let key = ToolRunEventKey::parse(text(&row["event"])).expect("key");
        let outcome = ToolRunMachine::apply(current, matrix_event(key, 7, 2));
        let label = format!("{from} × {}", key.as_str());
        match row.get("rejection") {
            Some(rejection) => assert_eq!(outcome.map_err(ToolRunRejection::code), Err(text(rejection)), "{label}"),
            None => {
                let transition = outcome.unwrap_or_else(|rejection| panic!("{label} rejected with {rejection}"));
                assert_eq!(slot_state_name(transition.slot), text(&row["to"]), "{label}");
                assert_eq!(transition.effect.as_str(), text(&row["effect"]), "{label}");
                assert_eq!(transition.effect.commits(), row["commits"].as_bool().expect("commits"), "{label}");
                if let Some(next) = transition.slot {
                    let expected = match text(&row["generation"]) {
                        "reset" => 0,
                        "increment" => 3,
                        _ => 2,
                    };
                    assert_eq!(next.generation, expected, "{label}");
                    assert_eq!(next.run, if key == ToolRunEventKey::Start { 8 } else { 7 }, "{label}");
                }
            }
        }
    }
}

#[test]
fn reducer_matches_every_lifecycle_case() {
    let law = fixture(LIFECYCLE);
    for case in law["cases"].as_array().expect("cases") {
        let outcome = ToolRunMachine::apply(slot(&case["slot"]), event(&case["event"]));
        let name = text(&case["name"]);
        match case["expect"].get("rejection") {
            Some(rejection) => assert_eq!(outcome.map_err(ToolRunRejection::code), Err(text(rejection)), "{name}"),
            None => assert_eq!(transition_json(outcome.unwrap_or_else(|rejection| panic!("{name}: {rejection}"))), case["expect"]["transition"], "{name}"),
        }
    }
}

#[test]
fn reducer_replays_every_lifecycle_scenario_with_its_commit_count() {
    let law = fixture(LIFECYCLE);
    for scenario in law["scenarios"].as_array().expect("scenarios") {
        let name = text(&scenario["name"]);
        let mut current = None;
        let mut commits = 0;
        let mut observed = Vec::new();
        for value in scenario["events"].as_array().expect("events") {
            match ToolRunMachine::apply(current, event(value)) {
                Ok(transition) => {
                    commits += usize::from(transition.effect.commits());
                    current = transition.slot;
                    observed.push(slot_state_name(current).to_string());
                }
                Err(rejection) => observed.push(rejection.code().to_string()),
            }
        }
        let expected: Vec<String> = scenario["states"].as_array().expect("states").iter().map(|state| text(state).to_string()).collect();
        assert_eq!(observed, expected, "{name}");
        assert_eq!(commits, scenario["commits"].as_u64().expect("commits") as usize, "{name}");
    }
}

#[test]
fn lifecycle_invariants_hold_over_the_matrix_and_scenarios() {
    let law = fixture(LIFECYCLE);
    let ids: Vec<&str> = law["invariants"].as_array().expect("invariants").iter().map(|invariant| text(&invariant["id"])).collect();
    assert_eq!(ids, ["finalizeOnlyInComplete", "storeGenerationOnlyOnFinalized", "abortAndFaultLeaveZeroTrace", "pauseAndStepNeverCommit", "singleNonTerminalRun"]);
    for state in ToolRunState::ALL.map(Some).into_iter().chain([None]) {
        let current = state.map(|state| ToolRunSlot { run: 1, generation: 0, state });
        for key in ToolRunEventKey::ALL {
            let outcome = ToolRunMachine::apply(current, matrix_event(key, 1, 0));
            if key == ToolRunEventKey::Finalize {
                assert_eq!(outcome.is_ok(), state == Some(ToolRunState::Complete));
            }
            if let Ok(transition) = outcome {
                assert_eq!(transition.effect.commits(), state == Some(ToolRunState::Finalizing) && key == ToolRunEventKey::PublicationComplete);
                if matches!(key, ToolRunEventKey::Pause | ToolRunEventKey::Resume | ToolRunEventKey::Step) {
                    assert!(!transition.effect.commits());
                }
            }
            if key == ToolRunEventKey::Start && state.is_some_and(|state| !state.is_terminal()) {
                assert_eq!(outcome, Err(ToolRunRejection::Busy));
            }
        }
    }
    for scenario in law["scenarios"].as_array().expect("scenarios") {
        let states: Vec<&str> = scenario["states"].as_array().expect("states").iter().map(text).collect();
        let mut commits_since_start = 0;
        let mut current = None;
        for value in scenario["events"].as_array().expect("events") {
            if let Ok(transition) = ToolRunMachine::apply(current, event(value)) {
                if transition.effect == ToolRunEffect::SpawnJob {
                    commits_since_start = 0;
                }
                commits_since_start += usize::from(transition.effect.commits());
                current = transition.slot;
                if matches!(current.map(|slot| slot.state), Some(ToolRunState::Aborted | ToolRunState::Faulted)) {
                    assert_eq!(commits_since_start, 0, "{states:?}");
                }
            }
        }
    }
}
//#endregion 🔖️Lifecycle

//#region 🔖️ActionsAndLabels
#[test]
fn actions_chords_arguments_and_legality_match_the_fixture_and_the_matrix() {
    let law = fixture(LIFECYCLE);
    let rows = law["actions"].as_array().expect("actions");
    assert_eq!(rows.len(), ToolRunAction::ALL.len());
    for (row, action) in rows.iter().zip(ToolRunAction::ALL) {
        assert_eq!(action.id(), text(&row["id"]));
        assert_eq!(ToolRunAction::from_id(action.id()), Some(action));
        assert_eq!(action.chord(), text(&row["chord"]));
        assert_eq!(action.label().key(), text(&row["label"]));
        let args: Vec<(&str, bool)> = row["args"].as_array().expect("args").iter().map(|arg| (text(&arg["name"]), arg["required"].as_bool().expect("required"))).collect();
        assert_eq!(action.args().iter().map(|arg| (arg.name, arg.required)).collect::<Vec<_>>(), args);
        let legal: Vec<&str> = row["legalIn"].as_array().expect("legalIn").iter().map(text).collect();
        for state in ToolRunState::ALL.map(Some).into_iter().chain([None]) {
            let name = state.map_or("none", ToolRunState::as_str);
            assert_eq!(action.is_legal_in(state), legal.contains(&name), "{} in {name}", action.id());
            let key = match action {
                ToolRunAction::Start => ToolRunEventKey::Start,
                ToolRunAction::Pause => ToolRunEventKey::Pause,
                ToolRunAction::Resume => ToolRunEventKey::Resume,
                ToolRunAction::Step => ToolRunEventKey::Step,
                ToolRunAction::Abort => ToolRunEventKey::Abort,
                ToolRunAction::Finalize => ToolRunEventKey::Finalize,
                ToolRunAction::Dismiss => ToolRunEventKey::Dismiss,
            };
            let current = state.map(|state| ToolRunSlot { run: 1, generation: 0, state });
            assert_eq!(ToolRunMachine::apply(current, matrix_event(key, 1, 0)).is_ok(), action.is_legal_in(state), "{} matrix parity in {name}", action.id());
        }
    }
    assert_eq!(TOOL_RUN_ACTION_IDS, ToolRunAction::ALL.map(ToolRunAction::id));
}

#[test]
fn labels_reserved_reasons_and_templates_match_the_fixture() {
    let law = fixture(LIFECYCLE);
    let rows = law["labels"].as_array().expect("labels");
    assert_eq!(rows.len(), ToolRunLabel::ALL.len());
    for (row, label) in rows.iter().zip(ToolRunLabel::ALL) {
        assert_eq!(label.key(), text(&row["key"]));
        assert_eq!(ToolRunLabel::parse(label.key()), Some(label));
        assert_eq!(label.text(Locale::En), text(&row["en"]));
        assert_eq!(label.text(Locale::De), text(&row["de"]));
        let localized = label.localized();
        assert_eq!(localized.resolve(ui::wgpu::Terminology::Native, Locale::De), text(&row["de"]));
        assert_eq!(localized.resolve(ui::wgpu::Terminology::Reuse, Locale::En), text(&row["en"]));
    }
    for state in ToolRunState::ALL {
        assert_eq!(state.label().key(), format!("state{}{}", state.as_str()[..1].to_uppercase(), &state.as_str()[1..]));
    }
    for row in law["reservedReasons"].as_array().expect("reasons") {
        let code = u16_of(&row["code"]);
        assert!(code >= TOOL_RUN_RESERVED_REASON_FLOOR);
        assert_eq!(ToolRunLabel::for_reason(code).map(ToolRunLabel::key), Some(text(&row["label"])));
    }
    assert_eq!(ToolRunLabel::for_reason(TOOL_RUN_RESERVED_REASON_FLOOR - 1), None);
    for row in law["templates"].as_array().expect("templates") {
        let values = row["values"].as_object().expect("values");
        assert_eq!(tool_run_format(text(&row["template"]), |name| values.get(name).map(|value| text(value).to_string())), text(&row["expected"]));
    }
}

#[test]
fn limits_match_the_fixture() {
    let limits = &fixture(LIFECYCLE)["limits"];
    assert_eq!(limits["stepRingCapacity"], TOOL_RUN_STEP_RING_CAPACITY);
    assert_eq!(limits["stepArgsMax"], TOOL_RUN_STEP_ARGS_MAX);
    assert_eq!(limits["countersMax"], TOOL_RUN_COUNTERS_MAX);
    assert_eq!(limits["provisionalOpsMax"], TOOL_RUN_PROVISIONAL_OPS_MAX);
    assert_eq!(limits["traceResidentRecords"], TOOL_RUN_TRACE_RESIDENT_RECORDS);
    assert_eq!(limits["tracePageOpsMax"], TOOL_RUN_TRACE_PAGE_OPS_MAX);
    assert_eq!(limits["tracePageBytesMax"], TOOL_RUN_TRACE_PAGE_BYTES_MAX);
    assert_eq!(limits["tickBytesMax"], TOOL_RUN_TICK_BYTES_MAX);
    assert_eq!(limits["traceLogCompactFloor"], TOOL_RUN_TRACE_LOG_COMPACT_FLOOR);
    assert_eq!(limits["statusAnnounceIntervalMs"], TOOL_RUN_STATUS_ANNOUNCE_INTERVAL_MS);
    assert_eq!(limits["reservedReasonFloor"], TOOL_RUN_RESERVED_REASON_FLOOR);
}
//#endregion 🔖️ActionsAndLabels

//#region 🔖️Definition
#[test]
fn definition_round_trips_through_serde_and_value_and_validates() {
    let law = fixture(LIFECYCLE);
    let definition: ToolRunDefinition = serde_json::from_value(law["definition"].clone()).expect("definition deserializes");
    assert_eq!(definition.validate(), Ok(()));
    assert_eq!(serde_json::to_value(&definition).expect("serializes"), law["definition"]);
    assert_eq!(ToolRunDefinition::from_value(definition.to_value()).expect("value round trip"), definition);
    assert_eq!(definition.stage(1).map(|stage| stage.id.as_str()), Some("test"));
    assert_eq!(definition.reason(2).map(|reason| reason.verdict), Some(ToolRunVerdict::Danger));
    assert_eq!(definition.run_job.as_str(), "fillRun");
    let mut unknown = law["definition"].clone();
    unknown["extra"] = json!(true);
    assert!(serde_json::from_value::<ToolRunDefinition>(unknown).is_err());
    let mut invalid = definition.clone();
    invalid.reasons[1].code = TOOL_RUN_REASON_CONFLICT;
    assert_eq!(invalid.validate(), Err(ToolRunDefinitionError::ReservedReasonCode(TOOL_RUN_REASON_CONFLICT)));
    let mut invalid = definition.clone();
    invalid.reasons[1].code = 1;
    assert_eq!(invalid.validate(), Err(ToolRunDefinitionError::DuplicateReasonCode(1)));
    let mut invalid = definition.clone();
    invalid.stages[2].id = "plan".into();
    assert_eq!(invalid.validate(), Err(ToolRunDefinitionError::DuplicateStageId("plan".into())));
    let mut invalid = definition.clone();
    invalid.counters = (0..9).map(|index| ToolRunCounterDefinition { id: format!("c{index}"), label: LocalizedLabel::native("C", "C") }).collect();
    assert_eq!(invalid.validate(), Err(ToolRunDefinitionError::TooManyCounters));
    let mut invalid = definition.clone();
    invalid.settings.window_config.insert("main".into(), vec!["/grid/~2".into()]);
    assert_eq!(invalid.validate(), Err(ToolRunDefinitionError::InvalidSettingsPointer("/grid/~2".into())));
    let mut undeclared = law["definition"].clone();
    undeclared.as_object_mut().expect("definition object").remove("settings");
    let undeclared: ToolRunDefinition = serde_json::from_value(undeclared).expect("a definition without settings deserializes");
    assert!(undeclared.settings.is_empty(), "an absent settings declaration reads no settings");
    assert!(serde_json::to_value(&undeclared).expect("serializes").get("settings").is_none(), "an empty declaration stays off the wire");
    let mut invalid = definition;
    invalid.stages.clear();
    assert_eq!(invalid.validate(), Err(ToolRunDefinitionError::NoStages));
}

/// ⚖️ LAW: every `settingsPointers` row resolves to its fixture value over the fixture document, exactly as the
/// `serde_json` RFC 6901 implementation (`Value::pointer`, the oracle) resolves it; malformed pointers name nothing.
#[test]
fn settings_pointers_resolve_like_the_rfc_6901_oracle() {
    let law = fixture(LIFECYCLE);
    let cases = &law["settingsPointers"];
    let document = dsl::DslValue::from(cases["document"].clone());
    for row in cases["rows"].as_array().expect("rows") {
        let pointer = text(&row["pointer"]);
        let expected = (!row["resolves"].is_null()).then(|| dsl::DslValue::from(row["resolves"].clone()));
        assert_eq!(tool_run_pointer_value(&document, pointer).cloned(), expected, "{pointer}");
        assert_eq!(cases["document"].pointer(pointer).cloned().map(dsl::DslValue::from), expected, "{pointer}: the oracle agrees");
    }
    for pointer in cases["malformed"].as_array().expect("malformed").iter().map(text) {
        assert_eq!(tool_run_pointer_tokens(pointer), None, "{pointer}");
        assert_eq!(tool_run_pointer_value(&document, pointer), None, "{pointer}");
    }
}
//#endregion 🔖️Definition

//#region 🔖️Trace
#[test]
fn trace_pages_encode_to_and_decode_from_the_fixture_bytes() {
    let fixture = fixture(TRACE_PAGES);
    for row in fixture["pages"].as_array().expect("pages") {
        let name = text(&row["name"]);
        let expected = page(&row["page"]);
        assert_eq!(to_hex(&expected.encode().expect("encodes")), text(&row["hex"]), "{name}");
        assert_eq!(ToolRunTracePage::decode(&hex(&row["hex"])).expect("decodes"), expected, "{name}");
    }
    for row in fixture["deltas"].as_array().expect("deltas") {
        let name = text(&row["name"]);
        let expected = delta(&row["delta"]);
        assert_eq!(to_hex(&expected.encode().expect("encodes")), text(&row["hex"]), "{name}");
        assert_eq!(ToolRunTraceDelta::decode(&hex(&row["hex"])).expect("decodes"), expected, "{name}");
    }
}

#[test]
fn malformed_wire_values_are_rejected() {
    for row in fixture(TRACE_PAGES)["malformed"].as_array().expect("malformed") {
        let bytes = hex(&row["hex"]);
        let rejected = match text(&row["target"]) {
            "page" => ToolRunTracePage::decode(&bytes).is_err(),
            "delta" => ToolRunTraceDelta::decode(&bytes).is_err(),
            _ => ToolRunTick::decode(&bytes).is_err(),
        };
        assert!(rejected, "{}", text(&row["name"]));
    }
}

#[test]
fn trace_page_codec_keeps_full_u64_range_and_enforces_the_op_cap() {
    let identity = ToolRunIdentity::new(ToolRunId { app_instance_id: u32::MAX, run: u64::MAX }, [0xAB; 32]).next_generation();
    let page = ToolRunTracePage { identity, page: u32::MAX, ops: vec![ToolRunTraceOp::Upsert { key: u64::MAX, verdict: ToolRunVerdict::Warning, reason: u16::MAX, subject: ToolRunTraceSubject::Entity { entity: u64::MAX } }, ToolRunTraceOp::Retire { key: u64::MAX - 1 }] };
    assert_eq!(ToolRunTracePage::decode(&page.encode().expect("encodes")).expect("decodes"), page);
    let full = ToolRunTracePage { identity, page: 0, ops: vec![ToolRunTraceOp::Clear; TOOL_RUN_TRACE_PAGE_OPS_MAX] };
    assert!(full.encode().is_ok());
    let over = ToolRunTracePage { identity, page: 0, ops: vec![ToolRunTraceOp::Clear; TOOL_RUN_TRACE_PAGE_OPS_MAX + 1] };
    assert_eq!(over.encode(), Err(ToolRunCodecError::Limit("trace page ops")));
    assert_eq!(identity.id.group_id(), format!("toolRun:{}", u64::MAX));
    assert!(identity.freshness(0) > ToolRunIdentity::new(identity.id, [0; 32]).freshness(u64::MAX));
}

#[test]
fn trace_store_residency_matches_the_fixture() {
    for case in fixture(TRACE_PAGES)["residency"].as_array().expect("residency") {
        let name = text(&case["name"]);
        let (store, applied) = store_for(case);
        let expected_applied: Vec<Value> = case["applied"].as_array().expect("applied").clone();
        let observed_applied: Vec<Value> = applied.iter().map(|apply| json!({ "logged": apply.logged, "evicted": apply.evicted, "overflowed": apply.overflowed, "compacted": apply.compacted })).collect();
        assert_eq!(observed_applied, expected_applied, "{name}");
        let mut resident: Vec<(u64, &ToolRunTraceRecord)> = store.records().collect();
        resident.sort_unstable_by_key(|(key, _)| *key);
        let observed: Vec<Value> = resident.iter().map(|(key, record)| json!({ "key": key, "verdict": record.verdict.as_str(), "reason": record.reason })).collect();
        assert_eq!(Value::Array(observed), case["resident"], "{name}");
        assert_eq!(store.log_base(), u32_of(&case["logBase"]), "{name}");
        let log_pages: Vec<Vec<ToolRunTraceOp>> = (store.log_base()..store.next_page()).map(|page| store.log_page(page).expect("retained").to_vec()).collect();
        assert_eq!(log_pages, case["logPages"].as_array().expect("logPages").iter().map(ops).collect::<Vec<_>>(), "{name}");
    }
}

#[test]
fn trace_store_delivery_matches_the_fixture() {
    let fixture = fixture(TRACE_PAGES);
    let residency = fixture["residency"].as_array().expect("residency");
    for case in fixture["delivery"].as_array().expect("delivery") {
        let name = text(&case["name"]);
        let source = residency.iter().find(|row| row["name"] == case["residency"]).expect("residency case");
        let (mut store, _) = store_for(source);
        store.rebind(ToolRunIdentity { generation: u32_of(&case["generation"]), ..store.identity() });
        let cursor = (!case["cursor"].is_null()).then(|| ToolRunTraceCursor { run: u64_of(&case["cursor"]["run"]), generation: u32_of(&case["cursor"]["generation"]), page: u32_of(&case["cursor"]["page"]) });
        let delta = store.delta_after(cursor, case["byteBudget"].as_u64().expect("budget") as usize);
        assert_eq!(delta.clear, case["expect"]["clear"].as_bool().expect("clear"), "{name}");
        assert_eq!(delta.next, u32_of(&case["expect"]["next"]), "{name}");
        assert_eq!(delta.pages.iter().map(|page| page.page).collect::<Vec<_>>(), case["expect"]["pages"].as_array().expect("pages").iter().map(u32_of).collect::<Vec<_>>(), "{name}");
        assert!(delta.pages.iter().all(|page| page.identity == store.identity()), "{name}");
        assert_eq!(ToolRunTraceDelta::decode(&delta.encode().expect("encodes")).expect("decodes"), delta, "{name}");
    }
}

#[test]
fn trace_store_rejects_stale_pages_and_resets_on_a_new_run() {
    let identity = ToolRunIdentity::new(ToolRunId { app_instance_id: 1, run: 5 }, [0; 32]);
    let mut store = ToolRunTraceStore::with_limits(identity, 4, 4);
    let upsert = ToolRunTraceOp::Upsert { key: 1, verdict: ToolRunVerdict::Testing, reason: 0, subject: ToolRunTraceSubject::Entity { entity: 1 } };
    assert_eq!(store.apply_page(&ToolRunTracePage { identity: identity.next_generation(), page: 0, ops: vec![upsert] }), Err(ToolRunRejection::Stale));
    assert_eq!(store.apply_page(&ToolRunTracePage { identity, page: 9, ops: vec![upsert] }).map(|apply| apply.logged), Ok(1));
    store.rebind(identity.next_generation());
    assert_eq!(store.len(), 1);
    store.rebind(ToolRunIdentity::new(ToolRunId { app_instance_id: 1, run: 6 }, [0; 32]));
    assert!(store.is_empty());
    assert_eq!((store.log_base(), store.next_page()), (0, 0));
}

#[test]
fn trace_store_keeps_every_record_of_a_five_thousand_candidate_run() {
    let identity = ToolRunIdentity::new(ToolRunId { app_instance_id: 1, run: 1 }, [0; 32]);
    let mut store = ToolRunTraceStore::new(identity);
    let mut renderer: HashMap<u64, ToolRunVerdict> = HashMap::new();
    let mut cursor = None;
    for candidate in 0..5_000u64 {
        let verdict = if candidate % 3 == 0 { ToolRunVerdict::Success } else { ToolRunVerdict::Danger };
        let subject = ToolRunTraceSubject::Instance3d { mesh: 0, position: [candidate as f32, 0.0, 0.0], rotation: [0.0, 0.0, 0.0, 1.0], scale: 1.0 };
        store.apply_ops(&[ToolRunTraceOp::Upsert { key: candidate, verdict: ToolRunVerdict::Testing, reason: 0, subject }, ToolRunTraceOp::Upsert { key: candidate, verdict, reason: 1, subject }]);
        if candidate % 97 == 0 {
            let delta = store.delta_after(cursor, 64_000);
            if delta.clear {
                renderer.clear();
            }
            for page in &delta.pages {
                for op in &page.ops {
                    match *op {
                        ToolRunTraceOp::Upsert { key, verdict, .. } => {
                            renderer.insert(key, verdict);
                        }
                        ToolRunTraceOp::Retire { key } => {
                            renderer.remove(&key);
                        }
                        ToolRunTraceOp::Clear => renderer.clear(),
                    }
                }
            }
            cursor = Some(ToolRunTraceCursor { run: 1, generation: 0, page: delta.next });
        }
    }
    loop {
        let delta = store.delta_after(cursor, 64_000);
        if delta.clear {
            renderer.clear();
        }
        for op in delta.pages.iter().flat_map(|page| page.ops.iter()) {
            match *op {
                ToolRunTraceOp::Upsert { key, verdict, .. } => {
                    renderer.insert(key, verdict);
                }
                ToolRunTraceOp::Retire { key } => {
                    renderer.remove(&key);
                }
                ToolRunTraceOp::Clear => renderer.clear(),
            }
        }
        cursor = Some(ToolRunTraceCursor { run: 1, generation: 0, page: delta.next });
        if delta.pages.is_empty() {
            break;
        }
    }
    assert_eq!(store.len(), 5_000);
    assert_eq!(renderer.len(), 5_000);
    assert!(store.records().all(|(key, record)| renderer.get(&key) == Some(&record.verdict)));
}
//#endregion 🔖️Trace

//#region 🔖️Tick
#[test]
fn ticks_encode_to_and_decode_from_the_fixture_bytes() {
    for row in fixture(TICKS)["ticks"].as_array().expect("ticks") {
        let name = text(&row["name"]);
        let expected = tick(&row["tick"]);
        assert_eq!(to_hex(&expected.encode().expect("encodes")), text(&row["hex"]), "{name}");
        assert_eq!(ToolRunTick::decode(&hex(&row["hex"])).expect("decodes"), expected, "{name}");
    }
}

#[test]
fn step_ring_matches_the_fixture() {
    for case in fixture(TICKS)["stepRing"].as_array().expect("stepRing") {
        let name = text(&case["name"]);
        let mut ring = ToolRunStepRing::new();
        for push in case["push"].as_array().expect("push") {
            if let Some(repeat) = push.get("repeatSequence") {
                let from = u64_of(&repeat["from"]);
                for sequence in from..from + u64_of(&repeat["count"]) {
                    ring.push(ToolRunStep::new(sequence, ToolRunStepKind::parse(text(&repeat["kind"])).expect("kind"), 0, sequence as u16));
                }
            } else {
                ring.push(step(push));
            }
        }
        assert_eq!(ring.len(), case["expect"]["length"].as_u64().expect("length") as usize, "{name}");
        let expected = |key: &str| (!case["expect"][key].is_null()).then(|| step(&case["expect"][key]));
        assert_eq!(ring.oldest().cloned(), expected("first"), "{name}");
        assert_eq!(ring.newest().cloned(), expected("last"), "{name}");
    }
}

#[test]
fn tick_writer_matches_the_fixture() {
    let identity = ToolRunIdentity::new(ToolRunId { app_instance_id: 1, run: 1 }, [0; 32]);
    for case in fixture(TICKS)["writer"].as_array().expect("writer") {
        let name = text(&case["name"]);
        let mut writer = ToolRunTickWriter::new(identity);
        for key in 0..u64_of(&case["upserts"]) {
            writer.upsert(key, ToolRunVerdict::Testing, 0, ToolRunTraceSubject::Entity { entity: key });
        }
        let rejected = (0..u64_of(&case["appendOps"])).filter(|index| writer.append_op(vec![*index as u8]).is_err()).count();
        let first = writer.finish();
        let retract = case["retractTo"].as_u64().map(|len| {
            writer.retract_to(len as u32);
            writer.finish().and_then(|tick| tick.retract_to)
        });
        let expect = &case["expect"];
        let page_ops: Vec<usize> = first.as_ref().map_or_else(Vec::new, |tick| tick.trace.iter().map(|page| page.ops.len()).collect());
        assert_eq!(page_ops, expect["pageOps"].as_array().expect("pageOps").iter().map(|ops| ops.as_u64().expect("ops") as usize).collect::<Vec<_>>(), "{name}");
        assert_eq!(first.as_ref().map_or(0, |tick| tick.append_ops.len()), expect["appendOps"].as_u64().expect("appendOps") as usize, "{name}");
        assert_eq!(retract.flatten(), expect["retractTo"].as_u64().map(|len| len as u32), "{name}");
        assert_eq!(rejected, expect["rejectedOps"].as_u64().expect("rejected") as usize, "{name}");
        if let Some(tick) = first.filter(|tick| tick.trace.iter().map(|page| page.ops.len()).sum::<usize>() < 64) {
            assert_eq!(ToolRunTick::decode(&tick.encode().expect("encodes")).expect("decodes"), tick, "{name}");
        }
    }
}

#[test]
fn tick_writer_resumes_from_a_provisional_base_and_retracts_its_entities() {
    let identity = ToolRunIdentity::new(ToolRunId { app_instance_id: 1, run: 4 }, [0; 32]);
    for case in fixture(TICKS)["writerResume"].as_array().expect("writerResume") {
        let name = text(&case["name"]);
        let mut writer = ToolRunTickWriter::with_provisional_base(identity, u64_of(&case["provisionalBase"]) as u32);
        let mut ticks = Vec::new();
        let mut rejected = 0;
        for step in case["script"].as_array().expect("script") {
            if let Some(count) = step["append"].as_u64() {
                rejected += (0..count).filter(|index| writer.append_op(vec![*index as u8]).is_err()).count();
            } else if !step["entity"].is_null() {
                writer.append_entity(u64_of(&step["entity"]));
            } else if let Some(len) = step["retractTo"].as_u64() {
                writer.retract_to(len as u32);
            } else {
                ticks.extend(writer.finish());
            }
        }
        let expect = &case["expect"];
        let expected: Vec<(usize, Vec<u64>, Option<u32>)> = expect["ticks"]
            .as_array()
            .expect("ticks")
            .iter()
            .map(|tick| (tick["appendOps"].as_u64().expect("appendOps") as usize, tick["appendEntities"].as_array().expect("appendEntities").iter().map(u64_of).collect(), tick["retractTo"].as_u64().map(|len| len as u32)))
            .collect();
        assert_eq!(ticks.iter().map(|tick| (tick.append_ops.len(), tick.append_entities.clone(), tick.retract_to)).collect::<Vec<_>>(), expected, "{name}");
        assert_eq!(u64::from(writer.provisional_len()), u64_of(&expect["provisionalLen"]), "{name}");
        assert_eq!(rejected as u64, u64_of(&expect["rejectedOps"]), "{name}");
        for tick in ticks {
            assert_eq!(ToolRunTick::decode(&tick.encode().expect("encodes")).expect("decodes"), tick, "{name}");
        }
    }
}

#[test]
fn tool_run_trace_cursor_round_trips_the_schema_json_and_value_forms() {
    let cursor = ToolRunTraceCursor { run: 7, generation: 2, page: 5 };
    let json = serde_json::to_value(cursor).expect("cursor serializes");
    assert_eq!(json, serde_json::json!({ "run": 7, "generation": 2, "page": 5 }));
    assert_eq!(serde_json::from_value::<ToolRunTraceCursor>(json).expect("cursor deserializes"), cursor);
    assert!(serde_json::from_value::<ToolRunTraceCursor>(serde_json::json!({ "run": 7, "generation": 2, "page": 5, "extra": 1 })).is_err(), "unknown fields are refused");
    assert_eq!(<ToolRunTraceCursor as FromValue>::from_value(ToValue::to_value(&cursor)).expect("value form"), cursor);
}

#[test]
fn tick_writer_sequences_steps_pages_and_retracts_pending_appends() {
    let identity = ToolRunIdentity::new(ToolRunId { app_instance_id: 2, run: 3 }, [1; 32]);
    let mut writer = ToolRunTickWriter::new(identity);
    assert!(writer.finish().is_none());
    writer.step(ToolRunStepKind::Info, 0, 1, None, &[ToolRunStepArg::Unsigned(1)]).expect("step");
    assert_eq!(writer.step(ToolRunStepKind::Info, 0, 1, None, &[ToolRunStepArg::Unsigned(1); 5]), Err(ToolRunLimitError::StepArgs));
    for op in 0..5u8 {
        writer.append_op(vec![op]).expect("append");
    }
    writer.retract_to(2);
    writer.append_entity(9);
    writer.upsert(1, ToolRunVerdict::Success, 1, ToolRunTraceSubject::Entity { entity: 9 });
    let first = writer.finish().expect("first tick");
    assert_eq!((first.sequence, first.append_ops.len(), first.retract_to, first.steps[0].sequence, first.trace[0].page), (0, 2, None, 0, 0));
    writer.rebind(identity.next_generation());
    writer.step(ToolRunStepKind::Warning, 0, 2, Some(1), &[]).expect("step");
    writer.clear_trace();
    let second = writer.finish().expect("second tick");
    assert_eq!((second.sequence, second.steps[0].sequence, second.trace[0].page, second.identity.generation, writer.provisional_len()), (1, 1, 1, 1, 2));
    assert!(!writer.should_flush());
}

#[test]
fn tick_codec_enforces_the_tick_byte_cap() {
    let identity = ToolRunIdentity::new(ToolRunId { app_instance_id: 0, run: 0 }, [0; 32]);
    let tick = ToolRunTick { identity, sequence: 0, progress: None, steps: Vec::new(), trace: Vec::new(), append_ops: vec![vec![0; TOOL_RUN_TICK_BYTES_MAX]], append_entities: Vec::new(), retract_to: None, payload: None };
    assert_eq!(tick.encode(), Err(ToolRunCodecError::Limit("tick bytes")));
}
//#endregion 🔖️Tick

/// ⚖️ LAW: a writer's pending payload makes a tick on its own, a later payload before `finish` replaces the earlier one,
/// and `finish` hands it over once.
#[test]
fn writer_payload_is_the_latest_and_makes_a_tick() {
    let fixture = fixture(TICKS);
    let row = fixture["ticks"].as_array().expect("ticks").iter().find(|row| row["name"] == "payload tick").expect("payload tick");
    let mut writer = ToolRunTickWriter::new(identity(&row["tick"]["identity"]));
    writer.payload(b"stale".to_vec());
    writer.payload(hex(&row["tick"]["payload"]));
    assert!(!writer.is_empty(), "a payload alone is pending work");
    let tick = writer.finish().expect("a payload tick");
    assert_eq!(tick.payload.as_deref(), Some(hex(&row["tick"]["payload"]).as_slice()));
    assert!(writer.finish().is_none(), "the payload is handed over once");
}
