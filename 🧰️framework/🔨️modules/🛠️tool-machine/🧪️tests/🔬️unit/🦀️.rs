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
