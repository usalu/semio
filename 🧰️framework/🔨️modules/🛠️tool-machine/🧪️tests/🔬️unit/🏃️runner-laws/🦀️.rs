/// 🏃️ Runner laws: the probe statecharts, the refused and unpublished yields and the timer, abort and rest behaviour of the runner.

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
