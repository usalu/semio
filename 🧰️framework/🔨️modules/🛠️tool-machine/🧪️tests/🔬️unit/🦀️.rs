//! 🔬️ Rust reducer, id minting and runner against the transaction-law fixture, with the example drag
//! gesture statecharts the fixture's chart variants describe; the third-party `blake3` crate pins the ids.
#![allow(unexpected_cfgs)]

use super::*;
use machine::{InvokeId, StatechartEvent, TestHost, Trigger};
use serde_json::{json, Value};

const LAW: &str = include_str!("../../🧫️fixtures/🧫️transaction-law/🔣️.json");

fn law() -> Value {
    serde_json::from_str(LAW).expect("fixture parses")
}

fn text(value: &Value) -> &str {
    value.as_str().expect("string")
}

fn int(value: &Value) -> i64 {
    value.as_i64().expect("integer")
}

fn clock(value: &Value) -> HybridLogicalTimestamp {
    HybridLogicalTimestamp { actor: value["actor"].as_u64().expect("u64"), physical_ms: value["physical_ms"].as_u64().expect("u64"), logical: value["logical"].as_u64().expect("u64") }
}

fn reference(value: &Value) -> TransactionRef {
    TransactionRef { id: text(&value["id"]).to_string(), tool: text(&value["tool"]).to_string() }
}

fn reference_json(reference: &TransactionRef) -> Value {
    json!({ "id": reference.id, "tool": reference.tool })
}

fn yielded(value: &Value) -> ToolYield<Value> {
    match text(&value["kind"]) {
        "upsert" => ToolYield::upsert(text(&value["key"]), value["mutation"].clone()),
        "retract" => ToolYield::retract(text(&value["key"])),
        "commit" => ToolYield::Commit,
        "abort" => ToolYield::Abort,
        other => panic!("unknown yield {other}"),
    }
}

fn entries_json<M>(entries: &[(String, M)], mutation: impl Fn(&M) -> Value) -> Value {
    Value::Array(entries.iter().map(|(key, value)| json!({ "key": key, "mutation": mutation(value) })).collect())
}

fn sample_yield(kind: ToolYieldKind) -> ToolYield<Value> {
    match kind {
        ToolYieldKind::Upsert => ToolYield::upsert("k", json!(1)),
        ToolYieldKind::Retract => ToolYield::retract("k"),
        ToolYieldKind::Commit => ToolYield::Commit,
        ToolYieldKind::Abort => ToolYield::Abort,
    }
}

fn transaction_in(state: ToolTransactionState) -> ToolTransaction<Value> {
    let mut transaction = ToolTransaction::open(TransactionRef { id: "tx-0000000000000000".into(), tool: "law#matrix".into() });
    match state {
        ToolTransactionState::Open => {}
        ToolTransactionState::Committed => transaction.apply(ToolYield::Commit).expect("open accepts commit"),
        ToolTransactionState::Aborted => transaction.apply(ToolYield::Abort).expect("open accepts abort"),
    }
    transaction
}

//#region 🔖️Reducer
#[test]
fn states_yield_kinds_refusals_and_reasons_mirror_the_fixture() {
    let law = law();
    assert_eq!(law["states"], json!(ToolTransactionState::ALL.map(ToolTransactionState::as_str)));
    assert_eq!(law["yieldKinds"], json!(ToolYieldKind::ALL.map(ToolYieldKind::as_str)));
    assert_eq!(law["refusals"], json!(ToolRefusal::ALL.map(ToolRefusal::code)));
    assert_eq!(law["abortReasons"], json!(ToolAbortReason::ALL.map(ToolAbortReason::as_str)));
    for refusal in ToolRefusal::ALL {
        assert_eq!(ToolRefusal::parse(&refusal.to_string()), Some(refusal));
    }
    for reason in ToolAbortReason::ALL {
        assert_eq!(ToolAbortReason::parse(reason.as_str()), Some(reason));
    }
    for kind in ToolYieldKind::ALL {
        assert_eq!(ToolYieldKind::parse(kind.as_str()), Some(kind));
        assert_eq!(sample_yield(kind).kind(), kind);
    }
    for state in ToolTransactionState::ALL {
        assert_eq!(ToolTransactionState::parse(state.as_str()), Some(state));
    }
}

