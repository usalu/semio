//! 🌊️ The shared streamed-gesture runner (`drive_gesture`) against `🧫️fixtures/🧫️gesture-drive-law`, authored by an
//! independent Python model of its counting tool (`🧪️s4-tools-a-gesture-drive-law.py` of ticket
//! 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING) and judged in TypeScript against xstate and fast-check: the phase argument
//! table, one dispatch per row from the window's persisted gesture with the abort reasons the drive called, the reason
//! each host fact ends an open gesture with, and step sequences over the per-window ledger (design §22.10) driven by the
//! counting tool as a real statechart on the shared runner.

use super::*;
use serde_json::{json, Value};
use std::cell::RefCell;

const GESTURE_DRIVE_LAW: &str = include_str!("../../🧫️fixtures/🧫️gesture-drive-law/🔣️.json");

thread_local! {
    static ABORTED: RefCell<Vec<ToolAbortReason>> = const { RefCell::new(Vec::new()) };
}

fn law() -> Value {
    serde_json::from_str(GESTURE_DRIVE_LAW).expect("the gesture-drive law parses")
}

fn text(value: &Value) -> &str {
    value.as_str().expect("a string")
}

/// 📜️ The counting tool's contract the fixture names: the verb `start` refuses and the code of each refusal.
struct Contract {
    refuse_start_verb: String,
    start: ToolRefusal,
    resume: ToolRefusal,
    send: ToolRefusal,
}

fn contract() -> &'static Contract {
    static CONTRACT: std::sync::OnceLock<Contract> = std::sync::OnceLock::new();
    CONTRACT.get_or_init(|| {
        let tool = law()["tool"].clone();
        let refusal = |key: &str| ToolRefusal::parse(text(&tool[key])).expect("a known refusal");
        Contract { refuse_start_verb: text(&tool["refuseStartVerb"]).to_string(), start: refusal("startRefusal"), resume: refusal("resumeRefusal"), send: refusal("sendRefusal") }
    })
}

/// 🧾️ The counting tool's persisted open gesture; `corrupt` marks one its `resume` refuses.
#[derive(Clone, Debug, PartialEq)]
struct LawGesture {
    verb: String,
    base: String,
    ticks: Vec<i64>,
    corrupt: bool,
}

impl LawGesture {
    fn from_json(value: &Value) -> Option<Self> {
        value.is_object().then(|| Self {
            verb: text(&value["verb"]).to_string(),
            base: text(&value["base"]).to_string(),
            ticks: value["ticks"].as_array().expect("ticks").iter().map(|tick| tick.as_i64().expect("an integer tick")).collect(),
            corrupt: value["corrupt"].as_bool().unwrap_or(false),
        })
    }

    fn to_json(&self) -> Value {
        let mut gesture = json!({ "verb": self.verb, "base": self.base, "ticks": self.ticks });
        if self.corrupt {
            gesture["corrupt"] = json!(true);
        }
        gesture
    }
}

/// 🧮️ The fixture's counting tool: a stream tick accumulates (open while it has ticks), a one-shot or commit folds its
/// tick in and commits the ticks, an abort clears them; `send` refuses a negative tick.
struct LawTool {
    verb: String,
    base: String,
    ticks: Vec<i64>,
}

impl GestureTool for LawTool {
    type Gesture = LawGesture;
    type Tick = i64;
    type Mutation = i64;

    fn start(verb: &str, _authoring_seed: &str, base_revision: &str) -> Result<Self, ToolRefusal> {
        if verb == contract().refuse_start_verb {
            return Err(contract().start);
        }
        Ok(Self { verb: verb.into(), base: base_revision.into(), ticks: Vec::new() })
    }

    fn resume(gesture: &LawGesture) -> Result<Self, ToolRefusal> {
        if gesture.corrupt {
            return Err(contract().resume);
        }
        Ok(Self { verb: gesture.verb.clone(), base: gesture.base.clone(), ticks: gesture.ticks.clone() })
    }

    fn verb(&self) -> &str {
        &self.verb
    }

    fn base_revision(&self) -> &str {
        &self.base
    }

    fn abort(&mut self, reason: ToolAbortReason) {
        ABORTED.with(|aborted| aborted.borrow_mut().push(reason));
        self.ticks.clear();
    }

