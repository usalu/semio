//! 🔬️ Rust reducer against the language-agnostic lifecycle-law fixture: every matrix row, every
//! stale generation, every case, every scenario, the invariants and the labels.

use super::*;
use protocol::value::{DslValue, FromValue, ToValue};
use semio_framework_diagnostic::FaultCode;
use protocol::MutationMessage;
use protocol::MutationReplayOutcome;
use semio_framework_diagnostic::Severity;
use serde_json::Value;

const LAW: &str = include_str!("../../🧫️fixtures/🧫️lifecycle-law/🔣️.json");

fn law() -> Value {
    serde_json::from_str(LAW).expect("fixture parses")
}

fn text(value: &Value) -> &str {
    value.as_str().expect("string")
}

fn u32_of(value: &Value) -> u32 {
    u32::try_from(value.as_u64().expect("u32")).expect("u32 range")
}

fn bytes(value: &Value) -> Vec<u8> {
    let hex = text(value);
    (0..hex.len()).step_by(2).map(|index| u8::from_str_radix(&hex[index..index + 2], 16).expect("hex")).collect()
}

fn optional<T>(value: &Value, parse: impl FnOnce(&Value) -> T) -> Option<T> {
    (!value.is_null()).then(|| parse(value))
}

fn mutation(value: &Value) -> MutationId {
    MutationId(text(value).to_string())
}

fn replacement(value: &Value) -> InputReplacement {
    match text(&value["kind"]) {
        "withdrawn" => InputReplacement::Withdrawn,
        _ => InputReplacement::Input { schema: text(&value["schema"]).to_string(), payload: value["payload"].as_array().expect("payload").iter().map(|byte| u8::try_from(byte.as_u64().expect("byte")).expect("u8")).collect() },
    }
}

fn target(value: &Value) -> TimeTravelTarget {
    TimeTravelTarget { mutation: mutation(&value["mutation"]), position: u32_of(&value["position"]) }
}

fn base(value: &Value) -> TimeTravelBase {
    TimeTravelBase { store_generation: value["storeGeneration"].as_u64().expect("u64"), content_revision: bytes(&value["contentRevision"]).try_into().expect("32 bytes") }
}

fn severity(value: &Value) -> Severity {
    match text(value) {
        "info" => Severity::Info,
        "warning" => Severity::Warning,
        "error" => Severity::Error,
        "fatal" => Severity::Fatal,
        other => panic!("unknown severity {other}"),
    }
}

fn message(value: &Value) -> MutationMessage {
    MutationMessage {
        level: severity(&value["level"]),
        code: FaultCode::new(text(&value["code"])),
        message: text(&value["message"]).to_string(),
        target: value.get("target").map_or_else(Vec::new, |target| target.as_array().expect("target").iter().map(|segment| text(segment).to_string()).collect()),
        op_index: value.get("opIndex").map(u32_of),
    }
}

fn report(value: &Value) -> ReplayReport {
    ReplayReport {
        from_position: u32_of(&value["fromPosition"]),
        outcomes: value["outcomes"]
            .as_array()
            .expect("outcomes")
            .iter()
            .map(|outcome| MutationReplayOutcome {
                mutation_id: mutation(&outcome["mutationId"]),
                edit_id: text(&outcome["editId"]).to_string(),
                op_index: u32_of(&outcome["opIndex"]),
                worst: optional(&outcome["worst"], severity),
                messages: outcome["messages"].as_array().expect("messages").iter().map(message).collect(),
                superseded: outcome["superseded"].as_bool().expect("superseded"),
                withdrawn: outcome["withdrawn"].as_bool().expect("withdrawn"),
            })
            .collect(),
        worst: optional(&value["worst"], severity),
    }
}

fn stage(value: &Value) -> TimeTravelStage {
    TimeTravelStage::parse(text(value)).expect("stage")
}

fn session(value: &Value) -> TimeTravelSession {
    TimeTravelSession {
        id: value["id"].as_u64().expect("id"),
        generation: u32_of(&value["generation"]),
        base: base(&value["base"]),
        stage: stage(&value["stage"]),
        accepted: value["accepted"].as_array().expect("accepted").iter().map(|draft| TimeTravelDraft { target: target(&draft["target"]), replacement: replacement(&draft["replacement"]) }).collect(),
        pending: optional(&value["pending"], |pending| TimeTravelPending { target: target(&pending["target"]), original: replacement(&pending["original"]), replacement: replacement(&pending["replacement"]), return_stage: stage(&pending["returnStage"]) }),
        report: optional(&value["report"], report),
        progress: optional(&value["progress"], |progress| TimeTravelProgress { done: u32_of(&progress["done"]), total: u32_of(&progress["total"]) }),
        fault: optional(&value["fault"], |fault| text(fault).to_string()),
    }
}