#[test]
fn reducer_matches_every_matrix_row() {
    let law = law();
    let rows = law["matrix"].as_array().expect("matrix");
    assert_eq!(rows.len(), ToolTransactionState::ALL.len() * ToolYieldKind::ALL.len());
    for row in rows {
        let from = ToolTransactionState::parse(text(&row["from"])).expect("state");
        let kind = ToolYieldKind::parse(text(&row["yield"])).expect("yield");
        let label = format!("{} × {}", from.as_str(), kind.as_str());
        let mut transaction = transaction_in(from);
        let before = transaction.clone();
        let result = transaction.apply(sample_yield(kind));
        match row.get("refusal") {
            Some(code) => {
                assert_eq!(result.map_err(|refusal| refusal.to_string()), Err(text(code).to_string()), "{label}");
                assert_eq!(transaction, before, "{label}: a refused yield changes nothing");
            }
            None => {
                assert_eq!(result, Ok(()), "{label}");
                assert_eq!(transaction.state().as_str(), text(&row["to"]), "{label}");
            }
        }
    }
}

#[test]
fn resume_upserts_entries_in_order() {
    let resumed = ToolTransaction::resume(TransactionRef { id: "tx-0000000000000000".into(), tool: "law#resume".into() }, vec![("a".into(), 1), ("b".into(), 2), ("a".into(), 3)]);
    assert_eq!(resumed.state(), ToolTransactionState::Open);
    assert_eq!(resumed.entries(), &[("a".to_string(), 3), ("b".to_string(), 2)]);
}

#[test]
fn reducer_matches_every_case() {
    let law = law();
    for case in law["cases"].as_array().expect("cases") {
        let name = text(&case["name"]);
        let mut transaction = ToolTransaction::open(reference(&case["transaction"]));
        let mut refused = Vec::new();
        for (index, value) in case["yields"].as_array().expect("yields").iter().enumerate() {
            if transaction.apply(yielded(value)).is_err() {
                refused.push(index);
            }
        }
        assert_eq!(transaction.state().as_str(), text(&case["expect"]["state"]), "{name}");
        assert_eq!(entries_json(transaction.entries(), Value::clone), case["expect"]["entries"], "{name}");
        assert_eq!(json!(refused), case["expect"]["refused"], "{name}");
        assert_eq!(transaction.reference(), &reference(&case["transaction"]), "{name}");
        if transaction.state() == ToolTransactionState::Open {
            let entries = transaction.entries().to_vec();
            assert_eq!(ToolTransaction::resume(transaction.reference().clone(), entries), transaction, "{name}: resuming its entries rebuilds the open transaction");
        }
        let (_, mutations) = transaction.into_parts();
        assert_eq!(Value::Array(mutations), Value::Array(case["expect"]["entries"].as_array().expect("entries").iter().map(|entry| entry["mutation"].clone()).collect()), "{name}");
    }
}
//#endregion 🔖️Reducer

//#region 🔖️Ids
fn leb128(out: &mut Vec<u8>, mut value: u64) {
    loop {
        let byte = (value & 0x7f) as u8;
        value >>= 7;
        if value == 0 {
            out.push(byte);
            return;
        }
        out.push(byte | 0x80);
    }
}

fn length_prefixed(out: &mut Vec<u8>, text: &str) {
    leb128(out, text.len() as u64);
    out.extend_from_slice(text.as_bytes());
}

#[test]
fn replication_mint_and_the_blake3_crate_reproduce_every_fixture_id() {
    let law = law();
    for row in law["ids"].as_array().expect("ids") {
        let (actor, tool, expected) = (text(&row["actor"]), text(&row["tool"]), text(&row["id"]));
        let clock = clock(&row["clock"]);
        let minted = TransactionRef::mint(&ActorId(actor.to_string()), &clock, tool);
        assert_eq!(minted, TransactionRef { id: expected.to_string(), tool: tool.to_string() }, "{row}");
        let mut material = Vec::new();
        length_prefixed(&mut material, actor);
        leb128(&mut material, clock.actor);
        leb128(&mut material, clock.physical_ms);
        leb128(&mut material, clock.logical);
        length_prefixed(&mut material, tool);
        let oracle: String = blake3::hash(&material).as_bytes()[..8].iter().map(|byte| format!("{byte:02x}")).collect();
        assert_eq!(format!("tx-{oracle}"), expected, "{row}");
    }
}
//#endregion 🔖️Ids

//#region 🔖️Gesture
/// 🧲️ Example drag gesture input: the magnet a dragged pointer connects to within `radius`.
#[derive(Clone, Debug)]
pub struct GestureInput {
    pub magnet: (i64, i64),
    pub radius: i64,
}

/// 🤏️ Example tool state: the input plus the press origin.
#[derive(Clone, Debug)]
pub struct GestureContext {
    pub input: GestureInput,
    pub origin: (i64, i64),
}

/// 🧩️ Example parametric mutations: the net drag offset and a connection to the magnet.
#[derive(Clone, Debug, PartialEq)]
pub enum GestureMutation {
    Translate { dx: i64, dy: i64 },
    Connect { x: i64, y: i64 },
}