    fn send(&mut self, phase: GesturePhase, tick: Option<i64>) -> Result<ToolStep<i64>, ToolRefusal> {
        if tick.is_some_and(|tick| tick < 0) {
            return Err(contract().send);
        }
        self.ticks.extend(tick);
        Ok(match phase {
            _ if self.ticks.is_empty() => ToolStep::Idle,
            GesturePhase::Stream => ToolStep::Open,
            GesturePhase::Once | GesturePhase::Commit | GesturePhase::Abort(_) => ToolStep::Committed(TransactionRef { id: format!("{}#1", self.verb), tool: self.verb.clone() }, std::mem::take(&mut self.ticks)),
        })
    }

    fn persist(self) -> Option<LawGesture> {
        (!self.ticks.is_empty()).then(|| LawGesture { verb: self.verb, base: self.base, ticks: self.ticks, corrupt: false })
    }
}

/// 🏃️ One dispatch through `drive_gesture` from the window's persisted gesture, reported in the fixture's outcome shape.
fn outcome(persisted: Option<&LawGesture>, dispatch: &Value) -> Value {
    ABORTED.with(|aborted| aborted.borrow_mut().clear());
    let verb = text(&dispatch["verb"]);
    let phase = GesturePhase::parse(dispatch["phase"].as_str(), dispatch["reason"].as_str()).expect("a known phase");
    let result = drive_gesture::<LawTool>(persisted, verb, phase, dispatch["tick"].as_i64(), "seed", text(&dispatch["base"]));
    let aborted = ABORTED.with(|aborted| aborted.borrow().clone());
    assert!(aborted.len() <= 1, "one dispatch aborts at most one gesture: {aborted:?}");
    let aborted = aborted.first().map_or(Value::Null, |reason| json!(reason.as_str()));
    match result {
        Err(refusal) => json!({ "aborted": aborted, "committed": null, "next": "unchanged", "refused": refusal.code() }),
        Ok(drive) => json!({
            "aborted": aborted,
            "committed": drive.committed.map_or(Value::Null, |(reference, ticks)| {
                assert_eq!(reference.tool, verb, "a commit is stamped by the dispatching verb's transaction");
                json!(ticks)
            }),
            "next": match drive.next {
                None => json!("unchanged"),
                Some(None) => json!("cleared"),
                Some(Some(gesture)) => gesture.to_json(),
            },
            "refused": null,
        }),
    }
}

/// 🗣️ LAW: a gesture verb's `phase`/`reason` arguments parse like the fixture's table (absent phase = one-shot, absent
/// reason = `tool`, unknown words refused).
#[test]
fn the_phase_arguments_parse_like_the_fixture() {
    for row in law()["phases"].as_array().expect("phases") {
        let parsed = match GesturePhase::parse(row["args"]["phase"].as_str(), row["args"]["reason"].as_str()) {
            None => Value::Null,
            Some(GesturePhase::Once) => json!({ "kind": "once" }),
            Some(GesturePhase::Stream) => json!({ "kind": "stream" }),
            Some(GesturePhase::Commit) => json!({ "kind": "commit" }),
            Some(GesturePhase::Abort(reason)) => json!({ "kind": "abort", "reason": reason.as_str() }),
        };
        assert_eq!(parsed, row["phase"], "{}", row["args"]);
    }
}

/// 🎞️ LAW: every fixture row drives to its expected outcome — one-shots, streams, commits and aborts at rest, open and
/// unrestorable; moved bases, verb switches and one-shot interruptions; refused starts and ticks with no effect at all.
#[test]
fn every_fixture_row_drives_to_its_expected_outcome() {
    for row in law()["rows"].as_array().expect("rows") {
        let persisted = LawGesture::from_json(&row["persisted"]);
        assert_eq!(outcome(persisted.as_ref(), &row["dispatch"]), row["expected"], "{}", row["name"]);
    }
}

//#region 🗄️Ledger
/// 🔢️ The counting tool as a real statechart on the shared runner: a stream tick upserts `tick:<n>` (the gesture is open
/// while it has ticks), a one-shot or commit folds its tick in and commits, a negative tick leaves the chart at rest with
/// its transaction open — the runner's `Unclosed` refusal (the fixture's `sendRefusal`).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CounterContext {
    ticks: usize,
}

