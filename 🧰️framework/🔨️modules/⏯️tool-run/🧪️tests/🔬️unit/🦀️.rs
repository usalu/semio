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
    assert_eq!(ids, ["finalizeOnlyWithAResult", "storeGenerationOnlyOnFinalized", "abortAndFaultLeaveZeroTrace", "pauseAndStepNeverCommit", "singleNonTerminalRun"]);
    for state in ToolRunState::ALL.map(Some).into_iter().chain([None]) {
        let current = state.map(|state| ToolRunSlot { run: 1, generation: 0, state });
        for key in ToolRunEventKey::ALL {
            let outcome = ToolRunMachine::apply(current, matrix_event(key, 1, 0));
            if key == ToolRunEventKey::Finalize {
                assert_eq!(outcome.is_ok(), matches!(state, Some(ToolRunState::Running | ToolRunState::Paused | ToolRunState::Complete)));
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
        assert_eq!(localized.resolve(semio_framework_ui_locale::Terminology::Native, Locale::De), text(&row["de"]));
        assert_eq!(localized.resolve(semio_framework_ui_locale::Terminology::Reuse, Locale::En), text(&row["en"]));
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
    assert_eq!(limits["memberOpsMax"], TOOL_RUN_MEMBER_OPS_MAX);
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
    let mut member = law["definition"].clone();
    member["member"] = json!("content");
    let member: ToolRunDefinition = serde_json::from_value(member).expect("a member-target definition deserializes");
    assert_eq!((member.member.as_deref(), member.validate()), (Some("content"), Ok(())));
    assert_eq!(ToolRunDefinition::from_value(member.to_value()).expect("member value round trip"), member);
    assert!(serde_json::to_value(&definition).expect("serializes").get("member").is_none(), "a document-target run carries no member on the wire");
    assert_eq!(ToolRunDefinition { member: Some(String::new()), ..member.clone() }.validate(), Err(ToolRunDefinitionError::EmptyMember));
    assert_eq!(ToolRunDefinition { mutating: false, ..member }.validate(), Err(ToolRunDefinitionError::ReadOnlyMember));
    let mut invalid = definition;
    invalid.stages.clear();
    assert_eq!(invalid.validate(), Err(ToolRunDefinitionError::NoStages));
}

/// ⚖️ LAW: every `settingsPointers` row resolves to its fixture value over the fixture artifact, exactly as the
/// `serde_json` RFC 6901 implementation (`Value::pointer`, the oracle) resolves it; malformed pointers name nothing.
#[test]
fn settings_pointers_resolve_like_the_rfc_6901_oracle() {
    let law = fixture(LIFECYCLE);
    let cases = &law["settingsPointers"];
    let document = semio_framework_value::DslValue::from(cases["artifact"].clone());
    for row in cases["rows"].as_array().expect("rows") {
        let pointer = text(&row["pointer"]);
        let expected = (!row["resolves"].is_null()).then(|| semio_framework_value::DslValue::from(row["resolves"].clone()));
        assert_eq!(tool_run_pointer_value(&document, pointer).cloned(), expected, "{pointer}");
        assert_eq!(cases["artifact"].pointer(pointer).cloned().map(semio_framework_value::DslValue::from), expected, "{pointer}: the oracle agrees");
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

/// ⚖️ LAW: every `admission` row — one non-terminal mutating run per actor, concurrent read-only runs keyed by tool and
/// window, and a start replacing exactly the terminal runs on its lane.
#[test]
fn start_admission_follows_the_lane_law() {
    let law = fixture(LIFECYCLE);
    let lane = |value: &Value| -> ToolRunLane { serde_json::from_value(value.clone()).expect("lane") };
    for row in law["admission"].as_array().expect("admission") {
        let live: Vec<(ToolRunLane, ToolRunState)> = row["live"].as_array().expect("live").iter().map(|entry| (lane(&entry["lane"]), ToolRunState::parse(text(&entry["state"])).expect("state"))).collect();
        let expected = match row.get("rejection") {
            Some(rejection) => Err(text(rejection).to_string()),
            None => Ok(row["replaces"].as_array().expect("replaces").iter().map(|index| index.as_u64().expect("index") as usize).collect::<Vec<_>>()),
        };
        assert_eq!(lane(&row["lane"]).admit(&live).map_err(|rejection| rejection.code().to_string()), expected, "{}", text(&row["name"]));
    }
}

/// ⚖️ LAW: `panelGroups` — which keys name a run's panel group, and which runs a rendered panel adds for a host to reveal.
#[test]
fn panel_group_keys_and_reveals_follow_the_fixture() {
    let law = fixture(LIFECYCLE);
    let cases = &law["panelGroups"];
    assert_eq!(text(&cases["rootId"]), TOOL_RUN_PANEL_ID);
    for row in cases["keys"].as_array().expect("keys") {
        assert_eq!(tool_run_panel_group_run(text(&row["key"])), row["run"].as_u64(), "{}", row["key"]);
        if let Some(run) = row["run"].as_u64() {
            assert_eq!(tool_run_panel_group_id(run), text(&row["key"]));
        }
    }
    let runs = |value: &Value| value.as_array().expect("runs").iter().map(u64_of).collect::<std::collections::BTreeSet<u64>>();
    for row in cases["reveals"].as_array().expect("reveals") {
        let (current, added) = tool_run_panel_new_runs(&runs(&row["previous"]), row["keys"].as_array().expect("keys").iter().map(text));
        assert_eq!((current, added.into_iter().collect::<std::collections::BTreeSet<u64>>()), (runs(&row["current"]), runs(&row["added"])), "{}", text(&row["name"]));
    }
}

#[test]
fn original_tool_run_progress_metadata_preserves_all_native_leaves_until_funded_retirement(){
 fn observe_retirement_allocations<R>(operation:impl FnOnce()->R)->(R,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(operation);(value,(heap.requested_bytes,heap.released_bytes))}
 use semio_framework_value::{retained_clone::RetainedCloneGrant,retirement::controlled::ControlledRetirement};
 let corpus=fixture(TICKS);
 for vector in corpus["ticks"].as_array().unwrap().iter().filter(|row|row["tick"].get("progress").is_some()){
  let expected=&vector["tick"]["progress"];let(original,heap)=observe_retirement_allocations(||progress(expected));let original_bytes=heap.0-heap.1;
  assert_eq!(original.identity,identity(&expected["identity"]));assert_eq!(original.state.as_str(),expected["state"].as_str().unwrap());assert_eq!(original.steps.len(),expected["steps"].as_array().unwrap().len());assert_eq!(original.counters.len(),expected["counters"].as_array().unwrap().len());
  let(mut close,heap)=observe_retirement_allocations(||ControlledRetirement::new(original).unwrap_or_else(|_|panic!("ToolRun progress must own its actual typed metadata fields")));assert_eq!(heap,(0,0));let(mut born,mut released)=(0,0);
  for _ in 0..100000{if close.terminal_is_empty(){break;}let copy=close.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:close.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:close.next_release_byte_demand().unwrap(),maximum_depth:close.next_depth_demand().unwrap()};
   for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant}),grant.maximum_capacity_bytes.checked_sub(1).map(|v|RetainedCloneGrant{maximum_capacity_bytes:v,..grant}),grant.maximum_release_bytes.checked_sub(1).map(|v|RetainedCloneGrant{maximum_release_bytes:v,..grant})].into_iter().flatten(){let(step,heap)=observe_retirement_allocations(||close.step(denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert!(!close.terminal_is_empty());}
   let(step,heap)=observe_retirement_allocations(||close.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;
  }
  assert!(close.terminal_is_empty());assert_eq!(original_bytes+born,released);assert_eq!(observe_retirement_allocations(||drop(close)).1,(0,0));println!("[DEBUG] original ToolRun progress metadata original={original_bytes} born={born} released={released}");
 }
}

#[test]
fn original_tool_run_progress_metadata_clone_preserves_original_fields_and_physical_custody(){
 use semio_framework_value::{retained_clone::{RetainedCloneSource,RetainedCloneGrant},retirement::{RetireOwned,controlled::ControlledRetirement}};
 use super::progress_clone::ToolRunProgressClone;
 fn measured<R>(work:impl FnOnce()->R)->(R,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(work);(value,(heap.requested_bytes,heap.released_bytes))}
 fn drain<T:RetireOwned>(value:T)->(usize,usize){let mut owner=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("progress clone retains its actual original owners"));let(mut born,mut released)=(0,0);for _ in 0..100000{if owner.terminal_is_empty(){break;}let copy=owner.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};let(step,heap)=measured(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;}assert!(owner.terminal_is_empty());assert_eq!(measured(||drop(owner)).1,(0,0));(born,released)}
 let corpus=fixture(TICKS);let law:Value=serde_json::from_str(include_str!("../../📊️progress/📸️clone/🧫️fixtures/🔣️.json")).unwrap();
 let mut vectors=corpus["ticks"].as_array().unwrap().clone();let mut maximum=vectors[0].clone();maximum["tick"]["progress"]["counters"]=Value::Array((0..law["maximumLeaves"]["counters"].as_u64().unwrap()).map(|counter|serde_json::json!({"counter":counter,"value":counter+1})).collect());let template=maximum["tick"]["steps"][0].clone();maximum["tick"]["progress"]["steps"]=Value::Array((0..law["maximumLeaves"]["steps"].as_u64().unwrap()).map(|index|{let mut step=template.clone();step["sequence"]=Value::from(index+1);step["args"]=Value::Array((0..law["maximumLeaves"]["arguments"].as_u64().unwrap()).map(|_|template["args"][0].clone()).collect());step}).collect());vectors.push(maximum);
 for vector in vectors.iter().filter(|row|row["tick"].get("progress").is_some()){for stop in std::iter::once(None).chain(law["interruptAfter"].as_array().unwrap().iter().map(|value|Some(value.as_u64().unwrap()as usize))){
  let expected=&vector["tick"]["progress"];let(original,heap)=measured(||progress(expected));let original_bytes=heap.0-heap.1;let counters=original.counters.as_ptr();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:RetainedCloneSource::<ToolRunProgress>::constructor_copy_bytes(),maximum_capacity_bytes:RetainedCloneSource::<ToolRunProgress>::owned_constructor_capacity_bytes::<()>(),maximum_depth:1,..Default::default()};let(admitted,heap)=measured(||RetainedCloneSource::admit_owned(original,(),grant));let(source,receipt)=admitted.map_err(|(error,_,_)|error).unwrap();assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));let(mut born,mut released)=(heap.0,heap.1);let(mut job,heap)=measured(||ToolRunProgressClone::new(source));assert_eq!(heap,(0,0));let mut turns=0;
  while !job.complete()&&turns<stop.unwrap_or(10000){let quote=job.next_demands().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:quote.copy_bytes,maximum_capacity_bytes:quote.capacity_bytes,maximum_release_bytes:quote.release_bytes,maximum_depth:quote.depth};for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant}),grant.maximum_copy_bytes.checked_sub(1).map(|v|RetainedCloneGrant{maximum_copy_bytes:v,..grant}),grant.maximum_capacity_bytes.checked_sub(1).map(|v|RetainedCloneGrant{maximum_capacity_bytes:v,..grant}),grant.maximum_release_bytes.checked_sub(1).map(|v|RetainedCloneGrant{maximum_release_bytes:v,..grant}),grant.maximum_depth.checked_sub(1).map(|v|RetainedCloneGrant{maximum_depth:v,..grant})].into_iter().flatten(){let(progress,heap)=measured(||job.advance(denied).unwrap());assert_eq!(progress,Default::default());assert_eq!(heap,(0,0));assert_eq!(quote,job.next_demands().unwrap());}assert_eq!(job.original().counters.as_ptr(),counters);let(progress,heap)=measured(||job.advance(grant).unwrap());assert!(progress.fits(grant));assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.0;released+=heap.1;turns+=1;}
  if stop.is_none(){assert!(job.complete());let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<ToolRunProgress>(),maximum_depth:1,..Default::default()};assert!(job.take_output(RetainedCloneGrant{maximum_copy_bytes:0,..grant}).unwrap().is_none());let(output,heap)=measured(||job.take_output(grant).unwrap());assert_eq!(heap,(0,0));let(output,receipt)=output.unwrap();assert!(receipt.fits(grant));assert_eq!(&output,job.original());assert!(job.take_output(grant).unwrap().is_none());let heap=drain(output);born+=heap.0;released+=heap.1;}else{job.cancel();assert!(job.take_output(Default::default()).unwrap().is_none());}
  let heap=drain(job);born+=heap.0;released+=heap.1;assert_eq!(original_bytes+born,released);println!("[DEBUG] ToolRun original progress clone stop={stop:?} turns={turns} original={original_bytes} born={born} physical={released}");
 }}
}