/// 🎨️ One statechart per fixture chart variant; the actions are shared, the charts differ.
pub trait GestureTool: ToolMachine<Mutation = GestureMutation> + Machine<Input = GestureInput, Context = GestureContext> {
    fn point(event: &Self::Event) -> Option<(i64, i64)>;
    fn event(value: &Value) -> Self::Event;
}

fn gesture_context(input: GestureInput) -> GestureContext {
    GestureContext { input, origin: (0, 0) }
}

fn press<T: GestureTool>(context: &mut GestureContext, event: Option<&T::Event>, _sink: &mut Vec<Command<T>>) {
    if let Some(point) = event.and_then(T::point) {
        context.origin = point;
    }
}

fn drag<T: GestureTool>(context: &mut GestureContext, event: Option<&T::Event>, sink: &mut Vec<Command<T>>) {
    let Some((x, y)) = event.and_then(T::point) else { return };
    let (dx, dy) = (x - context.origin.0, y - context.origin.1);
    sink.push(Command::Effect(if dx == 0 && dy == 0 { ToolYield::retract("translate") } else { ToolYield::upsert("translate", GestureMutation::Translate { dx, dy }) }));
    let (mx, my) = context.input.magnet;
    let near = (x - mx).pow(2) + (y - my).pow(2) <= context.input.radius.pow(2);
    sink.push(Command::Effect(if near { ToolYield::upsert("connect", GestureMutation::Connect { x: mx, y: my }) } else { ToolYield::retract("connect") }));
}

fn release<T: GestureTool>(_context: &mut GestureContext, _event: Option<&T::Event>, sink: &mut Vec<Command<T>>) {
    sink.push(Command::Effect(ToolYield::Commit));
}

fn cancel<T: GestureTool>(_context: &mut GestureContext, _event: Option<&T::Event>, sink: &mut Vec<Command<T>>) {
    sink.push(Command::Effect(ToolYield::Abort));
}

machine::statechart! {
    machine drag_gesture {
        context: GestureContext;
        event Event { PointerDown { x: i64, y: i64 }, PointerMove { x: i64, y: i64 }, PointerUp, Escape }
        input: GestureInput;
        output: ();
        effect: ToolYield<GestureMutation>;
        context_from_input: gesture_context;
        initial: idle;
        state idle {
            on PointerDown => pressed do press;
        }
        state pressed {
            on PointerMove => dragging do drag;
            on PointerUp => idle;
            on Escape => idle;
        }
        state dragging {
            on PointerMove => dragging do drag;
            on PointerUp => idle do release;
            on Escape => idle do cancel;
        }
    }
}

machine::statechart! {
    machine leaky_gesture {
        context: GestureContext;
        event Event { PointerDown { x: i64, y: i64 }, PointerMove { x: i64, y: i64 }, PointerUp, Escape }
        input: GestureInput;
        output: ();
        effect: ToolYield<GestureMutation>;
        context_from_input: gesture_context;
        initial: idle;
        state idle {
            on PointerDown => pressed do press;
        }
        state pressed {
            on PointerMove => dragging do drag;
            on PointerUp => idle;
            on Escape => idle;
        }
        state dragging {
            on PointerMove => dragging do drag;
            on PointerUp => idle;
            on Escape => idle do cancel;
        }
    }
}

macro_rules! gesture_tool {
    ($machine:ident :: $marker:ident) => {
        impl GestureTool for $machine::$marker {
            fn point(event: &$machine::Event) -> Option<(i64, i64)> {
                match event {
                    $machine::Event::PointerDown { x, y } | $machine::Event::PointerMove { x, y } => Some((*x, *y)),
                    $machine::Event::PointerUp | $machine::Event::Escape => None,
                }
            }

            fn event(value: &Value) -> $machine::Event {
                match text(&value["type"]) {
                    "pointerDown" => $machine::Event::PointerDown { x: int(&value["x"]), y: int(&value["y"]) },
                    "pointerMove" => $machine::Event::PointerMove { x: int(&value["x"]), y: int(&value["y"]) },
                    "pointerUp" => $machine::Event::PointerUp,
                    "escape" => $machine::Event::Escape,
                    other => panic!("unknown gesture event {other}"),
                }
            }
        }
    };
}

gesture_tool!(drag_gesture::DragGesture);
gesture_tool!(leaky_gesture::LeakyGesture);

fn gesture_mutation_json(mutation: &GestureMutation) -> Value {
    match mutation {
        GestureMutation::Translate { dx, dy } => json!({ "kind": "translate", "dx": dx, "dy": dy }),
        GestureMutation::Connect { x, y } => json!({ "kind": "connect", "x": x, "y": y }),
    }
}

