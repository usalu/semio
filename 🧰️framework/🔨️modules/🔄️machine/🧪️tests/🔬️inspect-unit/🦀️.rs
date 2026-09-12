mod tests {
    use super::*;

    #[test]
    fn null_inspector_observes_nothing_observable() {
        // Compile-only smoke test: NullInspector must be constructible and callable
        // without a concrete Machine — exercised indirectly by kernel tests.
        let _inspector = NullInspector;
    }

    #[test]
    fn trace_inspector_records_one_microstep_per_transition() {
        use super::super::kernel::{init, macrostep};
        use super::super::testing::toggle_model::{UnitToggleEvent, UnitToggleMachine};

        let mut sink = Vec::new();
        let mut snapshot = init::<UnitToggleMachine>((), &mut sink);
        let mut inspector = TraceInspector::<UnitToggleMachine>::default();
        macrostep(&mut snapshot, UnitToggleEvent::Flip, &mut sink, &mut inspector);
        macrostep(&mut snapshot, UnitToggleEvent::Flip, &mut sink, &mut inspector);

        assert_eq!(inspector.entries.len(), 2);
        assert_eq!(inspector.entries[0].exited, vec![NodeId(1)]);
        assert_eq!(inspector.entries[0].entered, vec![NodeId(2)]);
        assert_eq!(inspector.entries[1].exited, vec![NodeId(2)]);
        assert_eq!(inspector.entries[1].entered, vec![NodeId(1)]);
    }
}