#[test]
fn original_tool_run_progress_metadata_pending_tick_and_writer_keep_all_actual_fields(){
 use semio_framework_value::{retained_clone::RetainedCloneGrant,retirement::{RetireOwned,controlled::ControlledRetirement}};
 fn measured<R>(work:impl FnOnce()->R)->(R,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(work);(value,(heap.requested_bytes,heap.released_bytes))}
 fn close<T:RetireOwned>(value:T,original:usize,label:&str){let mut owner=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("pending ToolRun original requires typed retirement"));let(mut born,mut released)=(0,0);for _ in 0..100000{if owner.terminal_is_empty(){break;}let copy=owner.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant}),grant.maximum_copy_bytes.checked_sub(1).map(|x|RetainedCloneGrant{maximum_copy_bytes:x,..grant}),grant.maximum_capacity_bytes.checked_sub(1).map(|x|RetainedCloneGrant{maximum_capacity_bytes:x,..grant}),grant.maximum_release_bytes.checked_sub(1).map(|x|RetainedCloneGrant{maximum_release_bytes:x,..grant}),grant.maximum_depth.checked_sub(1).map(|x|RetainedCloneGrant{maximum_depth:x,..grant})].into_iter().flatten(){let(result,heap)=measured(||owner.step(denied));assert_eq!(heap,(0,0));match result{Ok(step)=>assert_eq!(step.progress(),Default::default()),Err(error)=>assert!(matches!(error.kind,semio_framework_value::ValueRefusalKind::DepthLimit|semio_framework_value::ValueRefusalKind::OwnershipLimit|semio_framework_value::ValueRefusalKind::WorkLimit))};}let(step,heap)=measured(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;}assert!(owner.terminal_is_empty());assert_eq!(original+born,released);assert_eq!(measured(||drop(owner)).1,(0,0));println!("[DEBUG] {label} original={original} born={born} physical={released}");}
 let law:Value=serde_json::from_str(include_str!("../../♻️retirement/🧫️fixtures/🔣️.json")).unwrap();let corpus=fixture(TICKS);
 for row in corpus["ticks"].as_array().unwrap(){for kind in law["owners"].as_array().unwrap(){
  if kind=="pendingTick"{let(value,heap)=measured(||tick(&row["tick"]));assert_eq!(value.identity,identity(&row["tick"]["identity"]));assert_eq!(value.append_entities.len(),row["tick"]["appendEntities"].as_array().unwrap().len());assert_eq!(value.trace.len(),row["tick"]["trace"].as_array().unwrap().len());close(value,heap.0-heap.1,"pending tick");}
  else{let(value,heap)=measured(||{let tick=tick(&row["tick"]);let mut writer=ToolRunTickWriter::new(tick.identity);writer.progress=tick.progress;writer.steps=tick.steps;writer.pages=tick.trace;writer.append_ops=tick.append_ops;writer.append_entities=tick.append_entities;writer.entity_marks=writer.append_entities.iter().map(|_|writer.append_ops.len()as u32).collect();writer.retract_to=tick.retract_to;writer.payload=tick.payload;if let Some(page)=writer.pages.pop(){writer.page_ops=page.ops;}writer});assert_eq!(value.identity,identity(&row["tick"]["identity"]));assert_eq!(value.append_entities.len(),row["tick"]["appendEntities"].as_array().unwrap().len());close(value,heap.0-heap.1,"pending writer");}
 }}
}