fn step_json<T: GestureTool>(runner: &ToolMachineRunner<T, TestHost<T>>, step: &ToolStep<GestureMutation>) -> Value {
    match step {
        ToolStep::Idle => json!({ "kind": "idle" }),
        ToolStep::Open => {
            let transaction = runner.transaction().expect("an open step has an open transaction");
            json!({ "kind": "open", "transaction": reference_json(transaction.reference()), "entries": entries_json(transaction.entries(), gesture_mutation_json) })
        }
        ToolStep::Committed(reference, mutations) => json!({ "kind": "committed", "transaction": reference_json(reference), "mutations": mutations.iter().map(gesture_mutation_json).collect::<Vec<_>>() }),
        ToolStep::Aborted(reference, reason) => json!({ "kind": "aborted", "transaction": reference_json(reference), "reason": reason.as_str() }),
        ToolStep::Empty(reference) => json!({ "kind": "empty", "transaction": reference_json(reference) }),
    }
}

fn active_state<T: GestureTool>(runner: &ToolMachineRunner<T, TestHost<T>>) -> &'static str {
    ["idle", "pressed", "dragging"].into_iter().find(|state| runner.snapshot().matches(state)).expect("one atomic state is active")
}

fn gesture_input(gesture: &Value) -> GestureInput {
    GestureInput { magnet: (int(&gesture["input"]["magnet"]["x"]), int(&gesture["input"]["magnet"]["y"])), radius: int(&gesture["input"]["radius"]) }
}

fn gesture_runner<T: GestureTool>(gesture: &Value) -> ToolMachineRunner<T, TestHost<T>> {
    ToolMachineRunner::start(text(&gesture["tool"]), ActorId(text(&gesture["actor"]).to_string()), gesture_input(gesture), TestHost::new()).expect("entering idle yields nothing")
}

fn resumed<T: GestureTool>(gesture: &Value, snapshot: Snapshot<T>, transaction: Option<ToolTransaction<GestureMutation>>) -> Result<ToolMachineRunner<T, TestHost<T>>, ToolRefusal> {
    ToolMachineRunner::resume(text(&gesture["tool"]), ActorId(text(&gesture["actor"]).to_string()), gesture_input(gesture), snapshot, transaction, TestHost::new())
}

fn lower_camel(name: &str) -> String {
    let mut chars = name.chars();
    chars.next().map(|first| first.to_ascii_lowercase().to_string() + chars.as_str()).unwrap_or_default()
}

fn chart_json<T: GestureTool>(chart: &Value) -> (Value, Value) {
    let definition = T::definition();
    let stable = |id: machine::NodeId| definition.nodes[id.0 as usize].stable_id;
    let transitions: Vec<Value> = definition
        .transitions
        .iter()
        .map(|transition| {
            let Trigger::Event(event) = transition.trigger else { panic!("the gesture only has event transitions") };
            json!({ "from": stable(transition.source), "event": lower_camel(<T::Event as StatechartEvent>::event_name(event)), "to": stable(transition.targets[0]), "hasAction": !transition.actions.is_empty() })
        })
        .collect();
    let actual = json!({ "id": definition.id, "initial": stable(definition.nodes[0].initial.expect("root initial")), "states": definition.nodes.iter().skip(1).map(|node| node.stable_id).collect::<Vec<_>>(), "transitions": transitions });
    let expected = json!({ "id": chart["id"], "initial": chart["initial"], "states": chart["states"], "transitions": chart["transitions"].as_array().expect("transitions").iter().map(|row| json!({ "from": row["from"], "event": row["event"], "to": row["to"], "hasAction": !row["action"].is_null() })).collect::<Vec<_>>() });
    (actual, expected)
}

