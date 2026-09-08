mod tests {
    use super::super::host::TestHost;
    use super::super::testing::support::{UnitToggleContext, UnitToggleEvent, UnitToggleMachine};
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn actor_system_drains_sent_events_through_one_macrostep_each() {
        let mut system: ActorSystem<UnitToggleMachine, TestHost<UnitToggleMachine>> = ActorSystem::new(TestHost::new());
        let root = system.spawn_root(());
        assert!(system.snapshot(root).unwrap().matches("off"));

        system.send(root, UnitToggleEvent::Flip);
        let reports = system.drain();
        assert_eq!(reports.len(), 1);
        assert!(system.snapshot(root).unwrap().matches("on"));

        system.send(root, UnitToggleEvent::Flip);
        system.drain();
        assert!(system.snapshot(root).unwrap().matches("off"));
        assert_eq!(system.snapshot(root).unwrap().context, UnitToggleContext::default());
    }
}