fn event(value: &Value) -> TimeTravelEvent {
    let generation = || u32_of(&value["generation"]);
    match text(&value["type"]) {
        "begin" => TimeTravelEvent::Begin { target: target(&value["target"]), original: replacement(&value["original"]) },
        "draft" => TimeTravelEvent::Draft { generation: generation(), replacement: replacement(&value["replacement"]) },
        "withdraw" => TimeTravelEvent::Withdraw { generation: generation() },
        "accept" => TimeTravelEvent::Accept { generation: generation() },
        "discard" => TimeTravelEvent::Discard { generation: generation() },
        "replayProgressed" => TimeTravelEvent::ReplayProgressed { generation: generation(), done: u32_of(&value["done"]), total: u32_of(&value["total"]) },
        "replayCompleted" => TimeTravelEvent::ReplayCompleted { generation: generation(), report: report(&value["report"]) },
        "replayCancelled" => TimeTravelEvent::ReplayCancelled { generation: generation() },
        "replayFaulted" => TimeTravelEvent::ReplayFaulted { generation: generation(), code: text(&value["code"]).to_string() },
        "rerun" => TimeTravelEvent::Rerun { generation: generation() },
        "requestFinalize" => TimeTravelEvent::RequestFinalize { generation: generation() },
        "choose" => TimeTravelEvent::Choose {
            generation: generation(),
            choice: match text(&value["choice"]["kind"]) {
                "overwrite" => TimeTravelChoice::Overwrite,
                _ => TimeTravelChoice::Alternative { name: text(&value["choice"]["name"]).to_string() },
            },
        },
        "back" => TimeTravelEvent::Back { generation: generation() },
        "finalized" => TimeTravelEvent::Finalized { generation: generation() },
        "finalizeFaulted" => TimeTravelEvent::FinalizeFaulted { generation: generation(), code: text(&value["code"]).to_string() },
        "baseMoved" => TimeTravelEvent::BaseMoved { base: base(&value["base"]), positions: value["positions"].as_array().expect("positions").iter().map(target).collect() },
        "exit" => TimeTravelEvent::Exit,
        other => panic!("unknown event {other}"),
    }
}

fn inputs(value: &Value) -> Vec<SupersededInput> {
    value.as_array().expect("inputs").iter().map(|input| SupersededInput { target: mutation(&input["target"]), replacement: replacement(&input["replacement"]) }).collect()
}

fn effect(value: &Value) -> TimeTravelEffect {
    match text(&value["type"]) {
        "showPreview" => TimeTravelEffect::ShowPreview { target: mutation(&value["target"]), replacement: replacement(&value["replacement"]) },
        "startReplay" => TimeTravelEffect::StartReplay { drafts: inputs(&value["drafts"]), from: mutation(&value["from"]) },
        "cancelReplay" => TimeTravelEffect::CancelReplay,
        "openFinalizePrompt" => TimeTravelEffect::OpenFinalizePrompt,
        "commitOverwrite" => TimeTravelEffect::CommitOverwrite { inputs: inputs(&value["inputs"]) },
        "commitAlternative" => TimeTravelEffect::CommitAlternative { name: text(&value["name"]).to_string(), inputs: inputs(&value["inputs"]) },
        "close" => TimeTravelEffect::Close,
        other => panic!("unknown effect {other}"),
    }
}

fn kinds(effects: &[TimeTravelEffect]) -> Vec<&'static str> {
    effects.iter().map(|effect| effect.kind().as_str()).collect()
}

fn expected_kinds(value: &Value) -> Vec<&str> {
    value.as_array().expect("effects").iter().map(text).collect()
}

fn rejection(value: &Value) -> TimeTravelRefusal {
    TimeTravelRefusal::parse(text(value)).expect("refusal")
}

fn strings(value: &Value) -> Vec<&str> {
    value.as_array().expect("array").iter().map(text).collect()
}