fn replay<T: GestureTool>(gesture: &Value, scenarios: &Value, resume_everywhere: bool) {
    for scenario in scenarios.as_array().expect("scenarios") {
        let name = text(&scenario["name"]);
        let mut runner = gesture_runner::<T>(gesture);
        for (index, row) in scenario["steps"].as_array().expect("steps").iter().enumerate() {
            let label = format!("{name} #{index} (resume everywhere: {resume_everywhere})");
            match row.get("host").map(text) {
                Some("resume") => {
                    let (snapshot, transaction) = runner.into_parts();
                    runner = resumed(gesture, snapshot, transaction).expect("a persisted gesture resumes");
                }
                Some("abort") => {
                    let step = runner.abort(ToolAbortReason::parse(text(&row["reason"])).expect("reason"));
                    assert_eq!(step_json(&runner, &step), row["step"], "{label}");
                }
                Some(_) => assert_eq!(runner.reset().as_ref().map_or(Value::Null, reference_json), row["dropped"], "{label}"),
                None => match (runner.send(T::event(&row["event"]), clock(&row["clock"])), row.get("refusal")) {
                    (Ok(step), None) => {
                        assert_eq!(step_json(&runner, &step), row["step"], "{label}");
                        assert_eq!(runner.transaction().is_some(), step == ToolStep::Open, "{label}: only an open step keeps a transaction");
                    }
                    (Err(refusal), Some(code)) => assert_eq!(refusal.to_string(), text(code), "{label}"),
                    (result, expected) => panic!("{label}: {result:?} but expected {expected:?}"),
                },
            }
            assert_eq!(active_state(&runner), text(&row["state"]), "{label}");
            assert!(runner.transaction().is_none() || !runner.at_rest(), "{label}: a resting tool never holds a transaction");
            if resume_everywhere {
                let (snapshot, transaction) = runner.into_parts();
                runner = resumed(gesture, snapshot, transaction).expect("every settled runner resumes");
            }
        }
    }
}

#[test]
fn every_statechart_is_its_fixture_chart() {
    let law = law();
    for variant in law["gesture"]["variants"].as_array().expect("variants") {
        let chart = &variant["chart"];
        let (actual, expected) = match text(&chart["id"]) {
            "drag_gesture" => chart_json::<drag_gesture::DragGesture>(chart),
            "leaky_gesture" => chart_json::<leaky_gesture::LeakyGesture>(chart),
            other => panic!("no statechart for chart {other}"),
        };
        assert_eq!(actual, expected);
    }
}

#[test]
fn runner_replays_every_gesture_scenario() {
    let law = law();
    let gesture = &law["gesture"];
    for variant in gesture["variants"].as_array().expect("variants") {
        for resume_everywhere in [false, true] {
            match text(&variant["chart"]["id"]) {
                "drag_gesture" => replay::<drag_gesture::DragGesture>(gesture, &variant["scenarios"], resume_everywhere),
                "leaky_gesture" => replay::<leaky_gesture::LeakyGesture>(gesture, &variant["scenarios"], resume_everywhere),
                other => panic!("no statechart for chart {other}"),
            }
        }
    }
}

#[test]
fn resume_admits_exactly_the_fixture_rows() {
    let law = law();
    let gesture = &law["gesture"];
    let clock = HybridLogicalTimestamp { actor: 7, physical_ms: 1, logical: 0 };
    for row in gesture["resume"].as_array().expect("resume") {
        let mut runner = gesture_runner::<drag_gesture::DragGesture>(gesture);
        let events: &[drag_gesture::Event] = match text(&row["state"]) {
            "idle" => &[],
            "pressed" => &[drag_gesture::Event::PointerDown { x: 0, y: 0 }],
            "dragging" => &[drag_gesture::Event::PointerDown { x: 0, y: 0 }, drag_gesture::Event::PointerMove { x: 1, y: 0 }],
            other => panic!("no path to {other}"),
        };
        for event in events {
            runner.send(event.clone(), clock).expect("legal gesture");
        }
        let (snapshot, _) = runner.into_parts();
        let open = || ToolTransaction::resume(TransactionRef::mint(&ActorId("actor-1".into()), &clock, "demo#drag"), vec![("translate".to_string(), GestureMutation::Translate { dx: 1, dy: 0 })]);
        let closed = |yielded: ToolYield<GestureMutation>| {
            let mut transaction = open();
            transaction.apply(yielded).expect("open accepts a close");
            transaction
        };
        let transaction = match text(&row["transaction"]) {
            "none" => None,
            "open" => Some(open()),
            "committed" => Some(closed(ToolYield::Commit)),
            "aborted" => Some(closed(ToolYield::Abort)),
            other => panic!("unknown transaction {other}"),
        };
        let outcome = resumed::<drag_gesture::DragGesture>(gesture, snapshot, transaction).map_or_else(|refusal| refusal.to_string(), |_| "ok".to_string());
        assert_eq!(outcome, text(&row["expect"]), "{row}");
    }
}

