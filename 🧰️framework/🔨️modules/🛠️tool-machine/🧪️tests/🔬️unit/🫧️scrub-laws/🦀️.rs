/// 🫧️ Scrub laws: the scrub tool tables, chart and phases against the fixture, the ledger replay, the persisted scrubs and the late inputs.

//#region 🔖️ScrubLaws
fn scrub_law_text() -> &'static str {
    crate::fixture_text("🧪️tests/🧫️fixtures/🧫️scrub-law/🔣️.json")
}

fn scrub_law() -> Value {
    serde_json::from_str(scrub_law_text()).expect("scrub fixture parses")
}

fn reference_scrub_context(input: ScrubContext) -> ScrubContext {
    input
}

fn same_gesture(_context: &ScrubContext, _event: Option<&scrub::Event>) -> bool {
    true
}

fn follow(_context: &mut ScrubContext, _event: Option<&scrub::Event>, _sink: &mut Vec<Command<scrub::Scrub>>) {}

fn settle(_context: &mut ScrubContext, _event: Option<&scrub::Event>, _sink: &mut Vec<Command<scrub::Scrub>>) {}

machine::statechart! {
    machine scrub {
        context: ScrubContext;
        event Event { Tick { gesture: String, leaves: Vec<Value> }, Commit { gesture: String, leaves: Vec<Value> } }
        input: ScrubContext;
        output: ();
        effect: ToolYield<Value>;
        context_from_input: reference_scrub_context;
        initial: idle;
        state idle {
            on Tick => scrubbing do follow;
            on Commit => idle do settle;
        }
        state scrubbing {
            on Tick if same_gesture => scrubbing do follow;
            on Commit if same_gesture => idle do settle;
        }
    }
}

#[test]
fn scrub_tables_are_the_statechart_compilation_of_the_scrub_chart() {
    let reference = <scrub::Scrub as Machine>::definition();
    let generic = <ScrubMachine<Value> as Machine>::definition();
    assert_eq!((generic.id, generic.guards.len(), generic.actions.len()), (reference.id, reference.guards.len(), reference.actions.len()));
    assert_eq!(format!("{:?}", generic.nodes), format!("{:?}", reference.nodes));
    assert_eq!(format!("{:?}", generic.transitions), format!("{:?}", reference.transitions));
    assert_eq!(generic.fingerprint, reference.fingerprint, "SCRUB_FINGERPRINT");
    assert_eq!(generic.manifest_json, reference.manifest_json, "SCRUB_MANIFEST_JSON");
    assert_eq!((ScrubEvent::<Value>::EVENT_COUNT, ScrubEvent::<Value>::event_name(machine::EventId(0)), ScrubEvent::<Value>::event_name(machine::EventId(1))), (scrub::Event::EVENT_COUNT, "Tick", "Commit"));
}

#[test]
fn scrub_chart_is_the_fixture_chart() {
    let law = scrub_law();
    let chart = &law["chart"];
    let definition = <ScrubMachine<Value> as Machine>::definition();
    assert_eq!((chart["id"].as_str(), chart["fingerprint"].as_str(), chart["manifestJson"].as_str()), (Some(definition.id), Some(SCRUB_FINGERPRINT.to_string().as_str()), Some(SCRUB_MANIFEST_JSON)));
    let states: Vec<&str> = definition.nodes[1..].iter().map(|node| node.stable_id).collect();
    assert_eq!(json!(states), chart["states"]);
    assert_eq!(chart["initial"], json!(definition.nodes[definition.nodes[0].initial.expect("initial").0 as usize].stable_id));
    let actions = ["follow", "settle"];
    let rows: Vec<Value> = definition
        .transitions
        .iter()
        .map(|transition| {
            let Trigger::Event(event) = transition.trigger else { panic!("event triggers only") };
            json!({
                "from": definition.nodes[transition.source.0 as usize].stable_id,
                "event": ScrubEvent::<Value>::event_name(event),
                "guard": transition.guard.map(|_| "sameGesture"),
                "to": definition.nodes[transition.targets[0].0 as usize].stable_id,
                "action": actions[transition.actions[0].0 as usize],
            })
        })
        .collect();
    assert_eq!(json!(rows), chart["transitions"]);
}

#[test]
fn scrub_phases_parse_like_the_fixture() {
    let law = scrub_law();
    assert_eq!(law["args"], json!({ "gesture": SCRUB_GESTURE_ARG, "commit": SCRUB_COMMIT_ARG, "abort": SCRUB_ABORT_ARG }));
    for case in law["phases"].as_array().expect("phases") {
        let args = &case["args"];
        let phase = ScrubPhase::parse(args[SCRUB_GESTURE_ARG].as_str(), args[SCRUB_COMMIT_ARG].as_bool(), args[SCRUB_ABORT_ARG].as_str());
        let expected = match phase {
            None => Value::Null,
            Some(ScrubPhase::Tick { gesture }) => json!({ "kind": "tick", "gesture": gesture }),
            Some(ScrubPhase::Commit { gesture }) => json!({ "kind": "commit", "gesture": gesture }),
            Some(ScrubPhase::Abort { gesture, reason }) => json!({ "kind": "abort", "gesture": gesture, "reason": reason.as_str() }),
        };
        assert_eq!(expected, case["phase"], "{args}");
    }
}

