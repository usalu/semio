/// 🌊️ Gesture tool laws: the example drag gesture statecharts, the gesture runner and its replay of every fixture scenario, the resume rows and the press, drag and release transaction.

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