#[test]
fn press_drag_release_is_one_transaction_and_escape_leaves_zero_trace() {
    let law = law();
    let mut runner = gesture_runner::<drag_gesture::DragGesture>(&law["gesture"]);
    let tick = |physical_ms| HybridLogicalTimestamp { actor: 7, physical_ms, logical: 0 };
    let mut commits = Vec::new();
    for (index, event) in [
        drag_gesture::Event::PointerDown { x: 0, y: 0 },
        drag_gesture::Event::PointerMove { x: 1, y: 0 },
        drag_gesture::Event::PointerMove { x: 2, y: 0 },
        drag_gesture::Event::PointerUp,
        drag_gesture::Event::PointerDown { x: 0, y: 0 },
        drag_gesture::Event::PointerMove { x: 9, y: 9 },
        drag_gesture::Event::Escape,
        drag_gesture::Event::PointerDown { x: 0, y: 0 },
        drag_gesture::Event::PointerMove { x: 0, y: 5 },
        drag_gesture::Event::PointerUp,
    ]
    .into_iter()
    .enumerate()
    {
        if let ToolStep::Committed(reference, mutations) = runner.send(event, tick(2000 + index as u64)).expect("legal gesture") {
            commits.push((reference, mutations));
        }
    }
    assert_eq!(commits.len(), 2, "two released gestures are two transactions; the escaped one left nothing");
    assert_ne!(commits[0].0, commits[1].0);
    assert_eq!(commits[0].1, vec![GestureMutation::Translate { dx: 2, dy: 0 }]);
    assert_eq!(commits[1].1, vec![GestureMutation::Translate { dx: 0, dy: 5 }]);
    assert_eq!(commits[0].0, TransactionRef::mint(&ActorId("actor-1".into()), &tick(2001), "demo#drag"));
    assert!(runner.transaction().is_none());
}
//#endregion 🔖️Gesture

//#region 🔖️RunnerLaws
fn unit_context(_input: ()) {}

type ProbeCommands = Vec<Command<probe::Probe>>;

fn probe_twice(_context: &mut (), _event: Option<&probe::Event>, sink: &mut ProbeCommands) {
    sink.extend([Command::Effect(ToolYield::upsert("a", 1)), Command::Effect(ToolYield::Commit), Command::Effect(ToolYield::upsert("b", 2))]);
}

fn probe_arm(_context: &mut (), _event: Option<&probe::Event>, sink: &mut ProbeCommands) {
    sink.push(Command::Effect(ToolYield::upsert("hold", 1)));
}

fn probe_hold(_context: &mut (), _event: Option<&probe::Event>, sink: &mut ProbeCommands) {
    sink.push(Command::Effect(ToolYield::Commit));
}

machine::statechart! {
    machine probe {
        context: ();
        event Event { Twice, Arm }
        input: ();
        output: ();
        effect: ToolYield<i64>;
        context_from_input: unit_context;
        initial: resting;
        state resting {
            on Twice => resting do probe_twice;
            on Arm => armed do probe_arm;
        }
        state armed {
            invoke hold_probe;
            after 500 => resting do probe_hold;
        }
    }
}

fn eager_entry(_context: &mut (), _event: Option<&eager::Event>, sink: &mut Vec<Command<eager::Eager>>) {
    sink.push(Command::Effect(ToolYield::upsert("early", 0)));
}

machine::statechart! {
    machine eager {
        context: ();
        event Event { Noop }
        input: ();
        output: ();
        effect: ToolYield<i64>;
        context_from_input: unit_context;
        initial: entered;
        state entered {
            entry eager_entry;
            on Noop => entered;
        }
    }
}

fn probe_runner() -> ToolMachineRunner<probe::Probe, TestHost<probe::Probe>> {
    ToolMachineRunner::start("law#probe", ActorId("actor-1".into()), (), TestHost::new()).expect("no entry yields")
}

#[test]
fn a_yield_while_entering_the_initial_configuration_is_refused() {
    assert_eq!(ToolMachineRunner::<eager::Eager, TestHost<eager::Eager>>::start("law#eager", ActorId("actor-1".into()), (), TestHost::new()).err(), Some(ToolRefusal::Closed));
    assert_eq!(<eager::Event as StatechartEvent>::event_name(eager::Event::Noop.event_id()), "Noop", "the refused tool never receives an event");
}

#[test]
fn a_yield_after_the_close_within_one_event_publishes_nothing() {
    let mut runner = probe_runner();
    let clock = HybridLogicalTimestamp { actor: 1, physical_ms: 10, logical: 0 };
    assert_eq!(runner.send(probe::Event::Twice, clock), Err(ToolRefusal::Closed));
    assert!(runner.transaction().is_none());
    assert_eq!(runner.send(probe::Event::Arm, clock), Ok(ToolStep::Open), "the runner stays usable");
}

#[test]
fn timers_route_to_the_host_and_settle_like_events() {
    let mut runner = probe_runner();
    let armed = HybridLogicalTimestamp { actor: 1, physical_ms: 20, logical: 0 };
    assert_eq!(runner.send(probe::Event::Arm, armed), Ok(ToolStep::Open));
    let due = runner.host.advance(500);
    assert_eq!(due, vec![(TOOL_MACHINE_ACTOR, TimerId(0))]);
    let step = runner.timer_elapsed(due[0].1, HybridLogicalTimestamp { actor: 1, physical_ms: 520, logical: 0 });
    assert_eq!(step, Ok(ToolStep::Committed(TransactionRef::mint(&ActorId("actor-1".into()), &armed, "law#probe"), vec![1])), "the id is minted from the opening event");
    assert!(runner.snapshot().matches("resting"));
}

