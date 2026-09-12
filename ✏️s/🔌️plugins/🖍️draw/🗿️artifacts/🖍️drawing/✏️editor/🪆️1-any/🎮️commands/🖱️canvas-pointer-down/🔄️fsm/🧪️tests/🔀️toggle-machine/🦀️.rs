pub(crate) mod toggle_model {
    use super::super::kernel::{MachineDefinition, NodeDef, NodeKind, ROOT, TransitionDef, TransitionKind, Trigger};
    use crate::{BitSet, EventId, Machine, StatechartEvent};

    #[derive(Clone, Debug, PartialEq)]
    pub struct UnitEvent;

    impl StatechartEvent for UnitEvent {
        const EVENT_COUNT: u16 = 1;
        fn event_id(&self) -> EventId {
            EventId(0)
        }
        fn event_name(_id: EventId) -> &'static str {
            "Unit"
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum UnitToggleEvent {
        Flip,
    }

    impl StatechartEvent for UnitToggleEvent {
        const EVENT_COUNT: u16 = 1;
        fn event_id(&self) -> EventId {
            EventId(0)
        }
        fn event_name(_id: EventId) -> &'static str {
            "Flip"
        }
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct UnitToggleContext {
        pub count: u32,
    }

    const NODES: &[NodeDef] = &[
        NodeDef { stable_id: "root", kind: NodeKind::Compound, parent: None, initial: Some(crate::NodeId(1)), children: &[crate::NodeId(1), crate::NodeId(2)], entry_actions: &[], exit_actions: &[], invokes: &[], timers: &[], doc_index: 0 },
        NodeDef { stable_id: "off", kind: NodeKind::Atomic, parent: Some(ROOT), initial: None, children: &[], entry_actions: &[], exit_actions: &[], invokes: &[], timers: &[], doc_index: 1 },
        NodeDef { stable_id: "on", kind: NodeKind::Atomic, parent: Some(ROOT), initial: None, children: &[], entry_actions: &[], exit_actions: &[], invokes: &[], timers: &[], doc_index: 2 },
    ];

    const TRANSITIONS: &[TransitionDef] = &[
        TransitionDef { source: crate::NodeId(1), trigger: Trigger::Event(EventId(0)), guard: None, targets: &[crate::NodeId(2)], kind: TransitionKind::External, actions: &[], doc_index: 0 },
        TransitionDef { source: crate::NodeId(2), trigger: Trigger::Event(EventId(0)), guard: None, targets: &[crate::NodeId(1)], kind: TransitionKind::External, actions: &[], doc_index: 1 },
    ];

    pub struct UnitToggleMachine;

    impl Machine for UnitToggleMachine {
        type Context = UnitToggleContext;
        type Event = UnitToggleEvent;
        type Input = ();
        type Output = ();
        type Effect = ();
        type Config = BitSet<1>;
        fn definition() -> &'static MachineDefinition<Self> {
            static DEF: MachineDefinition<UnitToggleMachine> =
                MachineDefinition { id: "unit_toggle", nodes: NODES, transitions: TRANSITIONS, context_from_input: |_| UnitToggleContext::default(), make_output: None, guards: &[], actions: &[], fingerprint: 42, manifest_json: "{}" };
            &DEF
        }
    }

    pub fn unit_toggle_definition() -> &'static MachineDefinition<UnitToggleMachine> {
        UnitToggleMachine::definition()
    }
}

mod tests {
    use super::super::testing::toggle_model::{UnitToggleEvent, UnitToggleMachine};
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn explore_reaches_both_toggle_states() {
        let model = Model::<UnitToggleMachine>::new(vec![UnitToggleEvent::Flip]);
        let coverage = explore(&model, ());
        assert!(coverage.reached_stable_ids.contains(&"off"));
        assert!(coverage.reached_stable_ids.contains(&"on"));
        assert_eq!(coverage.visited_configurations, 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn conformance_fixture_passes_for_matching_sequence() {
        let steps = [ConformanceStep { event: UnitToggleEvent::Flip, expect_active: &["on"] }, ConformanceStep { event: UnitToggleEvent::Flip, expect_active: &["off"] }];
        assert!(run_conformance::<UnitToggleMachine>((), &steps).is_ok());
    }

    #[semio_framework_async_macros::async_test]
    async fn conformance_fixture_fails_with_descriptive_message() {
        let steps = [ConformanceStep { event: UnitToggleEvent::Flip, expect_active: &["off"] }];
        let err = run_conformance::<UnitToggleMachine>((), &steps).unwrap_err().to_string();
        assert!(err.contains("step 0"));
        assert!(err.contains("off"));
    }

    #[semio_framework_async_macros::async_test]
    async fn invariant_reports_violation_by_name() {
        let mut sink: Vec<Command<UnitToggleMachine>> = Vec::new();
        let snapshot = init::<UnitToggleMachine>((), &mut sink);
        let invariants = [Invariant { name: "never off", check: |s: &Snapshot<UnitToggleMachine>| if s.matches("off") { Err(FsmError::Violation("was off".to_string())) } else { Ok(()) } }];
        let violations = check_invariants(&snapshot, &invariants);
        assert_eq!(violations, vec!["never off: was off".to_string()]);
    }
}