#[test]
fn original_tool_run_progress_metadata_step_ring_insertion_preserves_original_owners(){
 use semio_framework_value::{retained_clone::{RetainedCloneGrant,ordered_map::BoundedOrdGrant},retirement::{RetireOwned,controlled::ControlledRetirement}};
 use super::step_ring_insert::{ToolRunStepRingInsert,ToolRunStepRingInsertGrant};
 fn measured<R>(work:impl FnOnce()->R)->(R,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(work);(value,(heap.requested_bytes,heap.released_bytes))}
 fn drain<T:RetireOwned>(value:T)->(usize,usize){let mut owner=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("ring insertion retains original typed owners"));let(mut born,mut released)=(0,0);for _ in 0..100000{if owner.terminal_is_empty(){break;}let copy=owner.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};let(step,heap)=measured(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;}assert!(owner.terminal_is_empty());assert_eq!(measured(||drop(owner)).1,(0,0));(born,released)}
 let law:Value=serde_json::from_str(include_str!("../../📊️progress/💍️steps/➕️insert/🧫️fixtures/🔣️.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){for stop in std::iter::once(None).chain(law["interruptAfter"].as_array().unwrap().iter().map(|x|Some(x.as_u64().unwrap()as usize))){
  let((ring,input),heap)=measured(||{let ring=ToolRunStepRing::from_steps((0..row["initialCount"].as_u64().unwrap()).map(|index|{let mut value=step(&row["template"]);value.sequence=index+1;value}).collect()).unwrap();(ring,step(&row["input"]))});let original=heap.0-heap.1;let newest=ring.newest().map(|s|s.args.as_ptr());let input_pointer=input.args.as_ptr();let(mut job,heap)=measured(||ToolRunStepRingInsert::new(ring,input));assert_eq!(heap,(0,0));let(mut born,mut released,mut turns)=(0,0,0);
  while !job.complete()&&turns<stop.unwrap_or(10000){let demand=job.next_demands().unwrap();let grant=ToolRunStepRingInsertGrant{retirement:RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.retirement.copy_bytes,maximum_capacity_bytes:demand.retirement.capacity_bytes,maximum_release_bytes:demand.retirement.release_bytes,maximum_depth:demand.retirement.depth},comparison:BoundedOrdGrant{maximum_items:demand.compared_items,maximum_bytes:demand.compared_bytes},maximum_moved_items:demand.moved_items,maximum_moved_bytes:demand.moved_bytes};
   for denied in [Some(ToolRunStepRingInsertGrant{retirement:RetainedCloneGrant{maximum_items:0,..grant.retirement},..grant}),grant.retirement.maximum_copy_bytes.checked_sub(1).map(|x|ToolRunStepRingInsertGrant{retirement:RetainedCloneGrant{maximum_copy_bytes:x,..grant.retirement},..grant}),grant.retirement.maximum_capacity_bytes.checked_sub(1).map(|x|ToolRunStepRingInsertGrant{retirement:RetainedCloneGrant{maximum_capacity_bytes:x,..grant.retirement},..grant}),grant.retirement.maximum_release_bytes.checked_sub(1).map(|x|ToolRunStepRingInsertGrant{retirement:RetainedCloneGrant{maximum_release_bytes:x,..grant.retirement},..grant}),grant.retirement.maximum_depth.checked_sub(1).map(|x|ToolRunStepRingInsertGrant{retirement:RetainedCloneGrant{maximum_depth:x,..grant.retirement},..grant}),grant.comparison.maximum_items.checked_sub(1).map(|x|ToolRunStepRingInsertGrant{comparison:BoundedOrdGrant{maximum_items:x,..grant.comparison},..grant}),grant.comparison.maximum_bytes.checked_sub(1).map(|x|ToolRunStepRingInsertGrant{comparison:BoundedOrdGrant{maximum_bytes:x,..grant.comparison},..grant}),grant.maximum_moved_items.checked_sub(1).map(|x|ToolRunStepRingInsertGrant{maximum_moved_items:x,..grant}),grant.maximum_moved_bytes.checked_sub(1).map(|x|ToolRunStepRingInsertGrant{maximum_moved_bytes:x,..grant})].into_iter().flatten(){let(receipt,heap)=measured(||job.advance(denied).unwrap());assert_eq!(receipt,Default::default());assert_eq!(heap,(0,0));assert_eq!(job.next_demands().unwrap(),demand);}
   let(receipt,heap)=measured(||job.advance(grant).unwrap());assert!(receipt.fits(grant));assert_eq!(heap,(receipt.retirement.retained_capacity_bytes,receipt.retirement.released_bytes));born+=heap.0;released+=heap.1;turns+=1;
  }
  if stop.is_none(){assert!(job.complete());let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<ToolRunStepRing>(),maximum_depth:1,..Default::default()};assert!(job.take_output(RetainedCloneGrant{maximum_copy_bytes:0,..grant}).unwrap().is_none());let(output,receipt)=job.take_output(grant).unwrap().unwrap();assert!(receipt.fits(grant));assert_eq!(output.len(),row["expectedCount"].as_u64().unwrap()as usize);assert_eq!(output.oldest().unwrap().sequence,row["expectedFirstSequence"].as_u64().unwrap());assert_eq!(output.newest().unwrap().sequence,row["expectedLastSequence"].as_u64().unwrap());assert_eq!(output.newest().unwrap().repeat,row["expectedLastRepeat"].as_u64().unwrap()as u32);assert_eq!(output.newest().unwrap().args.as_ptr(),if row["name"].as_str().unwrap().starts_with("coalesce"){newest.unwrap()}else{input_pointer});let heap=drain(output);born+=heap.0;released+=heap.1;}else{job.cancel();assert!(job.take_output(Default::default()).unwrap().is_none());}
  let heap=drain(job);born+=heap.0;released+=heap.1;assert_eq!(original+born,released);println!("[DEBUG] original step-ring insertion case={} stop={stop:?} turns={turns} original={original} born={born} physical={released}",row["name"]);
 }}
}

