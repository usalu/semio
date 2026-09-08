mod tests {
    use super::*;

    struct DummyMachine;
    impl Machine for DummyMachine {
        type Context = ();
        type Event = super::super::testing::support::UnitEvent;
        type Input = ();
        type Output = ();
        type Effect = &'static str;
        type Config = crate::BitSet<1>;
        fn definition() -> &'static super::super::kernel::MachineDefinition<Self> {
            unimplemented!("host tests never step a machine")
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn test_host_advance_fires_due_timers_only() {
        let mut host = TestHost::<DummyMachine>::new();
        host.schedule(ActorId(0), TimerId(0), 100);
        host.schedule(ActorId(0), TimerId(1), 300);
        let due = host.advance(150);
        assert_eq!(due, vec![(ActorId(0), TimerId(0))]);
        let due = host.advance(200);
        assert_eq!(due, vec![(ActorId(0), TimerId(1))]);
    }

    #[semio_framework_async_macros::async_test]
    async fn test_host_cancel_timer_removes_pending() {
        let mut host = TestHost::<DummyMachine>::new();
        host.schedule(ActorId(0), TimerId(0), 100);
        host.cancel_timer(ActorId(0), TimerId(0));
        assert_eq!(host.advance(200), Vec::new());
    }

    #[semio_framework_async_macros::async_test]
    async fn test_host_records_effects_and_task_lifecycle() {
        let mut host = TestHost::<DummyMachine>::new();
        host.execute_effect(ActorId(0), "audit");
        assert_eq!(host.effects(), &[(ActorId(0), "audit")]);
        host.start_task(ActorId(0), InvokeId(0));
        assert_eq!(host.started_tasks(), &[(ActorId(0), InvokeId(0))]);
        host.cancel_task(ActorId(0), InvokeId(0));
        assert!(host.started_tasks().is_empty());
        assert_eq!(host.cancelled_tasks(), &[(ActorId(0), InvokeId(0))]);
    }
}