#[test]
fn vocabularies_mirror_the_fixture() {
    let law = law();
    assert_eq!(strings(&law["stages"]), TimeTravelStage::ALL.map(TimeTravelStage::as_str));
    assert_eq!(strings(&law["eventKeys"]), TimeTravelEventKey::ALL.map(TimeTravelEventKey::as_str));
    assert_eq!(strings(&law["effectKinds"]), TimeTravelEffectKind::ALL.map(TimeTravelEffectKind::as_str));
    assert_eq!(strings(&law["refusals"]), TimeTravelRefusal::ALL.map(TimeTravelRefusal::code));
    assert_eq!(strings(&law["reviewKinds"]), TimeTravelReview::ALL.map(TimeTravelReview::as_str));
    assert_eq!(text(&law["frozenCode"]), TIME_TRAVEL_FROZEN_CODE);
    assert_eq!(text(&law["cancelledCode"]), TIME_TRAVEL_CANCELLED_CODE);
    assert_eq!(law["limits"]["textMaxBytes"].as_u64(), Some(TIME_TRAVEL_TEXT_MAX_BYTES as u64));
    for refusal in TimeTravelRefusal::ALL {
        assert_eq!(TimeTravelRefusal::parse(&refusal.to_string()), Some(refusal));
    }
    for key in TimeTravelEventKey::ALL {
        let canonical = event(&law["events"][key.as_str()]);
        assert_eq!(canonical.key(), key, "canonical event of {}", key.as_str());
        assert!(canonical.is_well_formed(), "canonical event of {} is well formed", key.as_str());
    }
}

#[test]
fn every_stage_event_pair_is_covered_once_per_guard() {
    let law = law();
    let guards = strings(&law["guards"]);
    let rows = law["matrix"].as_array().expect("matrix");
    for stage in TimeTravelStage::ALL {
        for key in TimeTravelEventKey::ALL {
            let branches: Vec<&str> = rows.iter().filter(|row| text(&row["from"]) == stage.as_str() && text(&row["event"]) == key.as_str()).map(|row| text(&row["when"])).collect();
            assert!(!branches.is_empty(), "{} × {} has no row", stage.as_str(), key.as_str());
            assert!(branches.iter().all(|when| guards.contains(when)), "{} × {} uses an unknown guard", stage.as_str(), key.as_str());
            assert_eq!(branches.len(), branches.iter().collect::<std::collections::BTreeSet<_>>().len(), "{} × {} repeats a guard", stage.as_str(), key.as_str());
        }
    }
    assert_eq!(rows.len(), 130);
}

#[test]
fn every_matrix_row_holds() {
    let law = law();
    for row in law["matrix"].as_array().expect("matrix") {
        let label = format!("{} × {} [{}]", text(&row["from"]), text(&row["event"]), text(&row["when"]));
        let before = session(&law["contexts"][text(&row["context"])]);
        assert_eq!(before.stage.as_str(), text(&row["from"]), "{label}: context stage");
        assert_eq!(before.invariant_violation(), None, "{label}: context coherent");
        let mut after = before.clone();
        let input = row.get("input").map_or_else(|| event(&law["events"][text(&row["event"])]), event);
        assert_eq!(input.key().as_str(), text(&row["event"]), "{label}: input key");
        let result = after.apply(input);
        if let Some(code) = row.get("rejection") {
            assert_eq!(result, Err(rejection(code)), "{label}");
            assert_eq!(after, before, "{label}: a refusal leaves the session untouched");
            continue;
        }
        let effects = result.unwrap_or_else(|refusal| panic!("{label}: refused {refusal}"));
        assert_eq!(after.stage.as_str(), text(&row["to"]), "{label}: stage");
        assert_eq!(kinds(&effects), expected_kinds(&row["effects"]), "{label}: effects");
        let generation = if text(&row["generation"]) == "increment" { before.generation + 1 } else { before.generation };
        assert_eq!(after.generation, generation, "{label}: generation");
        let id = if text(&row["session"]) == "next" { before.id + 1 } else { before.id };
        assert_eq!(after.id, id, "{label}: session id");
        assert_eq!(after.invariant_violation(), None, "{label}: result coherent");
    }
}

#[test]
fn stale_generations_are_silent_no_ops_in_every_context() {
    let law = law();
    for (name, context) in law["contexts"].as_object().expect("contexts") {
        let before = session(context);
        for key in TimeTravelEventKey::ALL {
            let canonical = &law["events"][key.as_str()];
            if canonical.get("generation").is_none() {
                continue;
            }
            for generation in [before.generation.wrapping_sub(1), before.generation.wrapping_add(1)] {
                let mut stale = canonical.clone();
                stale["generation"] = generation.into();
                let mut after = before.clone();
                assert_eq!(after.apply(event(&stale)), Err(TimeTravelRefusal::Stale), "{name} × {} at generation {generation}", key.as_str());
                assert_eq!(after, before, "{name} × {}", key.as_str());
            }
        }
    }
}