#[test]
fn original_tool_run_progress_metadata_presentation_captures_keep_joint_original_body(){
 use semio_framework_value::{retained_clone::{RetainedCloneGrant,ordered_map::RetainedOrderedMap},retirement::{RetireOwned,controlled::ControlledRetirement,shared::sealed::{SealedShared,SharedIssuer}}};
 use super::{presentation::{ToolRunPresentation,ToolRunPresentationBody},entities::provenance::ToolRunEntityProvenance};
 fn measured<R>(work:impl FnOnce()->R)->(R,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(work);(value,(heap.requested_bytes,heap.released_bytes))}
 fn drain<T:RetireOwned>(value:T)->(usize,usize){let mut owner=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("presentation retains actual issuer-owned fields"));let(mut born,mut released)=(0,0);for _ in 0..100000{if owner.terminal_is_empty(){break;}let copy=owner.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};let(step,heap)=measured(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;}assert!(owner.terminal_is_empty());assert_eq!(measured(||drop(owner)).1,(0,0));(born,released)}
 let law:Value=serde_json::from_str(include_str!("../../📊️progress/🪟️presentation/🧫️fixtures/🔣️.json")).unwrap();let corpus=fixture(TICKS);
 for bytes in law["payloadCases"].as_array().unwrap(){for order in law["orders"].as_array().unwrap(){
  let(mut body,heap)=measured(||{let marks:RetainedOrderedMap<u128,()>=[((1u128<<64)|7,()),((3u128<<64)|9,())].into_iter().collect();let entities=[(7u64,()),(9,())].into_iter().collect();let(provenance,_)=ToolRunEntityProvenance::admit(marks,entities,0,RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:ToolRunEntityProvenance::birth_bytes(),maximum_depth:1,..Default::default()}).unwrap_or_else(|_|panic!("cold fixture original provenance"));let(payload,_)=SealedShared::admit(hex(bytes),SharedIssuer::owned(),RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:SealedShared::<Vec<u8>>::birth_bytes(),maximum_depth:1,..Default::default()}).unwrap_or_else(|_|panic!("cold fixture original payload"));let(tool_id,_)=SealedShared::admit(String::from("draw.command"),SharedIssuer::owned(),RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:SealedShared::<String>::birth_bytes(),maximum_depth:1,..Default::default()}).unwrap_or_else(|_|panic!("cold fixture original tool identifier"));ToolRunPresentationBody{tool_id,progress:progress(&corpus["ticks"][0]["tick"]["progress"]),provenance,payload:Some(payload)}});let original=heap.0-heap.1;let tool_pointer=body.tool_id.as_ptr();let counter_pointer=body.progress.counters.as_ptr();let payload_pointer=body.payload.as_ref().unwrap().get().as_ptr();let provenance_pointer=body.provenance.identity();let quote=ToolRunPresentation::birth_demands();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:quote.copy_bytes,maximum_capacity_bytes:quote.capacity_bytes,maximum_depth:quote.depth,..Default::default()};
  for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:0,..grant},RetainedCloneGrant{maximum_capacity_bytes:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(result,heap)=measured(||ToolRunPresentation::admit(body,denied));assert_eq!(heap,(0,0));let(_,returned)=result.err().unwrap();body=returned;assert_eq!(body.tool_id.as_ptr(),tool_pointer);assert_eq!(body.progress.counters.as_ptr(),counter_pointer);assert_eq!(body.payload.as_ref().unwrap().get().as_ptr(),payload_pointer);}
  let(admitted,heap)=measured(||ToolRunPresentation::admit(body,grant));let(presentation,receipt)=admitted.map_err(|(error,_)|error).unwrap();assert!(receipt.fits(grant));assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));let(mut born,mut released)=(heap.0,heap.1);let identity=presentation.identity();let mut source=Some(presentation);let mut captures=[None,None,None];let capture_grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:ToolRunPresentation::capture_copy_bytes(),maximum_depth:1,..Default::default()};
  for capture in &mut captures{for denied in [RetainedCloneGrant{maximum_items:0,..capture_grant},RetainedCloneGrant{maximum_copy_bytes:0,..capture_grant},RetainedCloneGrant{maximum_depth:0,..capture_grant}]{let(result,heap)=measured(||source.as_ref().unwrap().capture(denied));assert!(result.is_err());assert_eq!(heap,(0,0));}let((alias,receipt),heap)=measured(||source.as_ref().unwrap().capture(capture_grant).unwrap());assert!(receipt.fits(capture_grant));assert_eq!(heap,(0,0));assert_eq!(alias.identity(),identity);*capture=Some(alias);}
  let steps=if order.as_str().unwrap()=="sourceFirst"{[3,0,1,2]}else{[0,1,2,3]};for index in steps{for live in source.iter().chain(captures.iter().flatten()){assert_eq!(live.identity(),identity);assert_eq!(live.body().tool_id.as_ptr(),tool_pointer);assert_eq!(live.body().progress.counters.as_ptr(),counter_pointer);assert_eq!(live.body().payload.as_ref().unwrap().get().as_ptr(),payload_pointer);assert_eq!(live.body().provenance.identity(),provenance_pointer);}let value=if index==3{source.take().unwrap()}else{captures[index].take().unwrap()};let heap=drain(value);born+=heap.0;released+=heap.1;}
  assert_eq!(original+born,released);println!("[DEBUG] original presentation order={order} payload={bytes} original={original} born={born} physical={released}");
 }}
}

