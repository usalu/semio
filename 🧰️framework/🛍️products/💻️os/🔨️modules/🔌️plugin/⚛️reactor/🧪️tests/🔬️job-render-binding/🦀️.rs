use super::*;

#[test]
fn progress_accepts_only_the_current_instance_generation() {
    let mut registry = JobRenderBindingRegistry::new();
    let first = registry.bind(7, 41).expect("first binding");
    assert_eq!(registry.accepted(41), Some(first));
    let second = registry.bind(7, 42).expect("superseding binding");
    assert_ne!(first.generation, second.generation);
    assert_eq!(registry.accepted(41), None);
    assert_eq!(registry.accepted(42), Some(second));
    assert_eq!(registry.complete(41), None);
    assert_eq!(registry.complete(42), Some(second));
}

#[test]
fn direct_slots_reject_collisions_and_close_exactly_one_instance() {
    let mut registry = JobRenderBindingRegistry::new();
    let binding = registry.bind(3, 9).expect("binding");
    assert!(registry.bind(3 + REACTOR_TASK_SLOTS as u32, 10).is_err());
    assert!(registry.bind(4, 9 + REACTOR_TASK_SLOTS as u64).is_err());
    registry.close_instance(3);
    assert_eq!(registry.accepted(binding.job), None);
}