/// 🎟️ What one counting dispatch carries: its tick, if any.
type CounterTick = Option<i64>;

fn counter_context(input: CounterContext) -> CounterContext {
    input
}

fn counter_tick(event: Option<&counter::Event>) -> Option<i64> {
    match event {
        Some(counter::Event::Once(tick) | counter::Event::Stream(tick) | counter::Event::Finish(tick)) => *tick,
        _ => None,
    }
}

fn counter_ticks(_context: &CounterContext, event: Option<&counter::Event>) -> bool {
    counter_tick(event).is_some()
}

fn counter_refuses(_context: &CounterContext, event: Option<&counter::Event>) -> bool {
    counter_tick(event).is_some_and(|tick| tick < 0)
}

fn counter_upsert(context: &mut CounterContext, event: Option<&counter::Event>, sink: &mut Vec<Command<counter::Counter>>) {
    if let Some(tick) = counter_tick(event) {
        sink.push(Command::Effect(ToolYield::upsert(format!("tick:{}", context.ticks), tick)));
        context.ticks += 1;
    }
}

fn counter_commit(context: &mut CounterContext, event: Option<&counter::Event>, sink: &mut Vec<Command<counter::Counter>>) {
    counter_upsert(context, event, sink);
    sink.push(Command::Effect(ToolYield::Commit));
    context.ticks = 0;
}

machine::statechart! {
    machine counter {
        context: CounterContext;
        event Event { Once(CounterTick), Stream(CounterTick), Finish(CounterTick) }
        input: CounterContext;
        output: ();
        effect: ToolYield<i64>;
        context_from_input: counter_context;
        initial: idle;
        state idle {
            on Once if counter_refuses => idle do counter_upsert;
            on Stream if counter_refuses => idle do counter_upsert;
            on Once if counter_ticks => idle do counter_commit;
            on Stream if counter_ticks => streaming do counter_upsert;
        }
        state streaming {
            on Stream if counter_refuses => idle do counter_upsert;
            on Finish if counter_refuses => idle do counter_upsert;
            on Stream => streaming do counter_upsert;
            on Finish => idle do counter_commit;
        }
    }
}

/// 🏚️ The counting chart's host: no timer, no invoke, no foreign effect.
pub struct CounterHost;

impl Host<counter::Counter> for CounterHost {
    fn execute_effect(&mut self, _actor: machine::ActorId, _effect: ToolYield<i64>) {}
    fn schedule(&mut self, _actor: machine::ActorId, _timer: TimerId, _delay_ms: u64) {}
    fn cancel_timer(&mut self, _actor: machine::ActorId, _timer: TimerId) {}
    fn start_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn cancel_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn now_ms(&self) -> u64 {
        0
    }
}

impl GestureChart for counter::Counter {
    type Tick = i64;
    type Host = CounterHost;

    fn tool(verb: &str) -> String {
        format!("law#{verb}")
    }

    fn host() -> CounterHost {
        CounterHost
    }

    fn input() -> CounterContext {
        CounterContext::default()
    }

    fn restore(entries: &[(String, i64)], context: &protocol::DslValue) -> Option<CounterContext> {
        matches!(context, protocol::DslValue::Null).then_some(CounterContext { ticks: entries.len() })
    }

    fn event(phase: GesturePhase, at_rest: bool, tick: Option<i64>) -> Option<counter::Event> {
        match phase {
            GesturePhase::Stream => Some(counter::Event::Stream(tick)),
            GesturePhase::Commit if !at_rest => Some(counter::Event::Finish(tick)),
            GesturePhase::Once | GesturePhase::Commit => Some(counter::Event::Once(tick)),
            GesturePhase::Abort(_) => None,
        }
    }
}

type Counting = ChartGesture<counter::Counter>;

