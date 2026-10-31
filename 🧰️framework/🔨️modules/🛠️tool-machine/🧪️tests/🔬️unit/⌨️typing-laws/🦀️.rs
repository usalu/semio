/// ⌨️ Typing laws: the typing tool tables, chart and phases against the fixture, the algebra folds, the ledger replay and the persisted run.

//#region 🔖️TypingLaws
fn typing_law_text() -> &'static str {
    crate::fixture_text("🧪️tests/🧫️fixtures/🧫️typing-law/🔣️.json")
}

fn typing_law() -> Value {
    serde_json::from_str(typing_law_text()).expect("typing fixture parses")
}

fn reference_typing_context(input: TypingContext) -> TypingContext {
    input
}

fn same_buffer(_context: &TypingContext, _event: Option<&typing::Event>) -> bool {
    true
}

fn type_on(_context: &mut TypingContext, _event: Option<&typing::Event>, _sink: &mut Vec<Command<typing::Typing>>) {}

fn end_run(_context: &mut TypingContext, _event: Option<&typing::Event>, _sink: &mut Vec<Command<typing::Typing>>) {}

machine::statechart! {
    machine typing {
        context: TypingContext;
        event Event { Edit { buffer: String, leaves: Vec<Value> }, Commit { reason: String } }
        input: TypingContext;
        output: ();
        effect: ToolYield<Value>;
        context_from_input: reference_typing_context;
        initial: idle;
        state idle {
            on Edit => typing do type_on;
        }
        state typing {
            after 750 => idle do end_run;
            on Edit if same_buffer => typing do type_on;
            on Commit => idle do end_run;
        }
    }
}

#[test]
fn typing_tables_are_the_statechart_compilation_of_the_typing_chart() {
    let reference = <typing::Typing as Machine>::definition();
    let generic = <TypingMachine<Value> as Machine>::definition();
    assert_eq!((generic.id, generic.guards.len(), generic.actions.len()), (reference.id, reference.guards.len(), reference.actions.len()));
    assert_eq!(format!("{:?}", generic.nodes), format!("{:?}", reference.nodes));
    assert_eq!(format!("{:?}", generic.transitions), format!("{:?}", reference.transitions));
    assert_eq!(generic.fingerprint, reference.fingerprint, "TYPING_FINGERPRINT");
    assert_eq!(generic.manifest_json, reference.manifest_json, "TYPING_MANIFEST_JSON");
    assert_eq!((TypingEvent::<Value>::EVENT_COUNT, TypingEvent::<Value>::event_name(machine::EventId(0)), TypingEvent::<Value>::event_name(machine::EventId(1))), (typing::Event::EVENT_COUNT, "Edit", "Commit"));
}

#[test]
fn typing_chart_is_the_fixture_chart() {
    let law = typing_law();
    let chart = &law["chart"];
    let definition = <TypingMachine<Value> as Machine>::definition();
    assert_eq!((chart["id"].as_str(), chart["fingerprint"].as_str(), chart["manifestJson"].as_str()), (Some(definition.id), Some(TYPING_FINGERPRINT.to_string().as_str()), Some(TYPING_MANIFEST_JSON)));
    assert_eq!(law["idleMs"].as_u64(), Some(TYPING_IDLE_MS));
    let states: Vec<&str> = definition.nodes.iter().map(|node| node.stable_id).collect();
    assert_eq!(json!(states), chart["states"]);
    assert_eq!(chart["initial"], json!(definition.nodes[definition.nodes[0].initial.expect("initial").0 as usize].stable_id));
    let actions = ["follow", "settle"];
    let rows: Vec<Value> = definition
        .transitions
        .iter()
        .map(|transition| {
            let trigger = match transition.trigger {
                Trigger::Event(event) => json!({ "event": TypingEvent::<Value>::event_name(event) }),
                Trigger::Timer(timer) => json!({ "afterMs": definition.nodes[transition.source.0 as usize].timers.iter().find(|(id, _)| *id == timer).expect("declared timer").1 }),
                other => panic!("unexpected trigger {other:?}"),
            };
            json!({ "from": definition.nodes[transition.source.0 as usize].stable_id, "trigger": trigger, "guard": transition.guard.map(|_| "sameBuffer"), "to": definition.nodes[transition.targets[0].0 as usize].stable_id, "action": actions[transition.actions[0].0 as usize] })
        })
        .collect();
    assert_eq!(json!(rows), chart["transitions"]);
    assert_eq!(json!(TypingCommit::ALL.map(TypingCommit::as_str)), law["reasons"]);
}

#[test]
fn typing_phases_parse_like_the_fixture() {
    let law = typing_law();
    assert_eq!(law["args"], json!({ "buffer": TYPING_BUFFER_ARG, "commit": TYPING_COMMIT_ARG }));
    for case in law["phases"].as_array().expect("phases") {
        let args = &case["args"];
        let expected = match TypingPhase::parse(args[TYPING_BUFFER_ARG].as_str(), args[TYPING_COMMIT_ARG].as_str()) {
            None => Value::Null,
            Some(TypingPhase::Edit { buffer }) => json!({ "kind": "edit", "buffer": buffer }),
            Some(TypingPhase::Commit { buffer, reason }) => json!({ "kind": "commit", "buffer": buffer, "reason": reason.as_str() }),
        };
        assert_eq!(expected, case["phase"], "{args}");
    }
}