#[test]
fn original_tool_run_progress_metadata_presentation_update_is_atomic_and_physically_owned(){
 use semio_framework_value::{retained_clone::{RetainedCloneGrant,ordered_map::{RetainedOrderedMap,BoundedOrdGrant}},retirement::{RetireOwned,controlled::ControlledRetirement,shared::sealed::{SealedShared,SharedIssuer}}};
 use super::{presentation::{ToolRunPresentation,ToolRunPresentationBody,update::ToolRunPresentationUpdate},entities::{ToolRunEntityEditGrant,provenance::ToolRunEntityProvenance}};
 fn measured<R>(work:impl FnOnce()->R)->(R,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(work);(value,(heap.requested_bytes,heap.released_bytes))}
 fn drain<T:RetireOwned>(value:T)->(usize,usize){let mut owner=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("presentation update retains actual original children"));let(mut born,mut released)=(0,0);for _ in 0..100000{if owner.terminal_is_empty(){break;}let copy=owner.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};let(step,heap)=measured(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;}assert!(owner.terminal_is_empty());assert_eq!(measured(||drop(owner)).1,(0,0));(born,released)}
 let law:Value=serde_json::from_str(include_str!("../../📊️progress/🪟️presentation/🧬️update/🧫️fixtures/🔣️.json")).unwrap();let corpus=fixture(TICKS);let ring_law:Value=serde_json::from_str(include_str!("../../📊️progress/💍️steps/➕️insert/🧫️fixtures/🔣️.json")).unwrap();
 for with_progress in [false,true]{for payload in law["payloadCases"].as_array().unwrap(){for stop in std::iter::once(None).chain(law["interruptAfter"].as_array().unwrap().iter().map(|x|Some(x.as_u64().unwrap()as usize))){for source_first in [false,true]{
  let((source,input),heap)=measured(||{let marks:RetainedOrderedMap<u128,()>=law["originalMarks"].as_array().unwrap().iter().map(|mark|(((mark["end"].as_u64().unwrap()as u128)<<64)|mark["entity"].as_str().unwrap().parse::<u64>().unwrap()as u128,())).collect();let entities=[(7u64,()),(9,())].into_iter().collect();let(provenance,_)=ToolRunEntityProvenance::admit(marks,entities,0,RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:ToolRunEntityProvenance::birth_bytes(),maximum_depth:1,..Default::default()}).unwrap_or_else(|_|panic!("cold original provenance"));let(tool_id,_)=SealedShared::admit(String::from("draw.command"),SharedIssuer::owned(),RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:SealedShared::<String>::birth_bytes(),maximum_depth:1,..Default::default()}).unwrap_or_else(|_|panic!("cold original identifier"));let(bytes,_)=SealedShared::admit(vec![4u8,5],SharedIssuer::owned(),RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:SealedShared::<Vec<u8>>::birth_bytes(),maximum_depth:1,..Default::default()}).unwrap_or_else(|_|panic!("cold original payload"));let progress=progress(&corpus["ticks"][0]["tick"]["progress"]);let identity=progress.identity;let sequence=progress.sequence+1;let quote=ToolRunPresentation::birth_demands();let(source,_)=ToolRunPresentation::admit(ToolRunPresentationBody{tool_id,progress,provenance,payload:Some(bytes)},RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:quote.copy_bytes,maximum_capacity_bytes:quote.capacity_bytes,maximum_depth:quote.depth,..Default::default()}).unwrap_or_else(|_|panic!("cold original presentation"));let input=ToolRunTick{identity,sequence,progress:if with_progress{let mut update=self::progress(&corpus["ticks"][0]["tick"]["progress"]);update.stage=17;update.completed=99;update.total=Some(3);update.counters=vec![ToolRunCounter{counter:2,value:5}];Some(update)}else{None},steps:vec![step(&ring_law["cases"][0]["input"])],trace:Vec::new(),append_ops:Vec::new(),append_entities:law["append"].as_array().unwrap().iter().map(|v|v.as_str().unwrap().parse().unwrap()).collect(),retract_to:Some(law["retractTo"].as_u64().unwrap()as u32),payload:(!payload.is_null()).then(||hex(payload))};(source,input)});let original=heap.0-heap.1;let original_pointer=source.identity();let original_counters=source.body().progress.counters.as_ptr();let original_payload=source.body().payload.as_ref().unwrap().get().as_ptr();let input_payload=input.payload.as_ref().map(|bytes|bytes.as_ptr());let input_counters=input.progress.as_ref().map(|progress|progress.counters.as_ptr());let sequence=input.sequence;let alias=source.capture(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:ToolRunPresentation::capture_copy_bytes(),maximum_depth:1,..Default::default()}).unwrap().0;let(mut job,heap)=measured(||ToolRunPresentationUpdate::new(alias,input,law["end"].as_u64().unwrap()as u32,ToolRunState::Running,0.0,0));assert_eq!(heap,(0,0));let(mut born,mut released,mut turns)=(0,0,0);
  while !job.complete()&&turns<stop.unwrap_or(100000){let quote=job.next_demands(usize::MAX).unwrap();let grant=ToolRunEntityEditGrant{retirement:RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:quote.copy_bytes,maximum_capacity_bytes:quote.capacity_bytes,maximum_release_bytes:quote.release_bytes,maximum_depth:quote.depth},comparison:BoundedOrdGrant{maximum_items:1,maximum_bytes:32},maximum_moved_items:17,maximum_moved_bytes:17*size_of::<(u128,())>(),maximum_capacity_bytes:job.edit_capacity_bound().unwrap()};for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant.retirement}),grant.retirement.maximum_copy_bytes.checked_sub(1).map(|v|RetainedCloneGrant{maximum_copy_bytes:v,..grant.retirement}),grant.retirement.maximum_capacity_bytes.checked_sub(1).map(|v|RetainedCloneGrant{maximum_capacity_bytes:v,..grant.retirement}),grant.retirement.maximum_release_bytes.checked_sub(1).map(|v|RetainedCloneGrant{maximum_release_bytes:v,..grant.retirement}),grant.retirement.maximum_depth.checked_sub(1).map(|v|RetainedCloneGrant{maximum_depth:v,..grant.retirement})].into_iter().flatten(){let(receipt,heap)=measured(||job.advance(ToolRunEntityEditGrant{retirement:denied,..grant}).unwrap());assert_eq!(receipt,Default::default());assert_eq!(heap,(0,0));assert_eq!(job.next_demands(usize::MAX).unwrap(),quote);}let(receipt,heap)=measured(||job.advance(grant).unwrap());assert!(receipt.fits(grant));assert_eq!(heap,(receipt.retirement.retained_capacity_bytes+receipt.retained_capacity_bytes,receipt.retirement.released_bytes));born+=heap.0;released+=heap.1;assert_eq!(source.identity(),original_pointer);assert_eq!(source.body().progress.counters.as_ptr(),original_counters);turns+=1;}
  let output=if stop.is_none(){assert!(job.complete());let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<ToolRunPresentation>(),maximum_depth:1,..Default::default()};assert!(job.take_output(Default::default()).unwrap().is_none());let(output,receipt)=job.take_output(grant).unwrap().unwrap();assert!(receipt.fits(grant));assert_ne!(output.identity(),source.identity());assert_eq!(output.body().tool_id.as_ptr(),source.body().tool_id.as_ptr());assert_eq!(output.body().progress.sequence,sequence);if with_progress{assert_eq!(output.body().progress.stage,17);assert_eq!(output.body().progress.completed,3);assert_eq!(output.body().progress.counters.as_ptr(),input_counters.unwrap());}let entities:Vec<_>=output.body().provenance.entities().iter().map(|(key,_)|key.to_string()).collect();assert_eq!(entities,law["expectedEntities"].as_array().unwrap().iter().map(|v|v.as_str().unwrap().to_owned()).collect::<Vec<_>>());assert_eq!(output.body().payload.as_ref().unwrap().get().as_ptr(),input_payload.unwrap_or(original_payload));Some(output)}else{job.cancel();assert!(job.take_output(Default::default()).unwrap().is_none());None};let heap=drain(job);born+=heap.0;released+=heap.1;let mut source=Some(source);let mut output=output;for first in [source_first,!source_first]{let value=if first{source.take()}else{output.take()};if let Some(value)=value{let heap=drain(value);born+=heap.0;released+=heap.1;}}assert_eq!(original+born,released);println!("[DEBUG] atomic original presentation withProgress={with_progress} payload={payload} stop={stop:?} sourceFirst={source_first} turns={turns} original={original} born={born} physical={released}");
 }}}}
}