fn scrub_step_json(step: &ToolStep<Value>) -> Value {
    match step {
        ToolStep::Idle => json!({ "kind": "idle" }),
        ToolStep::Open => panic!("open steps are expected with their transaction"),
        ToolStep::Committed(transaction, mutations) => json!({ "kind": "committed", "transaction": reference_json(transaction), "mutations": mutations }),
        ToolStep::Aborted(transaction, reason) => json!({ "kind": "aborted", "transaction": reference_json(transaction), "reason": reason.as_str() }),
        ToolStep::Empty(transaction) => json!({ "kind": "empty", "transaction": reference_json(transaction) }),
    }
}

fn scrub_open_json(ledger: &ScrubLedger<Value>) -> Value {
    Value::Object(
        ledger
            .windows()
            .map(|window| {
                let state = ledger.open(window).expect("listed window");
                (window.to_string(), json!({ "gesture": state.gesture, "base": state.base_revision, "tool": state.tool, "transaction": reference_json(&state.transaction), "entries": entries_json(&state.entries, Value::clone) }))
            })
            .collect(),
    )
}

fn scrub_input(value: &Value) -> ScrubInput<Value> {
    let leaves = || value["leaves"].as_array().expect("leaves").clone();
    match text(&value["kind"]) {
        "tick" => ScrubInput::Tick { gesture: text(&value["gesture"]).to_string(), leaves: leaves() },
        "commit" => ScrubInput::Commit { gesture: text(&value["gesture"]).to_string(), leaves: leaves() },
        "abort" => ScrubInput::Abort { reason: ToolAbortReason::parse(text(&value["reason"])).expect("reason") },
        other => panic!("unknown scrub input {other}"),
    }
}

/// ⚖️ LAW: every scenario of the language-agnostic scrub fixture — one press is one transaction holding the net
/// value, a host abort leaves zero trace, two presses are two transactions, another press, tool or document revision
/// reopens, windows scrub independently — replays step by step with the exact minted ids, open scrubs and overlay.
#[test]
fn scrub_ledger_replays_every_fixture_scenario() {
    let law = scrub_law();
    let actor = ActorId(text(&law["actor"]).to_string());
    for scenario in law["scenarios"].as_array().expect("scenarios") {
        let mut ledger = ScrubLedger::<Value>::default();
        for (index, row) in scenario["steps"].as_array().expect("steps").iter().enumerate() {
            let context = format!("{} step {index}", text(&scenario["name"]));
            let expect = &row["expect"];
            if let Some(reason) = row["abortAll"].as_str() {
                let steps: Vec<Value> = ledger.abort_all(ToolAbortReason::parse(reason).expect("reason")).iter().map(scrub_step_json).collect();
                assert_eq!(json!(steps), expect["steps"], "{context}");
            } else if let Some(keep) = row["retain"].as_array() {
                let keep: Vec<&str> = keep.iter().map(text).collect();
                let steps: Vec<Value> = ledger.retain_windows(|window| keep.contains(&window)).iter().map(scrub_step_json).collect();
                assert_eq!(json!(steps), expect["steps"], "{context}");
            } else if let Some(abort) = row["abort"].as_object() {
                let step = ledger.abort(text(&row["window"]), abort["gesture"].as_str(), ToolAbortReason::parse(abort["reason"].as_str().expect("reason")).expect("reason"));
                assert_eq!(scrub_step_json(&step), expect["step"], "{context}");
            } else {
                let clock = HybridLogicalTimestamp { actor: 0, physical_ms: row["clock"].as_u64().expect("clock"), logical: 0 };
                let step = ledger.send(text(&row["window"]), text(&row["tool"]), &actor, text(&row["base"]), &mut Some(scrub_input(&row["input"])), clock).expect("scrubs are never refused");
                let step = match step {
                    ToolStep::Open => json!({ "kind": "open", "transaction": reference_json(&ledger.open(text(&row["window"])).expect("an open step persists").transaction) }),
                    other => scrub_step_json(&other),
                };
                assert_eq!(step, expect["step"], "{context}");
            }
            assert_eq!(scrub_open_json(&ledger), expect["open"], "{context}");
            assert_eq!(json!(ledger.provisional().cloned().collect::<Vec<_>>()), expect["provisional"], "{context}");
        }
    }
}