#[test]
fn every_case_holds() {
    let law = law();
    for case in law["cases"].as_array().expect("cases") {
        let name = text(&case["name"]);
        let before = session(&case["session"]);
        let mut after = before.clone();
        let result = after.apply(event(&case["event"]));
        if let Some(code) = case["expect"].get("rejection") {
            assert_eq!(result, Err(rejection(code)), "{name}");
            assert_eq!(after, before, "{name}: untouched");
            continue;
        }
        let transition = &case["expect"]["transition"];
        let expected: Vec<TimeTravelEffect> = transition["effects"].as_array().expect("effects").iter().map(effect).collect();
        assert_eq!(result, Ok(expected), "{name}: effects");
        assert_eq!(after, session(&transition["session"]), "{name}: session");
        assert_eq!(after.invariant_violation(), None, "{name}: coherent");
    }
}

#[test]
fn every_scenario_replays() {
    let law = law();
    for scenario in law["scenarios"].as_array().expect("scenarios") {
        let name = text(&scenario["name"]);
        let mut current = session(&scenario["initial"]);
        for (index, step) in scenario["steps"].as_array().expect("steps").iter().enumerate() {
            let before = current.clone();
            let result = current.apply(event(&step["event"]));
            let expect = &step["expect"];
            if let Some(code) = expect.get("rejection") {
                assert_eq!(result, Err(rejection(code)), "{name} step {index}");
                assert_eq!(current, before, "{name} step {index}: untouched");
                continue;
            }
            let effects = result.unwrap_or_else(|refusal| panic!("{name} step {index}: refused {refusal}"));
            assert_eq!(current.stage, stage(&expect["stage"]), "{name} step {index}: stage");
            assert_eq!(kinds(&effects), expected_kinds(&expect["effects"]), "{name} step {index}: effects");
            assert_eq!(current.invariant_violation(), None, "{name} step {index}: coherent");
        }
        assert_eq!(current, session(&scenario["final"]), "{name}: final session");
    }
}

#[test]
fn invariants_hold_for_every_context_and_fail_for_their_counterexample() {
    let law = law();
    for (name, context) in law["contexts"].as_object().expect("contexts") {
        assert_eq!(session(context).invariant_violation(), None, "{name}");
    }
    for invariant in law["invariants"].as_array().expect("invariants") {
        assert_eq!(session(&invariant["violation"]).invariant_violation(), Some(text(&invariant["id"])));
    }
}

#[test]
fn every_context_reviews_as_the_fixture_says() {
    let law = law();
    let rows = law["reviews"].as_array().expect("reviews");
    assert_eq!(rows.len(), law["contexts"].as_object().expect("contexts").len());
    for row in rows {
        let name = text(&row["context"]);
        let context = session(&law["contexts"][name]);
        assert_eq!(context.review(), optional(&row["review"], |review| TimeTravelReview::parse(text(review)).expect("review")), "{name}");
        if context.stage == TimeTravelStage::Reviewing {
            assert_eq!(context.rerun_refusal().is_none(), context.review() == Some(TimeTravelReview::NeedsReplay) || (context.fault.is_some() && !context.accepted.is_empty()), "{name}: rerun iff a replay is needed or a fault is shown");
        }
    }
}

/// ✏️ `begin_refusal` answers, in every fixture context, exactly what applying the canonical `Begin` answers — the query a
/// host disables its Edit control by never disagrees with the reducer.
#[test]
fn begin_is_refused_exactly_where_the_reducer_refuses_it() {
    let law = law();
    for (name, context) in law["contexts"].as_object().expect("contexts") {
        let mut applied = session(context);
        let refusal = applied.begin_refusal();
        assert_eq!(refusal, applied.apply(event(&law["events"]["begin"])).err(), "{name}");
    }
}

#[test]
fn text_limits_hold_at_their_edges() {
    let max = TIME_TRAVEL_TEXT_MAX_BYTES;
    assert!(is_time_travel_fault_code(&"x".repeat(max)) && !is_time_travel_fault_code(&"x".repeat(max + 1)));
    assert!(!is_time_travel_fault_code("") && !is_time_travel_fault_code("vcs rejected") && !is_time_travel_fault_code("vcs\u{85}rejected"));
    assert!(is_time_travel_fault_code("vcs\u{feff}rejected"), "U+FEFF is not White_Space");
    assert!(is_time_travel_alternative_name(&"ä".repeat(max / 2)) && !is_time_travel_alternative_name(&"ä".repeat(max / 2 + 1)));
    assert!(!is_time_travel_alternative_name("") && !is_time_travel_alternative_name("\u{a0}\u{85}\u{2003}\t") && is_time_travel_alternative_name(" Edited history "));
    assert_eq!(TimeTravelLabel::for_code(TIME_TRAVEL_CANCELLED_CODE), Some(TimeTravelLabel::ReplayCancelled));
    assert_eq!(TimeTravelLabel::for_code("vcs.rejected"), None);
}