/// 🔗️ The fixture's typing algebra: an insertion `{at, text}` folds into an open one that ends where it starts, a whole buffer
/// `{text}` replaces a whole buffer, an edit without leaves cancels the net, anything else splits.
fn fixture_fold(net: &[Value], next: &[Value]) -> TypingFold<Value> {
    match (net, next) {
        (_, []) => TypingFold::Net(Vec::new()),
        ([open], [typed]) if open.get("at").is_some() && typed.get("at").is_some() => {
            let end = open["at"].as_u64().expect("at") + text(&open["text"]).chars().count() as u64;
            if typed["at"].as_u64() == Some(end) {
                TypingFold::Net(vec![json!({ "at": open["at"], "text": format!("{}{}", text(&open["text"]), text(&typed["text"])) })])
            } else {
                TypingFold::Split
            }
        }
        ([open], [typed]) if open.get("at").is_none() && typed.get("at").is_none() => TypingFold::Net(vec![typed.clone()]),
        _ => TypingFold::Split,
    }
}

#[test]
fn typing_algebra_folds_like_the_fixture() {
    for case in typing_law()["algebra"]["folds"].as_array().expect("folds") {
        let fold = match fixture_fold(case["net"].as_array().expect("net"), case["next"].as_array().expect("next")) {
            TypingFold::Net(leaves) => json!({ "kind": "net", "leaves": leaves }),
            TypingFold::Split => json!({ "kind": "split" }),
        };
        assert_eq!(fold, case["fold"], "{case}");
    }
}

fn typing_step_json(ledger: &TypingLedger<Value>, window: &str, step: &ToolStep<Value>) -> Value {
    match step {
        ToolStep::Open => json!({ "kind": "open", "transaction": reference_json(&ledger.open(window).expect("an open step persists").transaction) }),
        other => scrub_step_json(other),
    }
}

fn typing_open_json(ledger: &TypingLedger<Value>) -> Value {
    Value::Object(
        ledger
            .windows()
            .map(|window| {
                let state = ledger.open(window).expect("listed window");
                (window.to_string(), json!({ "buffer": state.buffer, "tool": state.tool, "deadline": state.deadline_ms, "transaction": reference_json(&state.transaction), "net": state.net }))
            })
            .collect(),
    )
}

fn hlc(ms: u64) -> HybridLogicalTimestamp {
    HybridLogicalTimestamp { actor: 0, physical_ms: ms, logical: 0 }
}

fn windowed(ledger: &TypingLedger<Value>, all: Vec<(String, Result<ToolStep<Value>, ToolRefusal>)>) -> Value {
    json!(all.into_iter().map(|(window, step)| json!([window, typing_step_json(ledger, &window, &step.expect("never refused"))])).collect::<Vec<_>>())
}

/// ⚖️ LAW: every scenario of the language-agnostic typing fixture — one run is one transaction holding the net text, idle
/// commits it, a caret jump or another buffer or tool splits it, a commit signal ends it, a host abort or freeze leaves zero
/// trace, an erased run commits empty, windows type independently, a retired window commits — replays step by step with the
/// exact minted ids, open runs, deadlines and overlay.
#[test]
fn typing_ledger_replays_every_fixture_scenario() {
    let law = typing_law();
    let actor = ActorId(text(&law["actor"]).into());
    for scenario in law["scenarios"].as_array().expect("scenarios") {
        let mut ledger = TypingLedger::<Value>::default();
        for (index, row) in scenario["steps"].as_array().expect("steps").iter().enumerate() {
            let context = format!("{} step {index}", text(&scenario["name"]));
            let expect = &row["expect"];
            if let Some(ms) = row["lapse"].as_u64() {
                let lapsed = ledger.lapse(hlc(ms));
                assert_eq!(windowed(&ledger, lapsed), expect["lapsed"], "{context}");
            } else if let Some(reason) = row["abortAll"].as_str() {
                let all = ledger.abort_all(ToolAbortReason::parse(reason).expect("reason")).into_iter().map(|(window, step)| (window, Ok(step))).collect();
                assert_eq!(windowed(&ledger, all), expect["all"], "{context}");
            } else if let Some(reason) = row["commitAll"].as_str() {
                let all = ledger.commit_all(TypingCommit::parse(reason).expect("reason"), hlc(row["clock"].as_u64().expect("clock")));
                assert_eq!(windowed(&ledger, all), expect["all"], "{context}");
            } else if let Some(keep) = row["retain"].as_array() {
                let keep: Vec<&str> = keep.iter().map(text).collect();
                let all = ledger.retain_windows(|window| keep.contains(&window), hlc(row["clock"].as_u64().expect("clock")));
                assert_eq!(windowed(&ledger, all), expect["all"], "{context}");
            } else if let Some(reason) = row["abort"].as_str() {
                let step = ledger.abort(text(&row["window"]), ToolAbortReason::parse(reason).expect("reason"));
                assert_eq!(typing_step_json(&ledger, text(&row["window"]), &step), expect["step"], "{context}");
            } else if let Some(reason) = row["commit"].as_str() {
                let step = ledger.commit(text(&row["window"]), TypingCommit::parse(reason).expect("reason"), hlc(row["clock"].as_u64().expect("clock"))).expect("never refused");
                assert_eq!(typing_step_json(&ledger, text(&row["window"]), &step), expect["step"], "{context}");
            } else {
                let window = text(&row["window"]);
                let input = TypingInput::Edit { buffer: text(&row["input"]["buffer"]).to_string(), leaves: row["input"]["leaves"].as_array().expect("leaves").clone() };
                let steps = ledger.send(window, text(&row["tool"]), &actor, input, fixture_fold, hlc(row["clock"].as_u64().expect("clock"))).expect("typing is never refused");
                assert_eq!(json!(steps.iter().map(|step| typing_step_json(&ledger, window, step)).collect::<Vec<_>>()), expect["steps"], "{context}");
            }
            assert_eq!(typing_open_json(&ledger), expect["open"], "{context}");
            assert_eq!(json!(ledger.provisional().cloned().collect::<Vec<_>>()), expect["provisional"], "{context}");
        }
    }
}