/// 🪟️ A ledger slot in the fixture's gesture shape; `corrupt` marks one whose configuration the chart cannot restore.
fn slot_json(gesture: &GestureState<i64>) -> Value {
    let mut slot = json!({ "verb": gesture.verb, "base": gesture.base_revision, "ticks": gesture.entries.iter().map(|(_, tick)| *tick).collect::<Vec<_>>() });
    if gesture.states.iter().any(|state| state == "tampered") {
        slot["corrupt"] = json!(true);
    }
    if !gesture.press.is_empty() {
        slot["press"] = json!(gesture.press);
    }
    slot
}

fn ended(aborted: Vec<(String, ToolStep<i64>)>) -> Value {
    Value::Array(
        aborted
            .into_iter()
            .map(|(window, step)| match step {
                ToolStep::Aborted(_, reason) => json!({ "window": window, "reason": reason.as_str() }),
                other => panic!("a host fact reported {}", other.kind()),
            })
            .collect(),
    )
}

/// 👣️ One slot step through the real ledger, reported in the fixture's outcome shape.
fn ledger_step(ledger: &mut GestureLedger<i64>, step: &Value) -> Value {
    let (mut committed, mut refused, mut aborted) = (Value::Null, Value::Null, json!([]));
    if let Some(drive) = step.get("drive") {
        let phase = GesturePhase::parse(drive["phase"].as_str(), drive["reason"].as_str()).expect("a known phase");
        match ledger.drive::<Counting>(text(&drive["window"]), drive["press"].as_str(), text(&drive["verb"]), phase, drive["tick"].as_i64(), "seed", text(&drive["base"])) {
            Ok(Some((reference, ticks))) => {
                assert_eq!(reference.tool, format!("law#{}", text(&drive["verb"])), "a commit is stamped `<appId>#<verb>`");
                committed = json!(ticks);
            }
            Ok(None) => {}
            Err(refusal) => refused = json!(refusal.code()),
        }
    } else if let Some(host) = step.get("host") {
        let window = text(&host["window"]);
        let event = GestureHostEvent::parse(text(&host["event"])).expect("a known host fact");
        aborted = ended(Some(ledger.host_event(window, event)).filter(|step| !matches!(step, ToolStep::Idle)).map(|step| (window.to_string(), step)).into_iter().collect());
    } else if let Some(host) = step.get("hostAll") {
        aborted = ended(ledger.host_event_all(GestureHostEvent::parse(text(&host["event"])).expect("a known host fact")));
    } else if let Some(keep) = step.get("retain") {
        let keep: Vec<&str> = keep.as_array().expect("windows").iter().map(text).collect();
        aborted = ended(ledger.retain_windows(|window| keep.contains(&window)));
    } else {
        let window = text(&step["corrupt"]);
        let mut gesture = ledger.open(window).expect("a slot to tamper").clone();
        gesture.states = vec!["tampered".to_string()];
        ledger.settle(window, Some(gesture));
    }
    let open: serde_json::Map<String, Value> = ledger.windows().map(|window| (window.to_string(), slot_json(ledger.open(window).expect("a listed window is open")))).collect();
    let closed: serde_json::Map<String, Value> = ledger.closed_presses().map(|(window, press)| (window.to_string(), json!(press))).collect();
    json!({ "committed": committed, "refused": refused, "aborted": aborted, "open": open, "closed": closed })
}

/// 📡️ LAW (design §22.10): every host fact ends an open gesture with the fixture's reason — a history edit `frozen`, a
/// blur `blur`, a lost capture `captureLost`, a utility switch or a closing window `retired` — and a moved base ends only
/// a gesture pinned to a base revision; the table covers every fact for both kinds of gesture.
#[test]
fn every_host_fact_ends_a_gesture_with_the_fixtures_reason() {
    let law = law();
    let rows = law["hostEvents"].as_array().expect("hostEvents");
    assert_eq!(rows.len(), GestureHostEvent::ALL.len() * 2);
    for event in GestureHostEvent::ALL {
        assert_eq!(GestureHostEvent::parse(event.as_str()), Some(event));
        for base_bound in [true, false] {
            let row = rows.iter().find(|row| text(&row["event"]) == event.as_str() && row["baseBound"] == json!(base_bound)).unwrap_or_else(|| panic!("no row for {} / {base_bound}", event.as_str()));
            assert_eq!(event.abort_reason(base_bound).map_or(Value::Null, |reason| json!(reason.as_str())), row["reason"], "{} / {base_bound}", event.as_str());
        }
    }
    assert_eq!(GestureHostEvent::TimeTravelFrozen.abort_reason(false), Some(ToolAbortReason::Frozen));
}