#[test]
fn original_tool_run_progress_metadata_presentation_update_refuses_foreign_original_without_drop(){
 use semio_framework_value::{retained_clone::RetainedCloneGrant,retirement::{RetireOwned,controlled::ControlledRetirement,shared::sealed::{SealedShared,SharedIssuer}}};
 use super::{presentation::{ToolRunPresentation,ToolRunPresentationBody,update::ToolRunPresentationUpdate},entities::{ToolRunEntityEditGrant,provenance::ToolRunEntityProvenance}};
 fn measured<R>(work:impl FnOnce()->R)->(R,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(work);(value,(heap.requested_bytes,heap.released_bytes))}
 fn drain<T:RetireOwned>(value:T)->(usize,usize){let mut owner=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("refused presentation owns its actual original"));let(mut born,mut released)=(0,0);for _ in 0..100000{if owner.terminal_is_empty(){break;}let copy=owner.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};let(step,heap)=measured(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;}assert!(owner.terminal_is_empty());assert_eq!(measured(||drop(owner)).1,(0,0));(born,released)}
 let law:Value=serde_json::from_str(include_str!("../../📊️progress/🪟️presentation/🧬️update/🧫️fixtures/🔣️.json")).unwrap();assert_eq!(law["foreignRun"],true);let corpus=fixture(TICKS);
 for source_first in [false,true]{let((source,input),heap)=measured(||{let(provenance,_)=ToolRunEntityProvenance::admit(Default::default(),Default::default(),0,RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:ToolRunEntityProvenance::birth_bytes(),maximum_depth:1,..Default::default()}).unwrap_or_else(|_|panic!("cold original provenance"));let(tool_id,_)=SealedShared::admit(String::from("draw.command"),SharedIssuer::owned(),RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:SealedShared::<String>::birth_bytes(),maximum_depth:1,..Default::default()}).unwrap_or_else(|_|panic!("cold original identifier"));let progress=progress(&corpus["ticks"][0]["tick"]["progress"]);let mut identity=progress.identity;identity.id.run+=1;let quote=ToolRunPresentation::birth_demands();let(source,_)=ToolRunPresentation::admit(ToolRunPresentationBody{tool_id,progress,provenance,payload:None},RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:quote.copy_bytes,maximum_capacity_bytes:quote.capacity_bytes,maximum_depth:1,..Default::default()}).unwrap_or_else(|_|panic!("cold original presentation"));(source,ToolRunTick{identity,sequence:1,progress:None,steps:Vec::new(),trace:Vec::new(),append_ops:vec![vec![3,4,5]],append_entities:vec![7],retract_to:None,payload:Some(vec![8,9])})});let original=heap.0-heap.1;let pointer=source.identity();let counters=source.body().progress.counters.as_ptr();let capture=source.capture(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:ToolRunPresentation::capture_copy_bytes(),maximum_depth:1,..Default::default()}).unwrap().0;let mut job=ToolRunPresentationUpdate::new(capture,input,1,ToolRunState::Running,0.0,0);let(fault,heap)=measured(||job.advance(ToolRunEntityEditGrant{retirement:RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:ToolRunPresentation::capture_copy_bytes(),maximum_depth:1,..Default::default()},..Default::default()}));assert_eq!(fault.err().unwrap().kind,semio_framework_value::ValueRefusalKind::InvalidValue);assert_eq!(heap,(0,0));assert_eq!(source.identity(),pointer);assert_eq!(source.body().progress.counters.as_ptr(),counters);assert!(!job.complete());assert!(job.take_output(Default::default()).unwrap().is_none());job.cancel();let(mut born,mut released)=(0,0);let mut source=Some(source);let mut job=Some(job);for first in [source_first,!source_first]{let heap=if first{drain(source.take().unwrap())}else{drain(job.take().unwrap())};born+=heap.0;released+=heap.1;}assert_eq!(original+born,released);println!("[DEBUG] refused original presentation sourceFirst={source_first} original={original} born={born} physical={released}");}
}