#[test]
fn a_host_abort_cancels_the_left_timers_and_invokes_and_rests_the_tool() {
    let mut runner = probe_runner();
    let armed = HybridLogicalTimestamp { actor: 1, physical_ms: 30, logical: 0 };
    assert_eq!(runner.send(probe::Event::Arm, armed), Ok(ToolStep::Open));
    assert_eq!(runner.host.started_tasks(), &[(TOOL_MACHINE_ACTOR, InvokeId(0))]);
    let reference = runner.transaction().expect("open").reference().clone();
    assert_eq!(runner.abort(ToolAbortReason::Frozen), ToolStep::Aborted(reference, ToolAbortReason::Frozen));
    assert!(runner.transaction().is_none() && runner.at_rest() && runner.snapshot().matches("resting"));
    assert!(runner.host.advance(500).is_empty(), "the armed timer was cancelled with its state");
    assert_eq!(runner.host.cancelled_tasks(), &[(TOOL_MACHINE_ACTOR, InvokeId(0))]);
    assert!(runner.host.started_tasks().is_empty());
    assert_eq!(runner.timer_elapsed(TimerId(0), armed), Ok(ToolStep::Idle), "a stale timer finds nothing to fire");
    assert_eq!(runner.abort(ToolAbortReason::Retired), ToolStep::Idle);
    assert_eq!(runner.reset(), None);
}
//#endregion 🔖️RunnerLaws

//#region 🔖️ScrubLaws
const SCRUB_LAW: &str = include_str!("../../🧫️fixtures/🧫️scrub-law/🔣️.json");

fn scrub_law() -> Value {
    serde_json::from_str(SCRUB_LAW).expect("scrub fixture parses")
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
                let step = ledger.send(text(&row["window"]), text(&row["tool"]), &actor, text(&row["base"]), scrub_input(&row["input"]), clock).expect("scrubs are never refused");
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

/// ⚖️ LAW: a scrub persisted between every two ticks is the scrub that never persisted — the stable-id configuration,
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
            ScrubInput::Tick { gesture, .. } | ScrubInput::Commit { gesture, .. } => ledger.closed.get("w") == Some(gesture),
            ScrubInput::Abort { .. } => false,
        };
        let mut next = ledger.clone();
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

/// ⚖️ LAW (CLOSURE-3): the scrub ledger is total — every order of up to four inputs on one window, across two presses, two
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

//#region 🔖️TypingLaws
const TYPING_LAW: &str = include_str!("../../🧫️fixtures/🧫️typing-law/🔣️.json");