/// ✔️ LAW: a scrub persisted between every two ticks is the scrub that never persisted — the stable-id configuration,
/// the context rebuilt from the press and its entries, and the open transaction continue the press unchanged.
#[test]
fn a_scrub_persisted_between_ticks_continues_the_same_press() {
    let actor = ActorId("actor-1".into());
    let clock = |ms: u64| HybridLogicalTimestamp { actor: 0, physical_ms: ms, logical: 0 };
    let ticks = [json!([1, 2]), json!([3]), json!([4, 5, 6]), json!([])];
    let mut live = Scrub::<Value>::start("demo#set", actor.clone(), "r1");
    let mut persisted: Option<ScrubState<Value>> = None;
    for (index, leaves) in ticks.iter().enumerate() {
        let input = ScrubInput::Tick { gesture: "g".into(), leaves: leaves.as_array().expect("leaves").clone() };
        let live_step = live.send(input.clone(), clock(index as u64)).expect("live tick");
        let mut resumed = persisted.take().map_or_else(|| Scrub::start("demo#set", actor.clone(), "r1"), |state| Scrub::resume(state).expect("resumes"));
        assert_eq!(resumed.send(input, clock(index as u64)).expect("resumed tick"), live_step, "tick {index}");
        assert_eq!(resumed.transaction().map(|transaction| transaction.entries().to_vec()), live.transaction().map(|transaction| transaction.entries().to_vec()), "tick {index}");
        persisted = resumed.persist();
    }
    let commit = ScrubInput::Commit { gesture: "g".into(), leaves: vec![json!(7)] };
    let mut resumed = persisted.map_or_else(|| Scrub::start("demo#set", actor.clone(), "r1"), |state| Scrub::resume(state).expect("resumes"));
    assert_eq!(resumed.send(commit.clone(), clock(9)), live.send(commit, clock(9)));
    assert!(resumed.persist().is_none() && live.persist().is_none(), "a released scrub persists nothing");
}

fn explore_scrub_ledger(ledger: &ScrubLedger<Value>, alphabet: &[(&str, &str, ScrubInput<Value>)], depth: usize, actor: &ActorId, sends: &mut usize) {
    if depth == 0 {
        return;
    }
    for (tool, base, input) in alphabet {
        let late = match input {
            ScrubInput::Tick { gesture, .. } | ScrubInput::Commit { gesture, .. } => ledger.closed("w")==Some(gesture.as_str()),
            ScrubInput::Abort { .. } => false,
        };
        let mut next = ledger.reference_fork();
        let step = next.send("w", tool, actor, base, input.clone(), HybridLogicalTimestamp { actor: 0, physical_ms: *sends as u64, logical: 0 }).expect("the scrub ledger never refuses");
        *sends += 1;
        if late {
            assert_eq!((&step, &next), (&ToolStep::Idle, ledger), "a late input of the closed press changes nothing");
        }
        if !late && matches!(input, ScrubInput::Commit { .. }) {
            assert!(next.open("w").is_none(), "a release always decides its press");
        }
        explore_scrub_ledger(&next, alphabet, depth - 1, actor, sends);
    }
}

/// 🚫️ LAW (CLOSURE-3): the scrub ledger is total — every order of up to four inputs on one window, across two presses, two
/// tools, two document revisions, empty and non-empty ticks, releases and host aborts, answers a step and never a refusal,
/// so a release always decides its press (the runtime publishes every lane of the press from that one step); a late input
/// of the press the window already closed leaves the ledger exactly as it was.
#[test]
fn the_scrub_ledger_never_refuses_and_late_inputs_change_nothing() {
    let mut alphabet = vec![("t1", "r1", ScrubInput::Abort { reason: ToolAbortReason::Blur })];
    for (tool, base) in [("t1", "r1"), ("t1", "r2"), ("t2", "r1")] {
        for gesture in ["g1", "g2"] {
            alphabet.push((tool, base, ScrubInput::Tick { gesture: gesture.into(), leaves: vec![json!(gesture), json!(1)] }));
            alphabet.push((tool, base, ScrubInput::Tick { gesture: gesture.into(), leaves: Vec::new() }));
            alphabet.push((tool, base, ScrubInput::Commit { gesture: gesture.into(), leaves: vec![json!(2)] }));
        }
    }
    let mut sends = 0;
    explore_scrub_ledger(&ScrubLedger::default(), &alphabet, 4, &ActorId("actor".into()), &mut sends);
    assert_eq!(sends, (1..=4).map(|depth| alphabet.len().pow(depth)).sum::<usize>());
}

#[test]
fn a_persisted_scrub_of_another_chart_is_refused() {
    let state = ScrubState { states: vec!["nowhere".into()], tool: "demo#set".into(), actor: "a".into(), gesture: "g".into(), base_revision: "r".into(), transaction: TransactionRef { id: "tx-0000000000000000".into(), tool: "demo#set".into() }, entries: vec![("0".into(), json!(1))] };
    assert_eq!(Scrub::<Value>::resume(state.clone()).err(), Some(ToolRefusal::Closed));
    assert_eq!(Scrub::<Value>::resume(ScrubState { states: vec!["idle".into()], ..state }).err(), Some(ToolRefusal::Unclosed), "a resting scrub holding an open transaction");
}
//#endregion 🔖️ScrubLaws