#[test]
fn original_tool_run_progress_metadata_presentation_seed_is_admitted_without_identifier_copy(){
 use super::presentation::seed::ToolRunPresentationSeed;
 use semio_framework_value::{retained_clone::RetainedCloneGrant,retirement::{RetireOwned,controlled::ControlledRetirement}};
 fn measured<R>(work:impl FnOnce()->R)->(R,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(work);(value,(heap.requested_bytes,heap.released_bytes))}
 fn drain<T:RetireOwned>(value:T)->(usize,usize){let mut close=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("seed original fields have concrete authority"));let(mut born,mut released)=(0,0);for _ in 0..100000{if close.terminal_is_empty(){break;}let copy=close.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:close.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:close.next_release_byte_demand().unwrap(),maximum_depth:close.next_depth_demand().unwrap()};let(step,heap)=measured(||close.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;}assert!(close.terminal_is_empty());assert_eq!(measured(||drop(close)).1,(0,0));(born,released)}
 let law:Value=serde_json::from_str(include_str!("../../📊️progress/🪟️presentation/🌱️seed/🧫️fixtures/🔣️.json")).unwrap();
 for row in law["toolIds"].as_array().unwrap(){for stop in std::iter::once(None).chain(law["interruptAfter"].as_array().unwrap().iter().map(|value|Some(value.as_u64().unwrap()as usize))){let(id,heap)=measured(||row.as_str().unwrap().to_owned());let original=heap.0-heap.1;let pointer=id.as_ptr();let identity=identity(&fixture(TICKS)["ticks"][0]["tick"]["identity"]);let(mut seed,heap)=measured(||ToolRunPresentationSeed::new(id,identity,ToolRunState::Starting));assert_eq!(heap,(0,0));let(mut born,mut released,mut turns)=(0,0,0);while !seed.complete()&&turns<stop.unwrap_or(100){let quote=seed.next_demands().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:quote.copy_bytes,maximum_capacity_bytes:quote.capacity_bytes,maximum_release_bytes:quote.release_bytes,maximum_depth:quote.depth};for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(receipt,heap)=measured(||seed.advance(denied).unwrap());assert_eq!(heap,(0,0));assert_eq!(receipt,Default::default());assert_eq!(seed.original_identifier().as_ptr(),pointer);assert_eq!(seed.next_demands().unwrap(),quote);}if quote.copy_bytes>0{let(receipt,heap)=measured(||seed.advance(RetainedCloneGrant{maximum_copy_bytes:quote.copy_bytes-1,..grant}).unwrap());assert_eq!(receipt,Default::default());assert_eq!(heap,(0,0));}if quote.capacity_bytes>0{let(receipt,heap)=measured(||seed.advance(RetainedCloneGrant{maximum_capacity_bytes:quote.capacity_bytes-1,..grant}).unwrap());assert_eq!(receipt,Default::default());assert_eq!(heap,(0,0));}let(receipt,heap)=measured(||seed.advance(grant).unwrap());assert!(receipt.fits(grant));assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.0;released+=heap.1;assert_eq!(seed.original_identifier().as_ptr(),pointer);turns+=1;}
  if stop.is_none(){let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:std::mem::size_of::<super::presentation::ToolRunPresentation>(),maximum_depth:1,..Default::default()};assert!(seed.take_output(RetainedCloneGrant{maximum_items:0,..grant}).unwrap().is_none());let((output,receipt),heap)=measured(||seed.take_output(grant).unwrap().unwrap());assert!(receipt.fits(grant));assert_eq!(heap,(0,0));let body=output.body();assert_eq!(body.tool_id.get().as_ptr(),pointer);assert_eq!(body.tool_id.get(),row.as_str().unwrap());assert_eq!(body.progress.identity,identity);assert_eq!(body.progress.state,ToolRunState::Starting);assert_eq!(body.progress.stage,0);assert_eq!(body.progress.completed,0);assert!(body.progress.total.is_none());assert!(body.progress.counters.is_empty());assert!(body.progress.steps.is_empty());assert!(body.provenance.marks().is_empty());assert!(body.provenance.entities().is_empty());assert!(body.payload.is_none());let heap=drain(output);born+=heap.0;released+=heap.1;}else{assert_eq!(measured(||seed.cancel()).1,(0,0));assert!(seed.take_output(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:usize::MAX,maximum_depth:1,..Default::default()}).unwrap().is_none());}
  let heap=drain(seed);born+=heap.0;released+=heap.1;assert_eq!(original+born,released);println!("[DEBUG] original presentation seed id={row} stop={stop:?} original={original} born={born} physical={released}");
 }}
}
#[test]
fn original_tool_run_progress_metadata_presentation_slot_publishes_only_complete_original_candidates(){
 use super::presentation::{ToolRunCapturedView,slot::ToolRunPresentationSlot};
 use super::entities::ToolRunEntityEditGrant;
 use semio_framework_value::{retained_clone::{RetainedCloneGrant,ordered_map::BoundedOrdGrant},retirement::{RetireOwned,controlled::ControlledRetirement}};
 fn measured<R>(work:impl FnOnce()->R)->(R,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(work);(value,(heap.requested_bytes,heap.released_bytes))}
 fn drain<T:RetireOwned>(value:T)->(usize,usize){let mut close=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("slot owns actual seed, update, roots and completed children"));let(mut born,mut released)=(0,0);for _ in 0..100000{if close.terminal_is_empty(){break;}let copy=close.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:close.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:close.next_release_byte_demand().unwrap(),maximum_depth:close.next_depth_demand().unwrap()};let(step,heap)=measured(||close.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;}assert!(close.terminal_is_empty());assert_eq!(measured(||drop(close)).1,(0,0));(born,released)}
 fn step(slot:&mut ToolRunPresentationSlot)->(usize,usize){let quote=slot.next_demands(usize::MAX).unwrap();let retirement=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:quote.copy_bytes,maximum_capacity_bytes:quote.capacity_bytes,maximum_release_bytes:quote.release_bytes,maximum_depth:quote.depth};let grant=ToolRunEntityEditGrant{retirement,comparison:BoundedOrdGrant{maximum_items:17,maximum_bytes:4096},maximum_moved_items:17,maximum_moved_bytes:4096,maximum_capacity_bytes:slot.edit_capacity_bound().unwrap()};let(receipt,heap)=measured(||slot.advance(grant).unwrap());assert!(receipt.fits(grant));assert_eq!(heap,(receipt.retirement.retained_capacity_bytes+receipt.retained_capacity_bytes,receipt.retirement.released_bytes),"slot funded quote={quote:?} receipt={receipt:?}");heap}
 let law:Value=serde_json::from_str(include_str!("../../📊️progress/🪟️presentation/🗃️slot/🧫️fixtures/🔣️.json")).unwrap();let corpus=fixture(TICKS);
 for stop in std::iter::once(None).chain(law["interruptAfter"].as_array().unwrap().iter().map(|v|Some(v.as_u64().unwrap()as usize))){for order in law["orders"].as_array().unwrap(){let(id,heap)=measured(||law["toolId"].as_str().unwrap().to_owned());let pointer=id.as_ptr();let mut original=heap.0-heap.1;let incoming_identity=identity(&corpus["ticks"][0]["tick"]["identity"]);let(mut slot,heap)=measured(||ToolRunPresentationSlot::new(id,incoming_identity,ToolRunState::Starting));assert_eq!(heap,(0,0));let(mut born,mut released)=(0,0);for _ in 0..10000{if slot.ready(){break;}let heap=step(&mut slot);born+=heap.0;released+=heap.1;}assert!(slot.ready());assert_eq!(slot.body().unwrap().tool_id.get().as_ptr(),pointer);let capture_grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:ToolRunCapturedView::capture_copy_bytes(),maximum_depth:1,..Default::default()};let((capture,receipt),heap)=measured(||slot.capture_view(incoming_identity,ToolRunState::Running,capture_grant).unwrap().unwrap());assert_eq!(heap,(0,0));assert!(receipt.fits(capture_grant));let source_pointer=capture.source_identity();assert_eq!(measured(||capture.borrowed_view()).1,(0,0));let view=capture.borrowed_view();assert_eq!(view.identity,incoming_identity);assert_eq!(view.state,ToolRunState::Running);assert_eq!(view.tool_id.as_ptr(),pointer);assert_eq!(view.progress as *const _,&capture.body().progress as *const _);assert!(view.provisional_entities.is_empty());assert!(view.payload.is_none());assert_eq!(view.progress.state,ToolRunState::Starting);let(tick,heap)=measured(||{let mut input=ToolRunTick{identity:incoming_identity,sequence:1,retract_to:None,append_ops:Vec::new(),append_entities:vec![7,11],steps:Vec::new(),progress:None,trace:Vec::new(),payload:Some(vec![1,2,3])};input.progress=Some(progress(&corpus["ticks"][0]["tick"]["progress"]));input});original+=heap.0-heap.1;let incoming_counter=tick.progress.as_ref().unwrap().counters.as_ptr();let incoming_payload=tick.payload.as_ref().unwrap().as_ptr();let quote=ToolRunPresentationSlot::tick_admission_demands();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:quote.copy_bytes,maximum_depth:quote.depth,..Default::default()};let(((),receipt),heap)=measured(||slot.begin_tick(tick,1,ToolRunState::Running,0.0,0,grant).unwrap_or_else(|_|panic!("actual original tick admission")));assert_eq!(heap,(0,0));assert!(receipt.fits(grant));let mut turns=0;while !slot.ready()&&turns<stop.unwrap_or(10000){let previous=slot.body().unwrap().progress.sequence;let quote=slot.next_demands(usize::MAX).unwrap();let denied=ToolRunEntityEditGrant{retirement:RetainedCloneGrant{maximum_items:0,..Default::default()},comparison:BoundedOrdGrant{maximum_items:17,maximum_bytes:4096},maximum_moved_items:17,maximum_moved_bytes:4096,maximum_capacity_bytes:slot.edit_capacity_bound().unwrap()};assert_eq!(measured(||slot.advance(denied).unwrap()).1,(0,0));assert_eq!(slot.next_demands(usize::MAX).unwrap(),quote);assert_eq!(slot.body().unwrap().progress.sequence,previous);let heap=step(&mut slot);born+=heap.0;released+=heap.1;assert_eq!(capture.source_identity(),source_pointer);assert_eq!(capture.body().progress.sequence,0);assert!(capture.body().progress.counters.is_empty());turns+=1;}
  if stop.is_none(){assert!(slot.ready());let body=slot.body().unwrap();assert_eq!(body.progress.sequence,1);assert_eq!(body.progress.counters.as_ptr(),incoming_counter);assert_eq!(body.payload.as_ref().unwrap().get().as_ptr(),incoming_payload);let entities:Vec<String>=(0..body.provenance.entities().len()).map(|index|body.provenance.entities().get_index(index).unwrap().0.to_string()).collect();assert_eq!(entities,law["expectedEntities"].as_array().unwrap().iter().map(|v|v.as_str().unwrap().to_owned()).collect::<Vec<_>>());}
  if stop.is_some(){let before=slot.body().unwrap().progress.sequence;assert_eq!(measured(||slot.cancel_pending()).1,(0,0));for _ in 0..100000{if slot.ready(){break;}let heap=step(&mut slot);born+=heap.0;released+=heap.1;assert_eq!(slot.body().unwrap().progress.sequence,before);}assert!(slot.ready());let(tick,heap)=measured(||ToolRunTick{identity:incoming_identity,sequence:2,retract_to:Some(0),append_ops:Vec::new(),append_entities:vec![law["restartEntity"].as_str().unwrap().parse().unwrap()],steps:Vec::new(),progress:None,trace:Vec::new(),payload:None});original+=heap.0-heap.1;let quote=ToolRunPresentationSlot::tick_admission_demands();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:quote.copy_bytes,maximum_depth:quote.depth,..Default::default()};assert_eq!(measured(||slot.begin_tick(tick,2,ToolRunState::Running,0.0,0,grant).unwrap_or_else(|_|panic!("same original slot admits replacement only after pending owners close"))).1,(0,0));for _ in 0..100000{if slot.ready(){break;}let heap=step(&mut slot);born+=heap.0;released+=heap.1;}assert!(slot.ready());let body=slot.body().unwrap();assert_eq!(body.progress.sequence,2);assert_eq!(body.provenance.entities().len(),1);assert_eq!(*body.provenance.entities().get_index(0).unwrap().0,13);assert_eq!(capture.body().progress.sequence,0);}
  assert_eq!(measured(||slot.cancel()).1,(0,0));let heaps=if order.as_str().unwrap()=="slotFirst"{[drain(slot),drain(capture)]}else{let first=drain(capture);[first,drain(slot)]};for heap in heaps{born+=heap.0;released+=heap.1;}assert_eq!(original+born,released);println!("[DEBUG] original presentation slot stop={stop:?} order={order} turns={turns} original={original} born={born} physical={released}");
 }}
}
#[test]
fn original_tool_run_progress_metadata_tick_source_keeps_actual_operation_and_trace_owners(){
 use super::tick_source::ToolRunTickSource;
 use semio_framework_value::{retained_clone::{RetainedCloneGrant,RetainedCloneProgress},retirement::{RetireOwned,controlled::ControlledRetirement}};
 fn measured<R>(work:impl FnOnce()->R)->(R,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(work);(value,(heap.requested_bytes,heap.released_bytes))}
 fn drain<T:RetireOwned>(value:T)->(usize,usize){let mut close=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("actual tick source and borrowed children have typed original custody"));let(mut born,mut released)=(0,0);for _ in 0..100000{if close.terminal_is_empty(){break;}let copy=close.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:close.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:close.next_release_byte_demand().unwrap(),maximum_depth:close.next_depth_demand().unwrap()};let(step,heap)=measured(||close.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;}assert!(close.terminal_is_empty());assert_eq!(measured(||drop(close)).1,(0,0));(born,released)}
 let law:Value=serde_json::from_str(include_str!("../../📽️tick/🔐️source/🧫️fixtures/🔣️.json")).unwrap();let corpus=fixture(TICKS);
 for order in law["orders"].as_array().unwrap(){let(mut input,heap)=measured(||ToolRunTick{identity:identity(&corpus["ticks"][0]["tick"]["identity"]),sequence:1,retract_to:None,append_ops:Vec::new(),append_entities:Vec::new(),steps:Vec::new(),progress:None,trace:Vec::new(),payload:None});let mut original=heap.0-heap.1;let(operations,heap)=measured(||law["operations"].as_array().unwrap().iter().map(|v|hex(v)).collect::<Vec<_>>());original+=heap.0-heap.1;input.append_ops=operations;let operation_pointer=input.append_ops[1].as_ptr();let(mut input,heap)=measured(||{input.trace=vec![ToolRunTracePage{identity:input.identity,page:0,ops:vec![ToolRunTraceOp::Clear]}];input});original+=heap.0-heap.1;let trace_pointer=input.trace[0].ops.as_ptr();let quote=ToolRunTickSource::birth_demands();let funded=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:quote.copy_bytes,maximum_capacity_bytes:quote.capacity_bytes,maximum_depth:quote.depth,..Default::default()};for axis in [0,1,2,4]{let denied=match axis{0=>RetainedCloneGrant{maximum_items:0,..funded},1=>RetainedCloneGrant{maximum_copy_bytes:0,..funded},2=>RetainedCloneGrant{maximum_capacity_bytes:0,..funded},_=>RetainedCloneGrant{maximum_depth:0,..funded}};let(result,heap)=measured(||ToolRunTickSource::admit(input,denied));assert_eq!(heap,(0,0));let(_,returned)=result.err().unwrap();assert_eq!(returned.append_ops[1].as_ptr(),operation_pointer);input=returned;}let((source,receipt),heap)=measured(||ToolRunTickSource::admit(input,funded).unwrap_or_else(|_|panic!("original source admission is funded")));assert!(receipt.fits(funded));assert_eq!(heap,(receipt.retained_capacity_bytes,0));let(mut born,mut released)=heap;
 let operation_grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:ToolRunTickSource::operation_capture_copy_bytes(),maximum_depth:1,..Default::default()};let trace_grant=RetainedCloneGrant{maximum_copy_bytes:ToolRunTickSource::trace_capture_copy_bytes(),..operation_grant};let((operation,receipt),heap)=measured(||source.operation(1,operation_grant).unwrap());assert_eq!(heap,(0,0));assert!(receipt.fits(operation_grant));let((trace,receipt),heap)=measured(||source.trace(0,trace_grant).unwrap());assert_eq!(heap,(0,0));assert!(receipt.fits(trace_grant));assert_eq!(operation.bytes().as_ptr(),operation_pointer);assert_eq!(operation.bytes(),hex(&law["operations"][1]));assert_eq!(trace.page().ops.as_ptr(),trace_pointer);assert_eq!(trace.page().ops,[ToolRunTraceOp::Clear]);let take=ToolRunTickSource::take_demands();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:take.copy_bytes,maximum_release_bytes:take.release_bytes,maximum_depth:take.depth,..Default::default()};let(result,heap)=measured(||source.try_into_tick(grant));assert_eq!(heap,(0,0));let(_,mut source)=result.err().unwrap();assert_eq!(source.borrow().append_ops[1].as_ptr(),operation_pointer);
 for heap in if order.as_str().unwrap()=="operationFirst"{[drain(operation),drain(trace)]}else{let first=drain(trace);[first,drain(operation)]}{born+=heap.0;released+=heap.1;}for axis in [0,1,3,4]{let denied=match axis{0=>RetainedCloneGrant{maximum_items:0,..grant},1=>RetainedCloneGrant{maximum_copy_bytes:0,..grant},3=>RetainedCloneGrant{maximum_release_bytes:0,..grant},_=>RetainedCloneGrant{maximum_depth:0,..grant}};let(result,heap)=measured(||source.try_into_tick(denied));assert_eq!(heap,(0,0));let(_,returned)=result.err().unwrap();source=returned;}let((returned,receipt),heap)=measured(||source.try_into_tick(grant).unwrap_or_else(|_|panic!("same original tick returns after genuine child close")));assert!(receipt.fits(grant));assert_eq!(heap,(0,receipt.released_bytes));released+=heap.1;assert_eq!(returned.append_ops[1].as_ptr(),operation_pointer);assert_eq!(returned.trace[0].ops.as_ptr(),trace_pointer);let heap=drain(returned);born+=heap.0;released+=heap.1;assert_eq!(original+born,released);println!("[DEBUG] original Tick operation/trace source order={order} original={original} born={born} physical={released}");let _:RetainedCloneProgress=receipt;
 }
}