fn typing_law() -> Value {
    serde_json::from_str(TYPING_LAW).expect("typing fixture parses")
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
                (window.to_string(), json!({ "buffer": state.buffer, "tool": state.tool, "deadline": state.deadline_ms, "transaction": reference_json(&state.transaction), "entries": entries_json(&state.entries, Value::clone) }))
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
    let actor = ActorId(text(&law["actor"]).to_string());
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

/// ⚖️ LAW: a run persisted between every two edits is the run that never persisted — the stable-id configuration, the
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

/// ⚖️ LAW: 200 characters typed in one burst are ONE run — one transaction, one committed edit — however many deliveries carry
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

//#region 🔖️NodeDragLaws
const NODE_DRAG_LAW: &str = include_str!("../../🧫️fixtures/🧫️node-drag-law/🔣️.json");

fn node_drag_row(json: &Value) -> protocol::DslValue {
    match json {
        Value::Null => protocol::DslValue::Null,
        Value::Bool(flag) => protocol::DslValue::Bool(*flag),
        Value::Number(number) => protocol::DslValue::float(number.as_f64().expect("a finite fixture number")),
        Value::String(text) => protocol::DslValue::String(text.clone()),
        Value::Array(items) => protocol::DslValue::Array(items.iter().map(node_drag_row).collect()),
        Value::Object(fields) => protocol::DslValue::Object(fields.iter().map(|(name, value)| (name.clone(), node_drag_row(value))).collect()),
    }
}

/// ⚖️ LAW (design §13.3): every valid gesture record row decodes to its record, re-encodes to a row that decodes to the same
/// record, and moves as the fixture states; every invalid row — the retired absolute `move{nodeId,x,y}` among them — is
/// refused by name.
#[test]
fn node_drag_rows_decode_to_their_records_and_invalid_rows_are_refused() {
    let law: Value = serde_json::from_str(NODE_DRAG_LAW).expect("fixture parses");
    for case in law["valid"].as_array().expect("valid") {
        let id = text(&case["id"]);
        let record = NodeDragRecord::from_row(&node_drag_row(&case["row"])).unwrap_or_else(|reason| panic!("{id}: {reason}"));
        let expected = &case["record"];
        assert_eq!(record, NodeDragRecord { gesture_id: text(&expected["gestureId"]).into(), node_ids: expected["nodeIds"].as_array().expect("ids").iter().map(|id| text(id).to_string()).collect(), dx: expected["dx"].as_f64().expect("dx"), dy: expected["dy"].as_f64().expect("dy") }, "{id}");
        assert_eq!(NodeDragRecord::from_row(&record.to_row()).as_ref(), Ok(&record), "{id}: the row round trips");
        assert_eq!(record.moves(), case["moves"].as_bool().expect("moves"), "{id}");
    }
    for case in law["invalid"].as_array().expect("invalid") {
        assert!(NodeDragRecord::from_row(&node_drag_row(&case["row"])).is_err(), "{}: {}", text(&case["id"]), text(&case["reason"]));
    }
}

/// ⚖️ LAW: the node-drag machine commits a release as ONE transaction of the guest's leaves, its ref minted from the
/// authoring actor, the clock and the tool; a release that yields nothing commits nothing.
#[test]
fn a_released_node_drag_commits_one_transaction_of_its_leaves() {
    let law: Value = serde_json::from_str(NODE_DRAG_LAW).expect("fixture parses");
    let commit = &law["commit"];
    let (tool, actor, at) = (text(&commit["tool"]), ActorId(text(&commit["actor"]).into()), clock(&commit["clock"]));
    let leaves: Vec<Value> = commit["leaves"].as_array().expect("leaves").clone();
    let (transaction, committed) = node_drag_commit(tool, actor.clone(), text(&commit["gesture"]), leaves.clone(), at).expect("the release commits");
    assert_eq!(transaction, TransactionRef::mint(&actor, &at, tool));
    assert_eq!(committed, leaves);
    assert_eq!(node_drag_commit::<Value>(tool, actor, text(&commit["gesture"]), Vec::new(), at), None, "an empty release leaves zero trace");
    eprintln!("[DEBUG] released node drag retained exact guest leaves in one actor/clock/tool transaction; empty release emitted nothing");
}
//#endregion 🔖️NodeDragLaws

//#region 🎯️OnceLaws
/// 🎯️ LAW (design §22.32): a one-step tool dispatch is ONE transaction of its leaves stamped `<appId>#<verb>`; a dispatch
/// that yields nothing leaves zero trace; without an admission the leaves publish plainly.
#[test]
fn a_one_step_tool_commits_one_transaction_of_its_leaves() {
    let NodeDragEmit::Commit(transaction, leaves) = tool_once_emit("demo@1/*#editor", "place", "seed", vec![1, 2]) else { panic!("an admitted one-step dispatch commits") };
    assert!(transaction.id.starts_with("tx-"), "{transaction:?}");
    assert_eq!((transaction.tool.as_str(), leaves), ("demo@1/*#editor#place", vec![1, 2]));
    assert_eq!(tool_once_emit::<i64>("demo@1/*#editor", "place", "seed", Vec::new()), NodeDragEmit::Nothing);
    assert_eq!(tool_once_emit("demo@1/*#editor", "place", "", vec![3]), NodeDragEmit::Plain(vec![3]));
    eprintln!("[DEBUG] one-step tool emitted its two exact leaves in one admitted transaction, zero leaves emitted nothing, and unadmitted leaves remained plain");
}
//#endregion 🎯️OnceLaws

#[test]
fn independent_one_step_dispatches_never_share_a_transaction() {
    let mut ids = std::collections::BTreeSet::new();
    let fixture: Value = serde_json::from_str(NODE_DRAG_LAW).expect("fixture");
    for _ in 0..fixture["independentReleases"]["count"].as_u64().expect("count") {
        let NodeDragEmit::Commit(transaction, _) = tool_once_emit("demo@1/*#editor", "place", "seed", vec![1]) else { panic!("one-step dispatch commits") };
        assert!(ids.insert(transaction.id), "each independent gesture owns its transaction");
    }
    assert_eq!(ids.len() as u64, fixture["independentReleases"]["uniqueTransactions"].as_u64().expect("unique transactions"));
    eprintln!("[DEBUG] independent one-step dispatches retained {} distinct transactions against the authored release census", ids.len());
}