/// 🗄️ LAW (design §22.10): every slot scenario steps the real ledger — the statechart counting tool persisted by one
/// dispatch and resumed by the next — to the fixture's outcome: a gesture persists between dispatches and commits once, a
/// history edit ends every window's gesture with zero trace, windows never share a slot, a refused tick keeps the slot, an
/// unrestorable slot is dropped by the next dispatch; and the slot owns the press identity: a dispatch of the press a window
/// last closed is dropped with zero trace, a dispatch of another press interrupts the open gesture first.
#[test]
fn every_slot_scenario_steps_the_ledger_to_its_expected_outcome() {
    for scenario in law()["slots"].as_array().expect("slots") {
        let mut ledger = GestureLedger::<i64>::default();
        for (index, step) in scenario["steps"].as_array().expect("steps").iter().enumerate() {
            assert_eq!(ledger_step(&mut ledger, step), step["expected"], "{} step {index}", scenario["name"]);
        }
    }
}

/// 💾️ LAW (design §22.10): the persisted gesture is the framework's own form — one stream tick persists the chart's
/// configuration by stable id, the verb, the admission and revision it opened on and its open transaction; resuming and
/// persisting it again is the identity; the ledger's provisional overlay is exactly its entries; a gesture persisted with a
/// context its chart cannot restore, or resting with an open transaction, is refused and dropped with zero trace.
#[test]
fn a_persisted_gesture_resumes_to_itself_and_refuses_what_its_chart_cannot_restore() {
    let mut ledger = GestureLedger::<i64>::default();
    assert_eq!(ledger.drive::<Counting>("w1", Some("p1"), "drag", GesturePhase::Stream, Some(4), "seed", "r1"), Ok(None));
    let gesture = ledger.open("w1").expect("the tick opened the gesture").clone();
    assert!(gesture.states.iter().any(|state| state == "streaming") && !gesture.states.iter().any(|state| state == "idle"), "the configuration is persisted by stable id: {:?}", gesture.states);
    assert_eq!((gesture.verb.as_str(), gesture.press.as_str(), gesture.authoring_seed.as_str(), gesture.base_revision.as_str()), ("drag", "p1", "seed", "r1"));
    assert_eq!((gesture.transaction.tool.as_str(), gesture.entries.as_slice(), &gesture.context), ("law#drag", [("tick:0".to_string(), 4)].as_slice(), &protocol::DslValue::Null));
    assert_eq!(Counting::resume(&gesture).expect("the gesture resumes").persist(), Some(GestureState { press: String::new(), ..gesture.clone() }), "the tool persists no press: the slot stamps it");
    assert_eq!(ledger.provisional().copied().collect::<Vec<_>>(), vec![4]);
    assert_eq!(ledger.drive::<Counting>("w1", Some("p1"), "drag", GesturePhase::Stream, None, "other", "r1"), Ok(None));
    assert_eq!(ledger.open("w1"), Some(&gesture), "a tickless stream resumes and keeps the gesture");
    let unknown = GestureState { context: protocol::DslValue::Bool(true), ..gesture.clone() };
    assert_eq!(Counting::resume(&unknown).err(), Some(ToolRefusal::Closed));
    let resting = GestureState { states: vec!["idle".to_string()], ..gesture.clone() };
    assert_eq!(Counting::resume(&resting).err(), Some(ToolRefusal::Unclosed));
    ledger.settle("w1", Some(resting));
    assert_eq!(ledger.drive::<Counting>("w1", Some("p1"), "drag", GesturePhase::Commit, None, "seed", "r1"), Ok(None));
    assert!(ledger.is_empty(), "an unrestorable gesture is dropped with zero trace");
    assert_eq!(ledger.closed("w1"), Some("p1"), "and its press is closed");
    assert_eq!(drive_chart_gesture::<counter::Counter>(None, "drag", GesturePhase::Once, Some(7), "seed", "r1").expect("a one-shot at rest").committed.map(|(_, ticks)| ticks), Some(vec![7]));
}
//#endregion 🗄️Ledger