/// 🗂️ Every `timeTravel.*` code a host can be handed — the session refusals, the frozen and cancelled codes, the hosting
/// runtime's refusals and the driver faults — names exactly the fixture's label, and every one is a well-formed fault code.
#[test]
fn every_time_travel_code_names_the_fixture_label() {
    let law = law();
    let rows: Vec<(&str, &str)> = law["codeLabels"].as_array().expect("codeLabels").iter().map(|row| (text(&row["code"]), text(&row["key"]))).collect();
    assert_eq!(rows, TIME_TRAVEL_CODE_LABELS.map(|(code, label)| (code, label.key())));
    for (code, label) in TIME_TRAVEL_CODE_LABELS {
        assert!(is_time_travel_fault_code(code) && code.starts_with("timeTravel."), "{code}");
        assert_eq!(TimeTravelLabel::for_code(code), Some(label), "{code}");
    }
    for refusal in TimeTravelRefusal::ALL {
        assert_eq!(TimeTravelLabel::for_code(refusal.code()), Some(refusal.label()), "{refusal}");
    }
    assert_eq!(TimeTravelLabel::for_code(TIME_TRAVEL_FROZEN_CODE), Some(TimeTravelLabel::Frozen));
    let codes: std::collections::BTreeSet<&str> = TIME_TRAVEL_CODE_LABELS.iter().map(|(code, _)| *code).collect();
    assert_eq!(codes.len(), TIME_TRAVEL_CODE_LABELS.len(), "every code is listed once");
}

#[test]
fn labels_mirror_the_fixture() {
    let law = law();
    let rows: Vec<(&str, &str, &str)> = law["labels"].as_array().expect("labels").iter().map(|row| (text(&row["key"]), text(&row["en"]), text(&row["de"]))).collect();
    assert_eq!(rows, TimeTravelLabel::ALL.map(|label| (label.key(), label.en(), label.de())));
    for label in TimeTravelLabel::ALL {
        assert_eq!(TimeTravelLabel::parse(label.key()), Some(label));
        assert_eq!(label.localized(|en, de| (en, de)), (label.en(), label.de()));
        assert!(!label.en().is_empty() && !label.de().is_empty() && label.en() != label.de(), "{} has both locales", label.key());
    }
    for stage in TimeTravelStage::ALL {
        assert_eq!(stage.label().key(), format!("stage{}{}", stage.as_str()[..1].to_uppercase(), &stage.as_str()[1..]));
    }
    assert_eq!(TimeTravelReview::ALL.map(|review| review.label().key()), ["noChanges", "needsReplay", "reportBlocking", "readyToFinalize"]);
    for refusal in TimeTravelRefusal::ALL {
        let name = &refusal.code()["timeTravel.".len()..];
        assert_eq!(refusal.label().key(), format!("refusal{}{}", name[..1].to_uppercase(), &name[1..]));
    }
}

#[test]
fn stage_and_review_values_round_trip_as_bare_camel_case_strings() {
    for stage in TimeTravelStage::ALL {
        assert_eq!(stage.to_value(), DslValue::String(stage.as_str().to_string()));
        assert_eq!(TimeTravelStage::from_value(stage.to_value()).expect("round trip"), stage);
    }
    assert!(TimeTravelStage::from_value(DslValue::String("paused".to_string())).is_err());
    for review in TimeTravelReview::ALL {
        assert_eq!(review.to_value(), DslValue::String(review.as_str().to_string()));
        assert_eq!(TimeTravelReview::from_value(review.to_value()).expect("round trip"), review);
    }
}

#[test]
fn a_new_session_is_inactive_and_coherent() {
    let base = TimeTravelBase { store_generation: 7, content_revision: [3; 32] };
    let fresh = TimeTravelSession::new(base);
    assert_eq!((fresh.id, fresh.generation, fresh.base, fresh.stage), (0, 0, base, TimeTravelStage::Inactive));
    assert_eq!(fresh.invariant_violation(), None);
    assert_eq!(fresh.finalize_refusal(), Some(TimeTravelRefusal::Illegal));
}