/// ✏️ LAW: a run persisted between every two edits is the run that never persisted — the stable-id configuration, the
/// context rebuilt from the buffer and entries, the idle deadline and the open transaction continue the run unchanged.
#[test]
fn a_typing_run_persisted_between_edits_continues_the_same_run() {
    let actor = ActorId("actor-1".into());
    let mut live = Typing::<Value>::start("demo#textSplice", actor.clone());
    let mut persisted: Option<TypingState<Value>> = None;
    for (index, letter) in "hello".chars().enumerate() {
        let leaves = vec![json!({ "at": index, "text": letter.to_string() })];
        let clock = hlc(1_000 + 100 * index as u64);
        let live_steps = live.send(TypingInput::Edit { buffer: "s".into(), leaves: leaves.clone() }, fixture_fold, clock).expect("live edit");
        let mut resumed = persisted.take().map_or_else(|| Typing::start("demo#textSplice", actor.clone()), |state| Typing::resume(state).expect("resumes"));
        assert_eq!(resumed.send(TypingInput::Edit { buffer: "s".into(), leaves }, fixture_fold, clock).expect("resumed edit"), live_steps, "edit {index}");
        assert_eq!((resumed.deadline_ms(), resumed.transaction().map(|transaction| transaction.entries().to_vec())), (live.deadline_ms(), live.transaction().map(|transaction| transaction.entries().to_vec())), "edit {index}");
        persisted = resumed.persist();
    }
    let mut resumed = Typing::resume(persisted.expect("an open run persists")).expect("resumes");
    assert_eq!(resumed.lapse(hlc(2_000)).expect("lapse"), None, "not idle yet");
    let committed = resumed.lapse(hlc(1_400 + TYPING_IDLE_MS)).expect("lapse");
    assert_eq!(committed, live.lapse(hlc(1_400 + TYPING_IDLE_MS)).expect("lapse"));
    assert!(matches!(committed, Some(ToolStep::Committed(_, ref leaves)) if leaves == &vec![json!({ "at": 0, "text": "hello" })]));
    assert!(resumed.persist().is_none(), "a committed run persists nothing");
}

/// ▶️ LAW: 200 characters typed in one burst are ONE run — one transaction, one committed edit — however many deliveries carry
/// them, so typing never spends the store's fixed 64-slot edit ledger one keystroke at a time.
#[test]
fn two_hundred_typed_characters_are_one_run() {
    let actor = ActorId("actor-1".into());
    let mut ledger = TypingLedger::<Value>::default();
    let typed: String = (0..200).map(|index| char::from(b'a' + (index % 26) as u8)).collect();
    let mut transactions = std::collections::BTreeSet::new();
    for (index, letter) in typed.chars().enumerate() {
        let steps = ledger.send("w", "demo#textSplice", &actor, TypingInput::Edit { buffer: "s".into(), leaves: vec![json!({ "at": index, "text": letter.to_string() })] }, fixture_fold, hlc(10_000 + 120 * index as u64)).expect("typing");
        assert_eq!(steps, vec![ToolStep::Open], "character {index} stays in the run");
        transactions.insert(ledger.open("w").expect("open").transaction.id.clone());
    }
    assert_eq!(transactions.len(), 1, "one transaction for the whole burst");
    let committed = ledger.commit("w", TypingCommit::Blur, hlc(40_000)).expect("commit");
    assert!(matches!(committed, ToolStep::Committed(_, ref leaves) if leaves == &vec![json!({ "at": 0, "text": typed })]), "{committed:?}");
}
//#endregion 🔖️TypingLaws
